use std::{
    collections::{hash_map::Entry, HashMap},
    io::{self, ErrorKind, IoSlice},
    marker::PhantomData,
    os::fd::{AsRawFd, FromRawFd, OwnedFd},
    sync::Arc,
};

#[allow(unused_imports)]
use super::{
    chained::NetlinkReplyChained, reply::NetlinkReplyInner, NetlinkReply, Read, Socket, Write,
};
use crate::{ReplyError, RECV_BUF_SIZE};

use netlink_bindings::{
    builtin::Nlmsghdr,
    nlctrl,
    traits::{NetlinkChained, NetlinkRequest, Protocol},
};

pub struct NetlinkSocket {
    pub(crate) buf: Option<Arc<[u8; RECV_BUF_SIZE]>>,
    pub(crate) cache: HashMap<&'static [u8], u16>,
    pub(crate) sock: HashMap<u16, Socket>,
    pub(crate) seq: u32,
    /// Protonum of NetlinkSocketRaw. If set, it is the only socket created.
    pub(crate) protonum: Option<u16>,
}

impl Default for NetlinkSocket {
    fn default() -> Self {
        Self::new()
    }
}

impl NetlinkSocket {
    pub fn new() -> Self {
        Self {
            buf: None,
            cache: HashMap::new(),
            sock: HashMap::new(),
            seq: 1,
            protonum: None,
        }
    }

