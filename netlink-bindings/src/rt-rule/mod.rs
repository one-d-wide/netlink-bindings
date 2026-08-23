#![doc = "FIB rule management over rtnetlink.\n"]
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
pub const PROTONAME: &str = "rt-rule";
pub const PROTONAME_CSTR: &CStr = c"rt-rule";
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
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Rtgenmsg {
    pub family: u8,
    pub _pad: [u8; 3usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for Rtgenmsg {
    fn default() -> Self {
        Self::new()
    }
}
impl Rtgenmsg {
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
        const _: () = assert!(std::mem::size_of::<Rtgenmsg>() == 4usize);
        const _: () = assert!(std::mem::align_of::<Rtgenmsg>() == 1usize);
        4usize
    }
}
impl std::fmt::Debug for Rtgenmsg {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Rtgenmsg")
            .field("family", &self.family)
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct FibRuleHdr {
    pub family: u8,
    pub dst_len: u8,
    pub src_len: u8,
    pub tos: u8,
    pub table: u8,
    pub _res1: [u8; 1usize],
    pub _res2: [u8; 1usize],
    #[doc = "Associated type: [`FrAct`] (enum)"]
    pub action: u8,
    pub flags: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for FibRuleHdr {
    fn default() -> Self {
        Self::new()
    }
}
impl FibRuleHdr {
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
    pub fn new_from_array(buf: [u8; 12usize]) -> Self {
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
    pub fn as_array(&self) -> &[u8; 12usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 12usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 12usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<FibRuleHdr>() == 12usize);
        const _: () = assert!(std::mem::align_of::<FibRuleHdr>() == 4usize);
        12usize
    }
}
impl std::fmt::Debug for FibRuleHdr {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("FibRuleHdr")
            .field("family", &self.family)
            .field("dst_len", &self.dst_len)
            .field("src_len", &self.src_len)
            .field("tos", &self.tos)
            .field("table", &self.table)
            .field("action", &FormatEnum(self.action.into(), FrAct::from_value))
            .field("flags", &self.flags)
            .finish()
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct FibRulePortRange {
    pub start: u16,
    pub end: u16,
}
#[doc = "Create zero-initialized struct"]
impl Default for FibRulePortRange {
    fn default() -> Self {
        Self::new()
    }
}
impl FibRulePortRange {
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
        const _: () = assert!(std::mem::size_of::<FibRulePortRange>() == 4usize);
        const _: () = assert!(std::mem::align_of::<FibRulePortRange>() == 2usize);
        4usize
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct FibRuleUidRange {
    pub start: u32,
    pub end: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for FibRuleUidRange {
    fn default() -> Self {
        Self::new()
    }
}
impl FibRuleUidRange {
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
    pub fn new_from_array(buf: [u8; 8usize]) -> Self {
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
    pub fn as_array(&self) -> &[u8; 8usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 8usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 8usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<FibRuleUidRange>() == 8usize);
        const _: () = assert!(std::mem::align_of::<FibRuleUidRange>() == 4usize);
        8usize
    }
}
#[derive(Clone)]
pub enum FibRuleAttrs<'a> {
    Dst(std::net::IpAddr),
    Src(std::net::IpAddr),
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
    UidRange(FibRuleUidRange),
    Protocol(u8),
    IpProto(u8),
    SportRange(FibRulePortRange),
    DportRange(FibRulePortRange),
    Dscp(u8),
    Flowlabel(u32),
    FlowlabelMask(u32),
    SportMask(u16),
    DportMask(u16),
    DscpMask(u8),
}
impl<'a> IterableFibRuleAttrs<'a> {
    pub fn get_dst(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FibRuleAttrs::Dst(val)) = attr {
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
    pub fn get_src(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FibRuleAttrs::Src(val)) = attr {
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
            if let Ok(FibRuleAttrs::Iifname(val)) = attr {
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
            if let Ok(FibRuleAttrs::Goto(val)) = attr {
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
            if let Ok(FibRuleAttrs::Unused2(val)) = attr {
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
            if let Ok(FibRuleAttrs::Priority(val)) = attr {
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
            if let Ok(FibRuleAttrs::Unused3(val)) = attr {
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
            if let Ok(FibRuleAttrs::Unused4(val)) = attr {
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
            if let Ok(FibRuleAttrs::Unused5(val)) = attr {
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
            if let Ok(FibRuleAttrs::Fwmark(val)) = attr {
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
            if let Ok(FibRuleAttrs::Flow(val)) = attr {
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
            if let Ok(FibRuleAttrs::TunId(val)) = attr {
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
            if let Ok(FibRuleAttrs::SuppressIfgroup(val)) = attr {
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
            if let Ok(FibRuleAttrs::SuppressPrefixlen(val)) = attr {
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
            if let Ok(FibRuleAttrs::Table(val)) = attr {
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
            if let Ok(FibRuleAttrs::Fwmask(val)) = attr {
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
            if let Ok(FibRuleAttrs::Oifname(val)) = attr {
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
            if let Ok(FibRuleAttrs::Pad(val)) = attr {
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
            if let Ok(FibRuleAttrs::L3mdev(val)) = attr {
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
    pub fn get_uid_range(&self) -> Result<FibRuleUidRange, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FibRuleAttrs::UidRange(val)) = attr {
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
            if let Ok(FibRuleAttrs::Protocol(val)) = attr {
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
            if let Ok(FibRuleAttrs::IpProto(val)) = attr {
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
    pub fn get_sport_range(&self) -> Result<FibRulePortRange, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FibRuleAttrs::SportRange(val)) = attr {
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
    pub fn get_dport_range(&self) -> Result<FibRulePortRange, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FibRuleAttrs::DportRange(val)) = attr {
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
            if let Ok(FibRuleAttrs::Dscp(val)) = attr {
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
            if let Ok(FibRuleAttrs::Flowlabel(val)) = attr {
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
            if let Ok(FibRuleAttrs::FlowlabelMask(val)) = attr {
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
            if let Ok(FibRuleAttrs::SportMask(val)) = attr {
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
            if let Ok(FibRuleAttrs::DportMask(val)) = attr {
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
            if let Ok(FibRuleAttrs::DscpMask(val)) = attr {
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
impl FibRuleAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableFibRuleAttrs<'a> {
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
                1u16 => FibRuleAttrs::Dst({
                    let res = parse_ip(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => FibRuleAttrs::Src({
                    let res = parse_ip(next);
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
                    let res = Some(FibRuleUidRange::new_from_zeroed(next));
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
                    let res = Some(FibRulePortRange::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => FibRuleAttrs::DportRange({
                    let res = Some(FibRulePortRange::new_from_zeroed(next));
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
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
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
        let mut iter = IterableFibRuleAttrs::with_loc(&[], self.orig_loc);
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
        if missing_type.is_some() && cur == offset {
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
pub struct PushFibRuleAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushFibRuleAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushFibRuleAttrs<Prev> {
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
    pub fn push_dst(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_vec_mut(), 1u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_vec_mut(), value);
        self
    }
    pub fn push_src(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_vec_mut(), 2u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_vec_mut(), value);
        self
    }
    pub fn push_iifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            3u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_iifname_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_goto(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_unused2(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 5u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_priority(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_unused3(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 7u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_unused4(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 8u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_unused5(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 9u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_fwmark(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flow(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 11u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tun_id(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 12u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_suppress_ifgroup(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 13u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_suppress_prefixlen(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 14u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_table(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 15u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fwmask(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 16u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_oifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            17u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_oifname_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 17u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 18u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_l3mdev(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 19u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_uid_range(mut self, value: FibRuleUidRange) -> Self {
        push_header(self.as_vec_mut(), 20u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_protocol(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 21u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ip_proto(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 22u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sport_range(mut self, value: FibRulePortRange) -> Self {
        push_header(self.as_vec_mut(), 23u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_dport_range(mut self, value: FibRulePortRange) -> Self {
        push_header(self.as_vec_mut(), 24u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_dscp(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 25u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flowlabel(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 26u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_flowlabel_mask(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 27u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_sport_mask(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 28u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dport_mask(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 29u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dscp_mask(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 30u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushFibRuleAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Add new FIB rule\n\nRequest attributes:\n- [.push_iifname()](PushFibRuleAttrs::push_iifname)\n- [.push_goto()](PushFibRuleAttrs::push_goto)\n- [.push_priority()](PushFibRuleAttrs::push_priority)\n- [.push_fwmark()](PushFibRuleAttrs::push_fwmark)\n- [.push_flow()](PushFibRuleAttrs::push_flow)\n- [.push_tun_id()](PushFibRuleAttrs::push_tun_id)\n- [.push_suppress_ifgroup()](PushFibRuleAttrs::push_suppress_ifgroup)\n- [.push_suppress_prefixlen()](PushFibRuleAttrs::push_suppress_prefixlen)\n- [.push_table()](PushFibRuleAttrs::push_table)\n- [.push_fwmask()](PushFibRuleAttrs::push_fwmask)\n- [.push_oifname()](PushFibRuleAttrs::push_oifname)\n- [.push_l3mdev()](PushFibRuleAttrs::push_l3mdev)\n- [.push_uid_range()](PushFibRuleAttrs::push_uid_range)\n- [.push_protocol()](PushFibRuleAttrs::push_protocol)\n- [.push_ip_proto()](PushFibRuleAttrs::push_ip_proto)\n- [.push_sport_range()](PushFibRuleAttrs::push_sport_range)\n- [.push_dport_range()](PushFibRuleAttrs::push_dport_range)\n- [.push_dscp()](PushFibRuleAttrs::push_dscp)\n- [.push_flowlabel()](PushFibRuleAttrs::push_flowlabel)\n- [.push_flowlabel_mask()](PushFibRuleAttrs::push_flowlabel_mask)\n- [.push_sport_mask()](PushFibRuleAttrs::push_sport_mask)\n- [.push_dport_mask()](PushFibRuleAttrs::push_dport_mask)\n- [.push_dscp_mask()](PushFibRuleAttrs::push_dscp_mask)\n\n"]
#[derive(Debug)]
pub struct OpNewruleDo<'r> {
    request: Request<'r>,
}
impl<'r> OpNewruleDo<'r> {
    pub fn new(mut request: Request<'r>, header: &FibRuleHdr) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &FibRuleHdr,
    ) -> PushFibRuleAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushFibRuleAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushFibRuleAttrs<&mut Vec<u8>> {
        PushFibRuleAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushFibRuleAttrs<RequestBuf<'r>> {
        PushFibRuleAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (FibRuleHdr, IterableFibRuleAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(FibRuleHdr::len()));
        (
            FibRuleHdr::new_from_slice(header).unwrap_or_default(),
            IterableFibRuleAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &FibRuleHdr) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &FibRuleHdr {
        let pos = self.request.pos;
        FibRuleHdr::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut FibRuleHdr {
        let pos = self.request.pos;
        FibRuleHdr::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpNewruleDo<'_> {
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
    type ReplyType<'buf> = (FibRuleHdr, IterableFibRuleAttrs<'buf>);
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
#[doc = "Remove an existing FIB rule\n\nRequest attributes:\n- [.push_iifname()](PushFibRuleAttrs::push_iifname)\n- [.push_goto()](PushFibRuleAttrs::push_goto)\n- [.push_priority()](PushFibRuleAttrs::push_priority)\n- [.push_fwmark()](PushFibRuleAttrs::push_fwmark)\n- [.push_flow()](PushFibRuleAttrs::push_flow)\n- [.push_tun_id()](PushFibRuleAttrs::push_tun_id)\n- [.push_suppress_ifgroup()](PushFibRuleAttrs::push_suppress_ifgroup)\n- [.push_suppress_prefixlen()](PushFibRuleAttrs::push_suppress_prefixlen)\n- [.push_table()](PushFibRuleAttrs::push_table)\n- [.push_fwmask()](PushFibRuleAttrs::push_fwmask)\n- [.push_oifname()](PushFibRuleAttrs::push_oifname)\n- [.push_l3mdev()](PushFibRuleAttrs::push_l3mdev)\n- [.push_uid_range()](PushFibRuleAttrs::push_uid_range)\n- [.push_protocol()](PushFibRuleAttrs::push_protocol)\n- [.push_ip_proto()](PushFibRuleAttrs::push_ip_proto)\n- [.push_sport_range()](PushFibRuleAttrs::push_sport_range)\n- [.push_dport_range()](PushFibRuleAttrs::push_dport_range)\n- [.push_dscp()](PushFibRuleAttrs::push_dscp)\n- [.push_flowlabel()](PushFibRuleAttrs::push_flowlabel)\n- [.push_flowlabel_mask()](PushFibRuleAttrs::push_flowlabel_mask)\n- [.push_sport_mask()](PushFibRuleAttrs::push_sport_mask)\n- [.push_dport_mask()](PushFibRuleAttrs::push_dport_mask)\n- [.push_dscp_mask()](PushFibRuleAttrs::push_dscp_mask)\n\n"]
#[derive(Debug)]
pub struct OpDelruleDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDelruleDo<'r> {
    pub fn new(mut request: Request<'r>, header: &FibRuleHdr) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &FibRuleHdr,
    ) -> PushFibRuleAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushFibRuleAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushFibRuleAttrs<&mut Vec<u8>> {
        PushFibRuleAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushFibRuleAttrs<RequestBuf<'r>> {
        PushFibRuleAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (FibRuleHdr, IterableFibRuleAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(FibRuleHdr::len()));
        (
            FibRuleHdr::new_from_slice(header).unwrap_or_default(),
            IterableFibRuleAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &FibRuleHdr) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &FibRuleHdr {
        let pos = self.request.pos;
        FibRuleHdr::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut FibRuleHdr {
        let pos = self.request.pos;
        FibRuleHdr::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpDelruleDo<'_> {
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
    type ReplyType<'buf> = (FibRuleHdr, IterableFibRuleAttrs<'buf>);
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
#[doc = "Dump all FIB rules\n\nReply attributes:\n- [.get_iifname()](IterableFibRuleAttrs::get_iifname)\n- [.get_goto()](IterableFibRuleAttrs::get_goto)\n- [.get_priority()](IterableFibRuleAttrs::get_priority)\n- [.get_fwmark()](IterableFibRuleAttrs::get_fwmark)\n- [.get_flow()](IterableFibRuleAttrs::get_flow)\n- [.get_tun_id()](IterableFibRuleAttrs::get_tun_id)\n- [.get_suppress_ifgroup()](IterableFibRuleAttrs::get_suppress_ifgroup)\n- [.get_suppress_prefixlen()](IterableFibRuleAttrs::get_suppress_prefixlen)\n- [.get_table()](IterableFibRuleAttrs::get_table)\n- [.get_fwmask()](IterableFibRuleAttrs::get_fwmask)\n- [.get_oifname()](IterableFibRuleAttrs::get_oifname)\n- [.get_l3mdev()](IterableFibRuleAttrs::get_l3mdev)\n- [.get_uid_range()](IterableFibRuleAttrs::get_uid_range)\n- [.get_protocol()](IterableFibRuleAttrs::get_protocol)\n- [.get_ip_proto()](IterableFibRuleAttrs::get_ip_proto)\n- [.get_sport_range()](IterableFibRuleAttrs::get_sport_range)\n- [.get_dport_range()](IterableFibRuleAttrs::get_dport_range)\n- [.get_dscp()](IterableFibRuleAttrs::get_dscp)\n- [.get_flowlabel()](IterableFibRuleAttrs::get_flowlabel)\n- [.get_flowlabel_mask()](IterableFibRuleAttrs::get_flowlabel_mask)\n- [.get_sport_mask()](IterableFibRuleAttrs::get_sport_mask)\n- [.get_dport_mask()](IterableFibRuleAttrs::get_dport_mask)\n- [.get_dscp_mask()](IterableFibRuleAttrs::get_dscp_mask)\n\n"]
#[derive(Debug)]
pub struct OpGetruleDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetruleDump<'r> {
    pub fn new(mut request: Request<'r>, header: &FibRuleHdr) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &FibRuleHdr,
    ) -> PushFibRuleAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushFibRuleAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushFibRuleAttrs<&mut Vec<u8>> {
        PushFibRuleAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushFibRuleAttrs<RequestBuf<'r>> {
        PushFibRuleAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (FibRuleHdr, IterableFibRuleAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(FibRuleHdr::len()));
        (
            FibRuleHdr::new_from_slice(header).unwrap_or_default(),
            IterableFibRuleAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &FibRuleHdr) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &FibRuleHdr {
        let pos = self.request.pos;
        FibRuleHdr::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut FibRuleHdr {
        let pos = self.request.pos;
        FibRuleHdr::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetruleDump<'_> {
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
    type ReplyType<'buf> = (FibRuleHdr, IterableFibRuleAttrs<'buf>);
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
    #[doc = "Add new FIB rule\n\nRequest attributes:\n- [.push_iifname()](PushFibRuleAttrs::push_iifname)\n- [.push_goto()](PushFibRuleAttrs::push_goto)\n- [.push_priority()](PushFibRuleAttrs::push_priority)\n- [.push_fwmark()](PushFibRuleAttrs::push_fwmark)\n- [.push_flow()](PushFibRuleAttrs::push_flow)\n- [.push_tun_id()](PushFibRuleAttrs::push_tun_id)\n- [.push_suppress_ifgroup()](PushFibRuleAttrs::push_suppress_ifgroup)\n- [.push_suppress_prefixlen()](PushFibRuleAttrs::push_suppress_prefixlen)\n- [.push_table()](PushFibRuleAttrs::push_table)\n- [.push_fwmask()](PushFibRuleAttrs::push_fwmask)\n- [.push_oifname()](PushFibRuleAttrs::push_oifname)\n- [.push_l3mdev()](PushFibRuleAttrs::push_l3mdev)\n- [.push_uid_range()](PushFibRuleAttrs::push_uid_range)\n- [.push_protocol()](PushFibRuleAttrs::push_protocol)\n- [.push_ip_proto()](PushFibRuleAttrs::push_ip_proto)\n- [.push_sport_range()](PushFibRuleAttrs::push_sport_range)\n- [.push_dport_range()](PushFibRuleAttrs::push_dport_range)\n- [.push_dscp()](PushFibRuleAttrs::push_dscp)\n- [.push_flowlabel()](PushFibRuleAttrs::push_flowlabel)\n- [.push_flowlabel_mask()](PushFibRuleAttrs::push_flowlabel_mask)\n- [.push_sport_mask()](PushFibRuleAttrs::push_sport_mask)\n- [.push_dport_mask()](PushFibRuleAttrs::push_dport_mask)\n- [.push_dscp_mask()](PushFibRuleAttrs::push_dscp_mask)\n\n"]
    pub fn op_newrule_do(self, header: &FibRuleHdr) -> OpNewruleDo<'buf> {
        let mut res = OpNewruleDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-newrule-do", OpNewruleDo::lookup);
        res
    }
    #[doc = "Remove an existing FIB rule\n\nRequest attributes:\n- [.push_iifname()](PushFibRuleAttrs::push_iifname)\n- [.push_goto()](PushFibRuleAttrs::push_goto)\n- [.push_priority()](PushFibRuleAttrs::push_priority)\n- [.push_fwmark()](PushFibRuleAttrs::push_fwmark)\n- [.push_flow()](PushFibRuleAttrs::push_flow)\n- [.push_tun_id()](PushFibRuleAttrs::push_tun_id)\n- [.push_suppress_ifgroup()](PushFibRuleAttrs::push_suppress_ifgroup)\n- [.push_suppress_prefixlen()](PushFibRuleAttrs::push_suppress_prefixlen)\n- [.push_table()](PushFibRuleAttrs::push_table)\n- [.push_fwmask()](PushFibRuleAttrs::push_fwmask)\n- [.push_oifname()](PushFibRuleAttrs::push_oifname)\n- [.push_l3mdev()](PushFibRuleAttrs::push_l3mdev)\n- [.push_uid_range()](PushFibRuleAttrs::push_uid_range)\n- [.push_protocol()](PushFibRuleAttrs::push_protocol)\n- [.push_ip_proto()](PushFibRuleAttrs::push_ip_proto)\n- [.push_sport_range()](PushFibRuleAttrs::push_sport_range)\n- [.push_dport_range()](PushFibRuleAttrs::push_dport_range)\n- [.push_dscp()](PushFibRuleAttrs::push_dscp)\n- [.push_flowlabel()](PushFibRuleAttrs::push_flowlabel)\n- [.push_flowlabel_mask()](PushFibRuleAttrs::push_flowlabel_mask)\n- [.push_sport_mask()](PushFibRuleAttrs::push_sport_mask)\n- [.push_dport_mask()](PushFibRuleAttrs::push_dport_mask)\n- [.push_dscp_mask()](PushFibRuleAttrs::push_dscp_mask)\n\n"]
    pub fn op_delrule_do(self, header: &FibRuleHdr) -> OpDelruleDo<'buf> {
        let mut res = OpDelruleDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-delrule-do", OpDelruleDo::lookup);
        res
    }
    #[doc = "Dump all FIB rules\n\nReply attributes:\n- [.get_iifname()](IterableFibRuleAttrs::get_iifname)\n- [.get_goto()](IterableFibRuleAttrs::get_goto)\n- [.get_priority()](IterableFibRuleAttrs::get_priority)\n- [.get_fwmark()](IterableFibRuleAttrs::get_fwmark)\n- [.get_flow()](IterableFibRuleAttrs::get_flow)\n- [.get_tun_id()](IterableFibRuleAttrs::get_tun_id)\n- [.get_suppress_ifgroup()](IterableFibRuleAttrs::get_suppress_ifgroup)\n- [.get_suppress_prefixlen()](IterableFibRuleAttrs::get_suppress_prefixlen)\n- [.get_table()](IterableFibRuleAttrs::get_table)\n- [.get_fwmask()](IterableFibRuleAttrs::get_fwmask)\n- [.get_oifname()](IterableFibRuleAttrs::get_oifname)\n- [.get_l3mdev()](IterableFibRuleAttrs::get_l3mdev)\n- [.get_uid_range()](IterableFibRuleAttrs::get_uid_range)\n- [.get_protocol()](IterableFibRuleAttrs::get_protocol)\n- [.get_ip_proto()](IterableFibRuleAttrs::get_ip_proto)\n- [.get_sport_range()](IterableFibRuleAttrs::get_sport_range)\n- [.get_dport_range()](IterableFibRuleAttrs::get_dport_range)\n- [.get_dscp()](IterableFibRuleAttrs::get_dscp)\n- [.get_flowlabel()](IterableFibRuleAttrs::get_flowlabel)\n- [.get_flowlabel_mask()](IterableFibRuleAttrs::get_flowlabel_mask)\n- [.get_sport_mask()](IterableFibRuleAttrs::get_sport_mask)\n- [.get_dport_mask()](IterableFibRuleAttrs::get_dport_mask)\n- [.get_dscp_mask()](IterableFibRuleAttrs::get_dscp_mask)\n\n"]
    pub fn op_getrule_dump(self, header: &FibRuleHdr) -> OpGetruleDump<'buf> {
        let mut res = OpGetruleDump::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getrule-dump", OpGetruleDump::lookup);
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableFibRuleAttrs::get_dport_mask;
        let _ = IterableFibRuleAttrs::get_dport_range;
        let _ = IterableFibRuleAttrs::get_dscp;
        let _ = IterableFibRuleAttrs::get_dscp_mask;
        let _ = IterableFibRuleAttrs::get_flow;
        let _ = IterableFibRuleAttrs::get_flowlabel;
        let _ = IterableFibRuleAttrs::get_flowlabel_mask;
        let _ = IterableFibRuleAttrs::get_fwmark;
        let _ = IterableFibRuleAttrs::get_fwmask;
        let _ = IterableFibRuleAttrs::get_goto;
        let _ = IterableFibRuleAttrs::get_iifname;
        let _ = IterableFibRuleAttrs::get_ip_proto;
        let _ = IterableFibRuleAttrs::get_l3mdev;
        let _ = IterableFibRuleAttrs::get_oifname;
        let _ = IterableFibRuleAttrs::get_priority;
        let _ = IterableFibRuleAttrs::get_protocol;
        let _ = IterableFibRuleAttrs::get_sport_mask;
        let _ = IterableFibRuleAttrs::get_sport_range;
        let _ = IterableFibRuleAttrs::get_suppress_ifgroup;
        let _ = IterableFibRuleAttrs::get_suppress_prefixlen;
        let _ = IterableFibRuleAttrs::get_table;
        let _ = IterableFibRuleAttrs::get_tun_id;
        let _ = IterableFibRuleAttrs::get_uid_range;
        let _ = PushFibRuleAttrs::<&mut Vec<u8>>::push_dport_mask;
        let _ = PushFibRuleAttrs::<&mut Vec<u8>>::push_dport_range;
        let _ = PushFibRuleAttrs::<&mut Vec<u8>>::push_dscp;
        let _ = PushFibRuleAttrs::<&mut Vec<u8>>::push_dscp_mask;
        let _ = PushFibRuleAttrs::<&mut Vec<u8>>::push_flow;
        let _ = PushFibRuleAttrs::<&mut Vec<u8>>::push_flowlabel;
        let _ = PushFibRuleAttrs::<&mut Vec<u8>>::push_flowlabel_mask;
        let _ = PushFibRuleAttrs::<&mut Vec<u8>>::push_fwmark;
        let _ = PushFibRuleAttrs::<&mut Vec<u8>>::push_fwmask;
        let _ = PushFibRuleAttrs::<&mut Vec<u8>>::push_goto;
        let _ = PushFibRuleAttrs::<&mut Vec<u8>>::push_iifname;
        let _ = PushFibRuleAttrs::<&mut Vec<u8>>::push_ip_proto;
        let _ = PushFibRuleAttrs::<&mut Vec<u8>>::push_l3mdev;
        let _ = PushFibRuleAttrs::<&mut Vec<u8>>::push_oifname;
        let _ = PushFibRuleAttrs::<&mut Vec<u8>>::push_priority;
        let _ = PushFibRuleAttrs::<&mut Vec<u8>>::push_protocol;
        let _ = PushFibRuleAttrs::<&mut Vec<u8>>::push_sport_mask;
        let _ = PushFibRuleAttrs::<&mut Vec<u8>>::push_sport_range;
        let _ = PushFibRuleAttrs::<&mut Vec<u8>>::push_suppress_ifgroup;
        let _ = PushFibRuleAttrs::<&mut Vec<u8>>::push_suppress_prefixlen;
        let _ = PushFibRuleAttrs::<&mut Vec<u8>>::push_table;
        let _ = PushFibRuleAttrs::<&mut Vec<u8>>::push_tun_id;
        let _ = PushFibRuleAttrs::<&mut Vec<u8>>::push_uid_range;
    }
}
