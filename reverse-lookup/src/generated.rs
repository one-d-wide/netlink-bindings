#![allow(clippy::all)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]
#![allow(unreachable_code)]
#![allow(unreachable_patterns)]
use netlink_bindings::{
    builtin::PushBuiltinNfgenmsg,
    traits::{NetlinkRequest, Protocol},
};
use std::fmt::Debug;
#[derive(Clone)]
pub struct ReverseLookup<'a> {
    pub proto: Protocol,
    pub value: u16,
    pub request_value: Option<u16>,
    pub is_dump: bool,
    pub buf: &'a [u8],
}
fn consider(fmt: &mut std::fmt::Formatter<'_>, proto: &str) -> std::fmt::Result {
    write!(
        fmt,
        "Protocol {0:?} not enabled, consider --features={0}",
        proto
    )
}
impl Debug for ReverseLookup<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            proto,
            value,
            request_value,
            is_dump,
            buf,
        } = self.clone();
        match proto {
            Protocol::Raw { protonum, .. } => {
                if protonum == 0u16 {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "rt-addr")]
                    if let (20u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::rt_addr::OpNewaddrDoRequest::new(buf),
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
                            &netlink_bindings::rt_addr::OpNewaddrDoReply::new(buf),
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
                            &netlink_bindings::rt_addr::OpDeladdrDoRequest::new(buf),
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
                            &netlink_bindings::rt_addr::OpDeladdrDoReply::new(buf),
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
                            &netlink_bindings::rt_addr::OpGetaddrDumpRequest::new(buf),
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
                            &netlink_bindings::rt_addr::OpGetaddrDumpReply::new(buf),
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
                            &netlink_bindings::rt_addr::OpGetmulticastDumpRequest::new(buf),
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
                            &netlink_bindings::rt_addr::OpGetmulticastDumpReply::new(buf),
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
                            &netlink_bindings::rt_addr::OpGetmulticastDoRequest::new(buf),
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
                            &netlink_bindings::rt_addr::OpGetmulticastDoReply::new(buf),
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
                            &netlink_bindings::rt_link::OpNewlinkDoRequest::new(buf),
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
                            &netlink_bindings::rt_link::OpNewlinkDoReply::new(buf),
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
                            &netlink_bindings::rt_link::OpDellinkDoRequest::new(buf),
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
                            &netlink_bindings::rt_link::OpDellinkDoReply::new(buf),
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
                            &netlink_bindings::rt_link::OpGetlinkDumpRequest::new(buf),
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
                            &netlink_bindings::rt_link::OpGetlinkDumpReply::new(buf),
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
                            &netlink_bindings::rt_link::OpGetlinkDoRequest::new(buf),
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
                            &netlink_bindings::rt_link::OpGetlinkDoReply::new(buf),
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
                            &netlink_bindings::rt_link::OpSetlinkDoRequest::new(buf),
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
                            &netlink_bindings::rt_link::OpSetlinkDoReply::new(buf),
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
                            &netlink_bindings::rt_link::OpGetstatsDumpRequest::new(buf),
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
                            &netlink_bindings::rt_link::OpGetstatsDumpReply::new(buf),
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
                            &netlink_bindings::rt_link::OpGetstatsDoRequest::new(buf),
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
                            &netlink_bindings::rt_link::OpGetstatsDoReply::new(buf),
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
                            &netlink_bindings::rt_neigh::OpNewneighDoRequest::new(buf),
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
                            &netlink_bindings::rt_neigh::OpNewneighDoReply::new(buf),
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
                            &netlink_bindings::rt_neigh::OpDelneighDoRequest::new(buf),
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
                            &netlink_bindings::rt_neigh::OpDelneighDoReply::new(buf),
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
                            &netlink_bindings::rt_neigh::OpGetneighDumpRequest::new(buf),
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
                            &netlink_bindings::rt_neigh::OpGetneighDumpReply::new(buf),
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
                            &netlink_bindings::rt_neigh::OpGetneighDoRequest::new(buf),
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
                            &netlink_bindings::rt_neigh::OpGetneighDoReply::new(buf),
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
                            &netlink_bindings::rt_neigh::OpGetneightblDumpRequest::new(buf),
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
                            &netlink_bindings::rt_neigh::OpGetneightblDumpReply::new(buf),
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
                            &netlink_bindings::rt_neigh::OpSetneightblDoRequest::new(buf),
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
                            &netlink_bindings::rt_neigh::OpSetneightblDoReply::new(buf),
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
                            &netlink_bindings::rt_route::OpGetrouteDumpRequest::new(buf),
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
                            &netlink_bindings::rt_route::OpGetrouteDumpReply::new(buf),
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
                            &netlink_bindings::rt_route::OpGetrouteDoRequest::new(buf),
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
                            &netlink_bindings::rt_route::OpGetrouteDoReply::new(buf),
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
                            &netlink_bindings::rt_route::OpNewrouteDoRequest::new(buf),
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
                            &netlink_bindings::rt_route::OpNewrouteDoReply::new(buf),
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
                            &netlink_bindings::rt_route::OpDelrouteDoRequest::new(buf),
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
                            &netlink_bindings::rt_route::OpDelrouteDoReply::new(buf),
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
                            &netlink_bindings::rt_rule::OpNewruleDoRequest::new(buf),
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
                            &netlink_bindings::rt_rule::OpNewruleDoReply::new(buf),
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
                            &netlink_bindings::rt_rule::OpDelruleDoRequest::new(buf),
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
                            &netlink_bindings::rt_rule::OpDelruleDoReply::new(buf),
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
                            &netlink_bindings::rt_rule::OpGetruleDumpRequest::new(buf),
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
                            &netlink_bindings::rt_rule::OpGetruleDumpReply::new(buf),
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
                            &netlink_bindings::tc::OpNewqdiscDoRequest::new(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (36u16, None, false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (36u16, Some(36u16), false) = pat {
                        return Debug::fmt(&netlink_bindings::tc::OpNewqdiscDoReply::new(buf), fmt);
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (36u16, Some(36u16), false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (37u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpDelqdiscDoRequest::new(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (37u16, None, false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (37u16, Some(37u16), false) = pat {
                        return Debug::fmt(&netlink_bindings::tc::OpDelqdiscDoReply::new(buf), fmt);
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (37u16, Some(37u16), false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (38u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpGetqdiscDumpRequest::new(buf),
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
                            &netlink_bindings::tc::OpGetqdiscDumpReply::new(buf),
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
                            &netlink_bindings::tc::OpGetqdiscDoRequest::new(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (38u16, None, false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (36u16, Some(38u16), false) = pat {
                        return Debug::fmt(&netlink_bindings::tc::OpGetqdiscDoReply::new(buf), fmt);
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (36u16, Some(38u16), false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (40u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpNewtclassDoRequest::new(buf),
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
                            &netlink_bindings::tc::OpNewtclassDoReply::new(buf),
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
                            &netlink_bindings::tc::OpDeltclassDoRequest::new(buf),
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
                            &netlink_bindings::tc::OpDeltclassDoReply::new(buf),
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
                            &netlink_bindings::tc::OpGettclassDoRequest::new(buf),
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
                            &netlink_bindings::tc::OpGettclassDoReply::new(buf),
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
                            &netlink_bindings::tc::OpNewtfilterDoRequest::new(buf),
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
                            &netlink_bindings::tc::OpNewtfilterDoReply::new(buf),
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
                            &netlink_bindings::tc::OpDeltfilterDoRequest::new(buf),
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
                            &netlink_bindings::tc::OpDeltfilterDoReply::new(buf),
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
                            &netlink_bindings::tc::OpGettfilterDumpRequest::new(buf),
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
                            &netlink_bindings::tc::OpGettfilterDumpReply::new(buf),
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
                            &netlink_bindings::tc::OpGettfilterDoRequest::new(buf),
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
                            &netlink_bindings::tc::OpGettfilterDoReply::new(buf),
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
                            &netlink_bindings::tc::OpNewchainDoRequest::new(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (100u16, None, false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (100u16, Some(100u16), false) = pat {
                        return Debug::fmt(&netlink_bindings::tc::OpNewchainDoReply::new(buf), fmt);
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (100u16, Some(100u16), false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (101u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpDelchainDoRequest::new(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (101u16, None, false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (101u16, Some(101u16), false) = pat {
                        return Debug::fmt(&netlink_bindings::tc::OpDelchainDoReply::new(buf), fmt);
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (101u16, Some(101u16), false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (102u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::tc::OpGetchainDoRequest::new(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "tc"))]
                    if let (102u16, None, false) = pat {
                        return consider(fmt, "tc");
                    }
                    #[cfg(feature = "tc")]
                    if let (100u16, Some(102u16), false) = pat {
                        return Debug::fmt(&netlink_bindings::tc::OpGetchainDoReply::new(buf), fmt);
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
                if protonum == 12u16 {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "conntrack")]
                    if let (257u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::conntrack::OpGetDumpRequest::new(buf),
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
                            &netlink_bindings::conntrack::OpGetDumpReply::new(buf),
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
                            &netlink_bindings::conntrack::OpGetDoRequest::new(buf),
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
                            &netlink_bindings::conntrack::OpGetDoReply::new(buf),
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
                            &netlink_bindings::conntrack::OpGetStatsDumpRequest::new(buf),
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
                            &netlink_bindings::conntrack::OpGetStatsDumpReply::new(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "conntrack"))]
                    if let (260u16, Some(260u16), true) = pat {
                        return consider(fmt, "conntrack");
                    }
                    #[cfg(feature = "nftables")]
                    if let (2816u16, None, true) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpGetcompatDumpRequest::new(buf),
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
                            &netlink_bindings::nftables::OpGetcompatDumpReply::new(buf),
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
                            &netlink_bindings::nftables::OpGetcompatDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpGetcompatDoReply::new(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2816u16, Some(2816u16), false) = pat {
                        return consider(fmt, "nftables");
                    }
                    #[cfg(feature = "nftables")]
                    if let (16u16, None, false) = pat {
                        return Debug::fmt(
                            &netlink_bindings::nftables::OpBatchBeginDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpBatchBeginDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpBatchEndDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpBatchEndDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpNewtableDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpNewtableDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpGettableDumpRequest::new(buf),
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
                            &netlink_bindings::nftables::OpGettableDumpReply::new(buf),
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
                            &netlink_bindings::nftables::OpGettableDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpGettableDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpDeltableDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpDeltableDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpDestroytableDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpDestroytableDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpNewchainDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpNewchainDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpGetchainDumpRequest::new(buf),
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
                            &netlink_bindings::nftables::OpGetchainDumpReply::new(buf),
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
                            &netlink_bindings::nftables::OpGetchainDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpGetchainDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpDelchainDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpDelchainDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpDestroychainDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpDestroychainDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpNewruleDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpNewruleDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpGetruleDumpRequest::new(buf),
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
                            &netlink_bindings::nftables::OpGetruleDumpReply::new(buf),
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
                            &netlink_bindings::nftables::OpGetruleDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpGetruleDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpGetruleResetDumpRequest::new(buf),
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
                            &netlink_bindings::nftables::OpGetruleResetDumpReply::new(buf),
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
                            &netlink_bindings::nftables::OpGetruleResetDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpGetruleResetDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpDelruleDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpDelruleDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpDestroyruleDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpDestroyruleDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpNewsetDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpNewsetDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpGetsetDumpRequest::new(buf),
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
                            &netlink_bindings::nftables::OpGetsetDumpReply::new(buf),
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
                            &netlink_bindings::nftables::OpGetsetDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpGetsetDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpDelsetDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpDelsetDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpDestroysetDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpDestroysetDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpNewsetelemDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpNewsetelemDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpGetsetelemDumpRequest::new(buf),
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
                            &netlink_bindings::nftables::OpGetsetelemDumpReply::new(buf),
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
                            &netlink_bindings::nftables::OpGetsetelemDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpGetsetelemDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpGetsetelemResetDumpRequest::new(buf),
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
                            &netlink_bindings::nftables::OpGetsetelemResetDumpReply::new(buf),
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
                            &netlink_bindings::nftables::OpGetsetelemResetDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpGetsetelemResetDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpDelsetelemDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpDelsetelemDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpDestroysetelemDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpDestroysetelemDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpGetgenDumpRequest::new(buf),
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
                            &netlink_bindings::nftables::OpGetgenDumpReply::new(buf),
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
                            &netlink_bindings::nftables::OpGetgenDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpGetgenDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpNewobjDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpNewobjDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpGetobjDumpRequest::new(buf),
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
                            &netlink_bindings::nftables::OpGetobjDumpReply::new(buf),
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
                            &netlink_bindings::nftables::OpGetobjDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpGetobjDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpDelobjDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpDelobjDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpDestroyobjDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpDestroyobjDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpNewflowtableDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpNewflowtableDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpGetflowtableDumpRequest::new(buf),
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
                            &netlink_bindings::nftables::OpGetflowtableDumpReply::new(buf),
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
                            &netlink_bindings::nftables::OpGetflowtableDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpGetflowtableDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpDelflowtableDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpDelflowtableDoReply::new(buf),
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
                            &netlink_bindings::nftables::OpDestroyflowtableDoRequest::new(buf),
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
                            &netlink_bindings::nftables::OpDestroyflowtableDoReply::new(buf),
                            fmt,
                        );
                    }
                    #[cfg(not(feature = "nftables"))]
                    if let (2592u16, Some(2592u16), false) = pat {
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
                if name == b"devlink" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "devlink")]
                    {
                        if let (1u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpGetDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(1u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpGetDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (1u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(1u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (5u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortGetDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (3u8, Some(5u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortGetDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (5u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (7u8, Some(5u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (6u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortSetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (6u8, Some(6u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortSetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (7u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortNewDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (7u8, Some(7u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortNewDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (8u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortDelDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (8u8, Some(8u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortDelDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (9u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortSplitDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (9u8, Some(9u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortSplitDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (10u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortUnsplitDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (10u8, Some(10u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortUnsplitDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (11u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbGetDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (13u8, Some(11u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbGetDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (11u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (13u8, Some(11u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (15u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbPoolGetDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (17u8, Some(15u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbPoolGetDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (15u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbPoolGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (17u8, Some(15u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbPoolGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (16u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbPoolSetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (16u8, Some(16u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbPoolSetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (19u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbPortPoolGetDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (21u8, Some(19u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbPortPoolGetDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (19u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbPortPoolGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (21u8, Some(19u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbPortPoolGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (20u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbPortPoolSetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (20u8, Some(20u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbPortPoolSetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (23u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbTcPoolBindGetDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (25u8, Some(23u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbTcPoolBindGetDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (23u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbTcPoolBindGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (25u8, Some(23u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbTcPoolBindGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (24u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbTcPoolBindSetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (24u8, Some(24u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbTcPoolBindSetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (27u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbOccSnapshotDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (27u8, Some(27u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbOccSnapshotDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (28u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbOccMaxClearDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (28u8, Some(28u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSbOccMaxClearDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (29u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpEswitchGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (29u8, Some(29u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpEswitchGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (30u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpEswitchSetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (30u8, Some(30u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpEswitchSetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (31u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpDpipeTableGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (31u8, Some(31u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpDpipeTableGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (32u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpDpipeEntriesGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (32u8, Some(32u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpDpipeEntriesGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (33u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpDpipeHeadersGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (33u8, Some(33u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpDpipeHeadersGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (34u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpDpipeTableCountersSetDoRequest::new(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (34u8, Some(34u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpDpipeTableCountersSetDoReply::new(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (35u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpResourceSetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (35u8, Some(35u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpResourceSetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (36u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpResourceDumpDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (36u8, Some(36u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpResourceDumpDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (37u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpReloadDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (37u8, Some(37u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpReloadDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (38u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpParamGetDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (38u8, Some(38u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpParamGetDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (38u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpParamGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (38u8, Some(38u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpParamGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (39u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpParamSetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (39u8, Some(39u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpParamSetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (42u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRegionGetDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (42u8, Some(42u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRegionGetDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (42u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRegionGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (42u8, Some(42u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRegionGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (44u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRegionNewDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (44u8, Some(44u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRegionNewDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (45u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRegionDelDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (45u8, Some(45u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRegionDelDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (46u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRegionReadDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (46u8, Some(46u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRegionReadDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (47u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortParamGetDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (47u8, Some(47u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortParamGetDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (47u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortParamGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (47u8, Some(47u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortParamGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (48u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortParamSetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (48u8, Some(48u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpPortParamSetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (51u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpInfoGetDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (51u8, Some(51u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpInfoGetDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (51u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpInfoGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (51u8, Some(51u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpInfoGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (52u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterGetDumpRequest::new(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (52u8, Some(52u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterGetDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (52u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (52u8, Some(52u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (53u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterSetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (53u8, Some(53u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterSetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (54u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterRecoverDoRequest::new(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (54u8, Some(54u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterRecoverDoReply::new(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (55u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterDiagnoseDoRequest::new(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (55u8, Some(55u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterDiagnoseDoReply::new(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (56u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterDumpGetDumpRequest::new(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (56u8, Some(56u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterDumpGetDumpReply::new(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (57u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterDumpClearDoRequest::new(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (57u8, Some(57u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterDumpClearDoReply::new(
                                    buf,
                                ),
                                fmt,
                            );
                        }
                        if let (58u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpFlashUpdateDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (58u8, Some(58u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpFlashUpdateDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (61u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapGetDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (63u8, Some(61u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapGetDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (61u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (63u8, Some(61u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (62u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapSetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (62u8, Some(62u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapSetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (65u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapGroupGetDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (67u8, Some(65u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapGroupGetDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (65u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapGroupGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (67u8, Some(65u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapGroupGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (66u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapGroupSetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (66u8, Some(66u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapGroupSetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (69u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapPolicerGetDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (71u8, Some(69u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapPolicerGetDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (69u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapPolicerGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (71u8, Some(69u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapPolicerGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (70u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapPolicerSetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (70u8, Some(70u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpTrapPolicerSetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (73u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterTestDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (73u8, Some(73u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpHealthReporterTestDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (74u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRateGetDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (76u8, Some(74u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRateGetDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (74u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRateGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (76u8, Some(74u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRateGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (75u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRateSetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (75u8, Some(75u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRateSetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (76u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRateNewDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (76u8, Some(76u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRateNewDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (77u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRateDelDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (77u8, Some(77u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpRateDelDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (78u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpLinecardGetDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (80u8, Some(78u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpLinecardGetDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (78u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpLinecardGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (80u8, Some(78u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpLinecardGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (79u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpLinecardSetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (79u8, Some(79u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpLinecardSetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (82u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSelftestsGetDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (82u8, Some(82u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSelftestsGetDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (82u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSelftestsGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (82u8, Some(82u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSelftestsGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (83u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSelftestsRunDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (83u8, Some(83u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpSelftestsRunDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (84u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpNotifyFilterSetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (84u8, Some(84u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::devlink::OpNotifyFilterSetDoReply::new(buf),
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
                if name == b"netdev" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "netdev")]
                    {
                        if let (1u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpDevGetDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpDevGetDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (1u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpDevGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpDevGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (5u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpPagePoolGetDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (5u8, Some(5u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpPagePoolGetDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (5u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpPagePoolGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (5u8, Some(5u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpPagePoolGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (9u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpPagePoolStatsGetDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (9u8, Some(9u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpPagePoolStatsGetDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (9u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpPagePoolStatsGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (9u8, Some(9u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpPagePoolStatsGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (10u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpQueueGetDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (10u8, Some(10u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpQueueGetDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (10u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpQueueGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (10u8, Some(10u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpQueueGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (11u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpNapiGetDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (11u8, Some(11u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpNapiGetDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (11u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpNapiGetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (11u8, Some(11u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpNapiGetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (12u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpQstatsGetDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (12u8, Some(12u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpQstatsGetDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (13u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpBindRxDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (13u8, Some(13u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpBindRxDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (14u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpNapiSetDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (14u8, Some(14u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpNapiSetDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (15u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpBindTxDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (15u8, Some(15u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::netdev::OpBindTxDoReply::new(buf),
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
                if name == b"nl80211" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "nl80211")]
                    {
                        if let (1u8, None, true) = pat {
                            return Debug :: fmt (& netlink_bindings :: nl80211 :: RequestOpGetWiphyDumpRequest :: decode_reply (buf) , fmt) ;
                        }
                        if let (3u8, Some(1u8), true) = pat {
                            return Debug :: fmt (& netlink_bindings :: nl80211 :: RequestOpGetWiphyDumpRequest :: decode_reply (buf) , fmt) ;
                        }
                        if let (1u8, None, false) = pat {
                            return Debug :: fmt (& netlink_bindings :: nl80211 :: RequestOpGetWiphyDoRequest :: decode_reply (buf) , fmt) ;
                        }
                        if let (3u8, Some(1u8), false) = pat {
                            return Debug :: fmt (& netlink_bindings :: nl80211 :: RequestOpGetWiphyDoRequest :: decode_reply (buf) , fmt) ;
                        }
                        if let (5u8, None, true) = pat {
                            return Debug :: fmt (& netlink_bindings :: nl80211 :: RequestOpGetInterfaceDumpRequest :: decode_reply (buf) , fmt) ;
                        }
                        if let (7u8, Some(5u8), true) = pat {
                            return Debug :: fmt (& netlink_bindings :: nl80211 :: RequestOpGetInterfaceDumpRequest :: decode_reply (buf) , fmt) ;
                        }
                        if let (5u8, None, false) = pat {
                            return Debug :: fmt (& netlink_bindings :: nl80211 :: RequestOpGetInterfaceDoRequest :: decode_reply (buf) , fmt) ;
                        }
                        if let (7u8, Some(5u8), false) = pat {
                            return Debug :: fmt (& netlink_bindings :: nl80211 :: RequestOpGetInterfaceDoRequest :: decode_reply (buf) , fmt) ;
                        }
                        if let (95u8, None, false) = pat {
                            return Debug :: fmt (& netlink_bindings :: nl80211 :: RequestOpGetProtocolFeaturesDoRequest :: decode_reply (buf) , fmt) ;
                        }
                        if let (95u8, Some(95u8), false) = pat {
                            return Debug :: fmt (& netlink_bindings :: nl80211 :: RequestOpGetProtocolFeaturesDoRequest :: decode_reply (buf) , fmt) ;
                        }
                        writeln!(
                            fmt,
                            "Unknown genl operation, falling back to {:?} value={value}, request_value={request_value:?}, is_dump={is_dump}",
                            "Nl80211Attrs"
                        )?;
                        Debug::fmt(
                            &netlink_bindings::nl80211::RequestOpDoRequest::decode_reply(buf),
                            fmt,
                        )?;
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
                                &netlink_bindings::nlctrl::OpGetfamilyDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(3u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nlctrl::OpGetfamilyDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (3u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nlctrl::OpGetfamilyDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(3u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nlctrl::OpGetfamilyDoReply::new(buf),
                                fmt,
                            );
                        }
                        if let (10u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nlctrl::OpGetpolicyDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (10u8, Some(10u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::nlctrl::OpGetpolicyDumpReply::new(buf),
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
                if name == b"wireguard" {
                    let pat = (value, request_value, is_dump);
                    #[cfg(feature = "wireguard")]
                    {
                        if let (0u8, None, true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::wireguard::OpGetDeviceDumpRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (0u8, Some(0u8), true) = pat {
                            return Debug::fmt(
                                &netlink_bindings::wireguard::OpGetDeviceDumpReply::new(buf),
                                fmt,
                            );
                        }
                        if let (1u8, None, false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::wireguard::OpSetDeviceDoRequest::new(buf),
                                fmt,
                            );
                        }
                        if let (1u8, Some(1u8), false) = pat {
                            return Debug::fmt(
                                &netlink_bindings::wireguard::OpSetDeviceDoReply::new(buf),
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
