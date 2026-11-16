#![allow(clippy::doc_lazy_continuation)]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Netlink documentation recommends max(8192, page_size)
pub const RECV_BUF_SIZE: usize = 8192;

mod error;
pub use error::ReplyError;

#[cfg(all(feature = "std", feature = "async"))]
pub use crate::std::*;

#[cfg(not(feature = "async"))]
pub use crate::std::*;

#[cfg(all(not(feature = "std"), feature = "tokio", not(feature = "smol")))]
pub use crate::tokio::*;

#[cfg(all(not(feature = "std"), not(feature = "tokio"), feature = "smol"))]
pub use crate::smol::*;

use strip_async::{keep, skip, strip_async};

#[cfg_attr(docsrs, doc(cfg(any(feature = "std", not(feature = "async")))))]
#[cfg(any(feature = "std", not(feature = "async")))]
#[path = ""]
pub mod std {
    use crate::keep as only_sync;
    use crate::keep as not_tokio;
    use crate::skip as only_async;
    use crate::skip as only_tokio;

    use crate::strip_async;

    use std::{
        io::{Read, Write},
        net::TcpStream as Socket,
    };

    pub(crate) mod chained;
    pub(crate) mod multicast;
    pub(crate) mod sock;
    pub(crate) mod sock_raw;

    pub use {
        chained::NetlinkReplyChained,
        multicast::{MulticastRecv, MulticastSocketRaw},
        sock::{NetlinkReply, NetlinkSocket},
        sock_raw::NetlinkSocketRaw,
    };
}

#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
#[cfg(feature = "tokio")]
#[path = ""]
pub mod tokio {
    use crate::keep as only_async;
    use crate::keep as only_tokio;
    use crate::skip as only_sync;
    use crate::skip as not_tokio;

    use crate::keep as strip_async;

    use tokio::{
        io::{AsyncReadExt as Read, AsyncWriteExt as Write},
        net::TcpStream as Socket,
    };

    pub(crate) mod chained;
    pub(crate) mod multicast;
    pub(crate) mod sock;
    pub(crate) mod sock_raw;

    pub use {
        chained::NetlinkReplyChained,
        multicast::{MulticastRecv, MulticastSocketRaw},
        sock::{NetlinkReply, NetlinkSocket},
        sock_raw::NetlinkSocketRaw,
    };
}

#[cfg_attr(docsrs, doc(cfg(feature = "smol")))]
#[cfg(feature = "smol")]
#[path = ""]
pub mod smol {
    use crate::keep as only_async;
    use crate::keep as not_tokio;
    use crate::skip as only_tokio;
    use crate::skip as only_sync;

    use crate::keep as strip_async;

    use smol::io::{AsyncReadExt as Read, AsyncWriteExt as Write};
    type Socket = smol::Async<std::net::TcpStream>;

    pub(crate) mod chained;
    pub(crate) mod multicast;
    pub(crate) mod sock;
    pub(crate) mod sock_raw;

    pub use {
        chained::NetlinkReplyChained,
        multicast::{MulticastRecv, MulticastSocketRaw},
        sock::{NetlinkReply, NetlinkSocket},
        sock_raw::NetlinkSocketRaw,
    };
}
