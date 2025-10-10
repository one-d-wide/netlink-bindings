use std::{
    collections::{hash_map::Entry, HashMap},
    io::{self, ErrorKind, IoSlice},
    marker::PhantomData,
    os::fd::{AsRawFd, FromRawFd, OwnedFd},
    sync::Arc,
};

#[cfg(not(feature = "async"))]
use std::{
    io::{Read, Write},
    net::TcpStream as Socket,
};

#[cfg(feature = "tokio")]
use tokio::net::TcpStream as Socket;

#[cfg(feature = "smol")]
use smol::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream as Socket,
};

use netlink_bindings::{builtin::PushNlmsghdr, nlctrl, utils, NetlinkRequest, Protocol};

mod chained;
mod error;

pub use chained::NetlinkReplyChained;
pub use error::ReplyError;

// Netlink documentation recommends max(8192, page_size)
pub const RECV_BUF_SIZE: usize = 8192;

pub struct NetlinkSocket {
    buf: Arc<[u8; RECV_BUF_SIZE]>,
    cache: HashMap<&'static [u8], u16>,
    sock: HashMap<u16, Socket>,
    seq: u32,
}

impl NetlinkSocket {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            buf: Arc::new([0u8; RECV_BUF_SIZE]),
            cache: HashMap::default(),
            sock: HashMap::new(),
            seq: 1,
        }
    }

    fn get_socket_cached(
        cache: &mut HashMap<u16, Socket>,
        protonum: u16,
    ) -> io::Result<&mut Socket> {
        match cache.entry(protonum) {
            Entry::Occupied(sock) => Ok(sock.into_mut()),
            Entry::Vacant(ent) => {
                let sock = Self::get_socket_new(protonum)?;
                Ok(ent.insert(sock))
            }
        }
    }

    fn get_socket_new(family: u16) -> io::Result<Socket> {
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
                (&1u32) as *const u32 as *const libc::c_void,
                4,
            )
        };
        if res < 0 {
            return Err(io::Error::from_raw_os_error(-res));
        }

        let sock: std::net::TcpStream = fd.into();

        #[cfg(feature = "async")]
        {
            sock.set_nonblocking(true)?;
            Socket::try_from(sock)
        }

        #[cfg(not(feature = "async"))]
        Ok(sock)
    }

    pub fn reserve_seq(&mut self, len: u32) -> u32 {
        let seq = self.seq;
        self.seq = self.seq.wrapping_add(len);
        seq
    }

    #[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
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

    #[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
    pub async fn resolve(&mut self, family_name: &'static [u8]) -> io::Result<u16> {
        if let Some(id) = self.cache.get(family_name) {
            return Ok(*id);
        }

        let mut request = nlctrl::Request::new().op_getfamily_do_request();
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

    #[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
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

        let mut header = PushNlmsghdr::new();
        header.set_len(header.as_slice().len() as u32 + request.payload().len() as u32);
        header.set_type(request_type);
        header.set_flags(request.flags() | libc::NLM_F_REQUEST as u16 | libc::NLM_F_ACK as u16);
        header.set_seq(seq);

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
            buf: &mut self.buf,
            inner: NetlinkReplyInner {
                buf_offset: 0,
                buf_read: 0,
            },
            seq: header.seq(),
            done: false,
            phantom: PhantomData,
        })
    }

    #[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
    async fn write_buf(sock: &mut Socket, payload: &[IoSlice<'_>]) -> io::Result<()> {
        loop {
            #[cfg(not(feature = "tokio"))]
            let res = sock.write_vectored(payload).await;

            #[cfg(feature = "tokio")]
            let res = loop {
                // Some subsystems don't correctly implement io notifications, which tokio runtime
                // expects to receive before doing any actual io, hence we instead always attempt an io
                // operation first.
                let res = sock.try_write_vectored(payload);
                if matches!(&res, Err(err) if err.kind() == ErrorKind::WouldBlock) {
                    sock.writable().await?;
                    continue;
                }
                break res;
            };

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

struct NetlinkReplyInner {
    buf_offset: usize,
    buf_read: usize,
}

impl NetlinkReplyInner {
    #[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
    async fn read_buf(sock: &mut Socket, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            #[cfg(not(feature = "tokio"))]
            let res = sock.read(&mut buf[..]).await;

            #[cfg(feature = "tokio")]
            let res = {
                // Some subsystems don't correctly implement io notifications, which tokio
                // runtime expects to receive before doing any actual io, hence we instead
                // always attempt an io operation first.
                let res = sock.try_read(&mut buf[..]);
                if matches!(&res, Err(err) if err.kind() == ErrorKind::WouldBlock) {
                    sock.readable().await?;
                    continue;
                }
                res
            };

            match res {
                Ok(read) => return Ok(read),
                Err(err) if err.kind() == ErrorKind::Interrupted => continue,
                Err(err) => return Err(err),
            }
        }
    }

    #[allow(clippy::type_complexity)]
    #[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
    pub async fn recv(
        &mut self,
        sock: &mut Socket,
        buf: &mut [u8; RECV_BUF_SIZE],
    ) -> io::Result<(u32, Result<(usize, usize), ReplyError>)> {
        if self.buf_offset == self.buf_read {
            self.buf_read = Self::read_buf(sock, &mut buf[..]).await?;
            self.buf_offset = 0;
        }

        let packet = &buf[self.buf_offset..self.buf_read];

        let too_short_err = || io::Error::other("Received packet is too short");

        let Some(header) = packet.get(..PushNlmsghdr::len()) else {
            return Err(too_short_err());
        };
        let header = PushNlmsghdr::new_from_slice(header).unwrap();

        let payload_start = self.buf_offset + PushNlmsghdr::len();
        self.buf_offset += header.get_len() as usize;

        match header.get_type() as i32 {
            libc::NLMSG_DONE | libc::NLMSG_ERROR => {
                let Some(code) = packet.get(16..20) else {
                    return Err(too_short_err());
                };
                let code = utils::parse_i32(code).unwrap();

                let (echo_start, echo_end) =
                    if code == 0 || header.get_type() == libc::NLMSG_DONE as u16 {
                        (20, 20)
                    } else {
                        let Some(echo_header) = packet.get(20..(20 + PushNlmsghdr::len())) else {
                            return Err(too_short_err());
                        };
                        let echo_header = PushNlmsghdr::new_from_slice(echo_header).unwrap();

                        if echo_header.flags() & libc::NLM_F_CAPPED as u16 == 0 {
                            let start = echo_header.get_len();
                            if packet.len() < start as usize + 20 {
                                return Err(too_short_err());
                            }

                            (20 + 16, 20 + start as usize)
                        } else {
                            let ext_ack_start = 20 + PushNlmsghdr::len();
                            (ext_ack_start, ext_ack_start)
                        }
                    };

                Ok((
                    header.seq(),
                    Err(ReplyError {
                        code: io::Error::from_raw_os_error(-code),
                        request_bounds: (echo_start as u32, echo_end as u32),
                        ext_ack_bounds: (echo_end as u32, self.buf_offset as u32),
                        reply_buf: None,
                        chained_name: None,
                        lookup: |_, _, _| Default::default(),
                    }),
                ))
            }
            libc::NLMSG_NOOP => Ok((
                header.seq(),
                Err(io::Error::other("Received NLMSG_NOOP").into()),
            )),
            libc::NLMSG_OVERRUN => Ok((
                header.seq(),
                Err(io::Error::other("Received NLMSG_OVERRUN").into()),
            )),
            _ => Ok((header.seq(), Ok((payload_start, self.buf_offset)))),
        }
    }
}

pub struct NetlinkReply<'sock, Request: NetlinkRequest> {
    inner: NetlinkReplyInner,
    sock: &'sock mut Socket,
    buf: &'sock mut Arc<[u8; RECV_BUF_SIZE]>,
    seq: u32,
    done: bool,
    phantom: PhantomData<Request>,
}

impl<Request: NetlinkRequest> NetlinkReply<'_, Request> {
    #[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
    pub async fn recv_one(&mut self) -> Result<Request::ReplyType<'_>, ReplyError> {
        if let Some(res) = self.recv().await {
            return res;
        }
        Err(io::Error::other("Reply didn't contain data").into())
    }

    #[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
    pub async fn recv_ack(&mut self) -> Result<(), ReplyError> {
        if let Some(res) = self.recv().await {
            res?;
            return Err(io::Error::other("Reply isn't just an ack").into());
        }
        Ok(())
    }

    #[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
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
                Ok((seq, res)) => {
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
                                    err.lookup = Request::lookup;
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
