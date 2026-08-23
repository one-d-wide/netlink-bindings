#![allow(clippy::doc_lazy_continuation)]
#![cfg_attr(not(proc_macro), doc = include_str!("../README.md"))]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use ip_route_proc_macro::{ip, ip_dump};

pub mod utils;

#[doc(hidden)]
pub mod reexports {
    pub use crate as ip_route;
    pub use ip_route_proc_macro;
    pub use ipnet;
    pub use libc;
    pub use netlink_bindings;
    pub use netlink_socket2;
    pub use std;
}
