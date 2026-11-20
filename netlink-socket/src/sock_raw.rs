use std::{
    io,
    ops::{Deref, DerefMut},
    os::fd::{AsFd, AsRawFd, BorrowedFd, OwnedFd, RawFd},
};

use super::NetlinkSocket;
#[allow(unused_imports)]
use super::{Read, Socket, Write};

#[repr(transparent)]
pub struct NetlinkSocketRaw {
    pub(crate) inner: NetlinkSocket,
}

impl NetlinkSocketRaw {
    pub fn new(protonum: u16) -> io::Result<Self> {
        Ok(Self::new_from_sock(
            NetlinkSocket::get_socket_new(protonum)?,
            protonum,
        ))
    }
}

impl Deref for NetlinkSocketRaw {
    type Target = NetlinkSocket;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for NetlinkSocketRaw {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsFd for NetlinkSocketRaw {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.as_sock().as_fd()
    }
}

impl AsRawFd for NetlinkSocketRaw {
    fn as_raw_fd(&self) -> RawFd {
        self.as_sock().as_raw_fd()
    }
}

#[super::only_sync]
impl NetlinkSocketRaw {
    pub fn into_fd(self) -> io::Result<OwnedFd> {
        Ok(self.into_sock().into())
    }

    pub fn from_fd(fd: OwnedFd) -> io::Result<Self> {
        Self::from_sock(Socket::from(fd))
    }
}

#[super::only_tokio]
impl NetlinkSocketRaw {
    pub fn into_fd(self) -> io::Result<OwnedFd> {
        Ok(self.into_sock().into_std()?.into())
    }

    pub fn from_fd(fd: OwnedFd) -> io::Result<Self> {
        Self::from_sock(Socket::from_std(fd.into())?)
    }
}

#[super::only_smol]
impl NetlinkSocketRaw {
    pub fn into_fd(self) -> io::Result<OwnedFd> {
        Ok(self.into_sock().into_inner()?.into())
    }

    pub fn from_fd(fd: OwnedFd) -> io::Result<Self> {
        Self::from_sock(Socket::new(fd.into())?)
    }
}

impl NetlinkSocketRaw {
    fn new_from_sock(sock: Socket, protonum: u16) -> Self {
        let mut inner = NetlinkSocket::new();
        inner.protonum = Some(protonum);
        inner.sock.insert(protonum, sock);
        Self { inner }
    }

    fn as_sock(&self) -> &Socket {
        let protonum = self.inner.protonum.unwrap();
        self.inner.sock.get(&protonum).unwrap()
    }

    fn into_sock(mut self) -> Socket {
        let protonum = self.inner.protonum.unwrap();
        self.inner.sock.remove(&protonum).unwrap()
    }

    fn from_sock(sock: Socket) -> io::Result<Self> {
        let mut optval: i32 = 0;
        let mut optlen: libc::socklen_t = 4;
        let res = unsafe {
            libc::getsockopt(
                sock.as_raw_fd(),
                libc::SOL_SOCKET,
                libc::SO_PROTOCOL,
                &raw mut optval as *mut libc::c_void,
                &raw mut optlen,
            )
        };
        if res < 0 {
            return Err(io::Error::from_raw_os_error(-res));
        }

        if optval < 0 || optval > u16::MAX as i32 {
            return Err(io::Error::other(format!(
                "Invalid socket protonum {optval}"
            )));
        }

        Ok(Self::new_from_sock(sock, optval as u16))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use netlink_bindings::nlctrl;

    #[test]
    #[should_panic(
        expected = "Can't send a request. Requested protonum is 16, while NetlinkSocketRaw protonum is 0"
    )]
    fn wrong_protonum() {
        let mut sock = NetlinkSocketRaw::new(0).unwrap();
        let request = nlctrl::Request::new().op_getfamily_dump();
        sock.request_one(&request).unwrap();
    }

    #[test]
    fn correct_protonum() {
        let mut sock = NetlinkSocketRaw::new(16).unwrap();
        let request = nlctrl::Request::new().op_getfamily_dump();
        sock.request_one(&request).unwrap();
    }

    #[allow(unused)]
    trait SpawnCompatible: Send {}
    impl<'a> SpawnCompatible for NetlinkSocketRaw {}
}
