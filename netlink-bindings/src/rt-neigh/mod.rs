#![doc = "IP neighbour management over rtnetlink.\n"]
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
pub const PROTONAME: &str = "rt-neigh";
pub const PROTONAME_CSTR: &CStr = c"rt-neigh";
pub const PROTONUM: u16 = 0u16;
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum NudState {
    Incomplete = 1 << 0,
    Reachable = 1 << 1,
    Stale = 1 << 2,
    Delay = 1 << 3,
    Probe = 1 << 4,
    Failed = 1 << 5,
    Noarp = 1 << 6,
    Permanent = 1 << 7,
}
impl NudState {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::Incomplete,
            n if n == 1 << 1 => Self::Reachable,
            n if n == 1 << 2 => Self::Stale,
            n if n == 1 << 3 => Self::Delay,
            n if n == 1 << 4 => Self::Probe,
            n if n == 1 << 5 => Self::Failed,
            n if n == 1 << 6 => Self::Noarp,
            n if n == 1 << 7 => Self::Permanent,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum NtfFlags {
    Use = 1 << 0,
    _Self = 1 << 1,
    Master = 1 << 2,
    Proxy = 1 << 3,
    ExtLearned = 1 << 4,
    Offloaded = 1 << 5,
    Sticky = 1 << 6,
    Router = 1 << 7,
}
impl NtfFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::Use,
            n if n == 1 << 1 => Self::_Self,
            n if n == 1 << 2 => Self::Master,
            n if n == 1 << 3 => Self::Proxy,
            n if n == 1 << 4 => Self::ExtLearned,
            n if n == 1 << 5 => Self::Offloaded,
            n if n == 1 << 6 => Self::Sticky,
            n if n == 1 << 7 => Self::Router,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum NtfExtFlags {
    Managed = 1 << 0,
    Locked = 1 << 1,
    ExtValidated = 1 << 2,
}
impl NtfExtFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::Managed,
            n if n == 1 << 1 => Self::Locked,
            n if n == 1 << 2 => Self::ExtValidated,
            _ => return None,
        })
    }
}
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
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Ndmsg {
    pub ndm_family: u8,
    pub _ndm_pad: [u8; 3usize],
    pub ndm_ifindex: i32,
    #[doc = "Associated type: [`NudState`] (enum)"]
    pub ndm_state: u16,
    #[doc = "Associated type: [`NtfFlags`] (enum)"]
    pub ndm_flags: u8,
    #[doc = "Associated type: [`RtmType`] (enum)"]
    pub ndm_type: u8,
}
#[doc = "Create zero-initialized struct"]
impl Default for Ndmsg {
    fn default() -> Self {
        Self::new()
    }
}
impl Ndmsg {
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
        const _: () = assert!(std::mem::size_of::<Ndmsg>() == 12usize);
        const _: () = assert!(std::mem::align_of::<Ndmsg>() == 4usize);
        12usize
    }
}
impl std::fmt::Debug for Ndmsg {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Ndmsg")
            .field("ndm_family", &self.ndm_family)
            .field("ndm_ifindex", &self.ndm_ifindex)
            .field(
                "ndm_state",
                &FormatFlags(self.ndm_state.into(), NudState::from_value),
            )
            .field(
                "ndm_flags",
                &FormatFlags(self.ndm_flags.into(), NtfFlags::from_value),
            )
            .field(
                "ndm_type",
                &FormatEnum(self.ndm_type.into(), RtmType::from_value),
            )
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Ndtmsg {
    pub family: u8,
    pub _pad: [u8; 3usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for Ndtmsg {
    fn default() -> Self {
        Self::new()
    }
}
impl Ndtmsg {
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
        const _: () = assert!(std::mem::size_of::<Ndtmsg>() == 4usize);
        const _: () = assert!(std::mem::align_of::<Ndtmsg>() == 1usize);
        4usize
    }
}
impl std::fmt::Debug for Ndtmsg {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Ndtmsg")
            .field("family", &self.family)
            .finish()
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct NdaCacheinfo {
    pub confirmed: u32,
    pub used: u32,
    pub updated: u32,
    pub refcnt: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for NdaCacheinfo {
    fn default() -> Self {
        Self::new()
    }
}
impl NdaCacheinfo {
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
    pub fn new_from_array(buf: [u8; 16usize]) -> Self {
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
    pub fn as_array(&self) -> &[u8; 16usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 16usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 16usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<NdaCacheinfo>() == 16usize);
        const _: () = assert!(std::mem::align_of::<NdaCacheinfo>() == 4usize);
        16usize
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct NdtConfig {
    pub key_len: u16,
    pub entry_size: u16,
    pub entries: u32,
    pub last_flush: u32,
    pub last_rand: u32,
    pub hash_rnd: u32,
    pub hash_mask: u32,
    pub hash_chain_gc: u32,
    pub proxy_qlen: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for NdtConfig {
    fn default() -> Self {
        Self::new()
    }
}
impl NdtConfig {
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
    pub fn new_from_array(buf: [u8; 32usize]) -> Self {
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
    pub fn as_array(&self) -> &[u8; 32usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 32usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 32usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<NdtConfig>() == 32usize);
        const _: () = assert!(std::mem::align_of::<NdtConfig>() == 4usize);
        32usize
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(packed(4))]
#[repr(C)]
pub struct NdtStats {
    pub allocs: u64,
    pub destroys: u64,
    pub hash_grows: u64,
    pub res_failed: u64,
    pub lookups: u64,
    pub hits: u64,
    pub rcv_probes_mcast: u64,
    pub rcv_probes_ucast: u64,
    pub periodic_gc_runs: u64,
    pub forced_gc_runs: u64,
    pub table_fulls: u64,
}
#[doc = "Create zero-initialized struct"]
impl Default for NdtStats {
    fn default() -> Self {
        Self::new()
    }
}
impl NdtStats {
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
    pub fn new_from_array(buf: [u8; 88usize]) -> Self {
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
    pub fn as_array(&self) -> &[u8; 88usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 88usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 88usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<NdtStats>() == 88usize);
        const _: () = assert!(std::mem::align_of::<NdtStats>() == 4usize);
        88usize
    }
}
impl std::fmt::Debug for NdtStats {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("NdtStats")
            .field("allocs", &{ self.allocs })
            .field("destroys", &{ self.destroys })
            .field("hash_grows", &{ self.hash_grows })
            .field("res_failed", &{ self.res_failed })
            .field("lookups", &{ self.lookups })
            .field("hits", &{ self.hits })
            .field("rcv_probes_mcast", &{ self.rcv_probes_mcast })
            .field("rcv_probes_ucast", &{ self.rcv_probes_ucast })
            .field("periodic_gc_runs", &{ self.periodic_gc_runs })
            .field("forced_gc_runs", &{ self.forced_gc_runs })
            .field("table_fulls", &{ self.table_fulls })
            .finish()
    }
}
#[derive(Clone)]
pub enum NeighbourAttrs<'a> {
    Unspec(&'a [u8]),
    Dst(std::net::IpAddr),
    Lladdr(&'a [u8]),
    Cacheinfo(NdaCacheinfo),
    Probes(u32),
    Vlan(u16),
    Port(u16),
    Vni(u32),
    Ifindex(u32),
    Master(u32),
    LinkNetnsid(i32),
    SrcVni(u32),
    Protocol(u8),
    NhId(u32),
    FdbExtAttrs(&'a [u8]),
    #[doc = "Associated type: [`NtfExtFlags`] (enum)"]
    FlagsExt(u32),
    NdmStateMask(u16),
    NdmFlagsMask(u8),
}
impl<'a> IterableNeighbourAttrs<'a> {
    pub fn get_unspec(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NeighbourAttrs::Unspec(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NeighbourAttrs",
            "Unspec",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dst(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NeighbourAttrs::Dst(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NeighbourAttrs",
            "Dst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_lladdr(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NeighbourAttrs::Lladdr(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NeighbourAttrs",
            "Lladdr",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_cacheinfo(&self) -> Result<NdaCacheinfo, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NeighbourAttrs::Cacheinfo(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NeighbourAttrs",
            "Cacheinfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_probes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NeighbourAttrs::Probes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NeighbourAttrs",
            "Probes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vlan(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NeighbourAttrs::Vlan(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NeighbourAttrs",
            "Vlan",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NeighbourAttrs::Port(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NeighbourAttrs",
            "Port",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vni(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NeighbourAttrs::Vni(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NeighbourAttrs",
            "Vni",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NeighbourAttrs::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NeighbourAttrs",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_master(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NeighbourAttrs::Master(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NeighbourAttrs",
            "Master",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_link_netnsid(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NeighbourAttrs::LinkNetnsid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NeighbourAttrs",
            "LinkNetnsid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_src_vni(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NeighbourAttrs::SrcVni(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NeighbourAttrs",
            "SrcVni",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_protocol(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NeighbourAttrs::Protocol(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NeighbourAttrs",
            "Protocol",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nh_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NeighbourAttrs::NhId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NeighbourAttrs",
            "NhId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fdb_ext_attrs(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NeighbourAttrs::FdbExtAttrs(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NeighbourAttrs",
            "FdbExtAttrs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`NtfExtFlags`] (enum)"]
    pub fn get_flags_ext(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NeighbourAttrs::FlagsExt(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NeighbourAttrs",
            "FlagsExt",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ndm_state_mask(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NeighbourAttrs::NdmStateMask(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NeighbourAttrs",
            "NdmStateMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ndm_flags_mask(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NeighbourAttrs::NdmFlagsMask(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NeighbourAttrs",
            "NdmFlagsMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl NeighbourAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableNeighbourAttrs<'a> {
        IterableNeighbourAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "Unspec",
            1u16 => "Dst",
            2u16 => "Lladdr",
            3u16 => "Cacheinfo",
            4u16 => "Probes",
            5u16 => "Vlan",
            6u16 => "Port",
            7u16 => "Vni",
            8u16 => "Ifindex",
            9u16 => "Master",
            10u16 => "LinkNetnsid",
            11u16 => "SrcVni",
            12u16 => "Protocol",
            13u16 => "NhId",
            14u16 => "FdbExtAttrs",
            15u16 => "FlagsExt",
            16u16 => "NdmStateMask",
            17u16 => "NdmFlagsMask",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableNeighbourAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableNeighbourAttrs<'a> {
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
impl<'a> Iterator for IterableNeighbourAttrs<'a> {
    type Item = Result<NeighbourAttrs<'a>, ErrorContext>;
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
                0u16 => NeighbourAttrs::Unspec({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                1u16 => NeighbourAttrs::Dst({
                    let res = parse_ip(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => NeighbourAttrs::Lladdr({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => NeighbourAttrs::Cacheinfo({
                    let res = Some(NdaCacheinfo::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => NeighbourAttrs::Probes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => NeighbourAttrs::Vlan({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => NeighbourAttrs::Port({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => NeighbourAttrs::Vni({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => NeighbourAttrs::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => NeighbourAttrs::Master({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => NeighbourAttrs::LinkNetnsid({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => NeighbourAttrs::SrcVni({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => NeighbourAttrs::Protocol({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => NeighbourAttrs::NhId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => NeighbourAttrs::FdbExtAttrs({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => NeighbourAttrs::FlagsExt({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => NeighbourAttrs::NdmStateMask({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => NeighbourAttrs::NdmFlagsMask({
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
            "NeighbourAttrs",
            r#type.and_then(|t| NeighbourAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableNeighbourAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("NeighbourAttrs");
        let mut iter = IterableNeighbourAttrs::with_loc(&[], self.orig_loc);
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
                NeighbourAttrs::Unspec(val) => fmt.field("Unspec", &FormatHexdump(val)),
                NeighbourAttrs::Dst(val) => fmt.field("Dst", &val),
                NeighbourAttrs::Lladdr(val) => fmt.field("Lladdr", &FormatMac(val)),
                NeighbourAttrs::Cacheinfo(val) => fmt.field("Cacheinfo", &val),
                NeighbourAttrs::Probes(val) => fmt.field("Probes", &val),
                NeighbourAttrs::Vlan(val) => fmt.field("Vlan", &val),
                NeighbourAttrs::Port(val) => fmt.field("Port", &val),
                NeighbourAttrs::Vni(val) => fmt.field("Vni", &val),
                NeighbourAttrs::Ifindex(val) => fmt.field("Ifindex", &val),
                NeighbourAttrs::Master(val) => fmt.field("Master", &val),
                NeighbourAttrs::LinkNetnsid(val) => fmt.field("LinkNetnsid", &val),
                NeighbourAttrs::SrcVni(val) => fmt.field("SrcVni", &val),
                NeighbourAttrs::Protocol(val) => fmt.field("Protocol", &val),
                NeighbourAttrs::NhId(val) => fmt.field("NhId", &val),
                NeighbourAttrs::FdbExtAttrs(val) => fmt.field("FdbExtAttrs", &FormatHexdump(val)),
                NeighbourAttrs::FlagsExt(val) => fmt.field(
                    "FlagsExt",
                    &FormatFlags(val.into(), NtfExtFlags::from_value),
                ),
                NeighbourAttrs::NdmStateMask(val) => fmt.field("NdmStateMask", &val),
                NeighbourAttrs::NdmFlagsMask(val) => fmt.field("NdmFlagsMask", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableNeighbourAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("NeighbourAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| NeighbourAttrs::attr_from_type(t)),
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
                NeighbourAttrs::Unspec(val) => {
                    if last_off == offset {
                        stack.push(("Unspec", last_off));
                        break;
                    }
                }
                NeighbourAttrs::Dst(val) => {
                    if last_off == offset {
                        stack.push(("Dst", last_off));
                        break;
                    }
                }
                NeighbourAttrs::Lladdr(val) => {
                    if last_off == offset {
                        stack.push(("Lladdr", last_off));
                        break;
                    }
                }
                NeighbourAttrs::Cacheinfo(val) => {
                    if last_off == offset {
                        stack.push(("Cacheinfo", last_off));
                        break;
                    }
                }
                NeighbourAttrs::Probes(val) => {
                    if last_off == offset {
                        stack.push(("Probes", last_off));
                        break;
                    }
                }
                NeighbourAttrs::Vlan(val) => {
                    if last_off == offset {
                        stack.push(("Vlan", last_off));
                        break;
                    }
                }
                NeighbourAttrs::Port(val) => {
                    if last_off == offset {
                        stack.push(("Port", last_off));
                        break;
                    }
                }
                NeighbourAttrs::Vni(val) => {
                    if last_off == offset {
                        stack.push(("Vni", last_off));
                        break;
                    }
                }
                NeighbourAttrs::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                NeighbourAttrs::Master(val) => {
                    if last_off == offset {
                        stack.push(("Master", last_off));
                        break;
                    }
                }
                NeighbourAttrs::LinkNetnsid(val) => {
                    if last_off == offset {
                        stack.push(("LinkNetnsid", last_off));
                        break;
                    }
                }
                NeighbourAttrs::SrcVni(val) => {
                    if last_off == offset {
                        stack.push(("SrcVni", last_off));
                        break;
                    }
                }
                NeighbourAttrs::Protocol(val) => {
                    if last_off == offset {
                        stack.push(("Protocol", last_off));
                        break;
                    }
                }
                NeighbourAttrs::NhId(val) => {
                    if last_off == offset {
                        stack.push(("NhId", last_off));
                        break;
                    }
                }
                NeighbourAttrs::FdbExtAttrs(val) => {
                    if last_off == offset {
                        stack.push(("FdbExtAttrs", last_off));
                        break;
                    }
                }
                NeighbourAttrs::FlagsExt(val) => {
                    if last_off == offset {
                        stack.push(("FlagsExt", last_off));
                        break;
                    }
                }
                NeighbourAttrs::NdmStateMask(val) => {
                    if last_off == offset {
                        stack.push(("NdmStateMask", last_off));
                        break;
                    }
                }
                NeighbourAttrs::NdmFlagsMask(val) => {
                    if last_off == offset {
                        stack.push(("NdmFlagsMask", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("NeighbourAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum NdtAttrs<'a> {
    Name(&'a CStr),
    Thresh1(u32),
    Thresh2(u32),
    Thresh3(u32),
    Config(NdtConfig),
    Parms(IterableNdtpaAttrs<'a>),
    Stats(NdtStats),
    GcInterval(u64),
    Pad(&'a [u8]),
}
impl<'a> IterableNdtAttrs<'a> {
    pub fn get_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtAttrs::Name(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtAttrs",
            "Name",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_thresh1(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtAttrs::Thresh1(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtAttrs",
            "Thresh1",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_thresh2(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtAttrs::Thresh2(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtAttrs",
            "Thresh2",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_thresh3(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtAttrs::Thresh3(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtAttrs",
            "Thresh3",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_config(&self) -> Result<NdtConfig, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtAttrs::Config(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtAttrs",
            "Config",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_parms(&self) -> Result<IterableNdtpaAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtAttrs::Parms(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtAttrs",
            "Parms",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_stats(&self) -> Result<NdtStats, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtAttrs::Stats(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtAttrs",
            "Stats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_gc_interval(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtAttrs::GcInterval(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtAttrs",
            "GcInterval",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtAttrs::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtAttrs",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl NdtAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableNdtAttrs<'a> {
        IterableNdtAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Name",
            2u16 => "Thresh1",
            3u16 => "Thresh2",
            4u16 => "Thresh3",
            5u16 => "Config",
            6u16 => "Parms",
            7u16 => "Stats",
            8u16 => "GcInterval",
            9u16 => "Pad",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableNdtAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableNdtAttrs<'a> {
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
impl<'a> Iterator for IterableNdtAttrs<'a> {
    type Item = Result<NdtAttrs<'a>, ErrorContext>;
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
                1u16 => NdtAttrs::Name({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => NdtAttrs::Thresh1({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => NdtAttrs::Thresh2({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => NdtAttrs::Thresh3({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => NdtAttrs::Config({
                    let res = Some(NdtConfig::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => NdtAttrs::Parms({
                    let res = Some(IterableNdtpaAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => NdtAttrs::Stats({
                    let res = Some(NdtStats::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => NdtAttrs::GcInterval({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => NdtAttrs::Pad({
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
            "NdtAttrs",
            r#type.and_then(|t| NdtAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableNdtAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("NdtAttrs");
        let mut iter = IterableNdtAttrs::with_loc(&[], self.orig_loc);
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
                NdtAttrs::Name(val) => fmt.field("Name", &val),
                NdtAttrs::Thresh1(val) => fmt.field("Thresh1", &val),
                NdtAttrs::Thresh2(val) => fmt.field("Thresh2", &val),
                NdtAttrs::Thresh3(val) => fmt.field("Thresh3", &val),
                NdtAttrs::Config(val) => fmt.field("Config", &val),
                NdtAttrs::Parms(val) => fmt.field("Parms", &val),
                NdtAttrs::Stats(val) => fmt.field("Stats", &val),
                NdtAttrs::GcInterval(val) => fmt.field("GcInterval", &val),
                NdtAttrs::Pad(val) => fmt.field("Pad", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableNdtAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("NdtAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| NdtAttrs::attr_from_type(t)),
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
                NdtAttrs::Name(val) => {
                    if last_off == offset {
                        stack.push(("Name", last_off));
                        break;
                    }
                }
                NdtAttrs::Thresh1(val) => {
                    if last_off == offset {
                        stack.push(("Thresh1", last_off));
                        break;
                    }
                }
                NdtAttrs::Thresh2(val) => {
                    if last_off == offset {
                        stack.push(("Thresh2", last_off));
                        break;
                    }
                }
                NdtAttrs::Thresh3(val) => {
                    if last_off == offset {
                        stack.push(("Thresh3", last_off));
                        break;
                    }
                }
                NdtAttrs::Config(val) => {
                    if last_off == offset {
                        stack.push(("Config", last_off));
                        break;
                    }
                }
                NdtAttrs::Parms(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                NdtAttrs::Stats(val) => {
                    if last_off == offset {
                        stack.push(("Stats", last_off));
                        break;
                    }
                }
                NdtAttrs::GcInterval(val) => {
                    if last_off == offset {
                        stack.push(("GcInterval", last_off));
                        break;
                    }
                }
                NdtAttrs::Pad(val) => {
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
            stack.push(("NdtAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum NdtpaAttrs<'a> {
    Ifindex(u32),
    Refcnt(u32),
    ReachableTime(u64),
    BaseReachableTime(u64),
    RetransTime(u64),
    GcStaletime(u64),
    DelayProbeTime(u64),
    QueueLen(u32),
    AppProbes(u32),
    UcastProbes(u32),
    McastProbes(u32),
    AnycastDelay(u64),
    ProxyDelay(u64),
    ProxyQlen(u32),
    Locktime(u64),
    QueueLenbytes(u32),
    McastReprobes(u32),
    Pad(&'a [u8]),
    IntervalProbeTimeMs(u64),
}
impl<'a> IterableNdtpaAttrs<'a> {
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtpaAttrs::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtpaAttrs",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_refcnt(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtpaAttrs::Refcnt(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtpaAttrs",
            "Refcnt",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_reachable_time(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtpaAttrs::ReachableTime(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtpaAttrs",
            "ReachableTime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_base_reachable_time(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtpaAttrs::BaseReachableTime(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtpaAttrs",
            "BaseReachableTime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_retrans_time(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtpaAttrs::RetransTime(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtpaAttrs",
            "RetransTime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_gc_staletime(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtpaAttrs::GcStaletime(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtpaAttrs",
            "GcStaletime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_delay_probe_time(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtpaAttrs::DelayProbeTime(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtpaAttrs",
            "DelayProbeTime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_queue_len(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtpaAttrs::QueueLen(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtpaAttrs",
            "QueueLen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_app_probes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtpaAttrs::AppProbes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtpaAttrs",
            "AppProbes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ucast_probes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtpaAttrs::UcastProbes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtpaAttrs",
            "UcastProbes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_probes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtpaAttrs::McastProbes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtpaAttrs",
            "McastProbes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_anycast_delay(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtpaAttrs::AnycastDelay(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtpaAttrs",
            "AnycastDelay",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_proxy_delay(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtpaAttrs::ProxyDelay(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtpaAttrs",
            "ProxyDelay",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_proxy_qlen(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtpaAttrs::ProxyQlen(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtpaAttrs",
            "ProxyQlen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_locktime(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtpaAttrs::Locktime(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtpaAttrs",
            "Locktime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_queue_lenbytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtpaAttrs::QueueLenbytes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtpaAttrs",
            "QueueLenbytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_reprobes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtpaAttrs::McastReprobes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtpaAttrs",
            "McastReprobes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtpaAttrs::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtpaAttrs",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_interval_probe_time_ms(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NdtpaAttrs::IntervalProbeTimeMs(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NdtpaAttrs",
            "IntervalProbeTimeMs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl NdtpaAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableNdtpaAttrs<'a> {
        IterableNdtpaAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Ifindex",
            2u16 => "Refcnt",
            3u16 => "ReachableTime",
            4u16 => "BaseReachableTime",
            5u16 => "RetransTime",
            6u16 => "GcStaletime",
            7u16 => "DelayProbeTime",
            8u16 => "QueueLen",
            9u16 => "AppProbes",
            10u16 => "UcastProbes",
            11u16 => "McastProbes",
            12u16 => "AnycastDelay",
            13u16 => "ProxyDelay",
            14u16 => "ProxyQlen",
            15u16 => "Locktime",
            16u16 => "QueueLenbytes",
            17u16 => "McastReprobes",
            18u16 => "Pad",
            19u16 => "IntervalProbeTimeMs",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableNdtpaAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableNdtpaAttrs<'a> {
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
impl<'a> Iterator for IterableNdtpaAttrs<'a> {
    type Item = Result<NdtpaAttrs<'a>, ErrorContext>;
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
                1u16 => NdtpaAttrs::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => NdtpaAttrs::Refcnt({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => NdtpaAttrs::ReachableTime({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => NdtpaAttrs::BaseReachableTime({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => NdtpaAttrs::RetransTime({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => NdtpaAttrs::GcStaletime({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => NdtpaAttrs::DelayProbeTime({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => NdtpaAttrs::QueueLen({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => NdtpaAttrs::AppProbes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => NdtpaAttrs::UcastProbes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => NdtpaAttrs::McastProbes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => NdtpaAttrs::AnycastDelay({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => NdtpaAttrs::ProxyDelay({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => NdtpaAttrs::ProxyQlen({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => NdtpaAttrs::Locktime({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => NdtpaAttrs::QueueLenbytes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => NdtpaAttrs::McastReprobes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => NdtpaAttrs::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => NdtpaAttrs::IntervalProbeTimeMs({
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
            "NdtpaAttrs",
            r#type.and_then(|t| NdtpaAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableNdtpaAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("NdtpaAttrs");
        let mut iter = IterableNdtpaAttrs::with_loc(&[], self.orig_loc);
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
                NdtpaAttrs::Ifindex(val) => fmt.field("Ifindex", &val),
                NdtpaAttrs::Refcnt(val) => fmt.field("Refcnt", &val),
                NdtpaAttrs::ReachableTime(val) => fmt.field("ReachableTime", &val),
                NdtpaAttrs::BaseReachableTime(val) => fmt.field("BaseReachableTime", &val),
                NdtpaAttrs::RetransTime(val) => fmt.field("RetransTime", &val),
                NdtpaAttrs::GcStaletime(val) => fmt.field("GcStaletime", &val),
                NdtpaAttrs::DelayProbeTime(val) => fmt.field("DelayProbeTime", &val),
                NdtpaAttrs::QueueLen(val) => fmt.field("QueueLen", &val),
                NdtpaAttrs::AppProbes(val) => fmt.field("AppProbes", &val),
                NdtpaAttrs::UcastProbes(val) => fmt.field("UcastProbes", &val),
                NdtpaAttrs::McastProbes(val) => fmt.field("McastProbes", &val),
                NdtpaAttrs::AnycastDelay(val) => fmt.field("AnycastDelay", &val),
                NdtpaAttrs::ProxyDelay(val) => fmt.field("ProxyDelay", &val),
                NdtpaAttrs::ProxyQlen(val) => fmt.field("ProxyQlen", &val),
                NdtpaAttrs::Locktime(val) => fmt.field("Locktime", &val),
                NdtpaAttrs::QueueLenbytes(val) => fmt.field("QueueLenbytes", &val),
                NdtpaAttrs::McastReprobes(val) => fmt.field("McastReprobes", &val),
                NdtpaAttrs::Pad(val) => fmt.field("Pad", &val),
                NdtpaAttrs::IntervalProbeTimeMs(val) => fmt.field("IntervalProbeTimeMs", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableNdtpaAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("NdtpaAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| NdtpaAttrs::attr_from_type(t)),
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
                NdtpaAttrs::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                NdtpaAttrs::Refcnt(val) => {
                    if last_off == offset {
                        stack.push(("Refcnt", last_off));
                        break;
                    }
                }
                NdtpaAttrs::ReachableTime(val) => {
                    if last_off == offset {
                        stack.push(("ReachableTime", last_off));
                        break;
                    }
                }
                NdtpaAttrs::BaseReachableTime(val) => {
                    if last_off == offset {
                        stack.push(("BaseReachableTime", last_off));
                        break;
                    }
                }
                NdtpaAttrs::RetransTime(val) => {
                    if last_off == offset {
                        stack.push(("RetransTime", last_off));
                        break;
                    }
                }
                NdtpaAttrs::GcStaletime(val) => {
                    if last_off == offset {
                        stack.push(("GcStaletime", last_off));
                        break;
                    }
                }
                NdtpaAttrs::DelayProbeTime(val) => {
                    if last_off == offset {
                        stack.push(("DelayProbeTime", last_off));
                        break;
                    }
                }
                NdtpaAttrs::QueueLen(val) => {
                    if last_off == offset {
                        stack.push(("QueueLen", last_off));
                        break;
                    }
                }
                NdtpaAttrs::AppProbes(val) => {
                    if last_off == offset {
                        stack.push(("AppProbes", last_off));
                        break;
                    }
                }
                NdtpaAttrs::UcastProbes(val) => {
                    if last_off == offset {
                        stack.push(("UcastProbes", last_off));
                        break;
                    }
                }
                NdtpaAttrs::McastProbes(val) => {
                    if last_off == offset {
                        stack.push(("McastProbes", last_off));
                        break;
                    }
                }
                NdtpaAttrs::AnycastDelay(val) => {
                    if last_off == offset {
                        stack.push(("AnycastDelay", last_off));
                        break;
                    }
                }
                NdtpaAttrs::ProxyDelay(val) => {
                    if last_off == offset {
                        stack.push(("ProxyDelay", last_off));
                        break;
                    }
                }
                NdtpaAttrs::ProxyQlen(val) => {
                    if last_off == offset {
                        stack.push(("ProxyQlen", last_off));
                        break;
                    }
                }
                NdtpaAttrs::Locktime(val) => {
                    if last_off == offset {
                        stack.push(("Locktime", last_off));
                        break;
                    }
                }
                NdtpaAttrs::QueueLenbytes(val) => {
                    if last_off == offset {
                        stack.push(("QueueLenbytes", last_off));
                        break;
                    }
                }
                NdtpaAttrs::McastReprobes(val) => {
                    if last_off == offset {
                        stack.push(("McastReprobes", last_off));
                        break;
                    }
                }
                NdtpaAttrs::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                NdtpaAttrs::IntervalProbeTimeMs(val) => {
                    if last_off == offset {
                        stack.push(("IntervalProbeTimeMs", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("NdtpaAttrs", cur));
        }
        (stack, None)
    }
}
pub struct PushNeighbourAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushNeighbourAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushNeighbourAttrs<Prev> {
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
    pub fn push_unspec(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 0u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
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
    pub fn push_lladdr(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_cacheinfo(mut self, value: NdaCacheinfo) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_probes(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_vlan(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 5u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_port(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 6u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_vni(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_master(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_link_netnsid(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_src_vni(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 11u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_protocol(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 12u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_nh_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 13u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fdb_ext_attrs(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 14u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "Associated type: [`NtfExtFlags`] (enum)"]
    pub fn push_flags_ext(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 15u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ndm_state_mask(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 16u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ndm_flags_mask(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 17u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushNeighbourAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushNdtAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushNdtAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushNdtAttrs<Prev> {
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
    pub fn push_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_thresh1(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_thresh2(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_thresh3(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_config(mut self, value: NdtConfig) -> Self {
        push_header(self.as_vec_mut(), 5u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn nested_parms(mut self) -> PushNdtpaAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 6u16);
        PushNdtpaAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_stats(mut self, value: NdtStats) -> Self {
        push_header(self.as_vec_mut(), 7u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_gc_interval(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 8u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 9u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushNdtAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushNdtpaAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushNdtpaAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushNdtpaAttrs<Prev> {
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
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_refcnt(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_reachable_time(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 3u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_base_reachable_time(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 4u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_retrans_time(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 5u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_gc_staletime(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 6u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_delay_probe_time(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 7u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_queue_len(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_app_probes(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ucast_probes(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mcast_probes(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 11u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_anycast_delay(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 12u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_proxy_delay(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 13u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_proxy_qlen(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 14u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_locktime(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 15u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_queue_lenbytes(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 16u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mcast_reprobes(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 17u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 18u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_interval_probe_time_ms(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 19u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushNdtpaAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Add new neighbour entry\n\nRequest attributes:\n- [.push_dst()](PushNeighbourAttrs::push_dst)\n- [.push_lladdr()](PushNeighbourAttrs::push_lladdr)\n- [.push_probes()](PushNeighbourAttrs::push_probes)\n- [.push_vlan()](PushNeighbourAttrs::push_vlan)\n- [.push_port()](PushNeighbourAttrs::push_port)\n- [.push_vni()](PushNeighbourAttrs::push_vni)\n- [.push_ifindex()](PushNeighbourAttrs::push_ifindex)\n- [.push_master()](PushNeighbourAttrs::push_master)\n- [.push_protocol()](PushNeighbourAttrs::push_protocol)\n- [.push_nh_id()](PushNeighbourAttrs::push_nh_id)\n- [.push_fdb_ext_attrs()](PushNeighbourAttrs::push_fdb_ext_attrs)\n- [.push_flags_ext()](PushNeighbourAttrs::push_flags_ext)\n\n"]
#[derive(Debug)]
pub struct OpNewneighDo<'r> {
    request: Request<'r>,
}
impl<'r> OpNewneighDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Ndmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Ndmsg,
    ) -> PushNeighbourAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushNeighbourAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushNeighbourAttrs<&mut Vec<u8>> {
        PushNeighbourAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushNeighbourAttrs<RequestBuf<'r>> {
        PushNeighbourAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Ndmsg, IterableNeighbourAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Ndmsg::len()));
        (
            Ndmsg::new_from_slice(header).unwrap_or_default(),
            IterableNeighbourAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Ndmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Ndmsg {
        let pos = self.request.pos;
        Ndmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Ndmsg {
        let pos = self.request.pos;
        Ndmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpNewneighDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 28u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Ndmsg, IterableNeighbourAttrs<'buf>);
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
#[doc = "Remove an existing neighbour entry\n\nRequest attributes:\n- [.push_dst()](PushNeighbourAttrs::push_dst)\n- [.push_ifindex()](PushNeighbourAttrs::push_ifindex)\n\n"]
#[derive(Debug)]
pub struct OpDelneighDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDelneighDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Ndmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Ndmsg,
    ) -> PushNeighbourAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushNeighbourAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushNeighbourAttrs<&mut Vec<u8>> {
        PushNeighbourAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushNeighbourAttrs<RequestBuf<'r>> {
        PushNeighbourAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Ndmsg, IterableNeighbourAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Ndmsg::len()));
        (
            Ndmsg::new_from_slice(header).unwrap_or_default(),
            IterableNeighbourAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Ndmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Ndmsg {
        let pos = self.request.pos;
        Ndmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Ndmsg {
        let pos = self.request.pos;
        Ndmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpDelneighDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 29u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Ndmsg, IterableNeighbourAttrs<'buf>);
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
#[doc = "Get or dump neighbour entries\n\nRequest attributes:\n- [.push_ifindex()](PushNeighbourAttrs::push_ifindex)\n- [.push_master()](PushNeighbourAttrs::push_master)\n\nReply attributes:\n- [.get_dst()](IterableNeighbourAttrs::get_dst)\n- [.get_lladdr()](IterableNeighbourAttrs::get_lladdr)\n- [.get_probes()](IterableNeighbourAttrs::get_probes)\n- [.get_vlan()](IterableNeighbourAttrs::get_vlan)\n- [.get_port()](IterableNeighbourAttrs::get_port)\n- [.get_vni()](IterableNeighbourAttrs::get_vni)\n- [.get_ifindex()](IterableNeighbourAttrs::get_ifindex)\n- [.get_master()](IterableNeighbourAttrs::get_master)\n- [.get_protocol()](IterableNeighbourAttrs::get_protocol)\n- [.get_nh_id()](IterableNeighbourAttrs::get_nh_id)\n- [.get_fdb_ext_attrs()](IterableNeighbourAttrs::get_fdb_ext_attrs)\n- [.get_flags_ext()](IterableNeighbourAttrs::get_flags_ext)\n\n"]
#[derive(Debug)]
pub struct OpGetneighDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetneighDump<'r> {
    pub fn new(mut request: Request<'r>, header: &Ndmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Ndmsg,
    ) -> PushNeighbourAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushNeighbourAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushNeighbourAttrs<&mut Vec<u8>> {
        PushNeighbourAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushNeighbourAttrs<RequestBuf<'r>> {
        PushNeighbourAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Ndmsg, IterableNeighbourAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Ndmsg::len()));
        (
            Ndmsg::new_from_slice(header).unwrap_or_default(),
            IterableNeighbourAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Ndmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Ndmsg {
        let pos = self.request.pos;
        Ndmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Ndmsg {
        let pos = self.request.pos;
        Ndmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetneighDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 30u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Ndmsg, IterableNeighbourAttrs<'buf>);
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
#[doc = "Get or dump neighbour entries\n\nRequest attributes:\n- [.push_dst()](PushNeighbourAttrs::push_dst)\n\nReply attributes:\n- [.get_dst()](IterableNeighbourAttrs::get_dst)\n- [.get_lladdr()](IterableNeighbourAttrs::get_lladdr)\n- [.get_probes()](IterableNeighbourAttrs::get_probes)\n- [.get_vlan()](IterableNeighbourAttrs::get_vlan)\n- [.get_port()](IterableNeighbourAttrs::get_port)\n- [.get_vni()](IterableNeighbourAttrs::get_vni)\n- [.get_ifindex()](IterableNeighbourAttrs::get_ifindex)\n- [.get_master()](IterableNeighbourAttrs::get_master)\n- [.get_protocol()](IterableNeighbourAttrs::get_protocol)\n- [.get_nh_id()](IterableNeighbourAttrs::get_nh_id)\n- [.get_fdb_ext_attrs()](IterableNeighbourAttrs::get_fdb_ext_attrs)\n- [.get_flags_ext()](IterableNeighbourAttrs::get_flags_ext)\n\n"]
#[derive(Debug)]
pub struct OpGetneighDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetneighDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Ndmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Ndmsg,
    ) -> PushNeighbourAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushNeighbourAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushNeighbourAttrs<&mut Vec<u8>> {
        PushNeighbourAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushNeighbourAttrs<RequestBuf<'r>> {
        PushNeighbourAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Ndmsg, IterableNeighbourAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Ndmsg::len()));
        (
            Ndmsg::new_from_slice(header).unwrap_or_default(),
            IterableNeighbourAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Ndmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Ndmsg {
        let pos = self.request.pos;
        Ndmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Ndmsg {
        let pos = self.request.pos;
        Ndmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetneighDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 30u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Ndmsg, IterableNeighbourAttrs<'buf>);
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
#[doc = "Get or dump neighbour tables\n\nReply attributes:\n- [.get_name()](IterableNdtAttrs::get_name)\n- [.get_thresh1()](IterableNdtAttrs::get_thresh1)\n- [.get_thresh2()](IterableNdtAttrs::get_thresh2)\n- [.get_thresh3()](IterableNdtAttrs::get_thresh3)\n- [.get_config()](IterableNdtAttrs::get_config)\n- [.get_parms()](IterableNdtAttrs::get_parms)\n- [.get_stats()](IterableNdtAttrs::get_stats)\n- [.get_gc_interval()](IterableNdtAttrs::get_gc_interval)\n\n"]
#[derive(Debug)]
pub struct OpGetneightblDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetneightblDump<'r> {
    pub fn new(mut request: Request<'r>, header: &Ndtmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Ndtmsg,
    ) -> PushNdtAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushNdtAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushNdtAttrs<&mut Vec<u8>> {
        PushNdtAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushNdtAttrs<RequestBuf<'r>> {
        PushNdtAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Ndtmsg, IterableNdtAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Ndtmsg::len()));
        (
            Ndtmsg::new_from_slice(header).unwrap_or_default(),
            IterableNdtAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Ndtmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Ndtmsg {
        let pos = self.request.pos;
        Ndtmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Ndtmsg {
        let pos = self.request.pos;
        Ndtmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetneightblDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 66u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Ndtmsg, IterableNdtAttrs<'buf>);
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
#[doc = "Set neighbour tables\n\nRequest attributes:\n- [.push_name()](PushNdtAttrs::push_name)\n- [.push_thresh1()](PushNdtAttrs::push_thresh1)\n- [.push_thresh2()](PushNdtAttrs::push_thresh2)\n- [.push_thresh3()](PushNdtAttrs::push_thresh3)\n- [.nested_parms()](PushNdtAttrs::nested_parms)\n- [.push_gc_interval()](PushNdtAttrs::push_gc_interval)\n\n"]
#[derive(Debug)]
pub struct OpSetneightblDo<'r> {
    request: Request<'r>,
}
impl<'r> OpSetneightblDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Ndtmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Ndtmsg,
    ) -> PushNdtAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushNdtAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushNdtAttrs<&mut Vec<u8>> {
        PushNdtAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushNdtAttrs<RequestBuf<'r>> {
        PushNdtAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Ndtmsg, IterableNdtAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Ndtmsg::len()));
        (
            Ndtmsg::new_from_slice(header).unwrap_or_default(),
            IterableNdtAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Ndtmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Ndtmsg {
        let pos = self.request.pos;
        Ndtmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Ndtmsg {
        let pos = self.request.pos;
        Ndtmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpSetneightblDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 67u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Ndtmsg, IterableNdtAttrs<'buf>);
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
    #[doc = "Add new neighbour entry\n\nRequest attributes:\n- [.push_dst()](PushNeighbourAttrs::push_dst)\n- [.push_lladdr()](PushNeighbourAttrs::push_lladdr)\n- [.push_probes()](PushNeighbourAttrs::push_probes)\n- [.push_vlan()](PushNeighbourAttrs::push_vlan)\n- [.push_port()](PushNeighbourAttrs::push_port)\n- [.push_vni()](PushNeighbourAttrs::push_vni)\n- [.push_ifindex()](PushNeighbourAttrs::push_ifindex)\n- [.push_master()](PushNeighbourAttrs::push_master)\n- [.push_protocol()](PushNeighbourAttrs::push_protocol)\n- [.push_nh_id()](PushNeighbourAttrs::push_nh_id)\n- [.push_fdb_ext_attrs()](PushNeighbourAttrs::push_fdb_ext_attrs)\n- [.push_flags_ext()](PushNeighbourAttrs::push_flags_ext)\n\n"]
    pub fn op_newneigh_do(self, header: &Ndmsg) -> OpNewneighDo<'buf> {
        let mut res = OpNewneighDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-newneigh-do", OpNewneighDo::lookup);
        res
    }
    #[doc = "Remove an existing neighbour entry\n\nRequest attributes:\n- [.push_dst()](PushNeighbourAttrs::push_dst)\n- [.push_ifindex()](PushNeighbourAttrs::push_ifindex)\n\n"]
    pub fn op_delneigh_do(self, header: &Ndmsg) -> OpDelneighDo<'buf> {
        let mut res = OpDelneighDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-delneigh-do", OpDelneighDo::lookup);
        res
    }
    #[doc = "Get or dump neighbour entries\n\nRequest attributes:\n- [.push_ifindex()](PushNeighbourAttrs::push_ifindex)\n- [.push_master()](PushNeighbourAttrs::push_master)\n\nReply attributes:\n- [.get_dst()](IterableNeighbourAttrs::get_dst)\n- [.get_lladdr()](IterableNeighbourAttrs::get_lladdr)\n- [.get_probes()](IterableNeighbourAttrs::get_probes)\n- [.get_vlan()](IterableNeighbourAttrs::get_vlan)\n- [.get_port()](IterableNeighbourAttrs::get_port)\n- [.get_vni()](IterableNeighbourAttrs::get_vni)\n- [.get_ifindex()](IterableNeighbourAttrs::get_ifindex)\n- [.get_master()](IterableNeighbourAttrs::get_master)\n- [.get_protocol()](IterableNeighbourAttrs::get_protocol)\n- [.get_nh_id()](IterableNeighbourAttrs::get_nh_id)\n- [.get_fdb_ext_attrs()](IterableNeighbourAttrs::get_fdb_ext_attrs)\n- [.get_flags_ext()](IterableNeighbourAttrs::get_flags_ext)\n\n"]
    pub fn op_getneigh_dump(self, header: &Ndmsg) -> OpGetneighDump<'buf> {
        let mut res = OpGetneighDump::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getneigh-dump", OpGetneighDump::lookup);
        res
    }
    #[doc = "Get or dump neighbour entries\n\nRequest attributes:\n- [.push_dst()](PushNeighbourAttrs::push_dst)\n\nReply attributes:\n- [.get_dst()](IterableNeighbourAttrs::get_dst)\n- [.get_lladdr()](IterableNeighbourAttrs::get_lladdr)\n- [.get_probes()](IterableNeighbourAttrs::get_probes)\n- [.get_vlan()](IterableNeighbourAttrs::get_vlan)\n- [.get_port()](IterableNeighbourAttrs::get_port)\n- [.get_vni()](IterableNeighbourAttrs::get_vni)\n- [.get_ifindex()](IterableNeighbourAttrs::get_ifindex)\n- [.get_master()](IterableNeighbourAttrs::get_master)\n- [.get_protocol()](IterableNeighbourAttrs::get_protocol)\n- [.get_nh_id()](IterableNeighbourAttrs::get_nh_id)\n- [.get_fdb_ext_attrs()](IterableNeighbourAttrs::get_fdb_ext_attrs)\n- [.get_flags_ext()](IterableNeighbourAttrs::get_flags_ext)\n\n"]
    pub fn op_getneigh_do(self, header: &Ndmsg) -> OpGetneighDo<'buf> {
        let mut res = OpGetneighDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getneigh-do", OpGetneighDo::lookup);
        res
    }
    #[doc = "Get or dump neighbour tables\n\nReply attributes:\n- [.get_name()](IterableNdtAttrs::get_name)\n- [.get_thresh1()](IterableNdtAttrs::get_thresh1)\n- [.get_thresh2()](IterableNdtAttrs::get_thresh2)\n- [.get_thresh3()](IterableNdtAttrs::get_thresh3)\n- [.get_config()](IterableNdtAttrs::get_config)\n- [.get_parms()](IterableNdtAttrs::get_parms)\n- [.get_stats()](IterableNdtAttrs::get_stats)\n- [.get_gc_interval()](IterableNdtAttrs::get_gc_interval)\n\n"]
    pub fn op_getneightbl_dump(self, header: &Ndtmsg) -> OpGetneightblDump<'buf> {
        let mut res = OpGetneightblDump::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-getneightbl-dump",
            OpGetneightblDump::lookup,
        );
        res
    }
    #[doc = "Set neighbour tables\n\nRequest attributes:\n- [.push_name()](PushNdtAttrs::push_name)\n- [.push_thresh1()](PushNdtAttrs::push_thresh1)\n- [.push_thresh2()](PushNdtAttrs::push_thresh2)\n- [.push_thresh3()](PushNdtAttrs::push_thresh3)\n- [.nested_parms()](PushNdtAttrs::nested_parms)\n- [.push_gc_interval()](PushNdtAttrs::push_gc_interval)\n\n"]
    pub fn op_setneightbl_do(self, header: &Ndtmsg) -> OpSetneightblDo<'buf> {
        let mut res = OpSetneightblDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-setneightbl-do", OpSetneightblDo::lookup);
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableNdtAttrs::get_config;
        let _ = IterableNdtAttrs::get_gc_interval;
        let _ = IterableNdtAttrs::get_name;
        let _ = IterableNdtAttrs::get_parms;
        let _ = IterableNdtAttrs::get_stats;
        let _ = IterableNdtAttrs::get_thresh1;
        let _ = IterableNdtAttrs::get_thresh2;
        let _ = IterableNdtAttrs::get_thresh3;
        let _ = IterableNeighbourAttrs::get_dst;
        let _ = IterableNeighbourAttrs::get_fdb_ext_attrs;
        let _ = IterableNeighbourAttrs::get_flags_ext;
        let _ = IterableNeighbourAttrs::get_ifindex;
        let _ = IterableNeighbourAttrs::get_lladdr;
        let _ = IterableNeighbourAttrs::get_master;
        let _ = IterableNeighbourAttrs::get_nh_id;
        let _ = IterableNeighbourAttrs::get_port;
        let _ = IterableNeighbourAttrs::get_probes;
        let _ = IterableNeighbourAttrs::get_protocol;
        let _ = IterableNeighbourAttrs::get_vlan;
        let _ = IterableNeighbourAttrs::get_vni;
        let _ = PushNdtAttrs::<&mut Vec<u8>>::nested_parms;
        let _ = PushNdtAttrs::<&mut Vec<u8>>::push_gc_interval;
        let _ = PushNdtAttrs::<&mut Vec<u8>>::push_name;
        let _ = PushNdtAttrs::<&mut Vec<u8>>::push_thresh1;
        let _ = PushNdtAttrs::<&mut Vec<u8>>::push_thresh2;
        let _ = PushNdtAttrs::<&mut Vec<u8>>::push_thresh3;
        let _ = PushNeighbourAttrs::<&mut Vec<u8>>::push_dst;
        let _ = PushNeighbourAttrs::<&mut Vec<u8>>::push_fdb_ext_attrs;
        let _ = PushNeighbourAttrs::<&mut Vec<u8>>::push_flags_ext;
        let _ = PushNeighbourAttrs::<&mut Vec<u8>>::push_ifindex;
        let _ = PushNeighbourAttrs::<&mut Vec<u8>>::push_lladdr;
        let _ = PushNeighbourAttrs::<&mut Vec<u8>>::push_master;
        let _ = PushNeighbourAttrs::<&mut Vec<u8>>::push_nh_id;
        let _ = PushNeighbourAttrs::<&mut Vec<u8>>::push_port;
        let _ = PushNeighbourAttrs::<&mut Vec<u8>>::push_probes;
        let _ = PushNeighbourAttrs::<&mut Vec<u8>>::push_protocol;
        let _ = PushNeighbourAttrs::<&mut Vec<u8>>::push_vlan;
        let _ = PushNeighbourAttrs::<&mut Vec<u8>>::push_vni;
    }
}
