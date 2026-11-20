use std::{
    fmt,
    marker::{Send, Sync},
    sync::Arc,
};

use netlink_bindings::traits::NetlinkChained;

use super::{reply::NetlinkReplyInner, Socket};
use crate::{ReplyError, RECV_BUF_SIZE};

#[must_use = "The request is sent, but not acknowledged. Consider .recv_all() or .recv()"]
pub struct NetlinkReplyChained<'sock> {
    pub(crate) inner: NetlinkReplyInner,
    pub(crate) request: &'sock (dyn NetlinkChained + Send + Sync),
    pub(crate) sock: &'sock mut Socket,
    pub(crate) buf: &'sock mut Arc<[u8; RECV_BUF_SIZE]>,
    pub(crate) done: Bits,
}

impl NetlinkReplyChained<'_> {
    #[super::strip_async]
    pub async fn recv_all(&mut self) -> Result<(), ReplyError> {
        while let Some(res) = self.recv().await {
            res?;
        }
        Ok(())
    }

    #[super::strip_async]
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
                Ok((seq, _type, res)) => {
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
                                err.chained_name_pos =
                                    Some((self.request.name(index), index as u32));
                                if err.has_context() {
                                    err.lookup = Some(self.request.lookup(index));
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
pub(crate) enum Bits {
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
    pub(crate) fn with_len(len: usize) -> Self {
        if len < 64 {
            Self::Inline(u64::MAX << (len % 64))
        } else {
            let mut vec = vec![0; len.div_ceil(64)];
            *vec.last_mut().unwrap() |= u64::MAX << (len % 64);
            Self::Vec(vec)
        }
    }

    pub(crate) fn set(&mut self, index: usize) {
        match self {
            Self::Inline(w) => *w |= 1u64 << index,
            Self::Vec(bits) => bits[index / 64] |= 1u64 << (index % 64),
        }
    }

    pub(crate) fn is_all(&self) -> bool {
        match self {
            Self::Inline(w) => *w == u64::MAX,
            Self::Vec(bits) => bits.iter().all(|w| *w == u64::MAX),
        }
    }

    pub(crate) fn set_all(&mut self) {
        match self {
            Self::Inline(w) => *w = u64::MAX,
            Self::Vec(bits) => bits.iter_mut().for_each(|w| *w = u64::MAX),
        }
    }

    pub(crate) fn count_zeros(&self) -> usize {
        match self {
            Self::Inline(w) => w.count_zeros() as usize,
            Self::Vec(bits) => bits.iter().map(|s| s.count_zeros() as usize).sum(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(unused)]
    trait SpawnCompatible: Send {}
    impl<'a> SpawnCompatible for NetlinkReplyChained<'a> {}
}
