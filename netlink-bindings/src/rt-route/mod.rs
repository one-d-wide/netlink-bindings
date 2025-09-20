#![doc = "Route configuration over rtnetlink."]
#![allow(clippy::all)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]
#![allow(unreachable_code)]
#![allow(unreachable_patterns)]
use crate::utils::*;
pub const PROTONUM: u8 = 0u8;
#[doc = "Original name: \"rtm-type\" (enum) - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Clone)]
pub enum RtmType {
    Unspec = 0,
    Unicast = 1,
    Local = 2,
    Broadcast = 3,
    Anycast = 4,
    Multicast = 5,
    Blackhole = 6,
    Unreachable = 7,
    Prohibit = 8,
    Throw = 9,
    Nat = 10,
    Xresolve = 11,
}
#[doc = "Original name: \"route-attrs\""]
#[derive(Clone)]
pub enum RouteAttrs<'a> {
    Dst(&'a [u8]),
    Src(&'a [u8]),
    Iif(u32),
    Oif(u32),
    Gateway(&'a [u8]),
    Priority(u32),
    Prefsrc(&'a [u8]),
    Metrics(Iterable<'a, Metrics<'a>>),
    Multipath(&'a [u8]),
    Protoinfo(&'a [u8]),
    Flow(u32),
    Cacheinfo(PushRtaCacheinfo),
    Session(&'a [u8]),
    MpAlgo(&'a [u8]),
    Table(u32),
    Mark(u32),
    MfcStats(&'a [u8]),
    Via(&'a [u8]),
    Newdst(&'a [u8]),
    Pref(u8),
    EncapType(u16),
    Encap(&'a [u8]),
    Expires(u32),
    Pad(&'a [u8]),
    Uid(u32),
    TtlPropagate(u8),
    IpProto(u8),
    Sport(u16),
    Dport(u16),
    NhId(u32),
    Flowlabel(u32),
}
impl<'a> Iterable<'a, RouteAttrs<'a>> {
    pub fn get_dst(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::Dst(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_src(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::Src(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_iif(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::Iif(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_oif(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::Oif(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_gateway(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::Gateway(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_priority(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::Priority(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_prefsrc(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::Prefsrc(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_metrics(&self) -> Iterable<'a, Metrics<'a>> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::Metrics(val) = attr {
                return val;
            }
        }
        Iterable::new(&[])
    }
    pub fn get_multipath(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::Multipath(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_protoinfo(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::Protoinfo(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_flow(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::Flow(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_cacheinfo(&self) -> Option<PushRtaCacheinfo> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::Cacheinfo(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_session(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::Session(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_mp_algo(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::MpAlgo(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_table(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::Table(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_mark(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::Mark(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_mfc_stats(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::MfcStats(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_via(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::Via(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_newdst(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::Newdst(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_pref(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::Pref(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_encap_type(&self) -> Option<u16> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::EncapType(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_encap(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::Encap(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_expires(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::Expires(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_pad(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::Pad(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_uid(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::Uid(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_ttl_propagate(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::TtlPropagate(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_ip_proto(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::IpProto(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_sport(&self) -> Option<u16> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::Sport(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_dport(&self) -> Option<u16> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::Dport(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_nh_id(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::NhId(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_flowlabel(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let RouteAttrs::Flowlabel(val) = attr {
                return Some(val);
            }
        }
        None
    }
}
impl<'a> RouteAttrs<'a> {
    pub fn new(buf: &'a [u8]) -> Iterable<'a, RouteAttrs<'a>> {
        Iterable::new(buf)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Dst",
            2u16 => "Src",
            3u16 => "Iif",
            4u16 => "Oif",
            5u16 => "Gateway",
            6u16 => "Priority",
            7u16 => "Prefsrc",
            8u16 => "Metrics",
            9u16 => "Multipath",
            10u16 => "Protoinfo",
            11u16 => "Flow",
            12u16 => "Cacheinfo",
            13u16 => "Session",
            14u16 => "MpAlgo",
            15u16 => "Table",
            16u16 => "Mark",
            17u16 => "MfcStats",
            18u16 => "Via",
            19u16 => "Newdst",
            20u16 => "Pref",
            21u16 => "EncapType",
            22u16 => "Encap",
            23u16 => "Expires",
            24u16 => "Pad",
            25u16 => "Uid",
            26u16 => "TtlPropagate",
            27u16 => "IpProto",
            28u16 => "Sport",
            29u16 => "Dport",
            30u16 => "NhId",
            31u16 => "Flowlabel",
            _ => return None,
        };
        Some(res)
    }
}
impl<'a> Iterator for Iterable<'a, RouteAttrs<'a>> {
    type Item = Result<RouteAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => RouteAttrs::Dst({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => RouteAttrs::Src({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => RouteAttrs::Iif({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => RouteAttrs::Oif({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => RouteAttrs::Gateway({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => RouteAttrs::Priority({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => RouteAttrs::Prefsrc({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => RouteAttrs::Metrics({
                    let res = Some(Iterable::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => RouteAttrs::Multipath({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => RouteAttrs::Protoinfo({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => RouteAttrs::Flow({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => RouteAttrs::Cacheinfo({
                    let res = PushRtaCacheinfo::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => RouteAttrs::Session({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => RouteAttrs::MpAlgo({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => RouteAttrs::Table({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => RouteAttrs::Mark({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => RouteAttrs::MfcStats({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => RouteAttrs::Via({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => RouteAttrs::Newdst({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => RouteAttrs::Pref({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => RouteAttrs::EncapType({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => RouteAttrs::Encap({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => RouteAttrs::Expires({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => RouteAttrs::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                25u16 => RouteAttrs::Uid({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                26u16 => RouteAttrs::TtlPropagate({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                27u16 => RouteAttrs::IpProto({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                28u16 => RouteAttrs::Sport({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                29u16 => RouteAttrs::Dport({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                30u16 => RouteAttrs::NhId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                31u16 => RouteAttrs::Flowlabel({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "RouteAttrs",
            r#type.and_then(|t| RouteAttrs::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, RouteAttrs<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("RouteAttrs");
        for attr in self.clone() {
            let attr = match attr {
                Ok(a) => a,
                Err(err) => {
                    fmt.finish()?;
                    f.write_str("Err(")?;
                    err.fmt(f)?;
                    return f.write_str(")");
                }
            };
            match attr {
                RouteAttrs::Dst(val) => fmt.field("Dst", &val),
                RouteAttrs::Src(val) => fmt.field("Src", &val),
                RouteAttrs::Iif(val) => fmt.field("Iif", &val),
                RouteAttrs::Oif(val) => fmt.field("Oif", &val),
                RouteAttrs::Gateway(val) => fmt.field("Gateway", &val),
                RouteAttrs::Priority(val) => fmt.field("Priority", &val),
                RouteAttrs::Prefsrc(val) => fmt.field("Prefsrc", &val),
                RouteAttrs::Metrics(val) => fmt.field("Metrics", &val),
                RouteAttrs::Multipath(val) => fmt.field("Multipath", &val),
                RouteAttrs::Protoinfo(val) => fmt.field("Protoinfo", &val),
                RouteAttrs::Flow(val) => fmt.field("Flow", &val),
                RouteAttrs::Cacheinfo(val) => fmt.field("Cacheinfo", &val),
                RouteAttrs::Session(val) => fmt.field("Session", &val),
                RouteAttrs::MpAlgo(val) => fmt.field("MpAlgo", &val),
                RouteAttrs::Table(val) => fmt.field("Table", &val),
                RouteAttrs::Mark(val) => fmt.field("Mark", &val),
                RouteAttrs::MfcStats(val) => fmt.field("MfcStats", &val),
                RouteAttrs::Via(val) => fmt.field("Via", &val),
                RouteAttrs::Newdst(val) => fmt.field("Newdst", &val),
                RouteAttrs::Pref(val) => fmt.field("Pref", &val),
                RouteAttrs::EncapType(val) => fmt.field("EncapType", &val),
                RouteAttrs::Encap(val) => fmt.field("Encap", &val),
                RouteAttrs::Expires(val) => fmt.field("Expires", &val),
                RouteAttrs::Pad(val) => fmt.field("Pad", &val),
                RouteAttrs::Uid(val) => fmt.field("Uid", &val),
                RouteAttrs::TtlPropagate(val) => fmt.field("TtlPropagate", &val),
                RouteAttrs::IpProto(val) => fmt.field("IpProto", &val),
                RouteAttrs::Sport(val) => fmt.field("Sport", &val),
                RouteAttrs::Dport(val) => fmt.field("Dport", &val),
                RouteAttrs::NhId(val) => fmt.field("NhId", &val),
                RouteAttrs::Flowlabel(val) => fmt.field("Flowlabel", &val),
            };
        }
        fmt.finish()
    }
}
#[doc = "Original name: \"metrics\""]
#[derive(Clone)]
pub enum Metrics<'a> {
    Lock(u32),
    Mtu(u32),
    Window(u32),
    Rtt(u32),
    Rttvar(u32),
    Ssthresh(u32),
    Cwnd(u32),
    Advmss(u32),
    Reordering(u32),
    Hoplimit(u32),
    Initcwnd(u32),
    Features(u32),
    RtoMin(u32),
    Initrwnd(u32),
    Quickack(u32),
    CcAlgo(&'a CStr),
    FastopenNoCookie(u32),
}
impl<'a> Iterable<'a, Metrics<'a>> {
    pub fn get_lock(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let Metrics::Lock(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_mtu(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let Metrics::Mtu(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_window(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let Metrics::Window(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_rtt(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let Metrics::Rtt(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_rttvar(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let Metrics::Rttvar(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_ssthresh(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let Metrics::Ssthresh(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_cwnd(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let Metrics::Cwnd(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_advmss(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let Metrics::Advmss(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_reordering(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let Metrics::Reordering(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_hoplimit(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let Metrics::Hoplimit(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_initcwnd(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let Metrics::Initcwnd(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_features(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let Metrics::Features(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_rto_min(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let Metrics::RtoMin(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_initrwnd(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let Metrics::Initrwnd(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_quickack(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let Metrics::Quickack(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_cc_algo(&self) -> Option<&'a CStr> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let Metrics::CcAlgo(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_fastopen_no_cookie(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let Metrics::FastopenNoCookie(val) = attr {
                return Some(val);
            }
        }
        None
    }
}
impl<'a> Metrics<'a> {
    pub fn new(buf: &'a [u8]) -> Iterable<'a, Metrics<'a>> {
        Iterable::new(buf)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "Unspec",
            1u16 => "Lock",
            2u16 => "Mtu",
            3u16 => "Window",
            4u16 => "Rtt",
            5u16 => "Rttvar",
            6u16 => "Ssthresh",
            7u16 => "Cwnd",
            8u16 => "Advmss",
            9u16 => "Reordering",
            10u16 => "Hoplimit",
            11u16 => "Initcwnd",
            12u16 => "Features",
            13u16 => "RtoMin",
            14u16 => "Initrwnd",
            15u16 => "Quickack",
            16u16 => "CcAlgo",
            17u16 => "FastopenNoCookie",
            _ => return None,
        };
        Some(res)
    }
}
impl<'a> Iterator for Iterable<'a, Metrics<'a>> {
    type Item = Result<Metrics<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => Metrics::Lock({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Metrics::Mtu({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Metrics::Window({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Metrics::Rtt({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Metrics::Rttvar({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Metrics::Ssthresh({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => Metrics::Cwnd({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => Metrics::Advmss({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => Metrics::Reordering({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => Metrics::Hoplimit({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => Metrics::Initcwnd({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => Metrics::Features({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => Metrics::RtoMin({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => Metrics::Initrwnd({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => Metrics::Quickack({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => Metrics::CcAlgo({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => Metrics::FastopenNoCookie({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "Metrics",
            r#type.and_then(|t| Metrics::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, Metrics<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Metrics");
        for attr in self.clone() {
            let attr = match attr {
                Ok(a) => a,
                Err(err) => {
                    fmt.finish()?;
                    f.write_str("Err(")?;
                    err.fmt(f)?;
                    return f.write_str(")");
                }
            };
            match attr {
                Metrics::Lock(val) => fmt.field("Lock", &val),
                Metrics::Mtu(val) => fmt.field("Mtu", &val),
                Metrics::Window(val) => fmt.field("Window", &val),
                Metrics::Rtt(val) => fmt.field("Rtt", &val),
                Metrics::Rttvar(val) => fmt.field("Rttvar", &val),
                Metrics::Ssthresh(val) => fmt.field("Ssthresh", &val),
                Metrics::Cwnd(val) => fmt.field("Cwnd", &val),
                Metrics::Advmss(val) => fmt.field("Advmss", &val),
                Metrics::Reordering(val) => fmt.field("Reordering", &val),
                Metrics::Hoplimit(val) => fmt.field("Hoplimit", &val),
                Metrics::Initcwnd(val) => fmt.field("Initcwnd", &val),
                Metrics::Features(val) => fmt.field("Features", &val),
                Metrics::RtoMin(val) => fmt.field("RtoMin", &val),
                Metrics::Initrwnd(val) => fmt.field("Initrwnd", &val),
                Metrics::Quickack(val) => fmt.field("Quickack", &val),
                Metrics::CcAlgo(val) => fmt.field("CcAlgo", &val),
                Metrics::FastopenNoCookie(val) => fmt.field("FastopenNoCookie", &val),
            };
        }
        fmt.finish()
    }
}
pub struct PushRouteAttrs<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushRouteAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushRouteAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_dst(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 1u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_src(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_iif(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_oif(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_gateway(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 5u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_priority(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 6u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_prefsrc(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 7u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn nested_metrics(mut self) -> PushMetrics<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 8u16);
        PushMetrics {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_multipath(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 9u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_protoinfo(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 10u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_flow(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 11u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_cacheinfo(mut self, value: PushRtaCacheinfo) -> Self {
        push_header(self.as_rec_mut(), 12u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_session(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 13u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_mp_algo(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 14u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_table(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 15u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mark(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 16u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mfc_stats(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 17u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_via(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 18u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_newdst(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 19u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_pref(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 20u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap_type(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 21u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 22u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_expires(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 23u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 24u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_uid(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 25u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ttl_propagate(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 26u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ip_proto(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 27u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sport(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 28u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dport(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 29u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_nh_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 30u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flowlabel(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 31u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushRouteAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushMetrics<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushMetrics<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushMetrics<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_lock(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mtu(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_window(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_rtt(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_rttvar(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 5u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ssthresh(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 6u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_cwnd(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 7u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_advmss(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_reordering(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 9u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_hoplimit(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 10u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_initcwnd(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 11u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_features(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 12u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_rto_min(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 13u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_initrwnd(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 14u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_quickack(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 15u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_cc_algo(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            16u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_fastopen_no_cookie(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 17u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushMetrics<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Original name: \"rtmsg\""]
#[derive(Clone)]
pub struct PushRtmsg {
    buf: [u8; 12usize],
}
impl PushRtmsg {
    #[doc = "Create zero-initialized struct"]
    pub fn new() -> Self {
        Self {
            buf: [0u8; Self::len()],
        }
    }
    #[doc = "Copy from contents from other slice"]
    pub fn new_from_slice(other: &[u8]) -> Option<Self> {
        if other.len() != Self::len() {
            return None;
        }
        let mut buf = [0u8; Self::len()];
        buf.clone_from_slice(other);
        Some(Self { buf })
    }
    pub fn as_slice(&self) -> &[u8] {
        &self.buf
    }
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.buf
    }
    pub const fn len() -> usize {
        12usize
    }
    pub fn rtm_family(&self) -> u8 {
        parse_u8(&self.buf[0usize..1usize]).unwrap()
    }
    pub fn set_rtm_family(&mut self, value: u8) {
        self.buf[0usize..1usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn rtm_dst_len(&self) -> u8 {
        parse_u8(&self.buf[1usize..2usize]).unwrap()
    }
    pub fn set_rtm_dst_len(&mut self, value: u8) {
        self.buf[1usize..2usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn rtm_src_len(&self) -> u8 {
        parse_u8(&self.buf[2usize..3usize]).unwrap()
    }
    pub fn set_rtm_src_len(&mut self, value: u8) {
        self.buf[2usize..3usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn rtm_tos(&self) -> u8 {
        parse_u8(&self.buf[3usize..4usize]).unwrap()
    }
    pub fn set_rtm_tos(&mut self, value: u8) {
        self.buf[3usize..4usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn rtm_table(&self) -> u8 {
        parse_u8(&self.buf[4usize..5usize]).unwrap()
    }
    pub fn set_rtm_table(&mut self, value: u8) {
        self.buf[4usize..5usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn rtm_protocol(&self) -> u8 {
        parse_u8(&self.buf[5usize..6usize]).unwrap()
    }
    pub fn set_rtm_protocol(&mut self, value: u8) {
        self.buf[5usize..6usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn rtm_scope(&self) -> u8 {
        parse_u8(&self.buf[6usize..7usize]).unwrap()
    }
    pub fn set_rtm_scope(&mut self, value: u8) {
        self.buf[6usize..7usize].copy_from_slice(&value.to_ne_bytes())
    }
    #[doc = "Associated type: \"RtmType\" (enum)"]
    pub fn rtm_type(&self) -> u8 {
        parse_u8(&self.buf[7usize..8usize]).unwrap()
    }
    #[doc = "Associated type: \"RtmType\" (enum)"]
    pub fn set_rtm_type(&mut self, value: u8) {
        self.buf[7usize..8usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn rtm_flags(&self) -> u32 {
        parse_u32(&self.buf[8usize..12usize]).unwrap()
    }
    pub fn set_rtm_flags(&mut self, value: u32) {
        self.buf[8usize..12usize].copy_from_slice(&value.to_ne_bytes())
    }
}
impl std::fmt::Debug for PushRtmsg {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Rtmsg")
            .field("rtm_family", &self.rtm_family())
            .field("rtm_dst_len", &self.rtm_dst_len())
            .field("rtm_src_len", &self.rtm_src_len())
            .field("rtm_tos", &self.rtm_tos())
            .field("rtm_table", &self.rtm_table())
            .field("rtm_protocol", &self.rtm_protocol())
            .field("rtm_scope", &self.rtm_scope())
            .field("rtm_type", &self.rtm_type())
            .field("rtm_flags", &self.rtm_flags())
            .finish()
    }
}
#[doc = "Original name: \"rta-cacheinfo\""]
#[derive(Clone)]
pub struct PushRtaCacheinfo {
    buf: [u8; 20usize],
}
impl PushRtaCacheinfo {
    #[doc = "Create zero-initialized struct"]
    pub fn new() -> Self {
        Self {
            buf: [0u8; Self::len()],
        }
    }
    #[doc = "Copy from contents from other slice"]
    pub fn new_from_slice(other: &[u8]) -> Option<Self> {
        if other.len() != Self::len() {
            return None;
        }
        let mut buf = [0u8; Self::len()];
        buf.clone_from_slice(other);
        Some(Self { buf })
    }
    pub fn as_slice(&self) -> &[u8] {
        &self.buf
    }
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.buf
    }
    pub const fn len() -> usize {
        20usize
    }
    pub fn rta_clntref(&self) -> u32 {
        parse_u32(&self.buf[0usize..4usize]).unwrap()
    }
    pub fn set_rta_clntref(&mut self, value: u32) {
        self.buf[0usize..4usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn rta_lastuse(&self) -> u32 {
        parse_u32(&self.buf[4usize..8usize]).unwrap()
    }
    pub fn set_rta_lastuse(&mut self, value: u32) {
        self.buf[4usize..8usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn rta_expires(&self) -> u32 {
        parse_u32(&self.buf[8usize..12usize]).unwrap()
    }
    pub fn set_rta_expires(&mut self, value: u32) {
        self.buf[8usize..12usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn rta_error(&self) -> u32 {
        parse_u32(&self.buf[12usize..16usize]).unwrap()
    }
    pub fn set_rta_error(&mut self, value: u32) {
        self.buf[12usize..16usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn rta_used(&self) -> u32 {
        parse_u32(&self.buf[16usize..20usize]).unwrap()
    }
    pub fn set_rta_used(&mut self, value: u32) {
        self.buf[16usize..20usize].copy_from_slice(&value.to_ne_bytes())
    }
}
impl std::fmt::Debug for PushRtaCacheinfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("RtaCacheinfo")
            .field("rta_clntref", &self.rta_clntref())
            .field("rta_lastuse", &self.rta_lastuse())
            .field("rta_expires", &self.rta_expires())
            .field("rta_error", &self.rta_error())
            .field("rta_used", &self.rta_used())
            .finish()
    }
}
#[doc = "Dump route information."]
pub struct PushOpGetrouteDumpRequest<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetrouteDumpRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetrouteDumpRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushRtmsg) -> Self {
        prev.as_rec_mut().extend(header.as_slice());
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
}
impl<Prev: Rec> Drop for PushOpGetrouteDumpRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Dump route information."]
#[doc = "Original name: \"OpGetrouteDumpRequest\""]
#[derive(Clone)]
pub enum OpGetrouteDumpRequest {}
impl<'a> Iterable<'a, OpGetrouteDumpRequest> {}
impl OpGetrouteDumpRequest {
    pub fn new(buf: &'_ [u8]) -> (PushRtmsg, Iterable<'_, OpGetrouteDumpRequest>) {
        let mut header = PushRtmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushRtmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushRtmsg::len()..], buf.as_ptr()),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Dst",
            2u16 => "Src",
            3u16 => "Iif",
            4u16 => "Oif",
            5u16 => "Gateway",
            6u16 => "Priority",
            7u16 => "Prefsrc",
            8u16 => "Metrics",
            9u16 => "Multipath",
            10u16 => "Protoinfo",
            11u16 => "Flow",
            12u16 => "Cacheinfo",
            13u16 => "Session",
            14u16 => "MpAlgo",
            15u16 => "Table",
            16u16 => "Mark",
            17u16 => "MfcStats",
            18u16 => "Via",
            19u16 => "Newdst",
            20u16 => "Pref",
            21u16 => "EncapType",
            22u16 => "Encap",
            23u16 => "Expires",
            24u16 => "Pad",
            25u16 => "Uid",
            26u16 => "TtlPropagate",
            27u16 => "IpProto",
            28u16 => "Sport",
            29u16 => "Dport",
            30u16 => "NhId",
            31u16 => "Flowlabel",
            _ => return None,
        };
        Some(res)
    }
}
impl Iterator for Iterable<'_, OpGetrouteDumpRequest> {
    type Item = Result<OpGetrouteDumpRequest, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpGetrouteDumpRequest",
            r#type.and_then(|t| OpGetrouteDumpRequest::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl std::fmt::Debug for Iterable<'_, OpGetrouteDumpRequest> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetrouteDumpRequest");
        for attr in self.clone() {
            let attr = match attr {
                Ok(a) => a,
                Err(err) => {
                    fmt.finish()?;
                    f.write_str("Err(")?;
                    err.fmt(f)?;
                    return f.write_str(")");
                }
            };
            match attr {};
        }
        fmt.finish()
    }
}
#[doc = "Dump route information."]
pub struct PushOpGetrouteDumpReply<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetrouteDumpReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetrouteDumpReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushRtmsg) -> Self {
        prev.as_rec_mut().extend(header.as_slice());
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_dst(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 1u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_src(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_iif(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_oif(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_gateway(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 5u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_priority(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 6u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_prefsrc(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 7u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn nested_metrics(mut self) -> PushMetrics<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 8u16);
        PushMetrics {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_multipath(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 9u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_flow(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 11u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_cacheinfo(mut self, value: PushRtaCacheinfo) -> Self {
        push_header(self.as_rec_mut(), 12u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_table(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 15u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mark(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 16u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mfc_stats(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 17u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_via(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 18u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_newdst(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 19u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_pref(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 20u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap_type(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 21u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 22u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_expires(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 23u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 24u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_uid(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 25u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ttl_propagate(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 26u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ip_proto(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 27u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sport(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 28u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dport(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 29u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_nh_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 30u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flowlabel(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 31u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpGetrouteDumpReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Dump route information."]
#[doc = "Original name: \"OpGetrouteDumpReply\""]
#[derive(Clone)]
pub enum OpGetrouteDumpReply<'a> {
    Dst(&'a [u8]),
    Src(&'a [u8]),
    Iif(u32),
    Oif(u32),
    Gateway(&'a [u8]),
    Priority(u32),
    Prefsrc(&'a [u8]),
    Metrics(Iterable<'a, Metrics<'a>>),
    Multipath(&'a [u8]),
    Flow(u32),
    Cacheinfo(PushRtaCacheinfo),
    Table(u32),
    Mark(u32),
    MfcStats(&'a [u8]),
    Via(&'a [u8]),
    Newdst(&'a [u8]),
    Pref(u8),
    EncapType(u16),
    Encap(&'a [u8]),
    Expires(u32),
    Pad(&'a [u8]),
    Uid(u32),
    TtlPropagate(u8),
    IpProto(u8),
    Sport(u16),
    Dport(u16),
    NhId(u32),
    Flowlabel(u32),
}
impl<'a> Iterable<'a, OpGetrouteDumpReply<'a>> {
    pub fn get_dst(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::Dst(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_src(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::Src(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_iif(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::Iif(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_oif(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::Oif(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_gateway(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::Gateway(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_priority(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::Priority(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_prefsrc(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::Prefsrc(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_metrics(&self) -> Iterable<'a, Metrics<'a>> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::Metrics(val) = attr {
                return val;
            }
        }
        Iterable::new(&[])
    }
    pub fn get_multipath(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::Multipath(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_flow(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::Flow(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_cacheinfo(&self) -> Option<PushRtaCacheinfo> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::Cacheinfo(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_table(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::Table(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_mark(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::Mark(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_mfc_stats(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::MfcStats(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_via(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::Via(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_newdst(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::Newdst(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_pref(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::Pref(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_encap_type(&self) -> Option<u16> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::EncapType(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_encap(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::Encap(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_expires(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::Expires(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_pad(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::Pad(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_uid(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::Uid(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_ttl_propagate(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::TtlPropagate(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_ip_proto(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::IpProto(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_sport(&self) -> Option<u16> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::Sport(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_dport(&self) -> Option<u16> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::Dport(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_nh_id(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::NhId(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_flowlabel(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDumpReply::Flowlabel(val) = attr {
                return Some(val);
            }
        }
        None
    }
}
impl<'a> OpGetrouteDumpReply<'a> {
    pub fn new(buf: &'a [u8]) -> (PushRtmsg, Iterable<'a, OpGetrouteDumpReply<'a>>) {
        let mut header = PushRtmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushRtmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushRtmsg::len()..], buf.as_ptr()),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Dst",
            2u16 => "Src",
            3u16 => "Iif",
            4u16 => "Oif",
            5u16 => "Gateway",
            6u16 => "Priority",
            7u16 => "Prefsrc",
            8u16 => "Metrics",
            9u16 => "Multipath",
            10u16 => "Protoinfo",
            11u16 => "Flow",
            12u16 => "Cacheinfo",
            13u16 => "Session",
            14u16 => "MpAlgo",
            15u16 => "Table",
            16u16 => "Mark",
            17u16 => "MfcStats",
            18u16 => "Via",
            19u16 => "Newdst",
            20u16 => "Pref",
            21u16 => "EncapType",
            22u16 => "Encap",
            23u16 => "Expires",
            24u16 => "Pad",
            25u16 => "Uid",
            26u16 => "TtlPropagate",
            27u16 => "IpProto",
            28u16 => "Sport",
            29u16 => "Dport",
            30u16 => "NhId",
            31u16 => "Flowlabel",
            _ => return None,
        };
        Some(res)
    }
}
impl<'a> Iterator for Iterable<'a, OpGetrouteDumpReply<'a>> {
    type Item = Result<OpGetrouteDumpReply<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpGetrouteDumpReply::Dst({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpGetrouteDumpReply::Src({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpGetrouteDumpReply::Iif({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpGetrouteDumpReply::Oif({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpGetrouteDumpReply::Gateway({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpGetrouteDumpReply::Priority({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => OpGetrouteDumpReply::Prefsrc({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpGetrouteDumpReply::Metrics({
                    let res = Some(Iterable::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => OpGetrouteDumpReply::Multipath({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => OpGetrouteDumpReply::Flow({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => OpGetrouteDumpReply::Cacheinfo({
                    let res = PushRtaCacheinfo::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => OpGetrouteDumpReply::Table({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => OpGetrouteDumpReply::Mark({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => OpGetrouteDumpReply::MfcStats({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => OpGetrouteDumpReply::Via({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => OpGetrouteDumpReply::Newdst({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => OpGetrouteDumpReply::Pref({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => OpGetrouteDumpReply::EncapType({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => OpGetrouteDumpReply::Encap({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => OpGetrouteDumpReply::Expires({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => OpGetrouteDumpReply::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                25u16 => OpGetrouteDumpReply::Uid({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                26u16 => OpGetrouteDumpReply::TtlPropagate({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                27u16 => OpGetrouteDumpReply::IpProto({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                28u16 => OpGetrouteDumpReply::Sport({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                29u16 => OpGetrouteDumpReply::Dport({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                30u16 => OpGetrouteDumpReply::NhId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                31u16 => OpGetrouteDumpReply::Flowlabel({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpGetrouteDumpReply",
            r#type.and_then(|t| OpGetrouteDumpReply::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpGetrouteDumpReply<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetrouteDumpReply");
        for attr in self.clone() {
            let attr = match attr {
                Ok(a) => a,
                Err(err) => {
                    fmt.finish()?;
                    f.write_str("Err(")?;
                    err.fmt(f)?;
                    return f.write_str(")");
                }
            };
            match attr {
                OpGetrouteDumpReply::Dst(val) => fmt.field("Dst", &val),
                OpGetrouteDumpReply::Src(val) => fmt.field("Src", &val),
                OpGetrouteDumpReply::Iif(val) => fmt.field("Iif", &val),
                OpGetrouteDumpReply::Oif(val) => fmt.field("Oif", &val),
                OpGetrouteDumpReply::Gateway(val) => fmt.field("Gateway", &val),
                OpGetrouteDumpReply::Priority(val) => fmt.field("Priority", &val),
                OpGetrouteDumpReply::Prefsrc(val) => fmt.field("Prefsrc", &val),
                OpGetrouteDumpReply::Metrics(val) => fmt.field("Metrics", &val),
                OpGetrouteDumpReply::Multipath(val) => fmt.field("Multipath", &val),
                OpGetrouteDumpReply::Flow(val) => fmt.field("Flow", &val),
                OpGetrouteDumpReply::Cacheinfo(val) => fmt.field("Cacheinfo", &val),
                OpGetrouteDumpReply::Table(val) => fmt.field("Table", &val),
                OpGetrouteDumpReply::Mark(val) => fmt.field("Mark", &val),
                OpGetrouteDumpReply::MfcStats(val) => fmt.field("MfcStats", &val),
                OpGetrouteDumpReply::Via(val) => fmt.field("Via", &val),
                OpGetrouteDumpReply::Newdst(val) => fmt.field("Newdst", &val),
                OpGetrouteDumpReply::Pref(val) => fmt.field("Pref", &val),
                OpGetrouteDumpReply::EncapType(val) => fmt.field("EncapType", &val),
                OpGetrouteDumpReply::Encap(val) => fmt.field("Encap", &val),
                OpGetrouteDumpReply::Expires(val) => fmt.field("Expires", &val),
                OpGetrouteDumpReply::Pad(val) => fmt.field("Pad", &val),
                OpGetrouteDumpReply::Uid(val) => fmt.field("Uid", &val),
                OpGetrouteDumpReply::TtlPropagate(val) => fmt.field("TtlPropagate", &val),
                OpGetrouteDumpReply::IpProto(val) => fmt.field("IpProto", &val),
                OpGetrouteDumpReply::Sport(val) => fmt.field("Sport", &val),
                OpGetrouteDumpReply::Dport(val) => fmt.field("Dport", &val),
                OpGetrouteDumpReply::NhId(val) => fmt.field("NhId", &val),
                OpGetrouteDumpReply::Flowlabel(val) => fmt.field("Flowlabel", &val),
            };
        }
        fmt.finish()
    }
}
#[doc = "Dump route information."]
pub struct PushOpGetrouteDoRequest<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetrouteDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetrouteDoRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushRtmsg) -> Self {
        prev.as_rec_mut().extend(header.as_slice());
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_dst(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 1u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_src(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_iif(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_oif(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mark(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 16u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_uid(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 25u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ip_proto(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 27u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sport(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 28u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dport(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 29u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flowlabel(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 31u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpGetrouteDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Dump route information."]
#[doc = "Original name: \"OpGetrouteDoRequest\""]
#[derive(Clone)]
pub enum OpGetrouteDoRequest<'a> {
    Dst(&'a [u8]),
    Src(&'a [u8]),
    Iif(u32),
    Oif(u32),
    Mark(u32),
    Uid(u32),
    IpProto(u8),
    Sport(u16),
    Dport(u16),
    Flowlabel(u32),
}
impl<'a> Iterable<'a, OpGetrouteDoRequest<'a>> {
    pub fn get_dst(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoRequest::Dst(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_src(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoRequest::Src(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_iif(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoRequest::Iif(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_oif(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoRequest::Oif(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_mark(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoRequest::Mark(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_uid(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoRequest::Uid(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_ip_proto(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoRequest::IpProto(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_sport(&self) -> Option<u16> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoRequest::Sport(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_dport(&self) -> Option<u16> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoRequest::Dport(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_flowlabel(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoRequest::Flowlabel(val) = attr {
                return Some(val);
            }
        }
        None
    }
}
impl<'a> OpGetrouteDoRequest<'a> {
    pub fn new(buf: &'a [u8]) -> (PushRtmsg, Iterable<'a, OpGetrouteDoRequest<'a>>) {
        let mut header = PushRtmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushRtmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushRtmsg::len()..], buf.as_ptr()),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Dst",
            2u16 => "Src",
            3u16 => "Iif",
            4u16 => "Oif",
            5u16 => "Gateway",
            6u16 => "Priority",
            7u16 => "Prefsrc",
            8u16 => "Metrics",
            9u16 => "Multipath",
            10u16 => "Protoinfo",
            11u16 => "Flow",
            12u16 => "Cacheinfo",
            13u16 => "Session",
            14u16 => "MpAlgo",
            15u16 => "Table",
            16u16 => "Mark",
            17u16 => "MfcStats",
            18u16 => "Via",
            19u16 => "Newdst",
            20u16 => "Pref",
            21u16 => "EncapType",
            22u16 => "Encap",
            23u16 => "Expires",
            24u16 => "Pad",
            25u16 => "Uid",
            26u16 => "TtlPropagate",
            27u16 => "IpProto",
            28u16 => "Sport",
            29u16 => "Dport",
            30u16 => "NhId",
            31u16 => "Flowlabel",
            _ => return None,
        };
        Some(res)
    }
}
impl<'a> Iterator for Iterable<'a, OpGetrouteDoRequest<'a>> {
    type Item = Result<OpGetrouteDoRequest<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpGetrouteDoRequest::Dst({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpGetrouteDoRequest::Src({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpGetrouteDoRequest::Iif({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpGetrouteDoRequest::Oif({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => OpGetrouteDoRequest::Mark({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                25u16 => OpGetrouteDoRequest::Uid({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                27u16 => OpGetrouteDoRequest::IpProto({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                28u16 => OpGetrouteDoRequest::Sport({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                29u16 => OpGetrouteDoRequest::Dport({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                31u16 => OpGetrouteDoRequest::Flowlabel({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpGetrouteDoRequest",
            r#type.and_then(|t| OpGetrouteDoRequest::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpGetrouteDoRequest<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetrouteDoRequest");
        for attr in self.clone() {
            let attr = match attr {
                Ok(a) => a,
                Err(err) => {
                    fmt.finish()?;
                    f.write_str("Err(")?;
                    err.fmt(f)?;
                    return f.write_str(")");
                }
            };
            match attr {
                OpGetrouteDoRequest::Dst(val) => fmt.field("Dst", &val),
                OpGetrouteDoRequest::Src(val) => fmt.field("Src", &val),
                OpGetrouteDoRequest::Iif(val) => fmt.field("Iif", &val),
                OpGetrouteDoRequest::Oif(val) => fmt.field("Oif", &val),
                OpGetrouteDoRequest::Mark(val) => fmt.field("Mark", &val),
                OpGetrouteDoRequest::Uid(val) => fmt.field("Uid", &val),
                OpGetrouteDoRequest::IpProto(val) => fmt.field("IpProto", &val),
                OpGetrouteDoRequest::Sport(val) => fmt.field("Sport", &val),
                OpGetrouteDoRequest::Dport(val) => fmt.field("Dport", &val),
                OpGetrouteDoRequest::Flowlabel(val) => fmt.field("Flowlabel", &val),
            };
        }
        fmt.finish()
    }
}
#[doc = "Dump route information."]
pub struct PushOpGetrouteDoReply<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetrouteDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetrouteDoReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushRtmsg) -> Self {
        prev.as_rec_mut().extend(header.as_slice());
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_dst(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 1u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_src(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_iif(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_oif(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_gateway(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 5u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_priority(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 6u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_prefsrc(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 7u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn nested_metrics(mut self) -> PushMetrics<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 8u16);
        PushMetrics {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_multipath(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 9u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_flow(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 11u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_cacheinfo(mut self, value: PushRtaCacheinfo) -> Self {
        push_header(self.as_rec_mut(), 12u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_table(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 15u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mark(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 16u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mfc_stats(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 17u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_via(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 18u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_newdst(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 19u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_pref(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 20u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap_type(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 21u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 22u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_expires(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 23u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 24u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_uid(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 25u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ttl_propagate(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 26u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ip_proto(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 27u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sport(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 28u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dport(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 29u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_nh_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 30u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flowlabel(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 31u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpGetrouteDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Dump route information."]
#[doc = "Original name: \"OpGetrouteDoReply\""]
#[derive(Clone)]
pub enum OpGetrouteDoReply<'a> {
    Dst(&'a [u8]),
    Src(&'a [u8]),
    Iif(u32),
    Oif(u32),
    Gateway(&'a [u8]),
    Priority(u32),
    Prefsrc(&'a [u8]),
    Metrics(Iterable<'a, Metrics<'a>>),
    Multipath(&'a [u8]),
    Flow(u32),
    Cacheinfo(PushRtaCacheinfo),
    Table(u32),
    Mark(u32),
    MfcStats(&'a [u8]),
    Via(&'a [u8]),
    Newdst(&'a [u8]),
    Pref(u8),
    EncapType(u16),
    Encap(&'a [u8]),
    Expires(u32),
    Pad(&'a [u8]),
    Uid(u32),
    TtlPropagate(u8),
    IpProto(u8),
    Sport(u16),
    Dport(u16),
    NhId(u32),
    Flowlabel(u32),
}
impl<'a> Iterable<'a, OpGetrouteDoReply<'a>> {
    pub fn get_dst(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::Dst(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_src(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::Src(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_iif(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::Iif(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_oif(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::Oif(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_gateway(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::Gateway(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_priority(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::Priority(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_prefsrc(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::Prefsrc(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_metrics(&self) -> Iterable<'a, Metrics<'a>> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::Metrics(val) = attr {
                return val;
            }
        }
        Iterable::new(&[])
    }
    pub fn get_multipath(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::Multipath(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_flow(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::Flow(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_cacheinfo(&self) -> Option<PushRtaCacheinfo> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::Cacheinfo(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_table(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::Table(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_mark(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::Mark(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_mfc_stats(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::MfcStats(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_via(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::Via(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_newdst(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::Newdst(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_pref(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::Pref(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_encap_type(&self) -> Option<u16> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::EncapType(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_encap(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::Encap(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_expires(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::Expires(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_pad(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::Pad(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_uid(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::Uid(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_ttl_propagate(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::TtlPropagate(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_ip_proto(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::IpProto(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_sport(&self) -> Option<u16> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::Sport(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_dport(&self) -> Option<u16> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::Dport(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_nh_id(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::NhId(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_flowlabel(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetrouteDoReply::Flowlabel(val) = attr {
                return Some(val);
            }
        }
        None
    }
}
impl<'a> OpGetrouteDoReply<'a> {
    pub fn new(buf: &'a [u8]) -> (PushRtmsg, Iterable<'a, OpGetrouteDoReply<'a>>) {
        let mut header = PushRtmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushRtmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushRtmsg::len()..], buf.as_ptr()),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Dst",
            2u16 => "Src",
            3u16 => "Iif",
            4u16 => "Oif",
            5u16 => "Gateway",
            6u16 => "Priority",
            7u16 => "Prefsrc",
            8u16 => "Metrics",
            9u16 => "Multipath",
            10u16 => "Protoinfo",
            11u16 => "Flow",
            12u16 => "Cacheinfo",
            13u16 => "Session",
            14u16 => "MpAlgo",
            15u16 => "Table",
            16u16 => "Mark",
            17u16 => "MfcStats",
            18u16 => "Via",
            19u16 => "Newdst",
            20u16 => "Pref",
            21u16 => "EncapType",
            22u16 => "Encap",
            23u16 => "Expires",
            24u16 => "Pad",
            25u16 => "Uid",
            26u16 => "TtlPropagate",
            27u16 => "IpProto",
            28u16 => "Sport",
            29u16 => "Dport",
            30u16 => "NhId",
            31u16 => "Flowlabel",
            _ => return None,
        };
        Some(res)
    }
}
impl<'a> Iterator for Iterable<'a, OpGetrouteDoReply<'a>> {
    type Item = Result<OpGetrouteDoReply<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpGetrouteDoReply::Dst({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpGetrouteDoReply::Src({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpGetrouteDoReply::Iif({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpGetrouteDoReply::Oif({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpGetrouteDoReply::Gateway({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpGetrouteDoReply::Priority({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => OpGetrouteDoReply::Prefsrc({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpGetrouteDoReply::Metrics({
                    let res = Some(Iterable::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => OpGetrouteDoReply::Multipath({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => OpGetrouteDoReply::Flow({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => OpGetrouteDoReply::Cacheinfo({
                    let res = PushRtaCacheinfo::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => OpGetrouteDoReply::Table({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => OpGetrouteDoReply::Mark({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => OpGetrouteDoReply::MfcStats({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => OpGetrouteDoReply::Via({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => OpGetrouteDoReply::Newdst({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => OpGetrouteDoReply::Pref({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => OpGetrouteDoReply::EncapType({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => OpGetrouteDoReply::Encap({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => OpGetrouteDoReply::Expires({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => OpGetrouteDoReply::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                25u16 => OpGetrouteDoReply::Uid({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                26u16 => OpGetrouteDoReply::TtlPropagate({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                27u16 => OpGetrouteDoReply::IpProto({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                28u16 => OpGetrouteDoReply::Sport({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                29u16 => OpGetrouteDoReply::Dport({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                30u16 => OpGetrouteDoReply::NhId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                31u16 => OpGetrouteDoReply::Flowlabel({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpGetrouteDoReply",
            r#type.and_then(|t| OpGetrouteDoReply::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpGetrouteDoReply<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetrouteDoReply");
        for attr in self.clone() {
            let attr = match attr {
                Ok(a) => a,
                Err(err) => {
                    fmt.finish()?;
                    f.write_str("Err(")?;
                    err.fmt(f)?;
                    return f.write_str(")");
                }
            };
            match attr {
                OpGetrouteDoReply::Dst(val) => fmt.field("Dst", &val),
                OpGetrouteDoReply::Src(val) => fmt.field("Src", &val),
                OpGetrouteDoReply::Iif(val) => fmt.field("Iif", &val),
                OpGetrouteDoReply::Oif(val) => fmt.field("Oif", &val),
                OpGetrouteDoReply::Gateway(val) => fmt.field("Gateway", &val),
                OpGetrouteDoReply::Priority(val) => fmt.field("Priority", &val),
                OpGetrouteDoReply::Prefsrc(val) => fmt.field("Prefsrc", &val),
                OpGetrouteDoReply::Metrics(val) => fmt.field("Metrics", &val),
                OpGetrouteDoReply::Multipath(val) => fmt.field("Multipath", &val),
                OpGetrouteDoReply::Flow(val) => fmt.field("Flow", &val),
                OpGetrouteDoReply::Cacheinfo(val) => fmt.field("Cacheinfo", &val),
                OpGetrouteDoReply::Table(val) => fmt.field("Table", &val),
                OpGetrouteDoReply::Mark(val) => fmt.field("Mark", &val),
                OpGetrouteDoReply::MfcStats(val) => fmt.field("MfcStats", &val),
                OpGetrouteDoReply::Via(val) => fmt.field("Via", &val),
                OpGetrouteDoReply::Newdst(val) => fmt.field("Newdst", &val),
                OpGetrouteDoReply::Pref(val) => fmt.field("Pref", &val),
                OpGetrouteDoReply::EncapType(val) => fmt.field("EncapType", &val),
                OpGetrouteDoReply::Encap(val) => fmt.field("Encap", &val),
                OpGetrouteDoReply::Expires(val) => fmt.field("Expires", &val),
                OpGetrouteDoReply::Pad(val) => fmt.field("Pad", &val),
                OpGetrouteDoReply::Uid(val) => fmt.field("Uid", &val),
                OpGetrouteDoReply::TtlPropagate(val) => fmt.field("TtlPropagate", &val),
                OpGetrouteDoReply::IpProto(val) => fmt.field("IpProto", &val),
                OpGetrouteDoReply::Sport(val) => fmt.field("Sport", &val),
                OpGetrouteDoReply::Dport(val) => fmt.field("Dport", &val),
                OpGetrouteDoReply::NhId(val) => fmt.field("NhId", &val),
                OpGetrouteDoReply::Flowlabel(val) => fmt.field("Flowlabel", &val),
            };
        }
        fmt.finish()
    }
}
#[doc = "Create a new route"]
pub struct PushOpNewrouteDoRequest<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpNewrouteDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpNewrouteDoRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushRtmsg) -> Self {
        prev.as_rec_mut().extend(header.as_slice());
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_dst(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 1u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_src(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_iif(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_oif(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_gateway(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 5u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_priority(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 6u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_prefsrc(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 7u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn nested_metrics(mut self) -> PushMetrics<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 8u16);
        PushMetrics {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_multipath(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 9u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_flow(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 11u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_cacheinfo(mut self, value: PushRtaCacheinfo) -> Self {
        push_header(self.as_rec_mut(), 12u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_table(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 15u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mark(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 16u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mfc_stats(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 17u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_via(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 18u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_newdst(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 19u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_pref(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 20u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap_type(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 21u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 22u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_expires(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 23u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 24u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_uid(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 25u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ttl_propagate(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 26u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ip_proto(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 27u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sport(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 28u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dport(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 29u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_nh_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 30u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flowlabel(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 31u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpNewrouteDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Create a new route"]
#[doc = "Original name: \"OpNewrouteDoRequest\""]
#[derive(Clone)]
pub enum OpNewrouteDoRequest<'a> {
    Dst(&'a [u8]),
    Src(&'a [u8]),
    Iif(u32),
    Oif(u32),
    Gateway(&'a [u8]),
    Priority(u32),
    Prefsrc(&'a [u8]),
    Metrics(Iterable<'a, Metrics<'a>>),
    Multipath(&'a [u8]),
    Flow(u32),
    Cacheinfo(PushRtaCacheinfo),
    Table(u32),
    Mark(u32),
    MfcStats(&'a [u8]),
    Via(&'a [u8]),
    Newdst(&'a [u8]),
    Pref(u8),
    EncapType(u16),
    Encap(&'a [u8]),
    Expires(u32),
    Pad(&'a [u8]),
    Uid(u32),
    TtlPropagate(u8),
    IpProto(u8),
    Sport(u16),
    Dport(u16),
    NhId(u32),
    Flowlabel(u32),
}
impl<'a> Iterable<'a, OpNewrouteDoRequest<'a>> {
    pub fn get_dst(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::Dst(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_src(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::Src(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_iif(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::Iif(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_oif(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::Oif(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_gateway(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::Gateway(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_priority(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::Priority(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_prefsrc(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::Prefsrc(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_metrics(&self) -> Iterable<'a, Metrics<'a>> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::Metrics(val) = attr {
                return val;
            }
        }
        Iterable::new(&[])
    }
    pub fn get_multipath(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::Multipath(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_flow(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::Flow(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_cacheinfo(&self) -> Option<PushRtaCacheinfo> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::Cacheinfo(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_table(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::Table(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_mark(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::Mark(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_mfc_stats(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::MfcStats(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_via(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::Via(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_newdst(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::Newdst(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_pref(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::Pref(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_encap_type(&self) -> Option<u16> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::EncapType(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_encap(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::Encap(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_expires(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::Expires(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_pad(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::Pad(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_uid(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::Uid(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_ttl_propagate(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::TtlPropagate(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_ip_proto(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::IpProto(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_sport(&self) -> Option<u16> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::Sport(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_dport(&self) -> Option<u16> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::Dport(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_nh_id(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::NhId(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_flowlabel(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewrouteDoRequest::Flowlabel(val) = attr {
                return Some(val);
            }
        }
        None
    }
}
impl<'a> OpNewrouteDoRequest<'a> {
    pub fn new(buf: &'a [u8]) -> (PushRtmsg, Iterable<'a, OpNewrouteDoRequest<'a>>) {
        let mut header = PushRtmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushRtmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushRtmsg::len()..], buf.as_ptr()),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Dst",
            2u16 => "Src",
            3u16 => "Iif",
            4u16 => "Oif",
            5u16 => "Gateway",
            6u16 => "Priority",
            7u16 => "Prefsrc",
            8u16 => "Metrics",
            9u16 => "Multipath",
            10u16 => "Protoinfo",
            11u16 => "Flow",
            12u16 => "Cacheinfo",
            13u16 => "Session",
            14u16 => "MpAlgo",
            15u16 => "Table",
            16u16 => "Mark",
            17u16 => "MfcStats",
            18u16 => "Via",
            19u16 => "Newdst",
            20u16 => "Pref",
            21u16 => "EncapType",
            22u16 => "Encap",
            23u16 => "Expires",
            24u16 => "Pad",
            25u16 => "Uid",
            26u16 => "TtlPropagate",
            27u16 => "IpProto",
            28u16 => "Sport",
            29u16 => "Dport",
            30u16 => "NhId",
            31u16 => "Flowlabel",
            _ => return None,
        };
        Some(res)
    }
}
impl<'a> Iterator for Iterable<'a, OpNewrouteDoRequest<'a>> {
    type Item = Result<OpNewrouteDoRequest<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpNewrouteDoRequest::Dst({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpNewrouteDoRequest::Src({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpNewrouteDoRequest::Iif({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpNewrouteDoRequest::Oif({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpNewrouteDoRequest::Gateway({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpNewrouteDoRequest::Priority({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => OpNewrouteDoRequest::Prefsrc({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpNewrouteDoRequest::Metrics({
                    let res = Some(Iterable::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => OpNewrouteDoRequest::Multipath({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => OpNewrouteDoRequest::Flow({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => OpNewrouteDoRequest::Cacheinfo({
                    let res = PushRtaCacheinfo::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => OpNewrouteDoRequest::Table({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => OpNewrouteDoRequest::Mark({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => OpNewrouteDoRequest::MfcStats({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => OpNewrouteDoRequest::Via({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => OpNewrouteDoRequest::Newdst({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => OpNewrouteDoRequest::Pref({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => OpNewrouteDoRequest::EncapType({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => OpNewrouteDoRequest::Encap({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => OpNewrouteDoRequest::Expires({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => OpNewrouteDoRequest::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                25u16 => OpNewrouteDoRequest::Uid({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                26u16 => OpNewrouteDoRequest::TtlPropagate({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                27u16 => OpNewrouteDoRequest::IpProto({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                28u16 => OpNewrouteDoRequest::Sport({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                29u16 => OpNewrouteDoRequest::Dport({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                30u16 => OpNewrouteDoRequest::NhId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                31u16 => OpNewrouteDoRequest::Flowlabel({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpNewrouteDoRequest",
            r#type.and_then(|t| OpNewrouteDoRequest::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpNewrouteDoRequest<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpNewrouteDoRequest");
        for attr in self.clone() {
            let attr = match attr {
                Ok(a) => a,
                Err(err) => {
                    fmt.finish()?;
                    f.write_str("Err(")?;
                    err.fmt(f)?;
                    return f.write_str(")");
                }
            };
            match attr {
                OpNewrouteDoRequest::Dst(val) => fmt.field("Dst", &val),
                OpNewrouteDoRequest::Src(val) => fmt.field("Src", &val),
                OpNewrouteDoRequest::Iif(val) => fmt.field("Iif", &val),
                OpNewrouteDoRequest::Oif(val) => fmt.field("Oif", &val),
                OpNewrouteDoRequest::Gateway(val) => fmt.field("Gateway", &val),
                OpNewrouteDoRequest::Priority(val) => fmt.field("Priority", &val),
                OpNewrouteDoRequest::Prefsrc(val) => fmt.field("Prefsrc", &val),
                OpNewrouteDoRequest::Metrics(val) => fmt.field("Metrics", &val),
                OpNewrouteDoRequest::Multipath(val) => fmt.field("Multipath", &val),
                OpNewrouteDoRequest::Flow(val) => fmt.field("Flow", &val),
                OpNewrouteDoRequest::Cacheinfo(val) => fmt.field("Cacheinfo", &val),
                OpNewrouteDoRequest::Table(val) => fmt.field("Table", &val),
                OpNewrouteDoRequest::Mark(val) => fmt.field("Mark", &val),
                OpNewrouteDoRequest::MfcStats(val) => fmt.field("MfcStats", &val),
                OpNewrouteDoRequest::Via(val) => fmt.field("Via", &val),
                OpNewrouteDoRequest::Newdst(val) => fmt.field("Newdst", &val),
                OpNewrouteDoRequest::Pref(val) => fmt.field("Pref", &val),
                OpNewrouteDoRequest::EncapType(val) => fmt.field("EncapType", &val),
                OpNewrouteDoRequest::Encap(val) => fmt.field("Encap", &val),
                OpNewrouteDoRequest::Expires(val) => fmt.field("Expires", &val),
                OpNewrouteDoRequest::Pad(val) => fmt.field("Pad", &val),
                OpNewrouteDoRequest::Uid(val) => fmt.field("Uid", &val),
                OpNewrouteDoRequest::TtlPropagate(val) => fmt.field("TtlPropagate", &val),
                OpNewrouteDoRequest::IpProto(val) => fmt.field("IpProto", &val),
                OpNewrouteDoRequest::Sport(val) => fmt.field("Sport", &val),
                OpNewrouteDoRequest::Dport(val) => fmt.field("Dport", &val),
                OpNewrouteDoRequest::NhId(val) => fmt.field("NhId", &val),
                OpNewrouteDoRequest::Flowlabel(val) => fmt.field("Flowlabel", &val),
            };
        }
        fmt.finish()
    }
}
#[doc = "Create a new route"]
pub struct PushOpNewrouteDoReply<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpNewrouteDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpNewrouteDoReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushRtmsg) -> Self {
        prev.as_rec_mut().extend(header.as_slice());
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
}
impl<Prev: Rec> Drop for PushOpNewrouteDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Create a new route"]
#[doc = "Original name: \"OpNewrouteDoReply\""]
#[derive(Clone)]
pub enum OpNewrouteDoReply {}
impl<'a> Iterable<'a, OpNewrouteDoReply> {}
impl OpNewrouteDoReply {
    pub fn new(buf: &'_ [u8]) -> (PushRtmsg, Iterable<'_, OpNewrouteDoReply>) {
        let mut header = PushRtmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushRtmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushRtmsg::len()..], buf.as_ptr()),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Dst",
            2u16 => "Src",
            3u16 => "Iif",
            4u16 => "Oif",
            5u16 => "Gateway",
            6u16 => "Priority",
            7u16 => "Prefsrc",
            8u16 => "Metrics",
            9u16 => "Multipath",
            10u16 => "Protoinfo",
            11u16 => "Flow",
            12u16 => "Cacheinfo",
            13u16 => "Session",
            14u16 => "MpAlgo",
            15u16 => "Table",
            16u16 => "Mark",
            17u16 => "MfcStats",
            18u16 => "Via",
            19u16 => "Newdst",
            20u16 => "Pref",
            21u16 => "EncapType",
            22u16 => "Encap",
            23u16 => "Expires",
            24u16 => "Pad",
            25u16 => "Uid",
            26u16 => "TtlPropagate",
            27u16 => "IpProto",
            28u16 => "Sport",
            29u16 => "Dport",
            30u16 => "NhId",
            31u16 => "Flowlabel",
            _ => return None,
        };
        Some(res)
    }
}
impl Iterator for Iterable<'_, OpNewrouteDoReply> {
    type Item = Result<OpNewrouteDoReply, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpNewrouteDoReply",
            r#type.and_then(|t| OpNewrouteDoReply::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl std::fmt::Debug for Iterable<'_, OpNewrouteDoReply> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpNewrouteDoReply");
        for attr in self.clone() {
            let attr = match attr {
                Ok(a) => a,
                Err(err) => {
                    fmt.finish()?;
                    f.write_str("Err(")?;
                    err.fmt(f)?;
                    return f.write_str(")");
                }
            };
            match attr {};
        }
        fmt.finish()
    }
}
#[doc = "Delete an existing route"]
pub struct PushOpDelrouteDoRequest<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpDelrouteDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpDelrouteDoRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushRtmsg) -> Self {
        prev.as_rec_mut().extend(header.as_slice());
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_dst(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 1u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_src(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_iif(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_oif(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_gateway(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 5u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_priority(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 6u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_prefsrc(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 7u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn nested_metrics(mut self) -> PushMetrics<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 8u16);
        PushMetrics {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_multipath(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 9u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_flow(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 11u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_cacheinfo(mut self, value: PushRtaCacheinfo) -> Self {
        push_header(self.as_rec_mut(), 12u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_table(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 15u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mark(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 16u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mfc_stats(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 17u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_via(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 18u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_newdst(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 19u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_pref(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 20u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap_type(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 21u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 22u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_expires(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 23u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 24u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_uid(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 25u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ttl_propagate(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 26u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ip_proto(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 27u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sport(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 28u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dport(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 29u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_nh_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 30u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flowlabel(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 31u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpDelrouteDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Delete an existing route"]
#[doc = "Original name: \"OpDelrouteDoRequest\""]
#[derive(Clone)]
pub enum OpDelrouteDoRequest<'a> {
    Dst(&'a [u8]),
    Src(&'a [u8]),
    Iif(u32),
    Oif(u32),
    Gateway(&'a [u8]),
    Priority(u32),
    Prefsrc(&'a [u8]),
    Metrics(Iterable<'a, Metrics<'a>>),
    Multipath(&'a [u8]),
    Flow(u32),
    Cacheinfo(PushRtaCacheinfo),
    Table(u32),
    Mark(u32),
    MfcStats(&'a [u8]),
    Via(&'a [u8]),
    Newdst(&'a [u8]),
    Pref(u8),
    EncapType(u16),
    Encap(&'a [u8]),
    Expires(u32),
    Pad(&'a [u8]),
    Uid(u32),
    TtlPropagate(u8),
    IpProto(u8),
    Sport(u16),
    Dport(u16),
    NhId(u32),
    Flowlabel(u32),
}
impl<'a> Iterable<'a, OpDelrouteDoRequest<'a>> {
    pub fn get_dst(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::Dst(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_src(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::Src(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_iif(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::Iif(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_oif(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::Oif(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_gateway(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::Gateway(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_priority(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::Priority(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_prefsrc(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::Prefsrc(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_metrics(&self) -> Iterable<'a, Metrics<'a>> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::Metrics(val) = attr {
                return val;
            }
        }
        Iterable::new(&[])
    }
    pub fn get_multipath(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::Multipath(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_flow(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::Flow(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_cacheinfo(&self) -> Option<PushRtaCacheinfo> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::Cacheinfo(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_table(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::Table(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_mark(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::Mark(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_mfc_stats(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::MfcStats(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_via(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::Via(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_newdst(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::Newdst(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_pref(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::Pref(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_encap_type(&self) -> Option<u16> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::EncapType(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_encap(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::Encap(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_expires(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::Expires(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_pad(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::Pad(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_uid(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::Uid(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_ttl_propagate(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::TtlPropagate(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_ip_proto(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::IpProto(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_sport(&self) -> Option<u16> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::Sport(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_dport(&self) -> Option<u16> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::Dport(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_nh_id(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::NhId(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_flowlabel(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelrouteDoRequest::Flowlabel(val) = attr {
                return Some(val);
            }
        }
        None
    }
}
impl<'a> OpDelrouteDoRequest<'a> {
    pub fn new(buf: &'a [u8]) -> (PushRtmsg, Iterable<'a, OpDelrouteDoRequest<'a>>) {
        let mut header = PushRtmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushRtmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushRtmsg::len()..], buf.as_ptr()),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Dst",
            2u16 => "Src",
            3u16 => "Iif",
            4u16 => "Oif",
            5u16 => "Gateway",
            6u16 => "Priority",
            7u16 => "Prefsrc",
            8u16 => "Metrics",
            9u16 => "Multipath",
            10u16 => "Protoinfo",
            11u16 => "Flow",
            12u16 => "Cacheinfo",
            13u16 => "Session",
            14u16 => "MpAlgo",
            15u16 => "Table",
            16u16 => "Mark",
            17u16 => "MfcStats",
            18u16 => "Via",
            19u16 => "Newdst",
            20u16 => "Pref",
            21u16 => "EncapType",
            22u16 => "Encap",
            23u16 => "Expires",
            24u16 => "Pad",
            25u16 => "Uid",
            26u16 => "TtlPropagate",
            27u16 => "IpProto",
            28u16 => "Sport",
            29u16 => "Dport",
            30u16 => "NhId",
            31u16 => "Flowlabel",
            _ => return None,
        };
        Some(res)
    }
}
impl<'a> Iterator for Iterable<'a, OpDelrouteDoRequest<'a>> {
    type Item = Result<OpDelrouteDoRequest<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpDelrouteDoRequest::Dst({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpDelrouteDoRequest::Src({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpDelrouteDoRequest::Iif({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpDelrouteDoRequest::Oif({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpDelrouteDoRequest::Gateway({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpDelrouteDoRequest::Priority({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => OpDelrouteDoRequest::Prefsrc({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpDelrouteDoRequest::Metrics({
                    let res = Some(Iterable::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => OpDelrouteDoRequest::Multipath({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => OpDelrouteDoRequest::Flow({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => OpDelrouteDoRequest::Cacheinfo({
                    let res = PushRtaCacheinfo::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => OpDelrouteDoRequest::Table({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => OpDelrouteDoRequest::Mark({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => OpDelrouteDoRequest::MfcStats({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => OpDelrouteDoRequest::Via({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => OpDelrouteDoRequest::Newdst({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => OpDelrouteDoRequest::Pref({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => OpDelrouteDoRequest::EncapType({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => OpDelrouteDoRequest::Encap({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => OpDelrouteDoRequest::Expires({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => OpDelrouteDoRequest::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                25u16 => OpDelrouteDoRequest::Uid({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                26u16 => OpDelrouteDoRequest::TtlPropagate({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                27u16 => OpDelrouteDoRequest::IpProto({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                28u16 => OpDelrouteDoRequest::Sport({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                29u16 => OpDelrouteDoRequest::Dport({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                30u16 => OpDelrouteDoRequest::NhId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                31u16 => OpDelrouteDoRequest::Flowlabel({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpDelrouteDoRequest",
            r#type.and_then(|t| OpDelrouteDoRequest::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpDelrouteDoRequest<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpDelrouteDoRequest");
        for attr in self.clone() {
            let attr = match attr {
                Ok(a) => a,
                Err(err) => {
                    fmt.finish()?;
                    f.write_str("Err(")?;
                    err.fmt(f)?;
                    return f.write_str(")");
                }
            };
            match attr {
                OpDelrouteDoRequest::Dst(val) => fmt.field("Dst", &val),
                OpDelrouteDoRequest::Src(val) => fmt.field("Src", &val),
                OpDelrouteDoRequest::Iif(val) => fmt.field("Iif", &val),
                OpDelrouteDoRequest::Oif(val) => fmt.field("Oif", &val),
                OpDelrouteDoRequest::Gateway(val) => fmt.field("Gateway", &val),
                OpDelrouteDoRequest::Priority(val) => fmt.field("Priority", &val),
                OpDelrouteDoRequest::Prefsrc(val) => fmt.field("Prefsrc", &val),
                OpDelrouteDoRequest::Metrics(val) => fmt.field("Metrics", &val),
                OpDelrouteDoRequest::Multipath(val) => fmt.field("Multipath", &val),
                OpDelrouteDoRequest::Flow(val) => fmt.field("Flow", &val),
                OpDelrouteDoRequest::Cacheinfo(val) => fmt.field("Cacheinfo", &val),
                OpDelrouteDoRequest::Table(val) => fmt.field("Table", &val),
                OpDelrouteDoRequest::Mark(val) => fmt.field("Mark", &val),
                OpDelrouteDoRequest::MfcStats(val) => fmt.field("MfcStats", &val),
                OpDelrouteDoRequest::Via(val) => fmt.field("Via", &val),
                OpDelrouteDoRequest::Newdst(val) => fmt.field("Newdst", &val),
                OpDelrouteDoRequest::Pref(val) => fmt.field("Pref", &val),
                OpDelrouteDoRequest::EncapType(val) => fmt.field("EncapType", &val),
                OpDelrouteDoRequest::Encap(val) => fmt.field("Encap", &val),
                OpDelrouteDoRequest::Expires(val) => fmt.field("Expires", &val),
                OpDelrouteDoRequest::Pad(val) => fmt.field("Pad", &val),
                OpDelrouteDoRequest::Uid(val) => fmt.field("Uid", &val),
                OpDelrouteDoRequest::TtlPropagate(val) => fmt.field("TtlPropagate", &val),
                OpDelrouteDoRequest::IpProto(val) => fmt.field("IpProto", &val),
                OpDelrouteDoRequest::Sport(val) => fmt.field("Sport", &val),
                OpDelrouteDoRequest::Dport(val) => fmt.field("Dport", &val),
                OpDelrouteDoRequest::NhId(val) => fmt.field("NhId", &val),
                OpDelrouteDoRequest::Flowlabel(val) => fmt.field("Flowlabel", &val),
            };
        }
        fmt.finish()
    }
}
#[doc = "Delete an existing route"]
pub struct PushOpDelrouteDoReply<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpDelrouteDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpDelrouteDoReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushRtmsg) -> Self {
        prev.as_rec_mut().extend(header.as_slice());
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
}
impl<Prev: Rec> Drop for PushOpDelrouteDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Delete an existing route"]
#[doc = "Original name: \"OpDelrouteDoReply\""]
#[derive(Clone)]
pub enum OpDelrouteDoReply {}
impl<'a> Iterable<'a, OpDelrouteDoReply> {}
impl OpDelrouteDoReply {
    pub fn new(buf: &'_ [u8]) -> (PushRtmsg, Iterable<'_, OpDelrouteDoReply>) {
        let mut header = PushRtmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushRtmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushRtmsg::len()..], buf.as_ptr()),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Dst",
            2u16 => "Src",
            3u16 => "Iif",
            4u16 => "Oif",
            5u16 => "Gateway",
            6u16 => "Priority",
            7u16 => "Prefsrc",
            8u16 => "Metrics",
            9u16 => "Multipath",
            10u16 => "Protoinfo",
            11u16 => "Flow",
            12u16 => "Cacheinfo",
            13u16 => "Session",
            14u16 => "MpAlgo",
            15u16 => "Table",
            16u16 => "Mark",
            17u16 => "MfcStats",
            18u16 => "Via",
            19u16 => "Newdst",
            20u16 => "Pref",
            21u16 => "EncapType",
            22u16 => "Encap",
            23u16 => "Expires",
            24u16 => "Pad",
            25u16 => "Uid",
            26u16 => "TtlPropagate",
            27u16 => "IpProto",
            28u16 => "Sport",
            29u16 => "Dport",
            30u16 => "NhId",
            31u16 => "Flowlabel",
            _ => return None,
        };
        Some(res)
    }
}
impl Iterator for Iterable<'_, OpDelrouteDoReply> {
    type Item = Result<OpDelrouteDoReply, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpDelrouteDoReply",
            r#type.and_then(|t| OpDelrouteDoReply::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl std::fmt::Debug for Iterable<'_, OpDelrouteDoReply> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpDelrouteDoReply");
        for attr in self.clone() {
            let attr = match attr {
                Ok(a) => a,
                Err(err) => {
                    fmt.finish()?;
                    f.write_str("Err(")?;
                    err.fmt(f)?;
                    return f.write_str(")");
                }
            };
            match attr {};
        }
        fmt.finish()
    }
}
