#![allow(clippy::all)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]
#![allow(unreachable_code)]
#![allow(unreachable_patterns)]
use netlink_bindings::traits::Protocol;
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
            Protocol::Raw { protonum, .. } => match protonum {
                0u16 => match (value, request_value, is_dump) {
                    #[cfg(feature = "rt-route")]
                    (26u16, None, true) => Debug::fmt(
                        &netlink_bindings::rt_route::OpGetrouteDumpRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-route"))]
                    (26u16, None, true) => consider(fmt, "rt-route"),
                    #[cfg(feature = "rt-route")]
                    (24u16, Some(26u16), true) => Debug::fmt(
                        &netlink_bindings::rt_route::OpGetrouteDumpReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-route"))]
                    (24u16, Some(26u16), true) => consider(fmt, "rt-route"),
                    #[cfg(feature = "rt-route")]
                    (26u16, None, false) => Debug::fmt(
                        &netlink_bindings::rt_route::OpGetrouteDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-route"))]
                    (26u16, None, false) => consider(fmt, "rt-route"),
                    #[cfg(feature = "rt-route")]
                    (24u16, Some(26u16), false) => Debug::fmt(
                        &netlink_bindings::rt_route::OpGetrouteDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-route"))]
                    (24u16, Some(26u16), false) => consider(fmt, "rt-route"),
                    #[cfg(feature = "rt-route")]
                    (24u16, None, false) => Debug::fmt(
                        &netlink_bindings::rt_route::OpNewrouteDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-route"))]
                    (24u16, None, false) => consider(fmt, "rt-route"),
                    #[cfg(feature = "rt-route")]
                    (24u16, Some(24u16), false) => Debug::fmt(
                        &netlink_bindings::rt_route::OpNewrouteDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-route"))]
                    (24u16, Some(24u16), false) => consider(fmt, "rt-route"),
                    #[cfg(feature = "rt-route")]
                    (25u16, None, false) => Debug::fmt(
                        &netlink_bindings::rt_route::OpDelrouteDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-route"))]
                    (25u16, None, false) => consider(fmt, "rt-route"),
                    #[cfg(feature = "rt-route")]
                    (25u16, Some(25u16), false) => Debug::fmt(
                        &netlink_bindings::rt_route::OpDelrouteDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-route"))]
                    (25u16, Some(25u16), false) => consider(fmt, "rt-route"),
                    #[cfg(feature = "rt-link")]
                    (16u16, None, false) => Debug::fmt(
                        &netlink_bindings::rt_link::OpNewlinkDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-link"))]
                    (16u16, None, false) => consider(fmt, "rt-link"),
                    #[cfg(feature = "rt-link")]
                    (16u16, Some(16u16), false) => {
                        Debug::fmt(&netlink_bindings::rt_link::OpNewlinkDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "rt-link"))]
                    (16u16, Some(16u16), false) => consider(fmt, "rt-link"),
                    #[cfg(feature = "rt-link")]
                    (17u16, None, false) => Debug::fmt(
                        &netlink_bindings::rt_link::OpDellinkDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-link"))]
                    (17u16, None, false) => consider(fmt, "rt-link"),
                    #[cfg(feature = "rt-link")]
                    (17u16, Some(17u16), false) => {
                        Debug::fmt(&netlink_bindings::rt_link::OpDellinkDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "rt-link"))]
                    (17u16, Some(17u16), false) => consider(fmt, "rt-link"),
                    #[cfg(feature = "rt-link")]
                    (18u16, None, true) => Debug::fmt(
                        &netlink_bindings::rt_link::OpGetlinkDumpRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-link"))]
                    (18u16, None, true) => consider(fmt, "rt-link"),
                    #[cfg(feature = "rt-link")]
                    (16u16, Some(18u16), true) => Debug::fmt(
                        &netlink_bindings::rt_link::OpGetlinkDumpReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-link"))]
                    (16u16, Some(18u16), true) => consider(fmt, "rt-link"),
                    #[cfg(feature = "rt-link")]
                    (18u16, None, false) => Debug::fmt(
                        &netlink_bindings::rt_link::OpGetlinkDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-link"))]
                    (18u16, None, false) => consider(fmt, "rt-link"),
                    #[cfg(feature = "rt-link")]
                    (16u16, Some(18u16), false) => {
                        Debug::fmt(&netlink_bindings::rt_link::OpGetlinkDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "rt-link"))]
                    (16u16, Some(18u16), false) => consider(fmt, "rt-link"),
                    #[cfg(feature = "rt-link")]
                    (19u16, None, false) => Debug::fmt(
                        &netlink_bindings::rt_link::OpSetlinkDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-link"))]
                    (19u16, None, false) => consider(fmt, "rt-link"),
                    #[cfg(feature = "rt-link")]
                    (19u16, Some(19u16), false) => {
                        Debug::fmt(&netlink_bindings::rt_link::OpSetlinkDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "rt-link"))]
                    (19u16, Some(19u16), false) => consider(fmt, "rt-link"),
                    #[cfg(feature = "rt-link")]
                    (94u16, None, true) => Debug::fmt(
                        &netlink_bindings::rt_link::OpGetstatsDumpRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-link"))]
                    (94u16, None, true) => consider(fmt, "rt-link"),
                    #[cfg(feature = "rt-link")]
                    (92u16, Some(94u16), true) => Debug::fmt(
                        &netlink_bindings::rt_link::OpGetstatsDumpReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-link"))]
                    (92u16, Some(94u16), true) => consider(fmt, "rt-link"),
                    #[cfg(feature = "rt-link")]
                    (94u16, None, false) => Debug::fmt(
                        &netlink_bindings::rt_link::OpGetstatsDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-link"))]
                    (94u16, None, false) => consider(fmt, "rt-link"),
                    #[cfg(feature = "rt-link")]
                    (92u16, Some(94u16), false) => {
                        Debug::fmt(&netlink_bindings::rt_link::OpGetstatsDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "rt-link"))]
                    (92u16, Some(94u16), false) => consider(fmt, "rt-link"),
                    #[cfg(feature = "rt-neigh")]
                    (28u16, None, false) => Debug::fmt(
                        &netlink_bindings::rt_neigh::OpNewneighDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-neigh"))]
                    (28u16, None, false) => consider(fmt, "rt-neigh"),
                    #[cfg(feature = "rt-neigh")]
                    (28u16, Some(28u16), false) => Debug::fmt(
                        &netlink_bindings::rt_neigh::OpNewneighDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-neigh"))]
                    (28u16, Some(28u16), false) => consider(fmt, "rt-neigh"),
                    #[cfg(feature = "rt-neigh")]
                    (29u16, None, false) => Debug::fmt(
                        &netlink_bindings::rt_neigh::OpDelneighDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-neigh"))]
                    (29u16, None, false) => consider(fmt, "rt-neigh"),
                    #[cfg(feature = "rt-neigh")]
                    (29u16, Some(29u16), false) => Debug::fmt(
                        &netlink_bindings::rt_neigh::OpDelneighDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-neigh"))]
                    (29u16, Some(29u16), false) => consider(fmt, "rt-neigh"),
                    #[cfg(feature = "rt-neigh")]
                    (30u16, None, true) => Debug::fmt(
                        &netlink_bindings::rt_neigh::OpGetneighDumpRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-neigh"))]
                    (30u16, None, true) => consider(fmt, "rt-neigh"),
                    #[cfg(feature = "rt-neigh")]
                    (28u16, Some(30u16), true) => Debug::fmt(
                        &netlink_bindings::rt_neigh::OpGetneighDumpReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-neigh"))]
                    (28u16, Some(30u16), true) => consider(fmt, "rt-neigh"),
                    #[cfg(feature = "rt-neigh")]
                    (30u16, None, false) => Debug::fmt(
                        &netlink_bindings::rt_neigh::OpGetneighDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-neigh"))]
                    (30u16, None, false) => consider(fmt, "rt-neigh"),
                    #[cfg(feature = "rt-neigh")]
                    (28u16, Some(30u16), false) => Debug::fmt(
                        &netlink_bindings::rt_neigh::OpGetneighDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-neigh"))]
                    (28u16, Some(30u16), false) => consider(fmt, "rt-neigh"),
                    #[cfg(feature = "rt-neigh")]
                    (66u16, None, true) => Debug::fmt(
                        &netlink_bindings::rt_neigh::OpGetneightblDumpRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-neigh"))]
                    (66u16, None, true) => consider(fmt, "rt-neigh"),
                    #[cfg(feature = "rt-neigh")]
                    (64u16, Some(66u16), true) => Debug::fmt(
                        &netlink_bindings::rt_neigh::OpGetneightblDumpReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-neigh"))]
                    (64u16, Some(66u16), true) => consider(fmt, "rt-neigh"),
                    #[cfg(feature = "rt-neigh")]
                    (67u16, None, false) => Debug::fmt(
                        &netlink_bindings::rt_neigh::OpSetneightblDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-neigh"))]
                    (67u16, None, false) => consider(fmt, "rt-neigh"),
                    #[cfg(feature = "rt-neigh")]
                    (67u16, Some(67u16), false) => Debug::fmt(
                        &netlink_bindings::rt_neigh::OpSetneightblDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-neigh"))]
                    (67u16, Some(67u16), false) => consider(fmt, "rt-neigh"),
                    #[cfg(feature = "rt-rule")]
                    (32u16, None, false) => Debug::fmt(
                        &netlink_bindings::rt_rule::OpNewruleDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-rule"))]
                    (32u16, None, false) => consider(fmt, "rt-rule"),
                    #[cfg(feature = "rt-rule")]
                    (32u16, Some(32u16), false) => {
                        Debug::fmt(&netlink_bindings::rt_rule::OpNewruleDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "rt-rule"))]
                    (32u16, Some(32u16), false) => consider(fmt, "rt-rule"),
                    #[cfg(feature = "rt-rule")]
                    (33u16, None, false) => Debug::fmt(
                        &netlink_bindings::rt_rule::OpDelruleDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-rule"))]
                    (33u16, None, false) => consider(fmt, "rt-rule"),
                    #[cfg(feature = "rt-rule")]
                    (33u16, Some(33u16), false) => {
                        Debug::fmt(&netlink_bindings::rt_rule::OpDelruleDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "rt-rule"))]
                    (33u16, Some(33u16), false) => consider(fmt, "rt-rule"),
                    #[cfg(feature = "rt-rule")]
                    (34u16, None, true) => Debug::fmt(
                        &netlink_bindings::rt_rule::OpGetruleDumpRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-rule"))]
                    (34u16, None, true) => consider(fmt, "rt-rule"),
                    #[cfg(feature = "rt-rule")]
                    (32u16, Some(34u16), true) => Debug::fmt(
                        &netlink_bindings::rt_rule::OpGetruleDumpReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-rule"))]
                    (32u16, Some(34u16), true) => consider(fmt, "rt-rule"),
                    #[cfg(feature = "tc")]
                    (36u16, None, false) => {
                        Debug::fmt(&netlink_bindings::tc::OpNewqdiscDoRequest::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (36u16, None, false) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (36u16, Some(36u16), false) => {
                        Debug::fmt(&netlink_bindings::tc::OpNewqdiscDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (36u16, Some(36u16), false) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (37u16, None, false) => {
                        Debug::fmt(&netlink_bindings::tc::OpDelqdiscDoRequest::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (37u16, None, false) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (37u16, Some(37u16), false) => {
                        Debug::fmt(&netlink_bindings::tc::OpDelqdiscDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (37u16, Some(37u16), false) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (38u16, None, true) => {
                        Debug::fmt(&netlink_bindings::tc::OpGetqdiscDumpRequest::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (38u16, None, true) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (36u16, Some(38u16), true) => {
                        Debug::fmt(&netlink_bindings::tc::OpGetqdiscDumpReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (36u16, Some(38u16), true) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (38u16, None, false) => {
                        Debug::fmt(&netlink_bindings::tc::OpGetqdiscDoRequest::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (38u16, None, false) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (36u16, Some(38u16), false) => {
                        Debug::fmt(&netlink_bindings::tc::OpGetqdiscDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (36u16, Some(38u16), false) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (40u16, None, false) => {
                        Debug::fmt(&netlink_bindings::tc::OpNewtclassDoRequest::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (40u16, None, false) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (40u16, Some(40u16), false) => {
                        Debug::fmt(&netlink_bindings::tc::OpNewtclassDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (40u16, Some(40u16), false) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (41u16, None, false) => {
                        Debug::fmt(&netlink_bindings::tc::OpDeltclassDoRequest::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (41u16, None, false) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (41u16, Some(41u16), false) => {
                        Debug::fmt(&netlink_bindings::tc::OpDeltclassDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (41u16, Some(41u16), false) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (42u16, None, false) => {
                        Debug::fmt(&netlink_bindings::tc::OpGettclassDoRequest::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (42u16, None, false) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (40u16, Some(42u16), false) => {
                        Debug::fmt(&netlink_bindings::tc::OpGettclassDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (40u16, Some(42u16), false) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (44u16, None, false) => {
                        Debug::fmt(&netlink_bindings::tc::OpNewtfilterDoRequest::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (44u16, None, false) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (44u16, Some(44u16), false) => {
                        Debug::fmt(&netlink_bindings::tc::OpNewtfilterDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (44u16, Some(44u16), false) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (45u16, None, false) => {
                        Debug::fmt(&netlink_bindings::tc::OpDeltfilterDoRequest::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (45u16, None, false) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (45u16, Some(45u16), false) => {
                        Debug::fmt(&netlink_bindings::tc::OpDeltfilterDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (45u16, Some(45u16), false) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (46u16, None, true) => Debug::fmt(
                        &netlink_bindings::tc::OpGettfilterDumpRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "tc"))]
                    (46u16, None, true) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (44u16, Some(46u16), true) => {
                        Debug::fmt(&netlink_bindings::tc::OpGettfilterDumpReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (44u16, Some(46u16), true) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (46u16, None, false) => {
                        Debug::fmt(&netlink_bindings::tc::OpGettfilterDoRequest::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (46u16, None, false) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (44u16, Some(46u16), false) => {
                        Debug::fmt(&netlink_bindings::tc::OpGettfilterDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (44u16, Some(46u16), false) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (100u16, None, false) => {
                        Debug::fmt(&netlink_bindings::tc::OpNewchainDoRequest::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (100u16, None, false) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (100u16, Some(100u16), false) => {
                        Debug::fmt(&netlink_bindings::tc::OpNewchainDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (100u16, Some(100u16), false) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (101u16, None, false) => {
                        Debug::fmt(&netlink_bindings::tc::OpDelchainDoRequest::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (101u16, None, false) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (101u16, Some(101u16), false) => {
                        Debug::fmt(&netlink_bindings::tc::OpDelchainDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (101u16, Some(101u16), false) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (102u16, None, false) => {
                        Debug::fmt(&netlink_bindings::tc::OpGetchainDoRequest::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (102u16, None, false) => consider(fmt, "tc"),
                    #[cfg(feature = "tc")]
                    (100u16, Some(102u16), false) => {
                        Debug::fmt(&netlink_bindings::tc::OpGetchainDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "tc"))]
                    (100u16, Some(102u16), false) => consider(fmt, "tc"),
                    #[cfg(feature = "rt-addr")]
                    (20u16, None, false) => Debug::fmt(
                        &netlink_bindings::rt_addr::OpNewaddrDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-addr"))]
                    (20u16, None, false) => consider(fmt, "rt-addr"),
                    #[cfg(feature = "rt-addr")]
                    (20u16, Some(20u16), false) => {
                        Debug::fmt(&netlink_bindings::rt_addr::OpNewaddrDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "rt-addr"))]
                    (20u16, Some(20u16), false) => consider(fmt, "rt-addr"),
                    #[cfg(feature = "rt-addr")]
                    (21u16, None, false) => Debug::fmt(
                        &netlink_bindings::rt_addr::OpDeladdrDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-addr"))]
                    (21u16, None, false) => consider(fmt, "rt-addr"),
                    #[cfg(feature = "rt-addr")]
                    (21u16, Some(21u16), false) => {
                        Debug::fmt(&netlink_bindings::rt_addr::OpDeladdrDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "rt-addr"))]
                    (21u16, Some(21u16), false) => consider(fmt, "rt-addr"),
                    #[cfg(feature = "rt-addr")]
                    (22u16, None, true) => Debug::fmt(
                        &netlink_bindings::rt_addr::OpGetaddrDumpRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-addr"))]
                    (22u16, None, true) => consider(fmt, "rt-addr"),
                    #[cfg(feature = "rt-addr")]
                    (20u16, Some(22u16), true) => Debug::fmt(
                        &netlink_bindings::rt_addr::OpGetaddrDumpReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-addr"))]
                    (20u16, Some(22u16), true) => consider(fmt, "rt-addr"),
                    #[cfg(feature = "rt-addr")]
                    (58u16, None, true) => Debug::fmt(
                        &netlink_bindings::rt_addr::OpGetmulticastDumpRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-addr"))]
                    (58u16, None, true) => consider(fmt, "rt-addr"),
                    #[cfg(feature = "rt-addr")]
                    (58u16, Some(58u16), true) => Debug::fmt(
                        &netlink_bindings::rt_addr::OpGetmulticastDumpReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-addr"))]
                    (58u16, Some(58u16), true) => consider(fmt, "rt-addr"),
                    #[cfg(feature = "rt-addr")]
                    (58u16, None, false) => Debug::fmt(
                        &netlink_bindings::rt_addr::OpGetmulticastDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-addr"))]
                    (58u16, None, false) => consider(fmt, "rt-addr"),
                    #[cfg(feature = "rt-addr")]
                    (58u16, Some(58u16), false) => Debug::fmt(
                        &netlink_bindings::rt_addr::OpGetmulticastDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "rt-addr"))]
                    (58u16, Some(58u16), false) => consider(fmt, "rt-addr"),
                    _ => write!(
                        fmt,
                        "(Unknown operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                    ),
                },
                12u16 => match (value, request_value, is_dump) {
                    #[cfg(feature = "conntrack")]
                    (257u16, None, true) => Debug::fmt(
                        &netlink_bindings::conntrack::OpGetDumpRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "conntrack"))]
                    (257u16, None, true) => consider(fmt, "conntrack"),
                    #[cfg(feature = "conntrack")]
                    (256u16, Some(257u16), true) => {
                        Debug::fmt(&netlink_bindings::conntrack::OpGetDumpReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "conntrack"))]
                    (256u16, Some(257u16), true) => consider(fmt, "conntrack"),
                    #[cfg(feature = "conntrack")]
                    (257u16, None, false) => {
                        Debug::fmt(&netlink_bindings::conntrack::OpGetDoRequest::new(buf), fmt)
                    }
                    #[cfg(not(feature = "conntrack"))]
                    (257u16, None, false) => consider(fmt, "conntrack"),
                    #[cfg(feature = "conntrack")]
                    (256u16, Some(257u16), false) => {
                        Debug::fmt(&netlink_bindings::conntrack::OpGetDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "conntrack"))]
                    (256u16, Some(257u16), false) => consider(fmt, "conntrack"),
                    #[cfg(feature = "conntrack")]
                    (260u16, None, true) => Debug::fmt(
                        &netlink_bindings::conntrack::OpGetStatsDumpRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "conntrack"))]
                    (260u16, None, true) => consider(fmt, "conntrack"),
                    #[cfg(feature = "conntrack")]
                    (260u16, Some(260u16), true) => Debug::fmt(
                        &netlink_bindings::conntrack::OpGetStatsDumpReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "conntrack"))]
                    (260u16, Some(260u16), true) => consider(fmt, "conntrack"),
                    #[cfg(feature = "nftables")]
                    (2816u16, None, true) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetcompatDumpRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2816u16, None, true) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2816u16, Some(2816u16), true) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetcompatDumpReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2816u16, Some(2816u16), true) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2816u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetcompatDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2816u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2816u16, Some(2816u16), false) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetcompatDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2816u16, Some(2816u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (16u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpBatchBeginDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (16u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (16u16, Some(16u16), false) => Debug::fmt(
                        &netlink_bindings::nftables::OpBatchBeginDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (16u16, Some(16u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (17u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpBatchEndDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (17u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (17u16, Some(17u16), false) => Debug::fmt(
                        &netlink_bindings::nftables::OpBatchEndDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (17u16, Some(17u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2560u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpNewtableDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2560u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2560u16, Some(2560u16), false) => Debug::fmt(
                        &netlink_bindings::nftables::OpNewtableDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2560u16, Some(2560u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2561u16, None, true) => Debug::fmt(
                        &netlink_bindings::nftables::OpGettableDumpRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2561u16, None, true) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2560u16, Some(2561u16), true) => Debug::fmt(
                        &netlink_bindings::nftables::OpGettableDumpReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2560u16, Some(2561u16), true) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2561u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpGettableDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2561u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2560u16, Some(2561u16), false) => Debug::fmt(
                        &netlink_bindings::nftables::OpGettableDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2560u16, Some(2561u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2562u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpDeltableDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2562u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2562u16, Some(2562u16), false) => Debug::fmt(
                        &netlink_bindings::nftables::OpDeltableDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2562u16, Some(2562u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2586u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpDestroytableDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2586u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2586u16, Some(2586u16), false) => Debug::fmt(
                        &netlink_bindings::nftables::OpDestroytableDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2586u16, Some(2586u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2563u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpNewchainDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2563u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2563u16, Some(2563u16), false) => Debug::fmt(
                        &netlink_bindings::nftables::OpNewchainDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2563u16, Some(2563u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2564u16, None, true) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetchainDumpRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2564u16, None, true) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2563u16, Some(2564u16), true) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetchainDumpReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2563u16, Some(2564u16), true) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2564u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetchainDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2564u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2563u16, Some(2564u16), false) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetchainDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2563u16, Some(2564u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2565u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpDelchainDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2565u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2565u16, Some(2565u16), false) => Debug::fmt(
                        &netlink_bindings::nftables::OpDelchainDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2565u16, Some(2565u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2587u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpDestroychainDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2587u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2587u16, Some(2587u16), false) => Debug::fmt(
                        &netlink_bindings::nftables::OpDestroychainDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2587u16, Some(2587u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2566u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpNewruleDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2566u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2566u16, Some(2566u16), false) => {
                        Debug::fmt(&netlink_bindings::nftables::OpNewruleDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "nftables"))]
                    (2566u16, Some(2566u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2567u16, None, true) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetruleDumpRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2567u16, None, true) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2566u16, Some(2567u16), true) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetruleDumpReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2566u16, Some(2567u16), true) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2567u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetruleDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2567u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2566u16, Some(2567u16), false) => {
                        Debug::fmt(&netlink_bindings::nftables::OpGetruleDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "nftables"))]
                    (2566u16, Some(2567u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2585u16, None, true) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetruleResetDumpRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2585u16, None, true) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2566u16, Some(2585u16), true) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetruleResetDumpReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2566u16, Some(2585u16), true) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2585u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetruleResetDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2585u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2566u16, Some(2585u16), false) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetruleResetDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2566u16, Some(2585u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2568u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpDelruleDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2568u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2568u16, Some(2568u16), false) => {
                        Debug::fmt(&netlink_bindings::nftables::OpDelruleDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "nftables"))]
                    (2568u16, Some(2568u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2588u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpDestroyruleDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2588u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2588u16, Some(2588u16), false) => Debug::fmt(
                        &netlink_bindings::nftables::OpDestroyruleDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2588u16, Some(2588u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2569u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpNewsetDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2569u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2569u16, Some(2569u16), false) => {
                        Debug::fmt(&netlink_bindings::nftables::OpNewsetDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "nftables"))]
                    (2569u16, Some(2569u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2570u16, None, true) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetsetDumpRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2570u16, None, true) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2569u16, Some(2570u16), true) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetsetDumpReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2569u16, Some(2570u16), true) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2570u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetsetDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2570u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2569u16, Some(2570u16), false) => {
                        Debug::fmt(&netlink_bindings::nftables::OpGetsetDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "nftables"))]
                    (2569u16, Some(2570u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2571u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpDelsetDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2571u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2571u16, Some(2571u16), false) => {
                        Debug::fmt(&netlink_bindings::nftables::OpDelsetDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "nftables"))]
                    (2571u16, Some(2571u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2589u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpDestroysetDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2589u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2589u16, Some(2589u16), false) => Debug::fmt(
                        &netlink_bindings::nftables::OpDestroysetDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2589u16, Some(2589u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2572u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpNewsetelemDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2572u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2572u16, Some(2572u16), false) => Debug::fmt(
                        &netlink_bindings::nftables::OpNewsetelemDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2572u16, Some(2572u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2573u16, None, true) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetsetelemDumpRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2573u16, None, true) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2572u16, Some(2573u16), true) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetsetelemDumpReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2572u16, Some(2573u16), true) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2573u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetsetelemDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2573u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2572u16, Some(2573u16), false) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetsetelemDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2572u16, Some(2573u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2593u16, None, true) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetsetelemResetDumpRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2593u16, None, true) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2572u16, Some(2593u16), true) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetsetelemResetDumpReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2572u16, Some(2593u16), true) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2593u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetsetelemResetDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2593u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2572u16, Some(2593u16), false) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetsetelemResetDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2572u16, Some(2593u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2574u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpDelsetelemDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2574u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2574u16, Some(2574u16), false) => Debug::fmt(
                        &netlink_bindings::nftables::OpDelsetelemDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2574u16, Some(2574u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2590u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpDestroysetelemDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2590u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2590u16, Some(2590u16), false) => Debug::fmt(
                        &netlink_bindings::nftables::OpDestroysetelemDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2590u16, Some(2590u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2576u16, None, true) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetgenDumpRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2576u16, None, true) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2575u16, Some(2576u16), true) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetgenDumpReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2575u16, Some(2576u16), true) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2576u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetgenDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2576u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2575u16, Some(2576u16), false) => {
                        Debug::fmt(&netlink_bindings::nftables::OpGetgenDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "nftables"))]
                    (2575u16, Some(2576u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2578u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpNewobjDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2578u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2578u16, Some(2578u16), false) => {
                        Debug::fmt(&netlink_bindings::nftables::OpNewobjDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "nftables"))]
                    (2578u16, Some(2578u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2579u16, None, true) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetobjDumpRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2579u16, None, true) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2578u16, Some(2579u16), true) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetobjDumpReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2578u16, Some(2579u16), true) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2579u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetobjDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2579u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2578u16, Some(2579u16), false) => {
                        Debug::fmt(&netlink_bindings::nftables::OpGetobjDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "nftables"))]
                    (2578u16, Some(2579u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2580u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpDelobjDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2580u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2580u16, Some(2580u16), false) => {
                        Debug::fmt(&netlink_bindings::nftables::OpDelobjDoReply::new(buf), fmt)
                    }
                    #[cfg(not(feature = "nftables"))]
                    (2580u16, Some(2580u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2591u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpDestroyobjDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2591u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2591u16, Some(2591u16), false) => Debug::fmt(
                        &netlink_bindings::nftables::OpDestroyobjDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2591u16, Some(2591u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2582u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpNewflowtableDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2582u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2582u16, Some(2582u16), false) => Debug::fmt(
                        &netlink_bindings::nftables::OpNewflowtableDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2582u16, Some(2582u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2583u16, None, true) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetflowtableDumpRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2583u16, None, true) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2582u16, Some(2583u16), true) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetflowtableDumpReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2582u16, Some(2583u16), true) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2583u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetflowtableDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2583u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2582u16, Some(2583u16), false) => Debug::fmt(
                        &netlink_bindings::nftables::OpGetflowtableDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2582u16, Some(2583u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2584u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpDelflowtableDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2584u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2584u16, Some(2584u16), false) => Debug::fmt(
                        &netlink_bindings::nftables::OpDelflowtableDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2584u16, Some(2584u16), false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2592u16, None, false) => Debug::fmt(
                        &netlink_bindings::nftables::OpDestroyflowtableDoRequest::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2592u16, None, false) => consider(fmt, "nftables"),
                    #[cfg(feature = "nftables")]
                    (2592u16, Some(2592u16), false) => Debug::fmt(
                        &netlink_bindings::nftables::OpDestroyflowtableDoReply::new(buf),
                        fmt,
                    ),
                    #[cfg(not(feature = "nftables"))]
                    (2592u16, Some(2592u16), false) => consider(fmt, "nftables"),
                    _ => write!(
                        fmt,
                        "(Unknown operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                    ),
                },
                _ => write!(fmt, "(Protocol {protonum:?} not recognized)"),
            },
            Protocol::Generic(name) => {
                let value = value as u8;
                let request_value = request_value.map(|val| val as u8);
                match name {
                    #[cfg(feature = "nl80211")]
                    b"nl80211" => match (value, request_value, is_dump) {
                        (1u8, None, true) => Debug::fmt(
                            &netlink_bindings::nl80211::OpGetWiphyDumpRequest::new(buf),
                            fmt,
                        ),
                        (3u8, Some(1u8), true) => Debug::fmt(
                            &netlink_bindings::nl80211::OpGetWiphyDumpReply::new(buf),
                            fmt,
                        ),
                        (1u8, None, false) => Debug::fmt(
                            &netlink_bindings::nl80211::OpGetWiphyDoRequest::new(buf),
                            fmt,
                        ),
                        (3u8, Some(1u8), false) => {
                            Debug::fmt(&netlink_bindings::nl80211::OpGetWiphyDoReply::new(buf), fmt)
                        }
                        (5u8, None, true) => Debug::fmt(
                            &netlink_bindings::nl80211::OpGetInterfaceDumpRequest::new(buf),
                            fmt,
                        ),
                        (7u8, Some(5u8), true) => Debug::fmt(
                            &netlink_bindings::nl80211::OpGetInterfaceDumpReply::new(buf),
                            fmt,
                        ),
                        (5u8, None, false) => Debug::fmt(
                            &netlink_bindings::nl80211::OpGetInterfaceDoRequest::new(buf),
                            fmt,
                        ),
                        (7u8, Some(5u8), false) => Debug::fmt(
                            &netlink_bindings::nl80211::OpGetInterfaceDoReply::new(buf),
                            fmt,
                        ),
                        (95u8, None, false) => Debug::fmt(
                            &netlink_bindings::nl80211::OpGetProtocolFeaturesDoRequest::new(buf),
                            fmt,
                        ),
                        (95u8, Some(95u8), false) => Debug::fmt(
                            &netlink_bindings::nl80211::OpGetProtocolFeaturesDoReply::new(buf),
                            fmt,
                        ),
                        _ => write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        ),
                    },
                    #[cfg(not(feature = "nl80211"))]
                    b"nl80211" => consider(fmt, "nl80211"),
                    #[cfg(feature = "nlctrl")]
                    b"nlctrl" => match (value, request_value, is_dump) {
                        (3u8, None, true) => Debug::fmt(
                            &netlink_bindings::nlctrl::OpGetfamilyDumpRequest::new(buf),
                            fmt,
                        ),
                        (1u8, Some(3u8), true) => Debug::fmt(
                            &netlink_bindings::nlctrl::OpGetfamilyDumpReply::new(buf),
                            fmt,
                        ),
                        (3u8, None, false) => Debug::fmt(
                            &netlink_bindings::nlctrl::OpGetfamilyDoRequest::new(buf),
                            fmt,
                        ),
                        (1u8, Some(3u8), false) => {
                            Debug::fmt(&netlink_bindings::nlctrl::OpGetfamilyDoReply::new(buf), fmt)
                        }
                        (10u8, None, true) => Debug::fmt(
                            &netlink_bindings::nlctrl::OpGetpolicyDumpRequest::new(buf),
                            fmt,
                        ),
                        (10u8, Some(10u8), true) => Debug::fmt(
                            &netlink_bindings::nlctrl::OpGetpolicyDumpReply::new(buf),
                            fmt,
                        ),
                        _ => write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        ),
                    },
                    #[cfg(not(feature = "nlctrl"))]
                    b"nlctrl" => consider(fmt, "nlctrl"),
                    #[cfg(feature = "wireguard")]
                    b"wireguard" => match (value, request_value, is_dump) {
                        (0u8, None, true) => Debug::fmt(
                            &netlink_bindings::wireguard::OpGetDeviceDumpRequest::new(buf),
                            fmt,
                        ),
                        (0u8, Some(0u8), true) => Debug::fmt(
                            &netlink_bindings::wireguard::OpGetDeviceDumpReply::new(buf),
                            fmt,
                        ),
                        (1u8, None, false) => Debug::fmt(
                            &netlink_bindings::wireguard::OpSetDeviceDoRequest::new(buf),
                            fmt,
                        ),
                        (1u8, Some(1u8), false) => Debug::fmt(
                            &netlink_bindings::wireguard::OpSetDeviceDoReply::new(buf),
                            fmt,
                        ),
                        _ => write!(
                            fmt,
                            "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"
                        ),
                    },
                    #[cfg(not(feature = "wireguard"))]
                    b"wireguard" => consider(fmt, "wireguard"),
                    _ => write!(fmt, "(Protocol {name:?} not recognized)"),
                }
            }
        }
    }
}
