#![doc = "Nexthop object management over rtnetlink.\n"]
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
pub const PROTONAME: &str = "rt-nexthop";
pub const PROTONAME_CSTR: &CStr = c"rt-nexthop";
pub const PROTONUM: u16 = 0u16;
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum RtnhF {
    Dead = 1 << 0,
    Pervasive = 1 << 1,
    Onlink = 1 << 2,
    Offload = 1 << 3,
    Linkdown = 1 << 4,
    Unresolved = 1 << 5,
    Trap = 1 << 6,
}
impl RtnhF {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::Dead,
            n if n == 1 << 1 => Self::Pervasive,
            n if n == 1 << 2 => Self::Onlink,
            n if n == 1 << 3 => Self::Offload,
            n if n == 1 << 4 => Self::Linkdown,
            n if n == 1 << 5 => Self::Unresolved,
            n if n == 1 << 6 => Self::Trap,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum NexthopGrpType {
    Mpath = 0,
    Res = 1,
}
impl NexthopGrpType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Mpath,
            1 => Self::Res,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum EncapType {
    None = 0,
    Mpls = 1,
    Ip = 2,
    Ila = 3,
    Ip6 = 4,
    Seg6 = 5,
    Bpf = 6,
    Seg6Local = 7,
    Rpl = 8,
    Ioam6 = 9,
    Xfrm = 10,
}
impl EncapType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::None,
            1 => Self::Mpls,
            2 => Self::Ip,
            3 => Self::Ila,
            4 => Self::Ip6,
            5 => Self::Seg6,
            6 => Self::Bpf,
            7 => Self::Seg6Local,
            8 => Self::Rpl,
            9 => Self::Ioam6,
            10 => Self::Xfrm,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum Seg6LocalActions {
    Unspec = 0,
    End = 1,
    EndX = 2,
    EndT = 3,
    EndDx2 = 4,
    EndDx6 = 5,
    EndDx4 = 6,
    EndDt6 = 7,
    EndDt4 = 8,
    EndB6 = 9,
    EndB6Encap = 10,
    EndBm = 11,
    EndS = 12,
    EndAs = 13,
    EndAm = 14,
    EndBpf = 15,
    EndDt46 = 16,
}
impl Seg6LocalActions {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Unspec,
            1 => Self::End,
            2 => Self::EndX,
            3 => Self::EndT,
            4 => Self::EndDx2,
            5 => Self::EndDx6,
            6 => Self::EndDx4,
            7 => Self::EndDt6,
            8 => Self::EndDt4,
            9 => Self::EndB6,
            10 => Self::EndB6Encap,
            11 => Self::EndBm,
            12 => Self::EndS,
            13 => Self::EndAs,
            14 => Self::EndAm,
            15 => Self::EndBpf,
            16 => Self::EndDt46,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum Seg6LocalFlvOps {
    Unspec = 0,
    Psp = 1,
    Usp = 2,
    Usd = 3,
    NextCsid = 4,
}
impl Seg6LocalFlvOps {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Unspec,
            1 => Self::Psp,
            2 => Self::Usp,
            3 => Self::Usd,
            4 => Self::NextCsid,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum NhaOpFlag {
    DumpStats = 1 << 0,
    DumpHwStats = 1 << 1,
}
impl NhaOpFlag {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::DumpStats,
            n if n == 1 << 1 => Self::DumpHwStats,
            _ => return None,
        })
    }
}
pub const NHA_OP_FLAG_RESP_GRP_RESVD_0: u64 = 2147483648u64;
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Nhmsg {
    pub nh_family: u8,
    pub nh_scope: u8,
    pub nh_protocol: u8,
    pub _resvd: [u8; 1usize],
    #[doc = "Associated type: [`RtnhF`] (enum)"]
    pub nh_flags: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for Nhmsg {
    fn default() -> Self {
        Self::new()
    }
}
impl Nhmsg {
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
        const _: () = assert!(std::mem::size_of::<Nhmsg>() == 8usize);
        const _: () = assert!(std::mem::align_of::<Nhmsg>() == 4usize);
        8usize
    }
}
impl std::fmt::Debug for Nhmsg {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Nhmsg")
            .field("nh_family", &self.nh_family)
            .field("nh_scope", &self.nh_scope)
            .field("nh_protocol", &self.nh_protocol)
            .field(
                "nh_flags",
                &FormatFlags(self.nh_flags.into(), RtnhF::from_value),
            )
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct NexthopGrp {
    pub id: u32,
    pub weight: u8,
    pub weight_high: u8,
    pub _resvd2: [u8; 2usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for NexthopGrp {
    fn default() -> Self {
        Self::new()
    }
}
impl NexthopGrp {
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
        const _: () = assert!(std::mem::size_of::<NexthopGrp>() == 8usize);
        const _: () = assert!(std::mem::align_of::<NexthopGrp>() == 4usize);
        8usize
    }
}
impl std::fmt::Debug for NexthopGrp {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("NexthopGrp")
            .field("id", &self.id)
            .field("weight", &self.weight)
            .field("weight_high", &self.weight_high)
            .finish()
    }
}
#[derive(Clone)]
pub enum NexthopAttrs<'a> {
    Id(u32),
    Group(&'a [u8]),
    #[doc = "Associated type: [`NexthopGrpType`] (enum)"]
    GroupType(u16),
    Blackhole(()),
    Oif(u32),
    Gateway(std::net::IpAddr),
    #[doc = "Associated type: [`EncapType`] (enum)"]
    EncapType(u16),
    Encap(EncapData<'a>),
    Groups(()),
    Master(u32),
    Fdb(()),
    ResGroup(IterableResGroupAttrs<'a>),
    ResBucket(IterableResBucketAttrs<'a>),
    OpFlags(u32),
    GroupStats(IterableGroupStatsAttrs<'a>),
    HwStatsEnable(u32),
    HwStatsUsed(u32),
    DstPort(u16),
}
impl<'a> IterableNexthopAttrs<'a> {
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NexthopAttrs::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NexthopAttrs",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_group(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NexthopAttrs::Group(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NexthopAttrs",
            "Group",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`NexthopGrpType`] (enum)"]
    pub fn get_group_type(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NexthopAttrs::GroupType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NexthopAttrs",
            "GroupType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_blackhole(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NexthopAttrs::Blackhole(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NexthopAttrs",
            "Blackhole",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_oif(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NexthopAttrs::Oif(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NexthopAttrs",
            "Oif",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_gateway(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NexthopAttrs::Gateway(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NexthopAttrs",
            "Gateway",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`EncapType`] (enum)"]
    pub fn get_encap_type(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NexthopAttrs::EncapType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NexthopAttrs",
            "EncapType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap(&self) -> Result<EncapData<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NexthopAttrs::Encap(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NexthopAttrs",
            "Encap",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_groups(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NexthopAttrs::Groups(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NexthopAttrs",
            "Groups",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_master(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NexthopAttrs::Master(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NexthopAttrs",
            "Master",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fdb(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NexthopAttrs::Fdb(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NexthopAttrs",
            "Fdb",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_res_group(&self) -> Result<IterableResGroupAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NexthopAttrs::ResGroup(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NexthopAttrs",
            "ResGroup",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_res_bucket(&self) -> Result<IterableResBucketAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NexthopAttrs::ResBucket(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NexthopAttrs",
            "ResBucket",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_op_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NexthopAttrs::OpFlags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NexthopAttrs",
            "OpFlags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_group_stats(&self) -> Result<IterableGroupStatsAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NexthopAttrs::GroupStats(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NexthopAttrs",
            "GroupStats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_hw_stats_enable(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NexthopAttrs::HwStatsEnable(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NexthopAttrs",
            "HwStatsEnable",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_hw_stats_used(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NexthopAttrs::HwStatsUsed(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NexthopAttrs",
            "HwStatsUsed",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dst_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NexthopAttrs::DstPort(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NexthopAttrs",
            "DstPort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
#[derive(Debug, Clone)]
pub enum EncapData<'a> {
    Mpls(IterableMplsIptunnel<'a>),
    Ip(IterableLwtunnelIp<'a>),
    Ila(IterableIlaAttrs<'a>),
    Ip6(IterableLwtunnelIp6<'a>),
    Seg6(IterableSeg6Iptunnel<'a>),
    Bpf(IterableLwtBpf<'a>),
    Seg6Local(IterableSeg6Local<'a>),
    Rpl(IterableRplIptunnel<'a>),
    Ioam6(IterableIoam6Iptunnel<'a>),
    Xfrm(IterableLwtXfrm<'a>),
}
impl<'a> EncapData<'a> {
    fn select_with_loc(selector: u16, buf: &'a [u8], loc: usize) -> Option<Self> {
        match selector {
            val if val == EncapType::Mpls as u16 => {
                Some(EncapData::Mpls(IterableMplsIptunnel::with_loc(buf, loc)))
            }
            val if val == EncapType::Ip as u16 => {
                Some(EncapData::Ip(IterableLwtunnelIp::with_loc(buf, loc)))
            }
            val if val == EncapType::Ila as u16 => {
                Some(EncapData::Ila(IterableIlaAttrs::with_loc(buf, loc)))
            }
            val if val == EncapType::Ip6 as u16 => {
                Some(EncapData::Ip6(IterableLwtunnelIp6::with_loc(buf, loc)))
            }
            val if val == EncapType::Seg6 as u16 => {
                Some(EncapData::Seg6(IterableSeg6Iptunnel::with_loc(buf, loc)))
            }
            val if val == EncapType::Bpf as u16 => {
                Some(EncapData::Bpf(IterableLwtBpf::with_loc(buf, loc)))
            }
            val if val == EncapType::Seg6Local as u16 => {
                Some(EncapData::Seg6Local(IterableSeg6Local::with_loc(buf, loc)))
            }
            val if val == EncapType::Rpl as u16 => {
                Some(EncapData::Rpl(IterableRplIptunnel::with_loc(buf, loc)))
            }
            val if val == EncapType::Ioam6 as u16 => {
                Some(EncapData::Ioam6(IterableIoam6Iptunnel::with_loc(buf, loc)))
            }
            val if val == EncapType::Xfrm as u16 => {
                Some(EncapData::Xfrm(IterableLwtXfrm::with_loc(buf, loc)))
            }
            _ => None,
        }
    }
}
impl NexthopAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableNexthopAttrs<'a> {
        IterableNexthopAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Id",
            2u16 => "Group",
            3u16 => "GroupType",
            4u16 => "Blackhole",
            5u16 => "Oif",
            6u16 => "Gateway",
            7u16 => "EncapType",
            8u16 => "Encap",
            9u16 => "Groups",
            10u16 => "Master",
            11u16 => "Fdb",
            12u16 => "ResGroup",
            13u16 => "ResBucket",
            14u16 => "OpFlags",
            15u16 => "GroupStats",
            16u16 => "HwStatsEnable",
            17u16 => "HwStatsUsed",
            18u16 => "DstPort",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableNexthopAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
    selector_encap_type: Option<u16>,
}
impl<'a> IterableNexthopAttrs<'a> {
    fn with_loc(buf: &'a [u8], orig_loc: usize) -> Self {
        Self {
            buf,
            pos: 0,
            orig_loc,
            selector_encap_type: None,
        }
    }
    pub fn get_buf(&self) -> &'a [u8] {
        self.buf
    }
}
impl<'a> Iterator for IterableNexthopAttrs<'a> {
    type Item = Result<NexthopAttrs<'a>, ErrorContext>;
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
                1u16 => NexthopAttrs::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => NexthopAttrs::Group({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => NexthopAttrs::GroupType({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => NexthopAttrs::Blackhole(()),
                5u16 => NexthopAttrs::Oif({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => NexthopAttrs::Gateway({
                    let res = parse_ip(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => NexthopAttrs::EncapType({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => NexthopAttrs::Encap({
                    let res = {
                        let Some(selector) = self.selector_encap_type else {
                            break;
                        };
                        match EncapData::select_with_loc(selector, next, self.orig_loc) {
                            Some(sub) => Some(sub),
                            None if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                            None => continue,
                        }
                    };
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => NexthopAttrs::Groups(()),
                10u16 => NexthopAttrs::Master({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => NexthopAttrs::Fdb(()),
                12u16 => NexthopAttrs::ResGroup({
                    let res = Some(IterableResGroupAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => NexthopAttrs::ResBucket({
                    let res = Some(IterableResBucketAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => NexthopAttrs::OpFlags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => NexthopAttrs::GroupStats({
                    let res = Some(IterableGroupStatsAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => NexthopAttrs::HwStatsEnable({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => NexthopAttrs::HwStatsUsed({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => NexthopAttrs::DstPort({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            if let NexthopAttrs::EncapType(sel) = &res {
                self.selector_encap_type = Some(*sel);
            }
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "NexthopAttrs",
            r#type.and_then(|t| NexthopAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableNexthopAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("NexthopAttrs");
        let mut iter = IterableNexthopAttrs::with_loc(&[], self.orig_loc);
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
                NexthopAttrs::Id(val) => fmt.field("Id", &val),
                NexthopAttrs::Group(val) => fmt.field("Group", &FormatHexdump(val)),
                NexthopAttrs::GroupType(val) => fmt.field(
                    "GroupType",
                    &FormatEnum(val.into(), NexthopGrpType::from_value),
                ),
                NexthopAttrs::Blackhole(val) => fmt.field("Blackhole", &val),
                NexthopAttrs::Oif(val) => fmt.field("Oif", &val),
                NexthopAttrs::Gateway(val) => fmt.field("Gateway", &val),
                NexthopAttrs::EncapType(val) => {
                    fmt.field("EncapType", &FormatEnum(val.into(), EncapType::from_value))
                }
                NexthopAttrs::Encap(val) => fmt.field("Encap", &val),
                NexthopAttrs::Groups(val) => fmt.field("Groups", &val),
                NexthopAttrs::Master(val) => fmt.field("Master", &val),
                NexthopAttrs::Fdb(val) => fmt.field("Fdb", &val),
                NexthopAttrs::ResGroup(val) => fmt.field("ResGroup", &val),
                NexthopAttrs::ResBucket(val) => fmt.field("ResBucket", &val),
                NexthopAttrs::OpFlags(val) => fmt.field("OpFlags", &val),
                NexthopAttrs::GroupStats(val) => fmt.field("GroupStats", &val),
                NexthopAttrs::HwStatsEnable(val) => fmt.field("HwStatsEnable", &val),
                NexthopAttrs::HwStatsUsed(val) => fmt.field("HwStatsUsed", &val),
                NexthopAttrs::DstPort(val) => fmt.field("DstPort", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableNexthopAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("NexthopAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| NexthopAttrs::attr_from_type(t)),
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
                NexthopAttrs::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                NexthopAttrs::Group(val) => {
                    if last_off == offset {
                        stack.push(("Group", last_off));
                        break;
                    }
                }
                NexthopAttrs::GroupType(val) => {
                    if last_off == offset {
                        stack.push(("GroupType", last_off));
                        break;
                    }
                }
                NexthopAttrs::Blackhole(val) => {
                    if last_off == offset {
                        stack.push(("Blackhole", last_off));
                        break;
                    }
                }
                NexthopAttrs::Oif(val) => {
                    if last_off == offset {
                        stack.push(("Oif", last_off));
                        break;
                    }
                }
                NexthopAttrs::Gateway(val) => {
                    if last_off == offset {
                        stack.push(("Gateway", last_off));
                        break;
                    }
                }
                NexthopAttrs::EncapType(val) => {
                    if last_off == offset {
                        stack.push(("EncapType", last_off));
                        break;
                    }
                }
                NexthopAttrs::Encap(val) => {
                    if last_off == offset {
                        stack.push(("Encap", last_off));
                        break;
                    }
                }
                NexthopAttrs::Groups(val) => {
                    if last_off == offset {
                        stack.push(("Groups", last_off));
                        break;
                    }
                }
                NexthopAttrs::Master(val) => {
                    if last_off == offset {
                        stack.push(("Master", last_off));
                        break;
                    }
                }
                NexthopAttrs::Fdb(val) => {
                    if last_off == offset {
                        stack.push(("Fdb", last_off));
                        break;
                    }
                }
                NexthopAttrs::ResGroup(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                NexthopAttrs::ResBucket(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                NexthopAttrs::OpFlags(val) => {
                    if last_off == offset {
                        stack.push(("OpFlags", last_off));
                        break;
                    }
                }
                NexthopAttrs::GroupStats(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                NexthopAttrs::HwStatsEnable(val) => {
                    if last_off == offset {
                        stack.push(("HwStatsEnable", last_off));
                        break;
                    }
                }
                NexthopAttrs::HwStatsUsed(val) => {
                    if last_off == offset {
                        stack.push(("HwStatsUsed", last_off));
                        break;
                    }
                }
                NexthopAttrs::DstPort(val) => {
                    if last_off == offset {
                        stack.push(("DstPort", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("NexthopAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum MplsIptunnel<'a> {
    Dst(&'a [u8]),
    Ttl(u8),
}
impl<'a> IterableMplsIptunnel<'a> {
    pub fn get_dst(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(MplsIptunnel::Dst(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "MplsIptunnel",
            "Dst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ttl(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(MplsIptunnel::Ttl(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "MplsIptunnel",
            "Ttl",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl MplsIptunnel<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableMplsIptunnel<'a> {
        IterableMplsIptunnel::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Dst",
            2u16 => "Ttl",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableMplsIptunnel<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableMplsIptunnel<'a> {
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
impl<'a> Iterator for IterableMplsIptunnel<'a> {
    type Item = Result<MplsIptunnel<'a>, ErrorContext>;
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
                1u16 => MplsIptunnel::Dst({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => MplsIptunnel::Ttl({
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
            "MplsIptunnel",
            r#type.and_then(|t| MplsIptunnel::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableMplsIptunnel<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("MplsIptunnel");
        let mut iter = IterableMplsIptunnel::with_loc(&[], self.orig_loc);
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
                MplsIptunnel::Dst(val) => fmt.field("Dst", &FormatHexdump(val)),
                MplsIptunnel::Ttl(val) => fmt.field("Ttl", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableMplsIptunnel<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("MplsIptunnel", offset));
            return (
                stack,
                missing_type.and_then(|t| MplsIptunnel::attr_from_type(t)),
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
                MplsIptunnel::Dst(val) => {
                    if last_off == offset {
                        stack.push(("Dst", last_off));
                        break;
                    }
                }
                MplsIptunnel::Ttl(val) => {
                    if last_off == offset {
                        stack.push(("Ttl", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("MplsIptunnel", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LwtunnelIp<'a> {
    Id(u64),
    Dst(std::net::Ipv4Addr),
    Src(std::net::Ipv4Addr),
    Ttl(u8),
    Tos(u8),
    Flags(u16),
    Pad(&'a [u8]),
    Opts(IterableLwtunnelIpOpts<'a>),
}
impl<'a> IterableLwtunnelIp<'a> {
    pub fn get_id(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtunnelIp::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtunnelIp",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dst(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtunnelIp::Dst(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtunnelIp",
            "Dst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_src(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtunnelIp::Src(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtunnelIp",
            "Src",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ttl(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtunnelIp::Ttl(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtunnelIp",
            "Ttl",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tos(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtunnelIp::Tos(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtunnelIp",
            "Tos",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flags(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtunnelIp::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtunnelIp",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtunnelIp::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtunnelIp",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_opts(&self) -> Result<IterableLwtunnelIpOpts<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtunnelIp::Opts(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtunnelIp",
            "Opts",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LwtunnelIp<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLwtunnelIp<'a> {
        IterableLwtunnelIp::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Id",
            2u16 => "Dst",
            3u16 => "Src",
            4u16 => "Ttl",
            5u16 => "Tos",
            6u16 => "Flags",
            7u16 => "Pad",
            8u16 => "Opts",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLwtunnelIp<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLwtunnelIp<'a> {
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
impl<'a> Iterator for IterableLwtunnelIp<'a> {
    type Item = Result<LwtunnelIp<'a>, ErrorContext>;
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
                1u16 => LwtunnelIp::Id({
                    let res = parse_be_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => LwtunnelIp::Dst({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => LwtunnelIp::Src({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => LwtunnelIp::Ttl({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => LwtunnelIp::Tos({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => LwtunnelIp::Flags({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => LwtunnelIp::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => LwtunnelIp::Opts({
                    let res = Some(IterableLwtunnelIpOpts::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "LwtunnelIp",
            r#type.and_then(|t| LwtunnelIp::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableLwtunnelIp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LwtunnelIp");
        let mut iter = IterableLwtunnelIp::with_loc(&[], self.orig_loc);
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
                LwtunnelIp::Id(val) => fmt.field("Id", &val),
                LwtunnelIp::Dst(val) => fmt.field("Dst", &val),
                LwtunnelIp::Src(val) => fmt.field("Src", &val),
                LwtunnelIp::Ttl(val) => fmt.field("Ttl", &val),
                LwtunnelIp::Tos(val) => fmt.field("Tos", &val),
                LwtunnelIp::Flags(val) => fmt.field("Flags", &val),
                LwtunnelIp::Pad(val) => fmt.field("Pad", &val),
                LwtunnelIp::Opts(val) => fmt.field("Opts", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableLwtunnelIp<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LwtunnelIp", offset));
            return (
                stack,
                missing_type.and_then(|t| LwtunnelIp::attr_from_type(t)),
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
                LwtunnelIp::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                LwtunnelIp::Dst(val) => {
                    if last_off == offset {
                        stack.push(("Dst", last_off));
                        break;
                    }
                }
                LwtunnelIp::Src(val) => {
                    if last_off == offset {
                        stack.push(("Src", last_off));
                        break;
                    }
                }
                LwtunnelIp::Ttl(val) => {
                    if last_off == offset {
                        stack.push(("Ttl", last_off));
                        break;
                    }
                }
                LwtunnelIp::Tos(val) => {
                    if last_off == offset {
                        stack.push(("Tos", last_off));
                        break;
                    }
                }
                LwtunnelIp::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                LwtunnelIp::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                LwtunnelIp::Opts(val) => {
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
            stack.push(("LwtunnelIp", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum IlaAttrs<'a> {
    Locator(u64),
    Identifier(u64),
    LocatorMatch(u64),
    Ifindex(i32),
    Dir(u32),
    Pad(&'a [u8]),
    CsumMode(u8),
    IdentType(u8),
    HookType(u8),
}
impl<'a> IterableIlaAttrs<'a> {
    pub fn get_locator(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(IlaAttrs::Locator(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IlaAttrs",
            "Locator",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_identifier(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(IlaAttrs::Identifier(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IlaAttrs",
            "Identifier",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_locator_match(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(IlaAttrs::LocatorMatch(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IlaAttrs",
            "LocatorMatch",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ifindex(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(IlaAttrs::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IlaAttrs",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dir(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(IlaAttrs::Dir(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IlaAttrs",
            "Dir",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(IlaAttrs::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IlaAttrs",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_csum_mode(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(IlaAttrs::CsumMode(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IlaAttrs",
            "CsumMode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ident_type(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(IlaAttrs::IdentType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IlaAttrs",
            "IdentType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_hook_type(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(IlaAttrs::HookType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IlaAttrs",
            "HookType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl IlaAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableIlaAttrs<'a> {
        IterableIlaAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Locator",
            2u16 => "Identifier",
            3u16 => "LocatorMatch",
            4u16 => "Ifindex",
            5u16 => "Dir",
            6u16 => "Pad",
            7u16 => "CsumMode",
            8u16 => "IdentType",
            9u16 => "HookType",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableIlaAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableIlaAttrs<'a> {
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
impl<'a> Iterator for IterableIlaAttrs<'a> {
    type Item = Result<IlaAttrs<'a>, ErrorContext>;
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
                1u16 => IlaAttrs::Locator({
                    let res = parse_be_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => IlaAttrs::Identifier({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => IlaAttrs::LocatorMatch({
                    let res = parse_be_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => IlaAttrs::Ifindex({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => IlaAttrs::Dir({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => IlaAttrs::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => IlaAttrs::CsumMode({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => IlaAttrs::IdentType({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => IlaAttrs::HookType({
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
            "IlaAttrs",
            r#type.and_then(|t| IlaAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableIlaAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("IlaAttrs");
        let mut iter = IterableIlaAttrs::with_loc(&[], self.orig_loc);
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
                IlaAttrs::Locator(val) => fmt.field("Locator", &val),
                IlaAttrs::Identifier(val) => fmt.field("Identifier", &val),
                IlaAttrs::LocatorMatch(val) => fmt.field("LocatorMatch", &val),
                IlaAttrs::Ifindex(val) => fmt.field("Ifindex", &val),
                IlaAttrs::Dir(val) => fmt.field("Dir", &val),
                IlaAttrs::Pad(val) => fmt.field("Pad", &val),
                IlaAttrs::CsumMode(val) => fmt.field("CsumMode", &val),
                IlaAttrs::IdentType(val) => fmt.field("IdentType", &val),
                IlaAttrs::HookType(val) => fmt.field("HookType", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableIlaAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("IlaAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| IlaAttrs::attr_from_type(t)),
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
                IlaAttrs::Locator(val) => {
                    if last_off == offset {
                        stack.push(("Locator", last_off));
                        break;
                    }
                }
                IlaAttrs::Identifier(val) => {
                    if last_off == offset {
                        stack.push(("Identifier", last_off));
                        break;
                    }
                }
                IlaAttrs::LocatorMatch(val) => {
                    if last_off == offset {
                        stack.push(("LocatorMatch", last_off));
                        break;
                    }
                }
                IlaAttrs::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                IlaAttrs::Dir(val) => {
                    if last_off == offset {
                        stack.push(("Dir", last_off));
                        break;
                    }
                }
                IlaAttrs::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                IlaAttrs::CsumMode(val) => {
                    if last_off == offset {
                        stack.push(("CsumMode", last_off));
                        break;
                    }
                }
                IlaAttrs::IdentType(val) => {
                    if last_off == offset {
                        stack.push(("IdentType", last_off));
                        break;
                    }
                }
                IlaAttrs::HookType(val) => {
                    if last_off == offset {
                        stack.push(("HookType", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("IlaAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LwtunnelIp6<'a> {
    Id(u64),
    Dst(std::net::Ipv6Addr),
    Src(std::net::Ipv6Addr),
    Hoplimit(u8),
    Tc(u8),
    Flags(u16),
    Pad(&'a [u8]),
    Opts(IterableLwtunnelIpOpts<'a>),
}
impl<'a> IterableLwtunnelIp6<'a> {
    pub fn get_id(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtunnelIp6::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtunnelIp6",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dst(&self) -> Result<std::net::Ipv6Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtunnelIp6::Dst(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtunnelIp6",
            "Dst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_src(&self) -> Result<std::net::Ipv6Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtunnelIp6::Src(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtunnelIp6",
            "Src",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_hoplimit(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtunnelIp6::Hoplimit(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtunnelIp6",
            "Hoplimit",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tc(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtunnelIp6::Tc(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtunnelIp6",
            "Tc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flags(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtunnelIp6::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtunnelIp6",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtunnelIp6::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtunnelIp6",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_opts(&self) -> Result<IterableLwtunnelIpOpts<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtunnelIp6::Opts(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtunnelIp6",
            "Opts",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LwtunnelIp6<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLwtunnelIp6<'a> {
        IterableLwtunnelIp6::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Id",
            2u16 => "Dst",
            3u16 => "Src",
            4u16 => "Hoplimit",
            5u16 => "Tc",
            6u16 => "Flags",
            7u16 => "Pad",
            8u16 => "Opts",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLwtunnelIp6<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLwtunnelIp6<'a> {
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
impl<'a> Iterator for IterableLwtunnelIp6<'a> {
    type Item = Result<LwtunnelIp6<'a>, ErrorContext>;
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
                1u16 => LwtunnelIp6::Id({
                    let res = parse_be_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => LwtunnelIp6::Dst({
                    let res = parse_u128(next).map(Ipv6Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => LwtunnelIp6::Src({
                    let res = parse_u128(next).map(Ipv6Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => LwtunnelIp6::Hoplimit({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => LwtunnelIp6::Tc({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => LwtunnelIp6::Flags({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => LwtunnelIp6::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => LwtunnelIp6::Opts({
                    let res = Some(IterableLwtunnelIpOpts::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "LwtunnelIp6",
            r#type.and_then(|t| LwtunnelIp6::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableLwtunnelIp6<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LwtunnelIp6");
        let mut iter = IterableLwtunnelIp6::with_loc(&[], self.orig_loc);
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
                LwtunnelIp6::Id(val) => fmt.field("Id", &val),
                LwtunnelIp6::Dst(val) => fmt.field("Dst", &val),
                LwtunnelIp6::Src(val) => fmt.field("Src", &val),
                LwtunnelIp6::Hoplimit(val) => fmt.field("Hoplimit", &val),
                LwtunnelIp6::Tc(val) => fmt.field("Tc", &val),
                LwtunnelIp6::Flags(val) => fmt.field("Flags", &val),
                LwtunnelIp6::Pad(val) => fmt.field("Pad", &val),
                LwtunnelIp6::Opts(val) => fmt.field("Opts", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableLwtunnelIp6<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LwtunnelIp6", offset));
            return (
                stack,
                missing_type.and_then(|t| LwtunnelIp6::attr_from_type(t)),
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
                LwtunnelIp6::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                LwtunnelIp6::Dst(val) => {
                    if last_off == offset {
                        stack.push(("Dst", last_off));
                        break;
                    }
                }
                LwtunnelIp6::Src(val) => {
                    if last_off == offset {
                        stack.push(("Src", last_off));
                        break;
                    }
                }
                LwtunnelIp6::Hoplimit(val) => {
                    if last_off == offset {
                        stack.push(("Hoplimit", last_off));
                        break;
                    }
                }
                LwtunnelIp6::Tc(val) => {
                    if last_off == offset {
                        stack.push(("Tc", last_off));
                        break;
                    }
                }
                LwtunnelIp6::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                LwtunnelIp6::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                LwtunnelIp6::Opts(val) => {
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
            stack.push(("LwtunnelIp6", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum Seg6Iptunnel<'a> {
    Srh(&'a [u8]),
    Src(std::net::Ipv6Addr),
    Table(u32),
}
impl<'a> IterableSeg6Iptunnel<'a> {
    pub fn get_srh(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Seg6Iptunnel::Srh(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Seg6Iptunnel",
            "Srh",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_src(&self) -> Result<std::net::Ipv6Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Seg6Iptunnel::Src(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Seg6Iptunnel",
            "Src",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_table(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Seg6Iptunnel::Table(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Seg6Iptunnel",
            "Table",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Seg6Iptunnel<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableSeg6Iptunnel<'a> {
        IterableSeg6Iptunnel::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Srh",
            2u16 => "Src",
            3u16 => "Table",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableSeg6Iptunnel<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSeg6Iptunnel<'a> {
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
impl<'a> Iterator for IterableSeg6Iptunnel<'a> {
    type Item = Result<Seg6Iptunnel<'a>, ErrorContext>;
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
                1u16 => Seg6Iptunnel::Srh({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Seg6Iptunnel::Src({
                    let res = parse_u128(next).map(Ipv6Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Seg6Iptunnel::Table({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Seg6Iptunnel",
            r#type.and_then(|t| Seg6Iptunnel::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableSeg6Iptunnel<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Seg6Iptunnel");
        let mut iter = IterableSeg6Iptunnel::with_loc(&[], self.orig_loc);
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
                Seg6Iptunnel::Srh(val) => fmt.field("Srh", &FormatHexdump(val)),
                Seg6Iptunnel::Src(val) => fmt.field("Src", &val),
                Seg6Iptunnel::Table(val) => fmt.field("Table", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSeg6Iptunnel<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Seg6Iptunnel", offset));
            return (
                stack,
                missing_type.and_then(|t| Seg6Iptunnel::attr_from_type(t)),
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
                Seg6Iptunnel::Srh(val) => {
                    if last_off == offset {
                        stack.push(("Srh", last_off));
                        break;
                    }
                }
                Seg6Iptunnel::Src(val) => {
                    if last_off == offset {
                        stack.push(("Src", last_off));
                        break;
                    }
                }
                Seg6Iptunnel::Table(val) => {
                    if last_off == offset {
                        stack.push(("Table", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Seg6Iptunnel", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LwtBpf<'a> {
    In(IterableLwtBpfProg<'a>),
    Out(IterableLwtBpfProg<'a>),
    Xmit(IterableLwtBpfProg<'a>),
    XmitHeadroom(u32),
}
impl<'a> IterableLwtBpf<'a> {
    pub fn get_in(&self) -> Result<IterableLwtBpfProg<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtBpf::In(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtBpf",
            "In",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_out(&self) -> Result<IterableLwtBpfProg<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtBpf::Out(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtBpf",
            "Out",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_xmit(&self) -> Result<IterableLwtBpfProg<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtBpf::Xmit(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtBpf",
            "Xmit",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_xmit_headroom(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtBpf::XmitHeadroom(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtBpf",
            "XmitHeadroom",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LwtBpf<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLwtBpf<'a> {
        IterableLwtBpf::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "In",
            2u16 => "Out",
            3u16 => "Xmit",
            4u16 => "XmitHeadroom",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLwtBpf<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLwtBpf<'a> {
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
impl<'a> Iterator for IterableLwtBpf<'a> {
    type Item = Result<LwtBpf<'a>, ErrorContext>;
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
                1u16 => LwtBpf::In({
                    let res = Some(IterableLwtBpfProg::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => LwtBpf::Out({
                    let res = Some(IterableLwtBpfProg::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => LwtBpf::Xmit({
                    let res = Some(IterableLwtBpfProg::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => LwtBpf::XmitHeadroom({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "LwtBpf",
            r#type.and_then(|t| LwtBpf::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableLwtBpf<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LwtBpf");
        let mut iter = IterableLwtBpf::with_loc(&[], self.orig_loc);
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
                LwtBpf::In(val) => fmt.field("In", &val),
                LwtBpf::Out(val) => fmt.field("Out", &val),
                LwtBpf::Xmit(val) => fmt.field("Xmit", &val),
                LwtBpf::XmitHeadroom(val) => fmt.field("XmitHeadroom", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableLwtBpf<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LwtBpf", offset));
            return (stack, missing_type.and_then(|t| LwtBpf::attr_from_type(t)));
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
                LwtBpf::In(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                LwtBpf::Out(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                LwtBpf::Xmit(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                LwtBpf::XmitHeadroom(val) => {
                    if last_off == offset {
                        stack.push(("XmitHeadroom", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LwtBpf", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum Seg6Local<'a> {
    #[doc = "Associated type: [`Seg6LocalActions`] (enum)"]
    Action(u32),
    Srh(&'a [u8]),
    Table(u32),
    Nh4(std::net::Ipv4Addr),
    Nh6(std::net::Ipv6Addr),
    Iif(u32),
    Oif(u32),
    Bpf(IterableSeg6LocalBpf<'a>),
    Vrftable(u32),
    Counters(IterableSeg6LocalCnt<'a>),
    Flavors(IterableSeg6LocalFlv<'a>),
}
impl<'a> IterableSeg6Local<'a> {
    #[doc = "Associated type: [`Seg6LocalActions`] (enum)"]
    pub fn get_action(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Seg6Local::Action(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Seg6Local",
            "Action",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_srh(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Seg6Local::Srh(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Seg6Local",
            "Srh",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_table(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Seg6Local::Table(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Seg6Local",
            "Table",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nh4(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Seg6Local::Nh4(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Seg6Local",
            "Nh4",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nh6(&self) -> Result<std::net::Ipv6Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Seg6Local::Nh6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Seg6Local",
            "Nh6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_iif(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Seg6Local::Iif(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Seg6Local",
            "Iif",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_oif(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Seg6Local::Oif(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Seg6Local",
            "Oif",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_bpf(&self) -> Result<IterableSeg6LocalBpf<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Seg6Local::Bpf(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Seg6Local",
            "Bpf",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vrftable(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Seg6Local::Vrftable(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Seg6Local",
            "Vrftable",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_counters(&self) -> Result<IterableSeg6LocalCnt<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Seg6Local::Counters(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Seg6Local",
            "Counters",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flavors(&self) -> Result<IterableSeg6LocalFlv<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Seg6Local::Flavors(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Seg6Local",
            "Flavors",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Seg6Local<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableSeg6Local<'a> {
        IterableSeg6Local::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Action",
            2u16 => "Srh",
            3u16 => "Table",
            4u16 => "Nh4",
            5u16 => "Nh6",
            6u16 => "Iif",
            7u16 => "Oif",
            8u16 => "Bpf",
            9u16 => "Vrftable",
            10u16 => "Counters",
            11u16 => "Flavors",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableSeg6Local<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSeg6Local<'a> {
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
impl<'a> Iterator for IterableSeg6Local<'a> {
    type Item = Result<Seg6Local<'a>, ErrorContext>;
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
                1u16 => Seg6Local::Action({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Seg6Local::Srh({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Seg6Local::Table({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Seg6Local::Nh4({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Seg6Local::Nh6({
                    let res = parse_u128(next).map(Ipv6Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Seg6Local::Iif({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => Seg6Local::Oif({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => Seg6Local::Bpf({
                    let res = Some(IterableSeg6LocalBpf::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => Seg6Local::Vrftable({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => Seg6Local::Counters({
                    let res = Some(IterableSeg6LocalCnt::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => Seg6Local::Flavors({
                    let res = Some(IterableSeg6LocalFlv::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Seg6Local",
            r#type.and_then(|t| Seg6Local::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableSeg6Local<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Seg6Local");
        let mut iter = IterableSeg6Local::with_loc(&[], self.orig_loc);
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
                Seg6Local::Action(val) => fmt.field(
                    "Action",
                    &FormatEnum(val.into(), Seg6LocalActions::from_value),
                ),
                Seg6Local::Srh(val) => fmt.field("Srh", &FormatHexdump(val)),
                Seg6Local::Table(val) => fmt.field("Table", &val),
                Seg6Local::Nh4(val) => fmt.field("Nh4", &val),
                Seg6Local::Nh6(val) => fmt.field("Nh6", &val),
                Seg6Local::Iif(val) => fmt.field("Iif", &val),
                Seg6Local::Oif(val) => fmt.field("Oif", &val),
                Seg6Local::Bpf(val) => fmt.field("Bpf", &val),
                Seg6Local::Vrftable(val) => fmt.field("Vrftable", &val),
                Seg6Local::Counters(val) => fmt.field("Counters", &val),
                Seg6Local::Flavors(val) => fmt.field("Flavors", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSeg6Local<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Seg6Local", offset));
            return (
                stack,
                missing_type.and_then(|t| Seg6Local::attr_from_type(t)),
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
                Seg6Local::Action(val) => {
                    if last_off == offset {
                        stack.push(("Action", last_off));
                        break;
                    }
                }
                Seg6Local::Srh(val) => {
                    if last_off == offset {
                        stack.push(("Srh", last_off));
                        break;
                    }
                }
                Seg6Local::Table(val) => {
                    if last_off == offset {
                        stack.push(("Table", last_off));
                        break;
                    }
                }
                Seg6Local::Nh4(val) => {
                    if last_off == offset {
                        stack.push(("Nh4", last_off));
                        break;
                    }
                }
                Seg6Local::Nh6(val) => {
                    if last_off == offset {
                        stack.push(("Nh6", last_off));
                        break;
                    }
                }
                Seg6Local::Iif(val) => {
                    if last_off == offset {
                        stack.push(("Iif", last_off));
                        break;
                    }
                }
                Seg6Local::Oif(val) => {
                    if last_off == offset {
                        stack.push(("Oif", last_off));
                        break;
                    }
                }
                Seg6Local::Bpf(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Seg6Local::Vrftable(val) => {
                    if last_off == offset {
                        stack.push(("Vrftable", last_off));
                        break;
                    }
                }
                Seg6Local::Counters(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Seg6Local::Flavors(val) => {
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
            stack.push(("Seg6Local", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum RplIptunnel<'a> {
    Srh(&'a [u8]),
}
impl<'a> IterableRplIptunnel<'a> {
    pub fn get_srh(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RplIptunnel::Srh(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RplIptunnel",
            "Srh",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl RplIptunnel<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableRplIptunnel<'a> {
        IterableRplIptunnel::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Srh",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableRplIptunnel<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableRplIptunnel<'a> {
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
impl<'a> Iterator for IterableRplIptunnel<'a> {
    type Item = Result<RplIptunnel<'a>, ErrorContext>;
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
                1u16 => RplIptunnel::Srh({
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
            "RplIptunnel",
            r#type.and_then(|t| RplIptunnel::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableRplIptunnel<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("RplIptunnel");
        let mut iter = IterableRplIptunnel::with_loc(&[], self.orig_loc);
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
                RplIptunnel::Srh(val) => fmt.field("Srh", &FormatHexdump(val)),
            };
        }
        fmt.finish()
    }
}
impl IterableRplIptunnel<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("RplIptunnel", offset));
            return (
                stack,
                missing_type.and_then(|t| RplIptunnel::attr_from_type(t)),
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
                RplIptunnel::Srh(val) => {
                    if last_off == offset {
                        stack.push(("Srh", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("RplIptunnel", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum Ioam6Iptunnel<'a> {
    Mode(u8),
    Dst(std::net::Ipv6Addr),
    Trace(&'a [u8]),
    FreqK(u32),
    FreqN(u32),
    Src(std::net::Ipv6Addr),
}
impl<'a> IterableIoam6Iptunnel<'a> {
    pub fn get_mode(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Ioam6Iptunnel::Mode(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Ioam6Iptunnel",
            "Mode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dst(&self) -> Result<std::net::Ipv6Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Ioam6Iptunnel::Dst(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Ioam6Iptunnel",
            "Dst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_trace(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Ioam6Iptunnel::Trace(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Ioam6Iptunnel",
            "Trace",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_freq_k(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Ioam6Iptunnel::FreqK(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Ioam6Iptunnel",
            "FreqK",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_freq_n(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Ioam6Iptunnel::FreqN(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Ioam6Iptunnel",
            "FreqN",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_src(&self) -> Result<std::net::Ipv6Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Ioam6Iptunnel::Src(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Ioam6Iptunnel",
            "Src",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Ioam6Iptunnel<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableIoam6Iptunnel<'a> {
        IterableIoam6Iptunnel::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Mode",
            2u16 => "Dst",
            3u16 => "Trace",
            4u16 => "FreqK",
            5u16 => "FreqN",
            6u16 => "Src",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableIoam6Iptunnel<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableIoam6Iptunnel<'a> {
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
impl<'a> Iterator for IterableIoam6Iptunnel<'a> {
    type Item = Result<Ioam6Iptunnel<'a>, ErrorContext>;
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
                1u16 => Ioam6Iptunnel::Mode({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Ioam6Iptunnel::Dst({
                    let res = parse_u128(next).map(Ipv6Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Ioam6Iptunnel::Trace({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Ioam6Iptunnel::FreqK({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Ioam6Iptunnel::FreqN({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Ioam6Iptunnel::Src({
                    let res = parse_u128(next).map(Ipv6Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Ioam6Iptunnel",
            r#type.and_then(|t| Ioam6Iptunnel::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableIoam6Iptunnel<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Ioam6Iptunnel");
        let mut iter = IterableIoam6Iptunnel::with_loc(&[], self.orig_loc);
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
                Ioam6Iptunnel::Mode(val) => fmt.field("Mode", &val),
                Ioam6Iptunnel::Dst(val) => fmt.field("Dst", &val),
                Ioam6Iptunnel::Trace(val) => fmt.field("Trace", &FormatHexdump(val)),
                Ioam6Iptunnel::FreqK(val) => fmt.field("FreqK", &val),
                Ioam6Iptunnel::FreqN(val) => fmt.field("FreqN", &val),
                Ioam6Iptunnel::Src(val) => fmt.field("Src", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableIoam6Iptunnel<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Ioam6Iptunnel", offset));
            return (
                stack,
                missing_type.and_then(|t| Ioam6Iptunnel::attr_from_type(t)),
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
                Ioam6Iptunnel::Mode(val) => {
                    if last_off == offset {
                        stack.push(("Mode", last_off));
                        break;
                    }
                }
                Ioam6Iptunnel::Dst(val) => {
                    if last_off == offset {
                        stack.push(("Dst", last_off));
                        break;
                    }
                }
                Ioam6Iptunnel::Trace(val) => {
                    if last_off == offset {
                        stack.push(("Trace", last_off));
                        break;
                    }
                }
                Ioam6Iptunnel::FreqK(val) => {
                    if last_off == offset {
                        stack.push(("FreqK", last_off));
                        break;
                    }
                }
                Ioam6Iptunnel::FreqN(val) => {
                    if last_off == offset {
                        stack.push(("FreqN", last_off));
                        break;
                    }
                }
                Ioam6Iptunnel::Src(val) => {
                    if last_off == offset {
                        stack.push(("Src", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Ioam6Iptunnel", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LwtXfrm {
    IfId(u32),
    Link(u32),
}
impl<'a> IterableLwtXfrm<'a> {
    pub fn get_if_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtXfrm::IfId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtXfrm",
            "IfId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_link(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtXfrm::Link(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtXfrm",
            "Link",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LwtXfrm {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLwtXfrm<'a> {
        IterableLwtXfrm::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "IfId",
            2u16 => "Link",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLwtXfrm<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLwtXfrm<'a> {
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
impl<'a> Iterator for IterableLwtXfrm<'a> {
    type Item = Result<LwtXfrm, ErrorContext>;
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
                1u16 => LwtXfrm::IfId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => LwtXfrm::Link({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "LwtXfrm",
            r#type.and_then(|t| LwtXfrm::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableLwtXfrm<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LwtXfrm");
        let mut iter = IterableLwtXfrm::with_loc(&[], self.orig_loc);
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
                LwtXfrm::IfId(val) => fmt.field("IfId", &val),
                LwtXfrm::Link(val) => fmt.field("Link", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableLwtXfrm<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LwtXfrm", offset));
            return (stack, missing_type.and_then(|t| LwtXfrm::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                LwtXfrm::IfId(val) => {
                    if last_off == offset {
                        stack.push(("IfId", last_off));
                        break;
                    }
                }
                LwtXfrm::Link(val) => {
                    if last_off == offset {
                        stack.push(("Link", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LwtXfrm", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LwtunnelIpOpts<'a> {
    Geneve(&'a [u8]),
    Vxlan(IterableLwtunnelIpOptVxlan<'a>),
    Erspan(IterableLwtunnelIpOptErspan<'a>),
}
impl<'a> IterableLwtunnelIpOpts<'a> {
    pub fn get_geneve(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtunnelIpOpts::Geneve(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtunnelIpOpts",
            "Geneve",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vxlan(&self) -> Result<IterableLwtunnelIpOptVxlan<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtunnelIpOpts::Vxlan(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtunnelIpOpts",
            "Vxlan",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_erspan(&self) -> Result<IterableLwtunnelIpOptErspan<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtunnelIpOpts::Erspan(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtunnelIpOpts",
            "Erspan",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LwtunnelIpOpts<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLwtunnelIpOpts<'a> {
        IterableLwtunnelIpOpts::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Geneve",
            2u16 => "Vxlan",
            3u16 => "Erspan",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLwtunnelIpOpts<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLwtunnelIpOpts<'a> {
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
impl<'a> Iterator for IterableLwtunnelIpOpts<'a> {
    type Item = Result<LwtunnelIpOpts<'a>, ErrorContext>;
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
                1u16 => LwtunnelIpOpts::Geneve({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => LwtunnelIpOpts::Vxlan({
                    let res = Some(IterableLwtunnelIpOptVxlan::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => LwtunnelIpOpts::Erspan({
                    let res = Some(IterableLwtunnelIpOptErspan::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "LwtunnelIpOpts",
            r#type.and_then(|t| LwtunnelIpOpts::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableLwtunnelIpOpts<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LwtunnelIpOpts");
        let mut iter = IterableLwtunnelIpOpts::with_loc(&[], self.orig_loc);
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
                LwtunnelIpOpts::Geneve(val) => fmt.field("Geneve", &FormatHexdump(val)),
                LwtunnelIpOpts::Vxlan(val) => fmt.field("Vxlan", &val),
                LwtunnelIpOpts::Erspan(val) => fmt.field("Erspan", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableLwtunnelIpOpts<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LwtunnelIpOpts", offset));
            return (
                stack,
                missing_type.and_then(|t| LwtunnelIpOpts::attr_from_type(t)),
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
                LwtunnelIpOpts::Geneve(val) => {
                    if last_off == offset {
                        stack.push(("Geneve", last_off));
                        break;
                    }
                }
                LwtunnelIpOpts::Vxlan(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                LwtunnelIpOpts::Erspan(val) => {
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
            stack.push(("LwtunnelIpOpts", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum LwtunnelIpOptVxlan {
    Gbp(u32),
}
impl<'a> IterableLwtunnelIpOptVxlan<'a> {
    pub fn get_gbp(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtunnelIpOptVxlan::Gbp(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtunnelIpOptVxlan",
            "Gbp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LwtunnelIpOptVxlan {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLwtunnelIpOptVxlan<'a> {
        IterableLwtunnelIpOptVxlan::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Gbp",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLwtunnelIpOptVxlan<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLwtunnelIpOptVxlan<'a> {
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
impl<'a> Iterator for IterableLwtunnelIpOptVxlan<'a> {
    type Item = Result<LwtunnelIpOptVxlan, ErrorContext>;
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
                1u16 => LwtunnelIpOptVxlan::Gbp({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "LwtunnelIpOptVxlan",
            r#type.and_then(|t| LwtunnelIpOptVxlan::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableLwtunnelIpOptVxlan<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LwtunnelIpOptVxlan");
        let mut iter = IterableLwtunnelIpOptVxlan::with_loc(&[], self.orig_loc);
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
                LwtunnelIpOptVxlan::Gbp(val) => fmt.field("Gbp", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableLwtunnelIpOptVxlan<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LwtunnelIpOptVxlan", offset));
            return (
                stack,
                missing_type.and_then(|t| LwtunnelIpOptVxlan::attr_from_type(t)),
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
                LwtunnelIpOptVxlan::Gbp(val) => {
                    if last_off == offset {
                        stack.push(("Gbp", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LwtunnelIpOptVxlan", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LwtunnelIpOptErspan {
    Ver(u8),
    Index(u32),
    Dir(u8),
    Hwid(u8),
}
impl<'a> IterableLwtunnelIpOptErspan<'a> {
    pub fn get_ver(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtunnelIpOptErspan::Ver(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtunnelIpOptErspan",
            "Ver",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_index(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtunnelIpOptErspan::Index(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtunnelIpOptErspan",
            "Index",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dir(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtunnelIpOptErspan::Dir(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtunnelIpOptErspan",
            "Dir",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_hwid(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtunnelIpOptErspan::Hwid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtunnelIpOptErspan",
            "Hwid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LwtunnelIpOptErspan {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLwtunnelIpOptErspan<'a> {
        IterableLwtunnelIpOptErspan::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Ver",
            2u16 => "Index",
            3u16 => "Dir",
            4u16 => "Hwid",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLwtunnelIpOptErspan<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLwtunnelIpOptErspan<'a> {
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
impl<'a> Iterator for IterableLwtunnelIpOptErspan<'a> {
    type Item = Result<LwtunnelIpOptErspan, ErrorContext>;
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
                1u16 => LwtunnelIpOptErspan::Ver({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => LwtunnelIpOptErspan::Index({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => LwtunnelIpOptErspan::Dir({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => LwtunnelIpOptErspan::Hwid({
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
            "LwtunnelIpOptErspan",
            r#type.and_then(|t| LwtunnelIpOptErspan::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableLwtunnelIpOptErspan<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LwtunnelIpOptErspan");
        let mut iter = IterableLwtunnelIpOptErspan::with_loc(&[], self.orig_loc);
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
                LwtunnelIpOptErspan::Ver(val) => fmt.field("Ver", &val),
                LwtunnelIpOptErspan::Index(val) => fmt.field("Index", &val),
                LwtunnelIpOptErspan::Dir(val) => fmt.field("Dir", &val),
                LwtunnelIpOptErspan::Hwid(val) => fmt.field("Hwid", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableLwtunnelIpOptErspan<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LwtunnelIpOptErspan", offset));
            return (
                stack,
                missing_type.and_then(|t| LwtunnelIpOptErspan::attr_from_type(t)),
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
                LwtunnelIpOptErspan::Ver(val) => {
                    if last_off == offset {
                        stack.push(("Ver", last_off));
                        break;
                    }
                }
                LwtunnelIpOptErspan::Index(val) => {
                    if last_off == offset {
                        stack.push(("Index", last_off));
                        break;
                    }
                }
                LwtunnelIpOptErspan::Dir(val) => {
                    if last_off == offset {
                        stack.push(("Dir", last_off));
                        break;
                    }
                }
                LwtunnelIpOptErspan::Hwid(val) => {
                    if last_off == offset {
                        stack.push(("Hwid", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LwtunnelIpOptErspan", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LwtBpfProg<'a> {
    Fd(u32),
    Name(&'a CStr),
}
impl<'a> IterableLwtBpfProg<'a> {
    pub fn get_fd(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtBpfProg::Fd(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtBpfProg",
            "Fd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LwtBpfProg::Name(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LwtBpfProg",
            "Name",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LwtBpfProg<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLwtBpfProg<'a> {
        IterableLwtBpfProg::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Fd",
            2u16 => "Name",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLwtBpfProg<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLwtBpfProg<'a> {
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
impl<'a> Iterator for IterableLwtBpfProg<'a> {
    type Item = Result<LwtBpfProg<'a>, ErrorContext>;
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
                1u16 => LwtBpfProg::Fd({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => LwtBpfProg::Name({
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
            "LwtBpfProg",
            r#type.and_then(|t| LwtBpfProg::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableLwtBpfProg<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LwtBpfProg");
        let mut iter = IterableLwtBpfProg::with_loc(&[], self.orig_loc);
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
                LwtBpfProg::Fd(val) => fmt.field("Fd", &val),
                LwtBpfProg::Name(val) => fmt.field("Name", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableLwtBpfProg<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LwtBpfProg", offset));
            return (
                stack,
                missing_type.and_then(|t| LwtBpfProg::attr_from_type(t)),
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
                LwtBpfProg::Fd(val) => {
                    if last_off == offset {
                        stack.push(("Fd", last_off));
                        break;
                    }
                }
                LwtBpfProg::Name(val) => {
                    if last_off == offset {
                        stack.push(("Name", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LwtBpfProg", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum Seg6LocalBpf<'a> {
    Prog(u32),
    ProgName(&'a CStr),
}
impl<'a> IterableSeg6LocalBpf<'a> {
    pub fn get_prog(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Seg6LocalBpf::Prog(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Seg6LocalBpf",
            "Prog",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_prog_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Seg6LocalBpf::ProgName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Seg6LocalBpf",
            "ProgName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Seg6LocalBpf<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableSeg6LocalBpf<'a> {
        IterableSeg6LocalBpf::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Prog",
            2u16 => "ProgName",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableSeg6LocalBpf<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSeg6LocalBpf<'a> {
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
impl<'a> Iterator for IterableSeg6LocalBpf<'a> {
    type Item = Result<Seg6LocalBpf<'a>, ErrorContext>;
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
                1u16 => Seg6LocalBpf::Prog({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Seg6LocalBpf::ProgName({
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
            "Seg6LocalBpf",
            r#type.and_then(|t| Seg6LocalBpf::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableSeg6LocalBpf<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Seg6LocalBpf");
        let mut iter = IterableSeg6LocalBpf::with_loc(&[], self.orig_loc);
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
                Seg6LocalBpf::Prog(val) => fmt.field("Prog", &val),
                Seg6LocalBpf::ProgName(val) => fmt.field("ProgName", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSeg6LocalBpf<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Seg6LocalBpf", offset));
            return (
                stack,
                missing_type.and_then(|t| Seg6LocalBpf::attr_from_type(t)),
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
                Seg6LocalBpf::Prog(val) => {
                    if last_off == offset {
                        stack.push(("Prog", last_off));
                        break;
                    }
                }
                Seg6LocalBpf::ProgName(val) => {
                    if last_off == offset {
                        stack.push(("ProgName", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Seg6LocalBpf", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum Seg6LocalCnt<'a> {
    Pad(&'a [u8]),
    Packets(u64),
    Bytes(u64),
    Errors(u64),
}
impl<'a> IterableSeg6LocalCnt<'a> {
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Seg6LocalCnt::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Seg6LocalCnt",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_packets(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Seg6LocalCnt::Packets(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Seg6LocalCnt",
            "Packets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_bytes(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Seg6LocalCnt::Bytes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Seg6LocalCnt",
            "Bytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_errors(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Seg6LocalCnt::Errors(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Seg6LocalCnt",
            "Errors",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Seg6LocalCnt<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableSeg6LocalCnt<'a> {
        IterableSeg6LocalCnt::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Pad",
            2u16 => "Packets",
            3u16 => "Bytes",
            4u16 => "Errors",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableSeg6LocalCnt<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSeg6LocalCnt<'a> {
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
impl<'a> Iterator for IterableSeg6LocalCnt<'a> {
    type Item = Result<Seg6LocalCnt<'a>, ErrorContext>;
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
                1u16 => Seg6LocalCnt::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Seg6LocalCnt::Packets({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Seg6LocalCnt::Bytes({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Seg6LocalCnt::Errors({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Seg6LocalCnt",
            r#type.and_then(|t| Seg6LocalCnt::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableSeg6LocalCnt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Seg6LocalCnt");
        let mut iter = IterableSeg6LocalCnt::with_loc(&[], self.orig_loc);
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
                Seg6LocalCnt::Pad(val) => fmt.field("Pad", &val),
                Seg6LocalCnt::Packets(val) => fmt.field("Packets", &val),
                Seg6LocalCnt::Bytes(val) => fmt.field("Bytes", &val),
                Seg6LocalCnt::Errors(val) => fmt.field("Errors", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSeg6LocalCnt<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Seg6LocalCnt", offset));
            return (
                stack,
                missing_type.and_then(|t| Seg6LocalCnt::attr_from_type(t)),
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
                Seg6LocalCnt::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                Seg6LocalCnt::Packets(val) => {
                    if last_off == offset {
                        stack.push(("Packets", last_off));
                        break;
                    }
                }
                Seg6LocalCnt::Bytes(val) => {
                    if last_off == offset {
                        stack.push(("Bytes", last_off));
                        break;
                    }
                }
                Seg6LocalCnt::Errors(val) => {
                    if last_off == offset {
                        stack.push(("Errors", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Seg6LocalCnt", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum Seg6LocalFlv {
    #[doc = "Associated type: [`Seg6LocalFlvOps`] (1 bit per enumeration)"]
    Operation(u32),
    LcblockBits(u8),
    LcnodeFnBits(u8),
}
impl<'a> IterableSeg6LocalFlv<'a> {
    #[doc = "Associated type: [`Seg6LocalFlvOps`] (1 bit per enumeration)"]
    pub fn get_operation(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Seg6LocalFlv::Operation(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Seg6LocalFlv",
            "Operation",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_lcblock_bits(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Seg6LocalFlv::LcblockBits(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Seg6LocalFlv",
            "LcblockBits",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_lcnode_fn_bits(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Seg6LocalFlv::LcnodeFnBits(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Seg6LocalFlv",
            "LcnodeFnBits",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Seg6LocalFlv {
    pub fn new<'a>(buf: &'a [u8]) -> IterableSeg6LocalFlv<'a> {
        IterableSeg6LocalFlv::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Operation",
            2u16 => "LcblockBits",
            3u16 => "LcnodeFnBits",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableSeg6LocalFlv<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSeg6LocalFlv<'a> {
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
impl<'a> Iterator for IterableSeg6LocalFlv<'a> {
    type Item = Result<Seg6LocalFlv, ErrorContext>;
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
                1u16 => Seg6LocalFlv::Operation({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Seg6LocalFlv::LcblockBits({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Seg6LocalFlv::LcnodeFnBits({
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
            "Seg6LocalFlv",
            r#type.and_then(|t| Seg6LocalFlv::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableSeg6LocalFlv<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Seg6LocalFlv");
        let mut iter = IterableSeg6LocalFlv::with_loc(&[], self.orig_loc);
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
                Seg6LocalFlv::Operation(val) => fmt.field(
                    "Operation",
                    &FormatFlags(val.into(), |val| {
                        Seg6LocalFlvOps::from_value(val.trailing_zeros().into())
                    }),
                ),
                Seg6LocalFlv::LcblockBits(val) => fmt.field("LcblockBits", &val),
                Seg6LocalFlv::LcnodeFnBits(val) => fmt.field("LcnodeFnBits", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSeg6LocalFlv<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Seg6LocalFlv", offset));
            return (
                stack,
                missing_type.and_then(|t| Seg6LocalFlv::attr_from_type(t)),
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
                Seg6LocalFlv::Operation(val) => {
                    if last_off == offset {
                        stack.push(("Operation", last_off));
                        break;
                    }
                }
                Seg6LocalFlv::LcblockBits(val) => {
                    if last_off == offset {
                        stack.push(("LcblockBits", last_off));
                        break;
                    }
                }
                Seg6LocalFlv::LcnodeFnBits(val) => {
                    if last_off == offset {
                        stack.push(("LcnodeFnBits", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Seg6LocalFlv", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ResGroupAttrs<'a> {
    Pad(&'a [u8]),
    Buckets(u16),
    IdleTimer(u32),
    UnbalancedTimer(u32),
    UnbalancedTime(u64),
}
impl<'a> IterableResGroupAttrs<'a> {
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResGroupAttrs::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResGroupAttrs",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_buckets(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResGroupAttrs::Buckets(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResGroupAttrs",
            "Buckets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_idle_timer(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResGroupAttrs::IdleTimer(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResGroupAttrs",
            "IdleTimer",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_unbalanced_timer(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResGroupAttrs::UnbalancedTimer(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResGroupAttrs",
            "UnbalancedTimer",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_unbalanced_time(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResGroupAttrs::UnbalancedTime(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResGroupAttrs",
            "UnbalancedTime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ResGroupAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableResGroupAttrs<'a> {
        IterableResGroupAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "Pad",
            1u16 => "Buckets",
            2u16 => "IdleTimer",
            3u16 => "UnbalancedTimer",
            4u16 => "UnbalancedTime",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableResGroupAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableResGroupAttrs<'a> {
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
impl<'a> Iterator for IterableResGroupAttrs<'a> {
    type Item = Result<ResGroupAttrs<'a>, ErrorContext>;
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
                0u16 => ResGroupAttrs::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                1u16 => ResGroupAttrs::Buckets({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ResGroupAttrs::IdleTimer({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ResGroupAttrs::UnbalancedTimer({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => ResGroupAttrs::UnbalancedTime({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "ResGroupAttrs",
            r#type.and_then(|t| ResGroupAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableResGroupAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ResGroupAttrs");
        let mut iter = IterableResGroupAttrs::with_loc(&[], self.orig_loc);
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
                ResGroupAttrs::Pad(val) => fmt.field("Pad", &val),
                ResGroupAttrs::Buckets(val) => fmt.field("Buckets", &val),
                ResGroupAttrs::IdleTimer(val) => fmt.field("IdleTimer", &val),
                ResGroupAttrs::UnbalancedTimer(val) => fmt.field("UnbalancedTimer", &val),
                ResGroupAttrs::UnbalancedTime(val) => fmt.field("UnbalancedTime", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableResGroupAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ResGroupAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ResGroupAttrs::attr_from_type(t)),
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
                ResGroupAttrs::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                ResGroupAttrs::Buckets(val) => {
                    if last_off == offset {
                        stack.push(("Buckets", last_off));
                        break;
                    }
                }
                ResGroupAttrs::IdleTimer(val) => {
                    if last_off == offset {
                        stack.push(("IdleTimer", last_off));
                        break;
                    }
                }
                ResGroupAttrs::UnbalancedTimer(val) => {
                    if last_off == offset {
                        stack.push(("UnbalancedTimer", last_off));
                        break;
                    }
                }
                ResGroupAttrs::UnbalancedTime(val) => {
                    if last_off == offset {
                        stack.push(("UnbalancedTime", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ResGroupAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ResBucketAttrs<'a> {
    Pad(&'a [u8]),
    Index(u16),
    IdleTime(u64),
    NhId(u32),
}
impl<'a> IterableResBucketAttrs<'a> {
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResBucketAttrs::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResBucketAttrs",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_index(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResBucketAttrs::Index(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResBucketAttrs",
            "Index",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_idle_time(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResBucketAttrs::IdleTime(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResBucketAttrs",
            "IdleTime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nh_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ResBucketAttrs::NhId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ResBucketAttrs",
            "NhId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ResBucketAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableResBucketAttrs<'a> {
        IterableResBucketAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "Pad",
            1u16 => "Index",
            2u16 => "IdleTime",
            3u16 => "NhId",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableResBucketAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableResBucketAttrs<'a> {
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
impl<'a> Iterator for IterableResBucketAttrs<'a> {
    type Item = Result<ResBucketAttrs<'a>, ErrorContext>;
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
                0u16 => ResBucketAttrs::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                1u16 => ResBucketAttrs::Index({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ResBucketAttrs::IdleTime({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ResBucketAttrs::NhId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "ResBucketAttrs",
            r#type.and_then(|t| ResBucketAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableResBucketAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ResBucketAttrs");
        let mut iter = IterableResBucketAttrs::with_loc(&[], self.orig_loc);
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
                ResBucketAttrs::Pad(val) => fmt.field("Pad", &val),
                ResBucketAttrs::Index(val) => fmt.field("Index", &val),
                ResBucketAttrs::IdleTime(val) => fmt.field("IdleTime", &val),
                ResBucketAttrs::NhId(val) => fmt.field("NhId", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableResBucketAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ResBucketAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ResBucketAttrs::attr_from_type(t)),
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
                ResBucketAttrs::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                ResBucketAttrs::Index(val) => {
                    if last_off == offset {
                        stack.push(("Index", last_off));
                        break;
                    }
                }
                ResBucketAttrs::IdleTime(val) => {
                    if last_off == offset {
                        stack.push(("IdleTime", last_off));
                        break;
                    }
                }
                ResBucketAttrs::NhId(val) => {
                    if last_off == offset {
                        stack.push(("NhId", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ResBucketAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum GroupStatsAttrs<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    Entry(IterableGroupStatsEntryAttrs<'a>),
}
impl<'a> IterableGroupStatsAttrs<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_entry(
        &self,
    ) -> MultiAttrIterable<Self, GroupStatsAttrs<'a>, IterableGroupStatsEntryAttrs<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let GroupStatsAttrs::Entry(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl GroupStatsAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableGroupStatsAttrs<'a> {
        IterableGroupStatsAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Entry",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableGroupStatsAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableGroupStatsAttrs<'a> {
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
impl<'a> Iterator for IterableGroupStatsAttrs<'a> {
    type Item = Result<GroupStatsAttrs<'a>, ErrorContext>;
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
                1u16 => GroupStatsAttrs::Entry({
                    let res = Some(IterableGroupStatsEntryAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "GroupStatsAttrs",
            r#type.and_then(|t| GroupStatsAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableGroupStatsAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("GroupStatsAttrs");
        let mut iter = IterableGroupStatsAttrs::with_loc(&[], self.orig_loc);
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
                GroupStatsAttrs::Entry(val) => fmt.field("Entry", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableGroupStatsAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("GroupStatsAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| GroupStatsAttrs::attr_from_type(t)),
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
                GroupStatsAttrs::Entry(val) => {
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
            stack.push(("GroupStatsAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum GroupStatsEntryAttrs {
    Id(u32),
    Packets(u32),
    PacketsHw(u32),
}
impl<'a> IterableGroupStatsEntryAttrs<'a> {
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(GroupStatsEntryAttrs::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "GroupStatsEntryAttrs",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(GroupStatsEntryAttrs::Packets(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "GroupStatsEntryAttrs",
            "Packets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_packets_hw(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(GroupStatsEntryAttrs::PacketsHw(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "GroupStatsEntryAttrs",
            "PacketsHw",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl GroupStatsEntryAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableGroupStatsEntryAttrs<'a> {
        IterableGroupStatsEntryAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Id",
            2u16 => "Packets",
            3u16 => "PacketsHw",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableGroupStatsEntryAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableGroupStatsEntryAttrs<'a> {
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
impl<'a> Iterator for IterableGroupStatsEntryAttrs<'a> {
    type Item = Result<GroupStatsEntryAttrs, ErrorContext>;
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
                1u16 => GroupStatsEntryAttrs::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => GroupStatsEntryAttrs::Packets({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => GroupStatsEntryAttrs::PacketsHw({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "GroupStatsEntryAttrs",
            r#type.and_then(|t| GroupStatsEntryAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableGroupStatsEntryAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("GroupStatsEntryAttrs");
        let mut iter = IterableGroupStatsEntryAttrs::with_loc(&[], self.orig_loc);
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
                GroupStatsEntryAttrs::Id(val) => fmt.field("Id", &val),
                GroupStatsEntryAttrs::Packets(val) => fmt.field("Packets", &val),
                GroupStatsEntryAttrs::PacketsHw(val) => fmt.field("PacketsHw", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableGroupStatsEntryAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("GroupStatsEntryAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| GroupStatsEntryAttrs::attr_from_type(t)),
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
                GroupStatsEntryAttrs::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                GroupStatsEntryAttrs::Packets(val) => {
                    if last_off == offset {
                        stack.push(("Packets", last_off));
                        break;
                    }
                }
                GroupStatsEntryAttrs::PacketsHw(val) => {
                    if last_off == offset {
                        stack.push(("PacketsHw", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("GroupStatsEntryAttrs", cur));
        }
        (stack, None)
    }
}
pub struct PushNexthopAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushNexthopAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushNexthopAttrs<Prev> {
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
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_group(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "Associated type: [`NexthopGrpType`] (enum)"]
    pub fn push_group_type(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 3u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_blackhole(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 4u16, 0 as u16);
        self
    }
    pub fn push_oif(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_gateway(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_vec_mut(), 6u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_vec_mut(), value);
        self
    }
    #[doc = "Associated type: [`EncapType`] (enum)"]
    pub fn push_encap_type(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 7u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Selector attribute `encap-type` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_encap_mpls(mut self) -> PushMplsIptunnel<PushDummy<Prev>> {
        self = self.push_encap_type(EncapType::Mpls as u16);
        let new_header_offset = push_nested_header(self.as_vec_mut(), 8u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushMplsIptunnel {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `encap-type` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_encap_ip(mut self) -> PushLwtunnelIp<PushDummy<Prev>> {
        self = self.push_encap_type(EncapType::Ip as u16);
        let new_header_offset = push_nested_header(self.as_vec_mut(), 8u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushLwtunnelIp {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `encap-type` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_encap_ila(mut self) -> PushIlaAttrs<PushDummy<Prev>> {
        self = self.push_encap_type(EncapType::Ila as u16);
        let new_header_offset = push_nested_header(self.as_vec_mut(), 8u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushIlaAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `encap-type` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_encap_ip6(mut self) -> PushLwtunnelIp6<PushDummy<Prev>> {
        self = self.push_encap_type(EncapType::Ip6 as u16);
        let new_header_offset = push_nested_header(self.as_vec_mut(), 8u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushLwtunnelIp6 {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `encap-type` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_encap_seg6(mut self) -> PushSeg6Iptunnel<PushDummy<Prev>> {
        self = self.push_encap_type(EncapType::Seg6 as u16);
        let new_header_offset = push_nested_header(self.as_vec_mut(), 8u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushSeg6Iptunnel {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `encap-type` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_encap_bpf(mut self) -> PushLwtBpf<PushDummy<Prev>> {
        self = self.push_encap_type(EncapType::Bpf as u16);
        let new_header_offset = push_nested_header(self.as_vec_mut(), 8u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushLwtBpf {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `encap-type` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_encap_seg6_local(mut self) -> PushSeg6Local<PushDummy<Prev>> {
        self = self.push_encap_type(EncapType::Seg6Local as u16);
        let new_header_offset = push_nested_header(self.as_vec_mut(), 8u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushSeg6Local {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `encap-type` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_encap_rpl(mut self) -> PushRplIptunnel<PushDummy<Prev>> {
        self = self.push_encap_type(EncapType::Rpl as u16);
        let new_header_offset = push_nested_header(self.as_vec_mut(), 8u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushRplIptunnel {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `encap-type` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_encap_ioam6(mut self) -> PushIoam6Iptunnel<PushDummy<Prev>> {
        self = self.push_encap_type(EncapType::Ioam6 as u16);
        let new_header_offset = push_nested_header(self.as_vec_mut(), 8u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushIoam6Iptunnel {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `encap-type` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_encap_xfrm(mut self) -> PushLwtXfrm<PushDummy<Prev>> {
        self = self.push_encap_type(EncapType::Xfrm as u16);
        let new_header_offset = push_nested_header(self.as_vec_mut(), 8u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushLwtXfrm {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    pub fn push_groups(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 9u16, 0 as u16);
        self
    }
    pub fn push_master(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fdb(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 11u16, 0 as u16);
        self
    }
    pub fn nested_res_group(mut self) -> PushResGroupAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 12u16);
        PushResGroupAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_res_bucket(mut self) -> PushResBucketAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 13u16);
        PushResBucketAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_op_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 14u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_group_stats(mut self) -> PushGroupStatsAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 15u16);
        PushGroupStatsAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_hw_stats_enable(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 16u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_hw_stats_used(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 17u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dst_port(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 18u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushNexthopAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushMplsIptunnel<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushMplsIptunnel<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushMplsIptunnel<Prev> {
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
    pub fn push_dst(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_ttl(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 2u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushMplsIptunnel<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLwtunnelIp<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLwtunnelIp<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLwtunnelIp<Prev> {
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
    pub fn push_id(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 1u16, 8 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_dst(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    pub fn push_src(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    pub fn push_ttl(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 4u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tos(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 5u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flags(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 6u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 7u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn nested_opts(mut self) -> PushLwtunnelIpOpts<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 8u16);
        PushLwtunnelIpOpts {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushLwtunnelIp<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushIlaAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushIlaAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushIlaAttrs<Prev> {
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
    pub fn push_locator(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 1u16, 8 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_identifier(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 2u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_locator_match(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 3u16, 8 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_ifindex(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dir(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 6u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_csum_mode(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 7u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ident_type(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 8u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_hook_type(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 9u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushIlaAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLwtunnelIp6<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLwtunnelIp6<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLwtunnelIp6<Prev> {
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
    pub fn push_id(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 1u16, 8 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_dst(mut self, value: std::net::Ipv6Addr) -> Self {
        push_header(self.as_vec_mut(), 2u16, 16 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_ne_bytes());
        self
    }
    pub fn push_src(mut self, value: std::net::Ipv6Addr) -> Self {
        push_header(self.as_vec_mut(), 3u16, 16 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_ne_bytes());
        self
    }
    pub fn push_hoplimit(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 4u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tc(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 5u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flags(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 6u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 7u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn nested_opts(mut self) -> PushLwtunnelIpOpts<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 8u16);
        PushLwtunnelIpOpts {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushLwtunnelIp6<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSeg6Iptunnel<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushSeg6Iptunnel<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushSeg6Iptunnel<Prev> {
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
    pub fn push_srh(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_src(mut self, value: std::net::Ipv6Addr) -> Self {
        push_header(self.as_vec_mut(), 2u16, 16 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_ne_bytes());
        self
    }
    pub fn push_table(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushSeg6Iptunnel<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLwtBpf<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLwtBpf<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLwtBpf<Prev> {
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
    pub fn nested_in(mut self) -> PushLwtBpfProg<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushLwtBpfProg {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_out(mut self) -> PushLwtBpfProg<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushLwtBpfProg {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_xmit(mut self) -> PushLwtBpfProg<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 3u16);
        PushLwtBpfProg {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_xmit_headroom(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushLwtBpf<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSeg6Local<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushSeg6Local<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushSeg6Local<Prev> {
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
    #[doc = "Associated type: [`Seg6LocalActions`] (enum)"]
    pub fn push_action(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_srh(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_table(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_nh4(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    pub fn push_nh6(mut self, value: std::net::Ipv6Addr) -> Self {
        push_header(self.as_vec_mut(), 5u16, 16 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_ne_bytes());
        self
    }
    pub fn push_iif(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_oif(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_bpf(mut self) -> PushSeg6LocalBpf<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 8u16);
        PushSeg6LocalBpf {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_vrftable(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_counters(mut self) -> PushSeg6LocalCnt<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 10u16);
        PushSeg6LocalCnt {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_flavors(mut self) -> PushSeg6LocalFlv<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 11u16);
        PushSeg6LocalFlv {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushSeg6Local<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushRplIptunnel<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushRplIptunnel<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushRplIptunnel<Prev> {
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
    pub fn push_srh(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushRplIptunnel<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushIoam6Iptunnel<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushIoam6Iptunnel<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushIoam6Iptunnel<Prev> {
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
    pub fn push_mode(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 1u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dst(mut self, value: std::net::Ipv6Addr) -> Self {
        push_header(self.as_vec_mut(), 2u16, 16 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_ne_bytes());
        self
    }
    pub fn push_trace(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_freq_k(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_freq_n(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_src(mut self, value: std::net::Ipv6Addr) -> Self {
        push_header(self.as_vec_mut(), 6u16, 16 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushIoam6Iptunnel<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLwtXfrm<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLwtXfrm<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLwtXfrm<Prev> {
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
    pub fn push_if_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_link(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushLwtXfrm<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLwtunnelIpOpts<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLwtunnelIpOpts<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLwtunnelIpOpts<Prev> {
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
    pub fn push_geneve(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn nested_vxlan(mut self) -> PushLwtunnelIpOptVxlan<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushLwtunnelIpOptVxlan {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_erspan(mut self) -> PushLwtunnelIpOptErspan<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 3u16);
        PushLwtunnelIpOptErspan {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushLwtunnelIpOpts<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLwtunnelIpOptVxlan<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLwtunnelIpOptVxlan<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLwtunnelIpOptVxlan<Prev> {
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
    pub fn push_gbp(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushLwtunnelIpOptVxlan<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLwtunnelIpOptErspan<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLwtunnelIpOptErspan<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLwtunnelIpOptErspan<Prev> {
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
    pub fn push_ver(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 1u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_index(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_dir(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 3u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_hwid(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 4u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushLwtunnelIpOptErspan<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLwtBpfProg<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLwtBpfProg<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLwtBpfProg<Prev> {
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
    pub fn push_fd(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
}
impl<Prev: Pusher> Drop for PushLwtBpfProg<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSeg6LocalBpf<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushSeg6LocalBpf<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushSeg6LocalBpf<Prev> {
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
    pub fn push_prog(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_prog_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_prog_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
}
impl<Prev: Pusher> Drop for PushSeg6LocalBpf<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSeg6LocalCnt<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushSeg6LocalCnt<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushSeg6LocalCnt<Prev> {
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
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_packets(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 2u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_bytes(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 3u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_errors(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 4u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushSeg6LocalCnt<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSeg6LocalFlv<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushSeg6LocalFlv<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushSeg6LocalFlv<Prev> {
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
    #[doc = "Associated type: [`Seg6LocalFlvOps`] (1 bit per enumeration)"]
    pub fn push_operation(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_lcblock_bits(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 2u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_lcnode_fn_bits(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 3u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushSeg6LocalFlv<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushResGroupAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushResGroupAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushResGroupAttrs<Prev> {
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
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 0u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_buckets(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 1u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_idle_timer(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_unbalanced_timer(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_unbalanced_time(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 4u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushResGroupAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushResBucketAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushResBucketAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushResBucketAttrs<Prev> {
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
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 0u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_index(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 1u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_idle_time(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 2u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_nh_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushResBucketAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushGroupStatsAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushGroupStatsAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushGroupStatsAttrs<Prev> {
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
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_entry(mut self) -> PushGroupStatsEntryAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushGroupStatsEntryAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushGroupStatsAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushGroupStatsEntryAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushGroupStatsEntryAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushGroupStatsEntryAttrs<Prev> {
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
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_packets(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_packets_hw(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushGroupStatsEntryAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Add new nexthop entry\n\nRequest attributes:\n- [.push_id()](PushNexthopAttrs::push_id)\n- [.push_group()](PushNexthopAttrs::push_group)\n- [.push_group_type()](PushNexthopAttrs::push_group_type)\n- [.push_blackhole()](PushNexthopAttrs::push_blackhole)\n- [.push_oif()](PushNexthopAttrs::push_oif)\n- [.push_gateway()](PushNexthopAttrs::push_gateway)\n- [.push_encap_type()](PushNexthopAttrs::push_encap_type)\n- [.nested_encap_mpls()](PushNexthopAttrs::nested_encap_mpls)\n- [.nested_encap_ip()](PushNexthopAttrs::nested_encap_ip)\n- [.nested_encap_ila()](PushNexthopAttrs::nested_encap_ila)\n- [.nested_encap_ip6()](PushNexthopAttrs::nested_encap_ip6)\n- [.nested_encap_seg6()](PushNexthopAttrs::nested_encap_seg6)\n- [.nested_encap_bpf()](PushNexthopAttrs::nested_encap_bpf)\n- [.nested_encap_seg6_local()](PushNexthopAttrs::nested_encap_seg6_local)\n- [.nested_encap_rpl()](PushNexthopAttrs::nested_encap_rpl)\n- [.nested_encap_ioam6()](PushNexthopAttrs::nested_encap_ioam6)\n- [.nested_encap_xfrm()](PushNexthopAttrs::nested_encap_xfrm)\n- [.push_fdb()](PushNexthopAttrs::push_fdb)\n- [.nested_res_group()](PushNexthopAttrs::nested_res_group)\n- [.push_hw_stats_enable()](PushNexthopAttrs::push_hw_stats_enable)\n- [.push_dst_port()](PushNexthopAttrs::push_dst_port)\n\n"]
#[derive(Debug)]
pub struct OpNewnexthopDo<'r> {
    request: Request<'r>,
}
impl<'r> OpNewnexthopDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nhmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nhmsg,
    ) -> PushNexthopAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushNexthopAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushNexthopAttrs<&mut Vec<u8>> {
        PushNexthopAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushNexthopAttrs<RequestBuf<'r>> {
        PushNexthopAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nhmsg, IterableNexthopAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nhmsg::len()));
        (
            Nhmsg::new_from_slice(header).unwrap_or_default(),
            IterableNexthopAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Nhmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Nhmsg {
        let pos = self.request.pos;
        Nhmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Nhmsg {
        let pos = self.request.pos;
        Nhmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpNewnexthopDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 104u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nhmsg, IterableNexthopAttrs<'buf>);
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
#[doc = "Delete nexthop with given id.\n\nRequest attributes:\n- [.push_id()](PushNexthopAttrs::push_id)\n\n"]
#[derive(Debug)]
pub struct OpDelnexthopDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDelnexthopDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nhmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nhmsg,
    ) -> PushNexthopAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushNexthopAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushNexthopAttrs<&mut Vec<u8>> {
        PushNexthopAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushNexthopAttrs<RequestBuf<'r>> {
        PushNexthopAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nhmsg, IterableNexthopAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nhmsg::len()));
        (
            Nhmsg::new_from_slice(header).unwrap_or_default(),
            IterableNexthopAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Nhmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Nhmsg {
        let pos = self.request.pos;
        Nhmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Nhmsg {
        let pos = self.request.pos;
        Nhmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpDelnexthopDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 105u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nhmsg, IterableNexthopAttrs<'buf>);
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
#[doc = "Get or dump nexthop entries\n\nRequest attributes:\n- [.push_oif()](PushNexthopAttrs::push_oif)\n- [.push_groups()](PushNexthopAttrs::push_groups)\n- [.push_master()](PushNexthopAttrs::push_master)\n- [.push_fdb()](PushNexthopAttrs::push_fdb)\n- [.push_op_flags()](PushNexthopAttrs::push_op_flags)\n\nReply attributes:\n- [.get_id()](IterableNexthopAttrs::get_id)\n- [.get_group()](IterableNexthopAttrs::get_group)\n- [.get_group_type()](IterableNexthopAttrs::get_group_type)\n- [.get_blackhole()](IterableNexthopAttrs::get_blackhole)\n- [.get_oif()](IterableNexthopAttrs::get_oif)\n- [.get_gateway()](IterableNexthopAttrs::get_gateway)\n- [.get_encap_type()](IterableNexthopAttrs::get_encap_type)\n- [.get_encap()](IterableNexthopAttrs::get_encap)\n- [.get_fdb()](IterableNexthopAttrs::get_fdb)\n- [.get_res_group()](IterableNexthopAttrs::get_res_group)\n- [.get_op_flags()](IterableNexthopAttrs::get_op_flags)\n- [.get_group_stats()](IterableNexthopAttrs::get_group_stats)\n- [.get_hw_stats_enable()](IterableNexthopAttrs::get_hw_stats_enable)\n- [.get_hw_stats_used()](IterableNexthopAttrs::get_hw_stats_used)\n- [.get_dst_port()](IterableNexthopAttrs::get_dst_port)\n\n"]
#[derive(Debug)]
pub struct OpGetnexthopDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetnexthopDump<'r> {
    pub fn new(mut request: Request<'r>, header: &Nhmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nhmsg,
    ) -> PushNexthopAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushNexthopAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushNexthopAttrs<&mut Vec<u8>> {
        PushNexthopAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushNexthopAttrs<RequestBuf<'r>> {
        PushNexthopAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nhmsg, IterableNexthopAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nhmsg::len()));
        (
            Nhmsg::new_from_slice(header).unwrap_or_default(),
            IterableNexthopAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Nhmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Nhmsg {
        let pos = self.request.pos;
        Nhmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Nhmsg {
        let pos = self.request.pos;
        Nhmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetnexthopDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 106u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nhmsg, IterableNexthopAttrs<'buf>);
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
#[doc = "Get or dump nexthop entries\n\nRequest attributes:\n- [.push_id()](PushNexthopAttrs::push_id)\n- [.push_op_flags()](PushNexthopAttrs::push_op_flags)\n\nReply attributes:\n- [.get_id()](IterableNexthopAttrs::get_id)\n- [.get_group()](IterableNexthopAttrs::get_group)\n- [.get_group_type()](IterableNexthopAttrs::get_group_type)\n- [.get_blackhole()](IterableNexthopAttrs::get_blackhole)\n- [.get_oif()](IterableNexthopAttrs::get_oif)\n- [.get_gateway()](IterableNexthopAttrs::get_gateway)\n- [.get_encap_type()](IterableNexthopAttrs::get_encap_type)\n- [.get_encap()](IterableNexthopAttrs::get_encap)\n- [.get_fdb()](IterableNexthopAttrs::get_fdb)\n- [.get_res_group()](IterableNexthopAttrs::get_res_group)\n- [.get_op_flags()](IterableNexthopAttrs::get_op_flags)\n- [.get_group_stats()](IterableNexthopAttrs::get_group_stats)\n- [.get_hw_stats_enable()](IterableNexthopAttrs::get_hw_stats_enable)\n- [.get_hw_stats_used()](IterableNexthopAttrs::get_hw_stats_used)\n- [.get_dst_port()](IterableNexthopAttrs::get_dst_port)\n\n"]
#[derive(Debug)]
pub struct OpGetnexthopDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetnexthopDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nhmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nhmsg,
    ) -> PushNexthopAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushNexthopAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushNexthopAttrs<&mut Vec<u8>> {
        PushNexthopAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushNexthopAttrs<RequestBuf<'r>> {
        PushNexthopAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nhmsg, IterableNexthopAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nhmsg::len()));
        (
            Nhmsg::new_from_slice(header).unwrap_or_default(),
            IterableNexthopAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Nhmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Nhmsg {
        let pos = self.request.pos;
        Nhmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Nhmsg {
        let pos = self.request.pos;
        Nhmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetnexthopDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 106u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nhmsg, IterableNexthopAttrs<'buf>);
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
#[doc = "Get or dump nexthop buckets\n\nRequest attributes:\n- [.push_id()](PushNexthopAttrs::push_id)\n- [.push_oif()](PushNexthopAttrs::push_oif)\n- [.push_master()](PushNexthopAttrs::push_master)\n- [.nested_res_bucket()](PushNexthopAttrs::nested_res_bucket)\n\nReply attributes:\n- [.get_id()](IterableNexthopAttrs::get_id)\n- [.get_res_bucket()](IterableNexthopAttrs::get_res_bucket)\n\n"]
#[derive(Debug)]
pub struct OpGetnexthopbucketDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetnexthopbucketDump<'r> {
    pub fn new(mut request: Request<'r>, header: &Nhmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nhmsg,
    ) -> PushNexthopAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushNexthopAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushNexthopAttrs<&mut Vec<u8>> {
        PushNexthopAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushNexthopAttrs<RequestBuf<'r>> {
        PushNexthopAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nhmsg, IterableNexthopAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nhmsg::len()));
        (
            Nhmsg::new_from_slice(header).unwrap_or_default(),
            IterableNexthopAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Nhmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Nhmsg {
        let pos = self.request.pos;
        Nhmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Nhmsg {
        let pos = self.request.pos;
        Nhmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetnexthopbucketDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 118u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nhmsg, IterableNexthopAttrs<'buf>);
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
#[doc = "Get or dump nexthop buckets\n\nRequest attributes:\n- [.push_id()](PushNexthopAttrs::push_id)\n- [.nested_res_bucket()](PushNexthopAttrs::nested_res_bucket)\n\nReply attributes:\n- [.get_id()](IterableNexthopAttrs::get_id)\n- [.get_res_bucket()](IterableNexthopAttrs::get_res_bucket)\n\n"]
#[derive(Debug)]
pub struct OpGetnexthopbucketDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetnexthopbucketDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nhmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nhmsg,
    ) -> PushNexthopAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushNexthopAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushNexthopAttrs<&mut Vec<u8>> {
        PushNexthopAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushNexthopAttrs<RequestBuf<'r>> {
        PushNexthopAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nhmsg, IterableNexthopAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nhmsg::len()));
        (
            Nhmsg::new_from_slice(header).unwrap_or_default(),
            IterableNexthopAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Nhmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Nhmsg {
        let pos = self.request.pos;
        Nhmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Nhmsg {
        let pos = self.request.pos;
        Nhmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetnexthopbucketDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 118u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nhmsg, IterableNexthopAttrs<'buf>);
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
    #[doc = "Add new nexthop entry\n\nRequest attributes:\n- [.push_id()](PushNexthopAttrs::push_id)\n- [.push_group()](PushNexthopAttrs::push_group)\n- [.push_group_type()](PushNexthopAttrs::push_group_type)\n- [.push_blackhole()](PushNexthopAttrs::push_blackhole)\n- [.push_oif()](PushNexthopAttrs::push_oif)\n- [.push_gateway()](PushNexthopAttrs::push_gateway)\n- [.push_encap_type()](PushNexthopAttrs::push_encap_type)\n- [.nested_encap_mpls()](PushNexthopAttrs::nested_encap_mpls)\n- [.nested_encap_ip()](PushNexthopAttrs::nested_encap_ip)\n- [.nested_encap_ila()](PushNexthopAttrs::nested_encap_ila)\n- [.nested_encap_ip6()](PushNexthopAttrs::nested_encap_ip6)\n- [.nested_encap_seg6()](PushNexthopAttrs::nested_encap_seg6)\n- [.nested_encap_bpf()](PushNexthopAttrs::nested_encap_bpf)\n- [.nested_encap_seg6_local()](PushNexthopAttrs::nested_encap_seg6_local)\n- [.nested_encap_rpl()](PushNexthopAttrs::nested_encap_rpl)\n- [.nested_encap_ioam6()](PushNexthopAttrs::nested_encap_ioam6)\n- [.nested_encap_xfrm()](PushNexthopAttrs::nested_encap_xfrm)\n- [.push_fdb()](PushNexthopAttrs::push_fdb)\n- [.nested_res_group()](PushNexthopAttrs::nested_res_group)\n- [.push_hw_stats_enable()](PushNexthopAttrs::push_hw_stats_enable)\n- [.push_dst_port()](PushNexthopAttrs::push_dst_port)\n\n"]
    pub fn op_newnexthop_do(self, header: &Nhmsg) -> OpNewnexthopDo<'buf> {
        let mut res = OpNewnexthopDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-newnexthop-do", OpNewnexthopDo::lookup);
        res
    }
    #[doc = "Delete nexthop with given id.\n\nRequest attributes:\n- [.push_id()](PushNexthopAttrs::push_id)\n\n"]
    pub fn op_delnexthop_do(self, header: &Nhmsg) -> OpDelnexthopDo<'buf> {
        let mut res = OpDelnexthopDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-delnexthop-do", OpDelnexthopDo::lookup);
        res
    }
    #[doc = "Get or dump nexthop entries\n\nRequest attributes:\n- [.push_oif()](PushNexthopAttrs::push_oif)\n- [.push_groups()](PushNexthopAttrs::push_groups)\n- [.push_master()](PushNexthopAttrs::push_master)\n- [.push_fdb()](PushNexthopAttrs::push_fdb)\n- [.push_op_flags()](PushNexthopAttrs::push_op_flags)\n\nReply attributes:\n- [.get_id()](IterableNexthopAttrs::get_id)\n- [.get_group()](IterableNexthopAttrs::get_group)\n- [.get_group_type()](IterableNexthopAttrs::get_group_type)\n- [.get_blackhole()](IterableNexthopAttrs::get_blackhole)\n- [.get_oif()](IterableNexthopAttrs::get_oif)\n- [.get_gateway()](IterableNexthopAttrs::get_gateway)\n- [.get_encap_type()](IterableNexthopAttrs::get_encap_type)\n- [.get_encap()](IterableNexthopAttrs::get_encap)\n- [.get_fdb()](IterableNexthopAttrs::get_fdb)\n- [.get_res_group()](IterableNexthopAttrs::get_res_group)\n- [.get_op_flags()](IterableNexthopAttrs::get_op_flags)\n- [.get_group_stats()](IterableNexthopAttrs::get_group_stats)\n- [.get_hw_stats_enable()](IterableNexthopAttrs::get_hw_stats_enable)\n- [.get_hw_stats_used()](IterableNexthopAttrs::get_hw_stats_used)\n- [.get_dst_port()](IterableNexthopAttrs::get_dst_port)\n\n"]
    pub fn op_getnexthop_dump(self, header: &Nhmsg) -> OpGetnexthopDump<'buf> {
        let mut res = OpGetnexthopDump::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-getnexthop-dump",
            OpGetnexthopDump::lookup,
        );
        res
    }
    #[doc = "Get or dump nexthop entries\n\nRequest attributes:\n- [.push_id()](PushNexthopAttrs::push_id)\n- [.push_op_flags()](PushNexthopAttrs::push_op_flags)\n\nReply attributes:\n- [.get_id()](IterableNexthopAttrs::get_id)\n- [.get_group()](IterableNexthopAttrs::get_group)\n- [.get_group_type()](IterableNexthopAttrs::get_group_type)\n- [.get_blackhole()](IterableNexthopAttrs::get_blackhole)\n- [.get_oif()](IterableNexthopAttrs::get_oif)\n- [.get_gateway()](IterableNexthopAttrs::get_gateway)\n- [.get_encap_type()](IterableNexthopAttrs::get_encap_type)\n- [.get_encap()](IterableNexthopAttrs::get_encap)\n- [.get_fdb()](IterableNexthopAttrs::get_fdb)\n- [.get_res_group()](IterableNexthopAttrs::get_res_group)\n- [.get_op_flags()](IterableNexthopAttrs::get_op_flags)\n- [.get_group_stats()](IterableNexthopAttrs::get_group_stats)\n- [.get_hw_stats_enable()](IterableNexthopAttrs::get_hw_stats_enable)\n- [.get_hw_stats_used()](IterableNexthopAttrs::get_hw_stats_used)\n- [.get_dst_port()](IterableNexthopAttrs::get_dst_port)\n\n"]
    pub fn op_getnexthop_do(self, header: &Nhmsg) -> OpGetnexthopDo<'buf> {
        let mut res = OpGetnexthopDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getnexthop-do", OpGetnexthopDo::lookup);
        res
    }
    #[doc = "Get or dump nexthop buckets\n\nRequest attributes:\n- [.push_id()](PushNexthopAttrs::push_id)\n- [.push_oif()](PushNexthopAttrs::push_oif)\n- [.push_master()](PushNexthopAttrs::push_master)\n- [.nested_res_bucket()](PushNexthopAttrs::nested_res_bucket)\n\nReply attributes:\n- [.get_id()](IterableNexthopAttrs::get_id)\n- [.get_res_bucket()](IterableNexthopAttrs::get_res_bucket)\n\n"]
    pub fn op_getnexthopbucket_dump(self, header: &Nhmsg) -> OpGetnexthopbucketDump<'buf> {
        let mut res = OpGetnexthopbucketDump::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-getnexthopbucket-dump",
            OpGetnexthopbucketDump::lookup,
        );
        res
    }
    #[doc = "Get or dump nexthop buckets\n\nRequest attributes:\n- [.push_id()](PushNexthopAttrs::push_id)\n- [.nested_res_bucket()](PushNexthopAttrs::nested_res_bucket)\n\nReply attributes:\n- [.get_id()](IterableNexthopAttrs::get_id)\n- [.get_res_bucket()](IterableNexthopAttrs::get_res_bucket)\n\n"]
    pub fn op_getnexthopbucket_do(self, header: &Nhmsg) -> OpGetnexthopbucketDo<'buf> {
        let mut res = OpGetnexthopbucketDo::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-getnexthopbucket-do",
            OpGetnexthopbucketDo::lookup,
        );
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableNexthopAttrs::get_blackhole;
        let _ = IterableNexthopAttrs::get_dst_port;
        let _ = IterableNexthopAttrs::get_encap;
        let _ = IterableNexthopAttrs::get_encap_type;
        let _ = IterableNexthopAttrs::get_fdb;
        let _ = IterableNexthopAttrs::get_gateway;
        let _ = IterableNexthopAttrs::get_group;
        let _ = IterableNexthopAttrs::get_group_stats;
        let _ = IterableNexthopAttrs::get_group_type;
        let _ = IterableNexthopAttrs::get_hw_stats_enable;
        let _ = IterableNexthopAttrs::get_hw_stats_used;
        let _ = IterableNexthopAttrs::get_id;
        let _ = IterableNexthopAttrs::get_oif;
        let _ = IterableNexthopAttrs::get_op_flags;
        let _ = IterableNexthopAttrs::get_res_bucket;
        let _ = IterableNexthopAttrs::get_res_group;
        let _ = PushNexthopAttrs::<&mut Vec<u8>>::nested_encap_bpf;
        let _ = PushNexthopAttrs::<&mut Vec<u8>>::nested_encap_ila;
        let _ = PushNexthopAttrs::<&mut Vec<u8>>::nested_encap_ioam6;
        let _ = PushNexthopAttrs::<&mut Vec<u8>>::nested_encap_ip6;
        let _ = PushNexthopAttrs::<&mut Vec<u8>>::nested_encap_ip;
        let _ = PushNexthopAttrs::<&mut Vec<u8>>::nested_encap_mpls;
        let _ = PushNexthopAttrs::<&mut Vec<u8>>::nested_encap_rpl;
        let _ = PushNexthopAttrs::<&mut Vec<u8>>::nested_encap_seg6;
        let _ = PushNexthopAttrs::<&mut Vec<u8>>::nested_encap_seg6_local;
        let _ = PushNexthopAttrs::<&mut Vec<u8>>::nested_encap_xfrm;
        let _ = PushNexthopAttrs::<&mut Vec<u8>>::nested_res_bucket;
        let _ = PushNexthopAttrs::<&mut Vec<u8>>::nested_res_group;
        let _ = PushNexthopAttrs::<&mut Vec<u8>>::push_blackhole;
        let _ = PushNexthopAttrs::<&mut Vec<u8>>::push_dst_port;
        let _ = PushNexthopAttrs::<&mut Vec<u8>>::push_encap_type;
        let _ = PushNexthopAttrs::<&mut Vec<u8>>::push_fdb;
        let _ = PushNexthopAttrs::<&mut Vec<u8>>::push_gateway;
        let _ = PushNexthopAttrs::<&mut Vec<u8>>::push_group;
        let _ = PushNexthopAttrs::<&mut Vec<u8>>::push_group_type;
        let _ = PushNexthopAttrs::<&mut Vec<u8>>::push_groups;
        let _ = PushNexthopAttrs::<&mut Vec<u8>>::push_hw_stats_enable;
        let _ = PushNexthopAttrs::<&mut Vec<u8>>::push_id;
        let _ = PushNexthopAttrs::<&mut Vec<u8>>::push_master;
        let _ = PushNexthopAttrs::<&mut Vec<u8>>::push_oif;
        let _ = PushNexthopAttrs::<&mut Vec<u8>>::push_op_flags;
    }
}
