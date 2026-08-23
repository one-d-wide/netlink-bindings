#![allow(clippy::all)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]
#![allow(unreachable_code)]
#![allow(unreachable_patterns)]
use netlink_bindings::{
    builtin::{BuiltinNfgenmsg, Nlmsghdr},
    traits::{NetlinkRequest, Protocol},
};
use std::cell::Cell;
use std::fmt::Debug;
#[derive(Clone)]
pub struct ReverseLookup {
    pub header: Nlmsghdr,
    pub proto: Protocol,
    pub value: u16,
    pub request_value: Option<u16>,
    pub is_dump: bool,
    pub buf: Vec<u8>,
    pub last_filter: Cell<Option<usize>>,
}
#[allow(unused)]
fn consider(fmt: &mut std::fmt::Formatter<'_>, proto: &str) -> std::fmt::Result {
    write!(
        fmt,
        "Protocol {0:?} not enabled, consider --features={0}",
        proto
    )
}
impl Debug for ReverseLookup {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            proto,
            value,
            request_value,
            is_dump,
            buf,
            last_filter,
            ..
        } = self;
        let (proto, value, request_value, is_dump) = (
            proto.clone(),
            value.clone(),
            request_value.clone(),
            is_dump.clone(),
        );
        let last_filter_val = last_filter.take();
        match proto {
            Protocol::Raw { protonum, .. } => {
                if protonum == 0u16 {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "rt-addr")]
                    if let (20u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_addr::OpNewaddrDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-addr"))]
                    if let (20u16, None, false) = pat {
                        return consider(fmt, "rt-addr");
                    }
                    #[cfg(feature = "rt-addr")]
                    if let (20u16, Some(20u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_addr::OpNewaddrDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-addr"))]
                    if let (20u16, Some(20u16), false) = pat {
                        return consider(fmt, "rt-addr");
                    }
                    #[cfg(feature = "rt-addr")]
                    if let (21u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_addr::OpDeladdrDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-addr"))]
                    if let (21u16, None, false) = pat {
                        return consider(fmt, "rt-addr");
                    }
                    #[cfg(feature = "rt-addr")]
                    if let (21u16, Some(21u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_addr::OpDeladdrDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-addr"))]
                    if let (21u16, Some(21u16), false) = pat {
                        return consider(fmt, "rt-addr");
                    }
                    #[cfg(feature = "rt-addr")]
                    if let (22u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_addr::OpGetaddrDump::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-addr"))]
                    if let (22u16, None, true) = pat {
                        return consider(fmt, "rt-addr");
                    }
                    #[cfg(feature = "rt-addr")]
                    if let (20u16, Some(22u16), true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_addr::OpGetaddrDump::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-addr"))]
                    if let (20u16, Some(22u16), true) = pat {
                        return consider(fmt, "rt-addr");
                    }
                    #[cfg(feature = "rt-addr")]
                    if let (58u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_addr::OpGetmulticastDump::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-addr"))]
                    if let (58u16, None, true) = pat {
                        return consider(fmt, "rt-addr");
                    }
                    #[cfg(feature = "rt-addr")]
                    if let (58u16, Some(58u16), true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_addr::OpGetmulticastDump::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-addr"))]
                    if let (58u16, Some(58u16), true) = pat {
                        return consider(fmt, "rt-addr");
                    }
                    #[cfg(feature = "rt-addr")]
                    if let (58u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_addr::OpGetmulticastDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-addr"))]
                    if let (58u16, None, false) = pat {
                        return consider(fmt, "rt-addr");
                    }
                    #[cfg(feature = "rt-addr")]
                    if let (58u16, Some(58u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_addr::OpGetmulticastDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-addr"))]
                    if let (58u16, Some(58u16), false) = pat {
                        return consider(fmt, "rt-addr");
                    }
                    #[cfg(feature = "rt-link")]
                    if let (16u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_link::OpNewlinkDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-link"))]
                    if let (16u16, None, false) = pat {
                        return consider(fmt, "rt-link");
                    }
                    #[cfg(feature = "rt-link")]
                    if let (16u16, Some(16u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_link::OpNewlinkDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-link"))]
                    if let (16u16, Some(16u16), false) = pat {
                        return consider(fmt, "rt-link");
                    }
                    #[cfg(feature = "rt-link")]
                    if let (17u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_link::OpDellinkDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-link"))]
                    if let (17u16, None, false) = pat {
                        return consider(fmt, "rt-link");
                    }
                    #[cfg(feature = "rt-link")]
                    if let (17u16, Some(17u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_link::OpDellinkDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-link"))]
                    if let (17u16, Some(17u16), false) = pat {
                        return consider(fmt, "rt-link");
                    }
                    #[cfg(feature = "rt-link")]
                    if let (18u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_link::OpGetlinkDump::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-link"))]
                    if let (18u16, None, true) = pat {
                        return consider(fmt, "rt-link");
                    }
                    #[cfg(feature = "rt-link")]
                    if let (16u16, Some(18u16), true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_link::OpGetlinkDump::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-link"))]
                    if let (16u16, Some(18u16), true) = pat {
                        return consider(fmt, "rt-link");
                    }
                    #[cfg(feature = "rt-link")]
                    if let (18u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_link::OpGetlinkDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-link"))]
                    if let (18u16, None, false) = pat {
                        return consider(fmt, "rt-link");
                    }
                    #[cfg(feature = "rt-link")]
                    if let (16u16, Some(18u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_link::OpGetlinkDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-link"))]
                    if let (16u16, Some(18u16), false) = pat {
                        return consider(fmt, "rt-link");
                    }
                    #[cfg(feature = "rt-link")]
                    if let (19u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_link::OpSetlinkDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-link"))]
                    if let (19u16, None, false) = pat {
                        return consider(fmt, "rt-link");
                    }
                    #[cfg(feature = "rt-link")]
                    if let (19u16, Some(19u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_link::OpSetlinkDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-link"))]
                    if let (19u16, Some(19u16), false) = pat {
                        return consider(fmt, "rt-link");
                    }
                    #[cfg(feature = "rt-link")]
                    if let (94u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_link::OpGetstatsDump::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-link"))]
                    if let (94u16, None, true) = pat {
                        return consider(fmt, "rt-link");
                    }
                    #[cfg(feature = "rt-link")]
                    if let (92u16, Some(94u16), true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_link::OpGetstatsDump::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-link"))]
                    if let (92u16, Some(94u16), true) = pat {
                        return consider(fmt, "rt-link");
                    }
                    #[cfg(feature = "rt-link")]
                    if let (94u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_link::OpGetstatsDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-link"))]
                    if let (94u16, None, false) = pat {
                        return consider(fmt, "rt-link");
                    }
                    #[cfg(feature = "rt-link")]
                    if let (92u16, Some(94u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_link::OpGetstatsDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-link"))]
                    if let (92u16, Some(94u16), false) = pat {
                        return consider(fmt, "rt-link");
                    }
                    #[cfg(feature = "rt-neigh")]
                    if let (28u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_neigh::OpNewneighDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-neigh"))]
                    if let (28u16, None, false) = pat {
                        return consider(fmt, "rt-neigh");
                    }
                    #[cfg(feature = "rt-neigh")]
                    if let (28u16, Some(28u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_neigh::OpNewneighDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-neigh"))]
                    if let (28u16, Some(28u16), false) = pat {
                        return consider(fmt, "rt-neigh");
                    }
                    #[cfg(feature = "rt-neigh")]
                    if let (29u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_neigh::OpDelneighDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-neigh"))]
                    if let (29u16, None, false) = pat {
                        return consider(fmt, "rt-neigh");
                    }
                    #[cfg(feature = "rt-neigh")]
                    if let (29u16, Some(29u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_neigh::OpDelneighDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-neigh"))]
                    if let (29u16, Some(29u16), false) = pat {
                        return consider(fmt, "rt-neigh");
                    }
                    #[cfg(feature = "rt-neigh")]
                    if let (30u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_neigh::OpGetneighDump::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-neigh"))]
                    if let (30u16, None, true) = pat {
                        return consider(fmt, "rt-neigh");
                    }
                    #[cfg(feature = "rt-neigh")]
                    if let (28u16, Some(30u16), true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_neigh::OpGetneighDump::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-neigh"))]
                    if let (28u16, Some(30u16), true) = pat {
                        return consider(fmt, "rt-neigh");
                    }
                    #[cfg(feature = "rt-neigh")]
                    if let (30u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_neigh::OpGetneighDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-neigh"))]
                    if let (30u16, None, false) = pat {
                        return consider(fmt, "rt-neigh");
                    }
                    #[cfg(feature = "rt-neigh")]
                    if let (28u16, Some(30u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_neigh::OpGetneighDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-neigh"))]
                    if let (28u16, Some(30u16), false) = pat {
                        return consider(fmt, "rt-neigh");
                    }
                    #[cfg(feature = "rt-neigh")]
                    if let (66u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_neigh::OpGetneightblDump::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-neigh"))]
                    if let (66u16, None, true) = pat {
                        return consider(fmt, "rt-neigh");
                    }
                    #[cfg(feature = "rt-neigh")]
                    if let (64u16, Some(66u16), true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_neigh::OpGetneightblDump::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-neigh"))]
                    if let (64u16, Some(66u16), true) = pat {
                        return consider(fmt, "rt-neigh");
                    }
                    #[cfg(feature = "rt-neigh")]
                    if let (67u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_neigh::OpSetneightblDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-neigh"))]
                    if let (67u16, None, false) = pat {
                        return consider(fmt, "rt-neigh");
                    }
                    #[cfg(feature = "rt-neigh")]
                    if let (67u16, Some(67u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_neigh::OpSetneightblDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-neigh"))]
                    if let (67u16, Some(67u16), false) = pat {
                        return consider(fmt, "rt-neigh");
                    }
                    #[cfg(feature = "rt-route")]
                    if let (26u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_route::OpGetrouteDump::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-route"))]
                    if let (26u16, None, true) = pat {
                        return consider(fmt, "rt-route");
                    }
                    #[cfg(feature = "rt-route")]
                    if let (24u16, Some(26u16), true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_route::OpGetrouteDump::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-route"))]
                    if let (24u16, Some(26u16), true) = pat {
                        return consider(fmt, "rt-route");
                    }
                    #[cfg(feature = "rt-route")]
                    if let (26u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_route::OpGetrouteDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-route"))]
                    if let (26u16, None, false) = pat {
                        return consider(fmt, "rt-route");
                    }
                    #[cfg(feature = "rt-route")]
                    if let (24u16, Some(26u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_route::OpGetrouteDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-route"))]
                    if let (24u16, Some(26u16), false) = pat {
                        return consider(fmt, "rt-route");
                    }
                    #[cfg(feature = "rt-route")]
                    if let (24u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_route::OpNewrouteDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-route"))]
                    if let (24u16, None, false) = pat {
                        return consider(fmt, "rt-route");
                    }
                    #[cfg(feature = "rt-route")]
                    if let (24u16, Some(24u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_route::OpNewrouteDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-route"))]
                    if let (24u16, Some(24u16), false) = pat {
                        return consider(fmt, "rt-route");
                    }
                    #[cfg(feature = "rt-route")]
                    if let (25u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_route::OpDelrouteDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-route"))]
                    if let (25u16, None, false) = pat {
                        return consider(fmt, "rt-route");
                    }
                    #[cfg(feature = "rt-route")]
                    if let (25u16, Some(25u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_route::OpDelrouteDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-route"))]
                    if let (25u16, Some(25u16), false) = pat {
                        return consider(fmt, "rt-route");
                    }
                    #[cfg(feature = "rt-rule")]
                    if let (32u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_rule::OpNewruleDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-rule"))]
                    if let (32u16, None, false) = pat {
                        return consider(fmt, "rt-rule");
                    }
                    #[cfg(feature = "rt-rule")]
                    if let (32u16, Some(32u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_rule::OpNewruleDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-rule"))]
                    if let (32u16, Some(32u16), false) = pat {
                        return consider(fmt, "rt-rule");
                    }
                    #[cfg(feature = "rt-rule")]
                    if let (33u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_rule::OpDelruleDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-rule"))]
                    if let (33u16, None, false) = pat {
                        return consider(fmt, "rt-rule");
                    }
                    #[cfg(feature = "rt-rule")]
                    if let (33u16, Some(33u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_rule::OpDelruleDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-rule"))]
                    if let (33u16, Some(33u16), false) = pat {
                        return consider(fmt, "rt-rule");
                    }
                    #[cfg(feature = "rt-rule")]
                    if let (34u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_rule::OpGetruleDump::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-rule"))]
                    if let (34u16, None, true) = pat {
                        return consider(fmt, "rt-rule");
                    }
                    #[cfg(feature = "rt-rule")]
                    if let (32u16, Some(34u16), true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_rule::OpGetruleDump::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "rt-rule"))]
                    if let (32u16, Some(34u16), true) = pat {
                        return consider(fmt, "rt-rule");
                    }
                    #[cfg(feature = "tc")]
                    if let (36u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpNewqdiscDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (36u16, None, false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (36u16, Some(36u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpNewqdiscDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (36u16, Some(36u16), false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (37u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpDelqdiscDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (37u16, None, false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (37u16, Some(37u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpDelqdiscDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (37u16, Some(37u16), false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (38u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpGetqdiscDump::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (38u16, None, true) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (36u16, Some(38u16), true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpGetqdiscDump::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (36u16, Some(38u16), true) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (38u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpGetqdiscDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (38u16, None, false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (36u16, Some(38u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpGetqdiscDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (36u16, Some(38u16), false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (40u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpNewtclassDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (40u16, None, false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (40u16, Some(40u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpNewtclassDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (40u16, Some(40u16), false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (41u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpDeltclassDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (41u16, None, false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (41u16, Some(41u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpDeltclassDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (41u16, Some(41u16), false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (42u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpGettclassDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (42u16, None, false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (40u16, Some(42u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpGettclassDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (40u16, Some(42u16), false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (44u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpNewtfilterDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (44u16, None, false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (44u16, Some(44u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpNewtfilterDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (44u16, Some(44u16), false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (45u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpDeltfilterDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (45u16, None, false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (45u16, Some(45u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpDeltfilterDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (45u16, Some(45u16), false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (46u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpGettfilterDump::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (46u16, None, true) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (44u16, Some(46u16), true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpGettfilterDump::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (44u16, Some(46u16), true) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (46u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpGettfilterDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (46u16, None, false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (44u16, Some(46u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpGettfilterDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (44u16, Some(46u16), false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (100u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpNewchainDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (100u16, None, false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (100u16, Some(100u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpNewchainDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (100u16, Some(100u16), false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (101u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpDelchainDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (101u16, None, false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (101u16, Some(101u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpDelchainDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (101u16, Some(101u16), false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (102u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpGetchainDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (102u16, None, false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (100u16, Some(102u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpGetchainDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (100u16, Some(102u16), false) = pat {
                        return consider(fmt, "tc");
                    }
                    write!(
                        fmt,
                        "(Unknown operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                    )?;
                    return Ok(());
                }
                if protonum == 4u16 {
                    let pat = (value, request_value, is_dump);
                    let mut pass = false;
                    let filter: fn(&[u8]) -> bool = |buf| buf[1] == 6;
                    if request_value.is_none()
                        && last_filter_val.is_none_or(|l| l == 0usize)
                        && filter(buf)
                    {
                        last_filter.set(Some(0usize));
                        pass = true;
                    }
                    if request_value.is_some() && last_filter_val.is_some_and(|l| l == 0usize) {
                        last_filter.set(Some(0usize));
                        pass = true;
                    }
                    let filter: fn(&[u8]) -> bool = |buf| buf[0] == 2 || buf[0] == 10;
                    pass = filter(buf) && pass;
                    if pass {
                        #[cfg(feature = "inet-diag")]
                        if let (20u16, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::inet_diag::OpTcpDiagDump::decode_request(buf),
                                fmt,
                            );
                        }
                        #[cfg(not(feature = "inet-diag"))]
                        if let (20u16, None, true) = pat {
                            return consider(fmt, "inet-diag");
                        }
                        #[cfg(feature = "inet-diag")]
                        if let (20u16, Some(20u16), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::inet_diag::OpTcpDiagDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        #[cfg(not(feature = "inet-diag"))]
                        if let (20u16, Some(20u16), true) = pat {
                            return consider(fmt, "inet-diag");
                        }
                    }
                    let mut pass = false;
                    let filter: fn(&[u8]) -> bool = |buf| buf[1] == 17;
                    if request_value.is_none()
                        && last_filter_val.is_none_or(|l| l == 1usize)
                        && filter(buf)
                    {
                        last_filter.set(Some(1usize));
                        pass = true;
                    }
                    if request_value.is_some() && last_filter_val.is_some_and(|l| l == 1usize) {
                        last_filter.set(Some(1usize));
                        pass = true;
                    }
                    let filter: fn(&[u8]) -> bool = |buf| buf[0] == 2 || buf[0] == 10;
                    pass = filter(buf) && pass;
                    if pass {
                        #[cfg(feature = "inet-diag")]
                        if let (20u16, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::inet_diag::OpUdpDiagDump::decode_request(buf),
                                fmt,
                            );
                        }
                        #[cfg(not(feature = "inet-diag"))]
                        if let (20u16, None, true) = pat {
                            return consider(fmt, "inet-diag");
                        }
                        #[cfg(feature = "inet-diag")]
                        if let (20u16, Some(20u16), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::inet_diag::OpUdpDiagDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        #[cfg(not(feature = "inet-diag"))]
                        if let (20u16, Some(20u16), true) = pat {
                            return consider(fmt, "inet-diag");
                        }
                    }
                    let mut pass = false;
                    let filter: fn(&[u8]) -> bool = |buf| buf[0] == 1;
                    pass = filter(buf) && pass;
                    if pass {
                        #[cfg(feature = "unix-diag")]
                        if let (20u16, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::unix_diag::OpUnixDiagDump::decode_request(buf),
                                fmt,
                            );
                        }
                        #[cfg(not(feature = "unix-diag"))]
                        if let (20u16, None, true) = pat {
                            return consider(fmt, "unix-diag");
                        }
                        #[cfg(feature = "unix-diag")]
                        if let (20u16, Some(20u16), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::unix_diag::OpUnixDiagDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        #[cfg(not(feature = "unix-diag"))]
                        if let (20u16, Some(20u16), true) = pat {
                            return consider(fmt, "unix-diag");
                        }
                    }
                    write!(
                        fmt,
                        "(Unknown operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                    )?;
                    return Ok(());
                }
                if protonum == 12u16 {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "conntrack")]
                    if let (257u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::conntrack::OpGetDump::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "conntrack"))]
                    if let (257u16, None, true) = pat {
                        return consider(fmt, "conntrack");
                    }
                    #[cfg(feature = "conntrack")]
                    if let (256u16, Some(257u16), true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::conntrack::OpGetDump::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "conntrack"))]
                    if let (256u16, Some(257u16), true) = pat {
                        return consider(fmt, "conntrack");
                    }
                    #[cfg(feature = "conntrack")]
                    if let (257u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::conntrack::OpGetDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "conntrack"))]
                    if let (257u16, None, false) = pat {
                        return consider(fmt, "conntrack");
                    }
                    #[cfg(feature = "conntrack")]
                    if let (256u16, Some(257u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::conntrack::OpGetDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "conntrack"))]
                    if let (256u16, Some(257u16), false) = pat {
                        return consider(fmt, "conntrack");
                    }
                    #[cfg(feature = "conntrack")]
                    if let (260u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::conntrack::OpGetStatsDump::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "conntrack"))]
                    if let (260u16, None, true) = pat {
                        return consider(fmt, "conntrack");
                    }
                    #[cfg(feature = "conntrack")]
                    if let (260u16, Some(260u16), true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::conntrack::OpGetStatsDump::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "conntrack"))]
                    if let (260u16, Some(260u16), true) = pat {
                        return consider(fmt, "conntrack");
                    }
                    #[cfg(feature = "nftables")]
                    if let (16u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpBatchBeginDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (16u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (16u16, Some(16u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpBatchBeginDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (16u16, Some(16u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (17u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpBatchEndDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (17u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (17u16, Some(17u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpBatchEndDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (17u16, Some(17u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2560u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpNewtableDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2560u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2560u16, Some(2560u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpNewtableDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2560u16, Some(2560u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2561u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGettableDump::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2561u16, None, true) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2560u16, Some(2561u16), true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGettableDump::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2560u16, Some(2561u16), true) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2561u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGettableDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2561u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2560u16, Some(2561u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGettableDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2560u16, Some(2561u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2562u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDeltableDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2562u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2562u16, Some(2562u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDeltableDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2562u16, Some(2562u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2586u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDestroytableDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2586u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2586u16, Some(2586u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDestroytableDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2586u16, Some(2586u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2563u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpNewchainDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2563u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2563u16, Some(2563u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpNewchainDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2563u16, Some(2563u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2564u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetchainDump::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2564u16, None, true) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2563u16, Some(2564u16), true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetchainDump::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2563u16, Some(2564u16), true) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2564u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetchainDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2564u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2563u16, Some(2564u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetchainDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2563u16, Some(2564u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2565u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDelchainDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2565u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2565u16, Some(2565u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDelchainDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2565u16, Some(2565u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2587u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDestroychainDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2587u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2587u16, Some(2587u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDestroychainDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2587u16, Some(2587u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2566u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpNewruleDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2566u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2566u16, Some(2566u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpNewruleDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2566u16, Some(2566u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2567u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetruleDump::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2567u16, None, true) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2566u16, Some(2567u16), true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetruleDump::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2566u16, Some(2567u16), true) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2567u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetruleDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2567u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2566u16, Some(2567u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetruleDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2566u16, Some(2567u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2585u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetruleResetDump::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2585u16, None, true) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2566u16, Some(2585u16), true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetruleResetDump::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2566u16, Some(2585u16), true) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2585u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetruleResetDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2585u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2566u16, Some(2585u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetruleResetDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2566u16, Some(2585u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2568u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDelruleDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2568u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2568u16, Some(2568u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDelruleDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2568u16, Some(2568u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2588u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDestroyruleDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2588u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2588u16, Some(2588u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDestroyruleDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2588u16, Some(2588u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2569u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpNewsetDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2569u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2569u16, Some(2569u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpNewsetDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2569u16, Some(2569u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2570u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetsetDump::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2570u16, None, true) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2569u16, Some(2570u16), true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetsetDump::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2569u16, Some(2570u16), true) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2570u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetsetDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2570u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2569u16, Some(2570u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetsetDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2569u16, Some(2570u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2571u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDelsetDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2571u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2571u16, Some(2571u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDelsetDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2571u16, Some(2571u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2589u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDestroysetDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2589u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2589u16, Some(2589u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDestroysetDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2589u16, Some(2589u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2572u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpNewsetelemDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2572u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2572u16, Some(2572u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpNewsetelemDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2572u16, Some(2572u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2573u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetsetelemDump::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2573u16, None, true) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2572u16, Some(2573u16), true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetsetelemDump::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2572u16, Some(2573u16), true) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2573u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetsetelemDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2573u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2572u16, Some(2573u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetsetelemDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2572u16, Some(2573u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2593u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetsetelemResetDump::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2593u16, None, true) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2572u16, Some(2593u16), true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetsetelemResetDump::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2572u16, Some(2593u16), true) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2593u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetsetelemResetDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2593u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2572u16, Some(2593u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetsetelemResetDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2572u16, Some(2593u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2574u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDelsetelemDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2574u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2574u16, Some(2574u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDelsetelemDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2574u16, Some(2574u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2590u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDestroysetelemDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2590u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2590u16, Some(2590u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDestroysetelemDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2590u16, Some(2590u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2576u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetgenDump::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2576u16, None, true) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2575u16, Some(2576u16), true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetgenDump::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2575u16, Some(2576u16), true) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2576u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetgenDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2576u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2575u16, Some(2576u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetgenDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2575u16, Some(2576u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2578u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpNewobjDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2578u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2578u16, Some(2578u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpNewobjDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2578u16, Some(2578u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2579u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetobjDump::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2579u16, None, true) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2578u16, Some(2579u16), true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetobjDump::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2578u16, Some(2579u16), true) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2579u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetobjDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2579u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2578u16, Some(2579u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetobjDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2578u16, Some(2579u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2580u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDelobjDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2580u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2580u16, Some(2580u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDelobjDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2580u16, Some(2580u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2591u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDestroyobjDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2591u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2591u16, Some(2591u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDestroyobjDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2591u16, Some(2591u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2582u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpNewflowtableDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2582u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2582u16, Some(2582u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpNewflowtableDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2582u16, Some(2582u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2583u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetflowtableDump::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2583u16, None, true) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2582u16, Some(2583u16), true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetflowtableDump::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2582u16, Some(2583u16), true) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2583u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetflowtableDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2583u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2582u16, Some(2583u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetflowtableDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2582u16, Some(2583u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2584u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDelflowtableDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2584u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2584u16, Some(2584u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDelflowtableDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2584u16, Some(2584u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2592u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDestroyflowtableDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2592u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2592u16, Some(2592u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpDestroyflowtableDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2592u16, Some(2592u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2816u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetcompatDump::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2816u16, None, true) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2816u16, Some(2816u16), true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetcompatDump::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2816u16, Some(2816u16), true) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2816u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetcompatDo::decode_request(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2816u16, None, false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2816u16, Some(2816u16), false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetcompatDo::decode_reply(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2816u16, Some(2816u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    write!(
                        fmt,
                        "(Unknown operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                    )?;
                    return Ok(());
                }
                write!(fmt, "(Protocol {protonum:?} not recognized)")
            }
            Protocol::Generic(name) => {
                let value = value as u8;
                let request_value = request_value.map(|val| val as u8);
                if name == b"amneziawg" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "amneziawg")]
                    {
                        if let (0u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::amneziawg::OpGetDeviceDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (0u8, Some(0u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::amneziawg::OpGetDeviceDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::amneziawg::OpSetDeviceDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::amneziawg::OpSetDeviceDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        )?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "amneziawg"))]
                    return consider(fmt, "amneziawg");
                }
                if name == b"binder" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "binder")]
                    {
                        write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        )?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "binder"))]
                    return consider(fmt, "binder");
                }
                if name == b"dev-energymodel" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "dev-energymodel")]
                    {
                        if let (1u8, None, true) = pat {
                            return Debug :: fmt (& netlink_bindings :: dev_energymodel :: OpGetPerfDomainsDump :: decode_reply (buf) , fmt) ;
                        }
                        if let (1u8, Some(1u8), true) = pat {
                            return Debug :: fmt (& netlink_bindings :: dev_energymodel :: OpGetPerfDomainsDump :: decode_reply (buf) , fmt) ;
                        }
                        if let (1u8, None, false) = pat {
                            return Debug :: fmt (& netlink_bindings :: dev_energymodel :: OpGetPerfDomainsDo :: decode_reply (buf) , fmt) ;
                        }
                        if let (1u8, Some(1u8), false) = pat {
                            return Debug :: fmt (& netlink_bindings :: dev_energymodel :: OpGetPerfDomainsDo :: decode_reply (buf) , fmt) ;
                        }
                        if let (2u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::dev_energymodel::OpGetPerfTableDo::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (2u8, Some(2u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::dev_energymodel::OpGetPerfTableDo::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        )?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "dev-energymodel"))]
                    return consider(fmt, "dev-energymodel");
                }
                if name == b"devlink" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "devlink")]
                    {
                        if let (1u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(1u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(1u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (5u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(5u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (5u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (7u8, Some(5u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (6u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (6u8, Some(6u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (7u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortNewDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (7u8, Some(7u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortNewDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (8u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortDelDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (8u8, Some(8u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortDelDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (9u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortSplitDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (9u8, Some(9u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortSplitDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (10u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortUnsplitDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (10u8, Some(10u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortUnsplitDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (11u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (13u8, Some(11u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (11u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (13u8, Some(11u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (15u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbPoolGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (17u8, Some(15u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbPoolGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (15u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbPoolGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (17u8, Some(15u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbPoolGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (16u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbPoolSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (16u8, Some(16u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbPoolSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (19u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbPortPoolGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (21u8, Some(19u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbPortPoolGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (19u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbPortPoolGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (21u8, Some(19u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbPortPoolGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (20u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbPortPoolSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (20u8, Some(20u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbPortPoolSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (23u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbTcPoolBindGetDump::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (25u8, Some(23u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbTcPoolBindGetDump::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (23u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbTcPoolBindGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (25u8, Some(23u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbTcPoolBindGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (24u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbTcPoolBindSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (24u8, Some(24u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbTcPoolBindSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (27u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbOccSnapshotDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (27u8, Some(27u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbOccSnapshotDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (28u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbOccMaxClearDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (28u8, Some(28u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbOccMaxClearDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (29u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpEswitchGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (29u8, Some(29u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpEswitchGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (30u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpEswitchSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (30u8, Some(30u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpEswitchSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (31u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpDpipeTableGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (31u8, Some(31u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpDpipeTableGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (32u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpDpipeEntriesGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (32u8, Some(32u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpDpipeEntriesGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (33u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpDpipeHeadersGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (33u8, Some(33u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpDpipeHeadersGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (34u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpDpipeTableCountersSetDo::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (34u8, Some(34u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpDpipeTableCountersSetDo::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (35u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpResourceSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (35u8, Some(35u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpResourceSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (36u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpResourceDumpDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (36u8, Some(36u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpResourceDumpDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (36u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpResourceDumpDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (36u8, Some(36u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpResourceDumpDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (37u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpReloadDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (37u8, Some(37u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpReloadDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (38u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpParamGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (38u8, Some(38u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpParamGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (38u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpParamGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (38u8, Some(38u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpParamGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (39u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpParamSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (39u8, Some(39u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpParamSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (42u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRegionGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (42u8, Some(42u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRegionGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (42u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRegionGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (42u8, Some(42u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRegionGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (44u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRegionNewDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (44u8, Some(44u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRegionNewDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (45u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRegionDelDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (45u8, Some(45u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRegionDelDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (46u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRegionReadDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (46u8, Some(46u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRegionReadDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (47u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortParamGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (47u8, Some(47u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortParamGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (47u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortParamGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (47u8, Some(47u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortParamGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (48u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortParamSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (48u8, Some(48u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortParamSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (51u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpInfoGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (51u8, Some(51u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpInfoGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (51u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpInfoGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (51u8, Some(51u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpInfoGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (52u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterGetDump::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (52u8, Some(52u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterGetDump::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (52u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterGetDo::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (52u8, Some(52u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterGetDo::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (53u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterSetDo::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (53u8, Some(53u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterSetDo::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (54u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterRecoverDo::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (54u8, Some(54u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterRecoverDo::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (55u8, None, false) = pat {
                            return Debug :: fmt (& netlink_bindings :: devlink :: OpHealthReporterDiagnoseDo :: decode_reply (buf) , fmt) ;
                        }
                        if let (55u8, Some(55u8), false) = pat {
                            return Debug :: fmt (& netlink_bindings :: devlink :: OpHealthReporterDiagnoseDo :: decode_reply (buf) , fmt) ;
                        }
                        if let (56u8, None, true) = pat {
                            return Debug :: fmt (& netlink_bindings :: devlink :: OpHealthReporterDumpGetDump :: decode_reply (buf) , fmt) ;
                        }
                        if let (56u8, Some(56u8), true) = pat {
                            return Debug :: fmt (& netlink_bindings :: devlink :: OpHealthReporterDumpGetDump :: decode_reply (buf) , fmt) ;
                        }
                        if let (57u8, None, false) = pat {
                            return Debug :: fmt (& netlink_bindings :: devlink :: OpHealthReporterDumpClearDo :: decode_reply (buf) , fmt) ;
                        }
                        if let (57u8, Some(57u8), false) = pat {
                            return Debug :: fmt (& netlink_bindings :: devlink :: OpHealthReporterDumpClearDo :: decode_reply (buf) , fmt) ;
                        }
                        if let (58u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpFlashUpdateDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (58u8, Some(58u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpFlashUpdateDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (61u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (63u8, Some(61u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (61u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (63u8, Some(61u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (62u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (62u8, Some(62u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (65u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapGroupGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (67u8, Some(65u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapGroupGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (65u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapGroupGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (67u8, Some(65u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapGroupGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (66u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapGroupSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (66u8, Some(66u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapGroupSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (69u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapPolicerGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (71u8, Some(69u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapPolicerGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (69u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapPolicerGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (71u8, Some(69u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapPolicerGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (70u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapPolicerSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (70u8, Some(70u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapPolicerSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (73u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterTestDo::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (73u8, Some(73u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterTestDo::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (74u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRateGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (76u8, Some(74u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRateGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (74u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRateGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (76u8, Some(74u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRateGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (75u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRateSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (75u8, Some(75u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRateSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (76u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRateNewDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (76u8, Some(76u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRateNewDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (77u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRateDelDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (77u8, Some(77u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRateDelDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (78u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpLinecardGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (80u8, Some(78u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpLinecardGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (78u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpLinecardGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (80u8, Some(78u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpLinecardGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (79u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpLinecardSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (79u8, Some(79u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpLinecardSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (82u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSelftestsGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (82u8, Some(82u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSelftestsGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (82u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSelftestsGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (82u8, Some(82u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSelftestsGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (83u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSelftestsRunDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (83u8, Some(83u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSelftestsRunDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (84u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpNotifyFilterSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (84u8, Some(84u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpNotifyFilterSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        )?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "devlink"))]
                    return consider(fmt, "devlink");
                }
                if name == b"dpll" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "dpll")]
                    {
                        if let (1u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::dpll::OpDeviceIdGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::dpll::OpDeviceIdGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::dpll::OpDeviceGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, Some(2u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::dpll::OpDeviceGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::dpll::OpDeviceGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, Some(2u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::dpll::OpDeviceGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::dpll::OpDeviceSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(3u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::dpll::OpDeviceSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (7u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::dpll::OpPinIdGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (7u8, Some(7u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::dpll::OpPinIdGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (8u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::dpll::OpPinGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (8u8, Some(8u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::dpll::OpPinGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (8u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::dpll::OpPinGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (8u8, Some(8u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::dpll::OpPinGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (9u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::dpll::OpPinSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (9u8, Some(9u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::dpll::OpPinSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        )?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "dpll"))]
                    return consider(fmt, "dpll");
                }
                if name == b"drm-ras" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "drm-ras")]
                    {
                        if let (1u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::drm_ras::OpListNodesDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::drm_ras::OpListNodesDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::drm_ras::OpGetErrorCounterDump::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (2u8, Some(2u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::drm_ras::OpGetErrorCounterDump::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (2u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::drm_ras::OpGetErrorCounterDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, Some(2u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::drm_ras::OpGetErrorCounterDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::drm_ras::OpClearErrorCounterDo::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (3u8, Some(3u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::drm_ras::OpClearErrorCounterDo::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        )?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "drm-ras"))]
                    return consider(fmt, "drm-ras");
                }
                if name == b"ethtool" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "ethtool")]
                    {
                        if let (1u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpStrsetGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpStrsetGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpStrsetGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpStrsetGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpLinkinfoGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, Some(2u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpLinkinfoGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpLinkinfoGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, Some(2u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpLinkinfoGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpLinkinfoSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(3u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpLinkinfoSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (5u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpLinkmodesGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (5u8, Some(5u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpLinkmodesGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (5u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpLinkmodesGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (5u8, Some(5u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpLinkmodesGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (6u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpLinkmodesSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (6u8, Some(6u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpLinkmodesSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (8u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpLinkstateGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (8u8, Some(8u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpLinkstateGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (8u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpLinkstateGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (8u8, Some(8u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpLinkstateGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (9u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpDebugGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (9u8, Some(9u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpDebugGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (9u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpDebugGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (9u8, Some(9u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpDebugGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (10u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpDebugSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (10u8, Some(10u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpDebugSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (12u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpWolGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (12u8, Some(12u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpWolGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (12u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpWolGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (12u8, Some(12u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpWolGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (13u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpWolSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (13u8, Some(13u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpWolSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (15u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpFeaturesGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (15u8, Some(15u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpFeaturesGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (15u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpFeaturesGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (15u8, Some(15u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpFeaturesGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (16u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpFeaturesSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (16u8, Some(16u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpFeaturesSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (18u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPrivflagsGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (18u8, Some(18u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPrivflagsGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (18u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPrivflagsGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (18u8, Some(18u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPrivflagsGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (19u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPrivflagsSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (19u8, Some(19u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPrivflagsSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (21u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpRingsGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (21u8, Some(21u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpRingsGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (21u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpRingsGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (21u8, Some(21u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpRingsGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (22u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpRingsSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (22u8, Some(22u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpRingsSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (24u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpChannelsGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (24u8, Some(24u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpChannelsGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (24u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpChannelsGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (24u8, Some(24u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpChannelsGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (25u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpChannelsSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (25u8, Some(25u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpChannelsSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (27u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpCoalesceGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (27u8, Some(27u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpCoalesceGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (27u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpCoalesceGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (27u8, Some(27u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpCoalesceGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (28u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpCoalesceSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (28u8, Some(28u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpCoalesceSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (30u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPauseGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (30u8, Some(30u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPauseGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (30u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPauseGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (30u8, Some(30u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPauseGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (31u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPauseSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (31u8, Some(31u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPauseSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (33u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpEeeGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (33u8, Some(33u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpEeeGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (33u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpEeeGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (33u8, Some(33u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpEeeGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (34u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpEeeSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (34u8, Some(34u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpEeeSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (36u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpTsinfoGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (36u8, Some(36u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpTsinfoGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (36u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpTsinfoGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (36u8, Some(36u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpTsinfoGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (37u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpCableTestActDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (37u8, Some(37u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpCableTestActDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (39u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpCableTestTdrActDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (39u8, Some(39u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpCableTestTdrActDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (41u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpTunnelInfoGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (41u8, Some(41u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpTunnelInfoGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (41u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpTunnelInfoGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (41u8, Some(41u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpTunnelInfoGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (42u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpFecGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (42u8, Some(42u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpFecGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (42u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpFecGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (42u8, Some(42u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpFecGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (43u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpFecSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (43u8, Some(43u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpFecSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (45u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpModuleEepromGetDump::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (45u8, Some(45u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpModuleEepromGetDump::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (45u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpModuleEepromGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (45u8, Some(45u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpModuleEepromGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (46u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpStatsGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (46u8, Some(46u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpStatsGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (46u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpStatsGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (46u8, Some(46u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpStatsGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (47u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPhcVclocksGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (47u8, Some(47u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPhcVclocksGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (47u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPhcVclocksGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (47u8, Some(47u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPhcVclocksGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (48u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpModuleGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (48u8, Some(48u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpModuleGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (48u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpModuleGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (48u8, Some(48u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpModuleGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (49u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpModuleSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (49u8, Some(49u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpModuleSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (51u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPseGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (51u8, Some(51u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPseGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (51u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPseGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (51u8, Some(51u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPseGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (52u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPseSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (52u8, Some(52u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPseSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (53u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpRssGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (53u8, Some(53u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpRssGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (53u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpRssGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (53u8, Some(53u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpRssGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (54u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPlcaGetCfgDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (54u8, Some(54u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPlcaGetCfgDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (54u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPlcaGetCfgDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (54u8, Some(54u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPlcaGetCfgDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (55u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPlcaSetCfgDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (55u8, Some(55u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPlcaSetCfgDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (56u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPlcaGetStatusDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (56u8, Some(56u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPlcaGetStatusDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (56u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPlcaGetStatusDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (56u8, Some(56u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPlcaGetStatusDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (58u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpMmGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (58u8, Some(58u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpMmGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (58u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpMmGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (58u8, Some(58u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpMmGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (59u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpMmSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (59u8, Some(59u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpMmSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (61u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpModuleFwFlashActDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (61u8, Some(61u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpModuleFwFlashActDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (63u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPhyGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (63u8, Some(63u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPhyGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (63u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPhyGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (63u8, Some(63u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpPhyGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (65u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpTsconfigGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (65u8, Some(65u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpTsconfigGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (65u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpTsconfigGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (65u8, Some(65u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpTsconfigGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (66u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpTsconfigSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (66u8, Some(66u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpTsconfigSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (68u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpRssSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (68u8, Some(68u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpRssSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (70u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpRssCreateActDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (70u8, Some(70u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpRssCreateActDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (72u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpRssDeleteActDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (72u8, Some(72u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpRssDeleteActDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (74u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpMseGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (74u8, Some(74u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpMseGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (74u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpMseGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (74u8, Some(74u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ethtool::OpMseGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        )?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "ethtool"))]
                    return consider(fmt, "ethtool");
                }
                if name == b"fou" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "fou")]
                    {
                        if let (1u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::fou::OpAddDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::fou::OpAddDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::fou::OpDelDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, Some(2u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::fou::OpDelDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::fou::OpGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(3u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::fou::OpGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::fou::OpGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(3u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::fou::OpGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        )?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "fou"))]
                    return consider(fmt, "fou");
                }
                if name == b"handshake" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "handshake")]
                    {
                        if let (2u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::handshake::OpAcceptDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, Some(2u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::handshake::OpAcceptDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::handshake::OpDoneDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(3u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::handshake::OpDoneDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        )?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "handshake"))]
                    return consider(fmt, "handshake");
                }
                if name == b"lockd" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "lockd")]
                    {
                        if let (1u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::lockd::OpServerSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::lockd::OpServerSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::lockd::OpServerGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, Some(2u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::lockd::OpServerGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        )?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "lockd"))]
                    return consider(fmt, "lockd");
                }
                if name == b"mptcp_pm" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "mptcp_pm")]
                    {
                        if let (1u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::mptcp_pm::OpAddAddrDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::mptcp_pm::OpAddAddrDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::mptcp_pm::OpDelAddrDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, Some(2u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::mptcp_pm::OpDelAddrDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::mptcp_pm::OpGetAddrDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(3u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::mptcp_pm::OpGetAddrDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::mptcp_pm::OpGetAddrDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(3u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::mptcp_pm::OpGetAddrDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (4u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::mptcp_pm::OpFlushAddrsDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (4u8, Some(4u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::mptcp_pm::OpFlushAddrsDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (5u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::mptcp_pm::OpSetLimitsDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (5u8, Some(5u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::mptcp_pm::OpSetLimitsDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (6u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::mptcp_pm::OpGetLimitsDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (6u8, Some(6u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::mptcp_pm::OpGetLimitsDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (7u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::mptcp_pm::OpSetFlagsDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (7u8, Some(7u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::mptcp_pm::OpSetFlagsDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (8u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::mptcp_pm::OpAnnounceDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (8u8, Some(8u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::mptcp_pm::OpAnnounceDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (9u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::mptcp_pm::OpRemoveDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (9u8, Some(9u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::mptcp_pm::OpRemoveDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (10u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::mptcp_pm::OpSubflowCreateDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (10u8, Some(10u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::mptcp_pm::OpSubflowCreateDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (11u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::mptcp_pm::OpSubflowDestroyDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (11u8, Some(11u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::mptcp_pm::OpSubflowDestroyDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        )?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "mptcp_pm"))]
                    return consider(fmt, "mptcp_pm");
                }
                if name == b"net-shaper" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "net-shaper")]
                    {
                        if let (1u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::net_shaper::OpGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::net_shaper::OpGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::net_shaper::OpGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::net_shaper::OpGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::net_shaper::OpSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, Some(2u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::net_shaper::OpSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::net_shaper::OpDeleteDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(3u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::net_shaper::OpDeleteDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (4u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::net_shaper::OpGroupDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (4u8, Some(4u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::net_shaper::OpGroupDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (5u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::net_shaper::OpCapGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (5u8, Some(5u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::net_shaper::OpCapGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (5u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::net_shaper::OpCapGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (5u8, Some(5u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::net_shaper::OpCapGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        )?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "net-shaper"))]
                    return consider(fmt, "net-shaper");
                }
                if name == b"netdev" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "netdev")]
                    {
                        if let (1u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpDevGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpDevGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpDevGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpDevGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (5u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpPagePoolGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (5u8, Some(5u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpPagePoolGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (5u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpPagePoolGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (5u8, Some(5u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpPagePoolGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (9u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpPagePoolStatsGetDump::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (9u8, Some(9u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpPagePoolStatsGetDump::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (9u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpPagePoolStatsGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (9u8, Some(9u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpPagePoolStatsGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (10u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpQueueGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (10u8, Some(10u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpQueueGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (10u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpQueueGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (10u8, Some(10u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpQueueGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (11u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpNapiGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (11u8, Some(11u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpNapiGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (11u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpNapiGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (11u8, Some(11u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpNapiGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (12u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpQstatsGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (12u8, Some(12u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpQstatsGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (13u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpBindRxDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (13u8, Some(13u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpBindRxDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (14u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpNapiSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (14u8, Some(14u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpNapiSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (15u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpBindTxDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (15u8, Some(15u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpBindTxDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (16u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpQueueCreateDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (16u8, Some(16u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpQueueCreateDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        )?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "netdev"))]
                    return consider(fmt, "netdev");
                }
                if name == b"nfsd" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "nfsd")]
                    {
                        if let (1u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpRpcStatusGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpRpcStatusGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpThreadsSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, Some(2u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpThreadsSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpThreadsGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(3u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpThreadsGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (4u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpVersionSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (4u8, Some(4u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpVersionSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (5u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpVersionGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (5u8, Some(5u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpVersionGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (6u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpListenerSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (6u8, Some(6u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpListenerSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (7u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpListenerGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (7u8, Some(7u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpListenerGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (8u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpPoolModeSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (8u8, Some(8u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpPoolModeSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (9u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpPoolModeGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (9u8, Some(9u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpPoolModeGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (11u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpSvcExportGetReqsDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (11u8, Some(11u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpSvcExportGetReqsDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (12u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpSvcExportSetReqsDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (12u8, Some(12u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpSvcExportSetReqsDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (13u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpExpkeyGetReqsDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (13u8, Some(13u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpExpkeyGetReqsDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (14u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpExpkeySetReqsDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (14u8, Some(14u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpExpkeySetReqsDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (15u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpCacheFlushDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (15u8, Some(15u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpCacheFlushDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (16u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpUnlockIpDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (16u8, Some(16u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpUnlockIpDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (17u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpUnlockFilesystemDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (17u8, Some(17u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpUnlockFilesystemDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (18u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpUnlockExportDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (18u8, Some(18u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nfsd::OpUnlockExportDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        )?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "nfsd"))]
                    return consider(fmt, "nfsd");
                }
                if name == b"nl80211" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "nl80211")]
                    {
                        if let (1u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nl80211::OpGetWiphyDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(1u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nl80211::OpGetWiphyDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nl80211::OpGetWiphyDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(1u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nl80211::OpGetWiphyDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (5u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nl80211::OpGetInterfaceDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (7u8, Some(5u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nl80211::OpGetInterfaceDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (5u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nl80211::OpGetInterfaceDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (7u8, Some(5u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nl80211::OpGetInterfaceDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (95u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nl80211::OpGetProtocolFeaturesDo::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (95u8, Some(95u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nl80211::OpGetProtocolFeaturesDo::decode_reply(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        writeln!(
                            fmt,
                            "Unknown genl operation, falling back to {:?} value={value}, request_value={request_value:?}, is_dump={is_dump}",
                            "Nl80211Attrs"
                        )?;
                        Debug::fmt(&netlink_bindings::nl80211::OpDo::decode_reply(buf), fmt)?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "nl80211"))]
                    return consider(fmt, "nl80211");
                }
                if name == b"nlctrl" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "nlctrl")]
                    {
                        if let (3u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nlctrl::OpGetfamilyDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(3u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nlctrl::OpGetfamilyDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nlctrl::OpGetfamilyDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(3u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nlctrl::OpGetfamilyDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (10u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nlctrl::OpGetpolicyDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (10u8, Some(10u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nlctrl::OpGetpolicyDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        )?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "nlctrl"))]
                    return consider(fmt, "nlctrl");
                }
                if name == b"ovpn" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "ovpn")]
                    {
                        if let (1u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovpn::OpPeerNewDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovpn::OpPeerNewDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovpn::OpPeerSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, Some(2u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovpn::OpPeerSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovpn::OpPeerGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(3u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovpn::OpPeerGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovpn::OpPeerGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(3u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovpn::OpPeerGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (4u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovpn::OpPeerDelDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (4u8, Some(4u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovpn::OpPeerDelDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (6u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovpn::OpKeyNewDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (6u8, Some(6u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovpn::OpKeyNewDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (7u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovpn::OpKeyGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (7u8, Some(7u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovpn::OpKeyGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (8u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovpn::OpKeySwapDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (8u8, Some(8u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovpn::OpKeySwapDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (10u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovpn::OpKeyDelDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (10u8, Some(10u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovpn::OpKeyDelDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        )?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "ovpn"))]
                    return consider(fmt, "ovpn");
                }
                if name == b"ovs_datapath" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "ovs_datapath")]
                    {
                        if let (3u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovs_datapath::OpGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(3u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovs_datapath::OpGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovs_datapath::OpGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(3u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovs_datapath::OpGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovs_datapath::OpNewDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovs_datapath::OpNewDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovs_datapath::OpDelDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, Some(2u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovs_datapath::OpDelDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        )?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "ovs_datapath"))]
                    return consider(fmt, "ovs_datapath");
                }
                if name == b"ovs_flow" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "ovs_flow")]
                    {
                        if let (3u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovs_flow::OpGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(3u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovs_flow::OpGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovs_flow::OpGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(3u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovs_flow::OpGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovs_flow::OpNewDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovs_flow::OpNewDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        )?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "ovs_flow"))]
                    return consider(fmt, "ovs_flow");
                }
                if name == b"ovs_packet" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "ovs_packet")]
                    {
                        if let (3u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovs_packet::OpExecuteDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(3u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovs_packet::OpExecuteDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        )?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "ovs_packet"))]
                    return consider(fmt, "ovs_packet");
                }
                if name == b"ovs_vport" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "ovs_vport")]
                    {
                        if let (1u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovs_vport::OpNewDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovs_vport::OpNewDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovs_vport::OpDelDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, Some(2u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovs_vport::OpDelDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovs_vport::OpGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(3u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovs_vport::OpGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovs_vport::OpGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(3u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::ovs_vport::OpGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        )?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "ovs_vport"))]
                    return consider(fmt, "ovs_vport");
                }
                if name == b"psp" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "psp")]
                    {
                        if let (1u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::psp::OpDevGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::psp::OpDevGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::psp::OpDevGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::psp::OpDevGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (4u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::psp::OpDevSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (4u8, Some(4u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::psp::OpDevSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (6u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::psp::OpKeyRotateDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (6u8, Some(6u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::psp::OpKeyRotateDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (8u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::psp::OpRxAssocDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (8u8, Some(8u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::psp::OpRxAssocDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (9u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::psp::OpTxAssocDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (9u8, Some(9u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::psp::OpTxAssocDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (10u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::psp::OpGetStatsDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (10u8, Some(10u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::psp::OpGetStatsDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (10u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::psp::OpGetStatsDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (10u8, Some(10u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::psp::OpGetStatsDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (11u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::psp::OpDevAssocDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (11u8, Some(11u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::psp::OpDevAssocDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (12u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::psp::OpDevDisassocDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (12u8, Some(12u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::psp::OpDevDisassocDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        )?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "psp"))]
                    return consider(fmt, "psp");
                }
                if name == b"sunrpc" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "sunrpc")]
                    {
                        if let (2u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::sunrpc::OpIpMapGetReqsDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, Some(2u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::sunrpc::OpIpMapGetReqsDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::sunrpc::OpIpMapSetReqsDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(3u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::sunrpc::OpIpMapSetReqsDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (4u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::sunrpc::OpUnixGidGetReqsDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (4u8, Some(4u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::sunrpc::OpUnixGidGetReqsDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (5u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::sunrpc::OpUnixGidSetReqsDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (5u8, Some(5u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::sunrpc::OpUnixGidSetReqsDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (6u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::sunrpc::OpCacheFlushDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (6u8, Some(6u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::sunrpc::OpCacheFlushDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        )?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "sunrpc"))]
                    return consider(fmt, "sunrpc");
                }
                if name == b"tcp_metrics" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "tcp_metrics")]
                    {
                        if let (1u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::tcp_metrics::OpGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::tcp_metrics::OpGetDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::tcp_metrics::OpGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::tcp_metrics::OpGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::tcp_metrics::OpDelDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, Some(2u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::tcp_metrics::OpDelDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        )?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "tcp_metrics"))]
                    return consider(fmt, "tcp_metrics");
                }
                if name == b"team" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "team")]
                    {
                        if let (0u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::team::OpNoopDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (0u8, Some(0u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::team::OpNoopDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::team::OpOptionsSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::team::OpOptionsSetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::team::OpOptionsGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (2u8, Some(2u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::team::OpOptionsGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::team::OpPortListGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(3u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::team::OpPortListGetDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        )?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "team"))]
                    return consider(fmt, "team");
                }
                if name == b"wireguard" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "wireguard")]
                    {
                        if let (0u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::wireguard::OpGetDeviceDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (0u8, Some(0u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::wireguard::OpGetDeviceDump::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::wireguard::OpSetDeviceDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::wireguard::OpSetDeviceDo::decode_reply(buf),
                                fmt,
                            );
                        }
                        write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        )?;
                        return Ok(());
                    }
                    #[cfg(not(feature = "wireguard"))]
                    return consider(fmt, "wireguard");
                }
                write!(fmt, "(Protocol {name:?} not recognized)")
            }
        }
    }
}
pub fn get_operation_raw(protonum: u16, request_value: u16) -> Vec<&'static str> {
    let mut res = Vec::new();
    if protonum == 0u16 {
        if request_value == 20u16 {
            res.push(stringify!(rt_addr::OpNewaddrDo));
        }
        if request_value == 21u16 {
            res.push(stringify!(rt_addr::OpDeladdrDo));
        }
        if request_value == 22u16 {
            res.push(stringify!(rt_addr::OpGetaddrDump));
        }
        if request_value == 58u16 {
            res.push(stringify!(rt_addr::OpGetmulticastDump));
        }
        if request_value == 58u16 {
            res.push(stringify!(rt_addr::OpGetmulticastDo));
        }
        if request_value == 16u16 {
            res.push(stringify!(rt_link::OpNewlinkDo));
        }
        if request_value == 17u16 {
            res.push(stringify!(rt_link::OpDellinkDo));
        }
        if request_value == 18u16 {
            res.push(stringify!(rt_link::OpGetlinkDump));
        }
        if request_value == 18u16 {
            res.push(stringify!(rt_link::OpGetlinkDo));
        }
        if request_value == 19u16 {
            res.push(stringify!(rt_link::OpSetlinkDo));
        }
        if request_value == 94u16 {
            res.push(stringify!(rt_link::OpGetstatsDump));
        }
        if request_value == 94u16 {
            res.push(stringify!(rt_link::OpGetstatsDo));
        }
        if request_value == 28u16 {
            res.push(stringify!(rt_neigh::OpNewneighDo));
        }
        if request_value == 29u16 {
            res.push(stringify!(rt_neigh::OpDelneighDo));
        }
        if request_value == 30u16 {
            res.push(stringify!(rt_neigh::OpGetneighDump));
        }
        if request_value == 30u16 {
            res.push(stringify!(rt_neigh::OpGetneighDo));
        }
        if request_value == 66u16 {
            res.push(stringify!(rt_neigh::OpGetneightblDump));
        }
        if request_value == 67u16 {
            res.push(stringify!(rt_neigh::OpSetneightblDo));
        }
        if request_value == 26u16 {
            res.push(stringify!(rt_route::OpGetrouteDump));
        }
        if request_value == 26u16 {
            res.push(stringify!(rt_route::OpGetrouteDo));
        }
        if request_value == 24u16 {
            res.push(stringify!(rt_route::OpNewrouteDo));
        }
        if request_value == 25u16 {
            res.push(stringify!(rt_route::OpDelrouteDo));
        }
        if request_value == 32u16 {
            res.push(stringify!(rt_rule::OpNewruleDo));
        }
        if request_value == 33u16 {
            res.push(stringify!(rt_rule::OpDelruleDo));
        }
        if request_value == 34u16 {
            res.push(stringify!(rt_rule::OpGetruleDump));
        }
        if request_value == 36u16 {
            res.push(stringify!(tc::OpNewqdiscDo));
        }
        if request_value == 37u16 {
            res.push(stringify!(tc::OpDelqdiscDo));
        }
        if request_value == 38u16 {
            res.push(stringify!(tc::OpGetqdiscDump));
        }
        if request_value == 38u16 {
            res.push(stringify!(tc::OpGetqdiscDo));
        }
        if request_value == 40u16 {
            res.push(stringify!(tc::OpNewtclassDo));
        }
        if request_value == 41u16 {
            res.push(stringify!(tc::OpDeltclassDo));
        }
        if request_value == 42u16 {
            res.push(stringify!(tc::OpGettclassDo));
        }
        if request_value == 44u16 {
            res.push(stringify!(tc::OpNewtfilterDo));
        }
        if request_value == 45u16 {
            res.push(stringify!(tc::OpDeltfilterDo));
        }
        if request_value == 46u16 {
            res.push(stringify!(tc::OpGettfilterDump));
        }
        if request_value == 46u16 {
            res.push(stringify!(tc::OpGettfilterDo));
        }
        if request_value == 100u16 {
            res.push(stringify!(tc::OpNewchainDo));
        }
        if request_value == 101u16 {
            res.push(stringify!(tc::OpDelchainDo));
        }
        if request_value == 102u16 {
            res.push(stringify!(tc::OpGetchainDo));
        }
    }
    if protonum == 4u16 {
        if request_value == 20u16 {
            res.push(stringify!(inet_diag::OpTcpDiagDump));
        }
        if request_value == 20u16 {
            res.push(stringify!(inet_diag::OpUdpDiagDump));
        }
        if request_value == 20u16 {
            res.push(stringify!(unix_diag::OpUnixDiagDump));
        }
    }
    if protonum == 12u16 {
        if request_value == 257u16 {
            res.push(stringify!(conntrack::OpGetDump));
        }
        if request_value == 257u16 {
            res.push(stringify!(conntrack::OpGetDo));
        }
        if request_value == 260u16 {
            res.push(stringify!(conntrack::OpGetStatsDump));
        }
        if request_value == 16u16 {
            res.push(stringify!(nftables::OpBatchBeginDo));
        }
        if request_value == 17u16 {
            res.push(stringify!(nftables::OpBatchEndDo));
        }
        if request_value == 2560u16 {
            res.push(stringify!(nftables::OpNewtableDo));
        }
        if request_value == 2561u16 {
            res.push(stringify!(nftables::OpGettableDump));
        }
        if request_value == 2561u16 {
            res.push(stringify!(nftables::OpGettableDo));
        }
        if request_value == 2562u16 {
            res.push(stringify!(nftables::OpDeltableDo));
        }
        if request_value == 2586u16 {
            res.push(stringify!(nftables::OpDestroytableDo));
        }
        if request_value == 2563u16 {
            res.push(stringify!(nftables::OpNewchainDo));
        }
        if request_value == 2564u16 {
            res.push(stringify!(nftables::OpGetchainDump));
        }
        if request_value == 2564u16 {
            res.push(stringify!(nftables::OpGetchainDo));
        }
        if request_value == 2565u16 {
            res.push(stringify!(nftables::OpDelchainDo));
        }
        if request_value == 2587u16 {
            res.push(stringify!(nftables::OpDestroychainDo));
        }
        if request_value == 2566u16 {
            res.push(stringify!(nftables::OpNewruleDo));
        }
        if request_value == 2567u16 {
            res.push(stringify!(nftables::OpGetruleDump));
        }
        if request_value == 2567u16 {
            res.push(stringify!(nftables::OpGetruleDo));
        }
        if request_value == 2585u16 {
            res.push(stringify!(nftables::OpGetruleResetDump));
        }
        if request_value == 2585u16 {
            res.push(stringify!(nftables::OpGetruleResetDo));
        }
        if request_value == 2568u16 {
            res.push(stringify!(nftables::OpDelruleDo));
        }
        if request_value == 2588u16 {
            res.push(stringify!(nftables::OpDestroyruleDo));
        }
        if request_value == 2569u16 {
            res.push(stringify!(nftables::OpNewsetDo));
        }
        if request_value == 2570u16 {
            res.push(stringify!(nftables::OpGetsetDump));
        }
        if request_value == 2570u16 {
            res.push(stringify!(nftables::OpGetsetDo));
        }
        if request_value == 2571u16 {
            res.push(stringify!(nftables::OpDelsetDo));
        }
        if request_value == 2589u16 {
            res.push(stringify!(nftables::OpDestroysetDo));
        }
        if request_value == 2572u16 {
            res.push(stringify!(nftables::OpNewsetelemDo));
        }
        if request_value == 2573u16 {
            res.push(stringify!(nftables::OpGetsetelemDump));
        }
        if request_value == 2573u16 {
            res.push(stringify!(nftables::OpGetsetelemDo));
        }
        if request_value == 2593u16 {
            res.push(stringify!(nftables::OpGetsetelemResetDump));
        }
        if request_value == 2593u16 {
            res.push(stringify!(nftables::OpGetsetelemResetDo));
        }
        if request_value == 2574u16 {
            res.push(stringify!(nftables::OpDelsetelemDo));
        }
        if request_value == 2590u16 {
            res.push(stringify!(nftables::OpDestroysetelemDo));
        }
        if request_value == 2576u16 {
            res.push(stringify!(nftables::OpGetgenDump));
        }
        if request_value == 2576u16 {
            res.push(stringify!(nftables::OpGetgenDo));
        }
        if request_value == 2578u16 {
            res.push(stringify!(nftables::OpNewobjDo));
        }
        if request_value == 2579u16 {
            res.push(stringify!(nftables::OpGetobjDump));
        }
        if request_value == 2579u16 {
            res.push(stringify!(nftables::OpGetobjDo));
        }
        if request_value == 2580u16 {
            res.push(stringify!(nftables::OpDelobjDo));
        }
        if request_value == 2591u16 {
            res.push(stringify!(nftables::OpDestroyobjDo));
        }
        if request_value == 2582u16 {
            res.push(stringify!(nftables::OpNewflowtableDo));
        }
        if request_value == 2583u16 {
            res.push(stringify!(nftables::OpGetflowtableDump));
        }
        if request_value == 2583u16 {
            res.push(stringify!(nftables::OpGetflowtableDo));
        }
        if request_value == 2584u16 {
            res.push(stringify!(nftables::OpDelflowtableDo));
        }
        if request_value == 2592u16 {
            res.push(stringify!(nftables::OpDestroyflowtableDo));
        }
        if request_value == 2816u16 {
            res.push(stringify!(nftables::OpGetcompatDump));
        }
        if request_value == 2816u16 {
            res.push(stringify!(nftables::OpGetcompatDo));
        }
    }
    res
}
pub fn get_operation_genl(proto: &[u8], cmd: u8) -> Vec<&'static str> {
    let mut res = Vec::new();
    if proto == b"binder" {}
    if proto == b"dev-energymodel" {
        if cmd == 1u8 {
            res.push(stringify!(dev_energymodel::OpGetPerfDomainsDump));
        }
        if cmd == 1u8 {
            res.push(stringify!(dev_energymodel::OpGetPerfDomainsDo));
        }
        if cmd == 2u8 {
            res.push(stringify!(dev_energymodel::OpGetPerfTableDo));
        }
    }
    if proto == b"devlink" {
        if cmd == 3u8 {
            res.push(stringify!(devlink::OpGetDump));
        }
        if cmd == 3u8 {
            res.push(stringify!(devlink::OpGetDo));
        }
        if cmd == 3u8 {
            res.push(stringify!(devlink::OpPortGetDump));
        }
        if cmd == 7u8 {
            res.push(stringify!(devlink::OpPortGetDo));
        }
        if cmd == 6u8 {
            res.push(stringify!(devlink::OpPortSetDo));
        }
        if cmd == 7u8 {
            res.push(stringify!(devlink::OpPortNewDo));
        }
        if cmd == 8u8 {
            res.push(stringify!(devlink::OpPortDelDo));
        }
        if cmd == 9u8 {
            res.push(stringify!(devlink::OpPortSplitDo));
        }
        if cmd == 10u8 {
            res.push(stringify!(devlink::OpPortUnsplitDo));
        }
        if cmd == 13u8 {
            res.push(stringify!(devlink::OpSbGetDump));
        }
        if cmd == 13u8 {
            res.push(stringify!(devlink::OpSbGetDo));
        }
        if cmd == 17u8 {
            res.push(stringify!(devlink::OpSbPoolGetDump));
        }
        if cmd == 17u8 {
            res.push(stringify!(devlink::OpSbPoolGetDo));
        }
        if cmd == 16u8 {
            res.push(stringify!(devlink::OpSbPoolSetDo));
        }
        if cmd == 21u8 {
            res.push(stringify!(devlink::OpSbPortPoolGetDump));
        }
        if cmd == 21u8 {
            res.push(stringify!(devlink::OpSbPortPoolGetDo));
        }
        if cmd == 20u8 {
            res.push(stringify!(devlink::OpSbPortPoolSetDo));
        }
        if cmd == 25u8 {
            res.push(stringify!(devlink::OpSbTcPoolBindGetDump));
        }
        if cmd == 25u8 {
            res.push(stringify!(devlink::OpSbTcPoolBindGetDo));
        }
        if cmd == 24u8 {
            res.push(stringify!(devlink::OpSbTcPoolBindSetDo));
        }
        if cmd == 27u8 {
            res.push(stringify!(devlink::OpSbOccSnapshotDo));
        }
        if cmd == 28u8 {
            res.push(stringify!(devlink::OpSbOccMaxClearDo));
        }
        if cmd == 29u8 {
            res.push(stringify!(devlink::OpEswitchGetDo));
        }
        if cmd == 30u8 {
            res.push(stringify!(devlink::OpEswitchSetDo));
        }
        if cmd == 31u8 {
            res.push(stringify!(devlink::OpDpipeTableGetDo));
        }
        if cmd == 32u8 {
            res.push(stringify!(devlink::OpDpipeEntriesGetDo));
        }
        if cmd == 33u8 {
            res.push(stringify!(devlink::OpDpipeHeadersGetDo));
        }
        if cmd == 34u8 {
            res.push(stringify!(devlink::OpDpipeTableCountersSetDo));
        }
        if cmd == 35u8 {
            res.push(stringify!(devlink::OpResourceSetDo));
        }
        if cmd == 36u8 {
            res.push(stringify!(devlink::OpResourceDumpDump));
        }
        if cmd == 36u8 {
            res.push(stringify!(devlink::OpResourceDumpDo));
        }
        if cmd == 37u8 {
            res.push(stringify!(devlink::OpReloadDo));
        }
        if cmd == 38u8 {
            res.push(stringify!(devlink::OpParamGetDump));
        }
        if cmd == 38u8 {
            res.push(stringify!(devlink::OpParamGetDo));
        }
        if cmd == 39u8 {
            res.push(stringify!(devlink::OpParamSetDo));
        }
        if cmd == 42u8 {
            res.push(stringify!(devlink::OpRegionGetDump));
        }
        if cmd == 42u8 {
            res.push(stringify!(devlink::OpRegionGetDo));
        }
        if cmd == 44u8 {
            res.push(stringify!(devlink::OpRegionNewDo));
        }
        if cmd == 45u8 {
            res.push(stringify!(devlink::OpRegionDelDo));
        }
        if cmd == 46u8 {
            res.push(stringify!(devlink::OpRegionReadDump));
        }
        if cmd == 47u8 {
            res.push(stringify!(devlink::OpPortParamGetDump));
        }
        if cmd == 47u8 {
            res.push(stringify!(devlink::OpPortParamGetDo));
        }
        if cmd == 48u8 {
            res.push(stringify!(devlink::OpPortParamSetDo));
        }
        if cmd == 51u8 {
            res.push(stringify!(devlink::OpInfoGetDump));
        }
        if cmd == 51u8 {
            res.push(stringify!(devlink::OpInfoGetDo));
        }
        if cmd == 52u8 {
            res.push(stringify!(devlink::OpHealthReporterGetDump));
        }
        if cmd == 52u8 {
            res.push(stringify!(devlink::OpHealthReporterGetDo));
        }
        if cmd == 53u8 {
            res.push(stringify!(devlink::OpHealthReporterSetDo));
        }
        if cmd == 54u8 {
            res.push(stringify!(devlink::OpHealthReporterRecoverDo));
        }
        if cmd == 55u8 {
            res.push(stringify!(devlink::OpHealthReporterDiagnoseDo));
        }
        if cmd == 56u8 {
            res.push(stringify!(devlink::OpHealthReporterDumpGetDump));
        }
        if cmd == 57u8 {
            res.push(stringify!(devlink::OpHealthReporterDumpClearDo));
        }
        if cmd == 58u8 {
            res.push(stringify!(devlink::OpFlashUpdateDo));
        }
        if cmd == 63u8 {
            res.push(stringify!(devlink::OpTrapGetDump));
        }
        if cmd == 63u8 {
            res.push(stringify!(devlink::OpTrapGetDo));
        }
        if cmd == 62u8 {
            res.push(stringify!(devlink::OpTrapSetDo));
        }
        if cmd == 67u8 {
            res.push(stringify!(devlink::OpTrapGroupGetDump));
        }
        if cmd == 67u8 {
            res.push(stringify!(devlink::OpTrapGroupGetDo));
        }
        if cmd == 66u8 {
            res.push(stringify!(devlink::OpTrapGroupSetDo));
        }
        if cmd == 71u8 {
            res.push(stringify!(devlink::OpTrapPolicerGetDump));
        }
        if cmd == 71u8 {
            res.push(stringify!(devlink::OpTrapPolicerGetDo));
        }
        if cmd == 70u8 {
            res.push(stringify!(devlink::OpTrapPolicerSetDo));
        }
        if cmd == 73u8 {
            res.push(stringify!(devlink::OpHealthReporterTestDo));
        }
        if cmd == 76u8 {
            res.push(stringify!(devlink::OpRateGetDump));
        }
        if cmd == 76u8 {
            res.push(stringify!(devlink::OpRateGetDo));
        }
        if cmd == 75u8 {
            res.push(stringify!(devlink::OpRateSetDo));
        }
        if cmd == 76u8 {
            res.push(stringify!(devlink::OpRateNewDo));
        }
        if cmd == 77u8 {
            res.push(stringify!(devlink::OpRateDelDo));
        }
        if cmd == 80u8 {
            res.push(stringify!(devlink::OpLinecardGetDump));
        }
        if cmd == 80u8 {
            res.push(stringify!(devlink::OpLinecardGetDo));
        }
        if cmd == 79u8 {
            res.push(stringify!(devlink::OpLinecardSetDo));
        }
        if cmd == 82u8 {
            res.push(stringify!(devlink::OpSelftestsGetDump));
        }
        if cmd == 82u8 {
            res.push(stringify!(devlink::OpSelftestsGetDo));
        }
        if cmd == 83u8 {
            res.push(stringify!(devlink::OpSelftestsRunDo));
        }
        if cmd == 84u8 {
            res.push(stringify!(devlink::OpNotifyFilterSetDo));
        }
    }
    if proto == b"dpll" {
        if cmd == 1u8 {
            res.push(stringify!(dpll::OpDeviceIdGetDo));
        }
        if cmd == 2u8 {
            res.push(stringify!(dpll::OpDeviceGetDump));
        }
        if cmd == 2u8 {
            res.push(stringify!(dpll::OpDeviceGetDo));
        }
        if cmd == 3u8 {
            res.push(stringify!(dpll::OpDeviceSetDo));
        }
        if cmd == 7u8 {
            res.push(stringify!(dpll::OpPinIdGetDo));
        }
        if cmd == 8u8 {
            res.push(stringify!(dpll::OpPinGetDump));
        }
        if cmd == 8u8 {
            res.push(stringify!(dpll::OpPinGetDo));
        }
        if cmd == 9u8 {
            res.push(stringify!(dpll::OpPinSetDo));
        }
    }
    if proto == b"drm-ras" {
        if cmd == 1u8 {
            res.push(stringify!(drm_ras::OpListNodesDump));
        }
        if cmd == 2u8 {
            res.push(stringify!(drm_ras::OpGetErrorCounterDump));
        }
        if cmd == 2u8 {
            res.push(stringify!(drm_ras::OpGetErrorCounterDo));
        }
        if cmd == 3u8 {
            res.push(stringify!(drm_ras::OpClearErrorCounterDo));
        }
    }
    if proto == b"ethtool" {
        if cmd == 1u8 {
            res.push(stringify!(ethtool::OpStrsetGetDump));
        }
        if cmd == 1u8 {
            res.push(stringify!(ethtool::OpStrsetGetDo));
        }
        if cmd == 2u8 {
            res.push(stringify!(ethtool::OpLinkinfoGetDump));
        }
        if cmd == 2u8 {
            res.push(stringify!(ethtool::OpLinkinfoGetDo));
        }
        if cmd == 3u8 {
            res.push(stringify!(ethtool::OpLinkinfoSetDo));
        }
        if cmd == 5u8 {
            res.push(stringify!(ethtool::OpLinkmodesGetDump));
        }
        if cmd == 5u8 {
            res.push(stringify!(ethtool::OpLinkmodesGetDo));
        }
        if cmd == 6u8 {
            res.push(stringify!(ethtool::OpLinkmodesSetDo));
        }
        if cmd == 8u8 {
            res.push(stringify!(ethtool::OpLinkstateGetDump));
        }
        if cmd == 8u8 {
            res.push(stringify!(ethtool::OpLinkstateGetDo));
        }
        if cmd == 9u8 {
            res.push(stringify!(ethtool::OpDebugGetDump));
        }
        if cmd == 9u8 {
            res.push(stringify!(ethtool::OpDebugGetDo));
        }
        if cmd == 10u8 {
            res.push(stringify!(ethtool::OpDebugSetDo));
        }
        if cmd == 12u8 {
            res.push(stringify!(ethtool::OpWolGetDump));
        }
        if cmd == 12u8 {
            res.push(stringify!(ethtool::OpWolGetDo));
        }
        if cmd == 13u8 {
            res.push(stringify!(ethtool::OpWolSetDo));
        }
        if cmd == 15u8 {
            res.push(stringify!(ethtool::OpFeaturesGetDump));
        }
        if cmd == 15u8 {
            res.push(stringify!(ethtool::OpFeaturesGetDo));
        }
        if cmd == 16u8 {
            res.push(stringify!(ethtool::OpFeaturesSetDo));
        }
        if cmd == 18u8 {
            res.push(stringify!(ethtool::OpPrivflagsGetDump));
        }
        if cmd == 18u8 {
            res.push(stringify!(ethtool::OpPrivflagsGetDo));
        }
        if cmd == 19u8 {
            res.push(stringify!(ethtool::OpPrivflagsSetDo));
        }
        if cmd == 21u8 {
            res.push(stringify!(ethtool::OpRingsGetDump));
        }
        if cmd == 21u8 {
            res.push(stringify!(ethtool::OpRingsGetDo));
        }
        if cmd == 22u8 {
            res.push(stringify!(ethtool::OpRingsSetDo));
        }
        if cmd == 24u8 {
            res.push(stringify!(ethtool::OpChannelsGetDump));
        }
        if cmd == 24u8 {
            res.push(stringify!(ethtool::OpChannelsGetDo));
        }
        if cmd == 25u8 {
            res.push(stringify!(ethtool::OpChannelsSetDo));
        }
        if cmd == 27u8 {
            res.push(stringify!(ethtool::OpCoalesceGetDump));
        }
        if cmd == 27u8 {
            res.push(stringify!(ethtool::OpCoalesceGetDo));
        }
        if cmd == 28u8 {
            res.push(stringify!(ethtool::OpCoalesceSetDo));
        }
        if cmd == 30u8 {
            res.push(stringify!(ethtool::OpPauseGetDump));
        }
        if cmd == 30u8 {
            res.push(stringify!(ethtool::OpPauseGetDo));
        }
        if cmd == 31u8 {
            res.push(stringify!(ethtool::OpPauseSetDo));
        }
        if cmd == 33u8 {
            res.push(stringify!(ethtool::OpEeeGetDump));
        }
        if cmd == 33u8 {
            res.push(stringify!(ethtool::OpEeeGetDo));
        }
        if cmd == 34u8 {
            res.push(stringify!(ethtool::OpEeeSetDo));
        }
        if cmd == 36u8 {
            res.push(stringify!(ethtool::OpTsinfoGetDump));
        }
        if cmd == 36u8 {
            res.push(stringify!(ethtool::OpTsinfoGetDo));
        }
        if cmd == 37u8 {
            res.push(stringify!(ethtool::OpCableTestActDo));
        }
        if cmd == 39u8 {
            res.push(stringify!(ethtool::OpCableTestTdrActDo));
        }
        if cmd == 41u8 {
            res.push(stringify!(ethtool::OpTunnelInfoGetDump));
        }
        if cmd == 41u8 {
            res.push(stringify!(ethtool::OpTunnelInfoGetDo));
        }
        if cmd == 42u8 {
            res.push(stringify!(ethtool::OpFecGetDump));
        }
        if cmd == 42u8 {
            res.push(stringify!(ethtool::OpFecGetDo));
        }
        if cmd == 43u8 {
            res.push(stringify!(ethtool::OpFecSetDo));
        }
        if cmd == 45u8 {
            res.push(stringify!(ethtool::OpModuleEepromGetDump));
        }
        if cmd == 45u8 {
            res.push(stringify!(ethtool::OpModuleEepromGetDo));
        }
        if cmd == 46u8 {
            res.push(stringify!(ethtool::OpStatsGetDump));
        }
        if cmd == 46u8 {
            res.push(stringify!(ethtool::OpStatsGetDo));
        }
        if cmd == 47u8 {
            res.push(stringify!(ethtool::OpPhcVclocksGetDump));
        }
        if cmd == 47u8 {
            res.push(stringify!(ethtool::OpPhcVclocksGetDo));
        }
        if cmd == 48u8 {
            res.push(stringify!(ethtool::OpModuleGetDump));
        }
        if cmd == 48u8 {
            res.push(stringify!(ethtool::OpModuleGetDo));
        }
        if cmd == 49u8 {
            res.push(stringify!(ethtool::OpModuleSetDo));
        }
        if cmd == 51u8 {
            res.push(stringify!(ethtool::OpPseGetDump));
        }
        if cmd == 51u8 {
            res.push(stringify!(ethtool::OpPseGetDo));
        }
        if cmd == 52u8 {
            res.push(stringify!(ethtool::OpPseSetDo));
        }
        if cmd == 53u8 {
            res.push(stringify!(ethtool::OpRssGetDump));
        }
        if cmd == 53u8 {
            res.push(stringify!(ethtool::OpRssGetDo));
        }
        if cmd == 54u8 {
            res.push(stringify!(ethtool::OpPlcaGetCfgDump));
        }
        if cmd == 54u8 {
            res.push(stringify!(ethtool::OpPlcaGetCfgDo));
        }
        if cmd == 55u8 {
            res.push(stringify!(ethtool::OpPlcaSetCfgDo));
        }
        if cmd == 56u8 {
            res.push(stringify!(ethtool::OpPlcaGetStatusDump));
        }
        if cmd == 56u8 {
            res.push(stringify!(ethtool::OpPlcaGetStatusDo));
        }
        if cmd == 58u8 {
            res.push(stringify!(ethtool::OpMmGetDump));
        }
        if cmd == 58u8 {
            res.push(stringify!(ethtool::OpMmGetDo));
        }
        if cmd == 59u8 {
            res.push(stringify!(ethtool::OpMmSetDo));
        }
        if cmd == 61u8 {
            res.push(stringify!(ethtool::OpModuleFwFlashActDo));
        }
        if cmd == 63u8 {
            res.push(stringify!(ethtool::OpPhyGetDump));
        }
        if cmd == 63u8 {
            res.push(stringify!(ethtool::OpPhyGetDo));
        }
        if cmd == 65u8 {
            res.push(stringify!(ethtool::OpTsconfigGetDump));
        }
        if cmd == 65u8 {
            res.push(stringify!(ethtool::OpTsconfigGetDo));
        }
        if cmd == 66u8 {
            res.push(stringify!(ethtool::OpTsconfigSetDo));
        }
        if cmd == 68u8 {
            res.push(stringify!(ethtool::OpRssSetDo));
        }
        if cmd == 70u8 {
            res.push(stringify!(ethtool::OpRssCreateActDo));
        }
        if cmd == 72u8 {
            res.push(stringify!(ethtool::OpRssDeleteActDo));
        }
        if cmd == 74u8 {
            res.push(stringify!(ethtool::OpMseGetDump));
        }
        if cmd == 74u8 {
            res.push(stringify!(ethtool::OpMseGetDo));
        }
    }
    if proto == b"fou" {
        if cmd == 1u8 {
            res.push(stringify!(fou::OpAddDo));
        }
        if cmd == 2u8 {
            res.push(stringify!(fou::OpDelDo));
        }
        if cmd == 3u8 {
            res.push(stringify!(fou::OpGetDump));
        }
        if cmd == 3u8 {
            res.push(stringify!(fou::OpGetDo));
        }
    }
    if proto == b"handshake" {
        if cmd == 2u8 {
            res.push(stringify!(handshake::OpAcceptDo));
        }
        if cmd == 3u8 {
            res.push(stringify!(handshake::OpDoneDo));
        }
    }
    if proto == b"lockd" {
        if cmd == 1u8 {
            res.push(stringify!(lockd::OpServerSetDo));
        }
        if cmd == 2u8 {
            res.push(stringify!(lockd::OpServerGetDo));
        }
    }
    if proto == b"mptcp_pm" {
        if cmd == 1u8 {
            res.push(stringify!(mptcp_pm::OpAddAddrDo));
        }
        if cmd == 2u8 {
            res.push(stringify!(mptcp_pm::OpDelAddrDo));
        }
        if cmd == 3u8 {
            res.push(stringify!(mptcp_pm::OpGetAddrDump));
        }
        if cmd == 3u8 {
            res.push(stringify!(mptcp_pm::OpGetAddrDo));
        }
        if cmd == 4u8 {
            res.push(stringify!(mptcp_pm::OpFlushAddrsDo));
        }
        if cmd == 5u8 {
            res.push(stringify!(mptcp_pm::OpSetLimitsDo));
        }
        if cmd == 6u8 {
            res.push(stringify!(mptcp_pm::OpGetLimitsDo));
        }
        if cmd == 7u8 {
            res.push(stringify!(mptcp_pm::OpSetFlagsDo));
        }
        if cmd == 8u8 {
            res.push(stringify!(mptcp_pm::OpAnnounceDo));
        }
        if cmd == 9u8 {
            res.push(stringify!(mptcp_pm::OpRemoveDo));
        }
        if cmd == 10u8 {
            res.push(stringify!(mptcp_pm::OpSubflowCreateDo));
        }
        if cmd == 11u8 {
            res.push(stringify!(mptcp_pm::OpSubflowDestroyDo));
        }
    }
    if proto == b"net-shaper" {
        if cmd == 1u8 {
            res.push(stringify!(net_shaper::OpGetDump));
        }
        if cmd == 1u8 {
            res.push(stringify!(net_shaper::OpGetDo));
        }
        if cmd == 2u8 {
            res.push(stringify!(net_shaper::OpSetDo));
        }
        if cmd == 3u8 {
            res.push(stringify!(net_shaper::OpDeleteDo));
        }
        if cmd == 4u8 {
            res.push(stringify!(net_shaper::OpGroupDo));
        }
        if cmd == 5u8 {
            res.push(stringify!(net_shaper::OpCapGetDump));
        }
        if cmd == 5u8 {
            res.push(stringify!(net_shaper::OpCapGetDo));
        }
    }
    if proto == b"netdev" {
        if cmd == 1u8 {
            res.push(stringify!(netdev::OpDevGetDump));
        }
        if cmd == 1u8 {
            res.push(stringify!(netdev::OpDevGetDo));
        }
        if cmd == 5u8 {
            res.push(stringify!(netdev::OpPagePoolGetDump));
        }
        if cmd == 5u8 {
            res.push(stringify!(netdev::OpPagePoolGetDo));
        }
        if cmd == 9u8 {
            res.push(stringify!(netdev::OpPagePoolStatsGetDump));
        }
        if cmd == 9u8 {
            res.push(stringify!(netdev::OpPagePoolStatsGetDo));
        }
        if cmd == 10u8 {
            res.push(stringify!(netdev::OpQueueGetDump));
        }
        if cmd == 10u8 {
            res.push(stringify!(netdev::OpQueueGetDo));
        }
        if cmd == 11u8 {
            res.push(stringify!(netdev::OpNapiGetDump));
        }
        if cmd == 11u8 {
            res.push(stringify!(netdev::OpNapiGetDo));
        }
        if cmd == 12u8 {
            res.push(stringify!(netdev::OpQstatsGetDump));
        }
        if cmd == 13u8 {
            res.push(stringify!(netdev::OpBindRxDo));
        }
        if cmd == 14u8 {
            res.push(stringify!(netdev::OpNapiSetDo));
        }
        if cmd == 15u8 {
            res.push(stringify!(netdev::OpBindTxDo));
        }
        if cmd == 16u8 {
            res.push(stringify!(netdev::OpQueueCreateDo));
        }
    }
    if proto == b"nfsd" {
        if cmd == 1u8 {
            res.push(stringify!(nfsd::OpRpcStatusGetDump));
        }
        if cmd == 2u8 {
            res.push(stringify!(nfsd::OpThreadsSetDo));
        }
        if cmd == 3u8 {
            res.push(stringify!(nfsd::OpThreadsGetDo));
        }
        if cmd == 4u8 {
            res.push(stringify!(nfsd::OpVersionSetDo));
        }
        if cmd == 5u8 {
            res.push(stringify!(nfsd::OpVersionGetDo));
        }
        if cmd == 6u8 {
            res.push(stringify!(nfsd::OpListenerSetDo));
        }
        if cmd == 7u8 {
            res.push(stringify!(nfsd::OpListenerGetDo));
        }
        if cmd == 8u8 {
            res.push(stringify!(nfsd::OpPoolModeSetDo));
        }
        if cmd == 9u8 {
            res.push(stringify!(nfsd::OpPoolModeGetDo));
        }
        if cmd == 11u8 {
            res.push(stringify!(nfsd::OpSvcExportGetReqsDump));
        }
        if cmd == 12u8 {
            res.push(stringify!(nfsd::OpSvcExportSetReqsDo));
        }
        if cmd == 13u8 {
            res.push(stringify!(nfsd::OpExpkeyGetReqsDump));
        }
        if cmd == 14u8 {
            res.push(stringify!(nfsd::OpExpkeySetReqsDo));
        }
        if cmd == 15u8 {
            res.push(stringify!(nfsd::OpCacheFlushDo));
        }
        if cmd == 16u8 {
            res.push(stringify!(nfsd::OpUnlockIpDo));
        }
        if cmd == 17u8 {
            res.push(stringify!(nfsd::OpUnlockFilesystemDo));
        }
        if cmd == 18u8 {
            res.push(stringify!(nfsd::OpUnlockExportDo));
        }
    }
    if proto == b"nl80211" {
        if cmd == 3u8 {
            res.push(stringify!(nl80211::OpGetWiphyDump));
        }
        if cmd == 3u8 {
            res.push(stringify!(nl80211::OpGetWiphyDo));
        }
        if cmd == 7u8 {
            res.push(stringify!(nl80211::OpGetInterfaceDump));
        }
        if cmd == 7u8 {
            res.push(stringify!(nl80211::OpGetInterfaceDo));
        }
        if cmd == 95u8 {
            res.push(stringify!(nl80211::OpGetProtocolFeaturesDo));
        }
    }
    if proto == b"nlctrl" {
        if cmd == 1u8 {
            res.push(stringify!(nlctrl::OpGetfamilyDump));
        }
        if cmd == 1u8 {
            res.push(stringify!(nlctrl::OpGetfamilyDo));
        }
        if cmd == 10u8 {
            res.push(stringify!(nlctrl::OpGetpolicyDump));
        }
    }
    if proto == b"ovpn" {
        if cmd == 1u8 {
            res.push(stringify!(ovpn::OpPeerNewDo));
        }
        if cmd == 2u8 {
            res.push(stringify!(ovpn::OpPeerSetDo));
        }
        if cmd == 3u8 {
            res.push(stringify!(ovpn::OpPeerGetDump));
        }
        if cmd == 3u8 {
            res.push(stringify!(ovpn::OpPeerGetDo));
        }
        if cmd == 4u8 {
            res.push(stringify!(ovpn::OpPeerDelDo));
        }
        if cmd == 6u8 {
            res.push(stringify!(ovpn::OpKeyNewDo));
        }
        if cmd == 7u8 {
            res.push(stringify!(ovpn::OpKeyGetDo));
        }
        if cmd == 8u8 {
            res.push(stringify!(ovpn::OpKeySwapDo));
        }
        if cmd == 10u8 {
            res.push(stringify!(ovpn::OpKeyDelDo));
        }
    }
    if proto == b"ovs_datapath" {
        if cmd == 3u8 {
            res.push(stringify!(ovs_datapath::OpGetDump));
        }
        if cmd == 3u8 {
            res.push(stringify!(ovs_datapath::OpGetDo));
        }
        if cmd == 1u8 {
            res.push(stringify!(ovs_datapath::OpNewDo));
        }
        if cmd == 2u8 {
            res.push(stringify!(ovs_datapath::OpDelDo));
        }
    }
    if proto == b"ovs_flow" {
        if cmd == 3u8 {
            res.push(stringify!(ovs_flow::OpGetDump));
        }
        if cmd == 3u8 {
            res.push(stringify!(ovs_flow::OpGetDo));
        }
        if cmd == 1u8 {
            res.push(stringify!(ovs_flow::OpNewDo));
        }
    }
    if proto == b"ovs_packet" {
        if cmd == 3u8 {
            res.push(stringify!(ovs_packet::OpExecuteDo));
        }
    }
    if proto == b"ovs_vport" {
        if cmd == 1u8 {
            res.push(stringify!(ovs_vport::OpNewDo));
        }
        if cmd == 2u8 {
            res.push(stringify!(ovs_vport::OpDelDo));
        }
        if cmd == 3u8 {
            res.push(stringify!(ovs_vport::OpGetDump));
        }
        if cmd == 3u8 {
            res.push(stringify!(ovs_vport::OpGetDo));
        }
    }
    if proto == b"psp" {
        if cmd == 1u8 {
            res.push(stringify!(psp::OpDevGetDump));
        }
        if cmd == 1u8 {
            res.push(stringify!(psp::OpDevGetDo));
        }
        if cmd == 4u8 {
            res.push(stringify!(psp::OpDevSetDo));
        }
        if cmd == 6u8 {
            res.push(stringify!(psp::OpKeyRotateDo));
        }
        if cmd == 8u8 {
            res.push(stringify!(psp::OpRxAssocDo));
        }
        if cmd == 9u8 {
            res.push(stringify!(psp::OpTxAssocDo));
        }
        if cmd == 10u8 {
            res.push(stringify!(psp::OpGetStatsDump));
        }
        if cmd == 10u8 {
            res.push(stringify!(psp::OpGetStatsDo));
        }
        if cmd == 11u8 {
            res.push(stringify!(psp::OpDevAssocDo));
        }
        if cmd == 12u8 {
            res.push(stringify!(psp::OpDevDisassocDo));
        }
    }
    if proto == b"sunrpc" {
        if cmd == 2u8 {
            res.push(stringify!(sunrpc::OpIpMapGetReqsDump));
        }
        if cmd == 3u8 {
            res.push(stringify!(sunrpc::OpIpMapSetReqsDo));
        }
        if cmd == 4u8 {
            res.push(stringify!(sunrpc::OpUnixGidGetReqsDump));
        }
        if cmd == 5u8 {
            res.push(stringify!(sunrpc::OpUnixGidSetReqsDo));
        }
        if cmd == 6u8 {
            res.push(stringify!(sunrpc::OpCacheFlushDo));
        }
    }
    if proto == b"tcp_metrics" {
        if cmd == 1u8 {
            res.push(stringify!(tcp_metrics::OpGetDump));
        }
        if cmd == 1u8 {
            res.push(stringify!(tcp_metrics::OpGetDo));
        }
        if cmd == 2u8 {
            res.push(stringify!(tcp_metrics::OpDelDo));
        }
    }
    if proto == b"team" {
        if cmd == 0u8 {
            res.push(stringify!(team::OpNoopDo));
        }
        if cmd == 1u8 {
            res.push(stringify!(team::OpOptionsSetDo));
        }
        if cmd == 2u8 {
            res.push(stringify!(team::OpOptionsGetDo));
        }
        if cmd == 3u8 {
            res.push(stringify!(team::OpPortListGetDo));
        }
    }
    if proto == b"wireguard" {
        if cmd == 0u8 {
            res.push(stringify!(wireguard::OpGetDeviceDump));
        }
        if cmd == 1u8 {
            res.push(stringify!(wireguard::OpSetDeviceDo));
        }
    }
    res
}
