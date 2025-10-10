use std::{fmt, io, sync::Arc};

use netlink_bindings::{
    builtin::{NlmsgerrAttrs, PushNlmsghdr},
    traits::LookupFn,
    utils::{ErrorContext, Iterable},
};

use crate::RECV_BUF_SIZE;

// For an error type to be convenient to use it has to be unconstrained by
// lifetime bounds and generics, so it can be freely passed around in a call
// chain, therefore the data buffers are ref counter.
pub struct ReplyError {
    pub(crate) code: io::Error,
    pub(crate) reply_buf: Option<Arc<[u8; RECV_BUF_SIZE]>>,
    pub(crate) ext_ack_bounds: (u32, u32),
    pub(crate) request_bounds: (u32, u32),
    pub(crate) lookup: LookupFn,
    pub(crate) chained_name: Option<&'static str>,
}

impl From<ErrorContext> for ReplyError {
    fn from(value: ErrorContext) -> Self {
        Self {
            code: io::Error::other(value),
            reply_buf: None,
            request_bounds: (0, 0),
            ext_ack_bounds: (0, 0),
            lookup: |_, _, _| Default::default(),
            chained_name: None,
        }
    }
}

impl From<io::Error> for ReplyError {
    fn from(value: io::Error) -> Self {
        Self {
            code: value,
            reply_buf: None,
            request_bounds: (0, 0),
            ext_ack_bounds: (0, 0),
            lookup: |_, _, _| Default::default(),
            chained_name: None,
        }
    }
}

impl ReplyError {
    pub fn as_io_error(&self) -> &io::Error {
        &self.code
    }

    pub fn ext_ack(&self) -> Option<Iterable<'_, NlmsgerrAttrs<'_>>> {
        let Some(reply_buf) = &self.reply_buf else {
            return None;
        };
        let (l, r) = self.ext_ack_bounds;
        Some(NlmsgerrAttrs::new(&reply_buf[l as usize..r as usize]))
    }

    pub fn request(&self) -> Option<&[u8]> {
        let Some(reply_buf) = &self.reply_buf else {
            return None;
        };
        let (l, r) = self.request_bounds;
        Some(&reply_buf[l as usize..r as usize])
    }

    pub(crate) fn has_context(&self) -> bool {
        let mut res = false;
        let (l, r) = self.ext_ack_bounds;
        res |= l != r;

        let (l, r) = self.request_bounds;
        res |= l != r;

        res
    }
}

impl From<ReplyError> for io::Error {
    fn from(value: ReplyError) -> Self {
        value.code
    }
}

impl std::error::Error for ReplyError {}

impl fmt::Debug for ReplyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for ReplyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code)?;

        let Some(ext_ack) = self.ext_ack() else {
            return Ok(());
        };

        if let Ok(msg) = ext_ack.get_msg() {
            f.write_str(": ")?;
            match msg.to_str() {
                Ok(m) => write!(f, "{m}")?,
                Err(_) => write!(f, "{msg:?}")?,
            }
        }

        if let Some(chained) = self.chained_name {
            write!(f, " in {chained:?}")?;
        }

        if let Ok(missing_offset) = ext_ack.get_missing_nest() {
            let missing_attr = ext_ack.get_missing_type().ok();

            let (trace, attr) = (self.lookup)(
                self.request().unwrap(),
                missing_offset as usize - PushNlmsghdr::len(),
                missing_attr,
            );

            if let Some(attr) = attr {
                write!(f, ": missing {attr:?}")?;
            }
            for (attrs, _) in trace.iter() {
                write!(f, " in {attrs:?}")?;
            }
        }

        if let Ok(invalid_offset) = ext_ack.get_offset() {
            let (trace, _) = (self.lookup)(
                self.request().unwrap(),
                invalid_offset as usize - PushNlmsghdr::len(),
                None,
            );

            if let Some((attr, _)) = trace.first() {
                write!(f, ": attribute {attr:?}")?;
            }
            for (attrs, _) in trace.iter().skip(1) {
                write!(f, " in {attrs:?}")?;
            }
            if let Ok(policy) = ext_ack.get_policy() {
                write!(f, ": {policy:?}")?;
            }
        }

        if ext_ack.get_buf().is_empty() {
            write!(f, " (no extended ack)")?;
        }

        Ok(())
    }
}
