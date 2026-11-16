use std::{
    collections::{hash_map::Entry, HashMap},
    io::{self, ErrorKind},
    marker::PhantomData,
    sync::Arc,
};

use super::{NetlinkSocketRaw, Read};
use crate::{ReplyError, RECV_BUF_SIZE};

use netlink_bindings::{
    builtin::Nlmsghdr,
    nlctrl,
    traits::{NetlinkRequest, Protocol},
    utils,
};

pub struct NetlinkSocket {
    pub(crate) buf: Arc<[u8; RECV_BUF_SIZE]>,
    pub(crate) cache: HashMap<&'static [u8], u16>,
    pub(crate) sock: HashMap<u16, NetlinkSocketRaw>,
    pub(crate) seq: u32,
}

impl Default for NetlinkSocket {
    fn default() -> Self {
        Self::new()
    }
}

impl NetlinkSocket {
    pub fn new() -> Self {
        Self {
            buf: Arc::new([0u8; RECV_BUF_SIZE]),
            cache: HashMap::default(),
            sock: HashMap::new(),
            seq: 1,
        }
    }

    /// Create a new [`NetlinkSocket`] from a map of [`NetlinkSocketRaw`] keyed by their corresponding `protonum`.
    pub fn from_raw_sockets(sock: HashMap<u16, NetlinkSocketRaw>) -> Self {
        Self {
            buf: Arc::new([0u8; RECV_BUF_SIZE]),
            cache: HashMap::default(),
            sock,
            seq: 1,
        }
    }

    /// Turns a [`NetlinkSocket`] back into a map of [`NetlinkSocketRaw`] keyed by their corresponding `protonum`.
    pub fn into_raw_sockets(self) -> HashMap<u16, NetlinkSocketRaw> {
        self.sock
    }

    pub(crate) fn get_socket_cached(
        cache: &mut HashMap<u16, NetlinkSocketRaw>,
        protonum: u16,
    ) -> io::Result<&mut NetlinkSocketRaw> {
        match cache.entry(protonum) {
            Entry::Occupied(sock) => Ok(sock.into_mut()),
            Entry::Vacant(ent) => {
                let sock = NetlinkSocketRaw::new(protonum)?;
                Ok(ent.insert(sock))
            }
        }
    }

    /// Reserve a sequential chunk of `seq` values, so chained messages don't
    /// get confused. A random `seq` number might be used just as well.
    pub fn reserve_seq(&mut self, len: u32) -> u32 {
        let seq = self.seq;
        self.seq = self.seq.wrapping_add(len);
        seq
    }

    #[super::strip_async]
    pub async fn request<'sock, Request>(
        &'sock mut self,
        request: &Request,
    ) -> io::Result<NetlinkReply<'sock, Request>>
    where
        Request: NetlinkRequest,
    {
        let (protonum, request_type) = match request.protocol() {
            Protocol::Raw {
                protonum,
                request_type,
            } => (protonum, request_type),
            Protocol::Generic(name) => (libc::GENL_ID_CTRL as u16, self.resolve(name).await?),
        };

        self.request_raw(request, protonum, request_type).await
    }

    #[super::strip_async]
    async fn resolve(&mut self, family_name: &'static [u8]) -> io::Result<u16> {
        if let Some(id) = self.cache.get(family_name) {
            return Ok(*id);
        }

        let mut request = nlctrl::Request::new().op_getfamily_do();
        request.encode().push_family_name_bytes(family_name);

        let Protocol::Raw {
            protonum,
            request_type,
        } = request.protocol()
        else {
            unreachable!()
        };
        assert_eq!(protonum, libc::NETLINK_GENERIC as u16);
        assert_eq!(request_type, libc::GENL_ID_CTRL as u16);

        let mut iter = self.request_raw(&request, protonum, request_type).await?;
        if let Some(reply) = iter.recv().await {
            let Ok(id) = reply?.get_family_id() else {
                return Err(ErrorKind::Unsupported.into());
            };
            self.cache.insert(family_name, id);
            return Ok(id);
        }

        Err(ErrorKind::UnexpectedEof.into())
    }

    #[super::strip_async]
    async fn request_raw<'sock, Request>(
        &'sock mut self,
        request: &Request,
        protonum: u16,
        request_type: u16,
    ) -> io::Result<NetlinkReply<'sock, Request>>
    where
        Request: NetlinkRequest,
    {
        let seq = self.reserve_seq(1);
        let sock = Self::get_socket_cached(&mut self.sock, protonum)?;

        sock.request(request, request_type, seq, &mut self.buf)
            .await
    }
}

pub(crate) struct NetlinkReplyInner {
    pub(crate) buf_offset: usize,
    pub(crate) buf_read: usize,
}

