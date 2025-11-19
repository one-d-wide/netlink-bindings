#![doc = "Netfilter connection tracking subsystem over nfnetlink"]
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
pub const PROTONAME: &CStr = c"conntrack";
pub const PROTONUM: u16 = 12u16;
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum NfCtTcpFlags {
    WindowScale = 1 << 0,
    SackPerm = 1 << 1,
    CloseInit = 1 << 2,
    BeLiberal = 1 << 3,
    Unacked = 1 << 4,
    Maxack = 1 << 5,
    ChallengeAck = 1 << 6,
    SimultaneousOpen = 1 << 7,
}
impl NfCtTcpFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::WindowScale,
            n if n == 1 << 1 => Self::SackPerm,
            n if n == 1 << 2 => Self::CloseInit,
            n if n == 1 << 3 => Self::BeLiberal,
            n if n == 1 << 4 => Self::Unacked,
            n if n == 1 << 5 => Self::Maxack,
            n if n == 1 << 6 => Self::ChallengeAck,
            n if n == 1 << 7 => Self::SimultaneousOpen,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum NfCtTcpState {
    None = 0,
    SynSent = 1,
    SynRecv = 2,
    Established = 3,
    FinWait = 4,
    CloseWait = 5,
    LastAck = 6,
    TimeWait = 7,
    Close = 8,
    SynSent2 = 9,
    Max = 10,
    Ignore = 11,
    Retrans = 12,
    Unack = 13,
    TimeoutMax = 14,
}
impl NfCtTcpState {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::None,
            1 => Self::SynSent,
            2 => Self::SynRecv,
            3 => Self::Established,
            4 => Self::FinWait,
            5 => Self::CloseWait,
            6 => Self::LastAck,
            7 => Self::TimeWait,
            8 => Self::Close,
            9 => Self::SynSent2,
            10 => Self::Max,
            11 => Self::Ignore,
            12 => Self::Retrans,
            13 => Self::Unack,
            14 => Self::TimeoutMax,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum NfCtSctpState {
    None = 0,
    Cloned = 1,
    CookieWait = 2,
    CookieEchoed = 3,
    Established = 4,
    ShutdownSent = 5,
    ShutdownReceived = 6,
    ShutdownAckSent = 7,
    ShutdownHeartbeatSent = 8,
}
impl NfCtSctpState {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::None,
            1 => Self::Cloned,
            2 => Self::CookieWait,
            3 => Self::CookieEchoed,
            4 => Self::Established,
            5 => Self::ShutdownSent,
            6 => Self::ShutdownReceived,
            7 => Self::ShutdownAckSent,
            8 => Self::ShutdownHeartbeatSent,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum NfCtStatus {
    Expected = 1 << 0,
    SeenReply = 1 << 1,
    Assured = 1 << 2,
    Confirmed = 1 << 3,
    SrcNat = 1 << 4,
    DstNat = 1 << 5,
    SeqAdj = 1 << 6,
    SrcNatDone = 1 << 7,
    DstNatDone = 1 << 8,
    Dying = 1 << 9,
    FixedTimeout = 1 << 10,
    Template = 1 << 11,
    NatClash = 1 << 12,
    Helper = 1 << 13,
    Offload = 1 << 14,
    HwOffload = 1 << 15,
}
impl NfCtStatus {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::Expected,
            n if n == 1 << 1 => Self::SeenReply,
            n if n == 1 << 2 => Self::Assured,
            n if n == 1 << 3 => Self::Confirmed,
            n if n == 1 << 4 => Self::SrcNat,
            n if n == 1 << 5 => Self::DstNat,
            n if n == 1 << 6 => Self::SeqAdj,
            n if n == 1 << 7 => Self::SrcNatDone,
            n if n == 1 << 8 => Self::DstNatDone,
            n if n == 1 << 9 => Self::Dying,
            n if n == 1 << 10 => Self::FixedTimeout,
            n if n == 1 << 11 => Self::Template,
            n if n == 1 << 12 => Self::NatClash,
            n if n == 1 << 13 => Self::Helper,
            n if n == 1 << 14 => Self::Offload,
            n if n == 1 << 15 => Self::HwOffload,
            _ => return None,
        })
    }
}
#[derive(Clone)]
pub enum CounterAttrs<'a> {
    Packets(u64),
    Bytes(u64),
    PacketsOld(u32),
    BytesOld(u32),
    Pad(&'a [u8]),
}
impl<'a> IterableCounterAttrs<'a> {
    pub fn get_packets(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let CounterAttrs::Packets(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CounterAttrs",
            "Packets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_bytes(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let CounterAttrs::Bytes(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CounterAttrs",
            "Bytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_packets_old(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let CounterAttrs::PacketsOld(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CounterAttrs",
            "PacketsOld",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_bytes_old(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let CounterAttrs::BytesOld(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CounterAttrs",
            "BytesOld",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let CounterAttrs::Pad(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CounterAttrs",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl CounterAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableCounterAttrs<'a> {
        IterableCounterAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Packets",
            2u16 => "Bytes",
            3u16 => "PacketsOld",
            4u16 => "BytesOld",
            5u16 => "Pad",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableCounterAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableCounterAttrs<'a> {
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
impl<'a> Iterator for IterableCounterAttrs<'a> {
    type Item = Result<CounterAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => CounterAttrs::Packets({
                    let res = parse_be_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => CounterAttrs::Bytes({
                    let res = parse_be_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => CounterAttrs::PacketsOld({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => CounterAttrs::BytesOld({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => CounterAttrs::Pad({
                    let res = Some(next);
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
            "CounterAttrs",
            r#type.and_then(|t| CounterAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableCounterAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("CounterAttrs");
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
                CounterAttrs::Packets(val) => fmt.field("Packets", &val),
                CounterAttrs::Bytes(val) => fmt.field("Bytes", &val),
                CounterAttrs::PacketsOld(val) => fmt.field("PacketsOld", &val),
                CounterAttrs::BytesOld(val) => fmt.field("BytesOld", &val),
                CounterAttrs::Pad(val) => fmt.field("Pad", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableCounterAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("CounterAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| CounterAttrs::attr_from_type(t)),
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
                CounterAttrs::Packets(val) => {
                    if last_off == offset {
                        stack.push(("Packets", last_off));
                        break;
                    }
                }
                CounterAttrs::Bytes(val) => {
                    if last_off == offset {
                        stack.push(("Bytes", last_off));
                        break;
                    }
                }
                CounterAttrs::PacketsOld(val) => {
                    if last_off == offset {
                        stack.push(("PacketsOld", last_off));
                        break;
                    }
                }
                CounterAttrs::BytesOld(val) => {
                    if last_off == offset {
                        stack.push(("BytesOld", last_off));
                        break;
                    }
                }
                CounterAttrs::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("CounterAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum TupleProtoAttrs {
    #[doc = "l4 protocol number"]
    ProtoNum(u8),
    #[doc = "l4 source port"]
    ProtoSrcPort(u16),
    #[doc = "l4 source port"]
    ProtoDstPort(u16),
    #[doc = "l4 icmp id"]
    ProtoIcmpId(u16),
    ProtoIcmpType(u8),
    ProtoIcmpCode(u8),
    #[doc = "l4 icmp id"]
    ProtoIcmpv6Id(u16),
    ProtoIcmpv6Type(u8),
    ProtoIcmpv6Code(u8),
}
impl<'a> IterableTupleProtoAttrs<'a> {
    #[doc = "l4 protocol number"]
    pub fn get_proto_num(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TupleProtoAttrs::ProtoNum(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TupleProtoAttrs",
            "ProtoNum",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "l4 source port"]
    pub fn get_proto_src_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TupleProtoAttrs::ProtoSrcPort(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TupleProtoAttrs",
            "ProtoSrcPort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "l4 source port"]
    pub fn get_proto_dst_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TupleProtoAttrs::ProtoDstPort(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TupleProtoAttrs",
            "ProtoDstPort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "l4 icmp id"]
    pub fn get_proto_icmp_id(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TupleProtoAttrs::ProtoIcmpId(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TupleProtoAttrs",
            "ProtoIcmpId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_proto_icmp_type(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TupleProtoAttrs::ProtoIcmpType(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TupleProtoAttrs",
            "ProtoIcmpType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_proto_icmp_code(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TupleProtoAttrs::ProtoIcmpCode(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TupleProtoAttrs",
            "ProtoIcmpCode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "l4 icmp id"]
    pub fn get_proto_icmpv6_id(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TupleProtoAttrs::ProtoIcmpv6Id(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TupleProtoAttrs",
            "ProtoIcmpv6Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_proto_icmpv6_type(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TupleProtoAttrs::ProtoIcmpv6Type(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TupleProtoAttrs",
            "ProtoIcmpv6Type",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_proto_icmpv6_code(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TupleProtoAttrs::ProtoIcmpv6Code(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TupleProtoAttrs",
            "ProtoIcmpv6Code",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl TupleProtoAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableTupleProtoAttrs<'a> {
        IterableTupleProtoAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "ProtoNum",
            2u16 => "ProtoSrcPort",
            3u16 => "ProtoDstPort",
            4u16 => "ProtoIcmpId",
            5u16 => "ProtoIcmpType",
            6u16 => "ProtoIcmpCode",
            7u16 => "ProtoIcmpv6Id",
            8u16 => "ProtoIcmpv6Type",
            9u16 => "ProtoIcmpv6Code",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableTupleProtoAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableTupleProtoAttrs<'a> {
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
impl<'a> Iterator for IterableTupleProtoAttrs<'a> {
    type Item = Result<TupleProtoAttrs, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => TupleProtoAttrs::ProtoNum({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => TupleProtoAttrs::ProtoSrcPort({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => TupleProtoAttrs::ProtoDstPort({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => TupleProtoAttrs::ProtoIcmpId({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => TupleProtoAttrs::ProtoIcmpType({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => TupleProtoAttrs::ProtoIcmpCode({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => TupleProtoAttrs::ProtoIcmpv6Id({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => TupleProtoAttrs::ProtoIcmpv6Type({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => TupleProtoAttrs::ProtoIcmpv6Code({
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
            "TupleProtoAttrs",
            r#type.and_then(|t| TupleProtoAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableTupleProtoAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("TupleProtoAttrs");
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
                TupleProtoAttrs::ProtoNum(val) => fmt.field("ProtoNum", &val),
                TupleProtoAttrs::ProtoSrcPort(val) => fmt.field("ProtoSrcPort", &val),
                TupleProtoAttrs::ProtoDstPort(val) => fmt.field("ProtoDstPort", &val),
                TupleProtoAttrs::ProtoIcmpId(val) => fmt.field("ProtoIcmpId", &val),
                TupleProtoAttrs::ProtoIcmpType(val) => fmt.field("ProtoIcmpType", &val),
                TupleProtoAttrs::ProtoIcmpCode(val) => fmt.field("ProtoIcmpCode", &val),
                TupleProtoAttrs::ProtoIcmpv6Id(val) => fmt.field("ProtoIcmpv6Id", &val),
                TupleProtoAttrs::ProtoIcmpv6Type(val) => fmt.field("ProtoIcmpv6Type", &val),
                TupleProtoAttrs::ProtoIcmpv6Code(val) => fmt.field("ProtoIcmpv6Code", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableTupleProtoAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("TupleProtoAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| TupleProtoAttrs::attr_from_type(t)),
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
                TupleProtoAttrs::ProtoNum(val) => {
                    if last_off == offset {
                        stack.push(("ProtoNum", last_off));
                        break;
                    }
                }
                TupleProtoAttrs::ProtoSrcPort(val) => {
                    if last_off == offset {
                        stack.push(("ProtoSrcPort", last_off));
                        break;
                    }
                }
                TupleProtoAttrs::ProtoDstPort(val) => {
                    if last_off == offset {
                        stack.push(("ProtoDstPort", last_off));
                        break;
                    }
                }
                TupleProtoAttrs::ProtoIcmpId(val) => {
                    if last_off == offset {
                        stack.push(("ProtoIcmpId", last_off));
                        break;
                    }
                }
                TupleProtoAttrs::ProtoIcmpType(val) => {
                    if last_off == offset {
                        stack.push(("ProtoIcmpType", last_off));
                        break;
                    }
                }
                TupleProtoAttrs::ProtoIcmpCode(val) => {
                    if last_off == offset {
                        stack.push(("ProtoIcmpCode", last_off));
                        break;
                    }
                }
                TupleProtoAttrs::ProtoIcmpv6Id(val) => {
                    if last_off == offset {
                        stack.push(("ProtoIcmpv6Id", last_off));
                        break;
                    }
                }
                TupleProtoAttrs::ProtoIcmpv6Type(val) => {
                    if last_off == offset {
                        stack.push(("ProtoIcmpv6Type", last_off));
                        break;
                    }
                }
                TupleProtoAttrs::ProtoIcmpv6Code(val) => {
                    if last_off == offset {
                        stack.push(("ProtoIcmpv6Code", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("TupleProtoAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum TupleIpAttrs {
    #[doc = "ipv4 source address"]
    IpV4Src(std::net::Ipv4Addr),
    #[doc = "ipv4 destination address"]
    IpV4Dst(std::net::Ipv4Addr),
    #[doc = "ipv6 source address"]
    IpV6Src(std::net::Ipv6Addr),
    #[doc = "ipv6 destination address"]
    IpV6Dst(std::net::Ipv6Addr),
}
impl<'a> IterableTupleIpAttrs<'a> {
    #[doc = "ipv4 source address"]
    pub fn get_ip_v4_src(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TupleIpAttrs::IpV4Src(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TupleIpAttrs",
            "IpV4Src",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ipv4 destination address"]
    pub fn get_ip_v4_dst(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TupleIpAttrs::IpV4Dst(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TupleIpAttrs",
            "IpV4Dst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ipv6 source address"]
    pub fn get_ip_v6_src(&self) -> Result<std::net::Ipv6Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TupleIpAttrs::IpV6Src(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TupleIpAttrs",
            "IpV6Src",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ipv6 destination address"]
    pub fn get_ip_v6_dst(&self) -> Result<std::net::Ipv6Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TupleIpAttrs::IpV6Dst(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TupleIpAttrs",
            "IpV6Dst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl TupleIpAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableTupleIpAttrs<'a> {
        IterableTupleIpAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "IpV4Src",
            2u16 => "IpV4Dst",
            3u16 => "IpV6Src",
            4u16 => "IpV6Dst",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableTupleIpAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableTupleIpAttrs<'a> {
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
impl<'a> Iterator for IterableTupleIpAttrs<'a> {
    type Item = Result<TupleIpAttrs, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => TupleIpAttrs::IpV4Src({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => TupleIpAttrs::IpV4Dst({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => TupleIpAttrs::IpV6Src({
                    let res = parse_be_u128(next).map(Ipv6Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => TupleIpAttrs::IpV6Dst({
                    let res = parse_be_u128(next).map(Ipv6Addr::from_bits);
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
            "TupleIpAttrs",
            r#type.and_then(|t| TupleIpAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableTupleIpAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("TupleIpAttrs");
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
                TupleIpAttrs::IpV4Src(val) => fmt.field("IpV4Src", &val),
                TupleIpAttrs::IpV4Dst(val) => fmt.field("IpV4Dst", &val),
                TupleIpAttrs::IpV6Src(val) => fmt.field("IpV6Src", &val),
                TupleIpAttrs::IpV6Dst(val) => fmt.field("IpV6Dst", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableTupleIpAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("TupleIpAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| TupleIpAttrs::attr_from_type(t)),
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
                TupleIpAttrs::IpV4Src(val) => {
                    if last_off == offset {
                        stack.push(("IpV4Src", last_off));
                        break;
                    }
                }
                TupleIpAttrs::IpV4Dst(val) => {
                    if last_off == offset {
                        stack.push(("IpV4Dst", last_off));
                        break;
                    }
                }
                TupleIpAttrs::IpV6Src(val) => {
                    if last_off == offset {
                        stack.push(("IpV6Src", last_off));
                        break;
                    }
                }
                TupleIpAttrs::IpV6Dst(val) => {
                    if last_off == offset {
                        stack.push(("IpV6Dst", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("TupleIpAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum TupleAttrs<'a> {
    #[doc = "conntrack l3 information"]
    TupleIp(IterableTupleIpAttrs<'a>),
    #[doc = "conntrack l4 information"]
    TupleProto(IterableTupleProtoAttrs<'a>),
    #[doc = "conntrack zone id"]
    TupleZone(u16),
}
impl<'a> IterableTupleAttrs<'a> {
    #[doc = "conntrack l3 information"]
    pub fn get_tuple_ip(&self) -> Result<IterableTupleIpAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TupleAttrs::TupleIp(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TupleAttrs",
            "TupleIp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "conntrack l4 information"]
    pub fn get_tuple_proto(&self) -> Result<IterableTupleProtoAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TupleAttrs::TupleProto(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TupleAttrs",
            "TupleProto",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "conntrack zone id"]
    pub fn get_tuple_zone(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let TupleAttrs::TupleZone(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TupleAttrs",
            "TupleZone",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl TupleAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableTupleAttrs<'a> {
        IterableTupleAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "TupleIp",
            2u16 => "TupleProto",
            3u16 => "TupleZone",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableTupleAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableTupleAttrs<'a> {
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
impl<'a> Iterator for IterableTupleAttrs<'a> {
    type Item = Result<TupleAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => TupleAttrs::TupleIp({
                    let res = Some(IterableTupleIpAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => TupleAttrs::TupleProto({
                    let res = Some(IterableTupleProtoAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => TupleAttrs::TupleZone({
                    let res = parse_be_u16(next);
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
            "TupleAttrs",
            r#type.and_then(|t| TupleAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableTupleAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("TupleAttrs");
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
                TupleAttrs::TupleIp(val) => fmt.field("TupleIp", &val),
                TupleAttrs::TupleProto(val) => fmt.field("TupleProto", &val),
                TupleAttrs::TupleZone(val) => fmt.field("TupleZone", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableTupleAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("TupleAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| TupleAttrs::attr_from_type(t)),
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
                TupleAttrs::TupleIp(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                TupleAttrs::TupleProto(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                TupleAttrs::TupleZone(val) => {
                    if last_off == offset {
                        stack.push(("TupleZone", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("TupleAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum ProtoinfoTcpAttrs {
    #[doc = "tcp connection state\nAssociated type: \"NfCtTcpState\" (enum)"]
    TcpState(u8),
    #[doc = "window scaling factor in original direction"]
    TcpWscaleOriginal(u8),
    #[doc = "window scaling factor in reply direction"]
    TcpWscaleReply(u8),
    TcpFlagsOriginal(PushNfCtTcpFlagsMask),
    TcpFlagsReply(PushNfCtTcpFlagsMask),
}
impl<'a> IterableProtoinfoTcpAttrs<'a> {
    #[doc = "tcp connection state\nAssociated type: \"NfCtTcpState\" (enum)"]
    pub fn get_tcp_state(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ProtoinfoTcpAttrs::TcpState(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ProtoinfoTcpAttrs",
            "TcpState",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "window scaling factor in original direction"]
    pub fn get_tcp_wscale_original(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ProtoinfoTcpAttrs::TcpWscaleOriginal(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ProtoinfoTcpAttrs",
            "TcpWscaleOriginal",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "window scaling factor in reply direction"]
    pub fn get_tcp_wscale_reply(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ProtoinfoTcpAttrs::TcpWscaleReply(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ProtoinfoTcpAttrs",
            "TcpWscaleReply",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tcp_flags_original(&self) -> Result<PushNfCtTcpFlagsMask, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ProtoinfoTcpAttrs::TcpFlagsOriginal(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ProtoinfoTcpAttrs",
            "TcpFlagsOriginal",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tcp_flags_reply(&self) -> Result<PushNfCtTcpFlagsMask, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ProtoinfoTcpAttrs::TcpFlagsReply(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ProtoinfoTcpAttrs",
            "TcpFlagsReply",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ProtoinfoTcpAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableProtoinfoTcpAttrs<'a> {
        IterableProtoinfoTcpAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "TcpState",
            2u16 => "TcpWscaleOriginal",
            3u16 => "TcpWscaleReply",
            4u16 => "TcpFlagsOriginal",
            5u16 => "TcpFlagsReply",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableProtoinfoTcpAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableProtoinfoTcpAttrs<'a> {
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
impl<'a> Iterator for IterableProtoinfoTcpAttrs<'a> {
    type Item = Result<ProtoinfoTcpAttrs, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => ProtoinfoTcpAttrs::TcpState({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ProtoinfoTcpAttrs::TcpWscaleOriginal({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ProtoinfoTcpAttrs::TcpWscaleReply({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => ProtoinfoTcpAttrs::TcpFlagsOriginal({
                    let res = PushNfCtTcpFlagsMask::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => ProtoinfoTcpAttrs::TcpFlagsReply({
                    let res = PushNfCtTcpFlagsMask::new_from_slice(next);
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
            "ProtoinfoTcpAttrs",
            r#type.and_then(|t| ProtoinfoTcpAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableProtoinfoTcpAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ProtoinfoTcpAttrs");
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
                ProtoinfoTcpAttrs::TcpState(val) => fmt.field(
                    "TcpState",
                    &FormatEnum(val.into(), NfCtTcpState::from_value),
                ),
                ProtoinfoTcpAttrs::TcpWscaleOriginal(val) => fmt.field("TcpWscaleOriginal", &val),
                ProtoinfoTcpAttrs::TcpWscaleReply(val) => fmt.field("TcpWscaleReply", &val),
                ProtoinfoTcpAttrs::TcpFlagsOriginal(val) => fmt.field("TcpFlagsOriginal", &val),
                ProtoinfoTcpAttrs::TcpFlagsReply(val) => fmt.field("TcpFlagsReply", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableProtoinfoTcpAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("ProtoinfoTcpAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ProtoinfoTcpAttrs::attr_from_type(t)),
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
                ProtoinfoTcpAttrs::TcpState(val) => {
                    if last_off == offset {
                        stack.push(("TcpState", last_off));
                        break;
                    }
                }
                ProtoinfoTcpAttrs::TcpWscaleOriginal(val) => {
                    if last_off == offset {
                        stack.push(("TcpWscaleOriginal", last_off));
                        break;
                    }
                }
                ProtoinfoTcpAttrs::TcpWscaleReply(val) => {
                    if last_off == offset {
                        stack.push(("TcpWscaleReply", last_off));
                        break;
                    }
                }
                ProtoinfoTcpAttrs::TcpFlagsOriginal(val) => {
                    if last_off == offset {
                        stack.push(("TcpFlagsOriginal", last_off));
                        break;
                    }
                }
                ProtoinfoTcpAttrs::TcpFlagsReply(val) => {
                    if last_off == offset {
                        stack.push(("TcpFlagsReply", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ProtoinfoTcpAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ProtoinfoDccpAttrs<'a> {
    #[doc = "dccp connection state"]
    DccpState(u8),
    DccpRole(u8),
    DccpHandshakeSeq(u64),
    DccpPad(&'a [u8]),
}
impl<'a> IterableProtoinfoDccpAttrs<'a> {
    #[doc = "dccp connection state"]
    pub fn get_dccp_state(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ProtoinfoDccpAttrs::DccpState(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ProtoinfoDccpAttrs",
            "DccpState",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dccp_role(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ProtoinfoDccpAttrs::DccpRole(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ProtoinfoDccpAttrs",
            "DccpRole",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dccp_handshake_seq(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ProtoinfoDccpAttrs::DccpHandshakeSeq(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ProtoinfoDccpAttrs",
            "DccpHandshakeSeq",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dccp_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ProtoinfoDccpAttrs::DccpPad(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ProtoinfoDccpAttrs",
            "DccpPad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ProtoinfoDccpAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableProtoinfoDccpAttrs<'a> {
        IterableProtoinfoDccpAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "DccpState",
            2u16 => "DccpRole",
            3u16 => "DccpHandshakeSeq",
            4u16 => "DccpPad",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableProtoinfoDccpAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableProtoinfoDccpAttrs<'a> {
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
impl<'a> Iterator for IterableProtoinfoDccpAttrs<'a> {
    type Item = Result<ProtoinfoDccpAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => ProtoinfoDccpAttrs::DccpState({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ProtoinfoDccpAttrs::DccpRole({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ProtoinfoDccpAttrs::DccpHandshakeSeq({
                    let res = parse_be_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => ProtoinfoDccpAttrs::DccpPad({
                    let res = Some(next);
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
            "ProtoinfoDccpAttrs",
            r#type.and_then(|t| ProtoinfoDccpAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableProtoinfoDccpAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ProtoinfoDccpAttrs");
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
                ProtoinfoDccpAttrs::DccpState(val) => fmt.field("DccpState", &val),
                ProtoinfoDccpAttrs::DccpRole(val) => fmt.field("DccpRole", &val),
                ProtoinfoDccpAttrs::DccpHandshakeSeq(val) => fmt.field("DccpHandshakeSeq", &val),
                ProtoinfoDccpAttrs::DccpPad(val) => fmt.field("DccpPad", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableProtoinfoDccpAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("ProtoinfoDccpAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ProtoinfoDccpAttrs::attr_from_type(t)),
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
                ProtoinfoDccpAttrs::DccpState(val) => {
                    if last_off == offset {
                        stack.push(("DccpState", last_off));
                        break;
                    }
                }
                ProtoinfoDccpAttrs::DccpRole(val) => {
                    if last_off == offset {
                        stack.push(("DccpRole", last_off));
                        break;
                    }
                }
                ProtoinfoDccpAttrs::DccpHandshakeSeq(val) => {
                    if last_off == offset {
                        stack.push(("DccpHandshakeSeq", last_off));
                        break;
                    }
                }
                ProtoinfoDccpAttrs::DccpPad(val) => {
                    if last_off == offset {
                        stack.push(("DccpPad", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ProtoinfoDccpAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ProtoinfoSctpAttrs {
    #[doc = "sctp connection state\nAssociated type: \"NfCtSctpState\" (enum)"]
    SctpState(u8),
    VtagOriginal(u32),
    VtagReply(u32),
}
impl<'a> IterableProtoinfoSctpAttrs<'a> {
    #[doc = "sctp connection state\nAssociated type: \"NfCtSctpState\" (enum)"]
    pub fn get_sctp_state(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ProtoinfoSctpAttrs::SctpState(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ProtoinfoSctpAttrs",
            "SctpState",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vtag_original(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ProtoinfoSctpAttrs::VtagOriginal(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ProtoinfoSctpAttrs",
            "VtagOriginal",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vtag_reply(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ProtoinfoSctpAttrs::VtagReply(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ProtoinfoSctpAttrs",
            "VtagReply",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ProtoinfoSctpAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableProtoinfoSctpAttrs<'a> {
        IterableProtoinfoSctpAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "SctpState",
            2u16 => "VtagOriginal",
            3u16 => "VtagReply",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableProtoinfoSctpAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableProtoinfoSctpAttrs<'a> {
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
impl<'a> Iterator for IterableProtoinfoSctpAttrs<'a> {
    type Item = Result<ProtoinfoSctpAttrs, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => ProtoinfoSctpAttrs::SctpState({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ProtoinfoSctpAttrs::VtagOriginal({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ProtoinfoSctpAttrs::VtagReply({
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
            "ProtoinfoSctpAttrs",
            r#type.and_then(|t| ProtoinfoSctpAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableProtoinfoSctpAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ProtoinfoSctpAttrs");
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
                ProtoinfoSctpAttrs::SctpState(val) => fmt.field(
                    "SctpState",
                    &FormatEnum(val.into(), NfCtSctpState::from_value),
                ),
                ProtoinfoSctpAttrs::VtagOriginal(val) => fmt.field("VtagOriginal", &val),
                ProtoinfoSctpAttrs::VtagReply(val) => fmt.field("VtagReply", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableProtoinfoSctpAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("ProtoinfoSctpAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ProtoinfoSctpAttrs::attr_from_type(t)),
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
                ProtoinfoSctpAttrs::SctpState(val) => {
                    if last_off == offset {
                        stack.push(("SctpState", last_off));
                        break;
                    }
                }
                ProtoinfoSctpAttrs::VtagOriginal(val) => {
                    if last_off == offset {
                        stack.push(("VtagOriginal", last_off));
                        break;
                    }
                }
                ProtoinfoSctpAttrs::VtagReply(val) => {
                    if last_off == offset {
                        stack.push(("VtagReply", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ProtoinfoSctpAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ProtoinfoAttrs<'a> {
    #[doc = "conntrack tcp state information"]
    ProtoinfoTcp(IterableProtoinfoTcpAttrs<'a>),
    #[doc = "conntrack dccp state information"]
    ProtoinfoDccp(IterableProtoinfoDccpAttrs<'a>),
    #[doc = "conntrack sctp state information"]
    ProtoinfoSctp(IterableProtoinfoSctpAttrs<'a>),
}
impl<'a> IterableProtoinfoAttrs<'a> {
    #[doc = "conntrack tcp state information"]
    pub fn get_protoinfo_tcp(&self) -> Result<IterableProtoinfoTcpAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ProtoinfoAttrs::ProtoinfoTcp(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ProtoinfoAttrs",
            "ProtoinfoTcp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "conntrack dccp state information"]
    pub fn get_protoinfo_dccp(&self) -> Result<IterableProtoinfoDccpAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ProtoinfoAttrs::ProtoinfoDccp(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ProtoinfoAttrs",
            "ProtoinfoDccp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "conntrack sctp state information"]
    pub fn get_protoinfo_sctp(&self) -> Result<IterableProtoinfoSctpAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ProtoinfoAttrs::ProtoinfoSctp(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ProtoinfoAttrs",
            "ProtoinfoSctp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ProtoinfoAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableProtoinfoAttrs<'a> {
        IterableProtoinfoAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "ProtoinfoTcp",
            2u16 => "ProtoinfoDccp",
            3u16 => "ProtoinfoSctp",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableProtoinfoAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableProtoinfoAttrs<'a> {
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
impl<'a> Iterator for IterableProtoinfoAttrs<'a> {
    type Item = Result<ProtoinfoAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => ProtoinfoAttrs::ProtoinfoTcp({
                    let res = Some(IterableProtoinfoTcpAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ProtoinfoAttrs::ProtoinfoDccp({
                    let res = Some(IterableProtoinfoDccpAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ProtoinfoAttrs::ProtoinfoSctp({
                    let res = Some(IterableProtoinfoSctpAttrs::with_loc(next, self.orig_loc));
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
            "ProtoinfoAttrs",
            r#type.and_then(|t| ProtoinfoAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableProtoinfoAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ProtoinfoAttrs");
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
                ProtoinfoAttrs::ProtoinfoTcp(val) => fmt.field("ProtoinfoTcp", &val),
                ProtoinfoAttrs::ProtoinfoDccp(val) => fmt.field("ProtoinfoDccp", &val),
                ProtoinfoAttrs::ProtoinfoSctp(val) => fmt.field("ProtoinfoSctp", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableProtoinfoAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("ProtoinfoAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ProtoinfoAttrs::attr_from_type(t)),
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
                ProtoinfoAttrs::ProtoinfoTcp(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ProtoinfoAttrs::ProtoinfoDccp(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ProtoinfoAttrs::ProtoinfoSctp(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ProtoinfoAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum HelpAttrs<'a> {
    #[doc = "helper name"]
    HelpName(&'a CStr),
}
impl<'a> IterableHelpAttrs<'a> {
    #[doc = "helper name"]
    pub fn get_help_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let HelpAttrs::HelpName(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "HelpAttrs",
            "HelpName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl HelpAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableHelpAttrs<'a> {
        IterableHelpAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "HelpName",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableHelpAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableHelpAttrs<'a> {
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
impl<'a> Iterator for IterableHelpAttrs<'a> {
    type Item = Result<HelpAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => HelpAttrs::HelpName({
                    let res = CStr::from_bytes_with_nul(next).ok();
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
            "HelpAttrs",
            r#type.and_then(|t| HelpAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableHelpAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("HelpAttrs");
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
                HelpAttrs::HelpName(val) => fmt.field("HelpName", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableHelpAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("HelpAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| HelpAttrs::attr_from_type(t)),
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
                HelpAttrs::HelpName(val) => {
                    if last_off == offset {
                        stack.push(("HelpName", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("HelpAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum NatProtoAttrs {
    NatPortMin(u16),
    NatPortMax(u16),
}
impl<'a> IterableNatProtoAttrs<'a> {
    pub fn get_nat_port_min(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NatProtoAttrs::NatPortMin(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NatProtoAttrs",
            "NatPortMin",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nat_port_max(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NatProtoAttrs::NatPortMax(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NatProtoAttrs",
            "NatPortMax",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl NatProtoAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableNatProtoAttrs<'a> {
        IterableNatProtoAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "NatPortMin",
            2u16 => "NatPortMax",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableNatProtoAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableNatProtoAttrs<'a> {
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
impl<'a> Iterator for IterableNatProtoAttrs<'a> {
    type Item = Result<NatProtoAttrs, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => NatProtoAttrs::NatPortMin({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => NatProtoAttrs::NatPortMax({
                    let res = parse_be_u16(next);
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
            "NatProtoAttrs",
            r#type.and_then(|t| NatProtoAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableNatProtoAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("NatProtoAttrs");
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
                NatProtoAttrs::NatPortMin(val) => fmt.field("NatPortMin", &val),
                NatProtoAttrs::NatPortMax(val) => fmt.field("NatPortMax", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableNatProtoAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("NatProtoAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| NatProtoAttrs::attr_from_type(t)),
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
                NatProtoAttrs::NatPortMin(val) => {
                    if last_off == offset {
                        stack.push(("NatPortMin", last_off));
                        break;
                    }
                }
                NatProtoAttrs::NatPortMax(val) => {
                    if last_off == offset {
                        stack.push(("NatPortMax", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("NatProtoAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum NatAttrs<'a> {
    NatV4Minip(u32),
    NatV4Maxip(u32),
    NatV6Minip(&'a [u8]),
    NatV6Maxip(&'a [u8]),
    NatProto(IterableNatProtoAttrs<'a>),
}
impl<'a> IterableNatAttrs<'a> {
    pub fn get_nat_v4_minip(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NatAttrs::NatV4Minip(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NatAttrs",
            "NatV4Minip",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nat_v4_maxip(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NatAttrs::NatV4Maxip(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NatAttrs",
            "NatV4Maxip",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nat_v6_minip(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NatAttrs::NatV6Minip(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NatAttrs",
            "NatV6Minip",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nat_v6_maxip(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NatAttrs::NatV6Maxip(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NatAttrs",
            "NatV6Maxip",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nat_proto(&self) -> Result<IterableNatProtoAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NatAttrs::NatProto(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NatAttrs",
            "NatProto",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl NatAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableNatAttrs<'a> {
        IterableNatAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "NatV4Minip",
            2u16 => "NatV4Maxip",
            3u16 => "NatV6Minip",
            4u16 => "NatV6Maxip",
            5u16 => "NatProto",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableNatAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableNatAttrs<'a> {
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
impl<'a> Iterator for IterableNatAttrs<'a> {
    type Item = Result<NatAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => NatAttrs::NatV4Minip({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => NatAttrs::NatV4Maxip({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => NatAttrs::NatV6Minip({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => NatAttrs::NatV6Maxip({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => NatAttrs::NatProto({
                    let res = Some(IterableNatProtoAttrs::with_loc(next, self.orig_loc));
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
            "NatAttrs",
            r#type.and_then(|t| NatAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableNatAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("NatAttrs");
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
                NatAttrs::NatV4Minip(val) => fmt.field("NatV4Minip", &val),
                NatAttrs::NatV4Maxip(val) => fmt.field("NatV4Maxip", &val),
                NatAttrs::NatV6Minip(val) => fmt.field("NatV6Minip", &val),
                NatAttrs::NatV6Maxip(val) => fmt.field("NatV6Maxip", &val),
                NatAttrs::NatProto(val) => fmt.field("NatProto", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableNatAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("NatAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| NatAttrs::attr_from_type(t)),
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
                NatAttrs::NatV4Minip(val) => {
                    if last_off == offset {
                        stack.push(("NatV4Minip", last_off));
                        break;
                    }
                }
                NatAttrs::NatV4Maxip(val) => {
                    if last_off == offset {
                        stack.push(("NatV4Maxip", last_off));
                        break;
                    }
                }
                NatAttrs::NatV6Minip(val) => {
                    if last_off == offset {
                        stack.push(("NatV6Minip", last_off));
                        break;
                    }
                }
                NatAttrs::NatV6Maxip(val) => {
                    if last_off == offset {
                        stack.push(("NatV6Maxip", last_off));
                        break;
                    }
                }
                NatAttrs::NatProto(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("NatAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum SeqadjAttrs {
    CorrectionPos(u32),
    OffsetBefore(u32),
    OffsetAfter(u32),
}
impl<'a> IterableSeqadjAttrs<'a> {
    pub fn get_correction_pos(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let SeqadjAttrs::CorrectionPos(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SeqadjAttrs",
            "CorrectionPos",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_offset_before(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let SeqadjAttrs::OffsetBefore(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SeqadjAttrs",
            "OffsetBefore",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_offset_after(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let SeqadjAttrs::OffsetAfter(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SeqadjAttrs",
            "OffsetAfter",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl SeqadjAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableSeqadjAttrs<'a> {
        IterableSeqadjAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "CorrectionPos",
            2u16 => "OffsetBefore",
            3u16 => "OffsetAfter",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableSeqadjAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSeqadjAttrs<'a> {
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
impl<'a> Iterator for IterableSeqadjAttrs<'a> {
    type Item = Result<SeqadjAttrs, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => SeqadjAttrs::CorrectionPos({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => SeqadjAttrs::OffsetBefore({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => SeqadjAttrs::OffsetAfter({
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
            "SeqadjAttrs",
            r#type.and_then(|t| SeqadjAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableSeqadjAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("SeqadjAttrs");
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
                SeqadjAttrs::CorrectionPos(val) => fmt.field("CorrectionPos", &val),
                SeqadjAttrs::OffsetBefore(val) => fmt.field("OffsetBefore", &val),
                SeqadjAttrs::OffsetAfter(val) => fmt.field("OffsetAfter", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSeqadjAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("SeqadjAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| SeqadjAttrs::attr_from_type(t)),
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
                SeqadjAttrs::CorrectionPos(val) => {
                    if last_off == offset {
                        stack.push(("CorrectionPos", last_off));
                        break;
                    }
                }
                SeqadjAttrs::OffsetBefore(val) => {
                    if last_off == offset {
                        stack.push(("OffsetBefore", last_off));
                        break;
                    }
                }
                SeqadjAttrs::OffsetAfter(val) => {
                    if last_off == offset {
                        stack.push(("OffsetAfter", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("SeqadjAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum SecctxAttrs<'a> {
    SecctxName(&'a CStr),
}
impl<'a> IterableSecctxAttrs<'a> {
    pub fn get_secctx_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let SecctxAttrs::SecctxName(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SecctxAttrs",
            "SecctxName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl SecctxAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableSecctxAttrs<'a> {
        IterableSecctxAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "SecctxName",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableSecctxAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSecctxAttrs<'a> {
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
impl<'a> Iterator for IterableSecctxAttrs<'a> {
    type Item = Result<SecctxAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => SecctxAttrs::SecctxName({
                    let res = CStr::from_bytes_with_nul(next).ok();
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
            "SecctxAttrs",
            r#type.and_then(|t| SecctxAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableSecctxAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("SecctxAttrs");
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
                SecctxAttrs::SecctxName(val) => fmt.field("SecctxName", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSecctxAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("SecctxAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| SecctxAttrs::attr_from_type(t)),
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
                SecctxAttrs::SecctxName(val) => {
                    if last_off == offset {
                        stack.push(("SecctxName", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("SecctxAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum SynproxyAttrs {
    Isn(u32),
    Its(u32),
    Tsoff(u32),
}
impl<'a> IterableSynproxyAttrs<'a> {
    pub fn get_isn(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let SynproxyAttrs::Isn(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SynproxyAttrs",
            "Isn",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_its(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let SynproxyAttrs::Its(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SynproxyAttrs",
            "Its",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tsoff(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let SynproxyAttrs::Tsoff(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SynproxyAttrs",
            "Tsoff",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl SynproxyAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableSynproxyAttrs<'a> {
        IterableSynproxyAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Isn",
            2u16 => "Its",
            3u16 => "Tsoff",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableSynproxyAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSynproxyAttrs<'a> {
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
impl<'a> Iterator for IterableSynproxyAttrs<'a> {
    type Item = Result<SynproxyAttrs, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => SynproxyAttrs::Isn({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => SynproxyAttrs::Its({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => SynproxyAttrs::Tsoff({
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
            "SynproxyAttrs",
            r#type.and_then(|t| SynproxyAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableSynproxyAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("SynproxyAttrs");
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
                SynproxyAttrs::Isn(val) => fmt.field("Isn", &val),
                SynproxyAttrs::Its(val) => fmt.field("Its", &val),
                SynproxyAttrs::Tsoff(val) => fmt.field("Tsoff", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSynproxyAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("SynproxyAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| SynproxyAttrs::attr_from_type(t)),
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
                SynproxyAttrs::Isn(val) => {
                    if last_off == offset {
                        stack.push(("Isn", last_off));
                        break;
                    }
                }
                SynproxyAttrs::Its(val) => {
                    if last_off == offset {
                        stack.push(("Its", last_off));
                        break;
                    }
                }
                SynproxyAttrs::Tsoff(val) => {
                    if last_off == offset {
                        stack.push(("Tsoff", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("SynproxyAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ConntrackAttrs<'a> {
    #[doc = "conntrack l3+l4 protocol information, original direction"]
    TupleOrig(IterableTupleAttrs<'a>),
    #[doc = "conntrack l3+l4 protocol information, reply direction"]
    TupleReply(IterableTupleAttrs<'a>),
    #[doc = "conntrack flag bits\nAssociated type: \"NfCtStatus\" (1 bit per enumeration)"]
    Status(u32),
    Protoinfo(IterableProtoinfoAttrs<'a>),
    Help(IterableHelpAttrs<'a>),
    NatSrc(IterableNatAttrs<'a>),
    Timeout(u32),
    Mark(u32),
    CountersOrig(IterableCounterAttrs<'a>),
    CountersReply(IterableCounterAttrs<'a>),
    Use(u32),
    Id(u32),
    NatDst(IterableNatAttrs<'a>),
    TupleMaster(IterableTupleAttrs<'a>),
    SeqAdjOrig(IterableSeqadjAttrs<'a>),
    SeqAdjReply(IterableSeqadjAttrs<'a>),
    #[doc = "obsolete"]
    Secmark(&'a [u8]),
    #[doc = "conntrack zone id"]
    Zone(u16),
    Secctx(IterableSecctxAttrs<'a>),
    Timestamp(u64),
    MarkMask(u32),
    Labels(&'a [u8]),
    LabelsMask(&'a [u8]),
    Synproxy(IterableSynproxyAttrs<'a>),
    Filter(IterableTupleAttrs<'a>),
    #[doc = "conntrack flag bits to change\nAssociated type: \"NfCtStatus\" (1 bit per enumeration)"]
    StatusMask(u32),
    TimestampEvent(u64),
}
impl<'a> IterableConntrackAttrs<'a> {
    #[doc = "conntrack l3+l4 protocol information, original direction"]
    pub fn get_tuple_orig(&self) -> Result<IterableTupleAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::TupleOrig(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "TupleOrig",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "conntrack l3+l4 protocol information, reply direction"]
    pub fn get_tuple_reply(&self) -> Result<IterableTupleAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::TupleReply(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "TupleReply",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "conntrack flag bits\nAssociated type: \"NfCtStatus\" (1 bit per enumeration)"]
    pub fn get_status(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::Status(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "Status",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_protoinfo(&self) -> Result<IterableProtoinfoAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::Protoinfo(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "Protoinfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_help(&self) -> Result<IterableHelpAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::Help(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "Help",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nat_src(&self) -> Result<IterableNatAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::NatSrc(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "NatSrc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_timeout(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::Timeout(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "Timeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::Mark(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "Mark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_counters_orig(&self) -> Result<IterableCounterAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::CountersOrig(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "CountersOrig",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_counters_reply(&self) -> Result<IterableCounterAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::CountersReply(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "CountersReply",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_use(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::Use(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "Use",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::Id(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nat_dst(&self) -> Result<IterableNatAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::NatDst(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "NatDst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tuple_master(&self) -> Result<IterableTupleAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::TupleMaster(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "TupleMaster",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_seq_adj_orig(&self) -> Result<IterableSeqadjAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::SeqAdjOrig(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "SeqAdjOrig",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_seq_adj_reply(&self) -> Result<IterableSeqadjAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::SeqAdjReply(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "SeqAdjReply",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "obsolete"]
    pub fn get_secmark(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::Secmark(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "Secmark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "conntrack zone id"]
    pub fn get_zone(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::Zone(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "Zone",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_secctx(&self) -> Result<IterableSecctxAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::Secctx(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "Secctx",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_timestamp(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::Timestamp(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "Timestamp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mark_mask(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::MarkMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "MarkMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_labels(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::Labels(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "Labels",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_labels_mask(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::LabelsMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "LabelsMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_synproxy(&self) -> Result<IterableSynproxyAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::Synproxy(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "Synproxy",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_filter(&self) -> Result<IterableTupleAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::Filter(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "Filter",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "conntrack flag bits to change\nAssociated type: \"NfCtStatus\" (1 bit per enumeration)"]
    pub fn get_status_mask(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::StatusMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "StatusMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_timestamp_event(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackAttrs::TimestampEvent(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackAttrs",
            "TimestampEvent",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ConntrackAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableConntrackAttrs<'a> {
        IterableConntrackAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "TupleOrig",
            2u16 => "TupleReply",
            3u16 => "Status",
            4u16 => "Protoinfo",
            5u16 => "Help",
            6u16 => "NatSrc",
            7u16 => "Timeout",
            8u16 => "Mark",
            9u16 => "CountersOrig",
            10u16 => "CountersReply",
            11u16 => "Use",
            12u16 => "Id",
            13u16 => "NatDst",
            14u16 => "TupleMaster",
            15u16 => "SeqAdjOrig",
            16u16 => "SeqAdjReply",
            17u16 => "Secmark",
            18u16 => "Zone",
            19u16 => "Secctx",
            20u16 => "Timestamp",
            21u16 => "MarkMask",
            22u16 => "Labels",
            23u16 => "LabelsMask",
            24u16 => "Synproxy",
            25u16 => "Filter",
            26u16 => "StatusMask",
            27u16 => "TimestampEvent",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableConntrackAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableConntrackAttrs<'a> {
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
impl<'a> Iterator for IterableConntrackAttrs<'a> {
    type Item = Result<ConntrackAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => ConntrackAttrs::TupleOrig({
                    let res = Some(IterableTupleAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ConntrackAttrs::TupleReply({
                    let res = Some(IterableTupleAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ConntrackAttrs::Status({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => ConntrackAttrs::Protoinfo({
                    let res = Some(IterableProtoinfoAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => ConntrackAttrs::Help({
                    let res = Some(IterableHelpAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => ConntrackAttrs::NatSrc({
                    let res = Some(IterableNatAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => ConntrackAttrs::Timeout({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => ConntrackAttrs::Mark({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => ConntrackAttrs::CountersOrig({
                    let res = Some(IterableCounterAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => ConntrackAttrs::CountersReply({
                    let res = Some(IterableCounterAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => ConntrackAttrs::Use({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => ConntrackAttrs::Id({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => ConntrackAttrs::NatDst({
                    let res = Some(IterableNatAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => ConntrackAttrs::TupleMaster({
                    let res = Some(IterableTupleAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => ConntrackAttrs::SeqAdjOrig({
                    let res = Some(IterableSeqadjAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => ConntrackAttrs::SeqAdjReply({
                    let res = Some(IterableSeqadjAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => ConntrackAttrs::Secmark({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => ConntrackAttrs::Zone({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => ConntrackAttrs::Secctx({
                    let res = Some(IterableSecctxAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => ConntrackAttrs::Timestamp({
                    let res = parse_be_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => ConntrackAttrs::MarkMask({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => ConntrackAttrs::Labels({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => ConntrackAttrs::LabelsMask({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => ConntrackAttrs::Synproxy({
                    let res = Some(IterableSynproxyAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                25u16 => ConntrackAttrs::Filter({
                    let res = Some(IterableTupleAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                26u16 => ConntrackAttrs::StatusMask({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                27u16 => ConntrackAttrs::TimestampEvent({
                    let res = parse_be_u64(next);
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
            "ConntrackAttrs",
            r#type.and_then(|t| ConntrackAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableConntrackAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ConntrackAttrs");
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
                ConntrackAttrs::TupleOrig(val) => fmt.field("TupleOrig", &val),
                ConntrackAttrs::TupleReply(val) => fmt.field("TupleReply", &val),
                ConntrackAttrs::Status(val) => {
                    fmt.field("Status", &FormatFlags(val.into(), NfCtStatus::from_value))
                }
                ConntrackAttrs::Protoinfo(val) => fmt.field("Protoinfo", &val),
                ConntrackAttrs::Help(val) => fmt.field("Help", &val),
                ConntrackAttrs::NatSrc(val) => fmt.field("NatSrc", &val),
                ConntrackAttrs::Timeout(val) => fmt.field("Timeout", &val),
                ConntrackAttrs::Mark(val) => fmt.field("Mark", &val),
                ConntrackAttrs::CountersOrig(val) => fmt.field("CountersOrig", &val),
                ConntrackAttrs::CountersReply(val) => fmt.field("CountersReply", &val),
                ConntrackAttrs::Use(val) => fmt.field("Use", &val),
                ConntrackAttrs::Id(val) => fmt.field("Id", &val),
                ConntrackAttrs::NatDst(val) => fmt.field("NatDst", &val),
                ConntrackAttrs::TupleMaster(val) => fmt.field("TupleMaster", &val),
                ConntrackAttrs::SeqAdjOrig(val) => fmt.field("SeqAdjOrig", &val),
                ConntrackAttrs::SeqAdjReply(val) => fmt.field("SeqAdjReply", &val),
                ConntrackAttrs::Secmark(val) => fmt.field("Secmark", &val),
                ConntrackAttrs::Zone(val) => fmt.field("Zone", &val),
                ConntrackAttrs::Secctx(val) => fmt.field("Secctx", &val),
                ConntrackAttrs::Timestamp(val) => fmt.field("Timestamp", &val),
                ConntrackAttrs::MarkMask(val) => fmt.field("MarkMask", &val),
                ConntrackAttrs::Labels(val) => fmt.field("Labels", &val),
                ConntrackAttrs::LabelsMask(val) => fmt.field("LabelsMask", &val),
                ConntrackAttrs::Synproxy(val) => fmt.field("Synproxy", &val),
                ConntrackAttrs::Filter(val) => fmt.field("Filter", &val),
                ConntrackAttrs::StatusMask(val) => fmt.field(
                    "StatusMask",
                    &FormatFlags(val.into(), NfCtStatus::from_value),
                ),
                ConntrackAttrs::TimestampEvent(val) => fmt.field("TimestampEvent", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableConntrackAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("ConntrackAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ConntrackAttrs::attr_from_type(t)),
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
                ConntrackAttrs::TupleOrig(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ConntrackAttrs::TupleReply(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ConntrackAttrs::Status(val) => {
                    if last_off == offset {
                        stack.push(("Status", last_off));
                        break;
                    }
                }
                ConntrackAttrs::Protoinfo(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ConntrackAttrs::Help(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ConntrackAttrs::NatSrc(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ConntrackAttrs::Timeout(val) => {
                    if last_off == offset {
                        stack.push(("Timeout", last_off));
                        break;
                    }
                }
                ConntrackAttrs::Mark(val) => {
                    if last_off == offset {
                        stack.push(("Mark", last_off));
                        break;
                    }
                }
                ConntrackAttrs::CountersOrig(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ConntrackAttrs::CountersReply(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ConntrackAttrs::Use(val) => {
                    if last_off == offset {
                        stack.push(("Use", last_off));
                        break;
                    }
                }
                ConntrackAttrs::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                ConntrackAttrs::NatDst(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ConntrackAttrs::TupleMaster(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ConntrackAttrs::SeqAdjOrig(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ConntrackAttrs::SeqAdjReply(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ConntrackAttrs::Secmark(val) => {
                    if last_off == offset {
                        stack.push(("Secmark", last_off));
                        break;
                    }
                }
                ConntrackAttrs::Zone(val) => {
                    if last_off == offset {
                        stack.push(("Zone", last_off));
                        break;
                    }
                }
                ConntrackAttrs::Secctx(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ConntrackAttrs::Timestamp(val) => {
                    if last_off == offset {
                        stack.push(("Timestamp", last_off));
                        break;
                    }
                }
                ConntrackAttrs::MarkMask(val) => {
                    if last_off == offset {
                        stack.push(("MarkMask", last_off));
                        break;
                    }
                }
                ConntrackAttrs::Labels(val) => {
                    if last_off == offset {
                        stack.push(("Labels", last_off));
                        break;
                    }
                }
                ConntrackAttrs::LabelsMask(val) => {
                    if last_off == offset {
                        stack.push(("LabelsMask", last_off));
                        break;
                    }
                }
                ConntrackAttrs::Synproxy(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ConntrackAttrs::Filter(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ConntrackAttrs::StatusMask(val) => {
                    if last_off == offset {
                        stack.push(("StatusMask", last_off));
                        break;
                    }
                }
                ConntrackAttrs::TimestampEvent(val) => {
                    if last_off == offset {
                        stack.push(("TimestampEvent", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ConntrackAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum ConntrackStatsAttrs {
    #[doc = "obsolete"]
    Searched(u32),
    Found(u32),
    #[doc = "obsolete"]
    New(u32),
    #[doc = "obsolete"]
    Invalid(u32),
    #[doc = "obsolete"]
    Ignore(u32),
    #[doc = "obsolete"]
    Delete(u32),
    #[doc = "obsolete"]
    DeleteList(u32),
    Insert(u32),
    InsertFailed(u32),
    Drop(u32),
    EarlyDrop(u32),
    Error(u32),
    SearchRestart(u32),
    ClashResolve(u32),
    ChainToolong(u32),
}
impl<'a> IterableConntrackStatsAttrs<'a> {
    #[doc = "obsolete"]
    pub fn get_searched(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackStatsAttrs::Searched(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackStatsAttrs",
            "Searched",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_found(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackStatsAttrs::Found(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackStatsAttrs",
            "Found",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "obsolete"]
    pub fn get_new(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackStatsAttrs::New(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackStatsAttrs",
            "New",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "obsolete"]
    pub fn get_invalid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackStatsAttrs::Invalid(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackStatsAttrs",
            "Invalid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "obsolete"]
    pub fn get_ignore(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackStatsAttrs::Ignore(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackStatsAttrs",
            "Ignore",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "obsolete"]
    pub fn get_delete(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackStatsAttrs::Delete(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackStatsAttrs",
            "Delete",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "obsolete"]
    pub fn get_delete_list(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackStatsAttrs::DeleteList(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackStatsAttrs",
            "DeleteList",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_insert(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackStatsAttrs::Insert(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackStatsAttrs",
            "Insert",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_insert_failed(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackStatsAttrs::InsertFailed(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackStatsAttrs",
            "InsertFailed",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_drop(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackStatsAttrs::Drop(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackStatsAttrs",
            "Drop",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_early_drop(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackStatsAttrs::EarlyDrop(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackStatsAttrs",
            "EarlyDrop",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_error(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackStatsAttrs::Error(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackStatsAttrs",
            "Error",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_search_restart(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackStatsAttrs::SearchRestart(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackStatsAttrs",
            "SearchRestart",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_clash_resolve(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackStatsAttrs::ClashResolve(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackStatsAttrs",
            "ClashResolve",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_chain_toolong(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let ConntrackStatsAttrs::ChainToolong(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ConntrackStatsAttrs",
            "ChainToolong",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ConntrackStatsAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableConntrackStatsAttrs<'a> {
        IterableConntrackStatsAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Searched",
            2u16 => "Found",
            3u16 => "New",
            4u16 => "Invalid",
            5u16 => "Ignore",
            6u16 => "Delete",
            7u16 => "DeleteList",
            8u16 => "Insert",
            9u16 => "InsertFailed",
            10u16 => "Drop",
            11u16 => "EarlyDrop",
            12u16 => "Error",
            13u16 => "SearchRestart",
            14u16 => "ClashResolve",
            15u16 => "ChainToolong",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableConntrackStatsAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableConntrackStatsAttrs<'a> {
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
impl<'a> Iterator for IterableConntrackStatsAttrs<'a> {
    type Item = Result<ConntrackStatsAttrs, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => ConntrackStatsAttrs::Searched({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ConntrackStatsAttrs::Found({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ConntrackStatsAttrs::New({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => ConntrackStatsAttrs::Invalid({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => ConntrackStatsAttrs::Ignore({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => ConntrackStatsAttrs::Delete({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => ConntrackStatsAttrs::DeleteList({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => ConntrackStatsAttrs::Insert({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => ConntrackStatsAttrs::InsertFailed({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => ConntrackStatsAttrs::Drop({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => ConntrackStatsAttrs::EarlyDrop({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => ConntrackStatsAttrs::Error({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => ConntrackStatsAttrs::SearchRestart({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => ConntrackStatsAttrs::ClashResolve({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => ConntrackStatsAttrs::ChainToolong({
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
            "ConntrackStatsAttrs",
            r#type.and_then(|t| ConntrackStatsAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableConntrackStatsAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ConntrackStatsAttrs");
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
                ConntrackStatsAttrs::Searched(val) => fmt.field("Searched", &val),
                ConntrackStatsAttrs::Found(val) => fmt.field("Found", &val),
                ConntrackStatsAttrs::New(val) => fmt.field("New", &val),
                ConntrackStatsAttrs::Invalid(val) => fmt.field("Invalid", &val),
                ConntrackStatsAttrs::Ignore(val) => fmt.field("Ignore", &val),
                ConntrackStatsAttrs::Delete(val) => fmt.field("Delete", &val),
                ConntrackStatsAttrs::DeleteList(val) => fmt.field("DeleteList", &val),
                ConntrackStatsAttrs::Insert(val) => fmt.field("Insert", &val),
                ConntrackStatsAttrs::InsertFailed(val) => fmt.field("InsertFailed", &val),
                ConntrackStatsAttrs::Drop(val) => fmt.field("Drop", &val),
                ConntrackStatsAttrs::EarlyDrop(val) => fmt.field("EarlyDrop", &val),
                ConntrackStatsAttrs::Error(val) => fmt.field("Error", &val),
                ConntrackStatsAttrs::SearchRestart(val) => fmt.field("SearchRestart", &val),
                ConntrackStatsAttrs::ClashResolve(val) => fmt.field("ClashResolve", &val),
                ConntrackStatsAttrs::ChainToolong(val) => fmt.field("ChainToolong", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableConntrackStatsAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("ConntrackStatsAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ConntrackStatsAttrs::attr_from_type(t)),
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
                ConntrackStatsAttrs::Searched(val) => {
                    if last_off == offset {
                        stack.push(("Searched", last_off));
                        break;
                    }
                }
                ConntrackStatsAttrs::Found(val) => {
                    if last_off == offset {
                        stack.push(("Found", last_off));
                        break;
                    }
                }
                ConntrackStatsAttrs::New(val) => {
                    if last_off == offset {
                        stack.push(("New", last_off));
                        break;
                    }
                }
                ConntrackStatsAttrs::Invalid(val) => {
                    if last_off == offset {
                        stack.push(("Invalid", last_off));
                        break;
                    }
                }
                ConntrackStatsAttrs::Ignore(val) => {
                    if last_off == offset {
                        stack.push(("Ignore", last_off));
                        break;
                    }
                }
                ConntrackStatsAttrs::Delete(val) => {
                    if last_off == offset {
                        stack.push(("Delete", last_off));
                        break;
                    }
                }
                ConntrackStatsAttrs::DeleteList(val) => {
                    if last_off == offset {
                        stack.push(("DeleteList", last_off));
                        break;
                    }
                }
                ConntrackStatsAttrs::Insert(val) => {
                    if last_off == offset {
                        stack.push(("Insert", last_off));
                        break;
                    }
                }
                ConntrackStatsAttrs::InsertFailed(val) => {
                    if last_off == offset {
                        stack.push(("InsertFailed", last_off));
                        break;
                    }
                }
                ConntrackStatsAttrs::Drop(val) => {
                    if last_off == offset {
                        stack.push(("Drop", last_off));
                        break;
                    }
                }
                ConntrackStatsAttrs::EarlyDrop(val) => {
                    if last_off == offset {
                        stack.push(("EarlyDrop", last_off));
                        break;
                    }
                }
                ConntrackStatsAttrs::Error(val) => {
                    if last_off == offset {
                        stack.push(("Error", last_off));
                        break;
                    }
                }
                ConntrackStatsAttrs::SearchRestart(val) => {
                    if last_off == offset {
                        stack.push(("SearchRestart", last_off));
                        break;
                    }
                }
                ConntrackStatsAttrs::ClashResolve(val) => {
                    if last_off == offset {
                        stack.push(("ClashResolve", last_off));
                        break;
                    }
                }
                ConntrackStatsAttrs::ChainToolong(val) => {
                    if last_off == offset {
                        stack.push(("ChainToolong", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ConntrackStatsAttrs", cur));
        }
        (stack, None)
    }
}
pub struct PushCounterAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushCounterAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushCounterAttrs<Prev> {
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
    pub fn push_packets(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 1u16, 8 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_bytes(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 2u16, 8 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_packets_old(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_bytes_old(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 5u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
}
impl<Prev: Rec> Drop for PushCounterAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushTupleProtoAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushTupleProtoAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushTupleProtoAttrs<Prev> {
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
    #[doc = "l4 protocol number"]
    pub fn push_proto_num(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 1u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "l4 source port"]
    pub fn push_proto_src_port(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 2u16, 2 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "l4 source port"]
    pub fn push_proto_dst_port(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 3u16, 2 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "l4 icmp id"]
    pub fn push_proto_icmp_id(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 4u16, 2 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_proto_icmp_type(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 5u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_proto_icmp_code(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 6u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "l4 icmp id"]
    pub fn push_proto_icmpv6_id(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 7u16, 2 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_proto_icmpv6_type(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 8u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_proto_icmpv6_code(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 9u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushTupleProtoAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushTupleIpAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushTupleIpAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushTupleIpAttrs<Prev> {
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
    #[doc = "ipv4 source address"]
    pub fn push_ip_v4_src(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    #[doc = "ipv4 destination address"]
    pub fn push_ip_v4_dst(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    #[doc = "ipv6 source address"]
    pub fn push_ip_v6_src(mut self, value: std::net::Ipv6Addr) -> Self {
        push_header(self.as_rec_mut(), 3u16, 16 as u16);
        self.as_rec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    #[doc = "ipv6 destination address"]
    pub fn push_ip_v6_dst(mut self, value: std::net::Ipv6Addr) -> Self {
        push_header(self.as_rec_mut(), 4u16, 16 as u16);
        self.as_rec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushTupleIpAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushTupleAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushTupleAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushTupleAttrs<Prev> {
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
    #[doc = "conntrack l3 information"]
    pub fn nested_tuple_ip(mut self) -> PushTupleIpAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 1u16);
        PushTupleIpAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "conntrack l4 information"]
    pub fn nested_tuple_proto(mut self) -> PushTupleProtoAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 2u16);
        PushTupleProtoAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "conntrack zone id"]
    pub fn push_tuple_zone(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 3u16, 2 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushTupleAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushProtoinfoTcpAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushProtoinfoTcpAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushProtoinfoTcpAttrs<Prev> {
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
    #[doc = "tcp connection state\nAssociated type: \"NfCtTcpState\" (enum)"]
    pub fn push_tcp_state(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 1u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "window scaling factor in original direction"]
    pub fn push_tcp_wscale_original(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 2u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "window scaling factor in reply direction"]
    pub fn push_tcp_wscale_reply(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 3u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tcp_flags_original(mut self, value: PushNfCtTcpFlagsMask) -> Self {
        push_header(self.as_rec_mut(), 4u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_tcp_flags_reply(mut self, value: PushNfCtTcpFlagsMask) -> Self {
        push_header(self.as_rec_mut(), 5u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
}
impl<Prev: Rec> Drop for PushProtoinfoTcpAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushProtoinfoDccpAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushProtoinfoDccpAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushProtoinfoDccpAttrs<Prev> {
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
    #[doc = "dccp connection state"]
    pub fn push_dccp_state(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 1u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dccp_role(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 2u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dccp_handshake_seq(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 3u16, 8 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_dccp_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 4u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
}
impl<Prev: Rec> Drop for PushProtoinfoDccpAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushProtoinfoSctpAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushProtoinfoSctpAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushProtoinfoSctpAttrs<Prev> {
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
    #[doc = "sctp connection state\nAssociated type: \"NfCtSctpState\" (enum)"]
    pub fn push_sctp_state(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 1u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_vtag_original(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_vtag_reply(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushProtoinfoSctpAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushProtoinfoAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushProtoinfoAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushProtoinfoAttrs<Prev> {
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
    #[doc = "conntrack tcp state information"]
    pub fn nested_protoinfo_tcp(mut self) -> PushProtoinfoTcpAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 1u16);
        PushProtoinfoTcpAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "conntrack dccp state information"]
    pub fn nested_protoinfo_dccp(mut self) -> PushProtoinfoDccpAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 2u16);
        PushProtoinfoDccpAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "conntrack sctp state information"]
    pub fn nested_protoinfo_sctp(mut self) -> PushProtoinfoSctpAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 3u16);
        PushProtoinfoSctpAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushProtoinfoAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushHelpAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushHelpAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushHelpAttrs<Prev> {
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
    #[doc = "helper name"]
    pub fn push_help_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "helper name"]
    pub fn push_help_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
}
impl<Prev: Rec> Drop for PushHelpAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushNatProtoAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushNatProtoAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushNatProtoAttrs<Prev> {
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
    pub fn push_nat_port_min(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 1u16, 2 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_nat_port_max(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 2u16, 2 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushNatProtoAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushNatAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushNatAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushNatAttrs<Prev> {
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
    pub fn push_nat_v4_minip(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_nat_v4_maxip(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_nat_v6_minip(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 3u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_nat_v6_maxip(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 4u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn nested_nat_proto(mut self) -> PushNatProtoAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 5u16);
        PushNatProtoAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushNatAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSeqadjAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushSeqadjAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushSeqadjAttrs<Prev> {
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
    pub fn push_correction_pos(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_offset_before(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_offset_after(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushSeqadjAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSecctxAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushSecctxAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushSecctxAttrs<Prev> {
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
    pub fn push_secctx_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_secctx_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
}
impl<Prev: Rec> Drop for PushSecctxAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSynproxyAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushSynproxyAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushSynproxyAttrs<Prev> {
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
    pub fn push_isn(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_its(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_tsoff(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushSynproxyAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushConntrackAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushConntrackAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushConntrackAttrs<Prev> {
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
    #[doc = "conntrack l3+l4 protocol information, original direction"]
    pub fn nested_tuple_orig(mut self) -> PushTupleAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 1u16);
        PushTupleAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "conntrack l3+l4 protocol information, reply direction"]
    pub fn nested_tuple_reply(mut self) -> PushTupleAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 2u16);
        PushTupleAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "conntrack flag bits\nAssociated type: \"NfCtStatus\" (1 bit per enumeration)"]
    pub fn push_status(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn nested_protoinfo(mut self) -> PushProtoinfoAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 4u16);
        PushProtoinfoAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_help(mut self) -> PushHelpAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 5u16);
        PushHelpAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_nat_src(mut self) -> PushNatAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 6u16);
        PushNatAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_timeout(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 7u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_mark(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn nested_counters_orig(mut self) -> PushCounterAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 9u16);
        PushCounterAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_counters_reply(mut self) -> PushCounterAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 10u16);
        PushCounterAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_use(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 11u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 12u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn nested_nat_dst(mut self) -> PushNatAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 13u16);
        PushNatAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_tuple_master(mut self) -> PushTupleAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 14u16);
        PushTupleAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_seq_adj_orig(mut self) -> PushSeqadjAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 15u16);
        PushSeqadjAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_seq_adj_reply(mut self) -> PushSeqadjAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 16u16);
        PushSeqadjAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "obsolete"]
    pub fn push_secmark(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 17u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    #[doc = "conntrack zone id"]
    pub fn push_zone(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 18u16, 2 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn nested_secctx(mut self) -> PushSecctxAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 19u16);
        PushSecctxAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_timestamp(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 20u16, 8 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_mark_mask(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 21u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_labels(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 22u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_labels_mask(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 23u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn nested_synproxy(mut self) -> PushSynproxyAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 24u16);
        PushSynproxyAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_filter(mut self) -> PushTupleAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 25u16);
        PushTupleAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "conntrack flag bits to change\nAssociated type: \"NfCtStatus\" (1 bit per enumeration)"]
    pub fn push_status_mask(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 26u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_timestamp_event(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 27u16, 8 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushConntrackAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushConntrackStatsAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushConntrackStatsAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushConntrackStatsAttrs<Prev> {
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
    #[doc = "obsolete"]
    pub fn push_searched(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_found(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "obsolete"]
    pub fn push_new(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "obsolete"]
    pub fn push_invalid(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "obsolete"]
    pub fn push_ignore(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 5u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "obsolete"]
    pub fn push_delete(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 6u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "obsolete"]
    pub fn push_delete_list(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 7u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_insert(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_insert_failed(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 9u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_drop(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 10u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_early_drop(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 11u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_error(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 12u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_search_restart(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 13u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_clash_resolve(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 14u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_chain_toolong(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 15u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushConntrackStatsAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[derive(Clone)]
pub struct PushNfgenmsg {
    pub(crate) buf: [u8; 4usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for PushNfgenmsg {
    fn default() -> Self {
        Self { buf: [0u8; 4usize] }
    }
}
impl PushNfgenmsg {
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
    pub fn nfgen_family(&self) -> u8 {
        parse_u8(&self.buf[0usize..1usize]).unwrap()
    }
    pub fn set_nfgen_family(&mut self, value: u8) {
        self.buf[0usize..1usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn version(&self) -> u8 {
        parse_u8(&self.buf[1usize..2usize]).unwrap()
    }
    pub fn set_version(&mut self, value: u8) {
        self.buf[1usize..2usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn res_id(&self) -> u16 {
        parse_be_u16(&self.buf[2usize..4usize]).unwrap()
    }
    pub fn set_res_id(&mut self, value: u16) {
        self.buf[2usize..4usize].copy_from_slice(&value.to_be_bytes())
    }
}
impl std::fmt::Debug for PushNfgenmsg {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Nfgenmsg")
            .field("nfgen_family", &self.nfgen_family())
            .field("version", &self.version())
            .field("res_id", &self.res_id())
            .finish()
    }
}
#[derive(Clone)]
pub struct PushNfCtTcpFlagsMask {
    pub(crate) buf: [u8; 2usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for PushNfCtTcpFlagsMask {
    fn default() -> Self {
        Self { buf: [0u8; 2usize] }
    }
}
impl PushNfCtTcpFlagsMask {
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
        2usize
    }
    #[doc = "Associated type: \"NfCtTcpFlags\" (1 bit per enumeration)"]
    pub fn flags(&self) -> u8 {
        parse_u8(&self.buf[0usize..1usize]).unwrap()
    }
    #[doc = "Associated type: \"NfCtTcpFlags\" (1 bit per enumeration)"]
    pub fn set_flags(&mut self, value: u8) {
        self.buf[0usize..1usize].copy_from_slice(&value.to_ne_bytes())
    }
    #[doc = "Associated type: \"NfCtTcpFlags\" (1 bit per enumeration)"]
    pub fn mask(&self) -> u8 {
        parse_u8(&self.buf[1usize..2usize]).unwrap()
    }
    #[doc = "Associated type: \"NfCtTcpFlags\" (1 bit per enumeration)"]
    pub fn set_mask(&mut self, value: u8) {
        self.buf[1usize..2usize].copy_from_slice(&value.to_ne_bytes())
    }
}
impl std::fmt::Debug for PushNfCtTcpFlagsMask {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("NfCtTcpFlagsMask")
            .field("flags", &self.flags())
            .field("mask", &self.mask())
            .finish()
    }
}
#[doc = "get / dump entries"]
pub struct PushOpGetDumpRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetDumpRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetDumpRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushNfgenmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushNfgenmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    #[doc = "conntrack flag bits\nAssociated type: \"NfCtStatus\" (1 bit per enumeration)"]
    pub fn push_status(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_mark(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "conntrack zone id"]
    pub fn push_zone(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 18u16, 2 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn nested_filter(mut self) -> PushTupleAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 25u16);
        PushTupleAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushOpGetDumpRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "get / dump entries"]
#[derive(Clone)]
pub enum OpGetDumpRequest<'a> {
    #[doc = "conntrack flag bits\nAssociated type: \"NfCtStatus\" (1 bit per enumeration)"]
    Status(u32),
    Mark(u32),
    #[doc = "conntrack zone id"]
    Zone(u16),
    Filter(IterableTupleAttrs<'a>),
}
impl<'a> IterableOpGetDumpRequest<'a> {
    #[doc = "conntrack flag bits\nAssociated type: \"NfCtStatus\" (1 bit per enumeration)"]
    pub fn get_status(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDumpRequest::Status(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDumpRequest",
            "Status",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDumpRequest::Mark(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDumpRequest",
            "Mark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "conntrack zone id"]
    pub fn get_zone(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDumpRequest::Zone(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDumpRequest",
            "Zone",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_filter(&self) -> Result<IterableTupleAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDumpRequest::Filter(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDumpRequest",
            "Filter",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpGetDumpRequest<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> (PushNfgenmsg, IterableOpGetDumpRequest<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushNfgenmsg::len()));
        (
            PushNfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableOpGetDumpRequest::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        ConntrackAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetDumpRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetDumpRequest<'a> {
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
impl<'a> Iterator for IterableOpGetDumpRequest<'a> {
    type Item = Result<OpGetDumpRequest<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                3u16 => OpGetDumpRequest::Status({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpGetDumpRequest::Mark({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => OpGetDumpRequest::Zone({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                25u16 => OpGetDumpRequest::Filter({
                    let res = Some(IterableTupleAttrs::with_loc(next, self.orig_loc));
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
            "OpGetDumpRequest",
            r#type.and_then(|t| OpGetDumpRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpGetDumpRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetDumpRequest");
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
                OpGetDumpRequest::Status(val) => {
                    fmt.field("Status", &FormatFlags(val.into(), NfCtStatus::from_value))
                }
                OpGetDumpRequest::Mark(val) => fmt.field("Mark", &val),
                OpGetDumpRequest::Zone(val) => fmt.field("Zone", &val),
                OpGetDumpRequest::Filter(val) => fmt.field("Filter", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpGetDumpRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushNfgenmsg::len() {
            stack.push(("OpGetDumpRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetDumpRequest::attr_from_type(t)),
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
                OpGetDumpRequest::Status(val) => {
                    if last_off == offset {
                        stack.push(("Status", last_off));
                        break;
                    }
                }
                OpGetDumpRequest::Mark(val) => {
                    if last_off == offset {
                        stack.push(("Mark", last_off));
                        break;
                    }
                }
                OpGetDumpRequest::Zone(val) => {
                    if last_off == offset {
                        stack.push(("Zone", last_off));
                        break;
                    }
                }
                OpGetDumpRequest::Filter(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetDumpRequest", cur));
        }
        (stack, missing)
    }
}
#[doc = "get / dump entries"]
pub struct PushOpGetDumpReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetDumpReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetDumpReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushNfgenmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushNfgenmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    #[doc = "conntrack l3+l4 protocol information, original direction"]
    pub fn nested_tuple_orig(mut self) -> PushTupleAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 1u16);
        PushTupleAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "conntrack l3+l4 protocol information, reply direction"]
    pub fn nested_tuple_reply(mut self) -> PushTupleAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 2u16);
        PushTupleAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "conntrack flag bits\nAssociated type: \"NfCtStatus\" (1 bit per enumeration)"]
    pub fn push_status(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn nested_protoinfo(mut self) -> PushProtoinfoAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 4u16);
        PushProtoinfoAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_help(mut self) -> PushHelpAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 5u16);
        PushHelpAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_nat_src(mut self) -> PushNatAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 6u16);
        PushNatAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_timeout(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 7u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_mark(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn nested_counters_orig(mut self) -> PushCounterAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 9u16);
        PushCounterAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_counters_reply(mut self) -> PushCounterAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 10u16);
        PushCounterAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_use(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 11u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 12u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn nested_nat_dst(mut self) -> PushNatAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 13u16);
        PushNatAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_tuple_master(mut self) -> PushTupleAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 14u16);
        PushTupleAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_seq_adj_orig(mut self) -> PushSeqadjAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 15u16);
        PushSeqadjAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_seq_adj_reply(mut self) -> PushSeqadjAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 16u16);
        PushSeqadjAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "conntrack zone id"]
    pub fn push_zone(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 18u16, 2 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn nested_secctx(mut self) -> PushSecctxAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 19u16);
        PushSecctxAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_labels(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 22u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn nested_synproxy(mut self) -> PushSynproxyAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 24u16);
        PushSynproxyAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushOpGetDumpReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "get / dump entries"]
#[derive(Clone)]
pub enum OpGetDumpReply<'a> {
    #[doc = "conntrack l3+l4 protocol information, original direction"]
    TupleOrig(IterableTupleAttrs<'a>),
    #[doc = "conntrack l3+l4 protocol information, reply direction"]
    TupleReply(IterableTupleAttrs<'a>),
    #[doc = "conntrack flag bits\nAssociated type: \"NfCtStatus\" (1 bit per enumeration)"]
    Status(u32),
    Protoinfo(IterableProtoinfoAttrs<'a>),
    Help(IterableHelpAttrs<'a>),
    NatSrc(IterableNatAttrs<'a>),
    Timeout(u32),
    Mark(u32),
    CountersOrig(IterableCounterAttrs<'a>),
    CountersReply(IterableCounterAttrs<'a>),
    Use(u32),
    Id(u32),
    NatDst(IterableNatAttrs<'a>),
    TupleMaster(IterableTupleAttrs<'a>),
    SeqAdjOrig(IterableSeqadjAttrs<'a>),
    SeqAdjReply(IterableSeqadjAttrs<'a>),
    #[doc = "conntrack zone id"]
    Zone(u16),
    Secctx(IterableSecctxAttrs<'a>),
    Labels(&'a [u8]),
    Synproxy(IterableSynproxyAttrs<'a>),
}
impl<'a> IterableOpGetDumpReply<'a> {
    #[doc = "conntrack l3+l4 protocol information, original direction"]
    pub fn get_tuple_orig(&self) -> Result<IterableTupleAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDumpReply::TupleOrig(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDumpReply",
            "TupleOrig",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "conntrack l3+l4 protocol information, reply direction"]
    pub fn get_tuple_reply(&self) -> Result<IterableTupleAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDumpReply::TupleReply(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDumpReply",
            "TupleReply",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "conntrack flag bits\nAssociated type: \"NfCtStatus\" (1 bit per enumeration)"]
    pub fn get_status(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDumpReply::Status(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDumpReply",
            "Status",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_protoinfo(&self) -> Result<IterableProtoinfoAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDumpReply::Protoinfo(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDumpReply",
            "Protoinfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_help(&self) -> Result<IterableHelpAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDumpReply::Help(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDumpReply",
            "Help",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nat_src(&self) -> Result<IterableNatAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDumpReply::NatSrc(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDumpReply",
            "NatSrc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_timeout(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDumpReply::Timeout(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDumpReply",
            "Timeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDumpReply::Mark(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDumpReply",
            "Mark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_counters_orig(&self) -> Result<IterableCounterAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDumpReply::CountersOrig(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDumpReply",
            "CountersOrig",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_counters_reply(&self) -> Result<IterableCounterAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDumpReply::CountersReply(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDumpReply",
            "CountersReply",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_use(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDumpReply::Use(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDumpReply",
            "Use",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDumpReply::Id(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDumpReply",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nat_dst(&self) -> Result<IterableNatAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDumpReply::NatDst(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDumpReply",
            "NatDst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tuple_master(&self) -> Result<IterableTupleAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDumpReply::TupleMaster(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDumpReply",
            "TupleMaster",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_seq_adj_orig(&self) -> Result<IterableSeqadjAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDumpReply::SeqAdjOrig(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDumpReply",
            "SeqAdjOrig",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_seq_adj_reply(&self) -> Result<IterableSeqadjAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDumpReply::SeqAdjReply(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDumpReply",
            "SeqAdjReply",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "conntrack zone id"]
    pub fn get_zone(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDumpReply::Zone(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDumpReply",
            "Zone",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_secctx(&self) -> Result<IterableSecctxAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDumpReply::Secctx(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDumpReply",
            "Secctx",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_labels(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDumpReply::Labels(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDumpReply",
            "Labels",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_synproxy(&self) -> Result<IterableSynproxyAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDumpReply::Synproxy(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDumpReply",
            "Synproxy",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpGetDumpReply<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> (PushNfgenmsg, IterableOpGetDumpReply<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushNfgenmsg::len()));
        (
            PushNfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableOpGetDumpReply::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        ConntrackAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetDumpReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetDumpReply<'a> {
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
impl<'a> Iterator for IterableOpGetDumpReply<'a> {
    type Item = Result<OpGetDumpReply<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpGetDumpReply::TupleOrig({
                    let res = Some(IterableTupleAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpGetDumpReply::TupleReply({
                    let res = Some(IterableTupleAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpGetDumpReply::Status({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpGetDumpReply::Protoinfo({
                    let res = Some(IterableProtoinfoAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpGetDumpReply::Help({
                    let res = Some(IterableHelpAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpGetDumpReply::NatSrc({
                    let res = Some(IterableNatAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => OpGetDumpReply::Timeout({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpGetDumpReply::Mark({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => OpGetDumpReply::CountersOrig({
                    let res = Some(IterableCounterAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => OpGetDumpReply::CountersReply({
                    let res = Some(IterableCounterAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => OpGetDumpReply::Use({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => OpGetDumpReply::Id({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => OpGetDumpReply::NatDst({
                    let res = Some(IterableNatAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => OpGetDumpReply::TupleMaster({
                    let res = Some(IterableTupleAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => OpGetDumpReply::SeqAdjOrig({
                    let res = Some(IterableSeqadjAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => OpGetDumpReply::SeqAdjReply({
                    let res = Some(IterableSeqadjAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => OpGetDumpReply::Zone({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => OpGetDumpReply::Secctx({
                    let res = Some(IterableSecctxAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => OpGetDumpReply::Labels({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => OpGetDumpReply::Synproxy({
                    let res = Some(IterableSynproxyAttrs::with_loc(next, self.orig_loc));
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
            "OpGetDumpReply",
            r#type.and_then(|t| OpGetDumpReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpGetDumpReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetDumpReply");
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
                OpGetDumpReply::TupleOrig(val) => fmt.field("TupleOrig", &val),
                OpGetDumpReply::TupleReply(val) => fmt.field("TupleReply", &val),
                OpGetDumpReply::Status(val) => {
                    fmt.field("Status", &FormatFlags(val.into(), NfCtStatus::from_value))
                }
                OpGetDumpReply::Protoinfo(val) => fmt.field("Protoinfo", &val),
                OpGetDumpReply::Help(val) => fmt.field("Help", &val),
                OpGetDumpReply::NatSrc(val) => fmt.field("NatSrc", &val),
                OpGetDumpReply::Timeout(val) => fmt.field("Timeout", &val),
                OpGetDumpReply::Mark(val) => fmt.field("Mark", &val),
                OpGetDumpReply::CountersOrig(val) => fmt.field("CountersOrig", &val),
                OpGetDumpReply::CountersReply(val) => fmt.field("CountersReply", &val),
                OpGetDumpReply::Use(val) => fmt.field("Use", &val),
                OpGetDumpReply::Id(val) => fmt.field("Id", &val),
                OpGetDumpReply::NatDst(val) => fmt.field("NatDst", &val),
                OpGetDumpReply::TupleMaster(val) => fmt.field("TupleMaster", &val),
                OpGetDumpReply::SeqAdjOrig(val) => fmt.field("SeqAdjOrig", &val),
                OpGetDumpReply::SeqAdjReply(val) => fmt.field("SeqAdjReply", &val),
                OpGetDumpReply::Zone(val) => fmt.field("Zone", &val),
                OpGetDumpReply::Secctx(val) => fmt.field("Secctx", &val),
                OpGetDumpReply::Labels(val) => fmt.field("Labels", &val),
                OpGetDumpReply::Synproxy(val) => fmt.field("Synproxy", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpGetDumpReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushNfgenmsg::len() {
            stack.push(("OpGetDumpReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetDumpReply::attr_from_type(t)),
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
                OpGetDumpReply::TupleOrig(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDumpReply::TupleReply(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDumpReply::Status(val) => {
                    if last_off == offset {
                        stack.push(("Status", last_off));
                        break;
                    }
                }
                OpGetDumpReply::Protoinfo(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDumpReply::Help(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDumpReply::NatSrc(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDumpReply::Timeout(val) => {
                    if last_off == offset {
                        stack.push(("Timeout", last_off));
                        break;
                    }
                }
                OpGetDumpReply::Mark(val) => {
                    if last_off == offset {
                        stack.push(("Mark", last_off));
                        break;
                    }
                }
                OpGetDumpReply::CountersOrig(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDumpReply::CountersReply(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDumpReply::Use(val) => {
                    if last_off == offset {
                        stack.push(("Use", last_off));
                        break;
                    }
                }
                OpGetDumpReply::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                OpGetDumpReply::NatDst(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDumpReply::TupleMaster(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDumpReply::SeqAdjOrig(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDumpReply::SeqAdjReply(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDumpReply::Zone(val) => {
                    if last_off == offset {
                        stack.push(("Zone", last_off));
                        break;
                    }
                }
                OpGetDumpReply::Secctx(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDumpReply::Labels(val) => {
                    if last_off == offset {
                        stack.push(("Labels", last_off));
                        break;
                    }
                }
                OpGetDumpReply::Synproxy(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetDumpReply", cur));
        }
        (stack, missing)
    }
}
#[derive(Debug)]
pub struct RequestOpGetDumpRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpGetDumpRequest<'r> {
    pub fn new(mut request: Request<'r>, header: &PushNfgenmsg) -> Self {
        PushOpGetDumpRequest::write_header(&mut request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode(&mut self) -> PushOpGetDumpRequest<&mut Vec<u8>> {
        PushOpGetDumpRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpGetDumpRequest<RequestBuf<'r>> {
        PushOpGetDumpRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpGetDumpRequest<'_> {
    type ReplyType<'buf> = (PushNfgenmsg, IterableOpGetDumpReply<'buf>);
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 257u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpGetDumpReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpGetDumpRequest::new(buf)
            .1
            .lookup_attr(offset, missing_type)
    }
}
#[doc = "get / dump entries"]
pub struct PushOpGetDoRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetDoRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushNfgenmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushNfgenmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    #[doc = "conntrack l3+l4 protocol information, original direction"]
    pub fn nested_tuple_orig(mut self) -> PushTupleAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 1u16);
        PushTupleAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "conntrack l3+l4 protocol information, reply direction"]
    pub fn nested_tuple_reply(mut self) -> PushTupleAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 2u16);
        PushTupleAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "conntrack zone id"]
    pub fn push_zone(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 18u16, 2 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpGetDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "get / dump entries"]
#[derive(Clone)]
pub enum OpGetDoRequest<'a> {
    #[doc = "conntrack l3+l4 protocol information, original direction"]
    TupleOrig(IterableTupleAttrs<'a>),
    #[doc = "conntrack l3+l4 protocol information, reply direction"]
    TupleReply(IterableTupleAttrs<'a>),
    #[doc = "conntrack zone id"]
    Zone(u16),
}
impl<'a> IterableOpGetDoRequest<'a> {
    #[doc = "conntrack l3+l4 protocol information, original direction"]
    pub fn get_tuple_orig(&self) -> Result<IterableTupleAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDoRequest::TupleOrig(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDoRequest",
            "TupleOrig",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "conntrack l3+l4 protocol information, reply direction"]
    pub fn get_tuple_reply(&self) -> Result<IterableTupleAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDoRequest::TupleReply(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDoRequest",
            "TupleReply",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "conntrack zone id"]
    pub fn get_zone(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDoRequest::Zone(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDoRequest",
            "Zone",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpGetDoRequest<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> (PushNfgenmsg, IterableOpGetDoRequest<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushNfgenmsg::len()));
        (
            PushNfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableOpGetDoRequest::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        ConntrackAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetDoRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetDoRequest<'a> {
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
impl<'a> Iterator for IterableOpGetDoRequest<'a> {
    type Item = Result<OpGetDoRequest<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpGetDoRequest::TupleOrig({
                    let res = Some(IterableTupleAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpGetDoRequest::TupleReply({
                    let res = Some(IterableTupleAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => OpGetDoRequest::Zone({
                    let res = parse_be_u16(next);
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
            "OpGetDoRequest",
            r#type.and_then(|t| OpGetDoRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpGetDoRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetDoRequest");
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
                OpGetDoRequest::TupleOrig(val) => fmt.field("TupleOrig", &val),
                OpGetDoRequest::TupleReply(val) => fmt.field("TupleReply", &val),
                OpGetDoRequest::Zone(val) => fmt.field("Zone", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpGetDoRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushNfgenmsg::len() {
            stack.push(("OpGetDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetDoRequest::attr_from_type(t)),
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
                OpGetDoRequest::TupleOrig(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDoRequest::TupleReply(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDoRequest::Zone(val) => {
                    if last_off == offset {
                        stack.push(("Zone", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetDoRequest", cur));
        }
        (stack, missing)
    }
}
#[doc = "get / dump entries"]
pub struct PushOpGetDoReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetDoReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushNfgenmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushNfgenmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    #[doc = "conntrack l3+l4 protocol information, original direction"]
    pub fn nested_tuple_orig(mut self) -> PushTupleAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 1u16);
        PushTupleAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "conntrack l3+l4 protocol information, reply direction"]
    pub fn nested_tuple_reply(mut self) -> PushTupleAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 2u16);
        PushTupleAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "conntrack flag bits\nAssociated type: \"NfCtStatus\" (1 bit per enumeration)"]
    pub fn push_status(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn nested_protoinfo(mut self) -> PushProtoinfoAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 4u16);
        PushProtoinfoAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_help(mut self) -> PushHelpAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 5u16);
        PushHelpAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_nat_src(mut self) -> PushNatAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 6u16);
        PushNatAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_timeout(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 7u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_mark(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn nested_counters_orig(mut self) -> PushCounterAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 9u16);
        PushCounterAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_counters_reply(mut self) -> PushCounterAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 10u16);
        PushCounterAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_use(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 11u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 12u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn nested_nat_dst(mut self) -> PushNatAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 13u16);
        PushNatAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_tuple_master(mut self) -> PushTupleAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 14u16);
        PushTupleAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_seq_adj_orig(mut self) -> PushSeqadjAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 15u16);
        PushSeqadjAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_seq_adj_reply(mut self) -> PushSeqadjAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 16u16);
        PushSeqadjAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "conntrack zone id"]
    pub fn push_zone(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 18u16, 2 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn nested_secctx(mut self) -> PushSecctxAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 19u16);
        PushSecctxAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_labels(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 22u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn nested_synproxy(mut self) -> PushSynproxyAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 24u16);
        PushSynproxyAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushOpGetDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "get / dump entries"]
#[derive(Clone)]
pub enum OpGetDoReply<'a> {
    #[doc = "conntrack l3+l4 protocol information, original direction"]
    TupleOrig(IterableTupleAttrs<'a>),
    #[doc = "conntrack l3+l4 protocol information, reply direction"]
    TupleReply(IterableTupleAttrs<'a>),
    #[doc = "conntrack flag bits\nAssociated type: \"NfCtStatus\" (1 bit per enumeration)"]
    Status(u32),
    Protoinfo(IterableProtoinfoAttrs<'a>),
    Help(IterableHelpAttrs<'a>),
    NatSrc(IterableNatAttrs<'a>),
    Timeout(u32),
    Mark(u32),
    CountersOrig(IterableCounterAttrs<'a>),
    CountersReply(IterableCounterAttrs<'a>),
    Use(u32),
    Id(u32),
    NatDst(IterableNatAttrs<'a>),
    TupleMaster(IterableTupleAttrs<'a>),
    SeqAdjOrig(IterableSeqadjAttrs<'a>),
    SeqAdjReply(IterableSeqadjAttrs<'a>),
    #[doc = "conntrack zone id"]
    Zone(u16),
    Secctx(IterableSecctxAttrs<'a>),
    Labels(&'a [u8]),
    Synproxy(IterableSynproxyAttrs<'a>),
}
impl<'a> IterableOpGetDoReply<'a> {
    #[doc = "conntrack l3+l4 protocol information, original direction"]
    pub fn get_tuple_orig(&self) -> Result<IterableTupleAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDoReply::TupleOrig(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDoReply",
            "TupleOrig",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "conntrack l3+l4 protocol information, reply direction"]
    pub fn get_tuple_reply(&self) -> Result<IterableTupleAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDoReply::TupleReply(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDoReply",
            "TupleReply",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "conntrack flag bits\nAssociated type: \"NfCtStatus\" (1 bit per enumeration)"]
    pub fn get_status(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDoReply::Status(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDoReply",
            "Status",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_protoinfo(&self) -> Result<IterableProtoinfoAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDoReply::Protoinfo(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDoReply",
            "Protoinfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_help(&self) -> Result<IterableHelpAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDoReply::Help(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDoReply",
            "Help",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nat_src(&self) -> Result<IterableNatAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDoReply::NatSrc(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDoReply",
            "NatSrc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_timeout(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDoReply::Timeout(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDoReply",
            "Timeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDoReply::Mark(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDoReply",
            "Mark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_counters_orig(&self) -> Result<IterableCounterAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDoReply::CountersOrig(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDoReply",
            "CountersOrig",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_counters_reply(&self) -> Result<IterableCounterAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDoReply::CountersReply(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDoReply",
            "CountersReply",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_use(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDoReply::Use(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDoReply",
            "Use",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDoReply::Id(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDoReply",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nat_dst(&self) -> Result<IterableNatAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDoReply::NatDst(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDoReply",
            "NatDst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tuple_master(&self) -> Result<IterableTupleAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDoReply::TupleMaster(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDoReply",
            "TupleMaster",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_seq_adj_orig(&self) -> Result<IterableSeqadjAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDoReply::SeqAdjOrig(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDoReply",
            "SeqAdjOrig",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_seq_adj_reply(&self) -> Result<IterableSeqadjAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDoReply::SeqAdjReply(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDoReply",
            "SeqAdjReply",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "conntrack zone id"]
    pub fn get_zone(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDoReply::Zone(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDoReply",
            "Zone",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_secctx(&self) -> Result<IterableSecctxAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDoReply::Secctx(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDoReply",
            "Secctx",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_labels(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDoReply::Labels(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDoReply",
            "Labels",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_synproxy(&self) -> Result<IterableSynproxyAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDoReply::Synproxy(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetDoReply",
            "Synproxy",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpGetDoReply<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> (PushNfgenmsg, IterableOpGetDoReply<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushNfgenmsg::len()));
        (
            PushNfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableOpGetDoReply::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        ConntrackAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetDoReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetDoReply<'a> {
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
impl<'a> Iterator for IterableOpGetDoReply<'a> {
    type Item = Result<OpGetDoReply<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpGetDoReply::TupleOrig({
                    let res = Some(IterableTupleAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpGetDoReply::TupleReply({
                    let res = Some(IterableTupleAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpGetDoReply::Status({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpGetDoReply::Protoinfo({
                    let res = Some(IterableProtoinfoAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpGetDoReply::Help({
                    let res = Some(IterableHelpAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpGetDoReply::NatSrc({
                    let res = Some(IterableNatAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => OpGetDoReply::Timeout({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpGetDoReply::Mark({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => OpGetDoReply::CountersOrig({
                    let res = Some(IterableCounterAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => OpGetDoReply::CountersReply({
                    let res = Some(IterableCounterAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => OpGetDoReply::Use({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => OpGetDoReply::Id({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => OpGetDoReply::NatDst({
                    let res = Some(IterableNatAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => OpGetDoReply::TupleMaster({
                    let res = Some(IterableTupleAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => OpGetDoReply::SeqAdjOrig({
                    let res = Some(IterableSeqadjAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => OpGetDoReply::SeqAdjReply({
                    let res = Some(IterableSeqadjAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => OpGetDoReply::Zone({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => OpGetDoReply::Secctx({
                    let res = Some(IterableSecctxAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => OpGetDoReply::Labels({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => OpGetDoReply::Synproxy({
                    let res = Some(IterableSynproxyAttrs::with_loc(next, self.orig_loc));
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
            "OpGetDoReply",
            r#type.and_then(|t| OpGetDoReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpGetDoReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetDoReply");
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
                OpGetDoReply::TupleOrig(val) => fmt.field("TupleOrig", &val),
                OpGetDoReply::TupleReply(val) => fmt.field("TupleReply", &val),
                OpGetDoReply::Status(val) => {
                    fmt.field("Status", &FormatFlags(val.into(), NfCtStatus::from_value))
                }
                OpGetDoReply::Protoinfo(val) => fmt.field("Protoinfo", &val),
                OpGetDoReply::Help(val) => fmt.field("Help", &val),
                OpGetDoReply::NatSrc(val) => fmt.field("NatSrc", &val),
                OpGetDoReply::Timeout(val) => fmt.field("Timeout", &val),
                OpGetDoReply::Mark(val) => fmt.field("Mark", &val),
                OpGetDoReply::CountersOrig(val) => fmt.field("CountersOrig", &val),
                OpGetDoReply::CountersReply(val) => fmt.field("CountersReply", &val),
                OpGetDoReply::Use(val) => fmt.field("Use", &val),
                OpGetDoReply::Id(val) => fmt.field("Id", &val),
                OpGetDoReply::NatDst(val) => fmt.field("NatDst", &val),
                OpGetDoReply::TupleMaster(val) => fmt.field("TupleMaster", &val),
                OpGetDoReply::SeqAdjOrig(val) => fmt.field("SeqAdjOrig", &val),
                OpGetDoReply::SeqAdjReply(val) => fmt.field("SeqAdjReply", &val),
                OpGetDoReply::Zone(val) => fmt.field("Zone", &val),
                OpGetDoReply::Secctx(val) => fmt.field("Secctx", &val),
                OpGetDoReply::Labels(val) => fmt.field("Labels", &val),
                OpGetDoReply::Synproxy(val) => fmt.field("Synproxy", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpGetDoReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushNfgenmsg::len() {
            stack.push(("OpGetDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetDoReply::attr_from_type(t)),
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
                OpGetDoReply::TupleOrig(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDoReply::TupleReply(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDoReply::Status(val) => {
                    if last_off == offset {
                        stack.push(("Status", last_off));
                        break;
                    }
                }
                OpGetDoReply::Protoinfo(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDoReply::Help(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDoReply::NatSrc(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDoReply::Timeout(val) => {
                    if last_off == offset {
                        stack.push(("Timeout", last_off));
                        break;
                    }
                }
                OpGetDoReply::Mark(val) => {
                    if last_off == offset {
                        stack.push(("Mark", last_off));
                        break;
                    }
                }
                OpGetDoReply::CountersOrig(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDoReply::CountersReply(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDoReply::Use(val) => {
                    if last_off == offset {
                        stack.push(("Use", last_off));
                        break;
                    }
                }
                OpGetDoReply::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                OpGetDoReply::NatDst(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDoReply::TupleMaster(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDoReply::SeqAdjOrig(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDoReply::SeqAdjReply(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDoReply::Zone(val) => {
                    if last_off == offset {
                        stack.push(("Zone", last_off));
                        break;
                    }
                }
                OpGetDoReply::Secctx(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetDoReply::Labels(val) => {
                    if last_off == offset {
                        stack.push(("Labels", last_off));
                        break;
                    }
                }
                OpGetDoReply::Synproxy(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetDoReply", cur));
        }
        (stack, missing)
    }
}
#[derive(Debug)]
pub struct RequestOpGetDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpGetDoRequest<'r> {
    pub fn new(mut request: Request<'r>, header: &PushNfgenmsg) -> Self {
        PushOpGetDoRequest::write_header(&mut request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpGetDoRequest<&mut Vec<u8>> {
        PushOpGetDoRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpGetDoRequest<RequestBuf<'r>> {
        PushOpGetDoRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpGetDoRequest<'_> {
    type ReplyType<'buf> = (PushNfgenmsg, IterableOpGetDoReply<'buf>);
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 257u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpGetDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpGetDoRequest::new(buf).1.lookup_attr(offset, missing_type)
    }
}
#[doc = "dump pcpu conntrack stats"]
pub struct PushOpGetStatsDumpRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetStatsDumpRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetStatsDumpRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushNfgenmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushNfgenmsg) {
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
impl<Prev: Rec> Drop for PushOpGetStatsDumpRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "dump pcpu conntrack stats"]
#[derive(Clone)]
pub enum OpGetStatsDumpRequest {}
impl<'a> IterableOpGetStatsDumpRequest<'a> {}
impl OpGetStatsDumpRequest {
    pub fn new<'a>(buf: &'a [u8]) -> (PushNfgenmsg, IterableOpGetStatsDumpRequest<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushNfgenmsg::len()));
        (
            PushNfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableOpGetStatsDumpRequest::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        ConntrackStatsAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetStatsDumpRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetStatsDumpRequest<'a> {
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
impl<'a> Iterator for IterableOpGetStatsDumpRequest<'a> {
    type Item = Result<OpGetStatsDumpRequest, ErrorContext>;
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
            "OpGetStatsDumpRequest",
            r#type.and_then(|t| OpGetStatsDumpRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpGetStatsDumpRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetStatsDumpRequest");
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
impl IterableOpGetStatsDumpRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushNfgenmsg::len() {
            stack.push(("OpGetStatsDumpRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetStatsDumpRequest::attr_from_type(t)),
            );
        }
        (stack, None)
    }
}
#[doc = "dump pcpu conntrack stats"]
pub struct PushOpGetStatsDumpReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetStatsDumpReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetStatsDumpReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushNfgenmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushNfgenmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    #[doc = "obsolete"]
    pub fn push_searched(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_found(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_insert(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_insert_failed(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 9u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_drop(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 10u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_early_drop(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 11u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_error(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 12u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_search_restart(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 13u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_clash_resolve(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 14u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_chain_toolong(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 15u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpGetStatsDumpReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "dump pcpu conntrack stats"]
#[derive(Clone)]
pub enum OpGetStatsDumpReply {
    #[doc = "obsolete"]
    Searched(u32),
    Found(u32),
    Insert(u32),
    InsertFailed(u32),
    Drop(u32),
    EarlyDrop(u32),
    Error(u32),
    SearchRestart(u32),
    ClashResolve(u32),
    ChainToolong(u32),
}
impl<'a> IterableOpGetStatsDumpReply<'a> {
    #[doc = "obsolete"]
    pub fn get_searched(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetStatsDumpReply::Searched(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetStatsDumpReply",
            "Searched",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_found(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetStatsDumpReply::Found(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetStatsDumpReply",
            "Found",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_insert(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetStatsDumpReply::Insert(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetStatsDumpReply",
            "Insert",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_insert_failed(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetStatsDumpReply::InsertFailed(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetStatsDumpReply",
            "InsertFailed",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_drop(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetStatsDumpReply::Drop(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetStatsDumpReply",
            "Drop",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_early_drop(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetStatsDumpReply::EarlyDrop(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetStatsDumpReply",
            "EarlyDrop",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_error(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetStatsDumpReply::Error(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetStatsDumpReply",
            "Error",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_search_restart(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetStatsDumpReply::SearchRestart(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetStatsDumpReply",
            "SearchRestart",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_clash_resolve(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetStatsDumpReply::ClashResolve(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetStatsDumpReply",
            "ClashResolve",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_chain_toolong(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetStatsDumpReply::ChainToolong(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetStatsDumpReply",
            "ChainToolong",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpGetStatsDumpReply {
    pub fn new<'a>(buf: &'a [u8]) -> (PushNfgenmsg, IterableOpGetStatsDumpReply<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushNfgenmsg::len()));
        (
            PushNfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableOpGetStatsDumpReply::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        ConntrackStatsAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetStatsDumpReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetStatsDumpReply<'a> {
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
impl<'a> Iterator for IterableOpGetStatsDumpReply<'a> {
    type Item = Result<OpGetStatsDumpReply, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpGetStatsDumpReply::Searched({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpGetStatsDumpReply::Found({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpGetStatsDumpReply::Insert({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => OpGetStatsDumpReply::InsertFailed({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => OpGetStatsDumpReply::Drop({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => OpGetStatsDumpReply::EarlyDrop({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => OpGetStatsDumpReply::Error({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => OpGetStatsDumpReply::SearchRestart({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => OpGetStatsDumpReply::ClashResolve({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => OpGetStatsDumpReply::ChainToolong({
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
            "OpGetStatsDumpReply",
            r#type.and_then(|t| OpGetStatsDumpReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpGetStatsDumpReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetStatsDumpReply");
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
                OpGetStatsDumpReply::Searched(val) => fmt.field("Searched", &val),
                OpGetStatsDumpReply::Found(val) => fmt.field("Found", &val),
                OpGetStatsDumpReply::Insert(val) => fmt.field("Insert", &val),
                OpGetStatsDumpReply::InsertFailed(val) => fmt.field("InsertFailed", &val),
                OpGetStatsDumpReply::Drop(val) => fmt.field("Drop", &val),
                OpGetStatsDumpReply::EarlyDrop(val) => fmt.field("EarlyDrop", &val),
                OpGetStatsDumpReply::Error(val) => fmt.field("Error", &val),
                OpGetStatsDumpReply::SearchRestart(val) => fmt.field("SearchRestart", &val),
                OpGetStatsDumpReply::ClashResolve(val) => fmt.field("ClashResolve", &val),
                OpGetStatsDumpReply::ChainToolong(val) => fmt.field("ChainToolong", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpGetStatsDumpReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushNfgenmsg::len() {
            stack.push(("OpGetStatsDumpReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetStatsDumpReply::attr_from_type(t)),
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
                OpGetStatsDumpReply::Searched(val) => {
                    if last_off == offset {
                        stack.push(("Searched", last_off));
                        break;
                    }
                }
                OpGetStatsDumpReply::Found(val) => {
                    if last_off == offset {
                        stack.push(("Found", last_off));
                        break;
                    }
                }
                OpGetStatsDumpReply::Insert(val) => {
                    if last_off == offset {
                        stack.push(("Insert", last_off));
                        break;
                    }
                }
                OpGetStatsDumpReply::InsertFailed(val) => {
                    if last_off == offset {
                        stack.push(("InsertFailed", last_off));
                        break;
                    }
                }
                OpGetStatsDumpReply::Drop(val) => {
                    if last_off == offset {
                        stack.push(("Drop", last_off));
                        break;
                    }
                }
                OpGetStatsDumpReply::EarlyDrop(val) => {
                    if last_off == offset {
                        stack.push(("EarlyDrop", last_off));
                        break;
                    }
                }
                OpGetStatsDumpReply::Error(val) => {
                    if last_off == offset {
                        stack.push(("Error", last_off));
                        break;
                    }
                }
                OpGetStatsDumpReply::SearchRestart(val) => {
                    if last_off == offset {
                        stack.push(("SearchRestart", last_off));
                        break;
                    }
                }
                OpGetStatsDumpReply::ClashResolve(val) => {
                    if last_off == offset {
                        stack.push(("ClashResolve", last_off));
                        break;
                    }
                }
                OpGetStatsDumpReply::ChainToolong(val) => {
                    if last_off == offset {
                        stack.push(("ChainToolong", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetStatsDumpReply", cur));
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpGetStatsDumpRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpGetStatsDumpRequest<'r> {
    pub fn new(mut request: Request<'r>, header: &PushNfgenmsg) -> Self {
        PushOpGetStatsDumpRequest::write_header(&mut request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode(&mut self) -> PushOpGetStatsDumpRequest<&mut Vec<u8>> {
        PushOpGetStatsDumpRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpGetStatsDumpRequest<RequestBuf<'r>> {
        PushOpGetStatsDumpRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpGetStatsDumpRequest<'_> {
    type ReplyType<'buf> = (PushNfgenmsg, IterableOpGetStatsDumpReply<'buf>);
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 260u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpGetStatsDumpReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpGetStatsDumpRequest::new(buf)
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
    pub fn op_get_dump_request(self, header: &PushNfgenmsg) -> RequestOpGetDumpRequest<'buf> {
        let mut res = RequestOpGetDumpRequest::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-get-dump-request",
            RequestOpGetDumpRequest::lookup,
        );
        res
    }
    pub fn op_get_do_request(self, header: &PushNfgenmsg) -> RequestOpGetDoRequest<'buf> {
        let mut res = RequestOpGetDoRequest::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-get-do-request",
            RequestOpGetDoRequest::lookup,
        );
        res
    }
    pub fn op_get_stats_dump_request(
        self,
        header: &PushNfgenmsg,
    ) -> RequestOpGetStatsDumpRequest<'buf> {
        let mut res = RequestOpGetStatsDumpRequest::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-get-stats-dump-request",
            RequestOpGetStatsDumpRequest::lookup,
        );
        res
    }
}
