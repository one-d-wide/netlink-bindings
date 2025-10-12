#![doc = "FIB rule management over rtnetlink."]
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
pub const PROTONAME: &CStr = c"rt-rule";
pub const PROTONUM: u16 = 0u16;
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum FrAct {
    Unspec = 0,
    ToTbl = 1,
    Goto = 2,
    Nop = 3,
    Res3 = 4,
    Res4 = 5,
    Blackhole = 6,
    Unreachable = 7,
    Prohibit = 8,
}
impl FrAct {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Unspec,
            1 => Self::ToTbl,
            2 => Self::Goto,
            3 => Self::Nop,
            4 => Self::Res3,
            5 => Self::Res4,
            6 => Self::Blackhole,
            7 => Self::Unreachable,
            8 => Self::Prohibit,
            _ => return None,
        })
    }
}
#[derive(Clone)]
pub enum FibRuleAttrs<'a> {
    Dst(u32),
    Src(u32),
    Iifname(&'a CStr),
    Goto(u32),
    Unused2(&'a [u8]),
    Priority(u32),
    Unused3(&'a [u8]),
    Unused4(&'a [u8]),
    Unused5(&'a [u8]),
    Fwmark(u32),
    Flow(u32),
    TunId(u64),
    SuppressIfgroup(u32),
    SuppressPrefixlen(u32),
    Table(u32),
    Fwmask(u32),
    Oifname(&'a CStr),
    Pad(&'a [u8]),
    L3mdev(u8),
    UidRange(PushFibRuleUidRange),
    Protocol(u8),
    IpProto(u8),
    SportRange(PushFibRulePortRange),
    DportRange(PushFibRulePortRange),
    Dscp(u8),
    Flowlabel(u32),
    FlowlabelMask(u32),
    SportMask(u16),
    DportMask(u16),
    DscpMask(u8),
}
impl<'a> IterableFibRuleAttrs<'a> {
    pub fn get_dst(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::Dst(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "Dst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_src(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::Src(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "Src",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_iifname(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::Iifname(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "Iifname",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_goto(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::Goto(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "Goto",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_unused2(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::Unused2(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "Unused2",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_priority(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::Priority(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "Priority",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_unused3(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::Unused3(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "Unused3",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_unused4(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::Unused4(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "Unused4",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_unused5(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::Unused5(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "Unused5",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fwmark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::Fwmark(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "Fwmark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flow(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::Flow(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "Flow",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tun_id(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::TunId(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "TunId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_suppress_ifgroup(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::SuppressIfgroup(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "SuppressIfgroup",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_suppress_prefixlen(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::SuppressPrefixlen(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "SuppressPrefixlen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_table(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::Table(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "Table",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fwmask(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::Fwmask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "Fwmask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_oifname(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::Oifname(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "Oifname",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::Pad(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_l3mdev(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::L3mdev(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "L3mdev",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_uid_range(&self) -> Result<PushFibRuleUidRange, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::UidRange(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "UidRange",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_protocol(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::Protocol(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "Protocol",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ip_proto(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::IpProto(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "IpProto",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sport_range(&self) -> Result<PushFibRulePortRange, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::SportRange(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "SportRange",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dport_range(&self) -> Result<PushFibRulePortRange, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::DportRange(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "DportRange",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dscp(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::Dscp(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "Dscp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flowlabel(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::Flowlabel(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "Flowlabel",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flowlabel_mask(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::FlowlabelMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "FlowlabelMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sport_mask(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::SportMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "SportMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dport_mask(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::DportMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "DportMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dscp_mask(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let FibRuleAttrs::DscpMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FibRuleAttrs",
            "DscpMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> FibRuleAttrs<'a> {
    pub fn new(buf: &'a [u8]) -> IterableFibRuleAttrs<'a> {
        IterableFibRuleAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Dst",
            2u16 => "Src",
            3u16 => "Iifname",
            4u16 => "Goto",
            5u16 => "Unused2",
            6u16 => "Priority",
            7u16 => "Unused3",
            8u16 => "Unused4",
            9u16 => "Unused5",
            10u16 => "Fwmark",
            11u16 => "Flow",
            12u16 => "TunId",
            13u16 => "SuppressIfgroup",
            14u16 => "SuppressPrefixlen",
            15u16 => "Table",
            16u16 => "Fwmask",
            17u16 => "Oifname",
            18u16 => "Pad",
            19u16 => "L3mdev",
            20u16 => "UidRange",
            21u16 => "Protocol",
            22u16 => "IpProto",
            23u16 => "SportRange",
            24u16 => "DportRange",
            25u16 => "Dscp",
            26u16 => "Flowlabel",
            27u16 => "FlowlabelMask",
            28u16 => "SportMask",
            29u16 => "DportMask",
            30u16 => "DscpMask",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableFibRuleAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableFibRuleAttrs<'a> {
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
impl<'a> Iterator for IterableFibRuleAttrs<'a> {
    type Item = Result<FibRuleAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => FibRuleAttrs::Dst({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => FibRuleAttrs::Src({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => FibRuleAttrs::Iifname({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => FibRuleAttrs::Goto({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => FibRuleAttrs::Unused2({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => FibRuleAttrs::Priority({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => FibRuleAttrs::Unused3({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => FibRuleAttrs::Unused4({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => FibRuleAttrs::Unused5({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => FibRuleAttrs::Fwmark({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => FibRuleAttrs::Flow({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => FibRuleAttrs::TunId({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => FibRuleAttrs::SuppressIfgroup({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => FibRuleAttrs::SuppressPrefixlen({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => FibRuleAttrs::Table({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => FibRuleAttrs::Fwmask({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => FibRuleAttrs::Oifname({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => FibRuleAttrs::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => FibRuleAttrs::L3mdev({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => FibRuleAttrs::UidRange({
                    let res = PushFibRuleUidRange::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => FibRuleAttrs::Protocol({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => FibRuleAttrs::IpProto({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => FibRuleAttrs::SportRange({
                    let res = PushFibRulePortRange::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => FibRuleAttrs::DportRange({
                    let res = PushFibRulePortRange::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                25u16 => FibRuleAttrs::Dscp({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                26u16 => FibRuleAttrs::Flowlabel({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                27u16 => FibRuleAttrs::FlowlabelMask({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                28u16 => FibRuleAttrs::SportMask({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                29u16 => FibRuleAttrs::DportMask({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                30u16 => FibRuleAttrs::DscpMask({
                    let res = parse_u8(next);
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
            "FibRuleAttrs",
            r#type.and_then(|t| FibRuleAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableFibRuleAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("FibRuleAttrs");
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
                FibRuleAttrs::Dst(val) => fmt.field("Dst", &val),
                FibRuleAttrs::Src(val) => fmt.field("Src", &val),
                FibRuleAttrs::Iifname(val) => fmt.field("Iifname", &val),
                FibRuleAttrs::Goto(val) => fmt.field("Goto", &val),
                FibRuleAttrs::Unused2(val) => fmt.field("Unused2", &val),
                FibRuleAttrs::Priority(val) => fmt.field("Priority", &val),
                FibRuleAttrs::Unused3(val) => fmt.field("Unused3", &val),
                FibRuleAttrs::Unused4(val) => fmt.field("Unused4", &val),
                FibRuleAttrs::Unused5(val) => fmt.field("Unused5", &val),
                FibRuleAttrs::Fwmark(val) => fmt.field("Fwmark", &val),
                FibRuleAttrs::Flow(val) => fmt.field("Flow", &val),
                FibRuleAttrs::TunId(val) => fmt.field("TunId", &val),
                FibRuleAttrs::SuppressIfgroup(val) => fmt.field("SuppressIfgroup", &val),
                FibRuleAttrs::SuppressPrefixlen(val) => fmt.field("SuppressPrefixlen", &val),
                FibRuleAttrs::Table(val) => fmt.field("Table", &val),
                FibRuleAttrs::Fwmask(val) => fmt.field("Fwmask", &val),
                FibRuleAttrs::Oifname(val) => fmt.field("Oifname", &val),
                FibRuleAttrs::Pad(val) => fmt.field("Pad", &val),
                FibRuleAttrs::L3mdev(val) => fmt.field("L3mdev", &val),
                FibRuleAttrs::UidRange(val) => fmt.field("UidRange", &val),
                FibRuleAttrs::Protocol(val) => fmt.field("Protocol", &val),
                FibRuleAttrs::IpProto(val) => fmt.field("IpProto", &val),
                FibRuleAttrs::SportRange(val) => fmt.field("SportRange", &val),
                FibRuleAttrs::DportRange(val) => fmt.field("DportRange", &val),
                FibRuleAttrs::Dscp(val) => fmt.field("Dscp", &val),
                FibRuleAttrs::Flowlabel(val) => fmt.field("Flowlabel", &val),
                FibRuleAttrs::FlowlabelMask(val) => fmt.field("FlowlabelMask", &val),
                FibRuleAttrs::SportMask(val) => fmt.field("SportMask", &val),
                FibRuleAttrs::DportMask(val) => fmt.field("DportMask", &val),
                FibRuleAttrs::DscpMask(val) => fmt.field("DscpMask", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableFibRuleAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("FibRuleAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| FibRuleAttrs::attr_from_type(t)),
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
                FibRuleAttrs::Dst(val) => {
                    if last_off == offset {
                        stack.push(("Dst", last_off));
                        break;
                    }
                }
                FibRuleAttrs::Src(val) => {
                    if last_off == offset {
                        stack.push(("Src", last_off));
                        break;
                    }
                }
                FibRuleAttrs::Iifname(val) => {
                    if last_off == offset {
                        stack.push(("Iifname", last_off));
                        break;
                    }
                }
                FibRuleAttrs::Goto(val) => {
                    if last_off == offset {
                        stack.push(("Goto", last_off));
                        break;
                    }
                }
                FibRuleAttrs::Unused2(val) => {
                    if last_off == offset {
                        stack.push(("Unused2", last_off));
                        break;
                    }
                }
                FibRuleAttrs::Priority(val) => {
                    if last_off == offset {
                        stack.push(("Priority", last_off));
                        break;
                    }
                }
                FibRuleAttrs::Unused3(val) => {
                    if last_off == offset {
                        stack.push(("Unused3", last_off));
                        break;
                    }
                }
                FibRuleAttrs::Unused4(val) => {
                    if last_off == offset {
                        stack.push(("Unused4", last_off));
                        break;
                    }
                }
                FibRuleAttrs::Unused5(val) => {
                    if last_off == offset {
                        stack.push(("Unused5", last_off));
                        break;
                    }
                }
                FibRuleAttrs::Fwmark(val) => {
                    if last_off == offset {
                        stack.push(("Fwmark", last_off));
                        break;
                    }
                }
                FibRuleAttrs::Flow(val) => {
                    if last_off == offset {
                        stack.push(("Flow", last_off));
                        break;
                    }
                }
                FibRuleAttrs::TunId(val) => {
                    if last_off == offset {
                        stack.push(("TunId", last_off));
                        break;
                    }
                }
                FibRuleAttrs::SuppressIfgroup(val) => {
                    if last_off == offset {
                        stack.push(("SuppressIfgroup", last_off));
                        break;
                    }
                }
                FibRuleAttrs::SuppressPrefixlen(val) => {
                    if last_off == offset {
                        stack.push(("SuppressPrefixlen", last_off));
                        break;
                    }
                }
                FibRuleAttrs::Table(val) => {
                    if last_off == offset {
                        stack.push(("Table", last_off));
                        break;
                    }
                }
                FibRuleAttrs::Fwmask(val) => {
                    if last_off == offset {
                        stack.push(("Fwmask", last_off));
                        break;
                    }
                }
                FibRuleAttrs::Oifname(val) => {
                    if last_off == offset {
                        stack.push(("Oifname", last_off));
                        break;
                    }
                }
                FibRuleAttrs::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                FibRuleAttrs::L3mdev(val) => {
                    if last_off == offset {
                        stack.push(("L3mdev", last_off));
                        break;
                    }
                }
                FibRuleAttrs::UidRange(val) => {
                    if last_off == offset {
                        stack.push(("UidRange", last_off));
                        break;
                    }
                }
                FibRuleAttrs::Protocol(val) => {
                    if last_off == offset {
                        stack.push(("Protocol", last_off));
                        break;
                    }
                }
                FibRuleAttrs::IpProto(val) => {
                    if last_off == offset {
                        stack.push(("IpProto", last_off));
                        break;
                    }
                }
                FibRuleAttrs::SportRange(val) => {
                    if last_off == offset {
                        stack.push(("SportRange", last_off));
                        break;
                    }
                }
                FibRuleAttrs::DportRange(val) => {
                    if last_off == offset {
                        stack.push(("DportRange", last_off));
                        break;
                    }
                }
                FibRuleAttrs::Dscp(val) => {
                    if last_off == offset {
                        stack.push(("Dscp", last_off));
                        break;
                    }
                }
                FibRuleAttrs::Flowlabel(val) => {
                    if last_off == offset {
                        stack.push(("Flowlabel", last_off));
                        break;
                    }
                }
                FibRuleAttrs::FlowlabelMask(val) => {
                    if last_off == offset {
                        stack.push(("FlowlabelMask", last_off));
                        break;
                    }
                }
                FibRuleAttrs::SportMask(val) => {
                    if last_off == offset {
                        stack.push(("SportMask", last_off));
                        break;
                    }
                }
                FibRuleAttrs::DportMask(val) => {
                    if last_off == offset {
                        stack.push(("DportMask", last_off));
                        break;
                    }
                }
                FibRuleAttrs::DscpMask(val) => {
                    if last_off == offset {
                        stack.push(("DscpMask", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("FibRuleAttrs", cur));
        }
        (stack, None)
    }
}
pub struct PushFibRuleAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushFibRuleAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushFibRuleAttrs<Prev> {
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
    pub fn push_dst(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_src(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_iifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            3u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_iifname_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 3u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_goto(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_unused2(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 5u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_priority(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 6u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_unused3(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 7u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_unused4(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 8u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_unused5(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 9u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_fwmark(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 10u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flow(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 11u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tun_id(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 12u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_suppress_ifgroup(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 13u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_suppress_prefixlen(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 14u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_table(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 15u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fwmask(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 16u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_oifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            17u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_oifname_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 17u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 18u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_l3mdev(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 19u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_uid_range(mut self, value: PushFibRuleUidRange) -> Self {
        push_header(self.as_rec_mut(), 20u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_protocol(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 21u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ip_proto(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 22u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sport_range(mut self, value: PushFibRulePortRange) -> Self {
        push_header(self.as_rec_mut(), 23u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_dport_range(mut self, value: PushFibRulePortRange) -> Self {
        push_header(self.as_rec_mut(), 24u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_dscp(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 25u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flowlabel(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 26u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_flowlabel_mask(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 27u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_sport_mask(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 28u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dport_mask(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 29u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dscp_mask(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 30u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushFibRuleAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[derive(Clone)]
pub struct PushRtgenmsg {
    pub(crate) buf: [u8; 4usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for PushRtgenmsg {
    fn default() -> Self {
        Self { buf: [0u8; 4usize] }
    }
}
impl PushRtgenmsg {
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
        4usize
    }
    pub fn family(&self) -> u8 {
        parse_u8(&self.buf[0usize..1usize]).unwrap()
    }
    pub fn set_family(&mut self, value: u8) {
        self.buf[0usize..1usize].copy_from_slice(&value.to_ne_bytes())
    }
}
impl std::fmt::Debug for PushRtgenmsg {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Rtgenmsg")
            .field("family", &self.family())
            .finish()
    }
}
#[derive(Clone)]
pub struct PushFibRuleHdr {
    pub(crate) buf: [u8; 12usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for PushFibRuleHdr {
    fn default() -> Self {
        Self {
            buf: [0u8; 12usize],
        }
    }
}
impl PushFibRuleHdr {
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
    pub fn family(&self) -> u8 {
        parse_u8(&self.buf[0usize..1usize]).unwrap()
    }
    pub fn set_family(&mut self, value: u8) {
        self.buf[0usize..1usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn dst_len(&self) -> u8 {
        parse_u8(&self.buf[1usize..2usize]).unwrap()
    }
    pub fn set_dst_len(&mut self, value: u8) {
        self.buf[1usize..2usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn src_len(&self) -> u8 {
        parse_u8(&self.buf[2usize..3usize]).unwrap()
    }
    pub fn set_src_len(&mut self, value: u8) {
        self.buf[2usize..3usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn tos(&self) -> u8 {
        parse_u8(&self.buf[3usize..4usize]).unwrap()
    }
    pub fn set_tos(&mut self, value: u8) {
        self.buf[3usize..4usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn table(&self) -> u8 {
        parse_u8(&self.buf[4usize..5usize]).unwrap()
    }
    pub fn set_table(&mut self, value: u8) {
        self.buf[4usize..5usize].copy_from_slice(&value.to_ne_bytes())
    }
    #[doc = "Associated type: \"FrAct\" (enum)"]
    pub fn action(&self) -> u8 {
        parse_u8(&self.buf[7usize..8usize]).unwrap()
    }
    #[doc = "Associated type: \"FrAct\" (enum)"]
    pub fn set_action(&mut self, value: u8) {
        self.buf[7usize..8usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn flags(&self) -> u32 {
        parse_u32(&self.buf[8usize..12usize]).unwrap()
    }
    pub fn set_flags(&mut self, value: u32) {
        self.buf[8usize..12usize].copy_from_slice(&value.to_ne_bytes())
    }
}
impl std::fmt::Debug for PushFibRuleHdr {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("FibRuleHdr")
            .field("family", &self.family())
            .field("dst_len", &self.dst_len())
            .field("src_len", &self.src_len())
            .field("tos", &self.tos())
            .field("table", &self.table())
            .field("action", &self.action())
            .field("flags", &self.flags())
            .finish()
    }
}
#[derive(Clone)]
pub struct PushFibRulePortRange {
    pub(crate) buf: [u8; 4usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for PushFibRulePortRange {
    fn default() -> Self {
        Self { buf: [0u8; 4usize] }
    }
}
impl PushFibRulePortRange {
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
        4usize
    }
    pub fn start(&self) -> u16 {
        parse_u16(&self.buf[0usize..2usize]).unwrap()
    }
    pub fn set_start(&mut self, value: u16) {
        self.buf[0usize..2usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn end(&self) -> u16 {
        parse_u16(&self.buf[2usize..4usize]).unwrap()
    }
    pub fn set_end(&mut self, value: u16) {
        self.buf[2usize..4usize].copy_from_slice(&value.to_ne_bytes())
    }
}
impl std::fmt::Debug for PushFibRulePortRange {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("FibRulePortRange")
            .field("start", &self.start())
            .field("end", &self.end())
            .finish()
    }
}
#[derive(Clone)]
pub struct PushFibRuleUidRange {
    pub(crate) buf: [u8; 8usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for PushFibRuleUidRange {
    fn default() -> Self {
        Self { buf: [0u8; 8usize] }
    }
}
impl PushFibRuleUidRange {
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
        8usize
    }
    pub fn start(&self) -> u32 {
        parse_u32(&self.buf[0usize..4usize]).unwrap()
    }
    pub fn set_start(&mut self, value: u32) {
        self.buf[0usize..4usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn end(&self) -> u32 {
        parse_u32(&self.buf[4usize..8usize]).unwrap()
    }
    pub fn set_end(&mut self, value: u32) {
        self.buf[4usize..8usize].copy_from_slice(&value.to_ne_bytes())
    }
}
impl std::fmt::Debug for PushFibRuleUidRange {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("FibRuleUidRange")
            .field("start", &self.start())
            .field("end", &self.end())
            .finish()
    }
}
#[doc = "Add new FIB rule"]
pub struct PushOpNewruleDoRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpNewruleDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpNewruleDoRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushFibRuleHdr) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushFibRuleHdr) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_iifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            3u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_iifname_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 3u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_goto(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_priority(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 6u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fwmark(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 10u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flow(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 11u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tun_id(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 12u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_suppress_ifgroup(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 13u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_suppress_prefixlen(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 14u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_table(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 15u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fwmask(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 16u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_oifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            17u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_oifname_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 17u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_l3mdev(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 19u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_uid_range(mut self, value: PushFibRuleUidRange) -> Self {
        push_header(self.as_rec_mut(), 20u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_protocol(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 21u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ip_proto(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 22u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sport_range(mut self, value: PushFibRulePortRange) -> Self {
        push_header(self.as_rec_mut(), 23u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_dport_range(mut self, value: PushFibRulePortRange) -> Self {
        push_header(self.as_rec_mut(), 24u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_dscp(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 25u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flowlabel(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 26u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_flowlabel_mask(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 27u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_sport_mask(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 28u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dport_mask(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 29u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dscp_mask(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 30u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpNewruleDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Add new FIB rule"]
#[derive(Clone)]
pub enum OpNewruleDoRequest<'a> {
    Iifname(&'a CStr),
    Goto(u32),
    Priority(u32),
    Fwmark(u32),
    Flow(u32),
    TunId(u64),
    SuppressIfgroup(u32),
    SuppressPrefixlen(u32),
    Table(u32),
    Fwmask(u32),
    Oifname(&'a CStr),
    L3mdev(u8),
    UidRange(PushFibRuleUidRange),
    Protocol(u8),
    IpProto(u8),
    SportRange(PushFibRulePortRange),
    DportRange(PushFibRulePortRange),
    Dscp(u8),
    Flowlabel(u32),
    FlowlabelMask(u32),
    SportMask(u16),
    DportMask(u16),
    DscpMask(u8),
}
impl<'a> IterableOpNewruleDoRequest<'a> {
    pub fn get_iifname(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewruleDoRequest::Iifname(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewruleDoRequest",
            "Iifname",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_goto(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewruleDoRequest::Goto(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewruleDoRequest",
            "Goto",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_priority(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewruleDoRequest::Priority(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewruleDoRequest",
            "Priority",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fwmark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewruleDoRequest::Fwmark(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewruleDoRequest",
            "Fwmark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flow(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewruleDoRequest::Flow(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewruleDoRequest",
            "Flow",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tun_id(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewruleDoRequest::TunId(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewruleDoRequest",
            "TunId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_suppress_ifgroup(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewruleDoRequest::SuppressIfgroup(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewruleDoRequest",
            "SuppressIfgroup",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_suppress_prefixlen(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewruleDoRequest::SuppressPrefixlen(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewruleDoRequest",
            "SuppressPrefixlen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_table(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewruleDoRequest::Table(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewruleDoRequest",
            "Table",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fwmask(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewruleDoRequest::Fwmask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewruleDoRequest",
            "Fwmask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_oifname(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewruleDoRequest::Oifname(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewruleDoRequest",
            "Oifname",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_l3mdev(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewruleDoRequest::L3mdev(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewruleDoRequest",
            "L3mdev",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_uid_range(&self) -> Result<PushFibRuleUidRange, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewruleDoRequest::UidRange(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewruleDoRequest",
            "UidRange",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_protocol(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewruleDoRequest::Protocol(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewruleDoRequest",
            "Protocol",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ip_proto(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewruleDoRequest::IpProto(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewruleDoRequest",
            "IpProto",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sport_range(&self) -> Result<PushFibRulePortRange, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewruleDoRequest::SportRange(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewruleDoRequest",
            "SportRange",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dport_range(&self) -> Result<PushFibRulePortRange, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewruleDoRequest::DportRange(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewruleDoRequest",
            "DportRange",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dscp(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewruleDoRequest::Dscp(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewruleDoRequest",
            "Dscp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flowlabel(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewruleDoRequest::Flowlabel(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewruleDoRequest",
            "Flowlabel",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flowlabel_mask(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewruleDoRequest::FlowlabelMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewruleDoRequest",
            "FlowlabelMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sport_mask(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewruleDoRequest::SportMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewruleDoRequest",
            "SportMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dport_mask(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewruleDoRequest::DportMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewruleDoRequest",
            "DportMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dscp_mask(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewruleDoRequest::DscpMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewruleDoRequest",
            "DscpMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> OpNewruleDoRequest<'a> {
    pub fn new(buf: &'a [u8]) -> (PushFibRuleHdr, IterableOpNewruleDoRequest<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushFibRuleHdr::len()));
        (
            PushFibRuleHdr::new_from_slice(header).unwrap_or_default(),
            IterableOpNewruleDoRequest::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        FibRuleAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpNewruleDoRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpNewruleDoRequest<'a> {
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
impl<'a> Iterator for IterableOpNewruleDoRequest<'a> {
    type Item = Result<OpNewruleDoRequest<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                3u16 => OpNewruleDoRequest::Iifname({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpNewruleDoRequest::Goto({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpNewruleDoRequest::Priority({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => OpNewruleDoRequest::Fwmark({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => OpNewruleDoRequest::Flow({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => OpNewruleDoRequest::TunId({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => OpNewruleDoRequest::SuppressIfgroup({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => OpNewruleDoRequest::SuppressPrefixlen({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => OpNewruleDoRequest::Table({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => OpNewruleDoRequest::Fwmask({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => OpNewruleDoRequest::Oifname({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => OpNewruleDoRequest::L3mdev({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => OpNewruleDoRequest::UidRange({
                    let res = PushFibRuleUidRange::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => OpNewruleDoRequest::Protocol({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => OpNewruleDoRequest::IpProto({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => OpNewruleDoRequest::SportRange({
                    let res = PushFibRulePortRange::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => OpNewruleDoRequest::DportRange({
                    let res = PushFibRulePortRange::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                25u16 => OpNewruleDoRequest::Dscp({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                26u16 => OpNewruleDoRequest::Flowlabel({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                27u16 => OpNewruleDoRequest::FlowlabelMask({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                28u16 => OpNewruleDoRequest::SportMask({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                29u16 => OpNewruleDoRequest::DportMask({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                30u16 => OpNewruleDoRequest::DscpMask({
                    let res = parse_u8(next);
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
            "OpNewruleDoRequest",
            r#type.and_then(|t| OpNewruleDoRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpNewruleDoRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpNewruleDoRequest");
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
                OpNewruleDoRequest::Iifname(val) => fmt.field("Iifname", &val),
                OpNewruleDoRequest::Goto(val) => fmt.field("Goto", &val),
                OpNewruleDoRequest::Priority(val) => fmt.field("Priority", &val),
                OpNewruleDoRequest::Fwmark(val) => fmt.field("Fwmark", &val),
                OpNewruleDoRequest::Flow(val) => fmt.field("Flow", &val),
                OpNewruleDoRequest::TunId(val) => fmt.field("TunId", &val),
                OpNewruleDoRequest::SuppressIfgroup(val) => fmt.field("SuppressIfgroup", &val),
                OpNewruleDoRequest::SuppressPrefixlen(val) => fmt.field("SuppressPrefixlen", &val),
                OpNewruleDoRequest::Table(val) => fmt.field("Table", &val),
                OpNewruleDoRequest::Fwmask(val) => fmt.field("Fwmask", &val),
                OpNewruleDoRequest::Oifname(val) => fmt.field("Oifname", &val),
                OpNewruleDoRequest::L3mdev(val) => fmt.field("L3mdev", &val),
                OpNewruleDoRequest::UidRange(val) => fmt.field("UidRange", &val),
                OpNewruleDoRequest::Protocol(val) => fmt.field("Protocol", &val),
                OpNewruleDoRequest::IpProto(val) => fmt.field("IpProto", &val),
                OpNewruleDoRequest::SportRange(val) => fmt.field("SportRange", &val),
                OpNewruleDoRequest::DportRange(val) => fmt.field("DportRange", &val),
                OpNewruleDoRequest::Dscp(val) => fmt.field("Dscp", &val),
                OpNewruleDoRequest::Flowlabel(val) => fmt.field("Flowlabel", &val),
                OpNewruleDoRequest::FlowlabelMask(val) => fmt.field("FlowlabelMask", &val),
                OpNewruleDoRequest::SportMask(val) => fmt.field("SportMask", &val),
                OpNewruleDoRequest::DportMask(val) => fmt.field("DportMask", &val),
                OpNewruleDoRequest::DscpMask(val) => fmt.field("DscpMask", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpNewruleDoRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushFibRuleHdr::len() {
            stack.push(("OpNewruleDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpNewruleDoRequest::attr_from_type(t)),
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
                OpNewruleDoRequest::Iifname(val) => {
                    if last_off == offset {
                        stack.push(("Iifname", last_off));
                        break;
                    }
                }
                OpNewruleDoRequest::Goto(val) => {
                    if last_off == offset {
                        stack.push(("Goto", last_off));
                        break;
                    }
                }
                OpNewruleDoRequest::Priority(val) => {
                    if last_off == offset {
                        stack.push(("Priority", last_off));
                        break;
                    }
                }
                OpNewruleDoRequest::Fwmark(val) => {
                    if last_off == offset {
                        stack.push(("Fwmark", last_off));
                        break;
                    }
                }
                OpNewruleDoRequest::Flow(val) => {
                    if last_off == offset {
                        stack.push(("Flow", last_off));
                        break;
                    }
                }
                OpNewruleDoRequest::TunId(val) => {
                    if last_off == offset {
                        stack.push(("TunId", last_off));
                        break;
                    }
                }
                OpNewruleDoRequest::SuppressIfgroup(val) => {
                    if last_off == offset {
                        stack.push(("SuppressIfgroup", last_off));
                        break;
                    }
                }
                OpNewruleDoRequest::SuppressPrefixlen(val) => {
                    if last_off == offset {
                        stack.push(("SuppressPrefixlen", last_off));
                        break;
                    }
                }
                OpNewruleDoRequest::Table(val) => {
                    if last_off == offset {
                        stack.push(("Table", last_off));
                        break;
                    }
                }
                OpNewruleDoRequest::Fwmask(val) => {
                    if last_off == offset {
                        stack.push(("Fwmask", last_off));
                        break;
                    }
                }
                OpNewruleDoRequest::Oifname(val) => {
                    if last_off == offset {
                        stack.push(("Oifname", last_off));
                        break;
                    }
                }
                OpNewruleDoRequest::L3mdev(val) => {
                    if last_off == offset {
                        stack.push(("L3mdev", last_off));
                        break;
                    }
                }
                OpNewruleDoRequest::UidRange(val) => {
                    if last_off == offset {
                        stack.push(("UidRange", last_off));
                        break;
                    }
                }
                OpNewruleDoRequest::Protocol(val) => {
                    if last_off == offset {
                        stack.push(("Protocol", last_off));
                        break;
                    }
                }
                OpNewruleDoRequest::IpProto(val) => {
                    if last_off == offset {
                        stack.push(("IpProto", last_off));
                        break;
                    }
                }
                OpNewruleDoRequest::SportRange(val) => {
                    if last_off == offset {
                        stack.push(("SportRange", last_off));
                        break;
                    }
                }
                OpNewruleDoRequest::DportRange(val) => {
                    if last_off == offset {
                        stack.push(("DportRange", last_off));
                        break;
                    }
                }
                OpNewruleDoRequest::Dscp(val) => {
                    if last_off == offset {
                        stack.push(("Dscp", last_off));
                        break;
                    }
                }
                OpNewruleDoRequest::Flowlabel(val) => {
                    if last_off == offset {
                        stack.push(("Flowlabel", last_off));
                        break;
                    }
                }
                OpNewruleDoRequest::FlowlabelMask(val) => {
                    if last_off == offset {
                        stack.push(("FlowlabelMask", last_off));
                        break;
                    }
                }
                OpNewruleDoRequest::SportMask(val) => {
                    if last_off == offset {
                        stack.push(("SportMask", last_off));
                        break;
                    }
                }
                OpNewruleDoRequest::DportMask(val) => {
                    if last_off == offset {
                        stack.push(("DportMask", last_off));
                        break;
                    }
                }
                OpNewruleDoRequest::DscpMask(val) => {
                    if last_off == offset {
                        stack.push(("DscpMask", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpNewruleDoRequest", cur));
        }
        (stack, None)
    }
}
#[doc = "Add new FIB rule"]
pub struct PushOpNewruleDoReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpNewruleDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpNewruleDoReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushFibRuleHdr) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushFibRuleHdr) {
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
impl<Prev: Rec> Drop for PushOpNewruleDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Add new FIB rule"]
#[derive(Clone)]
pub enum OpNewruleDoReply {}
impl<'a> IterableOpNewruleDoReply<'a> {}
impl OpNewruleDoReply {
    pub fn new(buf: &'_ [u8]) -> (PushFibRuleHdr, IterableOpNewruleDoReply<'_>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushFibRuleHdr::len()));
        (
            PushFibRuleHdr::new_from_slice(header).unwrap_or_default(),
            IterableOpNewruleDoReply::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        FibRuleAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpNewruleDoReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpNewruleDoReply<'a> {
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
impl<'a> Iterator for IterableOpNewruleDoReply<'a> {
    type Item = Result<OpNewruleDoReply, ErrorContext>;
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
            "OpNewruleDoReply",
            r#type.and_then(|t| OpNewruleDoReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpNewruleDoReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpNewruleDoReply");
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
impl IterableOpNewruleDoReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushFibRuleHdr::len() {
            stack.push(("OpNewruleDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpNewruleDoReply::attr_from_type(t)),
            );
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpNewruleDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpNewruleDoRequest<'r> {
    pub fn new(mut request: Request<'r>, header: &PushFibRuleHdr) -> Self {
        PushOpNewruleDoRequest::write_header(&mut request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpNewruleDoRequest<&mut Vec<u8>> {
        PushOpNewruleDoRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpNewruleDoRequest<RequestBuf<'r>> {
        PushOpNewruleDoRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpNewruleDoRequest<'_> {
    type ReplyType<'buf> = (PushFibRuleHdr, IterableOpNewruleDoReply<'buf>);
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 32u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpNewruleDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpNewruleDoRequest::new(buf)
            .1
            .lookup_attr(offset, missing_type)
    }
}
#[doc = "Remove an existing FIB rule"]
pub struct PushOpDelruleDoRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpDelruleDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpDelruleDoRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushFibRuleHdr) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushFibRuleHdr) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_iifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            3u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_iifname_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 3u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_goto(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_priority(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 6u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fwmark(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 10u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flow(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 11u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tun_id(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 12u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_suppress_ifgroup(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 13u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_suppress_prefixlen(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 14u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_table(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 15u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fwmask(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 16u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_oifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            17u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_oifname_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 17u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_l3mdev(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 19u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_uid_range(mut self, value: PushFibRuleUidRange) -> Self {
        push_header(self.as_rec_mut(), 20u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_protocol(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 21u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ip_proto(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 22u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sport_range(mut self, value: PushFibRulePortRange) -> Self {
        push_header(self.as_rec_mut(), 23u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_dport_range(mut self, value: PushFibRulePortRange) -> Self {
        push_header(self.as_rec_mut(), 24u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_dscp(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 25u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flowlabel(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 26u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_flowlabel_mask(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 27u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_sport_mask(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 28u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dport_mask(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 29u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dscp_mask(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 30u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpDelruleDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Remove an existing FIB rule"]
#[derive(Clone)]
pub enum OpDelruleDoRequest<'a> {
    Iifname(&'a CStr),
    Goto(u32),
    Priority(u32),
    Fwmark(u32),
    Flow(u32),
    TunId(u64),
    SuppressIfgroup(u32),
    SuppressPrefixlen(u32),
    Table(u32),
    Fwmask(u32),
    Oifname(&'a CStr),
    L3mdev(u8),
    UidRange(PushFibRuleUidRange),
    Protocol(u8),
    IpProto(u8),
    SportRange(PushFibRulePortRange),
    DportRange(PushFibRulePortRange),
    Dscp(u8),
    Flowlabel(u32),
    FlowlabelMask(u32),
    SportMask(u16),
    DportMask(u16),
    DscpMask(u8),
}
impl<'a> IterableOpDelruleDoRequest<'a> {
    pub fn get_iifname(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelruleDoRequest::Iifname(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelruleDoRequest",
            "Iifname",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_goto(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelruleDoRequest::Goto(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelruleDoRequest",
            "Goto",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_priority(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelruleDoRequest::Priority(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelruleDoRequest",
            "Priority",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fwmark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelruleDoRequest::Fwmark(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelruleDoRequest",
            "Fwmark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flow(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelruleDoRequest::Flow(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelruleDoRequest",
            "Flow",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tun_id(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelruleDoRequest::TunId(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelruleDoRequest",
            "TunId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_suppress_ifgroup(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelruleDoRequest::SuppressIfgroup(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelruleDoRequest",
            "SuppressIfgroup",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_suppress_prefixlen(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelruleDoRequest::SuppressPrefixlen(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelruleDoRequest",
            "SuppressPrefixlen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_table(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelruleDoRequest::Table(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelruleDoRequest",
            "Table",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fwmask(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelruleDoRequest::Fwmask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelruleDoRequest",
            "Fwmask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_oifname(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelruleDoRequest::Oifname(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelruleDoRequest",
            "Oifname",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_l3mdev(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelruleDoRequest::L3mdev(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelruleDoRequest",
            "L3mdev",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_uid_range(&self) -> Result<PushFibRuleUidRange, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelruleDoRequest::UidRange(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelruleDoRequest",
            "UidRange",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_protocol(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelruleDoRequest::Protocol(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelruleDoRequest",
            "Protocol",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ip_proto(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelruleDoRequest::IpProto(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelruleDoRequest",
            "IpProto",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sport_range(&self) -> Result<PushFibRulePortRange, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelruleDoRequest::SportRange(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelruleDoRequest",
            "SportRange",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dport_range(&self) -> Result<PushFibRulePortRange, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelruleDoRequest::DportRange(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelruleDoRequest",
            "DportRange",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dscp(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelruleDoRequest::Dscp(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelruleDoRequest",
            "Dscp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flowlabel(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelruleDoRequest::Flowlabel(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelruleDoRequest",
            "Flowlabel",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flowlabel_mask(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelruleDoRequest::FlowlabelMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelruleDoRequest",
            "FlowlabelMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sport_mask(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelruleDoRequest::SportMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelruleDoRequest",
            "SportMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dport_mask(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelruleDoRequest::DportMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelruleDoRequest",
            "DportMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dscp_mask(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelruleDoRequest::DscpMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDelruleDoRequest",
            "DscpMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> OpDelruleDoRequest<'a> {
    pub fn new(buf: &'a [u8]) -> (PushFibRuleHdr, IterableOpDelruleDoRequest<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushFibRuleHdr::len()));
        (
            PushFibRuleHdr::new_from_slice(header).unwrap_or_default(),
            IterableOpDelruleDoRequest::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        FibRuleAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpDelruleDoRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpDelruleDoRequest<'a> {
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
impl<'a> Iterator for IterableOpDelruleDoRequest<'a> {
    type Item = Result<OpDelruleDoRequest<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                3u16 => OpDelruleDoRequest::Iifname({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpDelruleDoRequest::Goto({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpDelruleDoRequest::Priority({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => OpDelruleDoRequest::Fwmark({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => OpDelruleDoRequest::Flow({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => OpDelruleDoRequest::TunId({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => OpDelruleDoRequest::SuppressIfgroup({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => OpDelruleDoRequest::SuppressPrefixlen({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => OpDelruleDoRequest::Table({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => OpDelruleDoRequest::Fwmask({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => OpDelruleDoRequest::Oifname({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => OpDelruleDoRequest::L3mdev({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => OpDelruleDoRequest::UidRange({
                    let res = PushFibRuleUidRange::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => OpDelruleDoRequest::Protocol({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => OpDelruleDoRequest::IpProto({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => OpDelruleDoRequest::SportRange({
                    let res = PushFibRulePortRange::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => OpDelruleDoRequest::DportRange({
                    let res = PushFibRulePortRange::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                25u16 => OpDelruleDoRequest::Dscp({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                26u16 => OpDelruleDoRequest::Flowlabel({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                27u16 => OpDelruleDoRequest::FlowlabelMask({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                28u16 => OpDelruleDoRequest::SportMask({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                29u16 => OpDelruleDoRequest::DportMask({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                30u16 => OpDelruleDoRequest::DscpMask({
                    let res = parse_u8(next);
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
            "OpDelruleDoRequest",
            r#type.and_then(|t| OpDelruleDoRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpDelruleDoRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpDelruleDoRequest");
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
                OpDelruleDoRequest::Iifname(val) => fmt.field("Iifname", &val),
                OpDelruleDoRequest::Goto(val) => fmt.field("Goto", &val),
                OpDelruleDoRequest::Priority(val) => fmt.field("Priority", &val),
                OpDelruleDoRequest::Fwmark(val) => fmt.field("Fwmark", &val),
                OpDelruleDoRequest::Flow(val) => fmt.field("Flow", &val),
                OpDelruleDoRequest::TunId(val) => fmt.field("TunId", &val),
                OpDelruleDoRequest::SuppressIfgroup(val) => fmt.field("SuppressIfgroup", &val),
                OpDelruleDoRequest::SuppressPrefixlen(val) => fmt.field("SuppressPrefixlen", &val),
                OpDelruleDoRequest::Table(val) => fmt.field("Table", &val),
                OpDelruleDoRequest::Fwmask(val) => fmt.field("Fwmask", &val),
                OpDelruleDoRequest::Oifname(val) => fmt.field("Oifname", &val),
                OpDelruleDoRequest::L3mdev(val) => fmt.field("L3mdev", &val),
                OpDelruleDoRequest::UidRange(val) => fmt.field("UidRange", &val),
                OpDelruleDoRequest::Protocol(val) => fmt.field("Protocol", &val),
                OpDelruleDoRequest::IpProto(val) => fmt.field("IpProto", &val),
                OpDelruleDoRequest::SportRange(val) => fmt.field("SportRange", &val),
                OpDelruleDoRequest::DportRange(val) => fmt.field("DportRange", &val),
                OpDelruleDoRequest::Dscp(val) => fmt.field("Dscp", &val),
                OpDelruleDoRequest::Flowlabel(val) => fmt.field("Flowlabel", &val),
                OpDelruleDoRequest::FlowlabelMask(val) => fmt.field("FlowlabelMask", &val),
                OpDelruleDoRequest::SportMask(val) => fmt.field("SportMask", &val),
                OpDelruleDoRequest::DportMask(val) => fmt.field("DportMask", &val),
                OpDelruleDoRequest::DscpMask(val) => fmt.field("DscpMask", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpDelruleDoRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushFibRuleHdr::len() {
            stack.push(("OpDelruleDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpDelruleDoRequest::attr_from_type(t)),
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
                OpDelruleDoRequest::Iifname(val) => {
                    if last_off == offset {
                        stack.push(("Iifname", last_off));
                        break;
                    }
                }
                OpDelruleDoRequest::Goto(val) => {
                    if last_off == offset {
                        stack.push(("Goto", last_off));
                        break;
                    }
                }
                OpDelruleDoRequest::Priority(val) => {
                    if last_off == offset {
                        stack.push(("Priority", last_off));
                        break;
                    }
                }
                OpDelruleDoRequest::Fwmark(val) => {
                    if last_off == offset {
                        stack.push(("Fwmark", last_off));
                        break;
                    }
                }
                OpDelruleDoRequest::Flow(val) => {
                    if last_off == offset {
                        stack.push(("Flow", last_off));
                        break;
                    }
                }
                OpDelruleDoRequest::TunId(val) => {
                    if last_off == offset {
                        stack.push(("TunId", last_off));
                        break;
                    }
                }
                OpDelruleDoRequest::SuppressIfgroup(val) => {
                    if last_off == offset {
                        stack.push(("SuppressIfgroup", last_off));
                        break;
                    }
                }
                OpDelruleDoRequest::SuppressPrefixlen(val) => {
                    if last_off == offset {
                        stack.push(("SuppressPrefixlen", last_off));
                        break;
                    }
                }
                OpDelruleDoRequest::Table(val) => {
                    if last_off == offset {
                        stack.push(("Table", last_off));
                        break;
                    }
                }
                OpDelruleDoRequest::Fwmask(val) => {
                    if last_off == offset {
                        stack.push(("Fwmask", last_off));
                        break;
                    }
                }
                OpDelruleDoRequest::Oifname(val) => {
                    if last_off == offset {
                        stack.push(("Oifname", last_off));
                        break;
                    }
                }
                OpDelruleDoRequest::L3mdev(val) => {
                    if last_off == offset {
                        stack.push(("L3mdev", last_off));
                        break;
                    }
                }
                OpDelruleDoRequest::UidRange(val) => {
                    if last_off == offset {
                        stack.push(("UidRange", last_off));
                        break;
                    }
                }
                OpDelruleDoRequest::Protocol(val) => {
                    if last_off == offset {
                        stack.push(("Protocol", last_off));
                        break;
                    }
                }
                OpDelruleDoRequest::IpProto(val) => {
                    if last_off == offset {
                        stack.push(("IpProto", last_off));
                        break;
                    }
                }
                OpDelruleDoRequest::SportRange(val) => {
                    if last_off == offset {
                        stack.push(("SportRange", last_off));
                        break;
                    }
                }
                OpDelruleDoRequest::DportRange(val) => {
                    if last_off == offset {
                        stack.push(("DportRange", last_off));
                        break;
                    }
                }
                OpDelruleDoRequest::Dscp(val) => {
                    if last_off == offset {
                        stack.push(("Dscp", last_off));
                        break;
                    }
                }
                OpDelruleDoRequest::Flowlabel(val) => {
                    if last_off == offset {
                        stack.push(("Flowlabel", last_off));
                        break;
                    }
                }
                OpDelruleDoRequest::FlowlabelMask(val) => {
                    if last_off == offset {
                        stack.push(("FlowlabelMask", last_off));
                        break;
                    }
                }
                OpDelruleDoRequest::SportMask(val) => {
                    if last_off == offset {
                        stack.push(("SportMask", last_off));
                        break;
                    }
                }
                OpDelruleDoRequest::DportMask(val) => {
                    if last_off == offset {
                        stack.push(("DportMask", last_off));
                        break;
                    }
                }
                OpDelruleDoRequest::DscpMask(val) => {
                    if last_off == offset {
                        stack.push(("DscpMask", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpDelruleDoRequest", cur));
        }
        (stack, None)
    }
}
#[doc = "Remove an existing FIB rule"]
pub struct PushOpDelruleDoReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpDelruleDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpDelruleDoReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushFibRuleHdr) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushFibRuleHdr) {
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
impl<Prev: Rec> Drop for PushOpDelruleDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Remove an existing FIB rule"]
#[derive(Clone)]
pub enum OpDelruleDoReply {}
impl<'a> IterableOpDelruleDoReply<'a> {}
impl OpDelruleDoReply {
    pub fn new(buf: &'_ [u8]) -> (PushFibRuleHdr, IterableOpDelruleDoReply<'_>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushFibRuleHdr::len()));
        (
            PushFibRuleHdr::new_from_slice(header).unwrap_or_default(),
            IterableOpDelruleDoReply::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        FibRuleAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpDelruleDoReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpDelruleDoReply<'a> {
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
impl<'a> Iterator for IterableOpDelruleDoReply<'a> {
    type Item = Result<OpDelruleDoReply, ErrorContext>;
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
            "OpDelruleDoReply",
            r#type.and_then(|t| OpDelruleDoReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpDelruleDoReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpDelruleDoReply");
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
impl IterableOpDelruleDoReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushFibRuleHdr::len() {
            stack.push(("OpDelruleDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpDelruleDoReply::attr_from_type(t)),
            );
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpDelruleDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpDelruleDoRequest<'r> {
    pub fn new(mut request: Request<'r>, header: &PushFibRuleHdr) -> Self {
        PushOpDelruleDoRequest::write_header(&mut request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpDelruleDoRequest<&mut Vec<u8>> {
        PushOpDelruleDoRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpDelruleDoRequest<RequestBuf<'r>> {
        PushOpDelruleDoRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpDelruleDoRequest<'_> {
    type ReplyType<'buf> = (PushFibRuleHdr, IterableOpDelruleDoReply<'buf>);
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 33u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpDelruleDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpDelruleDoRequest::new(buf)
            .1
            .lookup_attr(offset, missing_type)
    }
}
#[doc = "Dump all FIB rules"]
pub struct PushOpGetruleDumpRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetruleDumpRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetruleDumpRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushFibRuleHdr) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushFibRuleHdr) {
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
impl<Prev: Rec> Drop for PushOpGetruleDumpRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Dump all FIB rules"]
#[derive(Clone)]
pub enum OpGetruleDumpRequest {}
impl<'a> IterableOpGetruleDumpRequest<'a> {}
impl OpGetruleDumpRequest {
    pub fn new(buf: &'_ [u8]) -> (PushFibRuleHdr, IterableOpGetruleDumpRequest<'_>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushFibRuleHdr::len()));
        (
            PushFibRuleHdr::new_from_slice(header).unwrap_or_default(),
            IterableOpGetruleDumpRequest::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        FibRuleAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetruleDumpRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetruleDumpRequest<'a> {
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
impl<'a> Iterator for IterableOpGetruleDumpRequest<'a> {
    type Item = Result<OpGetruleDumpRequest, ErrorContext>;
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
            "OpGetruleDumpRequest",
            r#type.and_then(|t| OpGetruleDumpRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpGetruleDumpRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetruleDumpRequest");
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
impl IterableOpGetruleDumpRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushFibRuleHdr::len() {
            stack.push(("OpGetruleDumpRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetruleDumpRequest::attr_from_type(t)),
            );
        }
        (stack, None)
    }
}
#[doc = "Dump all FIB rules"]
pub struct PushOpGetruleDumpReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetruleDumpReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetruleDumpReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushFibRuleHdr) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushFibRuleHdr) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_iifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            3u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_iifname_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 3u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_goto(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_priority(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 6u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fwmark(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 10u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flow(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 11u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tun_id(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 12u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_suppress_ifgroup(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 13u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_suppress_prefixlen(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 14u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_table(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 15u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fwmask(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 16u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_oifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            17u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_oifname_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 17u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_l3mdev(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 19u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_uid_range(mut self, value: PushFibRuleUidRange) -> Self {
        push_header(self.as_rec_mut(), 20u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_protocol(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 21u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ip_proto(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 22u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sport_range(mut self, value: PushFibRulePortRange) -> Self {
        push_header(self.as_rec_mut(), 23u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_dport_range(mut self, value: PushFibRulePortRange) -> Self {
        push_header(self.as_rec_mut(), 24u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_dscp(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 25u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flowlabel(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 26u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_flowlabel_mask(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 27u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_sport_mask(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 28u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dport_mask(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 29u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dscp_mask(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 30u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpGetruleDumpReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Dump all FIB rules"]
#[derive(Clone)]
pub enum OpGetruleDumpReply<'a> {
    Iifname(&'a CStr),
    Goto(u32),
    Priority(u32),
    Fwmark(u32),
    Flow(u32),
    TunId(u64),
    SuppressIfgroup(u32),
    SuppressPrefixlen(u32),
    Table(u32),
    Fwmask(u32),
    Oifname(&'a CStr),
    L3mdev(u8),
    UidRange(PushFibRuleUidRange),
    Protocol(u8),
    IpProto(u8),
    SportRange(PushFibRulePortRange),
    DportRange(PushFibRulePortRange),
    Dscp(u8),
    Flowlabel(u32),
    FlowlabelMask(u32),
    SportMask(u16),
    DportMask(u16),
    DscpMask(u8),
}
impl<'a> IterableOpGetruleDumpReply<'a> {
    pub fn get_iifname(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetruleDumpReply::Iifname(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetruleDumpReply",
            "Iifname",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_goto(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetruleDumpReply::Goto(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetruleDumpReply",
            "Goto",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_priority(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetruleDumpReply::Priority(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetruleDumpReply",
            "Priority",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fwmark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetruleDumpReply::Fwmark(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetruleDumpReply",
            "Fwmark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flow(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetruleDumpReply::Flow(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetruleDumpReply",
            "Flow",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tun_id(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetruleDumpReply::TunId(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetruleDumpReply",
            "TunId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_suppress_ifgroup(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetruleDumpReply::SuppressIfgroup(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetruleDumpReply",
            "SuppressIfgroup",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_suppress_prefixlen(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetruleDumpReply::SuppressPrefixlen(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetruleDumpReply",
            "SuppressPrefixlen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_table(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetruleDumpReply::Table(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetruleDumpReply",
            "Table",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fwmask(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetruleDumpReply::Fwmask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetruleDumpReply",
            "Fwmask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_oifname(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetruleDumpReply::Oifname(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetruleDumpReply",
            "Oifname",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_l3mdev(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetruleDumpReply::L3mdev(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetruleDumpReply",
            "L3mdev",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_uid_range(&self) -> Result<PushFibRuleUidRange, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetruleDumpReply::UidRange(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetruleDumpReply",
            "UidRange",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_protocol(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetruleDumpReply::Protocol(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetruleDumpReply",
            "Protocol",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ip_proto(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetruleDumpReply::IpProto(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetruleDumpReply",
            "IpProto",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sport_range(&self) -> Result<PushFibRulePortRange, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetruleDumpReply::SportRange(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetruleDumpReply",
            "SportRange",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dport_range(&self) -> Result<PushFibRulePortRange, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetruleDumpReply::DportRange(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetruleDumpReply",
            "DportRange",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dscp(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetruleDumpReply::Dscp(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetruleDumpReply",
            "Dscp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flowlabel(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetruleDumpReply::Flowlabel(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetruleDumpReply",
            "Flowlabel",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flowlabel_mask(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetruleDumpReply::FlowlabelMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetruleDumpReply",
            "FlowlabelMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sport_mask(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetruleDumpReply::SportMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetruleDumpReply",
            "SportMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dport_mask(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetruleDumpReply::DportMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetruleDumpReply",
            "DportMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dscp_mask(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetruleDumpReply::DscpMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetruleDumpReply",
            "DscpMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> OpGetruleDumpReply<'a> {
    pub fn new(buf: &'a [u8]) -> (PushFibRuleHdr, IterableOpGetruleDumpReply<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushFibRuleHdr::len()));
        (
            PushFibRuleHdr::new_from_slice(header).unwrap_or_default(),
            IterableOpGetruleDumpReply::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        FibRuleAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetruleDumpReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetruleDumpReply<'a> {
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
impl<'a> Iterator for IterableOpGetruleDumpReply<'a> {
    type Item = Result<OpGetruleDumpReply<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                3u16 => OpGetruleDumpReply::Iifname({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpGetruleDumpReply::Goto({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpGetruleDumpReply::Priority({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => OpGetruleDumpReply::Fwmark({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => OpGetruleDumpReply::Flow({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => OpGetruleDumpReply::TunId({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => OpGetruleDumpReply::SuppressIfgroup({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => OpGetruleDumpReply::SuppressPrefixlen({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => OpGetruleDumpReply::Table({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => OpGetruleDumpReply::Fwmask({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => OpGetruleDumpReply::Oifname({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => OpGetruleDumpReply::L3mdev({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => OpGetruleDumpReply::UidRange({
                    let res = PushFibRuleUidRange::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => OpGetruleDumpReply::Protocol({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => OpGetruleDumpReply::IpProto({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => OpGetruleDumpReply::SportRange({
                    let res = PushFibRulePortRange::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => OpGetruleDumpReply::DportRange({
                    let res = PushFibRulePortRange::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                25u16 => OpGetruleDumpReply::Dscp({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                26u16 => OpGetruleDumpReply::Flowlabel({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                27u16 => OpGetruleDumpReply::FlowlabelMask({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                28u16 => OpGetruleDumpReply::SportMask({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                29u16 => OpGetruleDumpReply::DportMask({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                30u16 => OpGetruleDumpReply::DscpMask({
                    let res = parse_u8(next);
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
            "OpGetruleDumpReply",
            r#type.and_then(|t| OpGetruleDumpReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpGetruleDumpReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetruleDumpReply");
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
                OpGetruleDumpReply::Iifname(val) => fmt.field("Iifname", &val),
                OpGetruleDumpReply::Goto(val) => fmt.field("Goto", &val),
                OpGetruleDumpReply::Priority(val) => fmt.field("Priority", &val),
                OpGetruleDumpReply::Fwmark(val) => fmt.field("Fwmark", &val),
                OpGetruleDumpReply::Flow(val) => fmt.field("Flow", &val),
                OpGetruleDumpReply::TunId(val) => fmt.field("TunId", &val),
                OpGetruleDumpReply::SuppressIfgroup(val) => fmt.field("SuppressIfgroup", &val),
                OpGetruleDumpReply::SuppressPrefixlen(val) => fmt.field("SuppressPrefixlen", &val),
                OpGetruleDumpReply::Table(val) => fmt.field("Table", &val),
                OpGetruleDumpReply::Fwmask(val) => fmt.field("Fwmask", &val),
                OpGetruleDumpReply::Oifname(val) => fmt.field("Oifname", &val),
                OpGetruleDumpReply::L3mdev(val) => fmt.field("L3mdev", &val),
                OpGetruleDumpReply::UidRange(val) => fmt.field("UidRange", &val),
                OpGetruleDumpReply::Protocol(val) => fmt.field("Protocol", &val),
                OpGetruleDumpReply::IpProto(val) => fmt.field("IpProto", &val),
                OpGetruleDumpReply::SportRange(val) => fmt.field("SportRange", &val),
                OpGetruleDumpReply::DportRange(val) => fmt.field("DportRange", &val),
                OpGetruleDumpReply::Dscp(val) => fmt.field("Dscp", &val),
                OpGetruleDumpReply::Flowlabel(val) => fmt.field("Flowlabel", &val),
                OpGetruleDumpReply::FlowlabelMask(val) => fmt.field("FlowlabelMask", &val),
                OpGetruleDumpReply::SportMask(val) => fmt.field("SportMask", &val),
                OpGetruleDumpReply::DportMask(val) => fmt.field("DportMask", &val),
                OpGetruleDumpReply::DscpMask(val) => fmt.field("DscpMask", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpGetruleDumpReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushFibRuleHdr::len() {
            stack.push(("OpGetruleDumpReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetruleDumpReply::attr_from_type(t)),
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
                OpGetruleDumpReply::Iifname(val) => {
                    if last_off == offset {
                        stack.push(("Iifname", last_off));
                        break;
                    }
                }
                OpGetruleDumpReply::Goto(val) => {
                    if last_off == offset {
                        stack.push(("Goto", last_off));
                        break;
                    }
                }
                OpGetruleDumpReply::Priority(val) => {
                    if last_off == offset {
                        stack.push(("Priority", last_off));
                        break;
                    }
                }
                OpGetruleDumpReply::Fwmark(val) => {
                    if last_off == offset {
                        stack.push(("Fwmark", last_off));
                        break;
                    }
                }
                OpGetruleDumpReply::Flow(val) => {
                    if last_off == offset {
                        stack.push(("Flow", last_off));
                        break;
                    }
                }
                OpGetruleDumpReply::TunId(val) => {
                    if last_off == offset {
                        stack.push(("TunId", last_off));
                        break;
                    }
                }
                OpGetruleDumpReply::SuppressIfgroup(val) => {
                    if last_off == offset {
                        stack.push(("SuppressIfgroup", last_off));
                        break;
                    }
                }
                OpGetruleDumpReply::SuppressPrefixlen(val) => {
                    if last_off == offset {
                        stack.push(("SuppressPrefixlen", last_off));
                        break;
                    }
                }
                OpGetruleDumpReply::Table(val) => {
                    if last_off == offset {
                        stack.push(("Table", last_off));
                        break;
                    }
                }
                OpGetruleDumpReply::Fwmask(val) => {
                    if last_off == offset {
                        stack.push(("Fwmask", last_off));
                        break;
                    }
                }
                OpGetruleDumpReply::Oifname(val) => {
                    if last_off == offset {
                        stack.push(("Oifname", last_off));
                        break;
                    }
                }
                OpGetruleDumpReply::L3mdev(val) => {
                    if last_off == offset {
                        stack.push(("L3mdev", last_off));
                        break;
                    }
                }
                OpGetruleDumpReply::UidRange(val) => {
                    if last_off == offset {
                        stack.push(("UidRange", last_off));
                        break;
                    }
                }
                OpGetruleDumpReply::Protocol(val) => {
                    if last_off == offset {
                        stack.push(("Protocol", last_off));
                        break;
                    }
                }
                OpGetruleDumpReply::IpProto(val) => {
                    if last_off == offset {
                        stack.push(("IpProto", last_off));
                        break;
                    }
                }
                OpGetruleDumpReply::SportRange(val) => {
                    if last_off == offset {
                        stack.push(("SportRange", last_off));
                        break;
                    }
                }
                OpGetruleDumpReply::DportRange(val) => {
                    if last_off == offset {
                        stack.push(("DportRange", last_off));
                        break;
                    }
                }
                OpGetruleDumpReply::Dscp(val) => {
                    if last_off == offset {
                        stack.push(("Dscp", last_off));
                        break;
                    }
                }
                OpGetruleDumpReply::Flowlabel(val) => {
                    if last_off == offset {
                        stack.push(("Flowlabel", last_off));
                        break;
                    }
                }
                OpGetruleDumpReply::FlowlabelMask(val) => {
                    if last_off == offset {
                        stack.push(("FlowlabelMask", last_off));
                        break;
                    }
                }
                OpGetruleDumpReply::SportMask(val) => {
                    if last_off == offset {
                        stack.push(("SportMask", last_off));
                        break;
                    }
                }
                OpGetruleDumpReply::DportMask(val) => {
                    if last_off == offset {
                        stack.push(("DportMask", last_off));
                        break;
                    }
                }
                OpGetruleDumpReply::DscpMask(val) => {
                    if last_off == offset {
                        stack.push(("DscpMask", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetruleDumpReply", cur));
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpGetruleDumpRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpGetruleDumpRequest<'r> {
    pub fn new(mut request: Request<'r>, header: &PushFibRuleHdr) -> Self {
        PushOpGetruleDumpRequest::write_header(&mut request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode(&mut self) -> PushOpGetruleDumpRequest<&mut Vec<u8>> {
        PushOpGetruleDumpRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpGetruleDumpRequest<RequestBuf<'r>> {
        PushOpGetruleDumpRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpGetruleDumpRequest<'_> {
    type ReplyType<'buf> = (PushFibRuleHdr, IterableOpGetruleDumpReply<'buf>);
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 34u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpGetruleDumpReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpGetruleDumpRequest::new(buf)
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
    pub fn op_newrule_do_request(self, header: &PushFibRuleHdr) -> RequestOpNewruleDoRequest<'buf> {
        let mut res = RequestOpNewruleDoRequest::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-newrule-do-request",
            RequestOpNewruleDoRequest::lookup,
        );
        res
    }
    pub fn op_delrule_do_request(self, header: &PushFibRuleHdr) -> RequestOpDelruleDoRequest<'buf> {
        let mut res = RequestOpDelruleDoRequest::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-delrule-do-request",
            RequestOpDelruleDoRequest::lookup,
        );
        res
    }
    pub fn op_getrule_dump_request(
        self,
        header: &PushFibRuleHdr,
    ) -> RequestOpGetruleDumpRequest<'buf> {
        let mut res = RequestOpGetruleDumpRequest::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-getrule-dump-request",
            RequestOpGetruleDumpRequest::lookup,
        );
        res
    }
}
