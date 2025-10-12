#![doc = "Route configuration over rtnetlink."]
#![allow(clippy::all)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]
#![allow(unreachable_code)]
#![allow(unreachable_patterns)]
use crate::builtin::{PushBuiltinBitfield32, PushBuiltinNfgenmsg, PushDummy, PushNlmsghdr};
use crate::{
    consts,
    traits::{NetlinkRequest, Protocol},
    utils::*,
};
pub const PROTONAME: &CStr = c"rt-route";
pub const PROTONUM: u16 = 0u16;
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
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
impl RtmType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Unspec,
            1 => Self::Unicast,
            2 => Self::Local,
            3 => Self::Broadcast,
            4 => Self::Anycast,
            5 => Self::Multicast,
            6 => Self::Blackhole,
            7 => Self::Unreachable,
            8 => Self::Prohibit,
            9 => Self::Throw,
            10 => Self::Nat,
            11 => Self::Xresolve,
            _ => return None,
        })
    }
}
#[derive(Clone)]
pub enum RouteAttrs<'a> {
    Dst(&'a [u8]),
    Src(&'a [u8]),
    Iif(u32),
    Oif(u32),
    Gateway(&'a [u8]),
    Priority(u32),
    Prefsrc(&'a [u8]),
    Metrics(IterableMetrics<'a>),
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
impl<'a> IterableRouteAttrs<'a> {
    pub fn get_dst(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::Dst(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "Dst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_src(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::Src(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "Src",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_iif(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::Iif(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "Iif",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_oif(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::Oif(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "Oif",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_gateway(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::Gateway(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "Gateway",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_priority(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::Priority(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "Priority",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_prefsrc(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::Prefsrc(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "Prefsrc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_metrics(&self) -> Result<IterableMetrics<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::Metrics(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "Metrics",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_multipath(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::Multipath(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "Multipath",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_protoinfo(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::Protoinfo(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "Protoinfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flow(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::Flow(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "Flow",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_cacheinfo(&self) -> Result<PushRtaCacheinfo, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::Cacheinfo(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "Cacheinfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_session(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::Session(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "Session",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mp_algo(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::MpAlgo(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "MpAlgo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_table(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::Table(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "Table",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::Mark(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "Mark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mfc_stats(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::MfcStats(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "MfcStats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_via(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::Via(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "Via",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_newdst(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::Newdst(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "Newdst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pref(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::Pref(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "Pref",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap_type(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::EncapType(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "EncapType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::Encap(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "Encap",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_expires(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::Expires(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "Expires",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::Pad(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_uid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::Uid(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "Uid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ttl_propagate(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::TtlPropagate(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "TtlPropagate",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ip_proto(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::IpProto(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "IpProto",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::Sport(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "Sport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::Dport(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "Dport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nh_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::NhId(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "NhId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flowlabel(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RouteAttrs::Flowlabel(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "Flowlabel",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> RouteAttrs<'a> {
    pub fn new(buf: &'a [u8]) -> IterableRouteAttrs<'a> {
        IterableRouteAttrs::with_loc(buf, buf.as_ptr() as usize)
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
#[derive(Clone, Copy, Default)]
pub struct IterableRouteAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableRouteAttrs<'a> {
    fn with_loc(buf: &'a [u8], orig_loc: usize) -> Self {
        Self {
            buf,
            pos: 0,
            orig_loc,
        }
    }
    pub fn get_buf(&self) -> &'a [u8] {
        self.buf
    }
}
impl<'a> Iterator for IterableRouteAttrs<'a> {
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
                    let res = Some(IterableMetrics::with_loc(next, self.orig_loc));
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
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "RouteAttrs",
            r#type.and_then(|t| RouteAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableRouteAttrs<'_> {
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
impl IterableRouteAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("RouteAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| RouteAttrs::attr_from_type(t)),
            );
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        let mut missing = None;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                RouteAttrs::Dst(val) => {
                    if last_off == offset {
                        stack.push(("Dst", last_off));
                        break;
                    }
                }
                RouteAttrs::Src(val) => {
                    if last_off == offset {
                        stack.push(("Src", last_off));
                        break;
                    }
                }
                RouteAttrs::Iif(val) => {
                    if last_off == offset {
                        stack.push(("Iif", last_off));
                        break;
                    }
                }
                RouteAttrs::Oif(val) => {
                    if last_off == offset {
                        stack.push(("Oif", last_off));
                        break;
                    }
                }
                RouteAttrs::Gateway(val) => {
                    if last_off == offset {
                        stack.push(("Gateway", last_off));
                        break;
                    }
                }
                RouteAttrs::Priority(val) => {
                    if last_off == offset {
                        stack.push(("Priority", last_off));
                        break;
                    }
                }
                RouteAttrs::Prefsrc(val) => {
                    if last_off == offset {
                        stack.push(("Prefsrc", last_off));
                        break;
                    }
                }
                RouteAttrs::Metrics(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                RouteAttrs::Multipath(val) => {
                    if last_off == offset {
                        stack.push(("Multipath", last_off));
                        break;
                    }
                }
                RouteAttrs::Protoinfo(val) => {
                    if last_off == offset {
                        stack.push(("Protoinfo", last_off));
                        break;
                    }
                }
                RouteAttrs::Flow(val) => {
                    if last_off == offset {
                        stack.push(("Flow", last_off));
                        break;
                    }
                }
                RouteAttrs::Cacheinfo(val) => {
                    if last_off == offset {
                        stack.push(("Cacheinfo", last_off));
                        break;
                    }
                }
                RouteAttrs::Session(val) => {
                    if last_off == offset {
                        stack.push(("Session", last_off));
                        break;
                    }
                }
                RouteAttrs::MpAlgo(val) => {
                    if last_off == offset {
                        stack.push(("MpAlgo", last_off));
                        break;
                    }
                }
                RouteAttrs::Table(val) => {
                    if last_off == offset {
                        stack.push(("Table", last_off));
                        break;
                    }
                }
                RouteAttrs::Mark(val) => {
                    if last_off == offset {
                        stack.push(("Mark", last_off));
                        break;
                    }
                }
                RouteAttrs::MfcStats(val) => {
                    if last_off == offset {
                        stack.push(("MfcStats", last_off));
                        break;
                    }
                }
                RouteAttrs::Via(val) => {
                    if last_off == offset {
                        stack.push(("Via", last_off));
                        break;
                    }
                }
                RouteAttrs::Newdst(val) => {
                    if last_off == offset {
                        stack.push(("Newdst", last_off));
                        break;
                    }
                }
                RouteAttrs::Pref(val) => {
                    if last_off == offset {
                        stack.push(("Pref", last_off));
                        break;
                    }
                }
                RouteAttrs::EncapType(val) => {
                    if last_off == offset {
                        stack.push(("EncapType", last_off));
                        break;
                    }
                }
                RouteAttrs::Encap(val) => {
                    if last_off == offset {
                        stack.push(("Encap", last_off));
                        break;
                    }
                }
                RouteAttrs::Expires(val) => {
                    if last_off == offset {
                        stack.push(("Expires", last_off));
                        break;
                    }
                }
                RouteAttrs::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                RouteAttrs::Uid(val) => {
                    if last_off == offset {
                        stack.push(("Uid", last_off));
                        break;
                    }
                }
                RouteAttrs::TtlPropagate(val) => {
                    if last_off == offset {
                        stack.push(("TtlPropagate", last_off));
                        break;
                    }
                }
                RouteAttrs::IpProto(val) => {
                    if last_off == offset {
                        stack.push(("IpProto", last_off));
                        break;
                    }
                }
                RouteAttrs::Sport(val) => {
                    if last_off == offset {
                        stack.push(("Sport", last_off));
                        break;
                    }
                }
                RouteAttrs::Dport(val) => {
                    if last_off == offset {
                        stack.push(("Dport", last_off));
                        break;
                    }
                }
                RouteAttrs::NhId(val) => {
                    if last_off == offset {
                        stack.push(("NhId", last_off));
                        break;
                    }
                }
                RouteAttrs::Flowlabel(val) => {
                    if last_off == offset {
                        stack.push(("Flowlabel", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("RouteAttrs", cur));
        }
        (stack, missing)
    }
}
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
impl<'a> IterableMetrics<'a> {
    pub fn get_lock(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Metrics::Lock(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Metrics",
            "Lock",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mtu(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Metrics::Mtu(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Metrics",
            "Mtu",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_window(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Metrics::Window(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Metrics",
            "Window",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rtt(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Metrics::Rtt(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Metrics",
            "Rtt",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rttvar(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Metrics::Rttvar(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Metrics",
            "Rttvar",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ssthresh(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Metrics::Ssthresh(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Metrics",
            "Ssthresh",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_cwnd(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Metrics::Cwnd(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Metrics",
            "Cwnd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_advmss(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Metrics::Advmss(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Metrics",
            "Advmss",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_reordering(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Metrics::Reordering(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Metrics",
            "Reordering",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_hoplimit(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Metrics::Hoplimit(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Metrics",
            "Hoplimit",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_initcwnd(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Metrics::Initcwnd(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Metrics",
            "Initcwnd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_features(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Metrics::Features(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Metrics",
            "Features",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rto_min(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Metrics::RtoMin(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Metrics",
            "RtoMin",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_initrwnd(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Metrics::Initrwnd(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Metrics",
            "Initrwnd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_quickack(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Metrics::Quickack(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Metrics",
            "Quickack",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_cc_algo(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Metrics::CcAlgo(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Metrics",
            "CcAlgo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fastopen_no_cookie(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Metrics::FastopenNoCookie(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Metrics",
            "FastopenNoCookie",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> Metrics<'a> {
    pub fn new(buf: &'a [u8]) -> IterableMetrics<'a> {
        IterableMetrics::with_loc(buf, buf.as_ptr() as usize)
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
#[derive(Clone, Copy, Default)]
pub struct IterableMetrics<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableMetrics<'a> {
    fn with_loc(buf: &'a [u8], orig_loc: usize) -> Self {
        Self {
            buf,
            pos: 0,
            orig_loc,
        }
    }
    pub fn get_buf(&self) -> &'a [u8] {
        self.buf
    }
}
impl<'a> Iterator for IterableMetrics<'a> {
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
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Metrics",
            r#type.and_then(|t| Metrics::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableMetrics<'_> {
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
impl IterableMetrics<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("Metrics", offset));
            return (stack, missing_type.and_then(|t| Metrics::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Metrics::Lock(val) => {
                    if last_off == offset {
                        stack.push(("Lock", last_off));
                        break;
                    }
                }
                Metrics::Mtu(val) => {
                    if last_off == offset {
                        stack.push(("Mtu", last_off));
                        break;
                    }
                }
                Metrics::Window(val) => {
                    if last_off == offset {
                        stack.push(("Window", last_off));
                        break;
                    }
                }
                Metrics::Rtt(val) => {
                    if last_off == offset {
                        stack.push(("Rtt", last_off));
                        break;
                    }
                }
                Metrics::Rttvar(val) => {
                    if last_off == offset {
                        stack.push(("Rttvar", last_off));
                        break;
                    }
                }
                Metrics::Ssthresh(val) => {
                    if last_off == offset {
                        stack.push(("Ssthresh", last_off));
                        break;
                    }
                }
                Metrics::Cwnd(val) => {
                    if last_off == offset {
                        stack.push(("Cwnd", last_off));
                        break;
                    }
                }
                Metrics::Advmss(val) => {
                    if last_off == offset {
                        stack.push(("Advmss", last_off));
                        break;
                    }
                }
                Metrics::Reordering(val) => {
                    if last_off == offset {
                        stack.push(("Reordering", last_off));
                        break;
                    }
                }
                Metrics::Hoplimit(val) => {
                    if last_off == offset {
                        stack.push(("Hoplimit", last_off));
                        break;
                    }
                }
                Metrics::Initcwnd(val) => {
                    if last_off == offset {
                        stack.push(("Initcwnd", last_off));
                        break;
                    }
                }
                Metrics::Features(val) => {
                    if last_off == offset {
                        stack.push(("Features", last_off));
                        break;
                    }
                }
                Metrics::RtoMin(val) => {
                    if last_off == offset {
                        stack.push(("RtoMin", last_off));
                        break;
                    }
                }
                Metrics::Initrwnd(val) => {
                    if last_off == offset {
                        stack.push(("Initrwnd", last_off));
                        break;
                    }
                }
                Metrics::Quickack(val) => {
                    if last_off == offset {
                        stack.push(("Quickack", last_off));
                        break;
                    }
                }
                Metrics::CcAlgo(val) => {
                    if last_off == offset {
                        stack.push(("CcAlgo", last_off));
                        break;
                    }
                }
                Metrics::FastopenNoCookie(val) => {
                    if last_off == offset {
                        stack.push(("FastopenNoCookie", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Metrics", cur));
        }
        (stack, None)
    }
}
pub struct PushRouteAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
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
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
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
    pub fn push_cc_algo_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 16u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
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
#[derive(Clone)]
pub struct PushRtmsg {
    pub(crate) buf: [u8; 12usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for PushRtmsg {
    fn default() -> Self {
        Self {
            buf: [0u8; 12usize],
        }
    }
}
impl PushRtmsg {
    #[doc = "Create zero-initialized struct"]
    pub fn new() -> Self {
        Default::default()
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
#[derive(Clone)]
pub struct PushRtaCacheinfo {
    pub(crate) buf: [u8; 20usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for PushRtaCacheinfo {
    fn default() -> Self {
        Self {
            buf: [0u8; 20usize],
        }
    }
}
impl PushRtaCacheinfo {
    #[doc = "Create zero-initialized struct"]
    pub fn new() -> Self {
        Default::default()
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
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetrouteDumpRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetrouteDumpRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushRtmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushRtmsg) {
        prev.as_rec_mut().extend(header.as_slice());
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
#[derive(Clone)]
pub enum OpGetrouteDumpRequest {}
impl<'a> IterableOpGetrouteDumpRequest<'a> {}
impl OpGetrouteDumpRequest {
    pub fn new(buf: &'_ [u8]) -> (PushRtmsg, IterableOpGetrouteDumpRequest<'_>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushRtmsg::len()));
        (
            PushRtmsg::new_from_slice(header).unwrap_or_default(),
            IterableOpGetrouteDumpRequest::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        RouteAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetrouteDumpRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetrouteDumpRequest<'a> {
    fn with_loc(buf: &'a [u8], orig_loc: usize) -> Self {
        Self {
            buf,
            pos: 0,
            orig_loc,
        }
    }
    pub fn get_buf(&self) -> &'a [u8] {
        self.buf
    }
}
impl<'a> Iterator for IterableOpGetrouteDumpRequest<'a> {
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
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpGetrouteDumpRequest",
            r#type.and_then(|t| OpGetrouteDumpRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpGetrouteDumpRequest<'_> {
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
impl IterableOpGetrouteDumpRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushRtmsg::len() {
            stack.push(("OpGetrouteDumpRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetrouteDumpRequest::attr_from_type(t)),
            );
        }
        (stack, None)
    }
}
#[doc = "Dump route information."]
pub struct PushOpGetrouteDumpReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetrouteDumpReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetrouteDumpReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushRtmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushRtmsg) {
        prev.as_rec_mut().extend(header.as_slice());
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
#[derive(Clone)]
pub enum OpGetrouteDumpReply<'a> {
    Dst(&'a [u8]),
    Src(&'a [u8]),
    Iif(u32),
    Oif(u32),
    Gateway(&'a [u8]),
    Priority(u32),
    Prefsrc(&'a [u8]),
    Metrics(IterableMetrics<'a>),
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
impl<'a> IterableOpGetrouteDumpReply<'a> {
    pub fn get_dst(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::Dst(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "Dst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_src(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::Src(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "Src",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_iif(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::Iif(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "Iif",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_oif(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::Oif(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "Oif",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_gateway(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::Gateway(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "Gateway",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_priority(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::Priority(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "Priority",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_prefsrc(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::Prefsrc(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "Prefsrc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_metrics(&self) -> Result<IterableMetrics<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::Metrics(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "Metrics",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_multipath(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::Multipath(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "Multipath",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flow(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::Flow(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "Flow",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_cacheinfo(&self) -> Result<PushRtaCacheinfo, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::Cacheinfo(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "Cacheinfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_table(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::Table(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "Table",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::Mark(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "Mark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mfc_stats(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::MfcStats(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "MfcStats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_via(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::Via(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "Via",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_newdst(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::Newdst(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "Newdst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pref(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::Pref(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "Pref",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap_type(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::EncapType(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "EncapType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::Encap(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "Encap",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_expires(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::Expires(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "Expires",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::Pad(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_uid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::Uid(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "Uid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ttl_propagate(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::TtlPropagate(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "TtlPropagate",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ip_proto(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::IpProto(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "IpProto",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::Sport(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "Sport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::Dport(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "Dport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nh_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::NhId(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "NhId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flowlabel(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDumpReply::Flowlabel(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDumpReply",
            "Flowlabel",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> OpGetrouteDumpReply<'a> {
    pub fn new(buf: &'a [u8]) -> (PushRtmsg, IterableOpGetrouteDumpReply<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushRtmsg::len()));
        (
            PushRtmsg::new_from_slice(header).unwrap_or_default(),
            IterableOpGetrouteDumpReply::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        RouteAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetrouteDumpReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetrouteDumpReply<'a> {
    fn with_loc(buf: &'a [u8], orig_loc: usize) -> Self {
        Self {
            buf,
            pos: 0,
            orig_loc,
        }
    }
    pub fn get_buf(&self) -> &'a [u8] {
        self.buf
    }
}
impl<'a> Iterator for IterableOpGetrouteDumpReply<'a> {
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
                    let res = Some(IterableMetrics::with_loc(next, self.orig_loc));
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
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpGetrouteDumpReply",
            r#type.and_then(|t| OpGetrouteDumpReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpGetrouteDumpReply<'_> {
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
impl IterableOpGetrouteDumpReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushRtmsg::len() {
            stack.push(("OpGetrouteDumpReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetrouteDumpReply::attr_from_type(t)),
            );
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        let mut missing = None;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                OpGetrouteDumpReply::Dst(val) => {
                    if last_off == offset {
                        stack.push(("Dst", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::Src(val) => {
                    if last_off == offset {
                        stack.push(("Src", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::Iif(val) => {
                    if last_off == offset {
                        stack.push(("Iif", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::Oif(val) => {
                    if last_off == offset {
                        stack.push(("Oif", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::Gateway(val) => {
                    if last_off == offset {
                        stack.push(("Gateway", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::Priority(val) => {
                    if last_off == offset {
                        stack.push(("Priority", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::Prefsrc(val) => {
                    if last_off == offset {
                        stack.push(("Prefsrc", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::Metrics(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetrouteDumpReply::Multipath(val) => {
                    if last_off == offset {
                        stack.push(("Multipath", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::Flow(val) => {
                    if last_off == offset {
                        stack.push(("Flow", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::Cacheinfo(val) => {
                    if last_off == offset {
                        stack.push(("Cacheinfo", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::Table(val) => {
                    if last_off == offset {
                        stack.push(("Table", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::Mark(val) => {
                    if last_off == offset {
                        stack.push(("Mark", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::MfcStats(val) => {
                    if last_off == offset {
                        stack.push(("MfcStats", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::Via(val) => {
                    if last_off == offset {
                        stack.push(("Via", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::Newdst(val) => {
                    if last_off == offset {
                        stack.push(("Newdst", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::Pref(val) => {
                    if last_off == offset {
                        stack.push(("Pref", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::EncapType(val) => {
                    if last_off == offset {
                        stack.push(("EncapType", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::Encap(val) => {
                    if last_off == offset {
                        stack.push(("Encap", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::Expires(val) => {
                    if last_off == offset {
                        stack.push(("Expires", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::Uid(val) => {
                    if last_off == offset {
                        stack.push(("Uid", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::TtlPropagate(val) => {
                    if last_off == offset {
                        stack.push(("TtlPropagate", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::IpProto(val) => {
                    if last_off == offset {
                        stack.push(("IpProto", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::Sport(val) => {
                    if last_off == offset {
                        stack.push(("Sport", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::Dport(val) => {
                    if last_off == offset {
                        stack.push(("Dport", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::NhId(val) => {
                    if last_off == offset {
                        stack.push(("NhId", last_off));
                        break;
                    }
                }
                OpGetrouteDumpReply::Flowlabel(val) => {
                    if last_off == offset {
                        stack.push(("Flowlabel", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetrouteDumpReply", cur));
        }
        (stack, missing)
    }
}
#[derive(Debug)]
pub struct RequestOpGetrouteDumpRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpGetrouteDumpRequest<'r> {
    pub fn new(mut request: Request<'r>, header: &PushRtmsg) -> Self {
        PushOpGetrouteDumpRequest::write_header(&mut request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode(&mut self) -> PushOpGetrouteDumpRequest<&mut Vec<u8>> {
        PushOpGetrouteDumpRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpGetrouteDumpRequest<RequestBuf<'r>> {
        PushOpGetrouteDumpRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpGetrouteDumpRequest<'_> {
    type ReplyType<'buf> = (PushRtmsg, IterableOpGetrouteDumpReply<'buf>);
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 26u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpGetrouteDumpReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpGetrouteDumpRequest::new(buf)
            .1
            .lookup_attr(offset, missing_type)
    }
}
#[doc = "Dump route information."]
pub struct PushOpGetrouteDoRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetrouteDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetrouteDoRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushRtmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushRtmsg) {
        prev.as_rec_mut().extend(header.as_slice());
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
impl<'a> IterableOpGetrouteDoRequest<'a> {
    pub fn get_dst(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoRequest::Dst(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoRequest",
            "Dst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_src(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoRequest::Src(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoRequest",
            "Src",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_iif(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoRequest::Iif(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoRequest",
            "Iif",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_oif(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoRequest::Oif(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoRequest",
            "Oif",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoRequest::Mark(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoRequest",
            "Mark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_uid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoRequest::Uid(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoRequest",
            "Uid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ip_proto(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoRequest::IpProto(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoRequest",
            "IpProto",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoRequest::Sport(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoRequest",
            "Sport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoRequest::Dport(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoRequest",
            "Dport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flowlabel(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoRequest::Flowlabel(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoRequest",
            "Flowlabel",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> OpGetrouteDoRequest<'a> {
    pub fn new(buf: &'a [u8]) -> (PushRtmsg, IterableOpGetrouteDoRequest<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushRtmsg::len()));
        (
            PushRtmsg::new_from_slice(header).unwrap_or_default(),
            IterableOpGetrouteDoRequest::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        RouteAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetrouteDoRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetrouteDoRequest<'a> {
    fn with_loc(buf: &'a [u8], orig_loc: usize) -> Self {
        Self {
            buf,
            pos: 0,
            orig_loc,
        }
    }
    pub fn get_buf(&self) -> &'a [u8] {
        self.buf
    }
}
impl<'a> Iterator for IterableOpGetrouteDoRequest<'a> {
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
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpGetrouteDoRequest",
            r#type.and_then(|t| OpGetrouteDoRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpGetrouteDoRequest<'_> {
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
impl IterableOpGetrouteDoRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushRtmsg::len() {
            stack.push(("OpGetrouteDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetrouteDoRequest::attr_from_type(t)),
            );
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                OpGetrouteDoRequest::Dst(val) => {
                    if last_off == offset {
                        stack.push(("Dst", last_off));
                        break;
                    }
                }
                OpGetrouteDoRequest::Src(val) => {
                    if last_off == offset {
                        stack.push(("Src", last_off));
                        break;
                    }
                }
                OpGetrouteDoRequest::Iif(val) => {
                    if last_off == offset {
                        stack.push(("Iif", last_off));
                        break;
                    }
                }
                OpGetrouteDoRequest::Oif(val) => {
                    if last_off == offset {
                        stack.push(("Oif", last_off));
                        break;
                    }
                }
                OpGetrouteDoRequest::Mark(val) => {
                    if last_off == offset {
                        stack.push(("Mark", last_off));
                        break;
                    }
                }
                OpGetrouteDoRequest::Uid(val) => {
                    if last_off == offset {
                        stack.push(("Uid", last_off));
                        break;
                    }
                }
                OpGetrouteDoRequest::IpProto(val) => {
                    if last_off == offset {
                        stack.push(("IpProto", last_off));
                        break;
                    }
                }
                OpGetrouteDoRequest::Sport(val) => {
                    if last_off == offset {
                        stack.push(("Sport", last_off));
                        break;
                    }
                }
                OpGetrouteDoRequest::Dport(val) => {
                    if last_off == offset {
                        stack.push(("Dport", last_off));
                        break;
                    }
                }
                OpGetrouteDoRequest::Flowlabel(val) => {
                    if last_off == offset {
                        stack.push(("Flowlabel", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetrouteDoRequest", cur));
        }
        (stack, None)
    }
}
#[doc = "Dump route information."]
pub struct PushOpGetrouteDoReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetrouteDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetrouteDoReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushRtmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushRtmsg) {
        prev.as_rec_mut().extend(header.as_slice());
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
#[derive(Clone)]
pub enum OpGetrouteDoReply<'a> {
    Dst(&'a [u8]),
    Src(&'a [u8]),
    Iif(u32),
    Oif(u32),
    Gateway(&'a [u8]),
    Priority(u32),
    Prefsrc(&'a [u8]),
    Metrics(IterableMetrics<'a>),
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
impl<'a> IterableOpGetrouteDoReply<'a> {
    pub fn get_dst(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::Dst(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "Dst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_src(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::Src(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "Src",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_iif(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::Iif(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "Iif",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_oif(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::Oif(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "Oif",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_gateway(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::Gateway(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "Gateway",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_priority(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::Priority(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "Priority",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_prefsrc(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::Prefsrc(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "Prefsrc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_metrics(&self) -> Result<IterableMetrics<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::Metrics(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "Metrics",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_multipath(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::Multipath(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "Multipath",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flow(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::Flow(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "Flow",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_cacheinfo(&self) -> Result<PushRtaCacheinfo, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::Cacheinfo(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "Cacheinfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_table(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::Table(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "Table",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::Mark(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "Mark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mfc_stats(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::MfcStats(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "MfcStats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_via(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::Via(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "Via",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_newdst(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::Newdst(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "Newdst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pref(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::Pref(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "Pref",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap_type(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::EncapType(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "EncapType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::Encap(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "Encap",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_expires(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::Expires(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "Expires",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::Pad(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_uid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::Uid(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "Uid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ttl_propagate(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::TtlPropagate(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "TtlPropagate",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ip_proto(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::IpProto(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "IpProto",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::Sport(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "Sport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::Dport(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "Dport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nh_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::NhId(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "NhId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flowlabel(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetrouteDoReply::Flowlabel(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetrouteDoReply",
            "Flowlabel",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> OpGetrouteDoReply<'a> {
    pub fn new(buf: &'a [u8]) -> (PushRtmsg, IterableOpGetrouteDoReply<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushRtmsg::len()));
        (
            PushRtmsg::new_from_slice(header).unwrap_or_default(),
            IterableOpGetrouteDoReply::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        RouteAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetrouteDoReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetrouteDoReply<'a> {
    fn with_loc(buf: &'a [u8], orig_loc: usize) -> Self {
        Self {
            buf,
            pos: 0,
            orig_loc,
        }
    }
    pub fn get_buf(&self) -> &'a [u8] {
        self.buf
    }
}
impl<'a> Iterator for IterableOpGetrouteDoReply<'a> {
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
                    let res = Some(IterableMetrics::with_loc(next, self.orig_loc));
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
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpGetrouteDoReply",
            r#type.and_then(|t| OpGetrouteDoReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpGetrouteDoReply<'_> {
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
impl IterableOpGetrouteDoReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushRtmsg::len() {
            stack.push(("OpGetrouteDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetrouteDoReply::attr_from_type(t)),
            );
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        let mut missing = None;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                OpGetrouteDoReply::Dst(val) => {
                    if last_off == offset {
                        stack.push(("Dst", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::Src(val) => {
                    if last_off == offset {
                        stack.push(("Src", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::Iif(val) => {
                    if last_off == offset {
                        stack.push(("Iif", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::Oif(val) => {
                    if last_off == offset {
                        stack.push(("Oif", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::Gateway(val) => {
                    if last_off == offset {
                        stack.push(("Gateway", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::Priority(val) => {
                    if last_off == offset {
                        stack.push(("Priority", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::Prefsrc(val) => {
                    if last_off == offset {
                        stack.push(("Prefsrc", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::Metrics(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetrouteDoReply::Multipath(val) => {
                    if last_off == offset {
                        stack.push(("Multipath", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::Flow(val) => {
                    if last_off == offset {
                        stack.push(("Flow", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::Cacheinfo(val) => {
                    if last_off == offset {
                        stack.push(("Cacheinfo", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::Table(val) => {
                    if last_off == offset {
                        stack.push(("Table", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::Mark(val) => {
                    if last_off == offset {
                        stack.push(("Mark", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::MfcStats(val) => {
                    if last_off == offset {
                        stack.push(("MfcStats", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::Via(val) => {
                    if last_off == offset {
                        stack.push(("Via", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::Newdst(val) => {
                    if last_off == offset {
                        stack.push(("Newdst", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::Pref(val) => {
                    if last_off == offset {
                        stack.push(("Pref", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::EncapType(val) => {
                    if last_off == offset {
                        stack.push(("EncapType", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::Encap(val) => {
                    if last_off == offset {
                        stack.push(("Encap", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::Expires(val) => {
                    if last_off == offset {
                        stack.push(("Expires", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::Uid(val) => {
                    if last_off == offset {
                        stack.push(("Uid", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::TtlPropagate(val) => {
                    if last_off == offset {
                        stack.push(("TtlPropagate", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::IpProto(val) => {
                    if last_off == offset {
                        stack.push(("IpProto", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::Sport(val) => {
                    if last_off == offset {
                        stack.push(("Sport", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::Dport(val) => {
                    if last_off == offset {
                        stack.push(("Dport", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::NhId(val) => {
                    if last_off == offset {
                        stack.push(("NhId", last_off));
                        break;
                    }
                }
                OpGetrouteDoReply::Flowlabel(val) => {
                    if last_off == offset {
                        stack.push(("Flowlabel", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetrouteDoReply", cur));
        }
        (stack, missing)
    }
}
#[derive(Debug)]
pub struct RequestOpGetrouteDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpGetrouteDoRequest<'r> {
    pub fn new(mut request: Request<'r>, header: &PushRtmsg) -> Self {
        PushOpGetrouteDoRequest::write_header(&mut request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpGetrouteDoRequest<&mut Vec<u8>> {
        PushOpGetrouteDoRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpGetrouteDoRequest<RequestBuf<'r>> {
        PushOpGetrouteDoRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpGetrouteDoRequest<'_> {
    type ReplyType<'buf> = (PushRtmsg, IterableOpGetrouteDoReply<'buf>);
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 26u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpGetrouteDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpGetrouteDoRequest::new(buf)
            .1
            .lookup_attr(offset, missing_type)
    }
}
#[doc = "Create a new route"]
pub struct PushOpNewrouteDoRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpNewrouteDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpNewrouteDoRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushRtmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushRtmsg) {
        prev.as_rec_mut().extend(header.as_slice());
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
#[derive(Clone)]
pub enum OpNewrouteDoRequest<'a> {
    Dst(&'a [u8]),
    Src(&'a [u8]),
    Iif(u32),
    Oif(u32),
    Gateway(&'a [u8]),
    Priority(u32),
    Prefsrc(&'a [u8]),
    Metrics(IterableMetrics<'a>),
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
impl<'a> IterableOpNewrouteDoRequest<'a> {
    pub fn get_dst(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::Dst(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "Dst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_src(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::Src(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "Src",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_iif(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::Iif(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "Iif",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_oif(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::Oif(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "Oif",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_gateway(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::Gateway(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "Gateway",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_priority(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::Priority(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "Priority",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_prefsrc(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::Prefsrc(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "Prefsrc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_metrics(&self) -> Result<IterableMetrics<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::Metrics(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "Metrics",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_multipath(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::Multipath(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "Multipath",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flow(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::Flow(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "Flow",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_cacheinfo(&self) -> Result<PushRtaCacheinfo, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::Cacheinfo(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "Cacheinfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_table(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::Table(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "Table",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::Mark(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "Mark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mfc_stats(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::MfcStats(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "MfcStats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_via(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::Via(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "Via",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_newdst(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::Newdst(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "Newdst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pref(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::Pref(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "Pref",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap_type(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::EncapType(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "EncapType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::Encap(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "Encap",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_expires(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::Expires(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "Expires",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::Pad(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_uid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::Uid(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "Uid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ttl_propagate(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::TtlPropagate(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "TtlPropagate",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ip_proto(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::IpProto(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "IpProto",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::Sport(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "Sport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::Dport(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "Dport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nh_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::NhId(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "NhId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flowlabel(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewrouteDoRequest::Flowlabel(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewrouteDoRequest",
            "Flowlabel",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> OpNewrouteDoRequest<'a> {
    pub fn new(buf: &'a [u8]) -> (PushRtmsg, IterableOpNewrouteDoRequest<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushRtmsg::len()));
        (
            PushRtmsg::new_from_slice(header).unwrap_or_default(),
            IterableOpNewrouteDoRequest::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        RouteAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpNewrouteDoRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpNewrouteDoRequest<'a> {
    fn with_loc(buf: &'a [u8], orig_loc: usize) -> Self {
        Self {
            buf,
            pos: 0,
            orig_loc,
        }
    }
    pub fn get_buf(&self) -> &'a [u8] {
        self.buf
    }
}
impl<'a> Iterator for IterableOpNewrouteDoRequest<'a> {
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
                    let res = Some(IterableMetrics::with_loc(next, self.orig_loc));
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
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpNewrouteDoRequest",
            r#type.and_then(|t| OpNewrouteDoRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpNewrouteDoRequest<'_> {
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
impl IterableOpNewrouteDoRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushRtmsg::len() {
            stack.push(("OpNewrouteDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpNewrouteDoRequest::attr_from_type(t)),
            );
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        let mut missing = None;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                OpNewrouteDoRequest::Dst(val) => {
                    if last_off == offset {
                        stack.push(("Dst", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::Src(val) => {
                    if last_off == offset {
                        stack.push(("Src", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::Iif(val) => {
                    if last_off == offset {
                        stack.push(("Iif", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::Oif(val) => {
                    if last_off == offset {
                        stack.push(("Oif", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::Gateway(val) => {
                    if last_off == offset {
                        stack.push(("Gateway", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::Priority(val) => {
                    if last_off == offset {
                        stack.push(("Priority", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::Prefsrc(val) => {
                    if last_off == offset {
                        stack.push(("Prefsrc", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::Metrics(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpNewrouteDoRequest::Multipath(val) => {
                    if last_off == offset {
                        stack.push(("Multipath", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::Flow(val) => {
                    if last_off == offset {
                        stack.push(("Flow", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::Cacheinfo(val) => {
                    if last_off == offset {
                        stack.push(("Cacheinfo", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::Table(val) => {
                    if last_off == offset {
                        stack.push(("Table", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::Mark(val) => {
                    if last_off == offset {
                        stack.push(("Mark", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::MfcStats(val) => {
                    if last_off == offset {
                        stack.push(("MfcStats", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::Via(val) => {
                    if last_off == offset {
                        stack.push(("Via", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::Newdst(val) => {
                    if last_off == offset {
                        stack.push(("Newdst", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::Pref(val) => {
                    if last_off == offset {
                        stack.push(("Pref", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::EncapType(val) => {
                    if last_off == offset {
                        stack.push(("EncapType", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::Encap(val) => {
                    if last_off == offset {
                        stack.push(("Encap", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::Expires(val) => {
                    if last_off == offset {
                        stack.push(("Expires", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::Uid(val) => {
                    if last_off == offset {
                        stack.push(("Uid", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::TtlPropagate(val) => {
                    if last_off == offset {
                        stack.push(("TtlPropagate", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::IpProto(val) => {
                    if last_off == offset {
                        stack.push(("IpProto", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::Sport(val) => {
                    if last_off == offset {
                        stack.push(("Sport", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::Dport(val) => {
                    if last_off == offset {
                        stack.push(("Dport", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::NhId(val) => {
                    if last_off == offset {
                        stack.push(("NhId", last_off));
                        break;
                    }
                }
                OpNewrouteDoRequest::Flowlabel(val) => {
                    if last_off == offset {
                        stack.push(("Flowlabel", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpNewrouteDoRequest", cur));
        }
        (stack, missing)
    }
}
#[doc = "Create a new route"]
pub struct PushOpNewrouteDoReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpNewrouteDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpNewrouteDoReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushRtmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushRtmsg) {
        prev.as_rec_mut().extend(header.as_slice());
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
#[derive(Clone)]
pub enum OpNewrouteDoReply {}
impl<'a> IterableOpNewrouteDoReply<'a> {}
impl OpNewrouteDoReply {
    pub fn new(buf: &'_ [u8]) -> (PushRtmsg, IterableOpNewrouteDoReply<'_>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushRtmsg::len()));
        (
            PushRtmsg::new_from_slice(header).unwrap_or_default(),
            IterableOpNewrouteDoReply::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        RouteAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpNewrouteDoReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpNewrouteDoReply<'a> {
    fn with_loc(buf: &'a [u8], orig_loc: usize) -> Self {
        Self {
            buf,
            pos: 0,
            orig_loc,
        }
    }
    pub fn get_buf(&self) -> &'a [u8] {
        self.buf
    }
}
impl<'a> Iterator for IterableOpNewrouteDoReply<'a> {
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
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpNewrouteDoReply",
            r#type.and_then(|t| OpNewrouteDoReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpNewrouteDoReply<'_> {
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
impl IterableOpNewrouteDoReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushRtmsg::len() {
            stack.push(("OpNewrouteDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpNewrouteDoReply::attr_from_type(t)),
            );
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpNewrouteDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpNewrouteDoRequest<'r> {
    pub fn new(mut request: Request<'r>, header: &PushRtmsg) -> Self {
        PushOpNewrouteDoRequest::write_header(&mut request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpNewrouteDoRequest<&mut Vec<u8>> {
        PushOpNewrouteDoRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpNewrouteDoRequest<RequestBuf<'r>> {
        PushOpNewrouteDoRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpNewrouteDoRequest<'_> {
    type ReplyType<'buf> = (PushRtmsg, IterableOpNewrouteDoReply<'buf>);
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 24u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpNewrouteDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpNewrouteDoRequest::new(buf)
            .1
            .lookup_attr(offset, missing_type)
    }
}
#[doc = "Delete an existing route"]
pub struct PushOpDelrouteDoRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpDelrouteDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpDelrouteDoRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushRtmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushRtmsg) {
        prev.as_rec_mut().extend(header.as_slice());
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
#[derive(Clone)]
pub enum OpDelrouteDoRequest<'a> {
    Dst(&'a [u8]),
    Src(&'a [u8]),
    Iif(u32),
    Oif(u32),
    Gateway(&'a [u8]),
    Priority(u32),
    Prefsrc(&'a [u8]),
    Metrics(IterableMetrics<'a>),
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
impl<'a> IterableOpDelrouteDoRequest<'a> {
    pub fn get_dst(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::Dst(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "Dst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_src(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::Src(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "Src",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_iif(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::Iif(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "Iif",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_oif(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::Oif(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "Oif",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_gateway(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::Gateway(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "Gateway",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_priority(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::Priority(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "Priority",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_prefsrc(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::Prefsrc(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "Prefsrc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_metrics(&self) -> Result<IterableMetrics<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::Metrics(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "Metrics",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_multipath(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::Multipath(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "Multipath",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flow(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::Flow(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "Flow",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_cacheinfo(&self) -> Result<PushRtaCacheinfo, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::Cacheinfo(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "Cacheinfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_table(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::Table(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "Table",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::Mark(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "Mark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mfc_stats(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::MfcStats(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "MfcStats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_via(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::Via(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "Via",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_newdst(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::Newdst(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "Newdst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pref(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::Pref(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "Pref",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap_type(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::EncapType(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "EncapType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::Encap(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "Encap",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_expires(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::Expires(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "Expires",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::Pad(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_uid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::Uid(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "Uid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ttl_propagate(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::TtlPropagate(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "TtlPropagate",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ip_proto(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::IpProto(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "IpProto",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::Sport(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "Sport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::Dport(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "Dport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nh_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::NhId(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "NhId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flowlabel(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelrouteDoRequest::Flowlabel(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelrouteDoRequest",
            "Flowlabel",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> OpDelrouteDoRequest<'a> {
    pub fn new(buf: &'a [u8]) -> (PushRtmsg, IterableOpDelrouteDoRequest<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushRtmsg::len()));
        (
            PushRtmsg::new_from_slice(header).unwrap_or_default(),
            IterableOpDelrouteDoRequest::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        RouteAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpDelrouteDoRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpDelrouteDoRequest<'a> {
    fn with_loc(buf: &'a [u8], orig_loc: usize) -> Self {
        Self {
            buf,
            pos: 0,
            orig_loc,
        }
    }
    pub fn get_buf(&self) -> &'a [u8] {
        self.buf
    }
}
impl<'a> Iterator for IterableOpDelrouteDoRequest<'a> {
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
                    let res = Some(IterableMetrics::with_loc(next, self.orig_loc));
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
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpDelrouteDoRequest",
            r#type.and_then(|t| OpDelrouteDoRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpDelrouteDoRequest<'_> {
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
impl IterableOpDelrouteDoRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushRtmsg::len() {
            stack.push(("OpDelrouteDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpDelrouteDoRequest::attr_from_type(t)),
            );
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        let mut missing = None;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                OpDelrouteDoRequest::Dst(val) => {
                    if last_off == offset {
                        stack.push(("Dst", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::Src(val) => {
                    if last_off == offset {
                        stack.push(("Src", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::Iif(val) => {
                    if last_off == offset {
                        stack.push(("Iif", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::Oif(val) => {
                    if last_off == offset {
                        stack.push(("Oif", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::Gateway(val) => {
                    if last_off == offset {
                        stack.push(("Gateway", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::Priority(val) => {
                    if last_off == offset {
                        stack.push(("Priority", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::Prefsrc(val) => {
                    if last_off == offset {
                        stack.push(("Prefsrc", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::Metrics(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpDelrouteDoRequest::Multipath(val) => {
                    if last_off == offset {
                        stack.push(("Multipath", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::Flow(val) => {
                    if last_off == offset {
                        stack.push(("Flow", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::Cacheinfo(val) => {
                    if last_off == offset {
                        stack.push(("Cacheinfo", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::Table(val) => {
                    if last_off == offset {
                        stack.push(("Table", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::Mark(val) => {
                    if last_off == offset {
                        stack.push(("Mark", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::MfcStats(val) => {
                    if last_off == offset {
                        stack.push(("MfcStats", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::Via(val) => {
                    if last_off == offset {
                        stack.push(("Via", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::Newdst(val) => {
                    if last_off == offset {
                        stack.push(("Newdst", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::Pref(val) => {
                    if last_off == offset {
                        stack.push(("Pref", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::EncapType(val) => {
                    if last_off == offset {
                        stack.push(("EncapType", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::Encap(val) => {
                    if last_off == offset {
                        stack.push(("Encap", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::Expires(val) => {
                    if last_off == offset {
                        stack.push(("Expires", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::Uid(val) => {
                    if last_off == offset {
                        stack.push(("Uid", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::TtlPropagate(val) => {
                    if last_off == offset {
                        stack.push(("TtlPropagate", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::IpProto(val) => {
                    if last_off == offset {
                        stack.push(("IpProto", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::Sport(val) => {
                    if last_off == offset {
                        stack.push(("Sport", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::Dport(val) => {
                    if last_off == offset {
                        stack.push(("Dport", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::NhId(val) => {
                    if last_off == offset {
                        stack.push(("NhId", last_off));
                        break;
                    }
                }
                OpDelrouteDoRequest::Flowlabel(val) => {
                    if last_off == offset {
                        stack.push(("Flowlabel", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpDelrouteDoRequest", cur));
        }
        (stack, missing)
    }
}
#[doc = "Delete an existing route"]
pub struct PushOpDelrouteDoReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpDelrouteDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpDelrouteDoReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushRtmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushRtmsg) {
        prev.as_rec_mut().extend(header.as_slice());
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
#[derive(Clone)]
pub enum OpDelrouteDoReply {}
impl<'a> IterableOpDelrouteDoReply<'a> {}
impl OpDelrouteDoReply {
    pub fn new(buf: &'_ [u8]) -> (PushRtmsg, IterableOpDelrouteDoReply<'_>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushRtmsg::len()));
        (
            PushRtmsg::new_from_slice(header).unwrap_or_default(),
            IterableOpDelrouteDoReply::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        RouteAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpDelrouteDoReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpDelrouteDoReply<'a> {
    fn with_loc(buf: &'a [u8], orig_loc: usize) -> Self {
        Self {
            buf,
            pos: 0,
            orig_loc,
        }
    }
    pub fn get_buf(&self) -> &'a [u8] {
        self.buf
    }
}
impl<'a> Iterator for IterableOpDelrouteDoReply<'a> {
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
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpDelrouteDoReply",
            r#type.and_then(|t| OpDelrouteDoReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpDelrouteDoReply<'_> {
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
impl IterableOpDelrouteDoReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushRtmsg::len() {
            stack.push(("OpDelrouteDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpDelrouteDoReply::attr_from_type(t)),
            );
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpDelrouteDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpDelrouteDoRequest<'r> {
    pub fn new(mut request: Request<'r>, header: &PushRtmsg) -> Self {
        PushOpDelrouteDoRequest::write_header(&mut request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpDelrouteDoRequest<&mut Vec<u8>> {
        PushOpDelrouteDoRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpDelrouteDoRequest<RequestBuf<'r>> {
        PushOpDelrouteDoRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpDelrouteDoRequest<'_> {
    type ReplyType<'buf> = (PushRtmsg, IterableOpDelrouteDoReply<'buf>);
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 25u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpDelrouteDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpDelrouteDoRequest::new(buf)
            .1
            .lookup_attr(offset, missing_type)
    }
}
#[derive(Debug)]
pub struct ChainedFinal<'a> {
    inner: Chained<'a>,
}
#[derive(Debug)]
pub struct Chained<'a> {
    buf: RequestBuf<'a>,
    first_seq: u32,
    lookups: Vec<(&'static str, LookupFn)>,
    last_header_offset: usize,
    last_kind: Option<RequestInfo>,
}
impl<'a> ChainedFinal<'a> {
    pub fn into_chained(self) -> Chained<'a> {
        self.inner
    }
    pub fn buf(&self) -> &Vec<u8> {
        self.inner.buf()
    }
    pub fn buf_mut(&mut self) -> &mut Vec<u8> {
        self.inner.buf_mut()
    }
    fn get_index(&self, seq: u32) -> Option<u32> {
        let min = self.inner.first_seq;
        let max = min.wrapping_add(self.inner.lookups.len() as u32);
        return if min <= max {
            (min..max).contains(&seq).then(|| seq - min)
        } else if min <= seq {
            Some(seq - min)
        } else if seq < max {
            Some(u32::MAX - min + seq)
        } else {
            None
        };
    }
}
impl crate::traits::NetlinkChained for ChainedFinal<'_> {
    fn protonum(&self) -> u16 {
        PROTONUM
    }
    fn payload(&self) -> &[u8] {
        self.buf()
    }
    fn chain_len(&self) -> usize {
        self.inner.lookups.len()
    }
    fn get_index(&self, seq: u32) -> Option<usize> {
        self.get_index(seq).map(|n| n as usize)
    }
    fn name(&self, index: usize) -> &'static str {
        self.inner.lookups[index].0
    }
    fn lookup(&self, index: usize) -> LookupFn {
        self.inner.lookups[index].1
    }
}
impl Chained<'static> {
    pub fn new(first_seq: u32) -> Self {
        Self::new_from_buf(Vec::new(), first_seq)
    }
    pub fn new_from_buf(buf: Vec<u8>, first_seq: u32) -> Self {
        Self {
            buf: RequestBuf::Own(buf),
            first_seq,
            lookups: Vec::new(),
            last_header_offset: 0,
            last_kind: None,
        }
    }
    pub fn into_buf(self) -> Vec<u8> {
        match self.buf {
            RequestBuf::Own(buf) => buf,
            _ => unreachable!(),
        }
    }
}
impl<'a> Chained<'a> {
    pub fn new_with_buf(buf: &'a mut Vec<u8>, first_seq: u32) -> Self {
        Self {
            buf: RequestBuf::Ref(buf),
            first_seq,
            lookups: Vec::new(),
            last_header_offset: 0,
            last_kind: None,
        }
    }
    pub fn finalize(mut self) -> ChainedFinal<'a> {
        self.update_header();
        ChainedFinal { inner: self }
    }
    pub fn request(&mut self) -> Request<'_> {
        self.update_header();
        self.last_header_offset = self.buf().len();
        self.buf_mut()
            .extend_from_slice(PushNlmsghdr::new().as_slice());
        let mut request = Request::new_extend(self.buf.buf_mut());
        self.last_kind = None;
        request.writeback = Some(&mut self.last_kind);
        request
    }
    pub fn buf(&self) -> &Vec<u8> {
        self.buf.buf()
    }
    pub fn buf_mut(&mut self) -> &mut Vec<u8> {
        self.buf.buf_mut()
    }
    fn update_header(&mut self) {
        let Some(RequestInfo {
            protocol,
            flags,
            name,
            lookup,
        }) = self.last_kind
        else {
            if !self.buf().is_empty() {
                assert_eq!(
                    self.last_header_offset + PushNlmsghdr::len(),
                    self.buf().len()
                );
                self.buf.buf_mut().truncate(self.last_header_offset);
            }
            return;
        };
        let header_offset = self.last_header_offset;
        let request_type = match protocol {
            Protocol::Raw { request_type, .. } => request_type,
            Protocol::Generic(_) => unreachable!(),
        };
        let index = self.lookups.len();
        let seq = self.first_seq.wrapping_add(index as u32);
        self.lookups.push((name, lookup));
        let buf = self.buf_mut();
        align(buf);
        let mut header = PushNlmsghdr::new();
        header.set_len((buf.len() - header_offset) as u32);
        header.set_type(request_type);
        header.set_flags(flags | consts::NLM_F_REQUEST as u16 | consts::NLM_F_ACK as u16);
        header.set_seq(seq);
        buf[header_offset..(header_offset + 16)].clone_from_slice(header.as_slice());
    }
}
use crate::traits::LookupFn;
use crate::utils::RequestBuf;
#[derive(Debug)]
pub struct Request<'buf> {
    buf: RequestBuf<'buf>,
    flags: u16,
    writeback: Option<&'buf mut Option<RequestInfo>>,
}
#[allow(unused)]
#[derive(Debug, Clone)]
pub struct RequestInfo {
    protocol: Protocol,
    flags: u16,
    name: &'static str,
    lookup: LookupFn,
}
impl Request<'static> {
    pub fn new() -> Self {
        Self::new_from_buf(Vec::new())
    }
    pub fn new_from_buf(buf: Vec<u8>) -> Self {
        Self {
            flags: 0,
            buf: RequestBuf::Own(buf),
            writeback: None,
        }
    }
    pub fn into_buf(self) -> Vec<u8> {
        match self.buf {
            RequestBuf::Own(buf) => buf,
            _ => unreachable!(),
        }
    }
}
impl<'buf> Request<'buf> {
    pub fn new_with_buf(buf: &'buf mut Vec<u8>) -> Self {
        buf.clear();
        Self::new_extend(buf)
    }
    pub fn new_extend(buf: &'buf mut Vec<u8>) -> Self {
        Self {
            flags: 0,
            buf: RequestBuf::Ref(buf),
            writeback: None,
        }
    }
    fn do_writeback(&mut self, protocol: Protocol, name: &'static str, lookup: LookupFn) {
        let Some(writeback) = &mut self.writeback else {
            return;
        };
        **writeback = Some(RequestInfo {
            protocol,
            flags: self.flags,
            name,
            lookup,
        })
    }
    pub fn buf(&self) -> &Vec<u8> {
        self.buf.buf()
    }
    pub fn buf_mut(&mut self) -> &mut Vec<u8> {
        self.buf.buf_mut()
    }
    #[doc = "Set `NLM_F_CREATE` flag"]
    pub fn set_create(mut self) -> Self {
        self.flags |= consts::NLM_F_CREATE as u16;
        self
    }
    #[doc = "Set `NLM_F_EXCL` flag"]
    pub fn set_excl(mut self) -> Self {
        self.flags |= consts::NLM_F_EXCL as u16;
        self
    }
    #[doc = "Set `NLM_F_REPLACE` flag"]
    pub fn set_replace(mut self) -> Self {
        self.flags |= consts::NLM_F_REPLACE as u16;
        self
    }
    #[doc = "Set `NLM_F_CREATE` and `NLM_F_REPLACE` flag"]
    pub fn set_change(self) -> Self {
        self.set_create().set_replace()
    }
    #[doc = "Set `NLM_F_APPEND` flag"]
    pub fn set_append(mut self) -> Self {
        self.flags |= consts::NLM_F_APPEND as u16;
        self
    }
    #[doc = "Set `NLM_F_DUMP` flag"]
    fn set_dump(mut self) -> Self {
        self.flags |= consts::NLM_F_DUMP as u16;
        self
    }
    pub fn op_getroute_dump_request(
        self,
        header: &PushRtmsg,
    ) -> RequestOpGetrouteDumpRequest<'buf> {
        let mut res = RequestOpGetrouteDumpRequest::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-getroute-dump-request",
            RequestOpGetrouteDumpRequest::lookup,
        );
        res
    }
    pub fn op_getroute_do_request(self, header: &PushRtmsg) -> RequestOpGetrouteDoRequest<'buf> {
        let mut res = RequestOpGetrouteDoRequest::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-getroute-do-request",
            RequestOpGetrouteDoRequest::lookup,
        );
        res
    }
    pub fn op_newroute_do_request(self, header: &PushRtmsg) -> RequestOpNewrouteDoRequest<'buf> {
        let mut res = RequestOpNewrouteDoRequest::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-newroute-do-request",
            RequestOpNewrouteDoRequest::lookup,
        );
        res
    }
    pub fn op_delroute_do_request(self, header: &PushRtmsg) -> RequestOpDelrouteDoRequest<'buf> {
        let mut res = RequestOpDelrouteDoRequest::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-delroute-do-request",
            RequestOpDelrouteDoRequest::lookup,
        );
        res
    }
}
