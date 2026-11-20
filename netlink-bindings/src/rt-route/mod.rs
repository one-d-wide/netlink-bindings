#![doc = "Route configuration over rtnetlink.\n"]
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
    traits::{self, NetlinkRequest, Protocol},
    utils::*,
};
pub const PROTONAME: &str = "rt-route";
pub const PROTONAME_CSTR: &CStr = c"rt-route";
pub const PROTONUM: u16 = 0u16;
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[doc = "Why the kernel deleted a route. New causes may be appended. The value\nspace is family-agnostic, a value is never reinterpreted per address\nfamily.\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DelReason {
    #[doc = "The deletion path does not record a cause.\n"]
    Unspec = 0,
    #[doc = "The RTF_EXPIRES lifetime ran out and the route was garbage collected.\n"]
    Expired = 1,
    #[doc = "A Router Advertisement withdrew the route with a zero lifetime.\n"]
    RaWithdrawn = 2,
}
impl DelReason {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Unspec,
            1 => Self::Expired,
            2 => Self::RaWithdrawn,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RtmFlag {
    #[doc = "Notify user of route change\n"]
    Notify = 1 << 8,
    #[doc = "This route is cloned\n"]
    Cloned = 1 << 9,
    #[doc = "Multipath equalizer: NI\n"]
    Equalize = 1 << 10,
    #[doc = "Prefix addresses\n"]
    Prefix = 1 << 11,
    #[doc = "Set rtm_table to FIB lookup result\n"]
    LookupTable = 1 << 12,
    #[doc = "Return full fib lookup match\n"]
    FibMatch = 1 << 13,
    #[doc = "route is offloaded\n"]
    Offload = 1 << 14,
    #[doc = "route is trapping packets\n"]
    Trap = 1 << 15,
    #[doc = "route is offloaded route offload failed, this value is chosen to avoid\nconflicts with other flags defined in include/uapi/linux/ipv6_route.h\n"]
    OffloadFailed = 1 << 29,
}
impl RtmFlag {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 8 => Self::Notify,
            n if n == 1 << 9 => Self::Cloned,
            n if n == 1 << 10 => Self::Equalize,
            n if n == 1 << 11 => Self::Prefix,
            n if n == 1 << 12 => Self::LookupTable,
            n if n == 1 << 13 => Self::FibMatch,
            n if n == 1 << 14 => Self::Offload,
            n if n == 1 << 15 => Self::Trap,
            n if n == 1 << 29 => Self::OffloadFailed,
            _ => return None,
        })
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Rtmsg {
    pub rtm_family: u8,
    pub rtm_dst_len: u8,
    pub rtm_src_len: u8,
    pub rtm_tos: u8,
    pub rtm_table: u8,
    pub rtm_protocol: u8,
    pub rtm_scope: u8,
    #[doc = "Associated type: [`RtmType`] (enum)"]
    pub rtm_type: u8,
    #[doc = "Associated type: [`RtmFlag`] (enum)"]
    pub rtm_flags: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for Rtmsg {
    fn default() -> Self {
        Self::new()
    }
}
impl Rtmsg {
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
        const _: () = assert!(std::mem::size_of::<Rtmsg>() == 12usize);
        const _: () = assert!(std::mem::align_of::<Rtmsg>() == 4usize);
        12usize
    }
}
impl std::fmt::Debug for Rtmsg {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Rtmsg")
            .field("rtm_family", &self.rtm_family)
            .field("rtm_dst_len", &self.rtm_dst_len)
            .field("rtm_src_len", &self.rtm_src_len)
            .field("rtm_tos", &self.rtm_tos)
            .field("rtm_table", &self.rtm_table)
            .field("rtm_protocol", &self.rtm_protocol)
            .field("rtm_scope", &self.rtm_scope)
            .field(
                "rtm_type",
                &FormatEnum(self.rtm_type.into(), RtmType::from_value),
            )
            .field(
                "rtm_flags",
                &FormatFlags(self.rtm_flags.into(), RtmFlag::from_value),
            )
            .finish()
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct RtaCacheinfo {
    pub rta_clntref: u32,
    pub rta_lastuse: u32,
    pub rta_expires: u32,
    pub rta_error: u32,
    pub rta_used: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for RtaCacheinfo {
    fn default() -> Self {
        Self::new()
    }
}
impl RtaCacheinfo {
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
    pub fn new_from_array(buf: [u8; 20usize]) -> Self {
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
    pub fn as_array(&self) -> &[u8; 20usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 20usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 20usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<RtaCacheinfo>() == 20usize);
        const _: () = assert!(std::mem::align_of::<RtaCacheinfo>() == 4usize);
        20usize
    }
}
#[derive(Clone)]
pub enum RouteAttrs<'a> {
    Dst(std::net::IpAddr),
    Src(std::net::IpAddr),
    Iif(u32),
    Oif(u32),
    Gateway(std::net::IpAddr),
    Priority(u32),
    Prefsrc(std::net::IpAddr),
    Metrics(IterableMetrics<'a>),
    Multipath(&'a [u8]),
    Protoinfo(&'a [u8]),
    Flow(u32),
    Cacheinfo(RtaCacheinfo),
    Session(&'a [u8]),
    MpAlgo(&'a [u8]),
    Table(u32),
    Mark(u32),
    MfcStats(&'a [u8]),
    Via(std::net::IpAddr),
    Newdst(std::net::IpAddr),
    Pref(u8),
    #[doc = "Associated type: [`EncapType`] (enum)"]
    EncapType(u16),
    Encap(EncapData<'a>),
    Expires(u32),
    Pad(&'a [u8]),
    Uid(u32),
    TtlPropagate(u8),
    IpProto(u8),
    Sport(u16),
    Dport(u16),
    NhId(u32),
    Flowlabel(u32),
    #[doc = "Emitted only on RTM_DELROUTE notifications, and only when the deletion\npath records a cause. An absent attribute means either an older kernel\nor a deletion path that does not record its cause, consumers must treat\nabsent and unspec identically. Currently only IPv6 deletion paths record\na cause. The attribute is notification-only, the kernel rejects it in\nrequests.\n\nAssociated type: [`DelReason`] (enum)"]
    DelReason(u32),
}
impl<'a> IterableRouteAttrs<'a> {
    pub fn get_dst(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RouteAttrs::Dst(val)) = attr {
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
    pub fn get_src(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RouteAttrs::Src(val)) = attr {
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
            if let Ok(RouteAttrs::Iif(val)) = attr {
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
            if let Ok(RouteAttrs::Oif(val)) = attr {
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
    pub fn get_gateway(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RouteAttrs::Gateway(val)) = attr {
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
            if let Ok(RouteAttrs::Priority(val)) = attr {
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
    pub fn get_prefsrc(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RouteAttrs::Prefsrc(val)) = attr {
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
            if let Ok(RouteAttrs::Metrics(val)) = attr {
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
            if let Ok(RouteAttrs::Multipath(val)) = attr {
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
            if let Ok(RouteAttrs::Protoinfo(val)) = attr {
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
            if let Ok(RouteAttrs::Flow(val)) = attr {
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
    pub fn get_cacheinfo(&self) -> Result<RtaCacheinfo, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RouteAttrs::Cacheinfo(val)) = attr {
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
            if let Ok(RouteAttrs::Session(val)) = attr {
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
            if let Ok(RouteAttrs::MpAlgo(val)) = attr {
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
            if let Ok(RouteAttrs::Table(val)) = attr {
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
            if let Ok(RouteAttrs::Mark(val)) = attr {
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
            if let Ok(RouteAttrs::MfcStats(val)) = attr {
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
    pub fn get_via(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RouteAttrs::Via(val)) = attr {
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
    pub fn get_newdst(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RouteAttrs::Newdst(val)) = attr {
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
            if let Ok(RouteAttrs::Pref(val)) = attr {
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
    #[doc = "Associated type: [`EncapType`] (enum)"]
    pub fn get_encap_type(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RouteAttrs::EncapType(val)) = attr {
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
    pub fn get_encap(&self) -> Result<EncapData<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RouteAttrs::Encap(val)) = attr {
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
            if let Ok(RouteAttrs::Expires(val)) = attr {
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
            if let Ok(RouteAttrs::Pad(val)) = attr {
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
            if let Ok(RouteAttrs::Uid(val)) = attr {
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
            if let Ok(RouteAttrs::TtlPropagate(val)) = attr {
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
            if let Ok(RouteAttrs::IpProto(val)) = attr {
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
            if let Ok(RouteAttrs::Sport(val)) = attr {
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
            if let Ok(RouteAttrs::Dport(val)) = attr {
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
            if let Ok(RouteAttrs::NhId(val)) = attr {
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
            if let Ok(RouteAttrs::Flowlabel(val)) = attr {
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
    #[doc = "Emitted only on RTM_DELROUTE notifications, and only when the deletion\npath records a cause. An absent attribute means either an older kernel\nor a deletion path that does not record its cause, consumers must treat\nabsent and unspec identically. Currently only IPv6 deletion paths record\na cause. The attribute is notification-only, the kernel rejects it in\nrequests.\n\nAssociated type: [`DelReason`] (enum)"]
    pub fn get_del_reason(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RouteAttrs::DelReason(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RouteAttrs",
            "DelReason",
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
impl RouteAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableRouteAttrs<'a> {
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
            32u16 => "DelReason",
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
    selector_encap_type: Option<u16>,
}
impl<'a> IterableRouteAttrs<'a> {
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
impl<'a> Iterator for IterableRouteAttrs<'a> {
    type Item = Result<RouteAttrs<'a>, ErrorContext>;
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
                1u16 => RouteAttrs::Dst({
                    let res = parse_ip(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => RouteAttrs::Src({
                    let res = parse_ip(next);
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
                    let res = parse_ip(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => RouteAttrs::Priority({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => RouteAttrs::Prefsrc({
                    let res = parse_ip(next);
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
                    let res = Some(RtaCacheinfo::new_from_zeroed(next));
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
                    let res = parse_ip(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => RouteAttrs::Newdst({
                    let res = parse_ip(next);
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
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                29u16 => RouteAttrs::Dport({
                    let res = parse_be_u16(next);
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
                32u16 => RouteAttrs::DelReason({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            if let RouteAttrs::EncapType(sel) = &res {
                self.selector_encap_type = Some(*sel);
            }
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
        let mut iter = IterableRouteAttrs::with_loc(&[], self.orig_loc);
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
                RouteAttrs::Dst(val) => fmt.field("Dst", &val),
                RouteAttrs::Src(val) => fmt.field("Src", &val),
                RouteAttrs::Iif(val) => fmt.field("Iif", &val),
                RouteAttrs::Oif(val) => fmt.field("Oif", &val),
                RouteAttrs::Gateway(val) => fmt.field("Gateway", &val),
                RouteAttrs::Priority(val) => fmt.field("Priority", &val),
                RouteAttrs::Prefsrc(val) => fmt.field("Prefsrc", &val),
                RouteAttrs::Metrics(val) => fmt.field("Metrics", &val),
                RouteAttrs::Multipath(val) => fmt.field("Multipath", &FormatHexdump(val)),
                RouteAttrs::Protoinfo(val) => fmt.field("Protoinfo", &FormatHexdump(val)),
                RouteAttrs::Flow(val) => fmt.field("Flow", &val),
                RouteAttrs::Cacheinfo(val) => fmt.field("Cacheinfo", &val),
                RouteAttrs::Session(val) => fmt.field("Session", &FormatHexdump(val)),
                RouteAttrs::MpAlgo(val) => fmt.field("MpAlgo", &FormatHexdump(val)),
                RouteAttrs::Table(val) => fmt.field("Table", &val),
                RouteAttrs::Mark(val) => fmt.field("Mark", &val),
                RouteAttrs::MfcStats(val) => fmt.field("MfcStats", &FormatHexdump(val)),
                RouteAttrs::Via(val) => fmt.field("Via", &val),
                RouteAttrs::Newdst(val) => fmt.field("Newdst", &val),
                RouteAttrs::Pref(val) => fmt.field("Pref", &val),
                RouteAttrs::EncapType(val) => {
                    fmt.field("EncapType", &FormatEnum(val.into(), EncapType::from_value))
                }
                RouteAttrs::Encap(val) => fmt.field("Encap", &val),
                RouteAttrs::Expires(val) => fmt.field("Expires", &val),
                RouteAttrs::Pad(val) => fmt.field("Pad", &FormatHexdump(val)),
                RouteAttrs::Uid(val) => fmt.field("Uid", &val),
                RouteAttrs::TtlPropagate(val) => fmt.field("TtlPropagate", &val),
                RouteAttrs::IpProto(val) => fmt.field("IpProto", &val),
                RouteAttrs::Sport(val) => fmt.field("Sport", &val),
                RouteAttrs::Dport(val) => fmt.field("Dport", &val),
                RouteAttrs::NhId(val) => fmt.field("NhId", &val),
                RouteAttrs::Flowlabel(val) => fmt.field("Flowlabel", &val),
                RouteAttrs::DelReason(val) => {
                    fmt.field("DelReason", &FormatEnum(val.into(), DelReason::from_value))
                }
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
        if missing_type.is_some() && cur == offset {
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
                RouteAttrs::DelReason(val) => {
                    if last_off == offset {
                        stack.push(("DelReason", last_off));
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
            if let Ok(Metrics::Lock(val)) = attr {
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
            if let Ok(Metrics::Mtu(val)) = attr {
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
            if let Ok(Metrics::Window(val)) = attr {
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
            if let Ok(Metrics::Rtt(val)) = attr {
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
            if let Ok(Metrics::Rttvar(val)) = attr {
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
            if let Ok(Metrics::Ssthresh(val)) = attr {
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
            if let Ok(Metrics::Cwnd(val)) = attr {
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
            if let Ok(Metrics::Advmss(val)) = attr {
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
            if let Ok(Metrics::Reordering(val)) = attr {
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
            if let Ok(Metrics::Hoplimit(val)) = attr {
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
            if let Ok(Metrics::Initcwnd(val)) = attr {
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
            if let Ok(Metrics::Features(val)) = attr {
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
            if let Ok(Metrics::RtoMin(val)) = attr {
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
            if let Ok(Metrics::Initrwnd(val)) = attr {
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
            if let Ok(Metrics::Quickack(val)) = attr {
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
            if let Ok(Metrics::CcAlgo(val)) = attr {
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
            if let Ok(Metrics::FastopenNoCookie(val)) = attr {
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
impl Metrics<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableMetrics<'a> {
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
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
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
        let mut iter = IterableMetrics::with_loc(&[], self.orig_loc);
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
        if missing_type.is_some() && cur == offset {
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
pub struct PushRouteAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushRouteAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushRouteAttrs<Prev> {
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
    pub fn push_iif(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_oif(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_gateway(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_vec_mut(), 5u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_vec_mut(), value);
        self
    }
    pub fn push_priority(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_prefsrc(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_vec_mut(), 7u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_vec_mut(), value);
        self
    }
    pub fn nested_metrics(mut self) -> PushMetrics<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 8u16);
        PushMetrics {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_multipath(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 9u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_protoinfo(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 10u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_flow(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 11u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_cacheinfo(mut self, value: RtaCacheinfo) -> Self {
        push_header(self.as_vec_mut(), 12u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_session(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 13u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_mp_algo(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 14u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_table(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 15u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mark(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 16u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mfc_stats(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 17u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_via(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_vec_mut(), 18u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_vec_mut(), value);
        self
    }
    pub fn push_newdst(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_vec_mut(), 19u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_vec_mut(), value);
        self
    }
    pub fn push_pref(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 20u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`EncapType`] (enum)"]
    pub fn push_encap_type(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 21u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Selector attribute `encap-type` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_encap_mpls(mut self) -> PushMplsIptunnel<PushDummy<Prev>> {
        self = self.push_encap_type(EncapType::Mpls as u16);
        let new_header_offset = push_nested_header(self.as_vec_mut(), 22u16);
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
        let new_header_offset = push_nested_header(self.as_vec_mut(), 22u16);
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
        let new_header_offset = push_nested_header(self.as_vec_mut(), 22u16);
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
        let new_header_offset = push_nested_header(self.as_vec_mut(), 22u16);
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
        let new_header_offset = push_nested_header(self.as_vec_mut(), 22u16);
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
        let new_header_offset = push_nested_header(self.as_vec_mut(), 22u16);
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
        let new_header_offset = push_nested_header(self.as_vec_mut(), 22u16);
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
        let new_header_offset = push_nested_header(self.as_vec_mut(), 22u16);
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
        let new_header_offset = push_nested_header(self.as_vec_mut(), 22u16);
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
        let new_header_offset = push_nested_header(self.as_vec_mut(), 22u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushLwtXfrm {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    pub fn push_expires(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 23u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 24u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_uid(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 25u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ttl_propagate(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 26u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ip_proto(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 27u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sport(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 28u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_dport(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 29u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_nh_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 30u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flowlabel(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 31u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "Emitted only on RTM_DELROUTE notifications, and only when the deletion\npath records a cause. An absent attribute means either an older kernel\nor a deletion path that does not record its cause, consumers must treat\nabsent and unspec identically. Currently only IPv6 deletion paths record\na cause. The attribute is notification-only, the kernel rejects it in\nrequests.\n\nAssociated type: [`DelReason`] (enum)"]
    pub fn push_del_reason(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 32u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushRouteAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushMetrics<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushMetrics<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushMetrics<Prev> {
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
    pub fn push_lock(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mtu(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_window(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_rtt(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_rttvar(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ssthresh(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_cwnd(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_advmss(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_reordering(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_hoplimit(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_initcwnd(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 11u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_features(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 12u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_rto_min(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 13u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_initrwnd(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 14u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_quickack(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 15u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_cc_algo(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            16u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_cc_algo_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 16u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_fastopen_no_cookie(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 17u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushMetrics<Prev> {
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
#[doc = "Dump route information.\n\nReply attributes:\n- [.get_dst()](IterableRouteAttrs::get_dst)\n- [.get_src()](IterableRouteAttrs::get_src)\n- [.get_iif()](IterableRouteAttrs::get_iif)\n- [.get_oif()](IterableRouteAttrs::get_oif)\n- [.get_gateway()](IterableRouteAttrs::get_gateway)\n- [.get_priority()](IterableRouteAttrs::get_priority)\n- [.get_prefsrc()](IterableRouteAttrs::get_prefsrc)\n- [.get_metrics()](IterableRouteAttrs::get_metrics)\n- [.get_multipath()](IterableRouteAttrs::get_multipath)\n- [.get_flow()](IterableRouteAttrs::get_flow)\n- [.get_cacheinfo()](IterableRouteAttrs::get_cacheinfo)\n- [.get_table()](IterableRouteAttrs::get_table)\n- [.get_mark()](IterableRouteAttrs::get_mark)\n- [.get_mfc_stats()](IterableRouteAttrs::get_mfc_stats)\n- [.get_via()](IterableRouteAttrs::get_via)\n- [.get_newdst()](IterableRouteAttrs::get_newdst)\n- [.get_pref()](IterableRouteAttrs::get_pref)\n- [.get_encap_type()](IterableRouteAttrs::get_encap_type)\n- [.get_encap()](IterableRouteAttrs::get_encap)\n- [.get_expires()](IterableRouteAttrs::get_expires)\n- [.get_pad()](IterableRouteAttrs::get_pad)\n- [.get_uid()](IterableRouteAttrs::get_uid)\n- [.get_ttl_propagate()](IterableRouteAttrs::get_ttl_propagate)\n- [.get_ip_proto()](IterableRouteAttrs::get_ip_proto)\n- [.get_sport()](IterableRouteAttrs::get_sport)\n- [.get_dport()](IterableRouteAttrs::get_dport)\n- [.get_nh_id()](IterableRouteAttrs::get_nh_id)\n- [.get_flowlabel()](IterableRouteAttrs::get_flowlabel)\n- [.get_del_reason()](IterableRouteAttrs::get_del_reason)\n\n"]
#[derive(Debug)]
pub struct OpGetrouteDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetrouteDump<'r> {
    pub fn new(mut request: Request<'r>, header: &Rtmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Rtmsg,
    ) -> PushRouteAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushRouteAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushRouteAttrs<&mut Vec<u8>> {
        PushRouteAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushRouteAttrs<RequestBuf<'r>> {
        PushRouteAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Rtmsg, IterableRouteAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Rtmsg::len()));
        (
            Rtmsg::new_from_slice(header).unwrap_or_default(),
            IterableRouteAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Rtmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Rtmsg {
        let pos = self.request.pos;
        Rtmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Rtmsg {
        let pos = self.request.pos;
        Rtmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetrouteDump<'_> {
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
    type ReplyType<'buf> = (Rtmsg, IterableRouteAttrs<'buf>);
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
#[doc = "Dump route information.\n\nRequest attributes:\n- [.push_dst()](PushRouteAttrs::push_dst)\n- [.push_src()](PushRouteAttrs::push_src)\n- [.push_iif()](PushRouteAttrs::push_iif)\n- [.push_oif()](PushRouteAttrs::push_oif)\n- [.push_mark()](PushRouteAttrs::push_mark)\n- [.push_uid()](PushRouteAttrs::push_uid)\n- [.push_ip_proto()](PushRouteAttrs::push_ip_proto)\n- [.push_sport()](PushRouteAttrs::push_sport)\n- [.push_dport()](PushRouteAttrs::push_dport)\n- [.push_flowlabel()](PushRouteAttrs::push_flowlabel)\n\nReply attributes:\n- [.get_dst()](IterableRouteAttrs::get_dst)\n- [.get_src()](IterableRouteAttrs::get_src)\n- [.get_iif()](IterableRouteAttrs::get_iif)\n- [.get_oif()](IterableRouteAttrs::get_oif)\n- [.get_gateway()](IterableRouteAttrs::get_gateway)\n- [.get_priority()](IterableRouteAttrs::get_priority)\n- [.get_prefsrc()](IterableRouteAttrs::get_prefsrc)\n- [.get_metrics()](IterableRouteAttrs::get_metrics)\n- [.get_multipath()](IterableRouteAttrs::get_multipath)\n- [.get_flow()](IterableRouteAttrs::get_flow)\n- [.get_cacheinfo()](IterableRouteAttrs::get_cacheinfo)\n- [.get_table()](IterableRouteAttrs::get_table)\n- [.get_mark()](IterableRouteAttrs::get_mark)\n- [.get_mfc_stats()](IterableRouteAttrs::get_mfc_stats)\n- [.get_via()](IterableRouteAttrs::get_via)\n- [.get_newdst()](IterableRouteAttrs::get_newdst)\n- [.get_pref()](IterableRouteAttrs::get_pref)\n- [.get_encap_type()](IterableRouteAttrs::get_encap_type)\n- [.get_encap()](IterableRouteAttrs::get_encap)\n- [.get_expires()](IterableRouteAttrs::get_expires)\n- [.get_pad()](IterableRouteAttrs::get_pad)\n- [.get_uid()](IterableRouteAttrs::get_uid)\n- [.get_ttl_propagate()](IterableRouteAttrs::get_ttl_propagate)\n- [.get_ip_proto()](IterableRouteAttrs::get_ip_proto)\n- [.get_sport()](IterableRouteAttrs::get_sport)\n- [.get_dport()](IterableRouteAttrs::get_dport)\n- [.get_nh_id()](IterableRouteAttrs::get_nh_id)\n- [.get_flowlabel()](IterableRouteAttrs::get_flowlabel)\n- [.get_del_reason()](IterableRouteAttrs::get_del_reason)\n\n"]
#[derive(Debug)]
pub struct OpGetrouteDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetrouteDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Rtmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Rtmsg,
    ) -> PushRouteAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushRouteAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushRouteAttrs<&mut Vec<u8>> {
        PushRouteAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushRouteAttrs<RequestBuf<'r>> {
        PushRouteAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Rtmsg, IterableRouteAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Rtmsg::len()));
        (
            Rtmsg::new_from_slice(header).unwrap_or_default(),
            IterableRouteAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Rtmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Rtmsg {
        let pos = self.request.pos;
        Rtmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Rtmsg {
        let pos = self.request.pos;
        Rtmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetrouteDo<'_> {
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
    type ReplyType<'buf> = (Rtmsg, IterableRouteAttrs<'buf>);
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
#[doc = "Create a new route\n\nRequest attributes:\n- [.push_dst()](PushRouteAttrs::push_dst)\n- [.push_src()](PushRouteAttrs::push_src)\n- [.push_iif()](PushRouteAttrs::push_iif)\n- [.push_oif()](PushRouteAttrs::push_oif)\n- [.push_gateway()](PushRouteAttrs::push_gateway)\n- [.push_priority()](PushRouteAttrs::push_priority)\n- [.push_prefsrc()](PushRouteAttrs::push_prefsrc)\n- [.nested_metrics()](PushRouteAttrs::nested_metrics)\n- [.push_multipath()](PushRouteAttrs::push_multipath)\n- [.push_flow()](PushRouteAttrs::push_flow)\n- [.push_cacheinfo()](PushRouteAttrs::push_cacheinfo)\n- [.push_table()](PushRouteAttrs::push_table)\n- [.push_mark()](PushRouteAttrs::push_mark)\n- [.push_mfc_stats()](PushRouteAttrs::push_mfc_stats)\n- [.push_via()](PushRouteAttrs::push_via)\n- [.push_newdst()](PushRouteAttrs::push_newdst)\n- [.push_pref()](PushRouteAttrs::push_pref)\n- [.push_encap_type()](PushRouteAttrs::push_encap_type)\n- [.nested_encap_mpls()](PushRouteAttrs::nested_encap_mpls)\n- [.nested_encap_ip()](PushRouteAttrs::nested_encap_ip)\n- [.nested_encap_ila()](PushRouteAttrs::nested_encap_ila)\n- [.nested_encap_ip6()](PushRouteAttrs::nested_encap_ip6)\n- [.nested_encap_seg6()](PushRouteAttrs::nested_encap_seg6)\n- [.nested_encap_bpf()](PushRouteAttrs::nested_encap_bpf)\n- [.nested_encap_seg6_local()](PushRouteAttrs::nested_encap_seg6_local)\n- [.nested_encap_rpl()](PushRouteAttrs::nested_encap_rpl)\n- [.nested_encap_ioam6()](PushRouteAttrs::nested_encap_ioam6)\n- [.nested_encap_xfrm()](PushRouteAttrs::nested_encap_xfrm)\n- [.push_expires()](PushRouteAttrs::push_expires)\n- [.push_pad()](PushRouteAttrs::push_pad)\n- [.push_uid()](PushRouteAttrs::push_uid)\n- [.push_ttl_propagate()](PushRouteAttrs::push_ttl_propagate)\n- [.push_ip_proto()](PushRouteAttrs::push_ip_proto)\n- [.push_sport()](PushRouteAttrs::push_sport)\n- [.push_dport()](PushRouteAttrs::push_dport)\n- [.push_nh_id()](PushRouteAttrs::push_nh_id)\n- [.push_flowlabel()](PushRouteAttrs::push_flowlabel)\n\n"]
#[derive(Debug)]
pub struct OpNewrouteDo<'r> {
    request: Request<'r>,
}
impl<'r> OpNewrouteDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Rtmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Rtmsg,
    ) -> PushRouteAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushRouteAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushRouteAttrs<&mut Vec<u8>> {
        PushRouteAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushRouteAttrs<RequestBuf<'r>> {
        PushRouteAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Rtmsg, IterableRouteAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Rtmsg::len()));
        (
            Rtmsg::new_from_slice(header).unwrap_or_default(),
            IterableRouteAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Rtmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Rtmsg {
        let pos = self.request.pos;
        Rtmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Rtmsg {
        let pos = self.request.pos;
        Rtmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpNewrouteDo<'_> {
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
    type ReplyType<'buf> = (Rtmsg, IterableRouteAttrs<'buf>);
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
#[doc = "Delete an existing route\n\nRequest attributes:\n- [.push_dst()](PushRouteAttrs::push_dst)\n- [.push_src()](PushRouteAttrs::push_src)\n- [.push_iif()](PushRouteAttrs::push_iif)\n- [.push_oif()](PushRouteAttrs::push_oif)\n- [.push_gateway()](PushRouteAttrs::push_gateway)\n- [.push_priority()](PushRouteAttrs::push_priority)\n- [.push_prefsrc()](PushRouteAttrs::push_prefsrc)\n- [.nested_metrics()](PushRouteAttrs::nested_metrics)\n- [.push_multipath()](PushRouteAttrs::push_multipath)\n- [.push_flow()](PushRouteAttrs::push_flow)\n- [.push_cacheinfo()](PushRouteAttrs::push_cacheinfo)\n- [.push_table()](PushRouteAttrs::push_table)\n- [.push_mark()](PushRouteAttrs::push_mark)\n- [.push_mfc_stats()](PushRouteAttrs::push_mfc_stats)\n- [.push_via()](PushRouteAttrs::push_via)\n- [.push_newdst()](PushRouteAttrs::push_newdst)\n- [.push_pref()](PushRouteAttrs::push_pref)\n- [.push_encap_type()](PushRouteAttrs::push_encap_type)\n- [.nested_encap_mpls()](PushRouteAttrs::nested_encap_mpls)\n- [.nested_encap_ip()](PushRouteAttrs::nested_encap_ip)\n- [.nested_encap_ila()](PushRouteAttrs::nested_encap_ila)\n- [.nested_encap_ip6()](PushRouteAttrs::nested_encap_ip6)\n- [.nested_encap_seg6()](PushRouteAttrs::nested_encap_seg6)\n- [.nested_encap_bpf()](PushRouteAttrs::nested_encap_bpf)\n- [.nested_encap_seg6_local()](PushRouteAttrs::nested_encap_seg6_local)\n- [.nested_encap_rpl()](PushRouteAttrs::nested_encap_rpl)\n- [.nested_encap_ioam6()](PushRouteAttrs::nested_encap_ioam6)\n- [.nested_encap_xfrm()](PushRouteAttrs::nested_encap_xfrm)\n- [.push_expires()](PushRouteAttrs::push_expires)\n- [.push_pad()](PushRouteAttrs::push_pad)\n- [.push_uid()](PushRouteAttrs::push_uid)\n- [.push_ttl_propagate()](PushRouteAttrs::push_ttl_propagate)\n- [.push_ip_proto()](PushRouteAttrs::push_ip_proto)\n- [.push_sport()](PushRouteAttrs::push_sport)\n- [.push_dport()](PushRouteAttrs::push_dport)\n- [.push_nh_id()](PushRouteAttrs::push_nh_id)\n- [.push_flowlabel()](PushRouteAttrs::push_flowlabel)\n\n"]
#[derive(Debug)]
pub struct OpDelrouteDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDelrouteDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Rtmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Rtmsg,
    ) -> PushRouteAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushRouteAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushRouteAttrs<&mut Vec<u8>> {
        PushRouteAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushRouteAttrs<RequestBuf<'r>> {
        PushRouteAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Rtmsg, IterableRouteAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Rtmsg::len()));
        (
            Rtmsg::new_from_slice(header).unwrap_or_default(),
            IterableRouteAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Rtmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Rtmsg {
        let pos = self.request.pos;
        Rtmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Rtmsg {
        let pos = self.request.pos;
        Rtmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpDelrouteDo<'_> {
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
    type ReplyType<'buf> = (Rtmsg, IterableRouteAttrs<'buf>);
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
impl<'a> std::ops::Deref for ChainedFinal<'a> {
    type Target = Chained<'a>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
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
    pub fn with_request<R: NetlinkRequest + ?Sized>(&mut self, request: &R) -> &mut Self {
        self.update_header();
        self.last_header_offset = self.buf().len();
        self.buf_mut().extend_from_slice(Nlmsghdr::new().as_slice());
        self.buf_mut().extend_from_slice(request.payload());
        self.last_kind = Some(RequestInfo {
            protocol: request.protocol(),
            flags: request.flags(),
            name: std::any::type_name::<R>(),
            lookup: R::lookup,
        });
        self
    }
    pub fn with_chained(&mut self, chain: &Self) -> &mut Self {
        self.update_header();
        let chain_off = self.buf().len();
        let chain_seq = self.first_seq.wrapping_add(self.lookups.len() as u32);
        self.buf_mut().extend_from_slice(chain.buf());
        for (i, (header, _)) in
            IterableMessagesMut::new(&mut self.buf_mut()[chain_off..]).enumerate()
        {
            header.seq = chain_seq.wrapping_add(i as u32);
        }
        self.lookups.extend_from_slice(&chain.lookups);
        self.last_header_offset = chain_off + chain.last_header_offset;
        self.last_kind = chain.last_kind.clone();
        self
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
        }) = self.last_kind.take()
        else {
            if !self.buf().is_empty() && self.buf().len() != self.last_header_offset {
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
        self.last_header_offset = buf.len();
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
    #[doc = "Dump route information.\n\nReply attributes:\n- [.get_dst()](IterableRouteAttrs::get_dst)\n- [.get_src()](IterableRouteAttrs::get_src)\n- [.get_iif()](IterableRouteAttrs::get_iif)\n- [.get_oif()](IterableRouteAttrs::get_oif)\n- [.get_gateway()](IterableRouteAttrs::get_gateway)\n- [.get_priority()](IterableRouteAttrs::get_priority)\n- [.get_prefsrc()](IterableRouteAttrs::get_prefsrc)\n- [.get_metrics()](IterableRouteAttrs::get_metrics)\n- [.get_multipath()](IterableRouteAttrs::get_multipath)\n- [.get_flow()](IterableRouteAttrs::get_flow)\n- [.get_cacheinfo()](IterableRouteAttrs::get_cacheinfo)\n- [.get_table()](IterableRouteAttrs::get_table)\n- [.get_mark()](IterableRouteAttrs::get_mark)\n- [.get_mfc_stats()](IterableRouteAttrs::get_mfc_stats)\n- [.get_via()](IterableRouteAttrs::get_via)\n- [.get_newdst()](IterableRouteAttrs::get_newdst)\n- [.get_pref()](IterableRouteAttrs::get_pref)\n- [.get_encap_type()](IterableRouteAttrs::get_encap_type)\n- [.get_encap()](IterableRouteAttrs::get_encap)\n- [.get_expires()](IterableRouteAttrs::get_expires)\n- [.get_pad()](IterableRouteAttrs::get_pad)\n- [.get_uid()](IterableRouteAttrs::get_uid)\n- [.get_ttl_propagate()](IterableRouteAttrs::get_ttl_propagate)\n- [.get_ip_proto()](IterableRouteAttrs::get_ip_proto)\n- [.get_sport()](IterableRouteAttrs::get_sport)\n- [.get_dport()](IterableRouteAttrs::get_dport)\n- [.get_nh_id()](IterableRouteAttrs::get_nh_id)\n- [.get_flowlabel()](IterableRouteAttrs::get_flowlabel)\n- [.get_del_reason()](IterableRouteAttrs::get_del_reason)\n\n"]
    pub fn op_getroute_dump(self, header: &Rtmsg) -> OpGetrouteDump<'buf> {
        let mut res = OpGetrouteDump::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getroute-dump", OpGetrouteDump::lookup);
        res
    }
    #[doc = "Dump route information.\n\nRequest attributes:\n- [.push_dst()](PushRouteAttrs::push_dst)\n- [.push_src()](PushRouteAttrs::push_src)\n- [.push_iif()](PushRouteAttrs::push_iif)\n- [.push_oif()](PushRouteAttrs::push_oif)\n- [.push_mark()](PushRouteAttrs::push_mark)\n- [.push_uid()](PushRouteAttrs::push_uid)\n- [.push_ip_proto()](PushRouteAttrs::push_ip_proto)\n- [.push_sport()](PushRouteAttrs::push_sport)\n- [.push_dport()](PushRouteAttrs::push_dport)\n- [.push_flowlabel()](PushRouteAttrs::push_flowlabel)\n\nReply attributes:\n- [.get_dst()](IterableRouteAttrs::get_dst)\n- [.get_src()](IterableRouteAttrs::get_src)\n- [.get_iif()](IterableRouteAttrs::get_iif)\n- [.get_oif()](IterableRouteAttrs::get_oif)\n- [.get_gateway()](IterableRouteAttrs::get_gateway)\n- [.get_priority()](IterableRouteAttrs::get_priority)\n- [.get_prefsrc()](IterableRouteAttrs::get_prefsrc)\n- [.get_metrics()](IterableRouteAttrs::get_metrics)\n- [.get_multipath()](IterableRouteAttrs::get_multipath)\n- [.get_flow()](IterableRouteAttrs::get_flow)\n- [.get_cacheinfo()](IterableRouteAttrs::get_cacheinfo)\n- [.get_table()](IterableRouteAttrs::get_table)\n- [.get_mark()](IterableRouteAttrs::get_mark)\n- [.get_mfc_stats()](IterableRouteAttrs::get_mfc_stats)\n- [.get_via()](IterableRouteAttrs::get_via)\n- [.get_newdst()](IterableRouteAttrs::get_newdst)\n- [.get_pref()](IterableRouteAttrs::get_pref)\n- [.get_encap_type()](IterableRouteAttrs::get_encap_type)\n- [.get_encap()](IterableRouteAttrs::get_encap)\n- [.get_expires()](IterableRouteAttrs::get_expires)\n- [.get_pad()](IterableRouteAttrs::get_pad)\n- [.get_uid()](IterableRouteAttrs::get_uid)\n- [.get_ttl_propagate()](IterableRouteAttrs::get_ttl_propagate)\n- [.get_ip_proto()](IterableRouteAttrs::get_ip_proto)\n- [.get_sport()](IterableRouteAttrs::get_sport)\n- [.get_dport()](IterableRouteAttrs::get_dport)\n- [.get_nh_id()](IterableRouteAttrs::get_nh_id)\n- [.get_flowlabel()](IterableRouteAttrs::get_flowlabel)\n- [.get_del_reason()](IterableRouteAttrs::get_del_reason)\n\n"]
    pub fn op_getroute_do(self, header: &Rtmsg) -> OpGetrouteDo<'buf> {
        let mut res = OpGetrouteDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getroute-do", OpGetrouteDo::lookup);
        res
    }
    #[doc = "Create a new route\n\nRequest attributes:\n- [.push_dst()](PushRouteAttrs::push_dst)\n- [.push_src()](PushRouteAttrs::push_src)\n- [.push_iif()](PushRouteAttrs::push_iif)\n- [.push_oif()](PushRouteAttrs::push_oif)\n- [.push_gateway()](PushRouteAttrs::push_gateway)\n- [.push_priority()](PushRouteAttrs::push_priority)\n- [.push_prefsrc()](PushRouteAttrs::push_prefsrc)\n- [.nested_metrics()](PushRouteAttrs::nested_metrics)\n- [.push_multipath()](PushRouteAttrs::push_multipath)\n- [.push_flow()](PushRouteAttrs::push_flow)\n- [.push_cacheinfo()](PushRouteAttrs::push_cacheinfo)\n- [.push_table()](PushRouteAttrs::push_table)\n- [.push_mark()](PushRouteAttrs::push_mark)\n- [.push_mfc_stats()](PushRouteAttrs::push_mfc_stats)\n- [.push_via()](PushRouteAttrs::push_via)\n- [.push_newdst()](PushRouteAttrs::push_newdst)\n- [.push_pref()](PushRouteAttrs::push_pref)\n- [.push_encap_type()](PushRouteAttrs::push_encap_type)\n- [.nested_encap_mpls()](PushRouteAttrs::nested_encap_mpls)\n- [.nested_encap_ip()](PushRouteAttrs::nested_encap_ip)\n- [.nested_encap_ila()](PushRouteAttrs::nested_encap_ila)\n- [.nested_encap_ip6()](PushRouteAttrs::nested_encap_ip6)\n- [.nested_encap_seg6()](PushRouteAttrs::nested_encap_seg6)\n- [.nested_encap_bpf()](PushRouteAttrs::nested_encap_bpf)\n- [.nested_encap_seg6_local()](PushRouteAttrs::nested_encap_seg6_local)\n- [.nested_encap_rpl()](PushRouteAttrs::nested_encap_rpl)\n- [.nested_encap_ioam6()](PushRouteAttrs::nested_encap_ioam6)\n- [.nested_encap_xfrm()](PushRouteAttrs::nested_encap_xfrm)\n- [.push_expires()](PushRouteAttrs::push_expires)\n- [.push_pad()](PushRouteAttrs::push_pad)\n- [.push_uid()](PushRouteAttrs::push_uid)\n- [.push_ttl_propagate()](PushRouteAttrs::push_ttl_propagate)\n- [.push_ip_proto()](PushRouteAttrs::push_ip_proto)\n- [.push_sport()](PushRouteAttrs::push_sport)\n- [.push_dport()](PushRouteAttrs::push_dport)\n- [.push_nh_id()](PushRouteAttrs::push_nh_id)\n- [.push_flowlabel()](PushRouteAttrs::push_flowlabel)\n\n"]
    pub fn op_newroute_do(self, header: &Rtmsg) -> OpNewrouteDo<'buf> {
        let mut res = OpNewrouteDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-newroute-do", OpNewrouteDo::lookup);
        res
    }
    #[doc = "Delete an existing route\n\nRequest attributes:\n- [.push_dst()](PushRouteAttrs::push_dst)\n- [.push_src()](PushRouteAttrs::push_src)\n- [.push_iif()](PushRouteAttrs::push_iif)\n- [.push_oif()](PushRouteAttrs::push_oif)\n- [.push_gateway()](PushRouteAttrs::push_gateway)\n- [.push_priority()](PushRouteAttrs::push_priority)\n- [.push_prefsrc()](PushRouteAttrs::push_prefsrc)\n- [.nested_metrics()](PushRouteAttrs::nested_metrics)\n- [.push_multipath()](PushRouteAttrs::push_multipath)\n- [.push_flow()](PushRouteAttrs::push_flow)\n- [.push_cacheinfo()](PushRouteAttrs::push_cacheinfo)\n- [.push_table()](PushRouteAttrs::push_table)\n- [.push_mark()](PushRouteAttrs::push_mark)\n- [.push_mfc_stats()](PushRouteAttrs::push_mfc_stats)\n- [.push_via()](PushRouteAttrs::push_via)\n- [.push_newdst()](PushRouteAttrs::push_newdst)\n- [.push_pref()](PushRouteAttrs::push_pref)\n- [.push_encap_type()](PushRouteAttrs::push_encap_type)\n- [.nested_encap_mpls()](PushRouteAttrs::nested_encap_mpls)\n- [.nested_encap_ip()](PushRouteAttrs::nested_encap_ip)\n- [.nested_encap_ila()](PushRouteAttrs::nested_encap_ila)\n- [.nested_encap_ip6()](PushRouteAttrs::nested_encap_ip6)\n- [.nested_encap_seg6()](PushRouteAttrs::nested_encap_seg6)\n- [.nested_encap_bpf()](PushRouteAttrs::nested_encap_bpf)\n- [.nested_encap_seg6_local()](PushRouteAttrs::nested_encap_seg6_local)\n- [.nested_encap_rpl()](PushRouteAttrs::nested_encap_rpl)\n- [.nested_encap_ioam6()](PushRouteAttrs::nested_encap_ioam6)\n- [.nested_encap_xfrm()](PushRouteAttrs::nested_encap_xfrm)\n- [.push_expires()](PushRouteAttrs::push_expires)\n- [.push_pad()](PushRouteAttrs::push_pad)\n- [.push_uid()](PushRouteAttrs::push_uid)\n- [.push_ttl_propagate()](PushRouteAttrs::push_ttl_propagate)\n- [.push_ip_proto()](PushRouteAttrs::push_ip_proto)\n- [.push_sport()](PushRouteAttrs::push_sport)\n- [.push_dport()](PushRouteAttrs::push_dport)\n- [.push_nh_id()](PushRouteAttrs::push_nh_id)\n- [.push_flowlabel()](PushRouteAttrs::push_flowlabel)\n\n"]
    pub fn op_delroute_do(self, header: &Rtmsg) -> OpDelrouteDo<'buf> {
        let mut res = OpDelrouteDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-delroute-do", OpDelrouteDo::lookup);
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableRouteAttrs::get_cacheinfo;
        let _ = IterableRouteAttrs::get_del_reason;
        let _ = IterableRouteAttrs::get_dport;
        let _ = IterableRouteAttrs::get_dst;
        let _ = IterableRouteAttrs::get_encap;
        let _ = IterableRouteAttrs::get_encap_type;
        let _ = IterableRouteAttrs::get_expires;
        let _ = IterableRouteAttrs::get_flow;
        let _ = IterableRouteAttrs::get_flowlabel;
        let _ = IterableRouteAttrs::get_gateway;
        let _ = IterableRouteAttrs::get_iif;
        let _ = IterableRouteAttrs::get_ip_proto;
        let _ = IterableRouteAttrs::get_mark;
        let _ = IterableRouteAttrs::get_metrics;
        let _ = IterableRouteAttrs::get_mfc_stats;
        let _ = IterableRouteAttrs::get_multipath;
        let _ = IterableRouteAttrs::get_newdst;
        let _ = IterableRouteAttrs::get_nh_id;
        let _ = IterableRouteAttrs::get_oif;
        let _ = IterableRouteAttrs::get_pad;
        let _ = IterableRouteAttrs::get_pref;
        let _ = IterableRouteAttrs::get_prefsrc;
        let _ = IterableRouteAttrs::get_priority;
        let _ = IterableRouteAttrs::get_sport;
        let _ = IterableRouteAttrs::get_src;
        let _ = IterableRouteAttrs::get_table;
        let _ = IterableRouteAttrs::get_ttl_propagate;
        let _ = IterableRouteAttrs::get_uid;
        let _ = IterableRouteAttrs::get_via;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::nested_encap_bpf;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::nested_encap_ila;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::nested_encap_ioam6;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::nested_encap_ip6;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::nested_encap_ip;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::nested_encap_mpls;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::nested_encap_rpl;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::nested_encap_seg6;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::nested_encap_seg6_local;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::nested_encap_xfrm;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::nested_metrics;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_cacheinfo;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_dport;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_dst;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_encap_type;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_expires;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_flow;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_flowlabel;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_gateway;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_iif;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_ip_proto;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_mark;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_mfc_stats;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_multipath;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_newdst;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_nh_id;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_oif;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_pad;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_pref;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_prefsrc;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_priority;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_sport;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_src;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_table;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_ttl_propagate;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_uid;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_via;
    }
}