impl NetlinkReplyInner {
    #[super::not_tokio]
    #[super::strip_async]
    async fn read_buf(sock: &mut NetlinkSocketRaw, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            match sock.read(&mut buf[..]).await {
                Ok(read) => return Ok(read),
                Err(err) if err.kind() == ErrorKind::Interrupted => continue,
                Err(err) => return Err(err),
            }
        }
    }

    #[super::only_tokio]
    #[super::strip_async]
    async fn read_buf(sock: &mut NetlinkSocketRaw, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            // Some subsystems don't correctly implement io notifications, which tokio
            // runtime expects to receive before doing any actual io, hence we instead
            // always attempt an io operation first.
            let res = sock.try_read(&mut buf[..]);
            if matches!(&res, Err(err) if err.kind() == ErrorKind::WouldBlock) {
                sock.readable().await?;
                continue;
            }

            match res {
                Ok(read) => return Ok(read),
                Err(err) if err.kind() == ErrorKind::Interrupted => continue,
                Err(err) => return Err(err),
            }
        }
    }

    #[allow(clippy::type_complexity)]
    #[super::strip_async]
    pub async fn recv(
        &mut self,
        sock: &mut NetlinkSocketRaw,
        buf: &mut [u8; RECV_BUF_SIZE],
    ) -> io::Result<(u32, u16, Result<(usize, usize), ReplyError>)> {
        if self.buf_offset == self.buf_read {
            self.buf_read = Self::read_buf(sock, &mut buf[..]).await?;
            self.buf_offset = 0;
        }
        self.parse_next(buf).await
    }

    #[allow(clippy::type_complexity)]
    #[super::strip_async]
    pub(crate) async fn parse_next(
        &mut self,
        buf: &[u8; RECV_BUF_SIZE],
    ) -> io::Result<(u32, u16, Result<(usize, usize), ReplyError>)> {
        let packet = &buf[self.buf_offset..self.buf_read];

        let too_short_err = || io::Error::other("Received packet is too short");

        let Some(header) = packet.get(..Nlmsghdr::len()) else {
            return Err(too_short_err());
        };
        let header = Nlmsghdr::from_slice(header);

        let payload_start = self.buf_offset + Nlmsghdr::len();
        self.buf_offset += header.len as usize;

        match header.r#type as i32 {
            libc::NLMSG_DONE | libc::NLMSG_ERROR => {
                let Some(code) = packet.get(16..20) else {
                    return Err(too_short_err());
                };
                let code = utils::parse_i32(code).unwrap();

                let mut request_bounds = (0, 0);
                if code != 0 && header.r#type & libc::NLMSG_ERROR as u16 != 0 {
                    let Some(echo_header) = packet.get(20..(20 + Nlmsghdr::len())) else {
                        return Err(too_short_err());
                    };

                    if header.flags & libc::NLM_F_CAPPED as u16 == 0 {
                        let start = Nlmsghdr::from_slice(echo_header).len;
                        if packet.len() < 20 + start as usize {
                            return Err(too_short_err());
                        }

                        request_bounds.0 += 20 + Nlmsghdr::len() as u32;
                        request_bounds.1 += 20 + start;
                    } else {
                        request_bounds.0 += 20 + Nlmsghdr::len() as u32;
                        request_bounds.1 += 20 + Nlmsghdr::len() as u32;
                    }
                };

                let mut ext_ack_bounds = (0, 0);
                if header.flags & libc::NLM_F_ACK_TLVS as u16 != 0 {
                    ext_ack_bounds = (
                        utils::align_up(request_bounds.1 as usize, utils::NLA_ALIGNTO)
                            .min(self.buf_offset) as u32,
                        self.buf_offset as u32,
                    );
                }

                Ok((
                    header.seq,
                    header.r#type,
                    Err(ReplyError {
                        code: io::Error::from_raw_os_error(-code),
                        request_bounds,
                        ext_ack_bounds,
                        reply_buf: None,
                        chained_name_pos: None,
                        lookup: None,
                    }),
                ))
            }
            libc::NLMSG_NOOP => Ok((
                header.seq,
                header.r#type,
                Err(io::Error::other("Received NLMSG_NOOP").into()),
            )),
            libc::NLMSG_OVERRUN => Ok((
                header.seq,
                header.r#type,
                Err(io::Error::other("Received NLMSG_OVERRUN").into()),
            )),
            _ => Ok((
                header.seq,
                header.r#type,
                Ok((payload_start, self.buf_offset)),
            )),
        }
    }
}

#[must_use = "The request is sent, but not acknowledged. Consider .recv_ack(), .recv_one() or .recv()"]
pub struct NetlinkReply<'sock, Request: NetlinkRequest> {
    pub(crate) inner: NetlinkReplyInner,
    pub(crate) sock: &'sock mut NetlinkSocketRaw,
    pub(crate) buf: &'sock mut Arc<[u8; RECV_BUF_SIZE]>,
    pub(crate) seq: u32,
    pub(crate) done: bool,
    pub(crate) phantom: PhantomData<Request>,
}

impl<Request: NetlinkRequest> NetlinkReply<'_, Request> {
    #[super::strip_async]
    pub async fn recv_one(&mut self) -> Result<Request::ReplyType<'_>, ReplyError> {
        if let Some(res) = self.recv().await {
            return res;
        }
        Err(io::Error::other("Reply didn't contain data").into())
    }

    #[super::strip_async]
    pub async fn recv_ack(&mut self) -> Result<(), ReplyError> {
        if let Some(res) = self.recv().await {
            res?;
            return Err(io::Error::other("Reply isn't just an ack").into());
        }
        Ok(())
    }

    #[super::strip_async]
    pub async fn recv(&mut self) -> Option<Result<Request::ReplyType<'_>, ReplyError>> {
        if self.done {
            return None;
        }

        let buf = Arc::make_mut(self.buf);

        loop {
            match self.inner.recv(self.sock, buf).await {
                Err(io_err) => {
                    self.done = true;
                    return Some(Err(io_err.into()));
                }
                Ok((seq, _type, res)) => {
                    if seq != self.seq {
                        continue;
                    }
                    return match res {
                        Ok((l, r)) => Some(Ok(Request::decode_reply(&self.buf[l..r]))),
                        Err(mut err) => {
                            self.done = true;
                            if err.code.raw_os_error().unwrap() == 0 {
                                None
                            } else {
                                if err.has_context() {
                                    err.lookup = Some(Request::lookup);
                                    err.reply_buf = Some(self.buf.clone());
                                }
                                Some(Err(err))
                            }
                        }
                    };
                }
            };
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(unused)]
    trait SpawnCompatible: Send {}
    impl<'a> SpawnCompatible for NetlinkSocket {}
}
