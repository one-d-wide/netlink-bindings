use std::{
    io::{self, ErrorKind},
    marker::PhantomData,
    sync::Arc,
};

#[allow(unused_imports)]
use super::{Read, Socket, Write};
use crate::{ReplyError, RECV_BUF_SIZE};

use netlink_bindings::{builtin::Nlmsghdr, traits::NetlinkRequest, utils};

pub(crate) struct NetlinkReplyInner {
    pub(crate) buf_offset: usize,
    pub(crate) buf_read: usize,
}

impl NetlinkReplyInner {
    #[super::not_tokio]
    #[super::strip_async]
    async fn read_buf(sock: &mut Socket, buf: &mut [u8]) -> io::Result<usize> {
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
    async fn read_buf(sock: &mut Socket, buf: &mut [u8]) -> io::Result<usize> {
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
        sock: &mut Socket,
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
pub struct NetlinkReply<'sock, Request: NetlinkRequest + ?Sized> {
    pub(crate) inner: NetlinkReplyInner,
    pub(crate) sock: &'sock mut Socket,
    pub(crate) buf: &'sock mut Arc<[u8; RECV_BUF_SIZE]>,
    pub(crate) seq: u32,
    pub(crate) done: bool,
    pub(crate) phantom: PhantomData<fn() -> Request>,
}

impl<Request: NetlinkRequest + ?Sized> NetlinkReply<'_, Request> {
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
        match self.recv_inner().await {
            None => None,
            Some(Ok((l, r))) => Some(Ok(Request::decode_reply(&self.buf[l..r]))),
            Some(Err(err)) => Some(Err(err)),
        }
    }

    #[super::strip_async]
    pub(crate) async fn recv_inner(&mut self) -> Option<Result<(usize, usize), ReplyError>> {
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
                        Ok((l, r)) => Some(Ok((l, r))),
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
