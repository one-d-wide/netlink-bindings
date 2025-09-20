#![doc = "FIB rule management over rtnetlink."]
#![allow(clippy::all)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]
#![allow(unreachable_code)]
#![allow(unreachable_patterns)]
use crate::utils::*;
pub const PROTONUM: u8 = 0u8;
#[doc = "Original name: \"fr-act\" (enum) - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Clone)]
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
#[doc = "Original name: \"fib-rule-attrs\""]
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
impl<'a> Iterable<'a, FibRuleAttrs<'a>> {
    pub fn get_dst(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let FibRuleAttrs::Dst(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_src(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let FibRuleAttrs::Src(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_iifname(&self) -> Option<&'a CStr> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let FibRuleAttrs::Iifname(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_goto(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let FibRuleAttrs::Goto(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_unused2(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let FibRuleAttrs::Unused2(val) = attr {
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
            if let FibRuleAttrs::Priority(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_unused3(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let FibRuleAttrs::Unused3(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_unused4(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let FibRuleAttrs::Unused4(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_unused5(&self) -> Option<&'a [u8]> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let FibRuleAttrs::Unused5(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_fwmark(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let FibRuleAttrs::Fwmark(val) = attr {
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
            if let FibRuleAttrs::Flow(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_tun_id(&self) -> Option<u64> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let FibRuleAttrs::TunId(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_suppress_ifgroup(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let FibRuleAttrs::SuppressIfgroup(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_suppress_prefixlen(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let FibRuleAttrs::SuppressPrefixlen(val) = attr {
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
            if let FibRuleAttrs::Table(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_fwmask(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let FibRuleAttrs::Fwmask(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_oifname(&self) -> Option<&'a CStr> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let FibRuleAttrs::Oifname(val) = attr {
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
            if let FibRuleAttrs::Pad(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_l3mdev(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let FibRuleAttrs::L3mdev(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_uid_range(&self) -> Option<PushFibRuleUidRange> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let FibRuleAttrs::UidRange(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_protocol(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let FibRuleAttrs::Protocol(val) = attr {
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
            if let FibRuleAttrs::IpProto(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_sport_range(&self) -> Option<PushFibRulePortRange> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let FibRuleAttrs::SportRange(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_dport_range(&self) -> Option<PushFibRulePortRange> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let FibRuleAttrs::DportRange(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_dscp(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let FibRuleAttrs::Dscp(val) = attr {
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
            if let FibRuleAttrs::Flowlabel(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_flowlabel_mask(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let FibRuleAttrs::FlowlabelMask(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_sport_mask(&self) -> Option<u16> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let FibRuleAttrs::SportMask(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_dport_mask(&self) -> Option<u16> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let FibRuleAttrs::DportMask(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_dscp_mask(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let FibRuleAttrs::DscpMask(val) = attr {
                return Some(val);
            }
        }
        None
    }
}
impl<'a> FibRuleAttrs<'a> {
    pub fn new(buf: &'a [u8]) -> Iterable<'a, FibRuleAttrs<'a>> {
        Iterable::new(buf)
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
impl<'a> Iterator for Iterable<'a, FibRuleAttrs<'a>> {
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
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "FibRuleAttrs",
            r#type.and_then(|t| FibRuleAttrs::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, FibRuleAttrs<'a>> {
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
pub struct PushFibRuleAttrs<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
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
#[doc = "Original name: \"rtgenmsg\""]
#[derive(Clone)]
pub struct PushRtgenmsg {
    buf: [u8; 4usize],
}
impl PushRtgenmsg {
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
#[doc = "Original name: \"fib-rule-hdr\""]
#[derive(Clone)]
pub struct PushFibRuleHdr {
    buf: [u8; 12usize],
}
impl PushFibRuleHdr {
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
#[doc = "Original name: \"fib-rule-port-range\""]
#[derive(Clone)]
pub struct PushFibRulePortRange {
    buf: [u8; 4usize],
}
impl PushFibRulePortRange {
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
#[doc = "Original name: \"fib-rule-uid-range\""]
#[derive(Clone)]
pub struct PushFibRuleUidRange {
    buf: [u8; 8usize],
}
impl PushFibRuleUidRange {
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
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpNewruleDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpNewruleDoRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushFibRuleHdr) -> Self {
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
    pub fn push_iifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            3u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
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
#[doc = "Original name: \"OpNewruleDoRequest\""]
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
impl<'a> Iterable<'a, OpNewruleDoRequest<'a>> {
    pub fn get_iifname(&self) -> Option<&'a CStr> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewruleDoRequest::Iifname(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_goto(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewruleDoRequest::Goto(val) = attr {
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
            if let OpNewruleDoRequest::Priority(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_fwmark(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewruleDoRequest::Fwmark(val) = attr {
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
            if let OpNewruleDoRequest::Flow(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_tun_id(&self) -> Option<u64> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewruleDoRequest::TunId(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_suppress_ifgroup(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewruleDoRequest::SuppressIfgroup(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_suppress_prefixlen(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewruleDoRequest::SuppressPrefixlen(val) = attr {
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
            if let OpNewruleDoRequest::Table(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_fwmask(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewruleDoRequest::Fwmask(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_oifname(&self) -> Option<&'a CStr> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewruleDoRequest::Oifname(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_l3mdev(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewruleDoRequest::L3mdev(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_uid_range(&self) -> Option<PushFibRuleUidRange> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewruleDoRequest::UidRange(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_protocol(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewruleDoRequest::Protocol(val) = attr {
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
            if let OpNewruleDoRequest::IpProto(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_sport_range(&self) -> Option<PushFibRulePortRange> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewruleDoRequest::SportRange(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_dport_range(&self) -> Option<PushFibRulePortRange> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewruleDoRequest::DportRange(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_dscp(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewruleDoRequest::Dscp(val) = attr {
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
            if let OpNewruleDoRequest::Flowlabel(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_flowlabel_mask(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewruleDoRequest::FlowlabelMask(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_sport_mask(&self) -> Option<u16> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewruleDoRequest::SportMask(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_dport_mask(&self) -> Option<u16> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewruleDoRequest::DportMask(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_dscp_mask(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpNewruleDoRequest::DscpMask(val) = attr {
                return Some(val);
            }
        }
        None
    }
}
impl<'a> OpNewruleDoRequest<'a> {
    pub fn new(buf: &'a [u8]) -> (PushFibRuleHdr, Iterable<'a, OpNewruleDoRequest<'a>>) {
        let mut header = PushFibRuleHdr::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushFibRuleHdr::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushFibRuleHdr::len()..], buf.as_ptr()),
        )
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
impl<'a> Iterator for Iterable<'a, OpNewruleDoRequest<'a>> {
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
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpNewruleDoRequest",
            r#type.and_then(|t| OpNewruleDoRequest::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpNewruleDoRequest<'a>> {
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
#[doc = "Add new FIB rule"]
pub struct PushOpNewruleDoReply<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpNewruleDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpNewruleDoReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushFibRuleHdr) -> Self {
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
#[doc = "Original name: \"OpNewruleDoReply\""]
#[derive(Clone)]
pub enum OpNewruleDoReply {}
impl<'a> Iterable<'a, OpNewruleDoReply> {}
impl OpNewruleDoReply {
    pub fn new(buf: &'_ [u8]) -> (PushFibRuleHdr, Iterable<'_, OpNewruleDoReply>) {
        let mut header = PushFibRuleHdr::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushFibRuleHdr::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushFibRuleHdr::len()..], buf.as_ptr()),
        )
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
impl Iterator for Iterable<'_, OpNewruleDoReply> {
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
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpNewruleDoReply",
            r#type.and_then(|t| OpNewruleDoReply::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl std::fmt::Debug for Iterable<'_, OpNewruleDoReply> {
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
#[doc = "Remove an existing FIB rule"]
pub struct PushOpDelruleDoRequest<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpDelruleDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpDelruleDoRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushFibRuleHdr) -> Self {
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
    pub fn push_iifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            3u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
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
#[doc = "Original name: \"OpDelruleDoRequest\""]
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
impl<'a> Iterable<'a, OpDelruleDoRequest<'a>> {
    pub fn get_iifname(&self) -> Option<&'a CStr> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelruleDoRequest::Iifname(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_goto(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelruleDoRequest::Goto(val) = attr {
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
            if let OpDelruleDoRequest::Priority(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_fwmark(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelruleDoRequest::Fwmark(val) = attr {
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
            if let OpDelruleDoRequest::Flow(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_tun_id(&self) -> Option<u64> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelruleDoRequest::TunId(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_suppress_ifgroup(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelruleDoRequest::SuppressIfgroup(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_suppress_prefixlen(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelruleDoRequest::SuppressPrefixlen(val) = attr {
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
            if let OpDelruleDoRequest::Table(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_fwmask(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelruleDoRequest::Fwmask(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_oifname(&self) -> Option<&'a CStr> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelruleDoRequest::Oifname(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_l3mdev(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelruleDoRequest::L3mdev(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_uid_range(&self) -> Option<PushFibRuleUidRange> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelruleDoRequest::UidRange(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_protocol(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelruleDoRequest::Protocol(val) = attr {
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
            if let OpDelruleDoRequest::IpProto(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_sport_range(&self) -> Option<PushFibRulePortRange> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelruleDoRequest::SportRange(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_dport_range(&self) -> Option<PushFibRulePortRange> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelruleDoRequest::DportRange(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_dscp(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelruleDoRequest::Dscp(val) = attr {
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
            if let OpDelruleDoRequest::Flowlabel(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_flowlabel_mask(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelruleDoRequest::FlowlabelMask(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_sport_mask(&self) -> Option<u16> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelruleDoRequest::SportMask(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_dport_mask(&self) -> Option<u16> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelruleDoRequest::DportMask(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_dscp_mask(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpDelruleDoRequest::DscpMask(val) = attr {
                return Some(val);
            }
        }
        None
    }
}
impl<'a> OpDelruleDoRequest<'a> {
    pub fn new(buf: &'a [u8]) -> (PushFibRuleHdr, Iterable<'a, OpDelruleDoRequest<'a>>) {
        let mut header = PushFibRuleHdr::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushFibRuleHdr::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushFibRuleHdr::len()..], buf.as_ptr()),
        )
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
impl<'a> Iterator for Iterable<'a, OpDelruleDoRequest<'a>> {
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
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpDelruleDoRequest",
            r#type.and_then(|t| OpDelruleDoRequest::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpDelruleDoRequest<'a>> {
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
#[doc = "Remove an existing FIB rule"]
pub struct PushOpDelruleDoReply<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpDelruleDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpDelruleDoReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushFibRuleHdr) -> Self {
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
#[doc = "Original name: \"OpDelruleDoReply\""]
#[derive(Clone)]
pub enum OpDelruleDoReply {}
impl<'a> Iterable<'a, OpDelruleDoReply> {}
impl OpDelruleDoReply {
    pub fn new(buf: &'_ [u8]) -> (PushFibRuleHdr, Iterable<'_, OpDelruleDoReply>) {
        let mut header = PushFibRuleHdr::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushFibRuleHdr::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushFibRuleHdr::len()..], buf.as_ptr()),
        )
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
impl Iterator for Iterable<'_, OpDelruleDoReply> {
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
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpDelruleDoReply",
            r#type.and_then(|t| OpDelruleDoReply::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl std::fmt::Debug for Iterable<'_, OpDelruleDoReply> {
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
#[doc = "Dump all FIB rules"]
pub struct PushOpGetruleDumpRequest<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetruleDumpRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetruleDumpRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushFibRuleHdr) -> Self {
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
#[doc = "Original name: \"OpGetruleDumpRequest\""]
#[derive(Clone)]
pub enum OpGetruleDumpRequest {}
impl<'a> Iterable<'a, OpGetruleDumpRequest> {}
impl OpGetruleDumpRequest {
    pub fn new(buf: &'_ [u8]) -> (PushFibRuleHdr, Iterable<'_, OpGetruleDumpRequest>) {
        let mut header = PushFibRuleHdr::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushFibRuleHdr::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushFibRuleHdr::len()..], buf.as_ptr()),
        )
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
impl Iterator for Iterable<'_, OpGetruleDumpRequest> {
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
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpGetruleDumpRequest",
            r#type.and_then(|t| OpGetruleDumpRequest::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl std::fmt::Debug for Iterable<'_, OpGetruleDumpRequest> {
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
#[doc = "Dump all FIB rules"]
pub struct PushOpGetruleDumpReply<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetruleDumpReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetruleDumpReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushFibRuleHdr) -> Self {
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
    pub fn push_iifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            3u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
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
#[doc = "Original name: \"OpGetruleDumpReply\""]
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
impl<'a> Iterable<'a, OpGetruleDumpReply<'a>> {
    pub fn get_iifname(&self) -> Option<&'a CStr> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetruleDumpReply::Iifname(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_goto(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetruleDumpReply::Goto(val) = attr {
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
            if let OpGetruleDumpReply::Priority(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_fwmark(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetruleDumpReply::Fwmark(val) = attr {
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
            if let OpGetruleDumpReply::Flow(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_tun_id(&self) -> Option<u64> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetruleDumpReply::TunId(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_suppress_ifgroup(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetruleDumpReply::SuppressIfgroup(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_suppress_prefixlen(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetruleDumpReply::SuppressPrefixlen(val) = attr {
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
            if let OpGetruleDumpReply::Table(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_fwmask(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetruleDumpReply::Fwmask(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_oifname(&self) -> Option<&'a CStr> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetruleDumpReply::Oifname(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_l3mdev(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetruleDumpReply::L3mdev(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_uid_range(&self) -> Option<PushFibRuleUidRange> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetruleDumpReply::UidRange(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_protocol(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetruleDumpReply::Protocol(val) = attr {
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
            if let OpGetruleDumpReply::IpProto(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_sport_range(&self) -> Option<PushFibRulePortRange> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetruleDumpReply::SportRange(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_dport_range(&self) -> Option<PushFibRulePortRange> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetruleDumpReply::DportRange(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_dscp(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetruleDumpReply::Dscp(val) = attr {
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
            if let OpGetruleDumpReply::Flowlabel(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_flowlabel_mask(&self) -> Option<u32> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetruleDumpReply::FlowlabelMask(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_sport_mask(&self) -> Option<u16> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetruleDumpReply::SportMask(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_dport_mask(&self) -> Option<u16> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetruleDumpReply::DportMask(val) = attr {
                return Some(val);
            }
        }
        None
    }
    pub fn get_dscp_mask(&self) -> Option<u8> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            let Ok(attr) = attr else { break };
            if let OpGetruleDumpReply::DscpMask(val) = attr {
                return Some(val);
            }
        }
        None
    }
}
impl<'a> OpGetruleDumpReply<'a> {
    pub fn new(buf: &'a [u8]) -> (PushFibRuleHdr, Iterable<'a, OpGetruleDumpReply<'a>>) {
        let mut header = PushFibRuleHdr::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushFibRuleHdr::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushFibRuleHdr::len()..], buf.as_ptr()),
        )
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
impl<'a> Iterator for Iterable<'a, OpGetruleDumpReply<'a>> {
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
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpGetruleDumpReply",
            r#type.and_then(|t| OpGetruleDumpReply::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpGetruleDumpReply<'a>> {
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
