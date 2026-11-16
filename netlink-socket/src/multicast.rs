use std::{io, mem::MaybeUninit, os::fd::AsRawFd, sync::Arc};

use netlink_bindings::utils;

use super::sock::NetlinkReplyInner;
#[allow(unused_imports)]
use super::{NetlinkSocketRaw, Read, Write};
use crate::{ReplyError, RECV_BUF_SIZE};

#[derive(Debug, Clone)]
pub struct MulticastRecv {
    pub multicast_group: u32,
    pub message_type: u16,
}

pub struct MulticastSocketRaw {
    buf: Arc<[u8; RECV_BUF_SIZE]>,
    sock: NetlinkSocketRaw,
    reply: NetlinkReplyInner,
    last_group: Option<u32>,
}

struct WrapSendUnsafe<T>(T);
unsafe impl<T> Send for WrapSendUnsafe<T> {}

type WrapMsghdr = WrapSendUnsafe<libc::msghdr>;

impl MulticastSocketRaw {
    pub fn new(protonum: u16) -> io::Result<Self> {
        let sock = NetlinkSocketRaw::new(protonum)?;

        // Enable multicast group number via recvmsg ancillary messages
        let res = unsafe {
            libc::setsockopt(
                sock.as_raw_fd(),
                libc::SOL_NETLINK,
                libc::NETLINK_PKTINFO,
                &1u32 as *const u32 as *const libc::c_void,
                4,
            )
        };
        if res < 0 {
            return Err(io::Error::from_raw_os_error(-res));
        }

        let mut buf: libc::sockaddr_nl = unsafe { std::mem::zeroed() };
        buf.nl_family = libc::AF_NETLINK as u16;
        buf.nl_groups = 0;

        let res = unsafe {
            libc::bind(
                sock.as_raw_fd(),
                &buf as *const _ as *const libc::sockaddr,
                std::mem::size_of_val(&buf) as libc::socklen_t,
            )
        };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(Self {
            buf: Arc::new([0u8; RECV_BUF_SIZE]),
            sock,
            reply: NetlinkReplyInner {
                buf_offset: 0,
                buf_read: 0,
            },
            last_group: None,
        })
    }