    pub(crate) fn get_socket_cached<'a>(
        cache: &'a mut HashMap<u16, Socket>,
        sock_protonum: &Option<u16>,
        protonum: u16,
    ) -> io::Result<&'a mut Socket> {
        if let Some(sock_protonum) = sock_protonum {
            if *sock_protonum != protonum {
                return Err(io::Error::other(format!(
                    "Can't send a request. Requested protonum is {protonum}, while NetlinkSocketRaw protonum is {sock_protonum}",
                ))
                .into());
            }
        }

        match cache.entry(protonum) {
            Entry::Occupied(sock) => Ok(sock.into_mut()),
            Entry::Vacant(ent) => {
                let sock = Self::get_socket_new(protonum)?;
                Ok(ent.insert(sock))
            }
        }
    }

    pub(crate) fn get_socket_new(family: u16) -> io::Result<Socket> {
        let fd = unsafe {
            libc::socket(
                libc::AF_NETLINK,
                libc::SOCK_RAW | libc::SOCK_CLOEXEC,
                family as i32,
            )
        };
        if fd < 0 {
            return Err(io::Error::from_raw_os_error(-fd));
        }
        let fd = unsafe { OwnedFd::from_raw_fd(fd) };

        // Enable extended attributes in libc::NLMSG_ERROR and libc::NLMSG_DONE
        let res = unsafe {
            libc::setsockopt(
                fd.as_raw_fd(),
                libc::SOL_NETLINK,
                libc::NETLINK_EXT_ACK,
                &1u32 as *const u32 as *const libc::c_void,
                4,
            )
        };
        if res < 0 {
            return Err(io::Error::from_raw_os_error(-res));
        }

        Self::convert_sock(fd.into())
    }

    #[super::only_async]
    fn convert_sock(sock: std::net::TcpStream) -> io::Result<Socket> {
        sock.set_nonblocking(true)?;
        Socket::try_from(sock)
    }

    #[super::only_sync]
    fn convert_sock(sock: std::net::TcpStream) -> io::Result<Socket> {
        Ok(sock)
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
        Request: NetlinkRequest + ?Sized,
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

    /// Same as `.request(&request)?.recv_ack()`
    #[super::strip_async]
    pub async fn request_ack<Request>(&mut self, request: &Request) -> Result<(), ReplyError>
    where
        Request: NetlinkRequest + ?Sized,
    {
        match self.request(request).await?.recv_inner().await {
            None => Ok(()),
            Some(Ok(_)) => Err(io::Error::other("Reply isn't just an ack").into()),
            Some(Err(err)) => Err(err),
        }
    }

    /// Same as `.request(&request)?.recv_one()`
    #[super::strip_async]
    pub async fn request_one<'sock, Request>(
        &'sock mut self,
        request: &Request,
    ) -> Result<Request::ReplyType<'sock>, ReplyError>
    where
        Request: NetlinkRequest + ?Sized + 'sock,
    {
        match self.request(request).await?.recv_inner().await {
            None => Err(io::Error::other("Reply didn't contain data").into()),
            Some(Ok((l, r))) => Ok(Request::decode_reply(&self.buf.as_mut().unwrap()[l..r])),
            Some(Err(err)) => Err(err),
        }
    }

    /// Execute a chained request
    ///
    /// Some subsystems have special requirements for related requests,
    /// expecting certain types of messages to be sent within a single write
    /// operation. For example transactions in nftables subsystem.
    ///
    /// Chained requests currently don't support replies carrying data.
    #[super::strip_async]
    pub async fn request_chained<'a, Chained>(
        &'a mut self,
        request: &'a Chained,
    ) -> io::Result<NetlinkReplyChained<'a>>
    where
        Chained: NetlinkChained + Send + Sync,
    {
        let sock = Self::get_socket_cached(&mut self.sock, &self.protonum, request.protonum())?;

        Self::write_buf(sock, &[IoSlice::new(request.payload())]).await?;

        let mut done = super::chained::Bits::with_len(request.chain_len());
        for i in 0..request.chain_len() {
            let Some(has_ack) = request.supports_ack(i) else {
                break;
            };
            if !has_ack {
                done.set(i);
            }
        }

        Ok(NetlinkReplyChained {
            sock,
            buf: self
                .buf
                .get_or_insert_with(|| Arc::new([0u8; RECV_BUF_SIZE])),
            request,
            inner: NetlinkReplyInner {
                buf_offset: 0,
                buf_read: 0,
            },
            done,
        })
    }

    #[super::strip_async]
    pub async fn request_chained_all<Chained>(
        &mut self,
        request: &Chained,
    ) -> Result<(), ReplyError>
    where
        Chained: NetlinkChained + Send + Sync,
    {
        self.request_chained(request).await?.recv_all().await
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
        Request: NetlinkRequest + ?Sized,
    {
        let seq = self.reserve_seq(1);
        let sock = Self::get_socket_cached(&mut self.sock, &self.protonum, protonum)?;

        let header = Nlmsghdr {
            len: Nlmsghdr::len() as u32 + request.payload().len() as u32,
            r#type: request_type,
            flags: request.flags() | libc::NLM_F_REQUEST as u16 | libc::NLM_F_ACK as u16,
            seq,
            pid: 0,
        };

        Self::write_buf(
            sock,
            &[
                IoSlice::new(header.as_slice()),
                IoSlice::new(request.payload()),
            ],
        )
        .await?;

        Ok(NetlinkReply {
            sock,
            buf: self
                .buf
                .get_or_insert_with(|| Arc::new([0u8; RECV_BUF_SIZE])),
            inner: NetlinkReplyInner {
                buf_offset: 0,
                buf_read: 0,
            },
            seq,
            done: false,
            phantom: PhantomData,
        })
    }

    #[super::not_tokio]
    #[super::strip_async]
    pub(crate) async fn write_buf(sock: &mut Socket, payload: &[IoSlice<'_>]) -> io::Result<()> {
        loop {
            match sock.write_vectored(payload).await {
                Ok(sent) if sent != payload.iter().map(|s| s.len()).sum() => {
                    return Err(io::Error::other("Couldn't send the whole message"));
                }
                Ok(_) => return Ok(()),
                Err(err) if err.kind() == ErrorKind::Interrupted => continue,
                Err(err) => return Err(err),
            }
        }
    }

    #[super::only_tokio]
    #[super::strip_async]
    pub(crate) async fn write_buf(sock: &mut Socket, payload: &[IoSlice<'_>]) -> io::Result<()> {
        loop {
            // Some subsystems don't correctly implement io notifications, which tokio runtime
            // expects to receive before doing any actual io, hence we instead always attempt an io
            // operation first.
            let res = sock.try_write_vectored(payload);
            if matches!(&res, Err(err) if err.kind() == ErrorKind::WouldBlock) {
                sock.writable().await?;
                continue;
            }

            match res {
                Ok(sent) if sent != payload.iter().map(|s| s.len()).sum() => {
                    return Err(io::Error::other("Couldn't send the whole message"));
                }
                Ok(_) => return Ok(()),
                Err(err) if err.kind() == ErrorKind::Interrupted => continue,
                Err(err) => return Err(err),
            }
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
