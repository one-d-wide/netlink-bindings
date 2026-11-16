use std::{
    io::{self, ErrorKind, IoSlice},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, OwnedFd, RawFd},
    sync::Arc,
};

use super::{Socket, Write};
use netlink_bindings::{builtin::Nlmsghdr, traits::NetlinkRequest};

use super::{sock::NetlinkReplyInner, NetlinkReply};
use crate::RECV_BUF_SIZE;

pub struct NetlinkSocketRaw(Socket);

impl NetlinkSocketRaw {
    pub fn new(protonum: u16) -> io::Result<Self> {
        let fd = unsafe {
            libc::socket(
                libc::AF_NETLINK,
                libc::SOCK_RAW | libc::SOCK_CLOEXEC,
                protonum as i32,
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

        Self::convert_sock(fd.into()).map(Self)
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

    #[super::strip_async]
    pub async fn request<'sock, Request>(
        &'sock mut self,
        request: &Request,
        request_type: u16,
        seq: u32,
        buf: &'sock mut Arc<[u8; RECV_BUF_SIZE]>,
    ) -> io::Result<NetlinkReply<'sock, Request>>
    where
        Request: NetlinkRequest,
    {
        let header = Nlmsghdr {
            len: Nlmsghdr::len() as u32 + request.payload().len() as u32,
            r#type: request_type,
            flags: request.flags() | libc::NLM_F_REQUEST as u16 | libc::NLM_F_ACK as u16,
            seq,
            pid: 0,
        };

        self.write_buf(&[
            IoSlice::new(header.as_slice()),
            IoSlice::new(request.payload()),
        ])
        .await?;

        Ok(NetlinkReply {
            sock: self,
            buf,
            inner: NetlinkReplyInner {
                buf_offset: 0,
                buf_read: 0,
            },
            seq: header.seq,
            done: false,
            phantom: PhantomData,
        })
    }

    #[super::not_tokio]
    #[super::strip_async]
    pub(crate) async fn write_buf(&mut self, payload: &[IoSlice<'_>]) -> io::Result<()> {
        loop {
            match self.0.write_vectored(payload).await {
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
    pub(crate) async fn write_buf(&mut self, payload: &[IoSlice<'_>]) -> io::Result<()> {
        loop {
            // Some subsystems don't correctly implement io notifications, which tokio runtime
            // expects to receive before doing any actual io, hence we instead always attempt an io
            // operation first.
            let res = self.0.try_write_vectored(payload);
            if matches!(&res, Err(err) if err.kind() == ErrorKind::WouldBlock) {
                self.0.writable().await?;
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

impl Deref for NetlinkSocketRaw {
    type Target = Socket;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NetlinkSocketRaw {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Socket> for NetlinkSocketRaw {
    fn from(value: Socket) -> Self {
        Self(value)
    }
}

impl Into<Socket> for NetlinkSocketRaw {
    fn into(self) -> Socket {
        self.0
    }
}

impl AsFd for NetlinkSocketRaw {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl AsRawFd for NetlinkSocketRaw {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}
