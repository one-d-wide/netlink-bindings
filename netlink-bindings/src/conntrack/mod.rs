#![doc = "Netfilter connection tracking subsystem over nfnetlink\n"]
#![allow(clippy::all)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]
#![allow(unreachable_code)]
#![allow(unreachable_patterns)]
use crate::builtin::{BuiltinBitfield32, BuiltinNfgenmsg, Nlmsghdr, PushDummy};
use crate::{
    consts,
    traits::{NetlinkRequest, Protocol},
    utils::*,
};
pub const PROTONAME: &str = "conntrack";
pub const PROTONAME_CSTR: &CStr = c"conntrack";
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
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Nfgenmsg {
    pub nfgen_family: u8,
    pub version: u8,
    pub _res_id_be: u16,
}
#[doc = "Create zero-initialized struct"]
impl Default for Nfgenmsg {
    fn default() -> Self {
        Self::new()
    }
}
impl Nfgenmsg {
    #[doc = "Create zero-initialized struct"]
    pub fn new() -> Self {
        Self::new_from_array([0u8; Self::len()])
    }
    #[doc = "Copy from contents from slice"]
    pub fn new_from_slice(other: &[u8]) -> Option<Self> {
        if other.len() != Self::len() {
            return None;
        }
        let mut buf = [0u8; Self::len()];
        buf.clone_from_slice(other);
        Some(Self::new_from_array(buf))
    }
    #[doc = "Copy from contents from another slice, padding with zeros or truncating when needed"]
    pub fn new_from_zeroed(other: &[u8]) -> Self {
        let mut buf = [0u8; Self::len()];
        let len = buf.len().min(other.len());
        buf[..len].clone_from_slice(&other[..len]);
        Self::new_from_array(buf)
    }
    pub fn new_from_array(buf: [u8; 4usize]) -> Self {
        unsafe { std::mem::transmute(buf) }
    }
    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            let ptr: *const u8 = std::mem::transmute(self as *const Self);
            std::slice::from_raw_parts(ptr, Self::len())
        }
    }
    pub fn from_slice(buf: &[u8]) -> &Self {
        assert!(buf.len() >= Self::len());
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf.as_ptr()) }
    }
    pub fn from_slice_mut(buf: &mut [u8]) -> &mut Self {
        assert!(buf.len() >= Self::len());
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf.as_ptr()) }
    }
    pub fn as_array(&self) -> &[u8; 4usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 4usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 4usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<Nfgenmsg>() == 4usize);
        const _: () = assert!(std::mem::align_of::<Nfgenmsg>() == 2usize);
        4usize
    }
    pub fn res_id(&self) -> u16 {
        u16::from_be(self._res_id_be)
    }
    pub fn set_res_id(&mut self, value: u16) {
        self._res_id_be = value.to_be();
    }
}
impl std::fmt::Debug for Nfgenmsg {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Nfgenmsg")
            .field("nfgen_family", &self.nfgen_family)
            .field("version", &self.version)
            .field("res_id", &self.res_id())
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct NfCtTcpFlagsMask {
    #[doc = "Associated type: [`NfCtTcpFlags`] (1 bit per enumeration)"]
    pub flags: u8,
    #[doc = "Associated type: [`NfCtTcpFlags`] (1 bit per enumeration)"]
    pub mask: u8,
}
#[doc = "Create zero-initialized struct"]
impl Default for NfCtTcpFlagsMask {
    fn default() -> Self {
        Self::new()
    }
}
impl NfCtTcpFlagsMask {
    #[doc = "Create zero-initialized struct"]
    pub fn new() -> Self {
        Self::new_from_array([0u8; Self::len()])
    }
    #[doc = "Copy from contents from slice"]
    pub fn new_from_slice(other: &[u8]) -> Option<Self> {
        if other.len() != Self::len() {
            return None;
        }
        let mut buf = [0u8; Self::len()];
        buf.clone_from_slice(other);
        Some(Self::new_from_array(buf))
    }
    #[doc = "Copy from contents from another slice, padding with zeros or truncating when needed"]
    pub fn new_from_zeroed(other: &[u8]) -> Self {
        let mut buf = [0u8; Self::len()];
        let len = buf.len().min(other.len());
        buf[..len].clone_from_slice(&other[..len]);
        Self::new_from_array(buf)
    }
    pub fn new_from_array(buf: [u8; 2usize]) -> Self {
        unsafe { std::mem::transmute(buf) }
    }
    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            let ptr: *const u8 = std::mem::transmute(self as *const Self);
            std::slice::from_raw_parts(ptr, Self::len())
        }
    }
    pub fn from_slice(buf: &[u8]) -> &Self {
        assert!(buf.len() >= Self::len());
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf.as_ptr()) }
    }
    pub fn from_slice_mut(buf: &mut [u8]) -> &mut Self {
        assert!(buf.len() >= Self::len());
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf.as_ptr()) }
    }
    pub fn as_array(&self) -> &[u8; 2usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 2usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 2usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<NfCtTcpFlagsMask>() == 2usize);
        const _: () = assert!(std::mem::align_of::<NfCtTcpFlagsMask>() == 1usize);
        2usize
    }
}
impl std::fmt::Debug for NfCtTcpFlagsMask {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("NfCtTcpFlagsMask")
            .field(
                "flags",
                &FormatFlags(self.flags.into(), NfCtTcpFlags::from_value),
            )
            .field(
                "mask",
                &FormatFlags(self.mask.into(), NfCtTcpFlags::from_value),
            )
            .finish()
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
            if let Ok(CounterAttrs::Packets(val)) = attr {
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
            if let Ok(CounterAttrs::Bytes(val)) = attr {
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
            if let Ok(CounterAttrs::PacketsOld(val)) = attr {
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
            if let Ok(CounterAttrs::BytesOld(val)) = attr {
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
            if let Ok(CounterAttrs::Pad(val)) = attr {
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
        let mut pos;
        let mut r#type;
        loop {
            pos = self.pos;
            r#type = None;
            if self.buf.len() == self.pos {
                return None;
            }
            let Some((header, next)) = chop_header(self.buf, &mut self.pos) else {
                self.pos = self.buf.len();
                break;
            };
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
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
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
        let mut iter = IterableCounterAttrs::with_loc(&[], self.orig_loc);
        for attr in IterateAttrs::new(self.get_buf()) {
            iter.buf = attr;
            iter.pos = 0;
            let Some(attr) = iter.next() else {
                fmt.field("Err", &FormatUnrecognized(attr));
                continue;
            };
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
        if missing_type.is_some() && cur == offset {
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
    #[doc = "l4 protocol number\n"]
    ProtoNum(u8),
    #[doc = "l4 source port\n"]
    ProtoSrcPort(u16),
    #[doc = "l4 source port\n"]
    ProtoDstPort(u16),
    #[doc = "l4 icmp id\n"]
    ProtoIcmpId(u16),
    ProtoIcmpType(u8),
    ProtoIcmpCode(u8),
    #[doc = "l4 icmp id\n"]
    ProtoIcmpv6Id(u16),
    ProtoIcmpv6Type(u8),
    ProtoIcmpv6Code(u8),
}
impl<'a> IterableTupleProtoAttrs<'a> {
    #[doc = "l4 protocol number\n"]
    pub fn get_proto_num(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TupleProtoAttrs::ProtoNum(val)) = attr {
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
    #[doc = "l4 source port\n"]
    pub fn get_proto_src_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TupleProtoAttrs::ProtoSrcPort(val)) = attr {
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
    #[doc = "l4 source port\n"]
    pub fn get_proto_dst_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TupleProtoAttrs::ProtoDstPort(val)) = attr {
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
    #[doc = "l4 icmp id\n"]
    pub fn get_proto_icmp_id(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TupleProtoAttrs::ProtoIcmpId(val)) = attr {
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
            if let Ok(TupleProtoAttrs::ProtoIcmpType(val)) = attr {
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
            if let Ok(TupleProtoAttrs::ProtoIcmpCode(val)) = attr {
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
    #[doc = "l4 icmp id\n"]
    pub fn get_proto_icmpv6_id(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TupleProtoAttrs::ProtoIcmpv6Id(val)) = attr {
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
            if let Ok(TupleProtoAttrs::ProtoIcmpv6Type(val)) = attr {
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
            if let Ok(TupleProtoAttrs::ProtoIcmpv6Code(val)) = attr {
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
        let mut pos;
        let mut r#type;
        loop {
            pos = self.pos;
            r#type = None;
            if self.buf.len() == self.pos {
                return None;
            }
            let Some((header, next)) = chop_header(self.buf, &mut self.pos) else {
                self.pos = self.buf.len();
                break;
            };
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
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
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
        let mut iter = IterableTupleProtoAttrs::with_loc(&[], self.orig_loc);
        for attr in IterateAttrs::new(self.get_buf()) {
            iter.buf = attr;
            iter.pos = 0;
            let Some(attr) = iter.next() else {
                fmt.field("Err", &FormatUnrecognized(attr));
                continue;
            };
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
        if missing_type.is_some() && cur == offset {
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
    #[doc = "ipv4 source address\n"]
    IpV4Src(std::net::Ipv4Addr),
    #[doc = "ipv4 destination address\n"]
    IpV4Dst(std::net::Ipv4Addr),
    #[doc = "ipv6 source address\n"]
    IpV6Src(std::net::Ipv6Addr),
    #[doc = "ipv6 destination address\n"]
    IpV6Dst(std::net::Ipv6Addr),
}
impl<'a> IterableTupleIpAttrs<'a> {
    #[doc = "ipv4 source address\n"]
    pub fn get_ip_v4_src(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TupleIpAttrs::IpV4Src(val)) = attr {
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
    #[doc = "ipv4 destination address\n"]
    pub fn get_ip_v4_dst(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TupleIpAttrs::IpV4Dst(val)) = attr {
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
    #[doc = "ipv6 source address\n"]
    pub fn get_ip_v6_src(&self) -> Result<std::net::Ipv6Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TupleIpAttrs::IpV6Src(val)) = attr {
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
    #[doc = "ipv6 destination address\n"]
    pub fn get_ip_v6_dst(&self) -> Result<std::net::Ipv6Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TupleIpAttrs::IpV6Dst(val)) = attr {
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
        let mut pos;
        let mut r#type;
        loop {
            pos = self.pos;
            r#type = None;
            if self.buf.len() == self.pos {
                return None;
            }
            let Some((header, next)) = chop_header(self.buf, &mut self.pos) else {
                self.pos = self.buf.len();
                break;
            };
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
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
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
        let mut iter = IterableTupleIpAttrs::with_loc(&[], self.orig_loc);
        for attr in IterateAttrs::new(self.get_buf()) {
            iter.buf = attr;
            iter.pos = 0;
            let Some(attr) = iter.next() else {
                fmt.field("Err", &FormatUnrecognized(attr));
                continue;
            };
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
        if missing_type.is_some() && cur == offset {
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
    #[doc = "conntrack l3 information\n"]
    TupleIp(IterableTupleIpAttrs<'a>),
    #[doc = "conntrack l4 information\n"]
    TupleProto(IterableTupleProtoAttrs<'a>),
    #[doc = "conntrack zone id\n"]
    TupleZone(u16),
}
impl<'a> IterableTupleAttrs<'a> {
    #[doc = "conntrack l3 information\n"]
    pub fn get_tuple_ip(&self) -> Result<IterableTupleIpAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TupleAttrs::TupleIp(val)) = attr {
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
    #[doc = "conntrack l4 information\n"]
    pub fn get_tuple_proto(&self) -> Result<IterableTupleProtoAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TupleAttrs::TupleProto(val)) = attr {
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
    #[doc = "conntrack zone id\n"]
    pub fn get_tuple_zone(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TupleAttrs::TupleZone(val)) = attr {
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
        let mut pos;
        let mut r#type;
        loop {
            pos = self.pos;
            r#type = None;
            if self.buf.len() == self.pos {
                return None;
            }
            let Some((header, next)) = chop_header(self.buf, &mut self.pos) else {
                self.pos = self.buf.len();
                break;
            };
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
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
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
        let mut iter = IterableTupleAttrs::with_loc(&[], self.orig_loc);
        for attr in IterateAttrs::new(self.get_buf()) {
            iter.buf = attr;
            iter.pos = 0;
            let Some(attr) = iter.next() else {
                fmt.field("Err", &FormatUnrecognized(attr));
                continue;
            };
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
        if missing_type.is_some() && cur == offset {
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
    #[doc = "tcp connection state\n\nAssociated type: [`NfCtTcpState`] (enum)"]
    TcpState(u8),
    #[doc = "window scaling factor in original direction\n"]
    TcpWscaleOriginal(u8),
    #[doc = "window scaling factor in reply direction\n"]
    TcpWscaleReply(u8),
    TcpFlagsOriginal(NfCtTcpFlagsMask),
    TcpFlagsReply(NfCtTcpFlagsMask),
}
impl<'a> IterableProtoinfoTcpAttrs<'a> {
    #[doc = "tcp connection state\n\nAssociated type: [`NfCtTcpState`] (enum)"]
    pub fn get_tcp_state(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ProtoinfoTcpAttrs::TcpState(val)) = attr {
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
    #[doc = "window scaling factor in original direction\n"]
    pub fn get_tcp_wscale_original(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ProtoinfoTcpAttrs::TcpWscaleOriginal(val)) = attr {
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
    #[doc = "window scaling factor in reply direction\n"]
    pub fn get_tcp_wscale_reply(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ProtoinfoTcpAttrs::TcpWscaleReply(val)) = attr {
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
    pub fn get_tcp_flags_original(&self) -> Result<NfCtTcpFlagsMask, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ProtoinfoTcpAttrs::TcpFlagsOriginal(val)) = attr {
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
    pub fn get_tcp_flags_reply(&self) -> Result<NfCtTcpFlagsMask, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ProtoinfoTcpAttrs::TcpFlagsReply(val)) = attr {
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
        let mut pos;
        let mut r#type;
        loop {
            pos = self.pos;
            r#type = None;
            if self.buf.len() == self.pos {
                return None;
            }
            let Some((header, next)) = chop_header(self.buf, &mut self.pos) else {
                self.pos = self.buf.len();
                break;
            };
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
                    let res = Some(NfCtTcpFlagsMask::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => ProtoinfoTcpAttrs::TcpFlagsReply({
                    let res = Some(NfCtTcpFlagsMask::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
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
        let mut iter = IterableProtoinfoTcpAttrs::with_loc(&[], self.orig_loc);
        for attr in IterateAttrs::new(self.get_buf()) {
            iter.buf = attr;
            iter.pos = 0;
            let Some(attr) = iter.next() else {
                fmt.field("Err", &FormatUnrecognized(attr));
                continue;
            };
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
        if missing_type.is_some() && cur == offset {
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
    #[doc = "dccp connection state\n"]
    DccpState(u8),
    DccpRole(u8),
    DccpHandshakeSeq(u64),
    DccpPad(&'a [u8]),
}
impl<'a> IterableProtoinfoDccpAttrs<'a> {
    #[doc = "dccp connection state\n"]
    pub fn get_dccp_state(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ProtoinfoDccpAttrs::DccpState(val)) = attr {
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
            if let Ok(ProtoinfoDccpAttrs::DccpRole(val)) = attr {
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
            if let Ok(ProtoinfoDccpAttrs::DccpHandshakeSeq(val)) = attr {
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
            if let Ok(ProtoinfoDccpAttrs::DccpPad(val)) = attr {
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
        let mut pos;
        let mut r#type;
        loop {
            pos = self.pos;
            r#type = None;
            if self.buf.len() == self.pos {
                return None;
            }
            let Some((header, next)) = chop_header(self.buf, &mut self.pos) else {
                self.pos = self.buf.len();
                break;
            };
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
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
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
        let mut iter = IterableProtoinfoDccpAttrs::with_loc(&[], self.orig_loc);
        for attr in IterateAttrs::new(self.get_buf()) {
            iter.buf = attr;
            iter.pos = 0;
            let Some(attr) = iter.next() else {
                fmt.field("Err", &FormatUnrecognized(attr));
                continue;
            };
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
        if missing_type.is_some() && cur == offset {
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
    #[doc = "sctp connection state\n\nAssociated type: [`NfCtSctpState`] (enum)"]
    SctpState(u8),
    VtagOriginal(u32),
    VtagReply(u32),
}
impl<'a> IterableProtoinfoSctpAttrs<'a> {
    #[doc = "sctp connection state\n\nAssociated type: [`NfCtSctpState`] (enum)"]
    pub fn get_sctp_state(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ProtoinfoSctpAttrs::SctpState(val)) = attr {
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
            if let Ok(ProtoinfoSctpAttrs::VtagOriginal(val)) = attr {
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
            if let Ok(ProtoinfoSctpAttrs::VtagReply(val)) = attr {
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
        let mut pos;
        let mut r#type;
        loop {
            pos = self.pos;
            r#type = None;
            if self.buf.len() == self.pos {
                return None;
            }
            let Some((header, next)) = chop_header(self.buf, &mut self.pos) else {
                self.pos = self.buf.len();
                break;
            };
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
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
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
        let mut iter = IterableProtoinfoSctpAttrs::with_loc(&[], self.orig_loc);
        for attr in IterateAttrs::new(self.get_buf()) {
            iter.buf = attr;
            iter.pos = 0;
            let Some(attr) = iter.next() else {
                fmt.field("Err", &FormatUnrecognized(attr));
                continue;
            };
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
        if missing_type.is_some() && cur == offset {
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
    #[doc = "conntrack tcp state information\n"]
    ProtoinfoTcp(IterableProtoinfoTcpAttrs<'a>),
    #[doc = "conntrack dccp state information\n"]
    ProtoinfoDccp(IterableProtoinfoDccpAttrs<'a>),
    #[doc = "conntrack sctp state information\n"]
    ProtoinfoSctp(IterableProtoinfoSctpAttrs<'a>),
}
impl<'a> IterableProtoinfoAttrs<'a> {
    #[doc = "conntrack tcp state information\n"]
    pub fn get_protoinfo_tcp(&self) -> Result<IterableProtoinfoTcpAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ProtoinfoAttrs::ProtoinfoTcp(val)) = attr {
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
    #[doc = "conntrack dccp state information\n"]
    pub fn get_protoinfo_dccp(&self) -> Result<IterableProtoinfoDccpAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ProtoinfoAttrs::ProtoinfoDccp(val)) = attr {
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
    #[doc = "conntrack sctp state information\n"]
    pub fn get_protoinfo_sctp(&self) -> Result<IterableProtoinfoSctpAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ProtoinfoAttrs::ProtoinfoSctp(val)) = attr {
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
        let mut pos;
        let mut r#type;
        loop {
            pos = self.pos;
            r#type = None;
            if self.buf.len() == self.pos {
                return None;
            }
            let Some((header, next)) = chop_header(self.buf, &mut self.pos) else {
                self.pos = self.buf.len();
                break;
            };
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
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
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
        let mut iter = IterableProtoinfoAttrs::with_loc(&[], self.orig_loc);
        for attr in IterateAttrs::new(self.get_buf()) {
            iter.buf = attr;
            iter.pos = 0;
            let Some(attr) = iter.next() else {
                fmt.field("Err", &FormatUnrecognized(attr));
                continue;
            };
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
        if missing_type.is_some() && cur == offset {
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
    #[doc = "helper name\n"]
    HelpName(&'a CStr),
}
impl<'a> IterableHelpAttrs<'a> {
    #[doc = "helper name\n"]
    pub fn get_help_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(HelpAttrs::HelpName(val)) = attr {
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
        let mut pos;
        let mut r#type;
        loop {
            pos = self.pos;
            r#type = None;
            if self.buf.len() == self.pos {
                return None;
            }
            let Some((header, next)) = chop_header(self.buf, &mut self.pos) else {
                self.pos = self.buf.len();
                break;
            };
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => HelpAttrs::HelpName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
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
        let mut iter = IterableHelpAttrs::with_loc(&[], self.orig_loc);
        for attr in IterateAttrs::new(self.get_buf()) {
            iter.buf = attr;
            iter.pos = 0;
            let Some(attr) = iter.next() else {
                fmt.field("Err", &FormatUnrecognized(attr));
                continue;
            };
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
        if missing_type.is_some() && cur == offset {
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
            if let Ok(NatProtoAttrs::NatPortMin(val)) = attr {
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
            if let Ok(NatProtoAttrs::NatPortMax(val)) = attr {
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
        let mut pos;
        let mut r#type;
        loop {
            pos = self.pos;
            r#type = None;
            if self.buf.len() == self.pos {
                return None;
            }
            let Some((header, next)) = chop_header(self.buf, &mut self.pos) else {
                self.pos = self.buf.len();
                break;
            };
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
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
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
        let mut iter = IterableNatProtoAttrs::with_loc(&[], self.orig_loc);
        for attr in IterateAttrs::new(self.get_buf()) {
            iter.buf = attr;
            iter.pos = 0;
            let Some(attr) = iter.next() else {
                fmt.field("Err", &FormatUnrecognized(attr));
                continue;
            };
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
        if missing_type.is_some() && cur == offset {
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
            if let Ok(NatAttrs::NatV4Minip(val)) = attr {
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
            if let Ok(NatAttrs::NatV4Maxip(val)) = attr {
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
            if let Ok(NatAttrs::NatV6Minip(val)) = attr {
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
            if let Ok(NatAttrs::NatV6Maxip(val)) = attr {
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
            if let Ok(NatAttrs::NatProto(val)) = attr {
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
        let mut pos;
        let mut r#type;
        loop {
            pos = self.pos;
            r#type = None;
            if self.buf.len() == self.pos {
                return None;
            }
            let Some((header, next)) = chop_header(self.buf, &mut self.pos) else {
                self.pos = self.buf.len();
                break;
            };
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
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
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
        let mut iter = IterableNatAttrs::with_loc(&[], self.orig_loc);
        for attr in IterateAttrs::new(self.get_buf()) {
            iter.buf = attr;
            iter.pos = 0;
            let Some(attr) = iter.next() else {
                fmt.field("Err", &FormatUnrecognized(attr));
                continue;
            };
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
                NatAttrs::NatV6Minip(val) => fmt.field("NatV6Minip", &FormatHexdump(val)),
                NatAttrs::NatV6Maxip(val) => fmt.field("NatV6Maxip", &FormatHexdump(val)),
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
        if missing_type.is_some() && cur == offset {
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
            if let Ok(SeqadjAttrs::CorrectionPos(val)) = attr {
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
            if let Ok(SeqadjAttrs::OffsetBefore(val)) = attr {
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
            if let Ok(SeqadjAttrs::OffsetAfter(val)) = attr {
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
        let mut pos;
        let mut r#type;
        loop {
            pos = self.pos;
            r#type = None;
            if self.buf.len() == self.pos {
                return None;
            }
            let Some((header, next)) = chop_header(self.buf, &mut self.pos) else {
                self.pos = self.buf.len();
                break;
            };
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
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
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
        let mut iter = IterableSeqadjAttrs::with_loc(&[], self.orig_loc);
        for attr in IterateAttrs::new(self.get_buf()) {
            iter.buf = attr;
            iter.pos = 0;
            let Some(attr) = iter.next() else {
                fmt.field("Err", &FormatUnrecognized(attr));
                continue;
            };
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
        if missing_type.is_some() && cur == offset {
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
            if let Ok(SecctxAttrs::SecctxName(val)) = attr {
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
        let mut pos;
        let mut r#type;
        loop {
            pos = self.pos;
            r#type = None;
            if self.buf.len() == self.pos {
                return None;
            }
            let Some((header, next)) = chop_header(self.buf, &mut self.pos) else {
                self.pos = self.buf.len();
                break;
            };
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => SecctxAttrs::SecctxName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
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
        let mut iter = IterableSecctxAttrs::with_loc(&[], self.orig_loc);
        for attr in IterateAttrs::new(self.get_buf()) {
            iter.buf = attr;
            iter.pos = 0;
            let Some(attr) = iter.next() else {
                fmt.field("Err", &FormatUnrecognized(attr));
                continue;
            };
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
        if missing_type.is_some() && cur == offset {
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
            if let Ok(SynproxyAttrs::Isn(val)) = attr {
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
            if let Ok(SynproxyAttrs::Its(val)) = attr {
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
            if let Ok(SynproxyAttrs::Tsoff(val)) = attr {
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
        let mut pos;
        let mut r#type;
        loop {
            pos = self.pos;
            r#type = None;
            if self.buf.len() == self.pos {
                return None;
            }
            let Some((header, next)) = chop_header(self.buf, &mut self.pos) else {
                self.pos = self.buf.len();
                break;
            };
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
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
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
        let mut iter = IterableSynproxyAttrs::with_loc(&[], self.orig_loc);
        for attr in IterateAttrs::new(self.get_buf()) {
            iter.buf = attr;
            iter.pos = 0;
            let Some(attr) = iter.next() else {
                fmt.field("Err", &FormatUnrecognized(attr));
                continue;
            };
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
        if missing_type.is_some() && cur == offset {
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
    #[doc = "conntrack l3+l4 protocol information, original direction\n"]
    TupleOrig(IterableTupleAttrs<'a>),
    #[doc = "conntrack l3+l4 protocol information, reply direction\n"]
    TupleReply(IterableTupleAttrs<'a>),
    #[doc = "conntrack flag bits\n\nAssociated type: [`NfCtStatus`] (1 bit per enumeration)"]
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
    #[doc = "obsolete\n"]
    Secmark(&'a [u8]),
    #[doc = "conntrack zone id\n"]
    Zone(u16),
    Secctx(IterableSecctxAttrs<'a>),
    Timestamp(u64),
    MarkMask(u32),
    Labels(&'a [u8]),
    LabelsMask(&'a [u8]),
    Synproxy(IterableSynproxyAttrs<'a>),
    Filter(IterableTupleAttrs<'a>),
    #[doc = "conntrack flag bits to change\n\nAssociated type: [`NfCtStatus`] (1 bit per enumeration)"]
    StatusMask(u32),
    TimestampEvent(u64),
}
impl<'a> IterableConntrackAttrs<'a> {
    #[doc = "conntrack l3+l4 protocol information, original direction\n"]
    pub fn get_tuple_orig(&self) -> Result<IterableTupleAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ConntrackAttrs::TupleOrig(val)) = attr {
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
    #[doc = "conntrack l3+l4 protocol information, reply direction\n"]
    pub fn get_tuple_reply(&self) -> Result<IterableTupleAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ConntrackAttrs::TupleReply(val)) = attr {
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
    #[doc = "conntrack flag bits\n\nAssociated type: [`NfCtStatus`] (1 bit per enumeration)"]
    pub fn get_status(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ConntrackAttrs::Status(val)) = attr {
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
            if let Ok(ConntrackAttrs::Protoinfo(val)) = attr {
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
            if let Ok(ConntrackAttrs::Help(val)) = attr {
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
            if let Ok(ConntrackAttrs::NatSrc(val)) = attr {
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
            if let Ok(ConntrackAttrs::Timeout(val)) = attr {
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
            if let Ok(ConntrackAttrs::Mark(val)) = attr {
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
            if let Ok(ConntrackAttrs::CountersOrig(val)) = attr {
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
            if let Ok(ConntrackAttrs::CountersReply(val)) = attr {
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
            if let Ok(ConntrackAttrs::Use(val)) = attr {
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
            if let Ok(ConntrackAttrs::Id(val)) = attr {
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
            if let Ok(ConntrackAttrs::NatDst(val)) = attr {
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
            if let Ok(ConntrackAttrs::TupleMaster(val)) = attr {
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
            if let Ok(ConntrackAttrs::SeqAdjOrig(val)) = attr {
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
            if let Ok(ConntrackAttrs::SeqAdjReply(val)) = attr {
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
    #[doc = "obsolete\n"]
    pub fn get_secmark(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ConntrackAttrs::Secmark(val)) = attr {
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
    #[doc = "conntrack zone id\n"]
    pub fn get_zone(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ConntrackAttrs::Zone(val)) = attr {
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
            if let Ok(ConntrackAttrs::Secctx(val)) = attr {
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
            if let Ok(ConntrackAttrs::Timestamp(val)) = attr {
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
            if let Ok(ConntrackAttrs::MarkMask(val)) = attr {
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
            if let Ok(ConntrackAttrs::Labels(val)) = attr {
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
            if let Ok(ConntrackAttrs::LabelsMask(val)) = attr {
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
            if let Ok(ConntrackAttrs::Synproxy(val)) = attr {
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
            if let Ok(ConntrackAttrs::Filter(val)) = attr {
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
    #[doc = "conntrack flag bits to change\n\nAssociated type: [`NfCtStatus`] (1 bit per enumeration)"]
    pub fn get_status_mask(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ConntrackAttrs::StatusMask(val)) = attr {
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
            if let Ok(ConntrackAttrs::TimestampEvent(val)) = attr {
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
        let mut pos;
        let mut r#type;
        loop {
            pos = self.pos;
            r#type = None;
            if self.buf.len() == self.pos {
                return None;
            }
            let Some((header, next)) = chop_header(self.buf, &mut self.pos) else {
                self.pos = self.buf.len();
                break;
            };
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
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
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
        let mut iter = IterableConntrackAttrs::with_loc(&[], self.orig_loc);
        for attr in IterateAttrs::new(self.get_buf()) {
            iter.buf = attr;
            iter.pos = 0;
            let Some(attr) = iter.next() else {
                fmt.field("Err", &FormatUnrecognized(attr));
                continue;
            };
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
                ConntrackAttrs::Secmark(val) => fmt.field("Secmark", &FormatHexdump(val)),
                ConntrackAttrs::Zone(val) => fmt.field("Zone", &val),
                ConntrackAttrs::Secctx(val) => fmt.field("Secctx", &val),
                ConntrackAttrs::Timestamp(val) => fmt.field("Timestamp", &val),
                ConntrackAttrs::MarkMask(val) => fmt.field("MarkMask", &val),
                ConntrackAttrs::Labels(val) => fmt.field("Labels", &FormatHexdump(val)),
                ConntrackAttrs::LabelsMask(val) => fmt.field("LabelsMask", &FormatHexdump(val)),
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
        if missing_type.is_some() && cur == offset {
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
    #[doc = "obsolete\n"]
    Searched(u32),
    Found(u32),
    #[doc = "obsolete\n"]
    New(u32),
    #[doc = "obsolete\n"]
    Invalid(u32),
    #[doc = "obsolete\n"]
    Ignore(u32),
    #[doc = "obsolete\n"]
    Delete(u32),
    #[doc = "obsolete\n"]
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
    #[doc = "obsolete\n"]
    pub fn get_searched(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ConntrackStatsAttrs::Searched(val)) = attr {
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
            if let Ok(ConntrackStatsAttrs::Found(val)) = attr {
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
    #[doc = "obsolete\n"]
    pub fn get_new(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ConntrackStatsAttrs::New(val)) = attr {
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
    #[doc = "obsolete\n"]
    pub fn get_invalid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ConntrackStatsAttrs::Invalid(val)) = attr {
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
    #[doc = "obsolete\n"]
    pub fn get_ignore(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ConntrackStatsAttrs::Ignore(val)) = attr {
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
    #[doc = "obsolete\n"]
    pub fn get_delete(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ConntrackStatsAttrs::Delete(val)) = attr {
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
    #[doc = "obsolete\n"]
    pub fn get_delete_list(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ConntrackStatsAttrs::DeleteList(val)) = attr {
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
            if let Ok(ConntrackStatsAttrs::Insert(val)) = attr {
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
            if let Ok(ConntrackStatsAttrs::InsertFailed(val)) = attr {
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
            if let Ok(ConntrackStatsAttrs::Drop(val)) = attr {
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
            if let Ok(ConntrackStatsAttrs::EarlyDrop(val)) = attr {
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
            if let Ok(ConntrackStatsAttrs::Error(val)) = attr {
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
            if let Ok(ConntrackStatsAttrs::SearchRestart(val)) = attr {
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
            if let Ok(ConntrackStatsAttrs::ClashResolve(val)) = attr {
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
            if let Ok(ConntrackStatsAttrs::ChainToolong(val)) = attr {
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
        let mut pos;
        let mut r#type;
        loop {
            pos = self.pos;
            r#type = None;
            if self.buf.len() == self.pos {
                return None;
            }
            let Some((header, next)) = chop_header(self.buf, &mut self.pos) else {
                self.pos = self.buf.len();
                break;
            };
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
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
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
        let mut iter = IterableConntrackStatsAttrs::with_loc(&[], self.orig_loc);
        for attr in IterateAttrs::new(self.get_buf()) {
            iter.buf = attr;
            iter.pos = 0;
            let Some(attr) = iter.next() else {
                fmt.field("Err", &FormatUnrecognized(attr));
                continue;
            };
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
        if missing_type.is_some() && cur == offset {
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
pub struct PushCounterAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushCounterAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushCounterAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_vec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_packets(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 1u16, 8 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_bytes(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 2u16, 8 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_packets_old(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_bytes_old(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 5u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushCounterAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushTupleProtoAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushTupleProtoAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushTupleProtoAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_vec_mut(), *header_offset);
        }
        prev
    }
    #[doc = "l4 protocol number\n"]
    pub fn push_proto_num(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 1u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "l4 source port\n"]
    pub fn push_proto_src_port(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 2u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "l4 source port\n"]
    pub fn push_proto_dst_port(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 3u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "l4 icmp id\n"]
    pub fn push_proto_icmp_id(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 4u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_proto_icmp_type(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 5u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_proto_icmp_code(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 6u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "l4 icmp id\n"]
    pub fn push_proto_icmpv6_id(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 7u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_proto_icmpv6_type(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 8u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_proto_icmpv6_code(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 9u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushTupleProtoAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushTupleIpAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushTupleIpAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushTupleIpAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_vec_mut(), *header_offset);
        }
        prev
    }
    #[doc = "ipv4 source address\n"]
    pub fn push_ip_v4_src(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    #[doc = "ipv4 destination address\n"]
    pub fn push_ip_v4_dst(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    #[doc = "ipv6 source address\n"]
    pub fn push_ip_v6_src(mut self, value: std::net::Ipv6Addr) -> Self {
        push_header(self.as_vec_mut(), 3u16, 16 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    #[doc = "ipv6 destination address\n"]
    pub fn push_ip_v6_dst(mut self, value: std::net::Ipv6Addr) -> Self {
        push_header(self.as_vec_mut(), 4u16, 16 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushTupleIpAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushTupleAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushTupleAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushTupleAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_vec_mut(), *header_offset);
        }
        prev
    }
    #[doc = "conntrack l3 information\n"]
    pub fn nested_tuple_ip(mut self) -> PushTupleIpAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushTupleIpAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "conntrack l4 information\n"]
    pub fn nested_tuple_proto(mut self) -> PushTupleProtoAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushTupleProtoAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "conntrack zone id\n"]
    pub fn push_tuple_zone(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 3u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushTupleAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushProtoinfoTcpAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushProtoinfoTcpAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushProtoinfoTcpAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_vec_mut(), *header_offset);
        }
        prev
    }
    #[doc = "tcp connection state\n\nAssociated type: [`NfCtTcpState`] (enum)"]
    pub fn push_tcp_state(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 1u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "window scaling factor in original direction\n"]
    pub fn push_tcp_wscale_original(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 2u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "window scaling factor in reply direction\n"]
    pub fn push_tcp_wscale_reply(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 3u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tcp_flags_original(mut self, value: NfCtTcpFlagsMask) -> Self {
        push_header(self.as_vec_mut(), 4u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_tcp_flags_reply(mut self, value: NfCtTcpFlagsMask) -> Self {
        push_header(self.as_vec_mut(), 5u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
}
impl<Prev: Pusher> Drop for PushProtoinfoTcpAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushProtoinfoDccpAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushProtoinfoDccpAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushProtoinfoDccpAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_vec_mut(), *header_offset);
        }
        prev
    }
    #[doc = "dccp connection state\n"]
    pub fn push_dccp_state(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 1u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dccp_role(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 2u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dccp_handshake_seq(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 3u16, 8 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_dccp_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 4u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushProtoinfoDccpAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushProtoinfoSctpAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushProtoinfoSctpAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushProtoinfoSctpAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_vec_mut(), *header_offset);
        }
        prev
    }
    #[doc = "sctp connection state\n\nAssociated type: [`NfCtSctpState`] (enum)"]
    pub fn push_sctp_state(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 1u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_vtag_original(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_vtag_reply(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushProtoinfoSctpAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushProtoinfoAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushProtoinfoAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushProtoinfoAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_vec_mut(), *header_offset);
        }
        prev
    }
    #[doc = "conntrack tcp state information\n"]
    pub fn nested_protoinfo_tcp(mut self) -> PushProtoinfoTcpAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushProtoinfoTcpAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "conntrack dccp state information\n"]
    pub fn nested_protoinfo_dccp(mut self) -> PushProtoinfoDccpAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushProtoinfoDccpAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "conntrack sctp state information\n"]
    pub fn nested_protoinfo_sctp(mut self) -> PushProtoinfoSctpAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 3u16);
        PushProtoinfoSctpAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushProtoinfoAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushHelpAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushHelpAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushHelpAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_vec_mut(), *header_offset);
        }
        prev
    }
    #[doc = "helper name\n"]
    pub fn push_help_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "helper name\n"]
    pub fn push_help_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
}
impl<Prev: Pusher> Drop for PushHelpAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushNatProtoAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushNatProtoAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushNatProtoAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_vec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_nat_port_min(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 1u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_nat_port_max(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 2u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushNatProtoAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushNatAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushNatAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushNatAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_vec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_nat_v4_minip(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_nat_v4_maxip(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_nat_v6_minip(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_nat_v6_maxip(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 4u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn nested_nat_proto(mut self) -> PushNatProtoAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 5u16);
        PushNatProtoAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushNatAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSeqadjAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushSeqadjAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushSeqadjAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_vec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_correction_pos(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_offset_before(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_offset_after(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushSeqadjAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSecctxAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushSecctxAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushSecctxAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_vec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_secctx_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_secctx_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
}
impl<Prev: Pusher> Drop for PushSecctxAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSynproxyAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushSynproxyAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushSynproxyAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_vec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_isn(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_its(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_tsoff(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushSynproxyAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushConntrackAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushConntrackAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushConntrackAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_vec_mut(), *header_offset);
        }
        prev
    }
    #[doc = "conntrack l3+l4 protocol information, original direction\n"]
    pub fn nested_tuple_orig(mut self) -> PushTupleAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushTupleAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "conntrack l3+l4 protocol information, reply direction\n"]
    pub fn nested_tuple_reply(mut self) -> PushTupleAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushTupleAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "conntrack flag bits\n\nAssociated type: [`NfCtStatus`] (1 bit per enumeration)"]
    pub fn push_status(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn nested_protoinfo(mut self) -> PushProtoinfoAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 4u16);
        PushProtoinfoAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_help(mut self) -> PushHelpAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 5u16);
        PushHelpAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_nat_src(mut self) -> PushNatAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 6u16);
        PushNatAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_timeout(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_mark(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn nested_counters_orig(mut self) -> PushCounterAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 9u16);
        PushCounterAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_counters_reply(mut self) -> PushCounterAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 10u16);
        PushCounterAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_use(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 11u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 12u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn nested_nat_dst(mut self) -> PushNatAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 13u16);
        PushNatAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_tuple_master(mut self) -> PushTupleAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 14u16);
        PushTupleAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_seq_adj_orig(mut self) -> PushSeqadjAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 15u16);
        PushSeqadjAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_seq_adj_reply(mut self) -> PushSeqadjAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 16u16);
        PushSeqadjAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "obsolete\n"]
    pub fn push_secmark(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 17u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "conntrack zone id\n"]
    pub fn push_zone(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 18u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn nested_secctx(mut self) -> PushSecctxAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 19u16);
        PushSecctxAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_timestamp(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 20u16, 8 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_mark_mask(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 21u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_labels(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 22u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_labels_mask(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 23u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn nested_synproxy(mut self) -> PushSynproxyAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 24u16);
        PushSynproxyAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_filter(mut self) -> PushTupleAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 25u16);
        PushTupleAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "conntrack flag bits to change\n\nAssociated type: [`NfCtStatus`] (1 bit per enumeration)"]
    pub fn push_status_mask(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 26u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_timestamp_event(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 27u16, 8 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushConntrackAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushConntrackStatsAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushConntrackStatsAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushConntrackStatsAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_vec_mut(), *header_offset);
        }
        prev
    }
    #[doc = "obsolete\n"]
    pub fn push_searched(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_found(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "obsolete\n"]
    pub fn push_new(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "obsolete\n"]
    pub fn push_invalid(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "obsolete\n"]
    pub fn push_ignore(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "obsolete\n"]
    pub fn push_delete(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "obsolete\n"]
    pub fn push_delete_list(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_insert(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_insert_failed(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_drop(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_early_drop(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 11u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_error(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 12u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_search_restart(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 13u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_clash_resolve(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 14u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_chain_toolong(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 15u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushConntrackStatsAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "get / dump entries\n\nRequest attributes:\n- [.push_status()](PushConntrackAttrs::push_status)\n- [.push_mark()](PushConntrackAttrs::push_mark)\n- [.push_zone()](PushConntrackAttrs::push_zone)\n- [.nested_filter()](PushConntrackAttrs::nested_filter)\n\nReply attributes:\n- [.get_tuple_orig()](IterableConntrackAttrs::get_tuple_orig)\n- [.get_tuple_reply()](IterableConntrackAttrs::get_tuple_reply)\n- [.get_status()](IterableConntrackAttrs::get_status)\n- [.get_protoinfo()](IterableConntrackAttrs::get_protoinfo)\n- [.get_help()](IterableConntrackAttrs::get_help)\n- [.get_nat_src()](IterableConntrackAttrs::get_nat_src)\n- [.get_timeout()](IterableConntrackAttrs::get_timeout)\n- [.get_mark()](IterableConntrackAttrs::get_mark)\n- [.get_counters_orig()](IterableConntrackAttrs::get_counters_orig)\n- [.get_counters_reply()](IterableConntrackAttrs::get_counters_reply)\n- [.get_use()](IterableConntrackAttrs::get_use)\n- [.get_id()](IterableConntrackAttrs::get_id)\n- [.get_nat_dst()](IterableConntrackAttrs::get_nat_dst)\n- [.get_tuple_master()](IterableConntrackAttrs::get_tuple_master)\n- [.get_seq_adj_orig()](IterableConntrackAttrs::get_seq_adj_orig)\n- [.get_seq_adj_reply()](IterableConntrackAttrs::get_seq_adj_reply)\n- [.get_zone()](IterableConntrackAttrs::get_zone)\n- [.get_secctx()](IterableConntrackAttrs::get_secctx)\n- [.get_labels()](IterableConntrackAttrs::get_labels)\n- [.get_synproxy()](IterableConntrackAttrs::get_synproxy)\n\n"]
#[derive(Debug)]
pub struct OpGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetDump<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushConntrackAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushConntrackAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushConntrackAttrs<&mut Vec<u8>> {
        PushConntrackAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushConntrackAttrs<RequestBuf<'r>> {
        PushConntrackAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableConntrackAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableConntrackAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Nfgenmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Nfgenmsg {
        let pos = self.request.pos;
        Nfgenmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Nfgenmsg {
        let pos = self.request.pos;
        Nfgenmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetDump<'_> {
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
    type ReplyType<'buf> = (Nfgenmsg, IterableConntrackAttrs<'buf>);
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        Self::decode_request(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        Self::decode_request(buf)
            .1
            .lookup_attr(offset, missing_type)
    }
}
#[doc = "get / dump entries\n\nRequest attributes:\n- [.nested_tuple_orig()](PushConntrackAttrs::nested_tuple_orig)\n- [.nested_tuple_reply()](PushConntrackAttrs::nested_tuple_reply)\n- [.push_zone()](PushConntrackAttrs::push_zone)\n\nReply attributes:\n- [.get_tuple_orig()](IterableConntrackAttrs::get_tuple_orig)\n- [.get_tuple_reply()](IterableConntrackAttrs::get_tuple_reply)\n- [.get_status()](IterableConntrackAttrs::get_status)\n- [.get_protoinfo()](IterableConntrackAttrs::get_protoinfo)\n- [.get_help()](IterableConntrackAttrs::get_help)\n- [.get_nat_src()](IterableConntrackAttrs::get_nat_src)\n- [.get_timeout()](IterableConntrackAttrs::get_timeout)\n- [.get_mark()](IterableConntrackAttrs::get_mark)\n- [.get_counters_orig()](IterableConntrackAttrs::get_counters_orig)\n- [.get_counters_reply()](IterableConntrackAttrs::get_counters_reply)\n- [.get_use()](IterableConntrackAttrs::get_use)\n- [.get_id()](IterableConntrackAttrs::get_id)\n- [.get_nat_dst()](IterableConntrackAttrs::get_nat_dst)\n- [.get_tuple_master()](IterableConntrackAttrs::get_tuple_master)\n- [.get_seq_adj_orig()](IterableConntrackAttrs::get_seq_adj_orig)\n- [.get_seq_adj_reply()](IterableConntrackAttrs::get_seq_adj_reply)\n- [.get_zone()](IterableConntrackAttrs::get_zone)\n- [.get_secctx()](IterableConntrackAttrs::get_secctx)\n- [.get_labels()](IterableConntrackAttrs::get_labels)\n- [.get_synproxy()](IterableConntrackAttrs::get_synproxy)\n\n"]
#[derive(Debug)]
pub struct OpGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushConntrackAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushConntrackAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushConntrackAttrs<&mut Vec<u8>> {
        PushConntrackAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushConntrackAttrs<RequestBuf<'r>> {
        PushConntrackAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableConntrackAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableConntrackAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Nfgenmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Nfgenmsg {
        let pos = self.request.pos;
        Nfgenmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Nfgenmsg {
        let pos = self.request.pos;
        Nfgenmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetDo<'_> {
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
    type ReplyType<'buf> = (Nfgenmsg, IterableConntrackAttrs<'buf>);
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        Self::decode_request(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        Self::decode_request(buf)
            .1
            .lookup_attr(offset, missing_type)
    }
}
#[doc = "dump pcpu conntrack stats\n\nReply attributes:\n- [.get_searched()](IterableConntrackStatsAttrs::get_searched)\n- [.get_found()](IterableConntrackStatsAttrs::get_found)\n- [.get_insert()](IterableConntrackStatsAttrs::get_insert)\n- [.get_insert_failed()](IterableConntrackStatsAttrs::get_insert_failed)\n- [.get_drop()](IterableConntrackStatsAttrs::get_drop)\n- [.get_early_drop()](IterableConntrackStatsAttrs::get_early_drop)\n- [.get_error()](IterableConntrackStatsAttrs::get_error)\n- [.get_search_restart()](IterableConntrackStatsAttrs::get_search_restart)\n- [.get_clash_resolve()](IterableConntrackStatsAttrs::get_clash_resolve)\n- [.get_chain_toolong()](IterableConntrackStatsAttrs::get_chain_toolong)\n\n"]
#[derive(Debug)]
pub struct OpGetStatsDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetStatsDump<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushConntrackStatsAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushConntrackStatsAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushConntrackStatsAttrs<&mut Vec<u8>> {
        PushConntrackStatsAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushConntrackStatsAttrs<RequestBuf<'r>> {
        PushConntrackStatsAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableConntrackStatsAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableConntrackStatsAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Nfgenmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Nfgenmsg {
        let pos = self.request.pos;
        Nfgenmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Nfgenmsg {
        let pos = self.request.pos;
        Nfgenmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetStatsDump<'_> {
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
    type ReplyType<'buf> = (Nfgenmsg, IterableConntrackStatsAttrs<'buf>);
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        Self::decode_request(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        Self::decode_request(buf)
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
    pub fn new_from_buf(mut buf: Vec<u8>, first_seq: u32) -> Self {
        buf.clear();
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
        buf.clear();
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
        self.buf_mut().extend_from_slice(Nlmsghdr::new().as_slice());
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
                assert_eq!(self.last_header_offset + Nlmsghdr::len(), self.buf().len());
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
        let header = Nlmsghdr {
            len: (buf.len() - header_offset) as u32,
            r#type: request_type,
            flags: flags | consts::NLM_F_REQUEST as u16 | consts::NLM_F_ACK as u16,
            seq,
            pid: 0,
        };
        buf[header_offset..(header_offset + 16)].clone_from_slice(header.as_slice());
    }
}
use crate::traits::LookupFn;
use crate::utils::RequestBuf;
#[derive(Debug)]
pub struct Request<'buf> {
    buf: RequestBuf<'buf>,
    pos: usize,
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
    pub fn new_from_buf(mut buf: Vec<u8>) -> Self {
        buf.clear();
        Self {
            flags: 0,
            buf: RequestBuf::Own(buf),
            pos: 0,
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
        align(buf);
        let pos = buf.len();
        Self {
            flags: 0,
            buf: RequestBuf::Ref(buf),
            pos,
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
    #[doc = "Set `self.flags |= flags`"]
    pub fn set_flags(mut self, flags: u16) -> Self {
        self.flags |= flags;
        self
    }
    #[doc = "Set `self.flags ^= self.flags & flags`"]
    pub fn unset_flags(mut self, flags: u16) -> Self {
        self.flags ^= self.flags & flags;
        self
    }
    #[doc = "Set `NLM_F_DUMP` flag"]
    fn set_dump(mut self) -> Self {
        self.flags |= consts::NLM_F_DUMP as u16;
        self
    }
    #[doc = "get / dump entries\n\nRequest attributes:\n- [.push_status()](PushConntrackAttrs::push_status)\n- [.push_mark()](PushConntrackAttrs::push_mark)\n- [.push_zone()](PushConntrackAttrs::push_zone)\n- [.nested_filter()](PushConntrackAttrs::nested_filter)\n\nReply attributes:\n- [.get_tuple_orig()](IterableConntrackAttrs::get_tuple_orig)\n- [.get_tuple_reply()](IterableConntrackAttrs::get_tuple_reply)\n- [.get_status()](IterableConntrackAttrs::get_status)\n- [.get_protoinfo()](IterableConntrackAttrs::get_protoinfo)\n- [.get_help()](IterableConntrackAttrs::get_help)\n- [.get_nat_src()](IterableConntrackAttrs::get_nat_src)\n- [.get_timeout()](IterableConntrackAttrs::get_timeout)\n- [.get_mark()](IterableConntrackAttrs::get_mark)\n- [.get_counters_orig()](IterableConntrackAttrs::get_counters_orig)\n- [.get_counters_reply()](IterableConntrackAttrs::get_counters_reply)\n- [.get_use()](IterableConntrackAttrs::get_use)\n- [.get_id()](IterableConntrackAttrs::get_id)\n- [.get_nat_dst()](IterableConntrackAttrs::get_nat_dst)\n- [.get_tuple_master()](IterableConntrackAttrs::get_tuple_master)\n- [.get_seq_adj_orig()](IterableConntrackAttrs::get_seq_adj_orig)\n- [.get_seq_adj_reply()](IterableConntrackAttrs::get_seq_adj_reply)\n- [.get_zone()](IterableConntrackAttrs::get_zone)\n- [.get_secctx()](IterableConntrackAttrs::get_secctx)\n- [.get_labels()](IterableConntrackAttrs::get_labels)\n- [.get_synproxy()](IterableConntrackAttrs::get_synproxy)\n\n"]
    pub fn op_get_dump(self, header: &Nfgenmsg) -> OpGetDump<'buf> {
        let mut res = OpGetDump::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-get-dump", OpGetDump::lookup);
        res
    }
    #[doc = "get / dump entries\n\nRequest attributes:\n- [.nested_tuple_orig()](PushConntrackAttrs::nested_tuple_orig)\n- [.nested_tuple_reply()](PushConntrackAttrs::nested_tuple_reply)\n- [.push_zone()](PushConntrackAttrs::push_zone)\n\nReply attributes:\n- [.get_tuple_orig()](IterableConntrackAttrs::get_tuple_orig)\n- [.get_tuple_reply()](IterableConntrackAttrs::get_tuple_reply)\n- [.get_status()](IterableConntrackAttrs::get_status)\n- [.get_protoinfo()](IterableConntrackAttrs::get_protoinfo)\n- [.get_help()](IterableConntrackAttrs::get_help)\n- [.get_nat_src()](IterableConntrackAttrs::get_nat_src)\n- [.get_timeout()](IterableConntrackAttrs::get_timeout)\n- [.get_mark()](IterableConntrackAttrs::get_mark)\n- [.get_counters_orig()](IterableConntrackAttrs::get_counters_orig)\n- [.get_counters_reply()](IterableConntrackAttrs::get_counters_reply)\n- [.get_use()](IterableConntrackAttrs::get_use)\n- [.get_id()](IterableConntrackAttrs::get_id)\n- [.get_nat_dst()](IterableConntrackAttrs::get_nat_dst)\n- [.get_tuple_master()](IterableConntrackAttrs::get_tuple_master)\n- [.get_seq_adj_orig()](IterableConntrackAttrs::get_seq_adj_orig)\n- [.get_seq_adj_reply()](IterableConntrackAttrs::get_seq_adj_reply)\n- [.get_zone()](IterableConntrackAttrs::get_zone)\n- [.get_secctx()](IterableConntrackAttrs::get_secctx)\n- [.get_labels()](IterableConntrackAttrs::get_labels)\n- [.get_synproxy()](IterableConntrackAttrs::get_synproxy)\n\n"]
    pub fn op_get_do(self, header: &Nfgenmsg) -> OpGetDo<'buf> {
        let mut res = OpGetDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-get-do", OpGetDo::lookup);
        res
    }
    #[doc = "dump pcpu conntrack stats\n\nReply attributes:\n- [.get_searched()](IterableConntrackStatsAttrs::get_searched)\n- [.get_found()](IterableConntrackStatsAttrs::get_found)\n- [.get_insert()](IterableConntrackStatsAttrs::get_insert)\n- [.get_insert_failed()](IterableConntrackStatsAttrs::get_insert_failed)\n- [.get_drop()](IterableConntrackStatsAttrs::get_drop)\n- [.get_early_drop()](IterableConntrackStatsAttrs::get_early_drop)\n- [.get_error()](IterableConntrackStatsAttrs::get_error)\n- [.get_search_restart()](IterableConntrackStatsAttrs::get_search_restart)\n- [.get_clash_resolve()](IterableConntrackStatsAttrs::get_clash_resolve)\n- [.get_chain_toolong()](IterableConntrackStatsAttrs::get_chain_toolong)\n\n"]
    pub fn op_get_stats_dump(self, header: &Nfgenmsg) -> OpGetStatsDump<'buf> {
        let mut res = OpGetStatsDump::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-get-stats-dump", OpGetStatsDump::lookup);
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableConntrackAttrs::get_counters_orig;
        let _ = IterableConntrackAttrs::get_counters_reply;
        let _ = IterableConntrackAttrs::get_help;
        let _ = IterableConntrackAttrs::get_id;
        let _ = IterableConntrackAttrs::get_labels;
        let _ = IterableConntrackAttrs::get_mark;
        let _ = IterableConntrackAttrs::get_nat_dst;
        let _ = IterableConntrackAttrs::get_nat_src;
        let _ = IterableConntrackAttrs::get_protoinfo;
        let _ = IterableConntrackAttrs::get_secctx;
        let _ = IterableConntrackAttrs::get_seq_adj_orig;
        let _ = IterableConntrackAttrs::get_seq_adj_reply;
        let _ = IterableConntrackAttrs::get_status;
        let _ = IterableConntrackAttrs::get_synproxy;
        let _ = IterableConntrackAttrs::get_timeout;
        let _ = IterableConntrackAttrs::get_tuple_master;
        let _ = IterableConntrackAttrs::get_tuple_orig;
        let _ = IterableConntrackAttrs::get_tuple_reply;
        let _ = IterableConntrackAttrs::get_use;
        let _ = IterableConntrackAttrs::get_zone;
        let _ = IterableConntrackStatsAttrs::get_chain_toolong;
        let _ = IterableConntrackStatsAttrs::get_clash_resolve;
        let _ = IterableConntrackStatsAttrs::get_drop;
        let _ = IterableConntrackStatsAttrs::get_early_drop;
        let _ = IterableConntrackStatsAttrs::get_error;
        let _ = IterableConntrackStatsAttrs::get_found;
        let _ = IterableConntrackStatsAttrs::get_insert;
        let _ = IterableConntrackStatsAttrs::get_insert_failed;
        let _ = IterableConntrackStatsAttrs::get_search_restart;
        let _ = IterableConntrackStatsAttrs::get_searched;
        let _ = PushConntrackAttrs::<&mut Vec<u8>>::nested_filter;
        let _ = PushConntrackAttrs::<&mut Vec<u8>>::nested_tuple_orig;
        let _ = PushConntrackAttrs::<&mut Vec<u8>>::nested_tuple_reply;
        let _ = PushConntrackAttrs::<&mut Vec<u8>>::push_mark;
        let _ = PushConntrackAttrs::<&mut Vec<u8>>::push_status;
        let _ = PushConntrackAttrs::<&mut Vec<u8>>::push_zone;
    }
}
