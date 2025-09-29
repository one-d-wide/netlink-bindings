use netlink_bindings::Protocol;
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
