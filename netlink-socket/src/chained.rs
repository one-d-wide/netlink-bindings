use std::{
    fmt,
    io::{self, IoSlice},
    sync::Arc,
};

use netlink_bindings::traits::NetlinkChained;

use crate::{NetlinkReplyInner, NetlinkSocket, ReplyError, Socket, RECV_BUF_SIZE};

impl NetlinkSocket {
    /// Chained requests (experimental)
    ///
    /// Some subsystems have special requirements for related requests,
    /// expecting certain types of messages to be sent within a single write
    /// operation. For example transactions in nftables subsystem.
    ///
    /// Chained requests currently don't support replies carrying data.
    #[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
    pub async fn request_chained<'a, Chained>(
        &'a mut self,
        request: &'a Chained,
    ) -> io::Result<NetlinkReplyChained<'a>>
    where
        Chained: NetlinkChained,
    {
        let sock = Self::get_socket_cached(&mut self.sock, request.protonum())?;

        Self::write_buf(sock, &[IoSlice::new(request.payload())]).await?;

        Ok(NetlinkReplyChained {
            sock,
            buf: &mut self.buf,
            request,
            inner: NetlinkReplyInner {
                buf_offset: 0,
                buf_read: 0,
            },
            done: Bits::with_len(request.chain_len()),
        })
    }
}

pub struct NetlinkReplyChained<'sock> {
    inner: NetlinkReplyInner,
    request: &'sock dyn NetlinkChained,
    sock: &'sock mut Socket,
    buf: &'sock mut Arc<[u8; RECV_BUF_SIZE]>,
    done: Bits,
}

impl NetlinkReplyChained<'_> {
    #[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
    pub async fn recv_all(&mut self) -> Result<(), ReplyError> {
        while let Some(res) = self.recv().await {
            res?;
        }
        Ok(())
    }

    #[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
    pub async fn recv(&mut self) -> Option<Result<(), ReplyError>> {
        if self.done.is_all() {
            return None;
        }

        let buf = Arc::make_mut(self.buf);

        loop {
            match self.inner.recv(self.sock, buf).await {
                Err(io_err) => {
                    self.done.set_all();
                    return Some(Err(io_err.into()));
                }
                Ok((seq, res)) => {
                    let Some(index) = self.request.get_index(seq) else {
                        continue;
                    };
                    match res {
                        Ok(_) => return Some(Ok(())),
                        Err(mut err) => {
                            if err.code.raw_os_error().unwrap() == 0 {
                                self.done.set(index);
                                return Some(Ok(()));
                            } else {
                                self.done.set_all();
                                err.chained_name = Some(self.request.name(index));
                                if err.has_context() {
                                    err.lookup = self.request.lookup(index);
                                    err.reply_buf = Some(self.buf.clone());
                                }
                                return Some(Err(err));
                            };
                        }
                    }
                }
            };
        }
    }
}

#[derive(Clone)]
enum Bits {
    Inline(u64),
    Vec(Vec<u64>),
}

impl fmt::Debug for Bits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.count_zeros();
        write!(f, "{n} replies pending")
    }
}

impl Bits {
    fn with_len(len: usize) -> Self {
        if len < 64 {
            Self::Inline(u64::MAX << (len % 64))
        } else {
            let mut vec = vec![0; len.div_ceil(64)];
            *vec.last_mut().unwrap() |= u64::MAX << (len % 64);
            Self::Vec(vec)
        }
    }

    fn set(&mut self, index: usize) {
        match self {
            Self::Inline(w) => *w |= 1u64 << index,
            Self::Vec(bits) => bits[index / 64] |= 1u64 << (index % 64),
        }
    }

    fn is_all(&self) -> bool {
        match self {
            Self::Inline(w) => *w == u64::MAX,
            Self::Vec(bits) => bits.iter().all(|w| *w == u64::MAX),
        }
    }

    fn set_all(&mut self) {
        match self {
            Self::Inline(w) => *w = u64::MAX,
            Self::Vec(bits) => bits.iter_mut().for_each(|w| *w = u64::MAX),
        }
    }

    fn count_zeros(&self) -> usize {
        match self {
            Self::Inline(w) => w.count_zeros() as usize,
            Self::Vec(bits) => bits.iter().map(|s| s.count_zeros() as usize).sum(),
        }
    }
}