    pub fn listen(&mut self, group_id: u32) -> io::Result<()> {
        let res = unsafe {
            libc::setsockopt(
                self.sock.as_raw_fd(),
                libc::SOL_NETLINK,
                libc::NETLINK_ADD_MEMBERSHIP,
                &group_id as *const u32 as *const libc::c_void,
                4,
            )
        };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    #[super::strip_async]
    pub async fn recv(&mut self) -> Result<(MulticastRecv, &[u8]), ReplyError> {
        let buf = Arc::make_mut(&mut self.buf);

        loop {
            if self.reply.buf_offset == self.reply.buf_read {
                let read = Self::read_buf(&self.sock, buf, &mut self.last_group).await?;
                self.reply.buf_read = read;
                self.reply.buf_offset = 0;
            }

            match self.reply.parse_next(buf).await {
                Err(io_err) => {
                    return Err(io_err.into());
                }
                Ok((_seq, message_type, res)) => {
                    // Seq number is mostly, but not always, 0. For example, removing a network
                    // interface on linux 6.1 creates a multicast notification with the same seq
                    // number as in the request. We don't allow sending requests on the multicast
                    // socket, so the only incoming messages here are notifications.

                    let Some(multicast_group) = self.last_group else {
                        continue;
                    };

                    match res {
                        Ok((l, r)) => {
                            return Ok((
                                MulticastRecv {
                                    multicast_group,
                                    message_type,
                                },
                                &self.buf[l..r],
                            ));
                        }
                        Err(mut err) => {
                            if err.code.raw_os_error().unwrap() == 0 {
                                continue;
                            }

                            if err.has_context() {
                                // err.lookup = Request::lookup;
                                err.reply_buf = Some(self.buf.clone());
                            }

                            return Err(err);
                        }
                    };
                }
            };
        }
    }

    #[super::strip_async]
    async fn read_buf(
        sock: &NetlinkSocketRaw,
        buf: &mut [u8],
        last_group: &mut Option<u32>,
    ) -> Result<usize, ReplyError> {
        let mut iov = WrapSendUnsafe(libc::iovec {
            iov_base: buf.as_mut_ptr() as *mut libc::c_void,
            iov_len: buf.len(),
        });

        const CONTROL_BUF_LEN: usize = 128;
        let mut control_buf = MaybeUninit::<[u8; CONTROL_BUF_LEN]>::uninit();

        // Use zeroed init + field assignment to avoid private-field struct
        // literal restrictions on musl targets (__pad1, __pad2 in msghdr).
        let mut msghdr: libc::msghdr = unsafe { std::mem::zeroed() };
        msghdr.msg_name = std::ptr::null_mut();
        msghdr.msg_namelen = 0;
        msghdr.msg_iov = &raw mut iov.0;
        msghdr.msg_iovlen = 1;
        msghdr.msg_control = control_buf.as_mut_ptr() as *mut libc::c_void;
        // msg_controllen is usize on glibc but u32 on musl - let the compiler infer.
        msghdr.msg_controllen = CONTROL_BUF_LEN as _;
        msghdr.msg_flags = 0;

        let mut msghdr = WrapSendUnsafe(msghdr);
        let read = Self::recvmsg(&sock, &mut msghdr).await?;

        *last_group = None;
        unsafe {
            let msghdr_ptr = &raw const msghdr.0;
            let mut cmsg_ptr: *const libc::cmsghdr = libc::CMSG_FIRSTHDR(msghdr_ptr);
            while !cmsg_ptr.is_null() {
                let libc::cmsghdr {
                        cmsg_len,
                        cmsg_level,
                        cmsg_type,
                        .. // ignore __pad1 on musl
                    } = *cmsg_ptr;

                match (cmsg_level, cmsg_type) {
                    (libc::SOL_NETLINK, libc::NETLINK_PKTINFO) => {
                        let data = std::slice::from_raw_parts(
                            libc::CMSG_DATA(cmsg_ptr),
                            cmsg_len as usize - libc::CMSG_LEN(0) as usize,
                        );
                        *last_group = Some(utils::parse_u32(&data[..4]).unwrap());
                    }
                    _ => {}
                }

                cmsg_ptr = libc::CMSG_NXTHDR(msghdr_ptr, cmsg_ptr);
            }
        }

        Ok(read)
    }

    fn recvmsg_sync(sock: &NetlinkSocketRaw, msghdr: &mut WrapMsghdr) -> io::Result<usize> {
        unsafe {
            let res = libc::recvmsg(sock.as_raw_fd(), &mut msghdr.0, 0);
            if res < 0 {
                return Err(io::Error::last_os_error());
            }
            Ok(res as usize)
        }
    }

    #[super::only_sync]
    fn recvmsg(sock: &NetlinkSocketRaw, msghdr: &mut WrapMsghdr) -> io::Result<usize> {
        Self::recvmsg_sync(sock, msghdr)
    }

    #[super::only_async]
    #[super::not_tokio]
    async fn recvmsg(sock: &NetlinkSocketRaw, msghdr: &mut WrapMsghdr) -> io::Result<usize> {
        loop {
            match Self::recvmsg_sync(sock, msghdr) {
                Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                    sock.readable().await?;
                    continue;
                }
                res => return res,
            }
        }
    }

    #[super::only_async]
    #[super::only_tokio]
    async fn recvmsg(sock: &NetlinkSocketRaw, msghdr: &mut WrapMsghdr) -> io::Result<usize> {
        // Tokio attempts to be clever and doesn't poll the fd if it hasn't seen EAGAIN first hand
        sock.async_io(tokio::io::Interest::READABLE, || {
            Self::recvmsg_sync(sock, msghdr)
        })
        .await
    }
}
