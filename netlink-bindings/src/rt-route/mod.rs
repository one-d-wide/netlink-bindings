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
    traits::{NetlinkRequest, Protocol},
    utils::*,
};
pub const PROTONAME: &str = "rt-route";
pub const PROTONAME_CSTR: &CStr = c"rt-route";
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
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
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
    pub fn get_encap(&self) -> Result<&'a [u8], ErrorContext> {
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
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
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
                RouteAttrs::EncapType(val) => fmt.field("EncapType", &val),
                RouteAttrs::Encap(val) => fmt.field("Encap", &FormatHexdump(val)),
                RouteAttrs::Expires(val) => fmt.field("Expires", &val),
                RouteAttrs::Pad(val) => fmt.field("Pad", &FormatHexdump(val)),
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
    pub fn push_encap_type(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 21u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 22u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
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
#[doc = "Dump route information.\n\nReply attributes:\n- [.get_dst()](IterableRouteAttrs::get_dst)\n- [.get_src()](IterableRouteAttrs::get_src)\n- [.get_iif()](IterableRouteAttrs::get_iif)\n- [.get_oif()](IterableRouteAttrs::get_oif)\n- [.get_gateway()](IterableRouteAttrs::get_gateway)\n- [.get_priority()](IterableRouteAttrs::get_priority)\n- [.get_prefsrc()](IterableRouteAttrs::get_prefsrc)\n- [.get_metrics()](IterableRouteAttrs::get_metrics)\n- [.get_multipath()](IterableRouteAttrs::get_multipath)\n- [.get_flow()](IterableRouteAttrs::get_flow)\n- [.get_cacheinfo()](IterableRouteAttrs::get_cacheinfo)\n- [.get_table()](IterableRouteAttrs::get_table)\n- [.get_mark()](IterableRouteAttrs::get_mark)\n- [.get_mfc_stats()](IterableRouteAttrs::get_mfc_stats)\n- [.get_via()](IterableRouteAttrs::get_via)\n- [.get_newdst()](IterableRouteAttrs::get_newdst)\n- [.get_pref()](IterableRouteAttrs::get_pref)\n- [.get_encap_type()](IterableRouteAttrs::get_encap_type)\n- [.get_encap()](IterableRouteAttrs::get_encap)\n- [.get_expires()](IterableRouteAttrs::get_expires)\n- [.get_pad()](IterableRouteAttrs::get_pad)\n- [.get_uid()](IterableRouteAttrs::get_uid)\n- [.get_ttl_propagate()](IterableRouteAttrs::get_ttl_propagate)\n- [.get_ip_proto()](IterableRouteAttrs::get_ip_proto)\n- [.get_sport()](IterableRouteAttrs::get_sport)\n- [.get_dport()](IterableRouteAttrs::get_dport)\n- [.get_nh_id()](IterableRouteAttrs::get_nh_id)\n- [.get_flowlabel()](IterableRouteAttrs::get_flowlabel)\n\n"]
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
#[doc = "Dump route information.\n\nRequest attributes:\n- [.push_dst()](PushRouteAttrs::push_dst)\n- [.push_src()](PushRouteAttrs::push_src)\n- [.push_iif()](PushRouteAttrs::push_iif)\n- [.push_oif()](PushRouteAttrs::push_oif)\n- [.push_mark()](PushRouteAttrs::push_mark)\n- [.push_uid()](PushRouteAttrs::push_uid)\n- [.push_ip_proto()](PushRouteAttrs::push_ip_proto)\n- [.push_sport()](PushRouteAttrs::push_sport)\n- [.push_dport()](PushRouteAttrs::push_dport)\n- [.push_flowlabel()](PushRouteAttrs::push_flowlabel)\n\nReply attributes:\n- [.get_dst()](IterableRouteAttrs::get_dst)\n- [.get_src()](IterableRouteAttrs::get_src)\n- [.get_iif()](IterableRouteAttrs::get_iif)\n- [.get_oif()](IterableRouteAttrs::get_oif)\n- [.get_gateway()](IterableRouteAttrs::get_gateway)\n- [.get_priority()](IterableRouteAttrs::get_priority)\n- [.get_prefsrc()](IterableRouteAttrs::get_prefsrc)\n- [.get_metrics()](IterableRouteAttrs::get_metrics)\n- [.get_multipath()](IterableRouteAttrs::get_multipath)\n- [.get_flow()](IterableRouteAttrs::get_flow)\n- [.get_cacheinfo()](IterableRouteAttrs::get_cacheinfo)\n- [.get_table()](IterableRouteAttrs::get_table)\n- [.get_mark()](IterableRouteAttrs::get_mark)\n- [.get_mfc_stats()](IterableRouteAttrs::get_mfc_stats)\n- [.get_via()](IterableRouteAttrs::get_via)\n- [.get_newdst()](IterableRouteAttrs::get_newdst)\n- [.get_pref()](IterableRouteAttrs::get_pref)\n- [.get_encap_type()](IterableRouteAttrs::get_encap_type)\n- [.get_encap()](IterableRouteAttrs::get_encap)\n- [.get_expires()](IterableRouteAttrs::get_expires)\n- [.get_pad()](IterableRouteAttrs::get_pad)\n- [.get_uid()](IterableRouteAttrs::get_uid)\n- [.get_ttl_propagate()](IterableRouteAttrs::get_ttl_propagate)\n- [.get_ip_proto()](IterableRouteAttrs::get_ip_proto)\n- [.get_sport()](IterableRouteAttrs::get_sport)\n- [.get_dport()](IterableRouteAttrs::get_dport)\n- [.get_nh_id()](IterableRouteAttrs::get_nh_id)\n- [.get_flowlabel()](IterableRouteAttrs::get_flowlabel)\n\n"]
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
#[doc = "Create a new route\n\nRequest attributes:\n- [.push_dst()](PushRouteAttrs::push_dst)\n- [.push_src()](PushRouteAttrs::push_src)\n- [.push_iif()](PushRouteAttrs::push_iif)\n- [.push_oif()](PushRouteAttrs::push_oif)\n- [.push_gateway()](PushRouteAttrs::push_gateway)\n- [.push_priority()](PushRouteAttrs::push_priority)\n- [.push_prefsrc()](PushRouteAttrs::push_prefsrc)\n- [.nested_metrics()](PushRouteAttrs::nested_metrics)\n- [.push_multipath()](PushRouteAttrs::push_multipath)\n- [.push_flow()](PushRouteAttrs::push_flow)\n- [.push_cacheinfo()](PushRouteAttrs::push_cacheinfo)\n- [.push_table()](PushRouteAttrs::push_table)\n- [.push_mark()](PushRouteAttrs::push_mark)\n- [.push_mfc_stats()](PushRouteAttrs::push_mfc_stats)\n- [.push_via()](PushRouteAttrs::push_via)\n- [.push_newdst()](PushRouteAttrs::push_newdst)\n- [.push_pref()](PushRouteAttrs::push_pref)\n- [.push_encap_type()](PushRouteAttrs::push_encap_type)\n- [.push_encap()](PushRouteAttrs::push_encap)\n- [.push_expires()](PushRouteAttrs::push_expires)\n- [.push_pad()](PushRouteAttrs::push_pad)\n- [.push_uid()](PushRouteAttrs::push_uid)\n- [.push_ttl_propagate()](PushRouteAttrs::push_ttl_propagate)\n- [.push_ip_proto()](PushRouteAttrs::push_ip_proto)\n- [.push_sport()](PushRouteAttrs::push_sport)\n- [.push_dport()](PushRouteAttrs::push_dport)\n- [.push_nh_id()](PushRouteAttrs::push_nh_id)\n- [.push_flowlabel()](PushRouteAttrs::push_flowlabel)\n\n"]
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
#[doc = "Delete an existing route\n\nRequest attributes:\n- [.push_dst()](PushRouteAttrs::push_dst)\n- [.push_src()](PushRouteAttrs::push_src)\n- [.push_iif()](PushRouteAttrs::push_iif)\n- [.push_oif()](PushRouteAttrs::push_oif)\n- [.push_gateway()](PushRouteAttrs::push_gateway)\n- [.push_priority()](PushRouteAttrs::push_priority)\n- [.push_prefsrc()](PushRouteAttrs::push_prefsrc)\n- [.nested_metrics()](PushRouteAttrs::nested_metrics)\n- [.push_multipath()](PushRouteAttrs::push_multipath)\n- [.push_flow()](PushRouteAttrs::push_flow)\n- [.push_cacheinfo()](PushRouteAttrs::push_cacheinfo)\n- [.push_table()](PushRouteAttrs::push_table)\n- [.push_mark()](PushRouteAttrs::push_mark)\n- [.push_mfc_stats()](PushRouteAttrs::push_mfc_stats)\n- [.push_via()](PushRouteAttrs::push_via)\n- [.push_newdst()](PushRouteAttrs::push_newdst)\n- [.push_pref()](PushRouteAttrs::push_pref)\n- [.push_encap_type()](PushRouteAttrs::push_encap_type)\n- [.push_encap()](PushRouteAttrs::push_encap)\n- [.push_expires()](PushRouteAttrs::push_expires)\n- [.push_pad()](PushRouteAttrs::push_pad)\n- [.push_uid()](PushRouteAttrs::push_uid)\n- [.push_ttl_propagate()](PushRouteAttrs::push_ttl_propagate)\n- [.push_ip_proto()](PushRouteAttrs::push_ip_proto)\n- [.push_sport()](PushRouteAttrs::push_sport)\n- [.push_dport()](PushRouteAttrs::push_dport)\n- [.push_nh_id()](PushRouteAttrs::push_nh_id)\n- [.push_flowlabel()](PushRouteAttrs::push_flowlabel)\n\n"]
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
    #[doc = "Dump route information.\n\nReply attributes:\n- [.get_dst()](IterableRouteAttrs::get_dst)\n- [.get_src()](IterableRouteAttrs::get_src)\n- [.get_iif()](IterableRouteAttrs::get_iif)\n- [.get_oif()](IterableRouteAttrs::get_oif)\n- [.get_gateway()](IterableRouteAttrs::get_gateway)\n- [.get_priority()](IterableRouteAttrs::get_priority)\n- [.get_prefsrc()](IterableRouteAttrs::get_prefsrc)\n- [.get_metrics()](IterableRouteAttrs::get_metrics)\n- [.get_multipath()](IterableRouteAttrs::get_multipath)\n- [.get_flow()](IterableRouteAttrs::get_flow)\n- [.get_cacheinfo()](IterableRouteAttrs::get_cacheinfo)\n- [.get_table()](IterableRouteAttrs::get_table)\n- [.get_mark()](IterableRouteAttrs::get_mark)\n- [.get_mfc_stats()](IterableRouteAttrs::get_mfc_stats)\n- [.get_via()](IterableRouteAttrs::get_via)\n- [.get_newdst()](IterableRouteAttrs::get_newdst)\n- [.get_pref()](IterableRouteAttrs::get_pref)\n- [.get_encap_type()](IterableRouteAttrs::get_encap_type)\n- [.get_encap()](IterableRouteAttrs::get_encap)\n- [.get_expires()](IterableRouteAttrs::get_expires)\n- [.get_pad()](IterableRouteAttrs::get_pad)\n- [.get_uid()](IterableRouteAttrs::get_uid)\n- [.get_ttl_propagate()](IterableRouteAttrs::get_ttl_propagate)\n- [.get_ip_proto()](IterableRouteAttrs::get_ip_proto)\n- [.get_sport()](IterableRouteAttrs::get_sport)\n- [.get_dport()](IterableRouteAttrs::get_dport)\n- [.get_nh_id()](IterableRouteAttrs::get_nh_id)\n- [.get_flowlabel()](IterableRouteAttrs::get_flowlabel)\n\n"]
    pub fn op_getroute_dump(self, header: &Rtmsg) -> OpGetrouteDump<'buf> {
        let mut res = OpGetrouteDump::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getroute-dump", OpGetrouteDump::lookup);
        res
    }
    #[doc = "Dump route information.\n\nRequest attributes:\n- [.push_dst()](PushRouteAttrs::push_dst)\n- [.push_src()](PushRouteAttrs::push_src)\n- [.push_iif()](PushRouteAttrs::push_iif)\n- [.push_oif()](PushRouteAttrs::push_oif)\n- [.push_mark()](PushRouteAttrs::push_mark)\n- [.push_uid()](PushRouteAttrs::push_uid)\n- [.push_ip_proto()](PushRouteAttrs::push_ip_proto)\n- [.push_sport()](PushRouteAttrs::push_sport)\n- [.push_dport()](PushRouteAttrs::push_dport)\n- [.push_flowlabel()](PushRouteAttrs::push_flowlabel)\n\nReply attributes:\n- [.get_dst()](IterableRouteAttrs::get_dst)\n- [.get_src()](IterableRouteAttrs::get_src)\n- [.get_iif()](IterableRouteAttrs::get_iif)\n- [.get_oif()](IterableRouteAttrs::get_oif)\n- [.get_gateway()](IterableRouteAttrs::get_gateway)\n- [.get_priority()](IterableRouteAttrs::get_priority)\n- [.get_prefsrc()](IterableRouteAttrs::get_prefsrc)\n- [.get_metrics()](IterableRouteAttrs::get_metrics)\n- [.get_multipath()](IterableRouteAttrs::get_multipath)\n- [.get_flow()](IterableRouteAttrs::get_flow)\n- [.get_cacheinfo()](IterableRouteAttrs::get_cacheinfo)\n- [.get_table()](IterableRouteAttrs::get_table)\n- [.get_mark()](IterableRouteAttrs::get_mark)\n- [.get_mfc_stats()](IterableRouteAttrs::get_mfc_stats)\n- [.get_via()](IterableRouteAttrs::get_via)\n- [.get_newdst()](IterableRouteAttrs::get_newdst)\n- [.get_pref()](IterableRouteAttrs::get_pref)\n- [.get_encap_type()](IterableRouteAttrs::get_encap_type)\n- [.get_encap()](IterableRouteAttrs::get_encap)\n- [.get_expires()](IterableRouteAttrs::get_expires)\n- [.get_pad()](IterableRouteAttrs::get_pad)\n- [.get_uid()](IterableRouteAttrs::get_uid)\n- [.get_ttl_propagate()](IterableRouteAttrs::get_ttl_propagate)\n- [.get_ip_proto()](IterableRouteAttrs::get_ip_proto)\n- [.get_sport()](IterableRouteAttrs::get_sport)\n- [.get_dport()](IterableRouteAttrs::get_dport)\n- [.get_nh_id()](IterableRouteAttrs::get_nh_id)\n- [.get_flowlabel()](IterableRouteAttrs::get_flowlabel)\n\n"]
    pub fn op_getroute_do(self, header: &Rtmsg) -> OpGetrouteDo<'buf> {
        let mut res = OpGetrouteDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getroute-do", OpGetrouteDo::lookup);
        res
    }
    #[doc = "Create a new route\n\nRequest attributes:\n- [.push_dst()](PushRouteAttrs::push_dst)\n- [.push_src()](PushRouteAttrs::push_src)\n- [.push_iif()](PushRouteAttrs::push_iif)\n- [.push_oif()](PushRouteAttrs::push_oif)\n- [.push_gateway()](PushRouteAttrs::push_gateway)\n- [.push_priority()](PushRouteAttrs::push_priority)\n- [.push_prefsrc()](PushRouteAttrs::push_prefsrc)\n- [.nested_metrics()](PushRouteAttrs::nested_metrics)\n- [.push_multipath()](PushRouteAttrs::push_multipath)\n- [.push_flow()](PushRouteAttrs::push_flow)\n- [.push_cacheinfo()](PushRouteAttrs::push_cacheinfo)\n- [.push_table()](PushRouteAttrs::push_table)\n- [.push_mark()](PushRouteAttrs::push_mark)\n- [.push_mfc_stats()](PushRouteAttrs::push_mfc_stats)\n- [.push_via()](PushRouteAttrs::push_via)\n- [.push_newdst()](PushRouteAttrs::push_newdst)\n- [.push_pref()](PushRouteAttrs::push_pref)\n- [.push_encap_type()](PushRouteAttrs::push_encap_type)\n- [.push_encap()](PushRouteAttrs::push_encap)\n- [.push_expires()](PushRouteAttrs::push_expires)\n- [.push_pad()](PushRouteAttrs::push_pad)\n- [.push_uid()](PushRouteAttrs::push_uid)\n- [.push_ttl_propagate()](PushRouteAttrs::push_ttl_propagate)\n- [.push_ip_proto()](PushRouteAttrs::push_ip_proto)\n- [.push_sport()](PushRouteAttrs::push_sport)\n- [.push_dport()](PushRouteAttrs::push_dport)\n- [.push_nh_id()](PushRouteAttrs::push_nh_id)\n- [.push_flowlabel()](PushRouteAttrs::push_flowlabel)\n\n"]
    pub fn op_newroute_do(self, header: &Rtmsg) -> OpNewrouteDo<'buf> {
        let mut res = OpNewrouteDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-newroute-do", OpNewrouteDo::lookup);
        res
    }
    #[doc = "Delete an existing route\n\nRequest attributes:\n- [.push_dst()](PushRouteAttrs::push_dst)\n- [.push_src()](PushRouteAttrs::push_src)\n- [.push_iif()](PushRouteAttrs::push_iif)\n- [.push_oif()](PushRouteAttrs::push_oif)\n- [.push_gateway()](PushRouteAttrs::push_gateway)\n- [.push_priority()](PushRouteAttrs::push_priority)\n- [.push_prefsrc()](PushRouteAttrs::push_prefsrc)\n- [.nested_metrics()](PushRouteAttrs::nested_metrics)\n- [.push_multipath()](PushRouteAttrs::push_multipath)\n- [.push_flow()](PushRouteAttrs::push_flow)\n- [.push_cacheinfo()](PushRouteAttrs::push_cacheinfo)\n- [.push_table()](PushRouteAttrs::push_table)\n- [.push_mark()](PushRouteAttrs::push_mark)\n- [.push_mfc_stats()](PushRouteAttrs::push_mfc_stats)\n- [.push_via()](PushRouteAttrs::push_via)\n- [.push_newdst()](PushRouteAttrs::push_newdst)\n- [.push_pref()](PushRouteAttrs::push_pref)\n- [.push_encap_type()](PushRouteAttrs::push_encap_type)\n- [.push_encap()](PushRouteAttrs::push_encap)\n- [.push_expires()](PushRouteAttrs::push_expires)\n- [.push_pad()](PushRouteAttrs::push_pad)\n- [.push_uid()](PushRouteAttrs::push_uid)\n- [.push_ttl_propagate()](PushRouteAttrs::push_ttl_propagate)\n- [.push_ip_proto()](PushRouteAttrs::push_ip_proto)\n- [.push_sport()](PushRouteAttrs::push_sport)\n- [.push_dport()](PushRouteAttrs::push_dport)\n- [.push_nh_id()](PushRouteAttrs::push_nh_id)\n- [.push_flowlabel()](PushRouteAttrs::push_flowlabel)\n\n"]
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
        let _ = PushRouteAttrs::<&mut Vec<u8>>::nested_metrics;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_cacheinfo;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_dport;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_dst;
        let _ = PushRouteAttrs::<&mut Vec<u8>>::push_encap;
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
