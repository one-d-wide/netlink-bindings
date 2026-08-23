#![doc = "Link configuration over rtnetlink.\n"]
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
pub const PROTONAME: &str = "rt-link";
pub const PROTONAME_CSTR: &CStr = c"rt-link";
pub const PROTONUM: u16 = 0u16;
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum IfinfoFlags {
    Up = 1 << 0,
    Broadcast = 1 << 1,
    Debug = 1 << 2,
    Loopback = 1 << 3,
    PointToPoint = 1 << 4,
    NoTrailers = 1 << 5,
    Running = 1 << 6,
    NoArp = 1 << 7,
    Promisc = 1 << 8,
    AllMulti = 1 << 9,
    Master = 1 << 10,
    Slave = 1 << 11,
    Multicast = 1 << 12,
    Portsel = 1 << 13,
    AutoMedia = 1 << 14,
    Dynamic = 1 << 15,
    LowerUp = 1 << 16,
    Dormant = 1 << 17,
    Echo = 1 << 18,
}
impl IfinfoFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::Up,
            n if n == 1 << 1 => Self::Broadcast,
            n if n == 1 << 2 => Self::Debug,
            n if n == 1 << 3 => Self::Loopback,
            n if n == 1 << 4 => Self::PointToPoint,
            n if n == 1 << 5 => Self::NoTrailers,
            n if n == 1 << 6 => Self::Running,
            n if n == 1 << 7 => Self::NoArp,
            n if n == 1 << 8 => Self::Promisc,
            n if n == 1 << 9 => Self::AllMulti,
            n if n == 1 << 10 => Self::Master,
            n if n == 1 << 11 => Self::Slave,
            n if n == 1 << 12 => Self::Multicast,
            n if n == 1 << 13 => Self::Portsel,
            n if n == 1 << 14 => Self::AutoMedia,
            n if n == 1 << 15 => Self::Dynamic,
            n if n == 1 << 16 => Self::LowerUp,
            n if n == 1 << 17 => Self::Dormant,
            n if n == 1 << 18 => Self::Echo,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum VlanProtocols {
    _8021q = 33024,
    _8021ad = 34984,
}
impl VlanProtocols {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            33024 => Self::_8021q,
            34984 => Self::_8021ad,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum Ipv4Devconf {
    Forwarding = 0,
    McForwarding = 1,
    ProxyArp = 2,
    AcceptRedirects = 3,
    SecureRedirects = 4,
    SendRedirects = 5,
    SharedMedia = 6,
    RpFilter = 7,
    AcceptSourceRoute = 8,
    BootpRelay = 9,
    LogMartians = 10,
    Tag = 11,
    Arpfilter = 12,
    MediumId = 13,
    Noxfrm = 14,
    Nopolicy = 15,
    ForceIgmpVersion = 16,
    ArpAnnounce = 17,
    ArpIgnore = 18,
    PromoteSecondaries = 19,
    ArpAccept = 20,
    ArpNotify = 21,
    AcceptLocal = 22,
    SrcVmark = 23,
    ProxyArpPvlan = 24,
    RouteLocalnet = 25,
    Igmpv2UnsolicitedReportInterval = 26,
    Igmpv3UnsolicitedReportInterval = 27,
    IgnoreRoutesWithLinkdown = 28,
    DropUnicastInL2Multicast = 29,
    DropGratuitousArp = 30,
    BcForwarding = 31,
    ArpEvictNocarrier = 32,
}
impl Ipv4Devconf {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Forwarding,
            1 => Self::McForwarding,
            2 => Self::ProxyArp,
            3 => Self::AcceptRedirects,
            4 => Self::SecureRedirects,
            5 => Self::SendRedirects,
            6 => Self::SharedMedia,
            7 => Self::RpFilter,
            8 => Self::AcceptSourceRoute,
            9 => Self::BootpRelay,
            10 => Self::LogMartians,
            11 => Self::Tag,
            12 => Self::Arpfilter,
            13 => Self::MediumId,
            14 => Self::Noxfrm,
            15 => Self::Nopolicy,
            16 => Self::ForceIgmpVersion,
            17 => Self::ArpAnnounce,
            18 => Self::ArpIgnore,
            19 => Self::PromoteSecondaries,
            20 => Self::ArpAccept,
            21 => Self::ArpNotify,
            22 => Self::AcceptLocal,
            23 => Self::SrcVmark,
            24 => Self::ProxyArpPvlan,
            25 => Self::RouteLocalnet,
            26 => Self::Igmpv2UnsolicitedReportInterval,
            27 => Self::Igmpv3UnsolicitedReportInterval,
            28 => Self::IgnoreRoutesWithLinkdown,
            29 => Self::DropUnicastInL2Multicast,
            30 => Self::DropGratuitousArp,
            31 => Self::BcForwarding,
            32 => Self::ArpEvictNocarrier,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum Ipv6Devconf {
    Forwarding = 0,
    Hoplimit = 1,
    Mtu6 = 2,
    AcceptRa = 3,
    AcceptRedirects = 4,
    Autoconf = 5,
    DadTransmits = 6,
    RtrSolicits = 7,
    RtrSolicitInterval = 8,
    RtrSolicitDelay = 9,
    UseTempaddr = 10,
    TempValidLft = 11,
    TempPreferedLft = 12,
    RegenMaxRetry = 13,
    MaxDesyncFactor = 14,
    MaxAddresses = 15,
    ForceMldVersion = 16,
    AcceptRaDefrtr = 17,
    AcceptRaPinfo = 18,
    AcceptRaRtrPref = 19,
    RtrProbeInterval = 20,
    AcceptRaRtInfoMaxPlen = 21,
    ProxyNdp = 22,
    OptimisticDad = 23,
    AcceptSourceRoute = 24,
    McForwarding = 25,
    DisableIpv6 = 26,
    AcceptDad = 27,
    ForceTllao = 28,
    NdiscNotify = 29,
    Mldv1UnsolicitedReportInterval = 30,
    Mldv2UnsolicitedReportInterval = 31,
    SuppressFragNdisc = 32,
    AcceptRaFromLocal = 33,
    UseOptimistic = 34,
    AcceptRaMtu = 35,
    StableSecret = 36,
    UseOifAddrsOnly = 37,
    AcceptRaMinHopLimit = 38,
    IgnoreRoutesWithLinkdown = 39,
    DropUnicastInL2Multicast = 40,
    DropUnsolicitedNa = 41,
    KeepAddrOnDown = 42,
    RtrSolicitMaxInterval = 43,
    Seg6Enabled = 44,
    Seg6RequireHmac = 45,
    EnhancedDad = 46,
    AddrGenMode = 47,
    DisablePolicy = 48,
    AcceptRaRtInfoMinPlen = 49,
    NdiscTclass = 50,
    RplSegEnabled = 51,
    RaDefrtrMetric = 52,
    Ioam6Enabled = 53,
    Ioam6Id = 54,
    Ioam6IdWide = 55,
    NdiscEvictNocarrier = 56,
    AcceptUntrackedNa = 57,
}
impl Ipv6Devconf {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Forwarding,
            1 => Self::Hoplimit,
            2 => Self::Mtu6,
            3 => Self::AcceptRa,
            4 => Self::AcceptRedirects,
            5 => Self::Autoconf,
            6 => Self::DadTransmits,
            7 => Self::RtrSolicits,
            8 => Self::RtrSolicitInterval,
            9 => Self::RtrSolicitDelay,
            10 => Self::UseTempaddr,
            11 => Self::TempValidLft,
            12 => Self::TempPreferedLft,
            13 => Self::RegenMaxRetry,
            14 => Self::MaxDesyncFactor,
            15 => Self::MaxAddresses,
            16 => Self::ForceMldVersion,
            17 => Self::AcceptRaDefrtr,
            18 => Self::AcceptRaPinfo,
            19 => Self::AcceptRaRtrPref,
            20 => Self::RtrProbeInterval,
            21 => Self::AcceptRaRtInfoMaxPlen,
            22 => Self::ProxyNdp,
            23 => Self::OptimisticDad,
            24 => Self::AcceptSourceRoute,
            25 => Self::McForwarding,
            26 => Self::DisableIpv6,
            27 => Self::AcceptDad,
            28 => Self::ForceTllao,
            29 => Self::NdiscNotify,
            30 => Self::Mldv1UnsolicitedReportInterval,
            31 => Self::Mldv2UnsolicitedReportInterval,
            32 => Self::SuppressFragNdisc,
            33 => Self::AcceptRaFromLocal,
            34 => Self::UseOptimistic,
            35 => Self::AcceptRaMtu,
            36 => Self::StableSecret,
            37 => Self::UseOifAddrsOnly,
            38 => Self::AcceptRaMinHopLimit,
            39 => Self::IgnoreRoutesWithLinkdown,
            40 => Self::DropUnicastInL2Multicast,
            41 => Self::DropUnsolicitedNa,
            42 => Self::KeepAddrOnDown,
            43 => Self::RtrSolicitMaxInterval,
            44 => Self::Seg6Enabled,
            45 => Self::Seg6RequireHmac,
            46 => Self::EnhancedDad,
            47 => Self::AddrGenMode,
            48 => Self::DisablePolicy,
            49 => Self::AcceptRaRtInfoMinPlen,
            50 => Self::NdiscTclass,
            51 => Self::RplSegEnabled,
            52 => Self::RaDefrtrMetric,
            53 => Self::Ioam6Enabled,
            54 => Self::Ioam6Id,
            55 => Self::Ioam6IdWide,
            56 => Self::NdiscEvictNocarrier,
            57 => Self::AcceptUntrackedNa,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum IflaIcmp6Stats {
    Num = 0,
    Inmsgs = 1,
    Inerrors = 2,
    Outmsgs = 3,
    Outerrors = 4,
    Csumerrors = 5,
    Ratelimithost = 6,
}
impl IflaIcmp6Stats {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Num,
            1 => Self::Inmsgs,
            2 => Self::Inerrors,
            3 => Self::Outmsgs,
            4 => Self::Outerrors,
            5 => Self::Csumerrors,
            6 => Self::Ratelimithost,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum IflaInet6Stats {
    Num = 0,
    Inpkts = 1,
    Inoctets = 2,
    Indelivers = 3,
    Outforwdatagrams = 4,
    Outpkts = 5,
    Outoctets = 6,
    Inhdrerrors = 7,
    Intoobigerrors = 8,
    Innoroutes = 9,
    Inaddrerrors = 10,
    Inunknownprotos = 11,
    Intruncatedpkts = 12,
    Indiscards = 13,
    Outdiscards = 14,
    Outnoroutes = 15,
    Reasmtimeout = 16,
    Reasmreqds = 17,
    Reasmoks = 18,
    Reasmfails = 19,
    Fragoks = 20,
    Fragfails = 21,
    Fragcreates = 22,
    Inmcastpkts = 23,
    Outmcastpkts = 24,
    Inbcastpkts = 25,
    Outbcastpkts = 26,
    Inmcastoctets = 27,
    Outmcastoctets = 28,
    Inbcastoctets = 29,
    Outbcastoctets = 30,
    Csumerrors = 31,
    Noectpkts = 32,
    Ect1Pkts = 33,
    Ect0Pkts = 34,
    Cepkts = 35,
    ReasmOverlaps = 36,
}
impl IflaInet6Stats {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Num,
            1 => Self::Inpkts,
            2 => Self::Inoctets,
            3 => Self::Indelivers,
            4 => Self::Outforwdatagrams,
            5 => Self::Outpkts,
            6 => Self::Outoctets,
            7 => Self::Inhdrerrors,
            8 => Self::Intoobigerrors,
            9 => Self::Innoroutes,
            10 => Self::Inaddrerrors,
            11 => Self::Inunknownprotos,
            12 => Self::Intruncatedpkts,
            13 => Self::Indiscards,
            14 => Self::Outdiscards,
            15 => Self::Outnoroutes,
            16 => Self::Reasmtimeout,
            17 => Self::Reasmreqds,
            18 => Self::Reasmoks,
            19 => Self::Reasmfails,
            20 => Self::Fragoks,
            21 => Self::Fragfails,
            22 => Self::Fragcreates,
            23 => Self::Inmcastpkts,
            24 => Self::Outmcastpkts,
            25 => Self::Inbcastpkts,
            26 => Self::Outbcastpkts,
            27 => Self::Inmcastoctets,
            28 => Self::Outmcastoctets,
            29 => Self::Inbcastoctets,
            30 => Self::Outbcastoctets,
            31 => Self::Csumerrors,
            32 => Self::Noectpkts,
            33 => Self::Ect1Pkts,
            34 => Self::Ect0Pkts,
            35 => Self::Cepkts,
            36 => Self::ReasmOverlaps,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum VlanFlags {
    ReorderHdr = 1 << 0,
    Gvrp = 1 << 1,
    LooseBinding = 1 << 2,
    Mvrp = 1 << 3,
    BridgeBinding = 1 << 4,
}
impl VlanFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::ReorderHdr,
            n if n == 1 << 1 => Self::Gvrp,
            n if n == 1 << 2 => Self::LooseBinding,
            n if n == 1 << 3 => Self::Mvrp,
            n if n == 1 << 4 => Self::BridgeBinding,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum IflaVfLinkStateEnum {
    Auto = 0,
    Enable = 1,
    Disable = 2,
}
impl IflaVfLinkStateEnum {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Auto,
            1 => Self::Enable,
            2 => Self::Disable,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum RtextFilter {
    Vf = 1 << 0,
    Brvlan = 1 << 1,
    BrvlanCompressed = 1 << 2,
    SkipStats = 1 << 3,
    Mrp = 1 << 4,
    CfmConfig = 1 << 5,
    CfmStatus = 1 << 6,
    Mst = 1 << 7,
}
impl RtextFilter {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::Vf,
            n if n == 1 << 1 => Self::Brvlan,
            n if n == 1 << 2 => Self::BrvlanCompressed,
            n if n == 1 << 3 => Self::SkipStats,
            n if n == 1 << 4 => Self::Mrp,
            n if n == 1 << 5 => Self::CfmConfig,
            n if n == 1 << 6 => Self::CfmStatus,
            n if n == 1 << 7 => Self::Mst,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum NetkitPolicy {
    Forward = 0,
    Blackhole = 2,
}
impl NetkitPolicy {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Forward,
            2 => Self::Blackhole,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum NetkitMode {
    L2 = 0,
    L3 = 1,
}
impl NetkitMode {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::L2,
            1 => Self::L3,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum NetkitScrub {
    None = 0,
    Default = 1,
}
impl NetkitScrub {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::None,
            1 => Self::Default,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum NetkitPairing {
    Pair = 0,
    Single = 1,
}
impl NetkitPairing {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Pair,
            1 => Self::Single,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum OvpnMode {
    P2p = 0,
    Mp = 1,
}
impl OvpnMode {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::P2p,
            1 => Self::Mp,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum BrStpMode {
    Auto = 0,
    User = 1,
    Kernel = 2,
}
impl BrStpMode {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Auto,
            1 => Self::User,
            2 => Self::Kernel,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum BrBooloptId {
    #[doc = "disable learning from link-local packets\n"]
    NoLlLearn = 1 << 0,
    #[doc = "control vlan multicast snooping\n"]
    McastVlanSnooping = 1 << 1,
    MstEnable = 1 << 2,
    MdbOffloadFailNotification = 1 << 3,
    #[doc = "local FDB entries installed by the bridge driver itself should only be\nadded on VLAN 0\n"]
    FdbLocalVlan0 = 1 << 4,
}
impl BrBooloptId {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::NoLlLearn,
            n if n == 1 << 1 => Self::McastVlanSnooping,
            n if n == 1 << 2 => Self::MstEnable,
            n if n == 1 << 3 => Self::MdbOffloadFailNotification,
            n if n == 1 << 4 => Self::FdbLocalVlan0,
            _ => return None,
        })
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Rtgenmsg {
    pub family: u8,
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
    pub fn new_from_array(buf: [u8; 1usize]) -> Self {
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
    pub fn as_array(&self) -> &[u8; 1usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 1usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 1usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<Rtgenmsg>() == 1usize);
        const _: () = assert!(std::mem::align_of::<Rtgenmsg>() == 1usize);
        1usize
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Ifinfomsg {
    pub ifi_family: u8,
    pub _pad: [u8; 1usize],
    pub ifi_type: u16,
    pub ifi_index: i32,
    #[doc = "Associated type: [`IfinfoFlags`] (1 bit per enumeration)"]
    pub ifi_flags: u32,
    pub ifi_change: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for Ifinfomsg {
    fn default() -> Self {
        Self::new()
    }
}
impl Ifinfomsg {
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
        const _: () = assert!(std::mem::size_of::<Ifinfomsg>() == 16usize);
        const _: () = assert!(std::mem::align_of::<Ifinfomsg>() == 4usize);
        16usize
    }
}
impl std::fmt::Debug for Ifinfomsg {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Ifinfomsg")
            .field("ifi_family", &self.ifi_family)
            .field("ifi_type", &self.ifi_type)
            .field("ifi_index", &self.ifi_index)
            .field(
                "ifi_flags",
                &FormatFlags(self.ifi_flags.into(), IfinfoFlags::from_value),
            )
            .field("ifi_change", &self.ifi_change)
            .finish()
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct IflaBridgeId {
    pub prio: u16,
    pub addr: [u8; 6usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for IflaBridgeId {
    fn default() -> Self {
        Self::new()
    }
}
impl IflaBridgeId {
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
        const _: () = assert!(std::mem::size_of::<IflaBridgeId>() == 8usize);
        const _: () = assert!(std::mem::align_of::<IflaBridgeId>() == 2usize);
        8usize
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct IflaCacheinfo {
    pub max_reasm_len: u32,
    pub tstamp: u32,
    pub reachable_time: i32,
    pub retrans_time: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for IflaCacheinfo {
    fn default() -> Self {
        Self::new()
    }
}
impl IflaCacheinfo {
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
        const _: () = assert!(std::mem::size_of::<IflaCacheinfo>() == 16usize);
        const _: () = assert!(std::mem::align_of::<IflaCacheinfo>() == 4usize);
        16usize
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct RtnlLinkStats {
    pub rx_packets: u32,
    pub tx_packets: u32,
    pub rx_bytes: u32,
    pub tx_bytes: u32,
    pub rx_errors: u32,
    pub tx_errors: u32,
    pub rx_dropped: u32,
    pub tx_dropped: u32,
    pub multicast: u32,
    pub collisions: u32,
    pub rx_length_errors: u32,
    pub rx_over_errors: u32,
    pub rx_crc_errors: u32,
    pub rx_frame_errors: u32,
    pub rx_fifo_errors: u32,
    pub rx_missed_errors: u32,
    pub tx_aborted_errors: u32,
    pub tx_carrier_errors: u32,
    pub tx_fifo_errors: u32,
    pub tx_heartbeat_errors: u32,
    pub tx_window_errors: u32,
    pub rx_compressed: u32,
    pub tx_compressed: u32,
    pub rx_nohandler: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for RtnlLinkStats {
    fn default() -> Self {
        Self::new()
    }
}
impl RtnlLinkStats {
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
    pub fn new_from_array(buf: [u8; 96usize]) -> Self {
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
    pub fn as_array(&self) -> &[u8; 96usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 96usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 96usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<RtnlLinkStats>() == 96usize);
        const _: () = assert!(std::mem::align_of::<RtnlLinkStats>() == 4usize);
        96usize
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(packed(4))]
#[repr(C)]
pub struct RtnlLinkStats64 {
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
    pub rx_dropped: u64,
    pub tx_dropped: u64,
    pub multicast: u64,
    pub collisions: u64,
    pub rx_length_errors: u64,
    pub rx_over_errors: u64,
    pub rx_crc_errors: u64,
    pub rx_frame_errors: u64,
    pub rx_fifo_errors: u64,
    pub rx_missed_errors: u64,
    pub tx_aborted_errors: u64,
    pub tx_carrier_errors: u64,
    pub tx_fifo_errors: u64,
    pub tx_heartbeat_errors: u64,
    pub tx_window_errors: u64,
    pub rx_compressed: u64,
    pub tx_compressed: u64,
    pub rx_nohandler: u64,
    pub rx_otherhost_dropped: u64,
}
#[doc = "Create zero-initialized struct"]
impl Default for RtnlLinkStats64 {
    fn default() -> Self {
        Self::new()
    }
}
impl RtnlLinkStats64 {
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
    pub fn new_from_array(buf: [u8; 200usize]) -> Self {
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
    pub fn as_array(&self) -> &[u8; 200usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 200usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 200usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<RtnlLinkStats64>() == 200usize);
        const _: () = assert!(std::mem::align_of::<RtnlLinkStats64>() == 4usize);
        200usize
    }
}
impl std::fmt::Debug for RtnlLinkStats64 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("RtnlLinkStats64")
            .field("rx_packets", &{ self.rx_packets })
            .field("tx_packets", &{ self.tx_packets })
            .field("rx_bytes", &{ self.rx_bytes })
            .field("tx_bytes", &{ self.tx_bytes })
            .field("rx_errors", &{ self.rx_errors })
            .field("tx_errors", &{ self.tx_errors })
            .field("rx_dropped", &{ self.rx_dropped })
            .field("tx_dropped", &{ self.tx_dropped })
            .field("multicast", &{ self.multicast })
            .field("collisions", &{ self.collisions })
            .field("rx_length_errors", &{ self.rx_length_errors })
            .field("rx_over_errors", &{ self.rx_over_errors })
            .field("rx_crc_errors", &{ self.rx_crc_errors })
            .field("rx_frame_errors", &{ self.rx_frame_errors })
            .field("rx_fifo_errors", &{ self.rx_fifo_errors })
            .field("rx_missed_errors", &{ self.rx_missed_errors })
            .field("tx_aborted_errors", &{ self.tx_aborted_errors })
            .field("tx_carrier_errors", &{ self.tx_carrier_errors })
            .field("tx_fifo_errors", &{ self.tx_fifo_errors })
            .field("tx_heartbeat_errors", &{ self.tx_heartbeat_errors })
            .field("tx_window_errors", &{ self.tx_window_errors })
            .field("rx_compressed", &{ self.rx_compressed })
            .field("tx_compressed", &{ self.tx_compressed })
            .field("rx_nohandler", &{ self.rx_nohandler })
            .field("rx_otherhost_dropped", &{ self.rx_otherhost_dropped })
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(packed(4))]
#[repr(C)]
pub struct RtnlLinkIfmap {
    pub mem_start: u64,
    pub mem_end: u64,
    pub base_addr: u64,
    pub irq: u16,
    pub dma: u8,
    pub port: u8,
    pub _pad_28: [u8; 4usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for RtnlLinkIfmap {
    fn default() -> Self {
        Self::new()
    }
}
impl RtnlLinkIfmap {
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
        const _: () = assert!(std::mem::size_of::<RtnlLinkIfmap>() == 32usize);
        const _: () = assert!(std::mem::align_of::<RtnlLinkIfmap>() == 4usize);
        32usize
    }
}
impl std::fmt::Debug for RtnlLinkIfmap {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("RtnlLinkIfmap")
            .field("mem_start", &{ self.mem_start })
            .field("mem_end", &{ self.mem_end })
            .field("base_addr", &{ self.base_addr })
            .field("irq", &self.irq)
            .field("dma", &self.dma)
            .field("port", &self.port)
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct BrBooloptMulti {
    #[doc = "Associated type: [`BrBooloptId`] (enum)"]
    pub optval: u32,
    #[doc = "Associated type: [`BrBooloptId`] (enum)"]
    pub optmask: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for BrBooloptMulti {
    fn default() -> Self {
        Self::new()
    }
}
impl BrBooloptMulti {
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
        const _: () = assert!(std::mem::size_of::<BrBooloptMulti>() == 8usize);
        const _: () = assert!(std::mem::align_of::<BrBooloptMulti>() == 4usize);
        8usize
    }
}
impl std::fmt::Debug for BrBooloptMulti {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("BrBooloptMulti")
            .field(
                "optval",
                &FormatFlags(self.optval.into(), BrBooloptId::from_value),
            )
            .field(
                "optmask",
                &FormatFlags(self.optmask.into(), BrBooloptId::from_value),
            )
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct IfStatsMsg {
    pub family: u8,
    pub _pad: [u8; 3usize],
    pub ifindex: u32,
    pub filter_mask: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for IfStatsMsg {
    fn default() -> Self {
        Self::new()
    }
}
impl IfStatsMsg {
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
        const _: () = assert!(std::mem::size_of::<IfStatsMsg>() == 12usize);
        const _: () = assert!(std::mem::align_of::<IfStatsMsg>() == 4usize);
        12usize
    }
}
impl std::fmt::Debug for IfStatsMsg {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("IfStatsMsg")
            .field("family", &self.family)
            .field("ifindex", &self.ifindex)
            .field("filter_mask", &self.filter_mask)
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct IflaVlanFlags {
    #[doc = "Associated type: [`VlanFlags`] (1 bit per enumeration)"]
    pub flags: u32,
    pub mask: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for IflaVlanFlags {
    fn default() -> Self {
        Self::new()
    }
}
impl IflaVlanFlags {
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
        const _: () = assert!(std::mem::size_of::<IflaVlanFlags>() == 8usize);
        const _: () = assert!(std::mem::align_of::<IflaVlanFlags>() == 4usize);
        8usize
    }
}
impl std::fmt::Debug for IflaVlanFlags {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("IflaVlanFlags")
            .field(
                "flags",
                &FormatFlags(self.flags.into(), VlanFlags::from_value),
            )
            .field("mask", &FormatHex(self.mask.to_ne_bytes()))
            .finish()
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct IflaVlanQosMapping {
    pub from: u32,
    pub to: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for IflaVlanQosMapping {
    fn default() -> Self {
        Self::new()
    }
}
impl IflaVlanQosMapping {
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
        const _: () = assert!(std::mem::size_of::<IflaVlanQosMapping>() == 8usize);
        const _: () = assert!(std::mem::align_of::<IflaVlanQosMapping>() == 4usize);
        8usize
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct IflaGenevePortRange {
    pub _low_be: u16,
    pub _high_be: u16,
}
#[doc = "Create zero-initialized struct"]
impl Default for IflaGenevePortRange {
    fn default() -> Self {
        Self::new()
    }
}
impl IflaGenevePortRange {
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
        const _: () = assert!(std::mem::size_of::<IflaGenevePortRange>() == 4usize);
        const _: () = assert!(std::mem::align_of::<IflaGenevePortRange>() == 2usize);
        4usize
    }
    pub fn low(&self) -> u16 {
        u16::from_be(self._low_be)
    }
    pub fn set_low(&mut self, value: u16) {
        self._low_be = value.to_be();
    }
    pub fn high(&self) -> u16 {
        u16::from_be(self._high_be)
    }
    pub fn set_high(&mut self, value: u16) {
        self._high_be = value.to_be();
    }
}
impl std::fmt::Debug for IflaGenevePortRange {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("IflaGenevePortRange")
            .field("low", &self.low())
            .field("high", &self.high())
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct IflaVfMac {
    pub vf: u32,
    pub mac: [u8; 32usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for IflaVfMac {
    fn default() -> Self {
        Self::new()
    }
}
impl IflaVfMac {
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
    pub fn new_from_array(buf: [u8; 36usize]) -> Self {
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
    pub fn as_array(&self) -> &[u8; 36usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 36usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 36usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<IflaVfMac>() == 36usize);
        const _: () = assert!(std::mem::align_of::<IflaVfMac>() == 4usize);
        36usize
    }
}
impl std::fmt::Debug for IflaVfMac {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("IflaVfMac")
            .field("vf", &self.vf)
            .field("mac", &FormatHexdump(self.mac.as_slice()))
            .finish()
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct IflaVfVlan {
    pub vf: u32,
    pub vlan: u32,
    pub qos: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for IflaVfVlan {
    fn default() -> Self {
        Self::new()
    }
}
impl IflaVfVlan {
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
        const _: () = assert!(std::mem::size_of::<IflaVfVlan>() == 12usize);
        const _: () = assert!(std::mem::align_of::<IflaVfVlan>() == 4usize);
        12usize
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct IflaVfTxRate {
    pub vf: u32,
    pub rate: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for IflaVfTxRate {
    fn default() -> Self {
        Self::new()
    }
}
impl IflaVfTxRate {
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
        const _: () = assert!(std::mem::size_of::<IflaVfTxRate>() == 8usize);
        const _: () = assert!(std::mem::align_of::<IflaVfTxRate>() == 4usize);
        8usize
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct IflaVfSpoofchk {
    pub vf: u32,
    pub setting: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for IflaVfSpoofchk {
    fn default() -> Self {
        Self::new()
    }
}
impl IflaVfSpoofchk {
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
        const _: () = assert!(std::mem::size_of::<IflaVfSpoofchk>() == 8usize);
        const _: () = assert!(std::mem::align_of::<IflaVfSpoofchk>() == 4usize);
        8usize
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct IflaVfLinkState {
    pub vf: u32,
    #[doc = "Associated type: [`IflaVfLinkStateEnum`] (enum)"]
    pub link_state: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for IflaVfLinkState {
    fn default() -> Self {
        Self::new()
    }
}
impl IflaVfLinkState {
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
        const _: () = assert!(std::mem::size_of::<IflaVfLinkState>() == 8usize);
        const _: () = assert!(std::mem::align_of::<IflaVfLinkState>() == 4usize);
        8usize
    }
}
impl std::fmt::Debug for IflaVfLinkState {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("IflaVfLinkState")
            .field("vf", &self.vf)
            .field(
                "link_state",
                &FormatEnum(self.link_state.into(), IflaVfLinkStateEnum::from_value),
            )
            .finish()
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct IflaVfRate {
    pub vf: u32,
    pub min_tx_rate: u32,
    pub max_tx_rate: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for IflaVfRate {
    fn default() -> Self {
        Self::new()
    }
}
impl IflaVfRate {
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
        const _: () = assert!(std::mem::size_of::<IflaVfRate>() == 12usize);
        const _: () = assert!(std::mem::align_of::<IflaVfRate>() == 4usize);
        12usize
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct IflaVfRssQueryEn {
    pub vf: u32,
    pub setting: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for IflaVfRssQueryEn {
    fn default() -> Self {
        Self::new()
    }
}
impl IflaVfRssQueryEn {
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
        const _: () = assert!(std::mem::size_of::<IflaVfRssQueryEn>() == 8usize);
        const _: () = assert!(std::mem::align_of::<IflaVfRssQueryEn>() == 4usize);
        8usize
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct IflaVfTrust {
    pub vf: u32,
    pub setting: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for IflaVfTrust {
    fn default() -> Self {
        Self::new()
    }
}
impl IflaVfTrust {
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
        const _: () = assert!(std::mem::size_of::<IflaVfTrust>() == 8usize);
        const _: () = assert!(std::mem::align_of::<IflaVfTrust>() == 4usize);
        8usize
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(packed(4))]
#[repr(C)]
pub struct IflaVfGuid {
    pub vf: u32,
    pub _pad_4: [u8; 4usize],
    pub guid: u64,
}
#[doc = "Create zero-initialized struct"]
impl Default for IflaVfGuid {
    fn default() -> Self {
        Self::new()
    }
}
impl IflaVfGuid {
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
        const _: () = assert!(std::mem::size_of::<IflaVfGuid>() == 16usize);
        const _: () = assert!(std::mem::align_of::<IflaVfGuid>() == 4usize);
        16usize
    }
}
impl std::fmt::Debug for IflaVfGuid {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("IflaVfGuid")
            .field("vf", &self.vf)
            .field("guid", &{ self.guid })
            .finish()
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct IflaVfVlanInfo {
    pub vf: u32,
    pub vlan: u32,
    pub qos: u32,
    pub vlan_proto: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for IflaVfVlanInfo {
    fn default() -> Self {
        Self::new()
    }
}
impl IflaVfVlanInfo {
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
        const _: () = assert!(std::mem::size_of::<IflaVfVlanInfo>() == 16usize);
        const _: () = assert!(std::mem::align_of::<IflaVfVlanInfo>() == 4usize);
        16usize
    }
}
#[derive(Clone)]
pub enum LinkAttrs<'a> {
    Address(&'a [u8]),
    Broadcast(&'a [u8]),
    Ifname(&'a CStr),
    Mtu(u32),
    Link(u32),
    Qdisc(&'a CStr),
    Stats(RtnlLinkStats),
    Master(u32),
    #[doc = "struct iw_event\n"]
    Wireless(&'a [u8]),
    #[doc = "A nest of ifla6-attrs or linkinfo-brport-attrs\n"]
    Protinfo(&'a [u8]),
    Txqlen(u32),
    Map(RtnlLinkIfmap),
    Weight(u32),
    Operstate(u8),
    Linkmode(u8),
    Linkinfo(IterableLinkinfoAttrs<'a>),
    NetNsPid(u32),
    Ifalias(&'a CStr),
    NumVf(u32),
    VfinfoList(IterableVfinfoListAttrs<'a>),
    Stats64(RtnlLinkStats64),
    VfPorts(IterableVfPortsAttrs<'a>),
    PortSelf(IterablePortSelfAttrs<'a>),
    AfSpec(IterableAfSpecAttrs<'a>),
    Group(u32),
    NetNsFd(u32),
    #[doc = "Associated type: [`RtextFilter`] (1 bit per enumeration)"]
    ExtMask(u32),
    Promiscuity(u32),
    NumTxQueues(u32),
    NumRxQueues(u32),
    Carrier(u8),
    PhysPortId(&'a [u8]),
    CarrierChanges(u32),
    PhysSwitchId(&'a [u8]),
    LinkNetnsid(i32),
    PhysPortName(&'a CStr),
    ProtoDown(u8),
    GsoMaxSegs(u32),
    GsoMaxSize(u32),
    Pad(&'a [u8]),
    Xdp(IterableXdpAttrs<'a>),
    Event(u32),
    NewNetnsid(i32),
    TargetNetnsid(i32),
    CarrierUpCount(u32),
    CarrierDownCount(u32),
    NewIfindex(i32),
    MinMtu(u32),
    MaxMtu(u32),
    PropList(IterablePropListLinkAttrs<'a>),
    AltIfname(&'a CStr),
    PermAddress(&'a [u8]),
    ProtoDownReason(&'a CStr),
    ParentDevName(&'a CStr),
    ParentDevBusName(&'a CStr),
    GroMaxSize(u32),
    TsoMaxSize(u32),
    TsoMaxSegs(u32),
    Allmulti(u32),
    DevlinkPort(&'a [u8]),
    GsoIpv4MaxSize(u32),
    GroIpv4MaxSize(u32),
    DpllPin(IterableLinkDpllPinAttrs<'a>),
    #[doc = "EDT offload horizon supported by the device (in nsec).\n"]
    MaxPacingOffloadHorizon(u32),
    NetnsImmutable(u8),
    Headroom(u16),
    Tailroom(u16),
}
impl<'a> IterableLinkAttrs<'a> {
    pub fn get_address(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Address(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Address",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_broadcast(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Broadcast(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Broadcast",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ifname(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Ifname(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Ifname",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mtu(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Mtu(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Mtu",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_link(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Link(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Link",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_qdisc(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Qdisc(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Qdisc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_stats(&self) -> Result<RtnlLinkStats, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Stats(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Stats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_master(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Master(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Master",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "struct iw_event\n"]
    pub fn get_wireless(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Wireless(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Wireless",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "A nest of ifla6-attrs or linkinfo-brport-attrs\n"]
    pub fn get_protinfo(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Protinfo(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Protinfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_txqlen(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Txqlen(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Txqlen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_map(&self) -> Result<RtnlLinkIfmap, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Map(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Map",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_weight(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Weight(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Weight",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_operstate(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Operstate(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Operstate",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_linkmode(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Linkmode(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Linkmode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_linkinfo(&self) -> Result<IterableLinkinfoAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Linkinfo(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Linkinfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_net_ns_pid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::NetNsPid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "NetNsPid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ifalias(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Ifalias(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Ifalias",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_num_vf(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::NumVf(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "NumVf",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vfinfo_list(&self) -> Result<IterableVfinfoListAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::VfinfoList(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "VfinfoList",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_stats64(&self) -> Result<RtnlLinkStats64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Stats64(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Stats64",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vf_ports(&self) -> Result<IterableVfPortsAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::VfPorts(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "VfPorts",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_port_self(&self) -> Result<IterablePortSelfAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::PortSelf(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "PortSelf",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_af_spec(&self) -> Result<IterableAfSpecAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::AfSpec(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "AfSpec",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_group(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Group(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Group",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_net_ns_fd(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::NetNsFd(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "NetNsFd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`RtextFilter`] (1 bit per enumeration)"]
    pub fn get_ext_mask(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::ExtMask(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "ExtMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_promiscuity(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Promiscuity(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Promiscuity",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_num_tx_queues(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::NumTxQueues(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "NumTxQueues",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_num_rx_queues(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::NumRxQueues(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "NumRxQueues",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_carrier(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Carrier(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Carrier",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_phys_port_id(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::PhysPortId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "PhysPortId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_carrier_changes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::CarrierChanges(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "CarrierChanges",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_phys_switch_id(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::PhysSwitchId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "PhysSwitchId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_link_netnsid(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::LinkNetnsid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "LinkNetnsid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_phys_port_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::PhysPortName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "PhysPortName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_proto_down(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::ProtoDown(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "ProtoDown",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_gso_max_segs(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::GsoMaxSegs(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "GsoMaxSegs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_gso_max_size(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::GsoMaxSize(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "GsoMaxSize",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_xdp(&self) -> Result<IterableXdpAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Xdp(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Xdp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_event(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Event(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Event",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_new_netnsid(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::NewNetnsid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "NewNetnsid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_target_netnsid(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::TargetNetnsid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "TargetNetnsid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_carrier_up_count(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::CarrierUpCount(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "CarrierUpCount",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_carrier_down_count(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::CarrierDownCount(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "CarrierDownCount",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_new_ifindex(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::NewIfindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "NewIfindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_min_mtu(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::MinMtu(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "MinMtu",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_mtu(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::MaxMtu(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "MaxMtu",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_prop_list(&self) -> Result<IterablePropListLinkAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::PropList(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "PropList",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_alt_ifname(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::AltIfname(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "AltIfname",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_perm_address(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::PermAddress(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "PermAddress",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_proto_down_reason(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::ProtoDownReason(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "ProtoDownReason",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_parent_dev_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::ParentDevName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "ParentDevName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_parent_dev_bus_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::ParentDevBusName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "ParentDevBusName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_gro_max_size(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::GroMaxSize(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "GroMaxSize",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tso_max_size(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::TsoMaxSize(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "TsoMaxSize",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tso_max_segs(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::TsoMaxSegs(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "TsoMaxSegs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_allmulti(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Allmulti(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Allmulti",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_devlink_port(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::DevlinkPort(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "DevlinkPort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_gso_ipv4_max_size(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::GsoIpv4MaxSize(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "GsoIpv4MaxSize",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_gro_ipv4_max_size(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::GroIpv4MaxSize(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "GroIpv4MaxSize",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dpll_pin(&self) -> Result<IterableLinkDpllPinAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::DpllPin(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "DpllPin",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "EDT offload horizon supported by the device (in nsec).\n"]
    pub fn get_max_pacing_offload_horizon(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::MaxPacingOffloadHorizon(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "MaxPacingOffloadHorizon",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_netns_immutable(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::NetnsImmutable(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "NetnsImmutable",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_headroom(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Headroom(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Headroom",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tailroom(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkAttrs::Tailroom(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkAttrs",
            "Tailroom",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LinkAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLinkAttrs<'a> {
        IterableLinkAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Address",
            2u16 => "Broadcast",
            3u16 => "Ifname",
            4u16 => "Mtu",
            5u16 => "Link",
            6u16 => "Qdisc",
            7u16 => "Stats",
            8u16 => "Cost",
            9u16 => "Priority",
            10u16 => "Master",
            11u16 => "Wireless",
            12u16 => "Protinfo",
            13u16 => "Txqlen",
            14u16 => "Map",
            15u16 => "Weight",
            16u16 => "Operstate",
            17u16 => "Linkmode",
            18u16 => "Linkinfo",
            19u16 => "NetNsPid",
            20u16 => "Ifalias",
            21u16 => "NumVf",
            22u16 => "VfinfoList",
            23u16 => "Stats64",
            24u16 => "VfPorts",
            25u16 => "PortSelf",
            26u16 => "AfSpec",
            27u16 => "Group",
            28u16 => "NetNsFd",
            29u16 => "ExtMask",
            30u16 => "Promiscuity",
            31u16 => "NumTxQueues",
            32u16 => "NumRxQueues",
            33u16 => "Carrier",
            34u16 => "PhysPortId",
            35u16 => "CarrierChanges",
            36u16 => "PhysSwitchId",
            37u16 => "LinkNetnsid",
            38u16 => "PhysPortName",
            39u16 => "ProtoDown",
            40u16 => "GsoMaxSegs",
            41u16 => "GsoMaxSize",
            42u16 => "Pad",
            43u16 => "Xdp",
            44u16 => "Event",
            45u16 => "NewNetnsid",
            46u16 => "TargetNetnsid",
            47u16 => "CarrierUpCount",
            48u16 => "CarrierDownCount",
            49u16 => "NewIfindex",
            50u16 => "MinMtu",
            51u16 => "MaxMtu",
            52u16 => "PropList",
            53u16 => "AltIfname",
            54u16 => "PermAddress",
            55u16 => "ProtoDownReason",
            56u16 => "ParentDevName",
            57u16 => "ParentDevBusName",
            58u16 => "GroMaxSize",
            59u16 => "TsoMaxSize",
            60u16 => "TsoMaxSegs",
            61u16 => "Allmulti",
            62u16 => "DevlinkPort",
            63u16 => "GsoIpv4MaxSize",
            64u16 => "GroIpv4MaxSize",
            65u16 => "DpllPin",
            66u16 => "MaxPacingOffloadHorizon",
            67u16 => "NetnsImmutable",
            68u16 => "Headroom",
            69u16 => "Tailroom",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLinkAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLinkAttrs<'a> {
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
impl<'a> Iterator for IterableLinkAttrs<'a> {
    type Item = Result<LinkAttrs<'a>, ErrorContext>;
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
                1u16 => LinkAttrs::Address({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => LinkAttrs::Broadcast({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => LinkAttrs::Ifname({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => LinkAttrs::Mtu({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => LinkAttrs::Link({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => LinkAttrs::Qdisc({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => LinkAttrs::Stats({
                    let res = Some(RtnlLinkStats::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => LinkAttrs::Master({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => LinkAttrs::Wireless({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => LinkAttrs::Protinfo({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => LinkAttrs::Txqlen({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => LinkAttrs::Map({
                    let res = Some(RtnlLinkIfmap::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => LinkAttrs::Weight({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => LinkAttrs::Operstate({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => LinkAttrs::Linkmode({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => LinkAttrs::Linkinfo({
                    let res = Some(IterableLinkinfoAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => LinkAttrs::NetNsPid({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => LinkAttrs::Ifalias({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => LinkAttrs::NumVf({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => LinkAttrs::VfinfoList({
                    let res = Some(IterableVfinfoListAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => LinkAttrs::Stats64({
                    let res = Some(RtnlLinkStats64::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => LinkAttrs::VfPorts({
                    let res = Some(IterableVfPortsAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                25u16 => LinkAttrs::PortSelf({
                    let res = Some(IterablePortSelfAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                26u16 => LinkAttrs::AfSpec({
                    let res = Some(IterableAfSpecAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                27u16 => LinkAttrs::Group({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                28u16 => LinkAttrs::NetNsFd({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                29u16 => LinkAttrs::ExtMask({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                30u16 => LinkAttrs::Promiscuity({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                31u16 => LinkAttrs::NumTxQueues({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                32u16 => LinkAttrs::NumRxQueues({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                33u16 => LinkAttrs::Carrier({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                34u16 => LinkAttrs::PhysPortId({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                35u16 => LinkAttrs::CarrierChanges({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                36u16 => LinkAttrs::PhysSwitchId({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                37u16 => LinkAttrs::LinkNetnsid({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                38u16 => LinkAttrs::PhysPortName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                39u16 => LinkAttrs::ProtoDown({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                40u16 => LinkAttrs::GsoMaxSegs({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                41u16 => LinkAttrs::GsoMaxSize({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                42u16 => LinkAttrs::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                43u16 => LinkAttrs::Xdp({
                    let res = Some(IterableXdpAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                44u16 => LinkAttrs::Event({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                45u16 => LinkAttrs::NewNetnsid({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                46u16 => LinkAttrs::TargetNetnsid({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                47u16 => LinkAttrs::CarrierUpCount({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                48u16 => LinkAttrs::CarrierDownCount({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                49u16 => LinkAttrs::NewIfindex({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                50u16 => LinkAttrs::MinMtu({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                51u16 => LinkAttrs::MaxMtu({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                52u16 => LinkAttrs::PropList({
                    let res = Some(IterablePropListLinkAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                53u16 => LinkAttrs::AltIfname({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                54u16 => LinkAttrs::PermAddress({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                55u16 => LinkAttrs::ProtoDownReason({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                56u16 => LinkAttrs::ParentDevName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                57u16 => LinkAttrs::ParentDevBusName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                58u16 => LinkAttrs::GroMaxSize({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                59u16 => LinkAttrs::TsoMaxSize({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                60u16 => LinkAttrs::TsoMaxSegs({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                61u16 => LinkAttrs::Allmulti({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                62u16 => LinkAttrs::DevlinkPort({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                63u16 => LinkAttrs::GsoIpv4MaxSize({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                64u16 => LinkAttrs::GroIpv4MaxSize({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                65u16 => LinkAttrs::DpllPin({
                    let res = Some(IterableLinkDpllPinAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                66u16 => LinkAttrs::MaxPacingOffloadHorizon({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                67u16 => LinkAttrs::NetnsImmutable({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                68u16 => LinkAttrs::Headroom({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                69u16 => LinkAttrs::Tailroom({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "LinkAttrs",
            r#type.and_then(|t| LinkAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableLinkAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LinkAttrs");
        let mut iter = IterableLinkAttrs::with_loc(&[], self.orig_loc);
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
                LinkAttrs::Address(val) => fmt.field("Address", &FormatMac(val)),
                LinkAttrs::Broadcast(val) => fmt.field("Broadcast", &FormatMac(val)),
                LinkAttrs::Ifname(val) => fmt.field("Ifname", &val),
                LinkAttrs::Mtu(val) => fmt.field("Mtu", &val),
                LinkAttrs::Link(val) => fmt.field("Link", &val),
                LinkAttrs::Qdisc(val) => fmt.field("Qdisc", &val),
                LinkAttrs::Stats(val) => fmt.field("Stats", &val),
                LinkAttrs::Master(val) => fmt.field("Master", &val),
                LinkAttrs::Wireless(val) => fmt.field("Wireless", &FormatHexdump(val)),
                LinkAttrs::Protinfo(val) => fmt.field("Protinfo", &FormatHexdump(val)),
                LinkAttrs::Txqlen(val) => fmt.field("Txqlen", &val),
                LinkAttrs::Map(val) => fmt.field("Map", &val),
                LinkAttrs::Weight(val) => fmt.field("Weight", &val),
                LinkAttrs::Operstate(val) => fmt.field("Operstate", &val),
                LinkAttrs::Linkmode(val) => fmt.field("Linkmode", &val),
                LinkAttrs::Linkinfo(val) => fmt.field("Linkinfo", &val),
                LinkAttrs::NetNsPid(val) => fmt.field("NetNsPid", &val),
                LinkAttrs::Ifalias(val) => fmt.field("Ifalias", &val),
                LinkAttrs::NumVf(val) => fmt.field("NumVf", &val),
                LinkAttrs::VfinfoList(val) => fmt.field("VfinfoList", &val),
                LinkAttrs::Stats64(val) => fmt.field("Stats64", &val),
                LinkAttrs::VfPorts(val) => fmt.field("VfPorts", &val),
                LinkAttrs::PortSelf(val) => fmt.field("PortSelf", &val),
                LinkAttrs::AfSpec(val) => fmt.field("AfSpec", &val),
                LinkAttrs::Group(val) => fmt.field("Group", &val),
                LinkAttrs::NetNsFd(val) => fmt.field("NetNsFd", &val),
                LinkAttrs::ExtMask(val) => {
                    fmt.field("ExtMask", &FormatFlags(val.into(), RtextFilter::from_value))
                }
                LinkAttrs::Promiscuity(val) => fmt.field("Promiscuity", &val),
                LinkAttrs::NumTxQueues(val) => fmt.field("NumTxQueues", &val),
                LinkAttrs::NumRxQueues(val) => fmt.field("NumRxQueues", &val),
                LinkAttrs::Carrier(val) => fmt.field("Carrier", &val),
                LinkAttrs::PhysPortId(val) => fmt.field("PhysPortId", &FormatHexdump(val)),
                LinkAttrs::CarrierChanges(val) => fmt.field("CarrierChanges", &val),
                LinkAttrs::PhysSwitchId(val) => fmt.field("PhysSwitchId", &FormatHexdump(val)),
                LinkAttrs::LinkNetnsid(val) => fmt.field("LinkNetnsid", &val),
                LinkAttrs::PhysPortName(val) => fmt.field("PhysPortName", &val),
                LinkAttrs::ProtoDown(val) => fmt.field("ProtoDown", &val),
                LinkAttrs::GsoMaxSegs(val) => fmt.field("GsoMaxSegs", &val),
                LinkAttrs::GsoMaxSize(val) => fmt.field("GsoMaxSize", &val),
                LinkAttrs::Pad(val) => fmt.field("Pad", &val),
                LinkAttrs::Xdp(val) => fmt.field("Xdp", &val),
                LinkAttrs::Event(val) => fmt.field("Event", &val),
                LinkAttrs::NewNetnsid(val) => fmt.field("NewNetnsid", &val),
                LinkAttrs::TargetNetnsid(val) => fmt.field("TargetNetnsid", &val),
                LinkAttrs::CarrierUpCount(val) => fmt.field("CarrierUpCount", &val),
                LinkAttrs::CarrierDownCount(val) => fmt.field("CarrierDownCount", &val),
                LinkAttrs::NewIfindex(val) => fmt.field("NewIfindex", &val),
                LinkAttrs::MinMtu(val) => fmt.field("MinMtu", &val),
                LinkAttrs::MaxMtu(val) => fmt.field("MaxMtu", &val),
                LinkAttrs::PropList(val) => fmt.field("PropList", &val),
                LinkAttrs::AltIfname(val) => fmt.field("AltIfname", &val),
                LinkAttrs::PermAddress(val) => fmt.field("PermAddress", &FormatMac(val)),
                LinkAttrs::ProtoDownReason(val) => fmt.field("ProtoDownReason", &val),
                LinkAttrs::ParentDevName(val) => fmt.field("ParentDevName", &val),
                LinkAttrs::ParentDevBusName(val) => fmt.field("ParentDevBusName", &val),
                LinkAttrs::GroMaxSize(val) => fmt.field("GroMaxSize", &val),
                LinkAttrs::TsoMaxSize(val) => fmt.field("TsoMaxSize", &val),
                LinkAttrs::TsoMaxSegs(val) => fmt.field("TsoMaxSegs", &val),
                LinkAttrs::Allmulti(val) => fmt.field("Allmulti", &val),
                LinkAttrs::DevlinkPort(val) => fmt.field("DevlinkPort", &FormatHexdump(val)),
                LinkAttrs::GsoIpv4MaxSize(val) => fmt.field("GsoIpv4MaxSize", &val),
                LinkAttrs::GroIpv4MaxSize(val) => fmt.field("GroIpv4MaxSize", &val),
                LinkAttrs::DpllPin(val) => fmt.field("DpllPin", &val),
                LinkAttrs::MaxPacingOffloadHorizon(val) => {
                    fmt.field("MaxPacingOffloadHorizon", &val)
                }
                LinkAttrs::NetnsImmutable(val) => fmt.field("NetnsImmutable", &val),
                LinkAttrs::Headroom(val) => fmt.field("Headroom", &val),
                LinkAttrs::Tailroom(val) => fmt.field("Tailroom", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableLinkAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LinkAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| LinkAttrs::attr_from_type(t)),
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
                LinkAttrs::Address(val) => {
                    if last_off == offset {
                        stack.push(("Address", last_off));
                        break;
                    }
                }
                LinkAttrs::Broadcast(val) => {
                    if last_off == offset {
                        stack.push(("Broadcast", last_off));
                        break;
                    }
                }
                LinkAttrs::Ifname(val) => {
                    if last_off == offset {
                        stack.push(("Ifname", last_off));
                        break;
                    }
                }
                LinkAttrs::Mtu(val) => {
                    if last_off == offset {
                        stack.push(("Mtu", last_off));
                        break;
                    }
                }
                LinkAttrs::Link(val) => {
                    if last_off == offset {
                        stack.push(("Link", last_off));
                        break;
                    }
                }
                LinkAttrs::Qdisc(val) => {
                    if last_off == offset {
                        stack.push(("Qdisc", last_off));
                        break;
                    }
                }
                LinkAttrs::Stats(val) => {
                    if last_off == offset {
                        stack.push(("Stats", last_off));
                        break;
                    }
                }
                LinkAttrs::Master(val) => {
                    if last_off == offset {
                        stack.push(("Master", last_off));
                        break;
                    }
                }
                LinkAttrs::Wireless(val) => {
                    if last_off == offset {
                        stack.push(("Wireless", last_off));
                        break;
                    }
                }
                LinkAttrs::Protinfo(val) => {
                    if last_off == offset {
                        stack.push(("Protinfo", last_off));
                        break;
                    }
                }
                LinkAttrs::Txqlen(val) => {
                    if last_off == offset {
                        stack.push(("Txqlen", last_off));
                        break;
                    }
                }
                LinkAttrs::Map(val) => {
                    if last_off == offset {
                        stack.push(("Map", last_off));
                        break;
                    }
                }
                LinkAttrs::Weight(val) => {
                    if last_off == offset {
                        stack.push(("Weight", last_off));
                        break;
                    }
                }
                LinkAttrs::Operstate(val) => {
                    if last_off == offset {
                        stack.push(("Operstate", last_off));
                        break;
                    }
                }
                LinkAttrs::Linkmode(val) => {
                    if last_off == offset {
                        stack.push(("Linkmode", last_off));
                        break;
                    }
                }
                LinkAttrs::Linkinfo(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                LinkAttrs::NetNsPid(val) => {
                    if last_off == offset {
                        stack.push(("NetNsPid", last_off));
                        break;
                    }
                }
                LinkAttrs::Ifalias(val) => {
                    if last_off == offset {
                        stack.push(("Ifalias", last_off));
                        break;
                    }
                }
                LinkAttrs::NumVf(val) => {
                    if last_off == offset {
                        stack.push(("NumVf", last_off));
                        break;
                    }
                }
                LinkAttrs::VfinfoList(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                LinkAttrs::Stats64(val) => {
                    if last_off == offset {
                        stack.push(("Stats64", last_off));
                        break;
                    }
                }
                LinkAttrs::VfPorts(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                LinkAttrs::PortSelf(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                LinkAttrs::AfSpec(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                LinkAttrs::Group(val) => {
                    if last_off == offset {
                        stack.push(("Group", last_off));
                        break;
                    }
                }
                LinkAttrs::NetNsFd(val) => {
                    if last_off == offset {
                        stack.push(("NetNsFd", last_off));
                        break;
                    }
                }
                LinkAttrs::ExtMask(val) => {
                    if last_off == offset {
                        stack.push(("ExtMask", last_off));
                        break;
                    }
                }
                LinkAttrs::Promiscuity(val) => {
                    if last_off == offset {
                        stack.push(("Promiscuity", last_off));
                        break;
                    }
                }
                LinkAttrs::NumTxQueues(val) => {
                    if last_off == offset {
                        stack.push(("NumTxQueues", last_off));
                        break;
                    }
                }
                LinkAttrs::NumRxQueues(val) => {
                    if last_off == offset {
                        stack.push(("NumRxQueues", last_off));
                        break;
                    }
                }
                LinkAttrs::Carrier(val) => {
                    if last_off == offset {
                        stack.push(("Carrier", last_off));
                        break;
                    }
                }
                LinkAttrs::PhysPortId(val) => {
                    if last_off == offset {
                        stack.push(("PhysPortId", last_off));
                        break;
                    }
                }
                LinkAttrs::CarrierChanges(val) => {
                    if last_off == offset {
                        stack.push(("CarrierChanges", last_off));
                        break;
                    }
                }
                LinkAttrs::PhysSwitchId(val) => {
                    if last_off == offset {
                        stack.push(("PhysSwitchId", last_off));
                        break;
                    }
                }
                LinkAttrs::LinkNetnsid(val) => {
                    if last_off == offset {
                        stack.push(("LinkNetnsid", last_off));
                        break;
                    }
                }
                LinkAttrs::PhysPortName(val) => {
                    if last_off == offset {
                        stack.push(("PhysPortName", last_off));
                        break;
                    }
                }
                LinkAttrs::ProtoDown(val) => {
                    if last_off == offset {
                        stack.push(("ProtoDown", last_off));
                        break;
                    }
                }
                LinkAttrs::GsoMaxSegs(val) => {
                    if last_off == offset {
                        stack.push(("GsoMaxSegs", last_off));
                        break;
                    }
                }
                LinkAttrs::GsoMaxSize(val) => {
                    if last_off == offset {
                        stack.push(("GsoMaxSize", last_off));
                        break;
                    }
                }
                LinkAttrs::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                LinkAttrs::Xdp(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                LinkAttrs::Event(val) => {
                    if last_off == offset {
                        stack.push(("Event", last_off));
                        break;
                    }
                }
                LinkAttrs::NewNetnsid(val) => {
                    if last_off == offset {
                        stack.push(("NewNetnsid", last_off));
                        break;
                    }
                }
                LinkAttrs::TargetNetnsid(val) => {
                    if last_off == offset {
                        stack.push(("TargetNetnsid", last_off));
                        break;
                    }
                }
                LinkAttrs::CarrierUpCount(val) => {
                    if last_off == offset {
                        stack.push(("CarrierUpCount", last_off));
                        break;
                    }
                }
                LinkAttrs::CarrierDownCount(val) => {
                    if last_off == offset {
                        stack.push(("CarrierDownCount", last_off));
                        break;
                    }
                }
                LinkAttrs::NewIfindex(val) => {
                    if last_off == offset {
                        stack.push(("NewIfindex", last_off));
                        break;
                    }
                }
                LinkAttrs::MinMtu(val) => {
                    if last_off == offset {
                        stack.push(("MinMtu", last_off));
                        break;
                    }
                }
                LinkAttrs::MaxMtu(val) => {
                    if last_off == offset {
                        stack.push(("MaxMtu", last_off));
                        break;
                    }
                }
                LinkAttrs::PropList(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                LinkAttrs::AltIfname(val) => {
                    if last_off == offset {
                        stack.push(("AltIfname", last_off));
                        break;
                    }
                }
                LinkAttrs::PermAddress(val) => {
                    if last_off == offset {
                        stack.push(("PermAddress", last_off));
                        break;
                    }
                }
                LinkAttrs::ProtoDownReason(val) => {
                    if last_off == offset {
                        stack.push(("ProtoDownReason", last_off));
                        break;
                    }
                }
                LinkAttrs::ParentDevName(val) => {
                    if last_off == offset {
                        stack.push(("ParentDevName", last_off));
                        break;
                    }
                }
                LinkAttrs::ParentDevBusName(val) => {
                    if last_off == offset {
                        stack.push(("ParentDevBusName", last_off));
                        break;
                    }
                }
                LinkAttrs::GroMaxSize(val) => {
                    if last_off == offset {
                        stack.push(("GroMaxSize", last_off));
                        break;
                    }
                }
                LinkAttrs::TsoMaxSize(val) => {
                    if last_off == offset {
                        stack.push(("TsoMaxSize", last_off));
                        break;
                    }
                }
                LinkAttrs::TsoMaxSegs(val) => {
                    if last_off == offset {
                        stack.push(("TsoMaxSegs", last_off));
                        break;
                    }
                }
                LinkAttrs::Allmulti(val) => {
                    if last_off == offset {
                        stack.push(("Allmulti", last_off));
                        break;
                    }
                }
                LinkAttrs::DevlinkPort(val) => {
                    if last_off == offset {
                        stack.push(("DevlinkPort", last_off));
                        break;
                    }
                }
                LinkAttrs::GsoIpv4MaxSize(val) => {
                    if last_off == offset {
                        stack.push(("GsoIpv4MaxSize", last_off));
                        break;
                    }
                }
                LinkAttrs::GroIpv4MaxSize(val) => {
                    if last_off == offset {
                        stack.push(("GroIpv4MaxSize", last_off));
                        break;
                    }
                }
                LinkAttrs::DpllPin(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                LinkAttrs::MaxPacingOffloadHorizon(val) => {
                    if last_off == offset {
                        stack.push(("MaxPacingOffloadHorizon", last_off));
                        break;
                    }
                }
                LinkAttrs::NetnsImmutable(val) => {
                    if last_off == offset {
                        stack.push(("NetnsImmutable", last_off));
                        break;
                    }
                }
                LinkAttrs::Headroom(val) => {
                    if last_off == offset {
                        stack.push(("Headroom", last_off));
                        break;
                    }
                }
                LinkAttrs::Tailroom(val) => {
                    if last_off == offset {
                        stack.push(("Tailroom", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LinkAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum PropListLinkAttrs<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    AltIfname(&'a CStr),
}
impl<'a> IterablePropListLinkAttrs<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_alt_ifname(&self) -> MultiAttrIterable<Self, PropListLinkAttrs<'a>, &'a CStr> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let PropListLinkAttrs::AltIfname(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl PropListLinkAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePropListLinkAttrs<'a> {
        IterablePropListLinkAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        LinkAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePropListLinkAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePropListLinkAttrs<'a> {
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
impl<'a> Iterator for IterablePropListLinkAttrs<'a> {
    type Item = Result<PropListLinkAttrs<'a>, ErrorContext>;
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
                53u16 => PropListLinkAttrs::AltIfname({
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
            "PropListLinkAttrs",
            r#type.and_then(|t| PropListLinkAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterablePropListLinkAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("PropListLinkAttrs");
        let mut iter = IterablePropListLinkAttrs::with_loc(&[], self.orig_loc);
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
                PropListLinkAttrs::AltIfname(val) => fmt.field("AltIfname", &val),
            };
        }
        fmt.finish()
    }
}
impl IterablePropListLinkAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("PropListLinkAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| PropListLinkAttrs::attr_from_type(t)),
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
                PropListLinkAttrs::AltIfname(val) => {
                    if last_off == offset {
                        stack.push(("AltIfname", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("PropListLinkAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum AfSpecAttrs<'a> {
    Inet(IterableIflaAttrs<'a>),
    Inet6(IterableIfla6Attrs<'a>),
    Mctp(IterableMctpAttrs<'a>),
}
impl<'a> IterableAfSpecAttrs<'a> {
    pub fn get_inet(&self) -> Result<IterableIflaAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AfSpecAttrs::Inet(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AfSpecAttrs",
            "Inet",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_inet6(&self) -> Result<IterableIfla6Attrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AfSpecAttrs::Inet6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AfSpecAttrs",
            "Inet6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mctp(&self) -> Result<IterableMctpAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AfSpecAttrs::Mctp(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AfSpecAttrs",
            "Mctp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl AfSpecAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableAfSpecAttrs<'a> {
        IterableAfSpecAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            2u16 => "Inet",
            10u16 => "Inet6",
            45u16 => "Mctp",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableAfSpecAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableAfSpecAttrs<'a> {
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
impl<'a> Iterator for IterableAfSpecAttrs<'a> {
    type Item = Result<AfSpecAttrs<'a>, ErrorContext>;
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
                2u16 => AfSpecAttrs::Inet({
                    let res = Some(IterableIflaAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => AfSpecAttrs::Inet6({
                    let res = Some(IterableIfla6Attrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                45u16 => AfSpecAttrs::Mctp({
                    let res = Some(IterableMctpAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "AfSpecAttrs",
            r#type.and_then(|t| AfSpecAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableAfSpecAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("AfSpecAttrs");
        let mut iter = IterableAfSpecAttrs::with_loc(&[], self.orig_loc);
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
                AfSpecAttrs::Inet(val) => fmt.field("Inet", &val),
                AfSpecAttrs::Inet6(val) => fmt.field("Inet6", &val),
                AfSpecAttrs::Mctp(val) => fmt.field("Mctp", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableAfSpecAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("AfSpecAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| AfSpecAttrs::attr_from_type(t)),
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
                AfSpecAttrs::Inet(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                AfSpecAttrs::Inet6(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                AfSpecAttrs::Mctp(val) => {
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
            stack.push(("AfSpecAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum VfinfoListAttrs<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    Info(IterableVfinfoAttrs<'a>),
}
impl<'a> IterableVfinfoListAttrs<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_info(
        &self,
    ) -> MultiAttrIterable<Self, VfinfoListAttrs<'a>, IterableVfinfoAttrs<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let VfinfoListAttrs::Info(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl VfinfoListAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableVfinfoListAttrs<'a> {
        IterableVfinfoListAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Info",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableVfinfoListAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableVfinfoListAttrs<'a> {
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
impl<'a> Iterator for IterableVfinfoListAttrs<'a> {
    type Item = Result<VfinfoListAttrs<'a>, ErrorContext>;
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
                1u16 => VfinfoListAttrs::Info({
                    let res = Some(IterableVfinfoAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "VfinfoListAttrs",
            r#type.and_then(|t| VfinfoListAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableVfinfoListAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("VfinfoListAttrs");
        let mut iter = IterableVfinfoListAttrs::with_loc(&[], self.orig_loc);
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
                VfinfoListAttrs::Info(val) => fmt.field("Info", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableVfinfoListAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("VfinfoListAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| VfinfoListAttrs::attr_from_type(t)),
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
                VfinfoListAttrs::Info(val) => {
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
            stack.push(("VfinfoListAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum VfinfoAttrs<'a> {
    Mac(IflaVfMac),
    Vlan(IflaVfVlan),
    TxRate(IflaVfTxRate),
    Spoofchk(IflaVfSpoofchk),
    LinkState(IflaVfLinkState),
    Rate(IflaVfRate),
    RssQueryEn(IflaVfRssQueryEn),
    Stats(IterableVfStatsAttrs<'a>),
    Trust(IflaVfTrust),
    IbNodeGuid(IflaVfGuid),
    IbPortGuid(IflaVfGuid),
    VlanList(IterableVfVlanAttrs<'a>),
    Broadcast(&'a [u8]),
}
impl<'a> IterableVfinfoAttrs<'a> {
    pub fn get_mac(&self) -> Result<IflaVfMac, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VfinfoAttrs::Mac(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VfinfoAttrs",
            "Mac",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vlan(&self) -> Result<IflaVfVlan, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VfinfoAttrs::Vlan(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VfinfoAttrs",
            "Vlan",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tx_rate(&self) -> Result<IflaVfTxRate, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VfinfoAttrs::TxRate(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VfinfoAttrs",
            "TxRate",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_spoofchk(&self) -> Result<IflaVfSpoofchk, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VfinfoAttrs::Spoofchk(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VfinfoAttrs",
            "Spoofchk",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_link_state(&self) -> Result<IflaVfLinkState, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VfinfoAttrs::LinkState(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VfinfoAttrs",
            "LinkState",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rate(&self) -> Result<IflaVfRate, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VfinfoAttrs::Rate(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VfinfoAttrs",
            "Rate",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rss_query_en(&self) -> Result<IflaVfRssQueryEn, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VfinfoAttrs::RssQueryEn(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VfinfoAttrs",
            "RssQueryEn",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_stats(&self) -> Result<IterableVfStatsAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VfinfoAttrs::Stats(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VfinfoAttrs",
            "Stats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_trust(&self) -> Result<IflaVfTrust, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VfinfoAttrs::Trust(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VfinfoAttrs",
            "Trust",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ib_node_guid(&self) -> Result<IflaVfGuid, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VfinfoAttrs::IbNodeGuid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VfinfoAttrs",
            "IbNodeGuid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ib_port_guid(&self) -> Result<IflaVfGuid, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VfinfoAttrs::IbPortGuid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VfinfoAttrs",
            "IbPortGuid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vlan_list(&self) -> Result<IterableVfVlanAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VfinfoAttrs::VlanList(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VfinfoAttrs",
            "VlanList",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_broadcast(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VfinfoAttrs::Broadcast(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VfinfoAttrs",
            "Broadcast",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl VfinfoAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableVfinfoAttrs<'a> {
        IterableVfinfoAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Mac",
            2u16 => "Vlan",
            3u16 => "TxRate",
            4u16 => "Spoofchk",
            5u16 => "LinkState",
            6u16 => "Rate",
            7u16 => "RssQueryEn",
            8u16 => "Stats",
            9u16 => "Trust",
            10u16 => "IbNodeGuid",
            11u16 => "IbPortGuid",
            12u16 => "VlanList",
            13u16 => "Broadcast",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableVfinfoAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableVfinfoAttrs<'a> {
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
impl<'a> Iterator for IterableVfinfoAttrs<'a> {
    type Item = Result<VfinfoAttrs<'a>, ErrorContext>;
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
                1u16 => VfinfoAttrs::Mac({
                    let res = Some(IflaVfMac::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => VfinfoAttrs::Vlan({
                    let res = Some(IflaVfVlan::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => VfinfoAttrs::TxRate({
                    let res = Some(IflaVfTxRate::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => VfinfoAttrs::Spoofchk({
                    let res = Some(IflaVfSpoofchk::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => VfinfoAttrs::LinkState({
                    let res = Some(IflaVfLinkState::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => VfinfoAttrs::Rate({
                    let res = Some(IflaVfRate::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => VfinfoAttrs::RssQueryEn({
                    let res = Some(IflaVfRssQueryEn::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => VfinfoAttrs::Stats({
                    let res = Some(IterableVfStatsAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => VfinfoAttrs::Trust({
                    let res = Some(IflaVfTrust::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => VfinfoAttrs::IbNodeGuid({
                    let res = Some(IflaVfGuid::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => VfinfoAttrs::IbPortGuid({
                    let res = Some(IflaVfGuid::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => VfinfoAttrs::VlanList({
                    let res = Some(IterableVfVlanAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => VfinfoAttrs::Broadcast({
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
            "VfinfoAttrs",
            r#type.and_then(|t| VfinfoAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableVfinfoAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("VfinfoAttrs");
        let mut iter = IterableVfinfoAttrs::with_loc(&[], self.orig_loc);
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
                VfinfoAttrs::Mac(val) => fmt.field("Mac", &val),
                VfinfoAttrs::Vlan(val) => fmt.field("Vlan", &val),
                VfinfoAttrs::TxRate(val) => fmt.field("TxRate", &val),
                VfinfoAttrs::Spoofchk(val) => fmt.field("Spoofchk", &val),
                VfinfoAttrs::LinkState(val) => fmt.field("LinkState", &val),
                VfinfoAttrs::Rate(val) => fmt.field("Rate", &val),
                VfinfoAttrs::RssQueryEn(val) => fmt.field("RssQueryEn", &val),
                VfinfoAttrs::Stats(val) => fmt.field("Stats", &val),
                VfinfoAttrs::Trust(val) => fmt.field("Trust", &val),
                VfinfoAttrs::IbNodeGuid(val) => fmt.field("IbNodeGuid", &val),
                VfinfoAttrs::IbPortGuid(val) => fmt.field("IbPortGuid", &val),
                VfinfoAttrs::VlanList(val) => fmt.field("VlanList", &val),
                VfinfoAttrs::Broadcast(val) => fmt.field("Broadcast", &FormatHexdump(val)),
            };
        }
        fmt.finish()
    }
}
impl IterableVfinfoAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("VfinfoAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| VfinfoAttrs::attr_from_type(t)),
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
                VfinfoAttrs::Mac(val) => {
                    if last_off == offset {
                        stack.push(("Mac", last_off));
                        break;
                    }
                }
                VfinfoAttrs::Vlan(val) => {
                    if last_off == offset {
                        stack.push(("Vlan", last_off));
                        break;
                    }
                }
                VfinfoAttrs::TxRate(val) => {
                    if last_off == offset {
                        stack.push(("TxRate", last_off));
                        break;
                    }
                }
                VfinfoAttrs::Spoofchk(val) => {
                    if last_off == offset {
                        stack.push(("Spoofchk", last_off));
                        break;
                    }
                }
                VfinfoAttrs::LinkState(val) => {
                    if last_off == offset {
                        stack.push(("LinkState", last_off));
                        break;
                    }
                }
                VfinfoAttrs::Rate(val) => {
                    if last_off == offset {
                        stack.push(("Rate", last_off));
                        break;
                    }
                }
                VfinfoAttrs::RssQueryEn(val) => {
                    if last_off == offset {
                        stack.push(("RssQueryEn", last_off));
                        break;
                    }
                }
                VfinfoAttrs::Stats(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                VfinfoAttrs::Trust(val) => {
                    if last_off == offset {
                        stack.push(("Trust", last_off));
                        break;
                    }
                }
                VfinfoAttrs::IbNodeGuid(val) => {
                    if last_off == offset {
                        stack.push(("IbNodeGuid", last_off));
                        break;
                    }
                }
                VfinfoAttrs::IbPortGuid(val) => {
                    if last_off == offset {
                        stack.push(("IbPortGuid", last_off));
                        break;
                    }
                }
                VfinfoAttrs::VlanList(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                VfinfoAttrs::Broadcast(val) => {
                    if last_off == offset {
                        stack.push(("Broadcast", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("VfinfoAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum VfStatsAttrs<'a> {
    RxPackets(u64),
    TxPackets(u64),
    RxBytes(u64),
    TxBytes(u64),
    Broadcast(u64),
    Multicast(u64),
    Pad(&'a [u8]),
    RxDropped(u64),
    TxDropped(u64),
}
impl<'a> IterableVfStatsAttrs<'a> {
    pub fn get_rx_packets(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VfStatsAttrs::RxPackets(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VfStatsAttrs",
            "RxPackets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tx_packets(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VfStatsAttrs::TxPackets(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VfStatsAttrs",
            "TxPackets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rx_bytes(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VfStatsAttrs::RxBytes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VfStatsAttrs",
            "RxBytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tx_bytes(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VfStatsAttrs::TxBytes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VfStatsAttrs",
            "TxBytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_broadcast(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VfStatsAttrs::Broadcast(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VfStatsAttrs",
            "Broadcast",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_multicast(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VfStatsAttrs::Multicast(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VfStatsAttrs",
            "Multicast",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VfStatsAttrs::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VfStatsAttrs",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rx_dropped(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VfStatsAttrs::RxDropped(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VfStatsAttrs",
            "RxDropped",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tx_dropped(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VfStatsAttrs::TxDropped(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VfStatsAttrs",
            "TxDropped",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl VfStatsAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableVfStatsAttrs<'a> {
        IterableVfStatsAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "RxPackets",
            1u16 => "TxPackets",
            2u16 => "RxBytes",
            3u16 => "TxBytes",
            4u16 => "Broadcast",
            5u16 => "Multicast",
            6u16 => "Pad",
            7u16 => "RxDropped",
            8u16 => "TxDropped",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableVfStatsAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableVfStatsAttrs<'a> {
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
impl<'a> Iterator for IterableVfStatsAttrs<'a> {
    type Item = Result<VfStatsAttrs<'a>, ErrorContext>;
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
                0u16 => VfStatsAttrs::RxPackets({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                1u16 => VfStatsAttrs::TxPackets({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => VfStatsAttrs::RxBytes({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => VfStatsAttrs::TxBytes({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => VfStatsAttrs::Broadcast({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => VfStatsAttrs::Multicast({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => VfStatsAttrs::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => VfStatsAttrs::RxDropped({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => VfStatsAttrs::TxDropped({
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
            "VfStatsAttrs",
            r#type.and_then(|t| VfStatsAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableVfStatsAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("VfStatsAttrs");
        let mut iter = IterableVfStatsAttrs::with_loc(&[], self.orig_loc);
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
                VfStatsAttrs::RxPackets(val) => fmt.field("RxPackets", &val),
                VfStatsAttrs::TxPackets(val) => fmt.field("TxPackets", &val),
                VfStatsAttrs::RxBytes(val) => fmt.field("RxBytes", &val),
                VfStatsAttrs::TxBytes(val) => fmt.field("TxBytes", &val),
                VfStatsAttrs::Broadcast(val) => fmt.field("Broadcast", &val),
                VfStatsAttrs::Multicast(val) => fmt.field("Multicast", &val),
                VfStatsAttrs::Pad(val) => fmt.field("Pad", &val),
                VfStatsAttrs::RxDropped(val) => fmt.field("RxDropped", &val),
                VfStatsAttrs::TxDropped(val) => fmt.field("TxDropped", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableVfStatsAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("VfStatsAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| VfStatsAttrs::attr_from_type(t)),
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
                VfStatsAttrs::RxPackets(val) => {
                    if last_off == offset {
                        stack.push(("RxPackets", last_off));
                        break;
                    }
                }
                VfStatsAttrs::TxPackets(val) => {
                    if last_off == offset {
                        stack.push(("TxPackets", last_off));
                        break;
                    }
                }
                VfStatsAttrs::RxBytes(val) => {
                    if last_off == offset {
                        stack.push(("RxBytes", last_off));
                        break;
                    }
                }
                VfStatsAttrs::TxBytes(val) => {
                    if last_off == offset {
                        stack.push(("TxBytes", last_off));
                        break;
                    }
                }
                VfStatsAttrs::Broadcast(val) => {
                    if last_off == offset {
                        stack.push(("Broadcast", last_off));
                        break;
                    }
                }
                VfStatsAttrs::Multicast(val) => {
                    if last_off == offset {
                        stack.push(("Multicast", last_off));
                        break;
                    }
                }
                VfStatsAttrs::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                VfStatsAttrs::RxDropped(val) => {
                    if last_off == offset {
                        stack.push(("RxDropped", last_off));
                        break;
                    }
                }
                VfStatsAttrs::TxDropped(val) => {
                    if last_off == offset {
                        stack.push(("TxDropped", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("VfStatsAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum VfVlanAttrs {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    Info(IflaVfVlanInfo),
}
impl<'a> IterableVfVlanAttrs<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_info(&self) -> MultiAttrIterable<Self, VfVlanAttrs, IflaVfVlanInfo> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let VfVlanAttrs::Info(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl VfVlanAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableVfVlanAttrs<'a> {
        IterableVfVlanAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Info",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableVfVlanAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableVfVlanAttrs<'a> {
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
impl<'a> Iterator for IterableVfVlanAttrs<'a> {
    type Item = Result<VfVlanAttrs, ErrorContext>;
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
                1u16 => VfVlanAttrs::Info({
                    let res = Some(IflaVfVlanInfo::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "VfVlanAttrs",
            r#type.and_then(|t| VfVlanAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableVfVlanAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("VfVlanAttrs");
        let mut iter = IterableVfVlanAttrs::with_loc(&[], self.orig_loc);
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
                VfVlanAttrs::Info(val) => fmt.field("Info", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableVfVlanAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("VfVlanAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| VfVlanAttrs::attr_from_type(t)),
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
                VfVlanAttrs::Info(val) => {
                    if last_off == offset {
                        stack.push(("Info", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("VfVlanAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum VfPortsAttrs {}
impl<'a> IterableVfPortsAttrs<'a> {}
impl VfPortsAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableVfPortsAttrs<'a> {
        IterableVfPortsAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        None
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableVfPortsAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableVfPortsAttrs<'a> {
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
impl<'a> Iterator for IterableVfPortsAttrs<'a> {
    type Item = Result<VfPortsAttrs, ErrorContext>;
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
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "VfPortsAttrs",
            r#type.and_then(|t| VfPortsAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableVfPortsAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("VfPortsAttrs");
        let mut iter = IterableVfPortsAttrs::with_loc(&[], self.orig_loc);
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
            match attr {};
        }
        fmt.finish()
    }
}
impl IterableVfPortsAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("VfPortsAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| VfPortsAttrs::attr_from_type(t)),
            );
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum PortSelfAttrs {}
impl<'a> IterablePortSelfAttrs<'a> {}
impl PortSelfAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePortSelfAttrs<'a> {
        IterablePortSelfAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        None
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePortSelfAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePortSelfAttrs<'a> {
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
impl<'a> Iterator for IterablePortSelfAttrs<'a> {
    type Item = Result<PortSelfAttrs, ErrorContext>;
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
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "PortSelfAttrs",
            r#type.and_then(|t| PortSelfAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterablePortSelfAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("PortSelfAttrs");
        let mut iter = IterablePortSelfAttrs::with_loc(&[], self.orig_loc);
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
            match attr {};
        }
        fmt.finish()
    }
}
impl IterablePortSelfAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("PortSelfAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| PortSelfAttrs::attr_from_type(t)),
            );
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LinkinfoAttrs<'a> {
    Kind(&'a CStr),
    Data(LinkinfoDataMsg<'a>),
    Xstats(&'a [u8]),
    SlaveKind(&'a CStr),
    SlaveData(LinkinfoMemberDataMsg<'a>),
}
impl<'a> IterableLinkinfoAttrs<'a> {
    pub fn get_kind(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoAttrs::Kind(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoAttrs",
            "Kind",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_data(&self) -> Result<LinkinfoDataMsg<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoAttrs::Data(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoAttrs",
            "Data",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_xstats(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoAttrs::Xstats(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoAttrs",
            "Xstats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_slave_kind(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoAttrs::SlaveKind(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoAttrs",
            "SlaveKind",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_slave_data(&self) -> Result<LinkinfoMemberDataMsg<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoAttrs::SlaveData(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoAttrs",
            "SlaveData",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
#[derive(Debug, Clone)]
pub enum LinkinfoDataMsg<'a> {
    Bond(IterableLinkinfoBondAttrs<'a>),
    Bridge(IterableLinkinfoBridgeAttrs<'a>),
    Erspan(IterableLinkinfoGreAttrs<'a>),
    Gre(IterableLinkinfoGreAttrs<'a>),
    Gretap(IterableLinkinfoGreAttrs<'a>),
    Ip6gre(IterableLinkinfoGre6Attrs<'a>),
    Geneve(IterableLinkinfoGeneveAttrs<'a>),
    Hsr(IterableLinkinfoHsrAttrs<'a>),
    Ipip(IterableLinkinfoIptunAttrs<'a>),
    Ip6tnl(IterableLinkinfoIp6tnlAttrs<'a>),
    Sit(IterableLinkinfoIptunAttrs<'a>),
    Tun(IterableLinkinfoTunAttrs<'a>),
    Vlan(IterableLinkinfoVlanAttrs<'a>),
    Vrf(IterableLinkinfoVrfAttrs<'a>),
    Vti(IterableLinkinfoVtiAttrs<'a>),
    Vti6(IterableLinkinfoVti6Attrs<'a>),
    Netkit(IterableLinkinfoNetkitAttrs<'a>),
    Ovpn(IterableLinkinfoOvpnAttrs<'a>),
}
impl<'a> LinkinfoDataMsg<'a> {
    fn select_with_loc(selector: &'_ [u8], buf: &'a [u8], loc: usize) -> Option<Self> {
        match selector {
            b"bond" => Some(LinkinfoDataMsg::Bond(IterableLinkinfoBondAttrs::with_loc(
                buf, loc,
            ))),
            b"bridge" => Some(LinkinfoDataMsg::Bridge(
                IterableLinkinfoBridgeAttrs::with_loc(buf, loc),
            )),
            b"erspan" => Some(LinkinfoDataMsg::Erspan(IterableLinkinfoGreAttrs::with_loc(
                buf, loc,
            ))),
            b"gre" => Some(LinkinfoDataMsg::Gre(IterableLinkinfoGreAttrs::with_loc(
                buf, loc,
            ))),
            b"gretap" => Some(LinkinfoDataMsg::Gretap(IterableLinkinfoGreAttrs::with_loc(
                buf, loc,
            ))),
            b"ip6gre" => Some(LinkinfoDataMsg::Ip6gre(
                IterableLinkinfoGre6Attrs::with_loc(buf, loc),
            )),
            b"geneve" => Some(LinkinfoDataMsg::Geneve(
                IterableLinkinfoGeneveAttrs::with_loc(buf, loc),
            )),
            b"hsr" => Some(LinkinfoDataMsg::Hsr(IterableLinkinfoHsrAttrs::with_loc(
                buf, loc,
            ))),
            b"ipip" => Some(LinkinfoDataMsg::Ipip(IterableLinkinfoIptunAttrs::with_loc(
                buf, loc,
            ))),
            b"ip6tnl" => Some(LinkinfoDataMsg::Ip6tnl(
                IterableLinkinfoIp6tnlAttrs::with_loc(buf, loc),
            )),
            b"sit" => Some(LinkinfoDataMsg::Sit(IterableLinkinfoIptunAttrs::with_loc(
                buf, loc,
            ))),
            b"tun" => Some(LinkinfoDataMsg::Tun(IterableLinkinfoTunAttrs::with_loc(
                buf, loc,
            ))),
            b"vlan" => Some(LinkinfoDataMsg::Vlan(IterableLinkinfoVlanAttrs::with_loc(
                buf, loc,
            ))),
            b"vrf" => Some(LinkinfoDataMsg::Vrf(IterableLinkinfoVrfAttrs::with_loc(
                buf, loc,
            ))),
            b"vti" => Some(LinkinfoDataMsg::Vti(IterableLinkinfoVtiAttrs::with_loc(
                buf, loc,
            ))),
            b"vti6" => Some(LinkinfoDataMsg::Vti6(IterableLinkinfoVti6Attrs::with_loc(
                buf, loc,
            ))),
            b"netkit" => Some(LinkinfoDataMsg::Netkit(
                IterableLinkinfoNetkitAttrs::with_loc(buf, loc),
            )),
            b"ovpn" => Some(LinkinfoDataMsg::Ovpn(IterableLinkinfoOvpnAttrs::with_loc(
                buf, loc,
            ))),
            _ => None,
        }
    }
}
#[derive(Debug, Clone)]
pub enum LinkinfoMemberDataMsg<'a> {
    Bridge(IterableLinkinfoBrportAttrs<'a>),
    Bond(IterableBondSlaveAttrs<'a>),
}
impl<'a> LinkinfoMemberDataMsg<'a> {
    fn select_with_loc(selector: &'_ [u8], buf: &'a [u8], loc: usize) -> Option<Self> {
        match selector {
            b"bridge" => Some(LinkinfoMemberDataMsg::Bridge(
                IterableLinkinfoBrportAttrs::with_loc(buf, loc),
            )),
            b"bond" => Some(LinkinfoMemberDataMsg::Bond(
                IterableBondSlaveAttrs::with_loc(buf, loc),
            )),
            _ => None,
        }
    }
}
impl LinkinfoAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLinkinfoAttrs<'a> {
        IterableLinkinfoAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Kind",
            2u16 => "Data",
            3u16 => "Xstats",
            4u16 => "SlaveKind",
            5u16 => "SlaveData",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLinkinfoAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
    selector_kind: Option<&'a [u8]>,
    selector_slave_kind: Option<&'a [u8]>,
}
impl<'a> IterableLinkinfoAttrs<'a> {
    fn with_loc(buf: &'a [u8], orig_loc: usize) -> Self {
        Self {
            buf,
            pos: 0,
            orig_loc,
            selector_kind: None,
            selector_slave_kind: None,
        }
    }
    pub fn get_buf(&self) -> &'a [u8] {
        self.buf
    }
}
impl<'a> Iterator for IterableLinkinfoAttrs<'a> {
    type Item = Result<LinkinfoAttrs<'a>, ErrorContext>;
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
                1u16 => LinkinfoAttrs::Kind({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => LinkinfoAttrs::Data({
                    let res = {
                        let Some(selector) = self.selector_kind else {
                            break;
                        };
                        match LinkinfoDataMsg::select_with_loc(selector, next, self.orig_loc) {
                            Some(sub) => Some(sub),
                            None if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                            None => continue,
                        }
                    };
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => LinkinfoAttrs::Xstats({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => LinkinfoAttrs::SlaveKind({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => LinkinfoAttrs::SlaveData({
                    let res = {
                        let Some(selector) = self.selector_slave_kind else {
                            break;
                        };
                        match LinkinfoMemberDataMsg::select_with_loc(selector, next, self.orig_loc)
                        {
                            Some(sub) => Some(sub),
                            None if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                            None => continue,
                        }
                    };
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            if let LinkinfoAttrs::SlaveKind(sel) = &res {
                self.selector_slave_kind = Some(sel.to_bytes());
            }
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "LinkinfoAttrs",
            r#type.and_then(|t| LinkinfoAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableLinkinfoAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LinkinfoAttrs");
        let mut iter = IterableLinkinfoAttrs::with_loc(&[], self.orig_loc);
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
                LinkinfoAttrs::Kind(val) => fmt.field("Kind", &val),
                LinkinfoAttrs::Data(val) => fmt.field("Data", &val),
                LinkinfoAttrs::Xstats(val) => fmt.field("Xstats", &FormatHexdump(val)),
                LinkinfoAttrs::SlaveKind(val) => fmt.field("SlaveKind", &val),
                LinkinfoAttrs::SlaveData(val) => fmt.field("SlaveData", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableLinkinfoAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LinkinfoAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| LinkinfoAttrs::attr_from_type(t)),
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
                LinkinfoAttrs::Kind(val) => {
                    if last_off == offset {
                        stack.push(("Kind", last_off));
                        break;
                    }
                }
                LinkinfoAttrs::Data(val) => {
                    if last_off == offset {
                        stack.push(("Data", last_off));
                        break;
                    }
                }
                LinkinfoAttrs::Xstats(val) => {
                    if last_off == offset {
                        stack.push(("Xstats", last_off));
                        break;
                    }
                }
                LinkinfoAttrs::SlaveKind(val) => {
                    if last_off == offset {
                        stack.push(("SlaveKind", last_off));
                        break;
                    }
                }
                LinkinfoAttrs::SlaveData(val) => {
                    if last_off == offset {
                        stack.push(("SlaveData", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LinkinfoAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LinkinfoBondAttrs<'a> {
    Mode(u8),
    ActiveSlave(u32),
    Miimon(u32),
    Updelay(u32),
    Downdelay(u32),
    UseCarrier(u8),
    ArpInterval(u32),
    ArpIpTarget(IterableArrayIpv4Addr<'a>),
    ArpValidate(u32),
    ArpAllTargets(u32),
    Primary(u32),
    PrimaryReselect(u8),
    FailOverMac(u8),
    XmitHashPolicy(u8),
    ResendIgmp(u32),
    NumPeerNotif(u8),
    AllSlavesActive(u8),
    MinLinks(u32),
    LpInterval(u32),
    PacketsPerSlave(u32),
    AdLacpRate(u8),
    AdSelect(u8),
    AdInfo(IterableBondAdInfoAttrs<'a>),
    AdActorSysPrio(u16),
    AdUserPortKey(u16),
    AdActorSystem(&'a [u8]),
    TlbDynamicLb(u8),
    PeerNotifDelay(u32),
    AdLacpActive(u8),
    MissedMax(u8),
    NsIp6Target(IterableArrayBinary<'a>),
    CoupledControl(u8),
    BroadcastNeigh(u8),
    LacpStrict(u8),
}
impl<'a> IterableLinkinfoBondAttrs<'a> {
    pub fn get_mode(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::Mode(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "Mode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_active_slave(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::ActiveSlave(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "ActiveSlave",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_miimon(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::Miimon(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "Miimon",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_updelay(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::Updelay(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "Updelay",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_downdelay(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::Downdelay(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "Downdelay",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_use_carrier(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::UseCarrier(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "UseCarrier",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_arp_interval(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::ArpInterval(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "ArpInterval",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_arp_ip_target(
        &self,
    ) -> Result<ArrayIterable<IterableArrayIpv4Addr<'a>, std::net::Ipv4Addr>, ErrorContext> {
        for attr in self.clone() {
            if let Ok(LinkinfoBondAttrs::ArpIpTarget(val)) = attr {
                return Ok(ArrayIterable::new(val));
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "ArpIpTarget",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_arp_validate(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::ArpValidate(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "ArpValidate",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_arp_all_targets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::ArpAllTargets(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "ArpAllTargets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_primary(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::Primary(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "Primary",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_primary_reselect(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::PrimaryReselect(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "PrimaryReselect",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fail_over_mac(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::FailOverMac(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "FailOverMac",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_xmit_hash_policy(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::XmitHashPolicy(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "XmitHashPolicy",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_resend_igmp(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::ResendIgmp(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "ResendIgmp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_num_peer_notif(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::NumPeerNotif(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "NumPeerNotif",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_all_slaves_active(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::AllSlavesActive(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "AllSlavesActive",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_min_links(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::MinLinks(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "MinLinks",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_lp_interval(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::LpInterval(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "LpInterval",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_packets_per_slave(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::PacketsPerSlave(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "PacketsPerSlave",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ad_lacp_rate(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::AdLacpRate(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "AdLacpRate",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ad_select(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::AdSelect(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "AdSelect",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ad_info(&self) -> Result<IterableBondAdInfoAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::AdInfo(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "AdInfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ad_actor_sys_prio(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::AdActorSysPrio(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "AdActorSysPrio",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ad_user_port_key(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::AdUserPortKey(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "AdUserPortKey",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ad_actor_system(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::AdActorSystem(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "AdActorSystem",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tlb_dynamic_lb(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::TlbDynamicLb(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "TlbDynamicLb",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_peer_notif_delay(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::PeerNotifDelay(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "PeerNotifDelay",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ad_lacp_active(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::AdLacpActive(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "AdLacpActive",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_missed_max(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::MissedMax(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "MissedMax",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ns_ip6_target(
        &self,
    ) -> Result<ArrayIterable<IterableArrayBinary<'a>, &'a [u8]>, ErrorContext> {
        for attr in self.clone() {
            if let Ok(LinkinfoBondAttrs::NsIp6Target(val)) = attr {
                return Ok(ArrayIterable::new(val));
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "NsIp6Target",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_coupled_control(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::CoupledControl(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "CoupledControl",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_broadcast_neigh(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::BroadcastNeigh(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "BroadcastNeigh",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_lacp_strict(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBondAttrs::LacpStrict(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBondAttrs",
            "LacpStrict",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableArrayIpv4Addr<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableArrayIpv4Addr<'a> {
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
impl<'a> Iterator for IterableArrayIpv4Addr<'a> {
    type Item = Result<std::net::Ipv4Addr, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            {
                let Some(res) = parse_be_u32(next).map(Ipv4Addr::from_bits) else {
                    break;
                };
                return Some(Ok(res));
            }
        }
        let pos = self.pos;
        self.pos = self.buf.len();
        Some(Err(ErrorContext::new(
            "Ipv4Addr",
            None,
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableArrayBinary<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableArrayBinary<'a> {
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
impl<'a> Iterator for IterableArrayBinary<'a> {
    type Item = Result<&'a [u8], ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            {
                let Some(res) = Some(next) else { break };
                return Some(Ok(res));
            }
        }
        let pos = self.pos;
        self.pos = self.buf.len();
        Some(Err(ErrorContext::new(
            "Binary",
            None,
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl LinkinfoBondAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLinkinfoBondAttrs<'a> {
        IterableLinkinfoBondAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Mode",
            2u16 => "ActiveSlave",
            3u16 => "Miimon",
            4u16 => "Updelay",
            5u16 => "Downdelay",
            6u16 => "UseCarrier",
            7u16 => "ArpInterval",
            8u16 => "ArpIpTarget",
            9u16 => "ArpValidate",
            10u16 => "ArpAllTargets",
            11u16 => "Primary",
            12u16 => "PrimaryReselect",
            13u16 => "FailOverMac",
            14u16 => "XmitHashPolicy",
            15u16 => "ResendIgmp",
            16u16 => "NumPeerNotif",
            17u16 => "AllSlavesActive",
            18u16 => "MinLinks",
            19u16 => "LpInterval",
            20u16 => "PacketsPerSlave",
            21u16 => "AdLacpRate",
            22u16 => "AdSelect",
            23u16 => "AdInfo",
            24u16 => "AdActorSysPrio",
            25u16 => "AdUserPortKey",
            26u16 => "AdActorSystem",
            27u16 => "TlbDynamicLb",
            28u16 => "PeerNotifDelay",
            29u16 => "AdLacpActive",
            30u16 => "MissedMax",
            31u16 => "NsIp6Target",
            32u16 => "CoupledControl",
            33u16 => "BroadcastNeigh",
            34u16 => "LacpStrict",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLinkinfoBondAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLinkinfoBondAttrs<'a> {
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
impl<'a> Iterator for IterableLinkinfoBondAttrs<'a> {
    type Item = Result<LinkinfoBondAttrs<'a>, ErrorContext>;
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
                1u16 => LinkinfoBondAttrs::Mode({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => LinkinfoBondAttrs::ActiveSlave({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => LinkinfoBondAttrs::Miimon({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => LinkinfoBondAttrs::Updelay({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => LinkinfoBondAttrs::Downdelay({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => LinkinfoBondAttrs::UseCarrier({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => LinkinfoBondAttrs::ArpInterval({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => LinkinfoBondAttrs::ArpIpTarget({
                    let res = Some(IterableArrayIpv4Addr::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => LinkinfoBondAttrs::ArpValidate({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => LinkinfoBondAttrs::ArpAllTargets({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => LinkinfoBondAttrs::Primary({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => LinkinfoBondAttrs::PrimaryReselect({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => LinkinfoBondAttrs::FailOverMac({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => LinkinfoBondAttrs::XmitHashPolicy({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => LinkinfoBondAttrs::ResendIgmp({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => LinkinfoBondAttrs::NumPeerNotif({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => LinkinfoBondAttrs::AllSlavesActive({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => LinkinfoBondAttrs::MinLinks({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => LinkinfoBondAttrs::LpInterval({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => LinkinfoBondAttrs::PacketsPerSlave({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => LinkinfoBondAttrs::AdLacpRate({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => LinkinfoBondAttrs::AdSelect({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => LinkinfoBondAttrs::AdInfo({
                    let res = Some(IterableBondAdInfoAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => LinkinfoBondAttrs::AdActorSysPrio({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                25u16 => LinkinfoBondAttrs::AdUserPortKey({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                26u16 => LinkinfoBondAttrs::AdActorSystem({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                27u16 => LinkinfoBondAttrs::TlbDynamicLb({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                28u16 => LinkinfoBondAttrs::PeerNotifDelay({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                29u16 => LinkinfoBondAttrs::AdLacpActive({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                30u16 => LinkinfoBondAttrs::MissedMax({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                31u16 => LinkinfoBondAttrs::NsIp6Target({
                    let res = Some(IterableArrayBinary::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                32u16 => LinkinfoBondAttrs::CoupledControl({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                33u16 => LinkinfoBondAttrs::BroadcastNeigh({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                34u16 => LinkinfoBondAttrs::LacpStrict({
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
            "LinkinfoBondAttrs",
            r#type.and_then(|t| LinkinfoBondAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableArrayIpv4Addr<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_list()
            .entries(self.clone().map(FlattenErrorContext))
            .finish()
    }
}
impl std::fmt::Debug for IterableArrayBinary<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_list()
            .entries(self.clone().map(FlattenErrorContext))
            .finish()
    }
}
impl<'a> std::fmt::Debug for IterableLinkinfoBondAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LinkinfoBondAttrs");
        let mut iter = IterableLinkinfoBondAttrs::with_loc(&[], self.orig_loc);
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
                LinkinfoBondAttrs::Mode(val) => fmt.field("Mode", &val),
                LinkinfoBondAttrs::ActiveSlave(val) => fmt.field("ActiveSlave", &val),
                LinkinfoBondAttrs::Miimon(val) => fmt.field("Miimon", &val),
                LinkinfoBondAttrs::Updelay(val) => fmt.field("Updelay", &val),
                LinkinfoBondAttrs::Downdelay(val) => fmt.field("Downdelay", &val),
                LinkinfoBondAttrs::UseCarrier(val) => fmt.field("UseCarrier", &val),
                LinkinfoBondAttrs::ArpInterval(val) => fmt.field("ArpInterval", &val),
                LinkinfoBondAttrs::ArpIpTarget(val) => fmt.field("ArpIpTarget", &val),
                LinkinfoBondAttrs::ArpValidate(val) => fmt.field("ArpValidate", &val),
                LinkinfoBondAttrs::ArpAllTargets(val) => fmt.field("ArpAllTargets", &val),
                LinkinfoBondAttrs::Primary(val) => fmt.field("Primary", &val),
                LinkinfoBondAttrs::PrimaryReselect(val) => fmt.field("PrimaryReselect", &val),
                LinkinfoBondAttrs::FailOverMac(val) => fmt.field("FailOverMac", &val),
                LinkinfoBondAttrs::XmitHashPolicy(val) => fmt.field("XmitHashPolicy", &val),
                LinkinfoBondAttrs::ResendIgmp(val) => fmt.field("ResendIgmp", &val),
                LinkinfoBondAttrs::NumPeerNotif(val) => fmt.field("NumPeerNotif", &val),
                LinkinfoBondAttrs::AllSlavesActive(val) => fmt.field("AllSlavesActive", &val),
                LinkinfoBondAttrs::MinLinks(val) => fmt.field("MinLinks", &val),
                LinkinfoBondAttrs::LpInterval(val) => fmt.field("LpInterval", &val),
                LinkinfoBondAttrs::PacketsPerSlave(val) => fmt.field("PacketsPerSlave", &val),
                LinkinfoBondAttrs::AdLacpRate(val) => fmt.field("AdLacpRate", &val),
                LinkinfoBondAttrs::AdSelect(val) => fmt.field("AdSelect", &val),
                LinkinfoBondAttrs::AdInfo(val) => fmt.field("AdInfo", &val),
                LinkinfoBondAttrs::AdActorSysPrio(val) => fmt.field("AdActorSysPrio", &val),
                LinkinfoBondAttrs::AdUserPortKey(val) => fmt.field("AdUserPortKey", &val),
                LinkinfoBondAttrs::AdActorSystem(val) => {
                    fmt.field("AdActorSystem", &FormatMac(val))
                }
                LinkinfoBondAttrs::TlbDynamicLb(val) => fmt.field("TlbDynamicLb", &val),
                LinkinfoBondAttrs::PeerNotifDelay(val) => fmt.field("PeerNotifDelay", &val),
                LinkinfoBondAttrs::AdLacpActive(val) => fmt.field("AdLacpActive", &val),
                LinkinfoBondAttrs::MissedMax(val) => fmt.field("MissedMax", &val),
                LinkinfoBondAttrs::NsIp6Target(val) => fmt.field("NsIp6Target", &val),
                LinkinfoBondAttrs::CoupledControl(val) => fmt.field("CoupledControl", &val),
                LinkinfoBondAttrs::BroadcastNeigh(val) => fmt.field("BroadcastNeigh", &val),
                LinkinfoBondAttrs::LacpStrict(val) => fmt.field("LacpStrict", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableLinkinfoBondAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LinkinfoBondAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| LinkinfoBondAttrs::attr_from_type(t)),
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
                LinkinfoBondAttrs::Mode(val) => {
                    if last_off == offset {
                        stack.push(("Mode", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::ActiveSlave(val) => {
                    if last_off == offset {
                        stack.push(("ActiveSlave", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::Miimon(val) => {
                    if last_off == offset {
                        stack.push(("Miimon", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::Updelay(val) => {
                    if last_off == offset {
                        stack.push(("Updelay", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::Downdelay(val) => {
                    if last_off == offset {
                        stack.push(("Downdelay", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::UseCarrier(val) => {
                    if last_off == offset {
                        stack.push(("UseCarrier", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::ArpInterval(val) => {
                    if last_off == offset {
                        stack.push(("ArpInterval", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::ArpIpTarget(val) => {
                    if last_off == offset {
                        stack.push(("ArpIpTarget", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::ArpValidate(val) => {
                    if last_off == offset {
                        stack.push(("ArpValidate", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::ArpAllTargets(val) => {
                    if last_off == offset {
                        stack.push(("ArpAllTargets", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::Primary(val) => {
                    if last_off == offset {
                        stack.push(("Primary", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::PrimaryReselect(val) => {
                    if last_off == offset {
                        stack.push(("PrimaryReselect", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::FailOverMac(val) => {
                    if last_off == offset {
                        stack.push(("FailOverMac", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::XmitHashPolicy(val) => {
                    if last_off == offset {
                        stack.push(("XmitHashPolicy", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::ResendIgmp(val) => {
                    if last_off == offset {
                        stack.push(("ResendIgmp", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::NumPeerNotif(val) => {
                    if last_off == offset {
                        stack.push(("NumPeerNotif", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::AllSlavesActive(val) => {
                    if last_off == offset {
                        stack.push(("AllSlavesActive", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::MinLinks(val) => {
                    if last_off == offset {
                        stack.push(("MinLinks", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::LpInterval(val) => {
                    if last_off == offset {
                        stack.push(("LpInterval", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::PacketsPerSlave(val) => {
                    if last_off == offset {
                        stack.push(("PacketsPerSlave", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::AdLacpRate(val) => {
                    if last_off == offset {
                        stack.push(("AdLacpRate", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::AdSelect(val) => {
                    if last_off == offset {
                        stack.push(("AdSelect", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::AdInfo(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                LinkinfoBondAttrs::AdActorSysPrio(val) => {
                    if last_off == offset {
                        stack.push(("AdActorSysPrio", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::AdUserPortKey(val) => {
                    if last_off == offset {
                        stack.push(("AdUserPortKey", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::AdActorSystem(val) => {
                    if last_off == offset {
                        stack.push(("AdActorSystem", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::TlbDynamicLb(val) => {
                    if last_off == offset {
                        stack.push(("TlbDynamicLb", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::PeerNotifDelay(val) => {
                    if last_off == offset {
                        stack.push(("PeerNotifDelay", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::AdLacpActive(val) => {
                    if last_off == offset {
                        stack.push(("AdLacpActive", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::MissedMax(val) => {
                    if last_off == offset {
                        stack.push(("MissedMax", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::NsIp6Target(val) => {
                    if last_off == offset {
                        stack.push(("NsIp6Target", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::CoupledControl(val) => {
                    if last_off == offset {
                        stack.push(("CoupledControl", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::BroadcastNeigh(val) => {
                    if last_off == offset {
                        stack.push(("BroadcastNeigh", last_off));
                        break;
                    }
                }
                LinkinfoBondAttrs::LacpStrict(val) => {
                    if last_off == offset {
                        stack.push(("LacpStrict", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LinkinfoBondAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum BondAdInfoAttrs<'a> {
    Aggregator(u16),
    NumPorts(u16),
    ActorKey(u16),
    PartnerKey(u16),
    PartnerMac(&'a [u8]),
}
impl<'a> IterableBondAdInfoAttrs<'a> {
    pub fn get_aggregator(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(BondAdInfoAttrs::Aggregator(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BondAdInfoAttrs",
            "Aggregator",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_num_ports(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(BondAdInfoAttrs::NumPorts(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BondAdInfoAttrs",
            "NumPorts",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_actor_key(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(BondAdInfoAttrs::ActorKey(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BondAdInfoAttrs",
            "ActorKey",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_partner_key(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(BondAdInfoAttrs::PartnerKey(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BondAdInfoAttrs",
            "PartnerKey",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_partner_mac(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(BondAdInfoAttrs::PartnerMac(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BondAdInfoAttrs",
            "PartnerMac",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl BondAdInfoAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableBondAdInfoAttrs<'a> {
        IterableBondAdInfoAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Aggregator",
            2u16 => "NumPorts",
            3u16 => "ActorKey",
            4u16 => "PartnerKey",
            5u16 => "PartnerMac",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableBondAdInfoAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableBondAdInfoAttrs<'a> {
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
impl<'a> Iterator for IterableBondAdInfoAttrs<'a> {
    type Item = Result<BondAdInfoAttrs<'a>, ErrorContext>;
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
                1u16 => BondAdInfoAttrs::Aggregator({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => BondAdInfoAttrs::NumPorts({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => BondAdInfoAttrs::ActorKey({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => BondAdInfoAttrs::PartnerKey({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => BondAdInfoAttrs::PartnerMac({
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
            "BondAdInfoAttrs",
            r#type.and_then(|t| BondAdInfoAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableBondAdInfoAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("BondAdInfoAttrs");
        let mut iter = IterableBondAdInfoAttrs::with_loc(&[], self.orig_loc);
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
                BondAdInfoAttrs::Aggregator(val) => fmt.field("Aggregator", &val),
                BondAdInfoAttrs::NumPorts(val) => fmt.field("NumPorts", &val),
                BondAdInfoAttrs::ActorKey(val) => fmt.field("ActorKey", &val),
                BondAdInfoAttrs::PartnerKey(val) => fmt.field("PartnerKey", &val),
                BondAdInfoAttrs::PartnerMac(val) => fmt.field("PartnerMac", &FormatMac(val)),
            };
        }
        fmt.finish()
    }
}
impl IterableBondAdInfoAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("BondAdInfoAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| BondAdInfoAttrs::attr_from_type(t)),
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
                BondAdInfoAttrs::Aggregator(val) => {
                    if last_off == offset {
                        stack.push(("Aggregator", last_off));
                        break;
                    }
                }
                BondAdInfoAttrs::NumPorts(val) => {
                    if last_off == offset {
                        stack.push(("NumPorts", last_off));
                        break;
                    }
                }
                BondAdInfoAttrs::ActorKey(val) => {
                    if last_off == offset {
                        stack.push(("ActorKey", last_off));
                        break;
                    }
                }
                BondAdInfoAttrs::PartnerKey(val) => {
                    if last_off == offset {
                        stack.push(("PartnerKey", last_off));
                        break;
                    }
                }
                BondAdInfoAttrs::PartnerMac(val) => {
                    if last_off == offset {
                        stack.push(("PartnerMac", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("BondAdInfoAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum BondSlaveAttrs<'a> {
    State(u8),
    MiiStatus(u8),
    LinkFailureCount(u32),
    PermHwaddr(&'a [u8]),
    QueueId(u16),
    AdAggregatorId(u16),
    AdActorOperPortState(u8),
    AdPartnerOperPortState(u16),
    Prio(u32),
}
impl<'a> IterableBondSlaveAttrs<'a> {
    pub fn get_state(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(BondSlaveAttrs::State(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BondSlaveAttrs",
            "State",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mii_status(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(BondSlaveAttrs::MiiStatus(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BondSlaveAttrs",
            "MiiStatus",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_link_failure_count(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(BondSlaveAttrs::LinkFailureCount(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BondSlaveAttrs",
            "LinkFailureCount",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_perm_hwaddr(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(BondSlaveAttrs::PermHwaddr(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BondSlaveAttrs",
            "PermHwaddr",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_queue_id(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(BondSlaveAttrs::QueueId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BondSlaveAttrs",
            "QueueId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ad_aggregator_id(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(BondSlaveAttrs::AdAggregatorId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BondSlaveAttrs",
            "AdAggregatorId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ad_actor_oper_port_state(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(BondSlaveAttrs::AdActorOperPortState(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BondSlaveAttrs",
            "AdActorOperPortState",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ad_partner_oper_port_state(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(BondSlaveAttrs::AdPartnerOperPortState(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BondSlaveAttrs",
            "AdPartnerOperPortState",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_prio(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(BondSlaveAttrs::Prio(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BondSlaveAttrs",
            "Prio",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl BondSlaveAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableBondSlaveAttrs<'a> {
        IterableBondSlaveAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "State",
            2u16 => "MiiStatus",
            3u16 => "LinkFailureCount",
            4u16 => "PermHwaddr",
            5u16 => "QueueId",
            6u16 => "AdAggregatorId",
            7u16 => "AdActorOperPortState",
            8u16 => "AdPartnerOperPortState",
            9u16 => "Prio",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableBondSlaveAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableBondSlaveAttrs<'a> {
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
impl<'a> Iterator for IterableBondSlaveAttrs<'a> {
    type Item = Result<BondSlaveAttrs<'a>, ErrorContext>;
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
                1u16 => BondSlaveAttrs::State({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => BondSlaveAttrs::MiiStatus({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => BondSlaveAttrs::LinkFailureCount({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => BondSlaveAttrs::PermHwaddr({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => BondSlaveAttrs::QueueId({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => BondSlaveAttrs::AdAggregatorId({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => BondSlaveAttrs::AdActorOperPortState({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => BondSlaveAttrs::AdPartnerOperPortState({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => BondSlaveAttrs::Prio({
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
            "BondSlaveAttrs",
            r#type.and_then(|t| BondSlaveAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableBondSlaveAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("BondSlaveAttrs");
        let mut iter = IterableBondSlaveAttrs::with_loc(&[], self.orig_loc);
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
                BondSlaveAttrs::State(val) => fmt.field("State", &val),
                BondSlaveAttrs::MiiStatus(val) => fmt.field("MiiStatus", &val),
                BondSlaveAttrs::LinkFailureCount(val) => fmt.field("LinkFailureCount", &val),
                BondSlaveAttrs::PermHwaddr(val) => fmt.field("PermHwaddr", &FormatMac(val)),
                BondSlaveAttrs::QueueId(val) => fmt.field("QueueId", &val),
                BondSlaveAttrs::AdAggregatorId(val) => fmt.field("AdAggregatorId", &val),
                BondSlaveAttrs::AdActorOperPortState(val) => {
                    fmt.field("AdActorOperPortState", &val)
                }
                BondSlaveAttrs::AdPartnerOperPortState(val) => {
                    fmt.field("AdPartnerOperPortState", &val)
                }
                BondSlaveAttrs::Prio(val) => fmt.field("Prio", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableBondSlaveAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("BondSlaveAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| BondSlaveAttrs::attr_from_type(t)),
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
                BondSlaveAttrs::State(val) => {
                    if last_off == offset {
                        stack.push(("State", last_off));
                        break;
                    }
                }
                BondSlaveAttrs::MiiStatus(val) => {
                    if last_off == offset {
                        stack.push(("MiiStatus", last_off));
                        break;
                    }
                }
                BondSlaveAttrs::LinkFailureCount(val) => {
                    if last_off == offset {
                        stack.push(("LinkFailureCount", last_off));
                        break;
                    }
                }
                BondSlaveAttrs::PermHwaddr(val) => {
                    if last_off == offset {
                        stack.push(("PermHwaddr", last_off));
                        break;
                    }
                }
                BondSlaveAttrs::QueueId(val) => {
                    if last_off == offset {
                        stack.push(("QueueId", last_off));
                        break;
                    }
                }
                BondSlaveAttrs::AdAggregatorId(val) => {
                    if last_off == offset {
                        stack.push(("AdAggregatorId", last_off));
                        break;
                    }
                }
                BondSlaveAttrs::AdActorOperPortState(val) => {
                    if last_off == offset {
                        stack.push(("AdActorOperPortState", last_off));
                        break;
                    }
                }
                BondSlaveAttrs::AdPartnerOperPortState(val) => {
                    if last_off == offset {
                        stack.push(("AdPartnerOperPortState", last_off));
                        break;
                    }
                }
                BondSlaveAttrs::Prio(val) => {
                    if last_off == offset {
                        stack.push(("Prio", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("BondSlaveAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LinkinfoBridgeAttrs<'a> {
    ForwardDelay(u32),
    HelloTime(u32),
    MaxAge(u32),
    AgeingTime(u32),
    StpState(u32),
    Priority(u16),
    VlanFiltering(u8),
    VlanProtocol(u16),
    GroupFwdMask(u16),
    RootId(IflaBridgeId),
    BridgeId(IflaBridgeId),
    RootPort(u16),
    RootPathCost(u32),
    TopologyChange(u8),
    TopologyChangeDetected(u8),
    HelloTimer(u64),
    TcnTimer(u64),
    TopologyChangeTimer(u64),
    GcTimer(u64),
    GroupAddr(&'a [u8]),
    FdbFlush(&'a [u8]),
    McastRouter(u8),
    McastSnooping(u8),
    McastQueryUseIfaddr(u8),
    McastQuerier(u8),
    McastHashElasticity(u32),
    McastHashMax(u32),
    McastLastMemberCnt(u32),
    McastStartupQueryCnt(u32),
    McastLastMemberIntvl(u64),
    McastMembershipIntvl(u64),
    McastQuerierIntvl(u64),
    McastQueryIntvl(u64),
    McastQueryResponseIntvl(u64),
    McastStartupQueryIntvl(u64),
    NfCallIptables(u8),
    NfCallIp6tables(u8),
    NfCallArptables(u8),
    VlanDefaultPvid(u16),
    Pad(&'a [u8]),
    VlanStatsEnabled(u8),
    McastStatsEnabled(u8),
    McastIgmpVersion(u8),
    McastMldVersion(u8),
    VlanStatsPerPort(u8),
    MultiBoolopt(BrBooloptMulti),
    McastQuerierState(&'a [u8]),
    FdbNLearned(u32),
    FdbMaxLearned(u32),
    #[doc = "Associated type: [`BrStpMode`] (enum)"]
    StpMode(u32),
}
impl<'a> IterableLinkinfoBridgeAttrs<'a> {
    pub fn get_forward_delay(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::ForwardDelay(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "ForwardDelay",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_hello_time(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::HelloTime(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "HelloTime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_age(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::MaxAge(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "MaxAge",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ageing_time(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::AgeingTime(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "AgeingTime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_stp_state(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::StpState(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "StpState",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_priority(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::Priority(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "Priority",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vlan_filtering(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::VlanFiltering(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "VlanFiltering",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vlan_protocol(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::VlanProtocol(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "VlanProtocol",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_group_fwd_mask(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::GroupFwdMask(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "GroupFwdMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_root_id(&self) -> Result<IflaBridgeId, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::RootId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "RootId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_bridge_id(&self) -> Result<IflaBridgeId, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::BridgeId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "BridgeId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_root_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::RootPort(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "RootPort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_root_path_cost(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::RootPathCost(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "RootPathCost",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_topology_change(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::TopologyChange(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "TopologyChange",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_topology_change_detected(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::TopologyChangeDetected(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "TopologyChangeDetected",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_hello_timer(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::HelloTimer(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "HelloTimer",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tcn_timer(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::TcnTimer(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "TcnTimer",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_topology_change_timer(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::TopologyChangeTimer(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "TopologyChangeTimer",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_gc_timer(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::GcTimer(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "GcTimer",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_group_addr(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::GroupAddr(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "GroupAddr",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fdb_flush(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::FdbFlush(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "FdbFlush",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_router(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::McastRouter(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "McastRouter",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_snooping(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::McastSnooping(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "McastSnooping",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_query_use_ifaddr(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::McastQueryUseIfaddr(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "McastQueryUseIfaddr",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_querier(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::McastQuerier(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "McastQuerier",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_hash_elasticity(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::McastHashElasticity(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "McastHashElasticity",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_hash_max(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::McastHashMax(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "McastHashMax",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_last_member_cnt(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::McastLastMemberCnt(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "McastLastMemberCnt",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_startup_query_cnt(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::McastStartupQueryCnt(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "McastStartupQueryCnt",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_last_member_intvl(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::McastLastMemberIntvl(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "McastLastMemberIntvl",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_membership_intvl(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::McastMembershipIntvl(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "McastMembershipIntvl",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_querier_intvl(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::McastQuerierIntvl(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "McastQuerierIntvl",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_query_intvl(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::McastQueryIntvl(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "McastQueryIntvl",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_query_response_intvl(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::McastQueryResponseIntvl(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "McastQueryResponseIntvl",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_startup_query_intvl(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::McastStartupQueryIntvl(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "McastStartupQueryIntvl",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nf_call_iptables(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::NfCallIptables(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "NfCallIptables",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nf_call_ip6tables(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::NfCallIp6tables(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "NfCallIp6tables",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nf_call_arptables(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::NfCallArptables(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "NfCallArptables",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vlan_default_pvid(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::VlanDefaultPvid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "VlanDefaultPvid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vlan_stats_enabled(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::VlanStatsEnabled(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "VlanStatsEnabled",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_stats_enabled(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::McastStatsEnabled(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "McastStatsEnabled",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_igmp_version(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::McastIgmpVersion(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "McastIgmpVersion",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_mld_version(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::McastMldVersion(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "McastMldVersion",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vlan_stats_per_port(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::VlanStatsPerPort(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "VlanStatsPerPort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_multi_boolopt(&self) -> Result<BrBooloptMulti, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::MultiBoolopt(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "MultiBoolopt",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_querier_state(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::McastQuerierState(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "McastQuerierState",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fdb_n_learned(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::FdbNLearned(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "FdbNLearned",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fdb_max_learned(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::FdbMaxLearned(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "FdbMaxLearned",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`BrStpMode`] (enum)"]
    pub fn get_stp_mode(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBridgeAttrs::StpMode(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBridgeAttrs",
            "StpMode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LinkinfoBridgeAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLinkinfoBridgeAttrs<'a> {
        IterableLinkinfoBridgeAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "ForwardDelay",
            2u16 => "HelloTime",
            3u16 => "MaxAge",
            4u16 => "AgeingTime",
            5u16 => "StpState",
            6u16 => "Priority",
            7u16 => "VlanFiltering",
            8u16 => "VlanProtocol",
            9u16 => "GroupFwdMask",
            10u16 => "RootId",
            11u16 => "BridgeId",
            12u16 => "RootPort",
            13u16 => "RootPathCost",
            14u16 => "TopologyChange",
            15u16 => "TopologyChangeDetected",
            16u16 => "HelloTimer",
            17u16 => "TcnTimer",
            18u16 => "TopologyChangeTimer",
            19u16 => "GcTimer",
            20u16 => "GroupAddr",
            21u16 => "FdbFlush",
            22u16 => "McastRouter",
            23u16 => "McastSnooping",
            24u16 => "McastQueryUseIfaddr",
            25u16 => "McastQuerier",
            26u16 => "McastHashElasticity",
            27u16 => "McastHashMax",
            28u16 => "McastLastMemberCnt",
            29u16 => "McastStartupQueryCnt",
            30u16 => "McastLastMemberIntvl",
            31u16 => "McastMembershipIntvl",
            32u16 => "McastQuerierIntvl",
            33u16 => "McastQueryIntvl",
            34u16 => "McastQueryResponseIntvl",
            35u16 => "McastStartupQueryIntvl",
            36u16 => "NfCallIptables",
            37u16 => "NfCallIp6tables",
            38u16 => "NfCallArptables",
            39u16 => "VlanDefaultPvid",
            40u16 => "Pad",
            41u16 => "VlanStatsEnabled",
            42u16 => "McastStatsEnabled",
            43u16 => "McastIgmpVersion",
            44u16 => "McastMldVersion",
            45u16 => "VlanStatsPerPort",
            46u16 => "MultiBoolopt",
            47u16 => "McastQuerierState",
            48u16 => "FdbNLearned",
            49u16 => "FdbMaxLearned",
            50u16 => "StpMode",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLinkinfoBridgeAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLinkinfoBridgeAttrs<'a> {
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
impl<'a> Iterator for IterableLinkinfoBridgeAttrs<'a> {
    type Item = Result<LinkinfoBridgeAttrs<'a>, ErrorContext>;
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
                1u16 => LinkinfoBridgeAttrs::ForwardDelay({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => LinkinfoBridgeAttrs::HelloTime({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => LinkinfoBridgeAttrs::MaxAge({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => LinkinfoBridgeAttrs::AgeingTime({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => LinkinfoBridgeAttrs::StpState({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => LinkinfoBridgeAttrs::Priority({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => LinkinfoBridgeAttrs::VlanFiltering({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => LinkinfoBridgeAttrs::VlanProtocol({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => LinkinfoBridgeAttrs::GroupFwdMask({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => LinkinfoBridgeAttrs::RootId({
                    let res = Some(IflaBridgeId::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => LinkinfoBridgeAttrs::BridgeId({
                    let res = Some(IflaBridgeId::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => LinkinfoBridgeAttrs::RootPort({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => LinkinfoBridgeAttrs::RootPathCost({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => LinkinfoBridgeAttrs::TopologyChange({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => LinkinfoBridgeAttrs::TopologyChangeDetected({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => LinkinfoBridgeAttrs::HelloTimer({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => LinkinfoBridgeAttrs::TcnTimer({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => LinkinfoBridgeAttrs::TopologyChangeTimer({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => LinkinfoBridgeAttrs::GcTimer({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => LinkinfoBridgeAttrs::GroupAddr({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => LinkinfoBridgeAttrs::FdbFlush({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => LinkinfoBridgeAttrs::McastRouter({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => LinkinfoBridgeAttrs::McastSnooping({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => LinkinfoBridgeAttrs::McastQueryUseIfaddr({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                25u16 => LinkinfoBridgeAttrs::McastQuerier({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                26u16 => LinkinfoBridgeAttrs::McastHashElasticity({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                27u16 => LinkinfoBridgeAttrs::McastHashMax({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                28u16 => LinkinfoBridgeAttrs::McastLastMemberCnt({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                29u16 => LinkinfoBridgeAttrs::McastStartupQueryCnt({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                30u16 => LinkinfoBridgeAttrs::McastLastMemberIntvl({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                31u16 => LinkinfoBridgeAttrs::McastMembershipIntvl({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                32u16 => LinkinfoBridgeAttrs::McastQuerierIntvl({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                33u16 => LinkinfoBridgeAttrs::McastQueryIntvl({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                34u16 => LinkinfoBridgeAttrs::McastQueryResponseIntvl({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                35u16 => LinkinfoBridgeAttrs::McastStartupQueryIntvl({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                36u16 => LinkinfoBridgeAttrs::NfCallIptables({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                37u16 => LinkinfoBridgeAttrs::NfCallIp6tables({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                38u16 => LinkinfoBridgeAttrs::NfCallArptables({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                39u16 => LinkinfoBridgeAttrs::VlanDefaultPvid({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                40u16 => LinkinfoBridgeAttrs::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                41u16 => LinkinfoBridgeAttrs::VlanStatsEnabled({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                42u16 => LinkinfoBridgeAttrs::McastStatsEnabled({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                43u16 => LinkinfoBridgeAttrs::McastIgmpVersion({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                44u16 => LinkinfoBridgeAttrs::McastMldVersion({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                45u16 => LinkinfoBridgeAttrs::VlanStatsPerPort({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                46u16 => LinkinfoBridgeAttrs::MultiBoolopt({
                    let res = Some(BrBooloptMulti::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                47u16 => LinkinfoBridgeAttrs::McastQuerierState({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                48u16 => LinkinfoBridgeAttrs::FdbNLearned({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                49u16 => LinkinfoBridgeAttrs::FdbMaxLearned({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                50u16 => LinkinfoBridgeAttrs::StpMode({
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
            "LinkinfoBridgeAttrs",
            r#type.and_then(|t| LinkinfoBridgeAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableLinkinfoBridgeAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LinkinfoBridgeAttrs");
        let mut iter = IterableLinkinfoBridgeAttrs::with_loc(&[], self.orig_loc);
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
                LinkinfoBridgeAttrs::ForwardDelay(val) => fmt.field("ForwardDelay", &val),
                LinkinfoBridgeAttrs::HelloTime(val) => fmt.field("HelloTime", &val),
                LinkinfoBridgeAttrs::MaxAge(val) => fmt.field("MaxAge", &val),
                LinkinfoBridgeAttrs::AgeingTime(val) => fmt.field("AgeingTime", &val),
                LinkinfoBridgeAttrs::StpState(val) => fmt.field("StpState", &val),
                LinkinfoBridgeAttrs::Priority(val) => fmt.field("Priority", &val),
                LinkinfoBridgeAttrs::VlanFiltering(val) => fmt.field("VlanFiltering", &val),
                LinkinfoBridgeAttrs::VlanProtocol(val) => fmt.field("VlanProtocol", &val),
                LinkinfoBridgeAttrs::GroupFwdMask(val) => fmt.field("GroupFwdMask", &val),
                LinkinfoBridgeAttrs::RootId(val) => fmt.field("RootId", &val),
                LinkinfoBridgeAttrs::BridgeId(val) => fmt.field("BridgeId", &val),
                LinkinfoBridgeAttrs::RootPort(val) => fmt.field("RootPort", &val),
                LinkinfoBridgeAttrs::RootPathCost(val) => fmt.field("RootPathCost", &val),
                LinkinfoBridgeAttrs::TopologyChange(val) => fmt.field("TopologyChange", &val),
                LinkinfoBridgeAttrs::TopologyChangeDetected(val) => {
                    fmt.field("TopologyChangeDetected", &val)
                }
                LinkinfoBridgeAttrs::HelloTimer(val) => fmt.field("HelloTimer", &val),
                LinkinfoBridgeAttrs::TcnTimer(val) => fmt.field("TcnTimer", &val),
                LinkinfoBridgeAttrs::TopologyChangeTimer(val) => {
                    fmt.field("TopologyChangeTimer", &val)
                }
                LinkinfoBridgeAttrs::GcTimer(val) => fmt.field("GcTimer", &val),
                LinkinfoBridgeAttrs::GroupAddr(val) => fmt.field("GroupAddr", &FormatMac(val)),
                LinkinfoBridgeAttrs::FdbFlush(val) => fmt.field("FdbFlush", &FormatHexdump(val)),
                LinkinfoBridgeAttrs::McastRouter(val) => fmt.field("McastRouter", &val),
                LinkinfoBridgeAttrs::McastSnooping(val) => fmt.field("McastSnooping", &val),
                LinkinfoBridgeAttrs::McastQueryUseIfaddr(val) => {
                    fmt.field("McastQueryUseIfaddr", &val)
                }
                LinkinfoBridgeAttrs::McastQuerier(val) => fmt.field("McastQuerier", &val),
                LinkinfoBridgeAttrs::McastHashElasticity(val) => {
                    fmt.field("McastHashElasticity", &val)
                }
                LinkinfoBridgeAttrs::McastHashMax(val) => fmt.field("McastHashMax", &val),
                LinkinfoBridgeAttrs::McastLastMemberCnt(val) => {
                    fmt.field("McastLastMemberCnt", &val)
                }
                LinkinfoBridgeAttrs::McastStartupQueryCnt(val) => {
                    fmt.field("McastStartupQueryCnt", &val)
                }
                LinkinfoBridgeAttrs::McastLastMemberIntvl(val) => {
                    fmt.field("McastLastMemberIntvl", &val)
                }
                LinkinfoBridgeAttrs::McastMembershipIntvl(val) => {
                    fmt.field("McastMembershipIntvl", &val)
                }
                LinkinfoBridgeAttrs::McastQuerierIntvl(val) => fmt.field("McastQuerierIntvl", &val),
                LinkinfoBridgeAttrs::McastQueryIntvl(val) => fmt.field("McastQueryIntvl", &val),
                LinkinfoBridgeAttrs::McastQueryResponseIntvl(val) => {
                    fmt.field("McastQueryResponseIntvl", &val)
                }
                LinkinfoBridgeAttrs::McastStartupQueryIntvl(val) => {
                    fmt.field("McastStartupQueryIntvl", &val)
                }
                LinkinfoBridgeAttrs::NfCallIptables(val) => fmt.field("NfCallIptables", &val),
                LinkinfoBridgeAttrs::NfCallIp6tables(val) => fmt.field("NfCallIp6tables", &val),
                LinkinfoBridgeAttrs::NfCallArptables(val) => fmt.field("NfCallArptables", &val),
                LinkinfoBridgeAttrs::VlanDefaultPvid(val) => fmt.field("VlanDefaultPvid", &val),
                LinkinfoBridgeAttrs::Pad(val) => fmt.field("Pad", &val),
                LinkinfoBridgeAttrs::VlanStatsEnabled(val) => fmt.field("VlanStatsEnabled", &val),
                LinkinfoBridgeAttrs::McastStatsEnabled(val) => fmt.field("McastStatsEnabled", &val),
                LinkinfoBridgeAttrs::McastIgmpVersion(val) => fmt.field("McastIgmpVersion", &val),
                LinkinfoBridgeAttrs::McastMldVersion(val) => fmt.field("McastMldVersion", &val),
                LinkinfoBridgeAttrs::VlanStatsPerPort(val) => fmt.field("VlanStatsPerPort", &val),
                LinkinfoBridgeAttrs::MultiBoolopt(val) => fmt.field("MultiBoolopt", &val),
                LinkinfoBridgeAttrs::McastQuerierState(val) => {
                    fmt.field("McastQuerierState", &FormatHexdump(val))
                }
                LinkinfoBridgeAttrs::FdbNLearned(val) => fmt.field("FdbNLearned", &val),
                LinkinfoBridgeAttrs::FdbMaxLearned(val) => fmt.field("FdbMaxLearned", &val),
                LinkinfoBridgeAttrs::StpMode(val) => {
                    fmt.field("StpMode", &FormatEnum(val.into(), BrStpMode::from_value))
                }
            };
        }
        fmt.finish()
    }
}
impl IterableLinkinfoBridgeAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LinkinfoBridgeAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| LinkinfoBridgeAttrs::attr_from_type(t)),
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
                LinkinfoBridgeAttrs::ForwardDelay(val) => {
                    if last_off == offset {
                        stack.push(("ForwardDelay", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::HelloTime(val) => {
                    if last_off == offset {
                        stack.push(("HelloTime", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::MaxAge(val) => {
                    if last_off == offset {
                        stack.push(("MaxAge", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::AgeingTime(val) => {
                    if last_off == offset {
                        stack.push(("AgeingTime", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::StpState(val) => {
                    if last_off == offset {
                        stack.push(("StpState", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::Priority(val) => {
                    if last_off == offset {
                        stack.push(("Priority", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::VlanFiltering(val) => {
                    if last_off == offset {
                        stack.push(("VlanFiltering", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::VlanProtocol(val) => {
                    if last_off == offset {
                        stack.push(("VlanProtocol", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::GroupFwdMask(val) => {
                    if last_off == offset {
                        stack.push(("GroupFwdMask", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::RootId(val) => {
                    if last_off == offset {
                        stack.push(("RootId", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::BridgeId(val) => {
                    if last_off == offset {
                        stack.push(("BridgeId", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::RootPort(val) => {
                    if last_off == offset {
                        stack.push(("RootPort", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::RootPathCost(val) => {
                    if last_off == offset {
                        stack.push(("RootPathCost", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::TopologyChange(val) => {
                    if last_off == offset {
                        stack.push(("TopologyChange", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::TopologyChangeDetected(val) => {
                    if last_off == offset {
                        stack.push(("TopologyChangeDetected", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::HelloTimer(val) => {
                    if last_off == offset {
                        stack.push(("HelloTimer", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::TcnTimer(val) => {
                    if last_off == offset {
                        stack.push(("TcnTimer", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::TopologyChangeTimer(val) => {
                    if last_off == offset {
                        stack.push(("TopologyChangeTimer", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::GcTimer(val) => {
                    if last_off == offset {
                        stack.push(("GcTimer", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::GroupAddr(val) => {
                    if last_off == offset {
                        stack.push(("GroupAddr", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::FdbFlush(val) => {
                    if last_off == offset {
                        stack.push(("FdbFlush", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::McastRouter(val) => {
                    if last_off == offset {
                        stack.push(("McastRouter", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::McastSnooping(val) => {
                    if last_off == offset {
                        stack.push(("McastSnooping", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::McastQueryUseIfaddr(val) => {
                    if last_off == offset {
                        stack.push(("McastQueryUseIfaddr", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::McastQuerier(val) => {
                    if last_off == offset {
                        stack.push(("McastQuerier", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::McastHashElasticity(val) => {
                    if last_off == offset {
                        stack.push(("McastHashElasticity", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::McastHashMax(val) => {
                    if last_off == offset {
                        stack.push(("McastHashMax", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::McastLastMemberCnt(val) => {
                    if last_off == offset {
                        stack.push(("McastLastMemberCnt", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::McastStartupQueryCnt(val) => {
                    if last_off == offset {
                        stack.push(("McastStartupQueryCnt", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::McastLastMemberIntvl(val) => {
                    if last_off == offset {
                        stack.push(("McastLastMemberIntvl", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::McastMembershipIntvl(val) => {
                    if last_off == offset {
                        stack.push(("McastMembershipIntvl", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::McastQuerierIntvl(val) => {
                    if last_off == offset {
                        stack.push(("McastQuerierIntvl", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::McastQueryIntvl(val) => {
                    if last_off == offset {
                        stack.push(("McastQueryIntvl", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::McastQueryResponseIntvl(val) => {
                    if last_off == offset {
                        stack.push(("McastQueryResponseIntvl", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::McastStartupQueryIntvl(val) => {
                    if last_off == offset {
                        stack.push(("McastStartupQueryIntvl", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::NfCallIptables(val) => {
                    if last_off == offset {
                        stack.push(("NfCallIptables", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::NfCallIp6tables(val) => {
                    if last_off == offset {
                        stack.push(("NfCallIp6tables", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::NfCallArptables(val) => {
                    if last_off == offset {
                        stack.push(("NfCallArptables", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::VlanDefaultPvid(val) => {
                    if last_off == offset {
                        stack.push(("VlanDefaultPvid", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::VlanStatsEnabled(val) => {
                    if last_off == offset {
                        stack.push(("VlanStatsEnabled", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::McastStatsEnabled(val) => {
                    if last_off == offset {
                        stack.push(("McastStatsEnabled", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::McastIgmpVersion(val) => {
                    if last_off == offset {
                        stack.push(("McastIgmpVersion", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::McastMldVersion(val) => {
                    if last_off == offset {
                        stack.push(("McastMldVersion", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::VlanStatsPerPort(val) => {
                    if last_off == offset {
                        stack.push(("VlanStatsPerPort", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::MultiBoolopt(val) => {
                    if last_off == offset {
                        stack.push(("MultiBoolopt", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::McastQuerierState(val) => {
                    if last_off == offset {
                        stack.push(("McastQuerierState", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::FdbNLearned(val) => {
                    if last_off == offset {
                        stack.push(("FdbNLearned", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::FdbMaxLearned(val) => {
                    if last_off == offset {
                        stack.push(("FdbMaxLearned", last_off));
                        break;
                    }
                }
                LinkinfoBridgeAttrs::StpMode(val) => {
                    if last_off == offset {
                        stack.push(("StpMode", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LinkinfoBridgeAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LinkinfoBrportAttrs<'a> {
    State(u8),
    Priority(u16),
    Cost(u32),
    Mode(()),
    Guard(()),
    Protect(()),
    FastLeave(()),
    Learning(()),
    UnicastFlood(()),
    Proxyarp(()),
    LearningSync(()),
    ProxyarpWifi(()),
    RootId(IflaBridgeId),
    BridgeId(IflaBridgeId),
    DesignatedPort(u16),
    DesignatedCost(u16),
    Id(u16),
    No(u16),
    TopologyChangeAck(u8),
    ConfigPending(u8),
    MessageAgeTimer(u64),
    ForwardDelayTimer(u64),
    HoldTimer(u64),
    Flush(()),
    MulticastRouter(u8),
    Pad(&'a [u8]),
    McastFlood(()),
    McastToUcast(()),
    VlanTunnel(()),
    BcastFlood(()),
    GroupFwdMask(u16),
    NeighSuppress(()),
    Isolated(()),
    BackupPort(u32),
    MrpRingOpen(()),
    MrpInOpen(()),
    McastEhtHostsLimit(u32),
    McastEhtHostsCnt(u32),
    Locked(()),
    Mab(()),
    McastNGroups(u32),
    McastMaxGroups(u32),
    NeighVlanSuppress(()),
    BackupNhid(u32),
    NeighForwardGrat(u8),
}
impl<'a> IterableLinkinfoBrportAttrs<'a> {
    pub fn get_state(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::State(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "State",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_priority(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::Priority(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "Priority",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_cost(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::Cost(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "Cost",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mode(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::Mode(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "Mode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_guard(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::Guard(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "Guard",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_protect(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::Protect(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "Protect",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fast_leave(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::FastLeave(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "FastLeave",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_learning(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::Learning(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "Learning",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_unicast_flood(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::UnicastFlood(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "UnicastFlood",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_proxyarp(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::Proxyarp(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "Proxyarp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_learning_sync(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::LearningSync(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "LearningSync",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_proxyarp_wifi(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::ProxyarpWifi(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "ProxyarpWifi",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_root_id(&self) -> Result<IflaBridgeId, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::RootId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "RootId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_bridge_id(&self) -> Result<IflaBridgeId, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::BridgeId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "BridgeId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_designated_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::DesignatedPort(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "DesignatedPort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_designated_cost(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::DesignatedCost(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "DesignatedCost",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_id(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_no(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::No(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "No",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_topology_change_ack(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::TopologyChangeAck(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "TopologyChangeAck",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_config_pending(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::ConfigPending(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "ConfigPending",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_message_age_timer(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::MessageAgeTimer(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "MessageAgeTimer",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_forward_delay_timer(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::ForwardDelayTimer(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "ForwardDelayTimer",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_hold_timer(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::HoldTimer(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "HoldTimer",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flush(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::Flush(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "Flush",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_multicast_router(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::MulticastRouter(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "MulticastRouter",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_flood(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::McastFlood(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "McastFlood",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_to_ucast(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::McastToUcast(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "McastToUcast",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vlan_tunnel(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::VlanTunnel(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "VlanTunnel",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_bcast_flood(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::BcastFlood(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "BcastFlood",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_group_fwd_mask(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::GroupFwdMask(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "GroupFwdMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_neigh_suppress(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::NeighSuppress(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "NeighSuppress",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_isolated(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::Isolated(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "Isolated",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_backup_port(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::BackupPort(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "BackupPort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mrp_ring_open(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::MrpRingOpen(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "MrpRingOpen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mrp_in_open(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::MrpInOpen(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "MrpInOpen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_eht_hosts_limit(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::McastEhtHostsLimit(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "McastEhtHostsLimit",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_eht_hosts_cnt(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::McastEhtHostsCnt(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "McastEhtHostsCnt",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_locked(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::Locked(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "Locked",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mab(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::Mab(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "Mab",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_n_groups(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::McastNGroups(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "McastNGroups",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_max_groups(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::McastMaxGroups(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "McastMaxGroups",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_neigh_vlan_suppress(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::NeighVlanSuppress(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "NeighVlanSuppress",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_backup_nhid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::BackupNhid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "BackupNhid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_neigh_forward_grat(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoBrportAttrs::NeighForwardGrat(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoBrportAttrs",
            "NeighForwardGrat",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LinkinfoBrportAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLinkinfoBrportAttrs<'a> {
        IterableLinkinfoBrportAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "State",
            2u16 => "Priority",
            3u16 => "Cost",
            4u16 => "Mode",
            5u16 => "Guard",
            6u16 => "Protect",
            7u16 => "FastLeave",
            8u16 => "Learning",
            9u16 => "UnicastFlood",
            10u16 => "Proxyarp",
            11u16 => "LearningSync",
            12u16 => "ProxyarpWifi",
            13u16 => "RootId",
            14u16 => "BridgeId",
            15u16 => "DesignatedPort",
            16u16 => "DesignatedCost",
            17u16 => "Id",
            18u16 => "No",
            19u16 => "TopologyChangeAck",
            20u16 => "ConfigPending",
            21u16 => "MessageAgeTimer",
            22u16 => "ForwardDelayTimer",
            23u16 => "HoldTimer",
            24u16 => "Flush",
            25u16 => "MulticastRouter",
            26u16 => "Pad",
            27u16 => "McastFlood",
            28u16 => "McastToUcast",
            29u16 => "VlanTunnel",
            30u16 => "BcastFlood",
            31u16 => "GroupFwdMask",
            32u16 => "NeighSuppress",
            33u16 => "Isolated",
            34u16 => "BackupPort",
            35u16 => "MrpRingOpen",
            36u16 => "MrpInOpen",
            37u16 => "McastEhtHostsLimit",
            38u16 => "McastEhtHostsCnt",
            39u16 => "Locked",
            40u16 => "Mab",
            41u16 => "McastNGroups",
            42u16 => "McastMaxGroups",
            43u16 => "NeighVlanSuppress",
            44u16 => "BackupNhid",
            45u16 => "NeighForwardGrat",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLinkinfoBrportAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLinkinfoBrportAttrs<'a> {
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
impl<'a> Iterator for IterableLinkinfoBrportAttrs<'a> {
    type Item = Result<LinkinfoBrportAttrs<'a>, ErrorContext>;
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
                1u16 => LinkinfoBrportAttrs::State({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => LinkinfoBrportAttrs::Priority({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => LinkinfoBrportAttrs::Cost({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => LinkinfoBrportAttrs::Mode(()),
                5u16 => LinkinfoBrportAttrs::Guard(()),
                6u16 => LinkinfoBrportAttrs::Protect(()),
                7u16 => LinkinfoBrportAttrs::FastLeave(()),
                8u16 => LinkinfoBrportAttrs::Learning(()),
                9u16 => LinkinfoBrportAttrs::UnicastFlood(()),
                10u16 => LinkinfoBrportAttrs::Proxyarp(()),
                11u16 => LinkinfoBrportAttrs::LearningSync(()),
                12u16 => LinkinfoBrportAttrs::ProxyarpWifi(()),
                13u16 => LinkinfoBrportAttrs::RootId({
                    let res = Some(IflaBridgeId::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => LinkinfoBrportAttrs::BridgeId({
                    let res = Some(IflaBridgeId::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => LinkinfoBrportAttrs::DesignatedPort({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => LinkinfoBrportAttrs::DesignatedCost({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => LinkinfoBrportAttrs::Id({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => LinkinfoBrportAttrs::No({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => LinkinfoBrportAttrs::TopologyChangeAck({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => LinkinfoBrportAttrs::ConfigPending({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => LinkinfoBrportAttrs::MessageAgeTimer({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => LinkinfoBrportAttrs::ForwardDelayTimer({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => LinkinfoBrportAttrs::HoldTimer({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => LinkinfoBrportAttrs::Flush(()),
                25u16 => LinkinfoBrportAttrs::MulticastRouter({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                26u16 => LinkinfoBrportAttrs::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                27u16 => LinkinfoBrportAttrs::McastFlood(()),
                28u16 => LinkinfoBrportAttrs::McastToUcast(()),
                29u16 => LinkinfoBrportAttrs::VlanTunnel(()),
                30u16 => LinkinfoBrportAttrs::BcastFlood(()),
                31u16 => LinkinfoBrportAttrs::GroupFwdMask({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                32u16 => LinkinfoBrportAttrs::NeighSuppress(()),
                33u16 => LinkinfoBrportAttrs::Isolated(()),
                34u16 => LinkinfoBrportAttrs::BackupPort({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                35u16 => LinkinfoBrportAttrs::MrpRingOpen(()),
                36u16 => LinkinfoBrportAttrs::MrpInOpen(()),
                37u16 => LinkinfoBrportAttrs::McastEhtHostsLimit({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                38u16 => LinkinfoBrportAttrs::McastEhtHostsCnt({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                39u16 => LinkinfoBrportAttrs::Locked(()),
                40u16 => LinkinfoBrportAttrs::Mab(()),
                41u16 => LinkinfoBrportAttrs::McastNGroups({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                42u16 => LinkinfoBrportAttrs::McastMaxGroups({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                43u16 => LinkinfoBrportAttrs::NeighVlanSuppress(()),
                44u16 => LinkinfoBrportAttrs::BackupNhid({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                45u16 => LinkinfoBrportAttrs::NeighForwardGrat({
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
            "LinkinfoBrportAttrs",
            r#type.and_then(|t| LinkinfoBrportAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableLinkinfoBrportAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LinkinfoBrportAttrs");
        let mut iter = IterableLinkinfoBrportAttrs::with_loc(&[], self.orig_loc);
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
                LinkinfoBrportAttrs::State(val) => fmt.field("State", &val),
                LinkinfoBrportAttrs::Priority(val) => fmt.field("Priority", &val),
                LinkinfoBrportAttrs::Cost(val) => fmt.field("Cost", &val),
                LinkinfoBrportAttrs::Mode(val) => fmt.field("Mode", &val),
                LinkinfoBrportAttrs::Guard(val) => fmt.field("Guard", &val),
                LinkinfoBrportAttrs::Protect(val) => fmt.field("Protect", &val),
                LinkinfoBrportAttrs::FastLeave(val) => fmt.field("FastLeave", &val),
                LinkinfoBrportAttrs::Learning(val) => fmt.field("Learning", &val),
                LinkinfoBrportAttrs::UnicastFlood(val) => fmt.field("UnicastFlood", &val),
                LinkinfoBrportAttrs::Proxyarp(val) => fmt.field("Proxyarp", &val),
                LinkinfoBrportAttrs::LearningSync(val) => fmt.field("LearningSync", &val),
                LinkinfoBrportAttrs::ProxyarpWifi(val) => fmt.field("ProxyarpWifi", &val),
                LinkinfoBrportAttrs::RootId(val) => fmt.field("RootId", &val),
                LinkinfoBrportAttrs::BridgeId(val) => fmt.field("BridgeId", &val),
                LinkinfoBrportAttrs::DesignatedPort(val) => fmt.field("DesignatedPort", &val),
                LinkinfoBrportAttrs::DesignatedCost(val) => fmt.field("DesignatedCost", &val),
                LinkinfoBrportAttrs::Id(val) => fmt.field("Id", &val),
                LinkinfoBrportAttrs::No(val) => fmt.field("No", &val),
                LinkinfoBrportAttrs::TopologyChangeAck(val) => fmt.field("TopologyChangeAck", &val),
                LinkinfoBrportAttrs::ConfigPending(val) => fmt.field("ConfigPending", &val),
                LinkinfoBrportAttrs::MessageAgeTimer(val) => fmt.field("MessageAgeTimer", &val),
                LinkinfoBrportAttrs::ForwardDelayTimer(val) => fmt.field("ForwardDelayTimer", &val),
                LinkinfoBrportAttrs::HoldTimer(val) => fmt.field("HoldTimer", &val),
                LinkinfoBrportAttrs::Flush(val) => fmt.field("Flush", &val),
                LinkinfoBrportAttrs::MulticastRouter(val) => fmt.field("MulticastRouter", &val),
                LinkinfoBrportAttrs::Pad(val) => fmt.field("Pad", &val),
                LinkinfoBrportAttrs::McastFlood(val) => fmt.field("McastFlood", &val),
                LinkinfoBrportAttrs::McastToUcast(val) => fmt.field("McastToUcast", &val),
                LinkinfoBrportAttrs::VlanTunnel(val) => fmt.field("VlanTunnel", &val),
                LinkinfoBrportAttrs::BcastFlood(val) => fmt.field("BcastFlood", &val),
                LinkinfoBrportAttrs::GroupFwdMask(val) => fmt.field("GroupFwdMask", &val),
                LinkinfoBrportAttrs::NeighSuppress(val) => fmt.field("NeighSuppress", &val),
                LinkinfoBrportAttrs::Isolated(val) => fmt.field("Isolated", &val),
                LinkinfoBrportAttrs::BackupPort(val) => fmt.field("BackupPort", &val),
                LinkinfoBrportAttrs::MrpRingOpen(val) => fmt.field("MrpRingOpen", &val),
                LinkinfoBrportAttrs::MrpInOpen(val) => fmt.field("MrpInOpen", &val),
                LinkinfoBrportAttrs::McastEhtHostsLimit(val) => {
                    fmt.field("McastEhtHostsLimit", &val)
                }
                LinkinfoBrportAttrs::McastEhtHostsCnt(val) => fmt.field("McastEhtHostsCnt", &val),
                LinkinfoBrportAttrs::Locked(val) => fmt.field("Locked", &val),
                LinkinfoBrportAttrs::Mab(val) => fmt.field("Mab", &val),
                LinkinfoBrportAttrs::McastNGroups(val) => fmt.field("McastNGroups", &val),
                LinkinfoBrportAttrs::McastMaxGroups(val) => fmt.field("McastMaxGroups", &val),
                LinkinfoBrportAttrs::NeighVlanSuppress(val) => fmt.field("NeighVlanSuppress", &val),
                LinkinfoBrportAttrs::BackupNhid(val) => fmt.field("BackupNhid", &val),
                LinkinfoBrportAttrs::NeighForwardGrat(val) => fmt.field("NeighForwardGrat", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableLinkinfoBrportAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LinkinfoBrportAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| LinkinfoBrportAttrs::attr_from_type(t)),
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
                LinkinfoBrportAttrs::State(val) => {
                    if last_off == offset {
                        stack.push(("State", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::Priority(val) => {
                    if last_off == offset {
                        stack.push(("Priority", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::Cost(val) => {
                    if last_off == offset {
                        stack.push(("Cost", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::Mode(val) => {
                    if last_off == offset {
                        stack.push(("Mode", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::Guard(val) => {
                    if last_off == offset {
                        stack.push(("Guard", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::Protect(val) => {
                    if last_off == offset {
                        stack.push(("Protect", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::FastLeave(val) => {
                    if last_off == offset {
                        stack.push(("FastLeave", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::Learning(val) => {
                    if last_off == offset {
                        stack.push(("Learning", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::UnicastFlood(val) => {
                    if last_off == offset {
                        stack.push(("UnicastFlood", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::Proxyarp(val) => {
                    if last_off == offset {
                        stack.push(("Proxyarp", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::LearningSync(val) => {
                    if last_off == offset {
                        stack.push(("LearningSync", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::ProxyarpWifi(val) => {
                    if last_off == offset {
                        stack.push(("ProxyarpWifi", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::RootId(val) => {
                    if last_off == offset {
                        stack.push(("RootId", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::BridgeId(val) => {
                    if last_off == offset {
                        stack.push(("BridgeId", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::DesignatedPort(val) => {
                    if last_off == offset {
                        stack.push(("DesignatedPort", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::DesignatedCost(val) => {
                    if last_off == offset {
                        stack.push(("DesignatedCost", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::No(val) => {
                    if last_off == offset {
                        stack.push(("No", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::TopologyChangeAck(val) => {
                    if last_off == offset {
                        stack.push(("TopologyChangeAck", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::ConfigPending(val) => {
                    if last_off == offset {
                        stack.push(("ConfigPending", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::MessageAgeTimer(val) => {
                    if last_off == offset {
                        stack.push(("MessageAgeTimer", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::ForwardDelayTimer(val) => {
                    if last_off == offset {
                        stack.push(("ForwardDelayTimer", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::HoldTimer(val) => {
                    if last_off == offset {
                        stack.push(("HoldTimer", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::Flush(val) => {
                    if last_off == offset {
                        stack.push(("Flush", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::MulticastRouter(val) => {
                    if last_off == offset {
                        stack.push(("MulticastRouter", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::McastFlood(val) => {
                    if last_off == offset {
                        stack.push(("McastFlood", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::McastToUcast(val) => {
                    if last_off == offset {
                        stack.push(("McastToUcast", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::VlanTunnel(val) => {
                    if last_off == offset {
                        stack.push(("VlanTunnel", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::BcastFlood(val) => {
                    if last_off == offset {
                        stack.push(("BcastFlood", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::GroupFwdMask(val) => {
                    if last_off == offset {
                        stack.push(("GroupFwdMask", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::NeighSuppress(val) => {
                    if last_off == offset {
                        stack.push(("NeighSuppress", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::Isolated(val) => {
                    if last_off == offset {
                        stack.push(("Isolated", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::BackupPort(val) => {
                    if last_off == offset {
                        stack.push(("BackupPort", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::MrpRingOpen(val) => {
                    if last_off == offset {
                        stack.push(("MrpRingOpen", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::MrpInOpen(val) => {
                    if last_off == offset {
                        stack.push(("MrpInOpen", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::McastEhtHostsLimit(val) => {
                    if last_off == offset {
                        stack.push(("McastEhtHostsLimit", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::McastEhtHostsCnt(val) => {
                    if last_off == offset {
                        stack.push(("McastEhtHostsCnt", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::Locked(val) => {
                    if last_off == offset {
                        stack.push(("Locked", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::Mab(val) => {
                    if last_off == offset {
                        stack.push(("Mab", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::McastNGroups(val) => {
                    if last_off == offset {
                        stack.push(("McastNGroups", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::McastMaxGroups(val) => {
                    if last_off == offset {
                        stack.push(("McastMaxGroups", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::NeighVlanSuppress(val) => {
                    if last_off == offset {
                        stack.push(("NeighVlanSuppress", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::BackupNhid(val) => {
                    if last_off == offset {
                        stack.push(("BackupNhid", last_off));
                        break;
                    }
                }
                LinkinfoBrportAttrs::NeighForwardGrat(val) => {
                    if last_off == offset {
                        stack.push(("NeighForwardGrat", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LinkinfoBrportAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LinkinfoGreAttrs {
    Link(u32),
    Iflags(u16),
    Oflags(u16),
    Ikey(u32),
    Okey(u32),
    Local(std::net::IpAddr),
    Remote(std::net::IpAddr),
    Ttl(u8),
    Tos(u8),
    Pmtudisc(u8),
    EncapLimit(u8),
    Flowinfo(u32),
    Flags(u32),
    EncapType(u16),
    EncapFlags(u16),
    EncapSport(u16),
    EncapDport(u16),
    CollectMetadata(()),
    IgnoreDf(u8),
    Fwmark(u32),
    ErspanIndex(u32),
    ErspanVer(u8),
    ErspanDir(u8),
    ErspanHwid(u16),
}
impl<'a> IterableLinkinfoGreAttrs<'a> {
    pub fn get_link(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGreAttrs::Link(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGreAttrs",
            "Link",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_iflags(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGreAttrs::Iflags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGreAttrs",
            "Iflags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_oflags(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGreAttrs::Oflags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGreAttrs",
            "Oflags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ikey(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGreAttrs::Ikey(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGreAttrs",
            "Ikey",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_okey(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGreAttrs::Okey(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGreAttrs",
            "Okey",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_local(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGreAttrs::Local(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGreAttrs",
            "Local",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_remote(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGreAttrs::Remote(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGreAttrs",
            "Remote",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ttl(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGreAttrs::Ttl(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGreAttrs",
            "Ttl",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tos(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGreAttrs::Tos(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGreAttrs",
            "Tos",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pmtudisc(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGreAttrs::Pmtudisc(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGreAttrs",
            "Pmtudisc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap_limit(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGreAttrs::EncapLimit(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGreAttrs",
            "EncapLimit",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flowinfo(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGreAttrs::Flowinfo(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGreAttrs",
            "Flowinfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGreAttrs::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGreAttrs",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap_type(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGreAttrs::EncapType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGreAttrs",
            "EncapType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap_flags(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGreAttrs::EncapFlags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGreAttrs",
            "EncapFlags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap_sport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGreAttrs::EncapSport(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGreAttrs",
            "EncapSport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap_dport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGreAttrs::EncapDport(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGreAttrs",
            "EncapDport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_collect_metadata(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGreAttrs::CollectMetadata(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGreAttrs",
            "CollectMetadata",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ignore_df(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGreAttrs::IgnoreDf(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGreAttrs",
            "IgnoreDf",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fwmark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGreAttrs::Fwmark(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGreAttrs",
            "Fwmark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_erspan_index(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGreAttrs::ErspanIndex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGreAttrs",
            "ErspanIndex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_erspan_ver(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGreAttrs::ErspanVer(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGreAttrs",
            "ErspanVer",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_erspan_dir(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGreAttrs::ErspanDir(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGreAttrs",
            "ErspanDir",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_erspan_hwid(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGreAttrs::ErspanHwid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGreAttrs",
            "ErspanHwid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LinkinfoGreAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLinkinfoGreAttrs<'a> {
        IterableLinkinfoGreAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Link",
            2u16 => "Iflags",
            3u16 => "Oflags",
            4u16 => "Ikey",
            5u16 => "Okey",
            6u16 => "Local",
            7u16 => "Remote",
            8u16 => "Ttl",
            9u16 => "Tos",
            10u16 => "Pmtudisc",
            11u16 => "EncapLimit",
            12u16 => "Flowinfo",
            13u16 => "Flags",
            14u16 => "EncapType",
            15u16 => "EncapFlags",
            16u16 => "EncapSport",
            17u16 => "EncapDport",
            18u16 => "CollectMetadata",
            19u16 => "IgnoreDf",
            20u16 => "Fwmark",
            21u16 => "ErspanIndex",
            22u16 => "ErspanVer",
            23u16 => "ErspanDir",
            24u16 => "ErspanHwid",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLinkinfoGreAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLinkinfoGreAttrs<'a> {
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
impl<'a> Iterator for IterableLinkinfoGreAttrs<'a> {
    type Item = Result<LinkinfoGreAttrs, ErrorContext>;
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
                1u16 => LinkinfoGreAttrs::Link({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => LinkinfoGreAttrs::Iflags({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => LinkinfoGreAttrs::Oflags({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => LinkinfoGreAttrs::Ikey({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => LinkinfoGreAttrs::Okey({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => LinkinfoGreAttrs::Local({
                    let res = parse_ip(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => LinkinfoGreAttrs::Remote({
                    let res = parse_ip(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => LinkinfoGreAttrs::Ttl({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => LinkinfoGreAttrs::Tos({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => LinkinfoGreAttrs::Pmtudisc({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => LinkinfoGreAttrs::EncapLimit({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => LinkinfoGreAttrs::Flowinfo({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => LinkinfoGreAttrs::Flags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => LinkinfoGreAttrs::EncapType({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => LinkinfoGreAttrs::EncapFlags({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => LinkinfoGreAttrs::EncapSport({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => LinkinfoGreAttrs::EncapDport({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => LinkinfoGreAttrs::CollectMetadata(()),
                19u16 => LinkinfoGreAttrs::IgnoreDf({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => LinkinfoGreAttrs::Fwmark({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => LinkinfoGreAttrs::ErspanIndex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => LinkinfoGreAttrs::ErspanVer({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => LinkinfoGreAttrs::ErspanDir({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => LinkinfoGreAttrs::ErspanHwid({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "LinkinfoGreAttrs",
            r#type.and_then(|t| LinkinfoGreAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableLinkinfoGreAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LinkinfoGreAttrs");
        let mut iter = IterableLinkinfoGreAttrs::with_loc(&[], self.orig_loc);
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
                LinkinfoGreAttrs::Link(val) => fmt.field("Link", &val),
                LinkinfoGreAttrs::Iflags(val) => fmt.field("Iflags", &val),
                LinkinfoGreAttrs::Oflags(val) => fmt.field("Oflags", &val),
                LinkinfoGreAttrs::Ikey(val) => fmt.field("Ikey", &val),
                LinkinfoGreAttrs::Okey(val) => fmt.field("Okey", &val),
                LinkinfoGreAttrs::Local(val) => fmt.field("Local", &val),
                LinkinfoGreAttrs::Remote(val) => fmt.field("Remote", &val),
                LinkinfoGreAttrs::Ttl(val) => fmt.field("Ttl", &val),
                LinkinfoGreAttrs::Tos(val) => fmt.field("Tos", &val),
                LinkinfoGreAttrs::Pmtudisc(val) => fmt.field("Pmtudisc", &val),
                LinkinfoGreAttrs::EncapLimit(val) => fmt.field("EncapLimit", &val),
                LinkinfoGreAttrs::Flowinfo(val) => fmt.field("Flowinfo", &val),
                LinkinfoGreAttrs::Flags(val) => fmt.field("Flags", &val),
                LinkinfoGreAttrs::EncapType(val) => fmt.field("EncapType", &val),
                LinkinfoGreAttrs::EncapFlags(val) => fmt.field("EncapFlags", &val),
                LinkinfoGreAttrs::EncapSport(val) => fmt.field("EncapSport", &val),
                LinkinfoGreAttrs::EncapDport(val) => fmt.field("EncapDport", &val),
                LinkinfoGreAttrs::CollectMetadata(val) => fmt.field("CollectMetadata", &val),
                LinkinfoGreAttrs::IgnoreDf(val) => fmt.field("IgnoreDf", &val),
                LinkinfoGreAttrs::Fwmark(val) => fmt.field("Fwmark", &val),
                LinkinfoGreAttrs::ErspanIndex(val) => fmt.field("ErspanIndex", &val),
                LinkinfoGreAttrs::ErspanVer(val) => fmt.field("ErspanVer", &val),
                LinkinfoGreAttrs::ErspanDir(val) => fmt.field("ErspanDir", &val),
                LinkinfoGreAttrs::ErspanHwid(val) => fmt.field("ErspanHwid", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableLinkinfoGreAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LinkinfoGreAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| LinkinfoGreAttrs::attr_from_type(t)),
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
                LinkinfoGreAttrs::Link(val) => {
                    if last_off == offset {
                        stack.push(("Link", last_off));
                        break;
                    }
                }
                LinkinfoGreAttrs::Iflags(val) => {
                    if last_off == offset {
                        stack.push(("Iflags", last_off));
                        break;
                    }
                }
                LinkinfoGreAttrs::Oflags(val) => {
                    if last_off == offset {
                        stack.push(("Oflags", last_off));
                        break;
                    }
                }
                LinkinfoGreAttrs::Ikey(val) => {
                    if last_off == offset {
                        stack.push(("Ikey", last_off));
                        break;
                    }
                }
                LinkinfoGreAttrs::Okey(val) => {
                    if last_off == offset {
                        stack.push(("Okey", last_off));
                        break;
                    }
                }
                LinkinfoGreAttrs::Local(val) => {
                    if last_off == offset {
                        stack.push(("Local", last_off));
                        break;
                    }
                }
                LinkinfoGreAttrs::Remote(val) => {
                    if last_off == offset {
                        stack.push(("Remote", last_off));
                        break;
                    }
                }
                LinkinfoGreAttrs::Ttl(val) => {
                    if last_off == offset {
                        stack.push(("Ttl", last_off));
                        break;
                    }
                }
                LinkinfoGreAttrs::Tos(val) => {
                    if last_off == offset {
                        stack.push(("Tos", last_off));
                        break;
                    }
                }
                LinkinfoGreAttrs::Pmtudisc(val) => {
                    if last_off == offset {
                        stack.push(("Pmtudisc", last_off));
                        break;
                    }
                }
                LinkinfoGreAttrs::EncapLimit(val) => {
                    if last_off == offset {
                        stack.push(("EncapLimit", last_off));
                        break;
                    }
                }
                LinkinfoGreAttrs::Flowinfo(val) => {
                    if last_off == offset {
                        stack.push(("Flowinfo", last_off));
                        break;
                    }
                }
                LinkinfoGreAttrs::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                LinkinfoGreAttrs::EncapType(val) => {
                    if last_off == offset {
                        stack.push(("EncapType", last_off));
                        break;
                    }
                }
                LinkinfoGreAttrs::EncapFlags(val) => {
                    if last_off == offset {
                        stack.push(("EncapFlags", last_off));
                        break;
                    }
                }
                LinkinfoGreAttrs::EncapSport(val) => {
                    if last_off == offset {
                        stack.push(("EncapSport", last_off));
                        break;
                    }
                }
                LinkinfoGreAttrs::EncapDport(val) => {
                    if last_off == offset {
                        stack.push(("EncapDport", last_off));
                        break;
                    }
                }
                LinkinfoGreAttrs::CollectMetadata(val) => {
                    if last_off == offset {
                        stack.push(("CollectMetadata", last_off));
                        break;
                    }
                }
                LinkinfoGreAttrs::IgnoreDf(val) => {
                    if last_off == offset {
                        stack.push(("IgnoreDf", last_off));
                        break;
                    }
                }
                LinkinfoGreAttrs::Fwmark(val) => {
                    if last_off == offset {
                        stack.push(("Fwmark", last_off));
                        break;
                    }
                }
                LinkinfoGreAttrs::ErspanIndex(val) => {
                    if last_off == offset {
                        stack.push(("ErspanIndex", last_off));
                        break;
                    }
                }
                LinkinfoGreAttrs::ErspanVer(val) => {
                    if last_off == offset {
                        stack.push(("ErspanVer", last_off));
                        break;
                    }
                }
                LinkinfoGreAttrs::ErspanDir(val) => {
                    if last_off == offset {
                        stack.push(("ErspanDir", last_off));
                        break;
                    }
                }
                LinkinfoGreAttrs::ErspanHwid(val) => {
                    if last_off == offset {
                        stack.push(("ErspanHwid", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LinkinfoGreAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LinkinfoGre6Attrs<'a> {
    Link(u32),
    Iflags(u16),
    Oflags(u16),
    Ikey(u32),
    Okey(u32),
    Local(&'a [u8]),
    Remote(&'a [u8]),
    Ttl(u8),
    EncapLimit(u8),
    Flowinfo(u32),
    Flags(u32),
    EncapType(u16),
    EncapFlags(u16),
    EncapSport(u16),
    EncapDport(u16),
    CollectMetadata(()),
    Fwmark(u32),
    ErspanIndex(u32),
    ErspanVer(u8),
    ErspanDir(u8),
    ErspanHwid(u16),
}
impl<'a> IterableLinkinfoGre6Attrs<'a> {
    pub fn get_link(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGre6Attrs::Link(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGre6Attrs",
            "Link",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_iflags(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGre6Attrs::Iflags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGre6Attrs",
            "Iflags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_oflags(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGre6Attrs::Oflags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGre6Attrs",
            "Oflags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ikey(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGre6Attrs::Ikey(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGre6Attrs",
            "Ikey",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_okey(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGre6Attrs::Okey(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGre6Attrs",
            "Okey",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_local(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGre6Attrs::Local(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGre6Attrs",
            "Local",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_remote(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGre6Attrs::Remote(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGre6Attrs",
            "Remote",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ttl(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGre6Attrs::Ttl(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGre6Attrs",
            "Ttl",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap_limit(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGre6Attrs::EncapLimit(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGre6Attrs",
            "EncapLimit",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flowinfo(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGre6Attrs::Flowinfo(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGre6Attrs",
            "Flowinfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGre6Attrs::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGre6Attrs",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap_type(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGre6Attrs::EncapType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGre6Attrs",
            "EncapType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap_flags(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGre6Attrs::EncapFlags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGre6Attrs",
            "EncapFlags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap_sport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGre6Attrs::EncapSport(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGre6Attrs",
            "EncapSport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap_dport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGre6Attrs::EncapDport(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGre6Attrs",
            "EncapDport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_collect_metadata(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGre6Attrs::CollectMetadata(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGre6Attrs",
            "CollectMetadata",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fwmark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGre6Attrs::Fwmark(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGre6Attrs",
            "Fwmark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_erspan_index(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGre6Attrs::ErspanIndex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGre6Attrs",
            "ErspanIndex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_erspan_ver(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGre6Attrs::ErspanVer(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGre6Attrs",
            "ErspanVer",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_erspan_dir(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGre6Attrs::ErspanDir(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGre6Attrs",
            "ErspanDir",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_erspan_hwid(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGre6Attrs::ErspanHwid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGre6Attrs",
            "ErspanHwid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LinkinfoGre6Attrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLinkinfoGre6Attrs<'a> {
        IterableLinkinfoGre6Attrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        LinkinfoGreAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLinkinfoGre6Attrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLinkinfoGre6Attrs<'a> {
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
impl<'a> Iterator for IterableLinkinfoGre6Attrs<'a> {
    type Item = Result<LinkinfoGre6Attrs<'a>, ErrorContext>;
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
                1u16 => LinkinfoGre6Attrs::Link({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => LinkinfoGre6Attrs::Iflags({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => LinkinfoGre6Attrs::Oflags({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => LinkinfoGre6Attrs::Ikey({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => LinkinfoGre6Attrs::Okey({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => LinkinfoGre6Attrs::Local({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => LinkinfoGre6Attrs::Remote({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => LinkinfoGre6Attrs::Ttl({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => LinkinfoGre6Attrs::EncapLimit({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => LinkinfoGre6Attrs::Flowinfo({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => LinkinfoGre6Attrs::Flags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => LinkinfoGre6Attrs::EncapType({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => LinkinfoGre6Attrs::EncapFlags({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => LinkinfoGre6Attrs::EncapSport({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => LinkinfoGre6Attrs::EncapDport({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => LinkinfoGre6Attrs::CollectMetadata(()),
                20u16 => LinkinfoGre6Attrs::Fwmark({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => LinkinfoGre6Attrs::ErspanIndex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => LinkinfoGre6Attrs::ErspanVer({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => LinkinfoGre6Attrs::ErspanDir({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => LinkinfoGre6Attrs::ErspanHwid({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "LinkinfoGre6Attrs",
            r#type.and_then(|t| LinkinfoGre6Attrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableLinkinfoGre6Attrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LinkinfoGre6Attrs");
        let mut iter = IterableLinkinfoGre6Attrs::with_loc(&[], self.orig_loc);
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
                LinkinfoGre6Attrs::Link(val) => fmt.field("Link", &val),
                LinkinfoGre6Attrs::Iflags(val) => fmt.field("Iflags", &val),
                LinkinfoGre6Attrs::Oflags(val) => fmt.field("Oflags", &val),
                LinkinfoGre6Attrs::Ikey(val) => fmt.field("Ikey", &val),
                LinkinfoGre6Attrs::Okey(val) => fmt.field("Okey", &val),
                LinkinfoGre6Attrs::Local(val) => fmt.field("Local", &val),
                LinkinfoGre6Attrs::Remote(val) => fmt.field("Remote", &val),
                LinkinfoGre6Attrs::Ttl(val) => fmt.field("Ttl", &val),
                LinkinfoGre6Attrs::EncapLimit(val) => fmt.field("EncapLimit", &val),
                LinkinfoGre6Attrs::Flowinfo(val) => fmt.field("Flowinfo", &val),
                LinkinfoGre6Attrs::Flags(val) => fmt.field("Flags", &val),
                LinkinfoGre6Attrs::EncapType(val) => fmt.field("EncapType", &val),
                LinkinfoGre6Attrs::EncapFlags(val) => fmt.field("EncapFlags", &val),
                LinkinfoGre6Attrs::EncapSport(val) => fmt.field("EncapSport", &val),
                LinkinfoGre6Attrs::EncapDport(val) => fmt.field("EncapDport", &val),
                LinkinfoGre6Attrs::CollectMetadata(val) => fmt.field("CollectMetadata", &val),
                LinkinfoGre6Attrs::Fwmark(val) => fmt.field("Fwmark", &val),
                LinkinfoGre6Attrs::ErspanIndex(val) => fmt.field("ErspanIndex", &val),
                LinkinfoGre6Attrs::ErspanVer(val) => fmt.field("ErspanVer", &val),
                LinkinfoGre6Attrs::ErspanDir(val) => fmt.field("ErspanDir", &val),
                LinkinfoGre6Attrs::ErspanHwid(val) => fmt.field("ErspanHwid", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableLinkinfoGre6Attrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LinkinfoGre6Attrs", offset));
            return (
                stack,
                missing_type.and_then(|t| LinkinfoGre6Attrs::attr_from_type(t)),
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
                LinkinfoGre6Attrs::Link(val) => {
                    if last_off == offset {
                        stack.push(("Link", last_off));
                        break;
                    }
                }
                LinkinfoGre6Attrs::Iflags(val) => {
                    if last_off == offset {
                        stack.push(("Iflags", last_off));
                        break;
                    }
                }
                LinkinfoGre6Attrs::Oflags(val) => {
                    if last_off == offset {
                        stack.push(("Oflags", last_off));
                        break;
                    }
                }
                LinkinfoGre6Attrs::Ikey(val) => {
                    if last_off == offset {
                        stack.push(("Ikey", last_off));
                        break;
                    }
                }
                LinkinfoGre6Attrs::Okey(val) => {
                    if last_off == offset {
                        stack.push(("Okey", last_off));
                        break;
                    }
                }
                LinkinfoGre6Attrs::Local(val) => {
                    if last_off == offset {
                        stack.push(("Local", last_off));
                        break;
                    }
                }
                LinkinfoGre6Attrs::Remote(val) => {
                    if last_off == offset {
                        stack.push(("Remote", last_off));
                        break;
                    }
                }
                LinkinfoGre6Attrs::Ttl(val) => {
                    if last_off == offset {
                        stack.push(("Ttl", last_off));
                        break;
                    }
                }
                LinkinfoGre6Attrs::EncapLimit(val) => {
                    if last_off == offset {
                        stack.push(("EncapLimit", last_off));
                        break;
                    }
                }
                LinkinfoGre6Attrs::Flowinfo(val) => {
                    if last_off == offset {
                        stack.push(("Flowinfo", last_off));
                        break;
                    }
                }
                LinkinfoGre6Attrs::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                LinkinfoGre6Attrs::EncapType(val) => {
                    if last_off == offset {
                        stack.push(("EncapType", last_off));
                        break;
                    }
                }
                LinkinfoGre6Attrs::EncapFlags(val) => {
                    if last_off == offset {
                        stack.push(("EncapFlags", last_off));
                        break;
                    }
                }
                LinkinfoGre6Attrs::EncapSport(val) => {
                    if last_off == offset {
                        stack.push(("EncapSport", last_off));
                        break;
                    }
                }
                LinkinfoGre6Attrs::EncapDport(val) => {
                    if last_off == offset {
                        stack.push(("EncapDport", last_off));
                        break;
                    }
                }
                LinkinfoGre6Attrs::CollectMetadata(val) => {
                    if last_off == offset {
                        stack.push(("CollectMetadata", last_off));
                        break;
                    }
                }
                LinkinfoGre6Attrs::Fwmark(val) => {
                    if last_off == offset {
                        stack.push(("Fwmark", last_off));
                        break;
                    }
                }
                LinkinfoGre6Attrs::ErspanIndex(val) => {
                    if last_off == offset {
                        stack.push(("ErspanIndex", last_off));
                        break;
                    }
                }
                LinkinfoGre6Attrs::ErspanVer(val) => {
                    if last_off == offset {
                        stack.push(("ErspanVer", last_off));
                        break;
                    }
                }
                LinkinfoGre6Attrs::ErspanDir(val) => {
                    if last_off == offset {
                        stack.push(("ErspanDir", last_off));
                        break;
                    }
                }
                LinkinfoGre6Attrs::ErspanHwid(val) => {
                    if last_off == offset {
                        stack.push(("ErspanHwid", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LinkinfoGre6Attrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LinkinfoVtiAttrs {
    Link(u32),
    Ikey(u32),
    Okey(u32),
    Local(std::net::IpAddr),
    Remote(std::net::IpAddr),
    Fwmark(u32),
}
impl<'a> IterableLinkinfoVtiAttrs<'a> {
    pub fn get_link(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoVtiAttrs::Link(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoVtiAttrs",
            "Link",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ikey(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoVtiAttrs::Ikey(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoVtiAttrs",
            "Ikey",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_okey(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoVtiAttrs::Okey(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoVtiAttrs",
            "Okey",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_local(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoVtiAttrs::Local(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoVtiAttrs",
            "Local",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_remote(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoVtiAttrs::Remote(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoVtiAttrs",
            "Remote",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fwmark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoVtiAttrs::Fwmark(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoVtiAttrs",
            "Fwmark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LinkinfoVtiAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLinkinfoVtiAttrs<'a> {
        IterableLinkinfoVtiAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Link",
            2u16 => "Ikey",
            3u16 => "Okey",
            4u16 => "Local",
            5u16 => "Remote",
            6u16 => "Fwmark",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLinkinfoVtiAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLinkinfoVtiAttrs<'a> {
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
impl<'a> Iterator for IterableLinkinfoVtiAttrs<'a> {
    type Item = Result<LinkinfoVtiAttrs, ErrorContext>;
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
                1u16 => LinkinfoVtiAttrs::Link({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => LinkinfoVtiAttrs::Ikey({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => LinkinfoVtiAttrs::Okey({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => LinkinfoVtiAttrs::Local({
                    let res = parse_ip(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => LinkinfoVtiAttrs::Remote({
                    let res = parse_ip(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => LinkinfoVtiAttrs::Fwmark({
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
            "LinkinfoVtiAttrs",
            r#type.and_then(|t| LinkinfoVtiAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableLinkinfoVtiAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LinkinfoVtiAttrs");
        let mut iter = IterableLinkinfoVtiAttrs::with_loc(&[], self.orig_loc);
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
                LinkinfoVtiAttrs::Link(val) => fmt.field("Link", &val),
                LinkinfoVtiAttrs::Ikey(val) => fmt.field("Ikey", &val),
                LinkinfoVtiAttrs::Okey(val) => fmt.field("Okey", &val),
                LinkinfoVtiAttrs::Local(val) => fmt.field("Local", &val),
                LinkinfoVtiAttrs::Remote(val) => fmt.field("Remote", &val),
                LinkinfoVtiAttrs::Fwmark(val) => fmt.field("Fwmark", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableLinkinfoVtiAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LinkinfoVtiAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| LinkinfoVtiAttrs::attr_from_type(t)),
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
                LinkinfoVtiAttrs::Link(val) => {
                    if last_off == offset {
                        stack.push(("Link", last_off));
                        break;
                    }
                }
                LinkinfoVtiAttrs::Ikey(val) => {
                    if last_off == offset {
                        stack.push(("Ikey", last_off));
                        break;
                    }
                }
                LinkinfoVtiAttrs::Okey(val) => {
                    if last_off == offset {
                        stack.push(("Okey", last_off));
                        break;
                    }
                }
                LinkinfoVtiAttrs::Local(val) => {
                    if last_off == offset {
                        stack.push(("Local", last_off));
                        break;
                    }
                }
                LinkinfoVtiAttrs::Remote(val) => {
                    if last_off == offset {
                        stack.push(("Remote", last_off));
                        break;
                    }
                }
                LinkinfoVtiAttrs::Fwmark(val) => {
                    if last_off == offset {
                        stack.push(("Fwmark", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LinkinfoVtiAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LinkinfoVti6Attrs<'a> {
    Link(u32),
    Ikey(u32),
    Okey(u32),
    Local(&'a [u8]),
    Remote(&'a [u8]),
    Fwmark(u32),
}
impl<'a> IterableLinkinfoVti6Attrs<'a> {
    pub fn get_link(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoVti6Attrs::Link(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoVti6Attrs",
            "Link",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ikey(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoVti6Attrs::Ikey(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoVti6Attrs",
            "Ikey",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_okey(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoVti6Attrs::Okey(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoVti6Attrs",
            "Okey",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_local(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoVti6Attrs::Local(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoVti6Attrs",
            "Local",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_remote(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoVti6Attrs::Remote(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoVti6Attrs",
            "Remote",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fwmark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoVti6Attrs::Fwmark(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoVti6Attrs",
            "Fwmark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LinkinfoVti6Attrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLinkinfoVti6Attrs<'a> {
        IterableLinkinfoVti6Attrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        LinkinfoVtiAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLinkinfoVti6Attrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLinkinfoVti6Attrs<'a> {
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
impl<'a> Iterator for IterableLinkinfoVti6Attrs<'a> {
    type Item = Result<LinkinfoVti6Attrs<'a>, ErrorContext>;
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
                1u16 => LinkinfoVti6Attrs::Link({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => LinkinfoVti6Attrs::Ikey({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => LinkinfoVti6Attrs::Okey({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => LinkinfoVti6Attrs::Local({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => LinkinfoVti6Attrs::Remote({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => LinkinfoVti6Attrs::Fwmark({
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
            "LinkinfoVti6Attrs",
            r#type.and_then(|t| LinkinfoVti6Attrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableLinkinfoVti6Attrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LinkinfoVti6Attrs");
        let mut iter = IterableLinkinfoVti6Attrs::with_loc(&[], self.orig_loc);
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
                LinkinfoVti6Attrs::Link(val) => fmt.field("Link", &val),
                LinkinfoVti6Attrs::Ikey(val) => fmt.field("Ikey", &val),
                LinkinfoVti6Attrs::Okey(val) => fmt.field("Okey", &val),
                LinkinfoVti6Attrs::Local(val) => fmt.field("Local", &val),
                LinkinfoVti6Attrs::Remote(val) => fmt.field("Remote", &val),
                LinkinfoVti6Attrs::Fwmark(val) => fmt.field("Fwmark", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableLinkinfoVti6Attrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LinkinfoVti6Attrs", offset));
            return (
                stack,
                missing_type.and_then(|t| LinkinfoVti6Attrs::attr_from_type(t)),
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
                LinkinfoVti6Attrs::Link(val) => {
                    if last_off == offset {
                        stack.push(("Link", last_off));
                        break;
                    }
                }
                LinkinfoVti6Attrs::Ikey(val) => {
                    if last_off == offset {
                        stack.push(("Ikey", last_off));
                        break;
                    }
                }
                LinkinfoVti6Attrs::Okey(val) => {
                    if last_off == offset {
                        stack.push(("Okey", last_off));
                        break;
                    }
                }
                LinkinfoVti6Attrs::Local(val) => {
                    if last_off == offset {
                        stack.push(("Local", last_off));
                        break;
                    }
                }
                LinkinfoVti6Attrs::Remote(val) => {
                    if last_off == offset {
                        stack.push(("Remote", last_off));
                        break;
                    }
                }
                LinkinfoVti6Attrs::Fwmark(val) => {
                    if last_off == offset {
                        stack.push(("Fwmark", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LinkinfoVti6Attrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LinkinfoGeneveAttrs<'a> {
    Id(u32),
    Remote(std::net::Ipv4Addr),
    Ttl(u8),
    Tos(u8),
    Port(u16),
    CollectMetadata(()),
    Remote6(&'a [u8]),
    UdpCsum(u8),
    UdpZeroCsum6Tx(u8),
    UdpZeroCsum6Rx(u8),
    Label(u32),
    TtlInherit(u8),
    Df(u8),
    InnerProtoInherit(()),
    PortRange(IflaGenevePortRange),
    GroHint(()),
    Local(std::net::Ipv4Addr),
    Local6(&'a [u8]),
}
impl<'a> IterableLinkinfoGeneveAttrs<'a> {
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGeneveAttrs::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGeneveAttrs",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_remote(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGeneveAttrs::Remote(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGeneveAttrs",
            "Remote",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ttl(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGeneveAttrs::Ttl(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGeneveAttrs",
            "Ttl",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tos(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGeneveAttrs::Tos(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGeneveAttrs",
            "Tos",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGeneveAttrs::Port(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGeneveAttrs",
            "Port",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_collect_metadata(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGeneveAttrs::CollectMetadata(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGeneveAttrs",
            "CollectMetadata",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_remote6(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGeneveAttrs::Remote6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGeneveAttrs",
            "Remote6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_udp_csum(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGeneveAttrs::UdpCsum(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGeneveAttrs",
            "UdpCsum",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_udp_zero_csum6_tx(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGeneveAttrs::UdpZeroCsum6Tx(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGeneveAttrs",
            "UdpZeroCsum6Tx",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_udp_zero_csum6_rx(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGeneveAttrs::UdpZeroCsum6Rx(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGeneveAttrs",
            "UdpZeroCsum6Rx",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_label(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGeneveAttrs::Label(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGeneveAttrs",
            "Label",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ttl_inherit(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGeneveAttrs::TtlInherit(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGeneveAttrs",
            "TtlInherit",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_df(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGeneveAttrs::Df(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGeneveAttrs",
            "Df",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_inner_proto_inherit(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGeneveAttrs::InnerProtoInherit(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGeneveAttrs",
            "InnerProtoInherit",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_port_range(&self) -> Result<IflaGenevePortRange, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGeneveAttrs::PortRange(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGeneveAttrs",
            "PortRange",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_gro_hint(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGeneveAttrs::GroHint(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGeneveAttrs",
            "GroHint",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_local(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGeneveAttrs::Local(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGeneveAttrs",
            "Local",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_local6(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoGeneveAttrs::Local6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoGeneveAttrs",
            "Local6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LinkinfoGeneveAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLinkinfoGeneveAttrs<'a> {
        IterableLinkinfoGeneveAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Id",
            2u16 => "Remote",
            3u16 => "Ttl",
            4u16 => "Tos",
            5u16 => "Port",
            6u16 => "CollectMetadata",
            7u16 => "Remote6",
            8u16 => "UdpCsum",
            9u16 => "UdpZeroCsum6Tx",
            10u16 => "UdpZeroCsum6Rx",
            11u16 => "Label",
            12u16 => "TtlInherit",
            13u16 => "Df",
            14u16 => "InnerProtoInherit",
            15u16 => "PortRange",
            16u16 => "GroHint",
            17u16 => "Local",
            18u16 => "Local6",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLinkinfoGeneveAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLinkinfoGeneveAttrs<'a> {
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
impl<'a> Iterator for IterableLinkinfoGeneveAttrs<'a> {
    type Item = Result<LinkinfoGeneveAttrs<'a>, ErrorContext>;
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
                1u16 => LinkinfoGeneveAttrs::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => LinkinfoGeneveAttrs::Remote({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => LinkinfoGeneveAttrs::Ttl({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => LinkinfoGeneveAttrs::Tos({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => LinkinfoGeneveAttrs::Port({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => LinkinfoGeneveAttrs::CollectMetadata(()),
                7u16 => LinkinfoGeneveAttrs::Remote6({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => LinkinfoGeneveAttrs::UdpCsum({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => LinkinfoGeneveAttrs::UdpZeroCsum6Tx({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => LinkinfoGeneveAttrs::UdpZeroCsum6Rx({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => LinkinfoGeneveAttrs::Label({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => LinkinfoGeneveAttrs::TtlInherit({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => LinkinfoGeneveAttrs::Df({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => LinkinfoGeneveAttrs::InnerProtoInherit(()),
                15u16 => LinkinfoGeneveAttrs::PortRange({
                    let res = Some(IflaGenevePortRange::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => LinkinfoGeneveAttrs::GroHint(()),
                17u16 => LinkinfoGeneveAttrs::Local({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => LinkinfoGeneveAttrs::Local6({
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
            "LinkinfoGeneveAttrs",
            r#type.and_then(|t| LinkinfoGeneveAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableLinkinfoGeneveAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LinkinfoGeneveAttrs");
        let mut iter = IterableLinkinfoGeneveAttrs::with_loc(&[], self.orig_loc);
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
                LinkinfoGeneveAttrs::Id(val) => fmt.field("Id", &val),
                LinkinfoGeneveAttrs::Remote(val) => fmt.field("Remote", &val),
                LinkinfoGeneveAttrs::Ttl(val) => fmt.field("Ttl", &val),
                LinkinfoGeneveAttrs::Tos(val) => fmt.field("Tos", &val),
                LinkinfoGeneveAttrs::Port(val) => fmt.field("Port", &val),
                LinkinfoGeneveAttrs::CollectMetadata(val) => fmt.field("CollectMetadata", &val),
                LinkinfoGeneveAttrs::Remote6(val) => fmt.field("Remote6", &val),
                LinkinfoGeneveAttrs::UdpCsum(val) => fmt.field("UdpCsum", &val),
                LinkinfoGeneveAttrs::UdpZeroCsum6Tx(val) => fmt.field("UdpZeroCsum6Tx", &val),
                LinkinfoGeneveAttrs::UdpZeroCsum6Rx(val) => fmt.field("UdpZeroCsum6Rx", &val),
                LinkinfoGeneveAttrs::Label(val) => fmt.field("Label", &val),
                LinkinfoGeneveAttrs::TtlInherit(val) => fmt.field("TtlInherit", &val),
                LinkinfoGeneveAttrs::Df(val) => fmt.field("Df", &val),
                LinkinfoGeneveAttrs::InnerProtoInherit(val) => fmt.field("InnerProtoInherit", &val),
                LinkinfoGeneveAttrs::PortRange(val) => fmt.field("PortRange", &val),
                LinkinfoGeneveAttrs::GroHint(val) => fmt.field("GroHint", &val),
                LinkinfoGeneveAttrs::Local(val) => fmt.field("Local", &val),
                LinkinfoGeneveAttrs::Local6(val) => fmt.field("Local6", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableLinkinfoGeneveAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LinkinfoGeneveAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| LinkinfoGeneveAttrs::attr_from_type(t)),
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
                LinkinfoGeneveAttrs::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                LinkinfoGeneveAttrs::Remote(val) => {
                    if last_off == offset {
                        stack.push(("Remote", last_off));
                        break;
                    }
                }
                LinkinfoGeneveAttrs::Ttl(val) => {
                    if last_off == offset {
                        stack.push(("Ttl", last_off));
                        break;
                    }
                }
                LinkinfoGeneveAttrs::Tos(val) => {
                    if last_off == offset {
                        stack.push(("Tos", last_off));
                        break;
                    }
                }
                LinkinfoGeneveAttrs::Port(val) => {
                    if last_off == offset {
                        stack.push(("Port", last_off));
                        break;
                    }
                }
                LinkinfoGeneveAttrs::CollectMetadata(val) => {
                    if last_off == offset {
                        stack.push(("CollectMetadata", last_off));
                        break;
                    }
                }
                LinkinfoGeneveAttrs::Remote6(val) => {
                    if last_off == offset {
                        stack.push(("Remote6", last_off));
                        break;
                    }
                }
                LinkinfoGeneveAttrs::UdpCsum(val) => {
                    if last_off == offset {
                        stack.push(("UdpCsum", last_off));
                        break;
                    }
                }
                LinkinfoGeneveAttrs::UdpZeroCsum6Tx(val) => {
                    if last_off == offset {
                        stack.push(("UdpZeroCsum6Tx", last_off));
                        break;
                    }
                }
                LinkinfoGeneveAttrs::UdpZeroCsum6Rx(val) => {
                    if last_off == offset {
                        stack.push(("UdpZeroCsum6Rx", last_off));
                        break;
                    }
                }
                LinkinfoGeneveAttrs::Label(val) => {
                    if last_off == offset {
                        stack.push(("Label", last_off));
                        break;
                    }
                }
                LinkinfoGeneveAttrs::TtlInherit(val) => {
                    if last_off == offset {
                        stack.push(("TtlInherit", last_off));
                        break;
                    }
                }
                LinkinfoGeneveAttrs::Df(val) => {
                    if last_off == offset {
                        stack.push(("Df", last_off));
                        break;
                    }
                }
                LinkinfoGeneveAttrs::InnerProtoInherit(val) => {
                    if last_off == offset {
                        stack.push(("InnerProtoInherit", last_off));
                        break;
                    }
                }
                LinkinfoGeneveAttrs::PortRange(val) => {
                    if last_off == offset {
                        stack.push(("PortRange", last_off));
                        break;
                    }
                }
                LinkinfoGeneveAttrs::GroHint(val) => {
                    if last_off == offset {
                        stack.push(("GroHint", last_off));
                        break;
                    }
                }
                LinkinfoGeneveAttrs::Local(val) => {
                    if last_off == offset {
                        stack.push(("Local", last_off));
                        break;
                    }
                }
                LinkinfoGeneveAttrs::Local6(val) => {
                    if last_off == offset {
                        stack.push(("Local6", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LinkinfoGeneveAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LinkinfoHsrAttrs<'a> {
    Slave1(u32),
    Slave2(u32),
    MulticastSpec(u8),
    SupervisionAddr(&'a [u8]),
    SeqNr(u16),
    Version(u8),
    Protocol(u8),
    Interlink(u32),
}
impl<'a> IterableLinkinfoHsrAttrs<'a> {
    pub fn get_slave1(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoHsrAttrs::Slave1(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoHsrAttrs",
            "Slave1",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_slave2(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoHsrAttrs::Slave2(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoHsrAttrs",
            "Slave2",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_multicast_spec(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoHsrAttrs::MulticastSpec(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoHsrAttrs",
            "MulticastSpec",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_supervision_addr(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoHsrAttrs::SupervisionAddr(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoHsrAttrs",
            "SupervisionAddr",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_seq_nr(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoHsrAttrs::SeqNr(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoHsrAttrs",
            "SeqNr",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_version(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoHsrAttrs::Version(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoHsrAttrs",
            "Version",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_protocol(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoHsrAttrs::Protocol(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoHsrAttrs",
            "Protocol",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_interlink(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoHsrAttrs::Interlink(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoHsrAttrs",
            "Interlink",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LinkinfoHsrAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLinkinfoHsrAttrs<'a> {
        IterableLinkinfoHsrAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Slave1",
            2u16 => "Slave2",
            3u16 => "MulticastSpec",
            4u16 => "SupervisionAddr",
            5u16 => "SeqNr",
            6u16 => "Version",
            7u16 => "Protocol",
            8u16 => "Interlink",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLinkinfoHsrAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLinkinfoHsrAttrs<'a> {
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
impl<'a> Iterator for IterableLinkinfoHsrAttrs<'a> {
    type Item = Result<LinkinfoHsrAttrs<'a>, ErrorContext>;
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
                1u16 => LinkinfoHsrAttrs::Slave1({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => LinkinfoHsrAttrs::Slave2({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => LinkinfoHsrAttrs::MulticastSpec({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => LinkinfoHsrAttrs::SupervisionAddr({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => LinkinfoHsrAttrs::SeqNr({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => LinkinfoHsrAttrs::Version({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => LinkinfoHsrAttrs::Protocol({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => LinkinfoHsrAttrs::Interlink({
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
            "LinkinfoHsrAttrs",
            r#type.and_then(|t| LinkinfoHsrAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableLinkinfoHsrAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LinkinfoHsrAttrs");
        let mut iter = IterableLinkinfoHsrAttrs::with_loc(&[], self.orig_loc);
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
                LinkinfoHsrAttrs::Slave1(val) => fmt.field("Slave1", &val),
                LinkinfoHsrAttrs::Slave2(val) => fmt.field("Slave2", &val),
                LinkinfoHsrAttrs::MulticastSpec(val) => fmt.field("MulticastSpec", &val),
                LinkinfoHsrAttrs::SupervisionAddr(val) => {
                    fmt.field("SupervisionAddr", &FormatMac(val))
                }
                LinkinfoHsrAttrs::SeqNr(val) => fmt.field("SeqNr", &val),
                LinkinfoHsrAttrs::Version(val) => fmt.field("Version", &val),
                LinkinfoHsrAttrs::Protocol(val) => fmt.field("Protocol", &val),
                LinkinfoHsrAttrs::Interlink(val) => fmt.field("Interlink", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableLinkinfoHsrAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LinkinfoHsrAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| LinkinfoHsrAttrs::attr_from_type(t)),
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
                LinkinfoHsrAttrs::Slave1(val) => {
                    if last_off == offset {
                        stack.push(("Slave1", last_off));
                        break;
                    }
                }
                LinkinfoHsrAttrs::Slave2(val) => {
                    if last_off == offset {
                        stack.push(("Slave2", last_off));
                        break;
                    }
                }
                LinkinfoHsrAttrs::MulticastSpec(val) => {
                    if last_off == offset {
                        stack.push(("MulticastSpec", last_off));
                        break;
                    }
                }
                LinkinfoHsrAttrs::SupervisionAddr(val) => {
                    if last_off == offset {
                        stack.push(("SupervisionAddr", last_off));
                        break;
                    }
                }
                LinkinfoHsrAttrs::SeqNr(val) => {
                    if last_off == offset {
                        stack.push(("SeqNr", last_off));
                        break;
                    }
                }
                LinkinfoHsrAttrs::Version(val) => {
                    if last_off == offset {
                        stack.push(("Version", last_off));
                        break;
                    }
                }
                LinkinfoHsrAttrs::Protocol(val) => {
                    if last_off == offset {
                        stack.push(("Protocol", last_off));
                        break;
                    }
                }
                LinkinfoHsrAttrs::Interlink(val) => {
                    if last_off == offset {
                        stack.push(("Interlink", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LinkinfoHsrAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LinkinfoIptunAttrs<'a> {
    Link(u32),
    Local(std::net::IpAddr),
    Remote(std::net::IpAddr),
    Ttl(u8),
    Tos(u8),
    EncapLimit(u8),
    Flowinfo(u32),
    Flags(u16),
    Proto(u8),
    Pmtudisc(u8),
    _6rdPrefix(&'a [u8]),
    _6rdRelayPrefix(std::net::Ipv4Addr),
    _6rdPrefixlen(u16),
    _6rdRelayPrefixlen(u16),
    EncapType(u16),
    EncapFlags(u16),
    EncapSport(u16),
    EncapDport(u16),
    CollectMetadata(()),
    Fwmark(u32),
}
impl<'a> IterableLinkinfoIptunAttrs<'a> {
    pub fn get_link(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIptunAttrs::Link(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIptunAttrs",
            "Link",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_local(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIptunAttrs::Local(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIptunAttrs",
            "Local",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_remote(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIptunAttrs::Remote(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIptunAttrs",
            "Remote",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ttl(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIptunAttrs::Ttl(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIptunAttrs",
            "Ttl",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tos(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIptunAttrs::Tos(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIptunAttrs",
            "Tos",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap_limit(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIptunAttrs::EncapLimit(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIptunAttrs",
            "EncapLimit",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flowinfo(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIptunAttrs::Flowinfo(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIptunAttrs",
            "Flowinfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flags(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIptunAttrs::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIptunAttrs",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_proto(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIptunAttrs::Proto(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIptunAttrs",
            "Proto",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pmtudisc(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIptunAttrs::Pmtudisc(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIptunAttrs",
            "Pmtudisc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_6rd_prefix(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIptunAttrs::_6rdPrefix(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIptunAttrs",
            "6rdPrefix",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_6rd_relay_prefix(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIptunAttrs::_6rdRelayPrefix(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIptunAttrs",
            "6rdRelayPrefix",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_6rd_prefixlen(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIptunAttrs::_6rdPrefixlen(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIptunAttrs",
            "6rdPrefixlen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_6rd_relay_prefixlen(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIptunAttrs::_6rdRelayPrefixlen(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIptunAttrs",
            "6rdRelayPrefixlen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap_type(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIptunAttrs::EncapType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIptunAttrs",
            "EncapType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap_flags(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIptunAttrs::EncapFlags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIptunAttrs",
            "EncapFlags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap_sport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIptunAttrs::EncapSport(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIptunAttrs",
            "EncapSport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap_dport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIptunAttrs::EncapDport(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIptunAttrs",
            "EncapDport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_collect_metadata(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIptunAttrs::CollectMetadata(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIptunAttrs",
            "CollectMetadata",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fwmark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIptunAttrs::Fwmark(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIptunAttrs",
            "Fwmark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LinkinfoIptunAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLinkinfoIptunAttrs<'a> {
        IterableLinkinfoIptunAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Link",
            2u16 => "Local",
            3u16 => "Remote",
            4u16 => "Ttl",
            5u16 => "Tos",
            6u16 => "EncapLimit",
            7u16 => "Flowinfo",
            8u16 => "Flags",
            9u16 => "Proto",
            10u16 => "Pmtudisc",
            11u16 => "6rdPrefix",
            12u16 => "6rdRelayPrefix",
            13u16 => "6rdPrefixlen",
            14u16 => "6rdRelayPrefixlen",
            15u16 => "EncapType",
            16u16 => "EncapFlags",
            17u16 => "EncapSport",
            18u16 => "EncapDport",
            19u16 => "CollectMetadata",
            20u16 => "Fwmark",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLinkinfoIptunAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLinkinfoIptunAttrs<'a> {
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
impl<'a> Iterator for IterableLinkinfoIptunAttrs<'a> {
    type Item = Result<LinkinfoIptunAttrs<'a>, ErrorContext>;
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
                1u16 => LinkinfoIptunAttrs::Link({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => LinkinfoIptunAttrs::Local({
                    let res = parse_ip(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => LinkinfoIptunAttrs::Remote({
                    let res = parse_ip(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => LinkinfoIptunAttrs::Ttl({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => LinkinfoIptunAttrs::Tos({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => LinkinfoIptunAttrs::EncapLimit({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => LinkinfoIptunAttrs::Flowinfo({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => LinkinfoIptunAttrs::Flags({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => LinkinfoIptunAttrs::Proto({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => LinkinfoIptunAttrs::Pmtudisc({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => LinkinfoIptunAttrs::_6rdPrefix({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => LinkinfoIptunAttrs::_6rdRelayPrefix({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => LinkinfoIptunAttrs::_6rdPrefixlen({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => LinkinfoIptunAttrs::_6rdRelayPrefixlen({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => LinkinfoIptunAttrs::EncapType({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => LinkinfoIptunAttrs::EncapFlags({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => LinkinfoIptunAttrs::EncapSport({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => LinkinfoIptunAttrs::EncapDport({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => LinkinfoIptunAttrs::CollectMetadata(()),
                20u16 => LinkinfoIptunAttrs::Fwmark({
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
            "LinkinfoIptunAttrs",
            r#type.and_then(|t| LinkinfoIptunAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableLinkinfoIptunAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LinkinfoIptunAttrs");
        let mut iter = IterableLinkinfoIptunAttrs::with_loc(&[], self.orig_loc);
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
                LinkinfoIptunAttrs::Link(val) => fmt.field("Link", &val),
                LinkinfoIptunAttrs::Local(val) => fmt.field("Local", &val),
                LinkinfoIptunAttrs::Remote(val) => fmt.field("Remote", &val),
                LinkinfoIptunAttrs::Ttl(val) => fmt.field("Ttl", &val),
                LinkinfoIptunAttrs::Tos(val) => fmt.field("Tos", &val),
                LinkinfoIptunAttrs::EncapLimit(val) => fmt.field("EncapLimit", &val),
                LinkinfoIptunAttrs::Flowinfo(val) => fmt.field("Flowinfo", &val),
                LinkinfoIptunAttrs::Flags(val) => fmt.field("Flags", &val),
                LinkinfoIptunAttrs::Proto(val) => fmt.field("Proto", &val),
                LinkinfoIptunAttrs::Pmtudisc(val) => fmt.field("Pmtudisc", &val),
                LinkinfoIptunAttrs::_6rdPrefix(val) => fmt.field("_6rdPrefix", &val),
                LinkinfoIptunAttrs::_6rdRelayPrefix(val) => fmt.field("_6rdRelayPrefix", &val),
                LinkinfoIptunAttrs::_6rdPrefixlen(val) => fmt.field("_6rdPrefixlen", &val),
                LinkinfoIptunAttrs::_6rdRelayPrefixlen(val) => {
                    fmt.field("_6rdRelayPrefixlen", &val)
                }
                LinkinfoIptunAttrs::EncapType(val) => fmt.field("EncapType", &val),
                LinkinfoIptunAttrs::EncapFlags(val) => fmt.field("EncapFlags", &val),
                LinkinfoIptunAttrs::EncapSport(val) => fmt.field("EncapSport", &val),
                LinkinfoIptunAttrs::EncapDport(val) => fmt.field("EncapDport", &val),
                LinkinfoIptunAttrs::CollectMetadata(val) => fmt.field("CollectMetadata", &val),
                LinkinfoIptunAttrs::Fwmark(val) => fmt.field("Fwmark", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableLinkinfoIptunAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LinkinfoIptunAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| LinkinfoIptunAttrs::attr_from_type(t)),
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
                LinkinfoIptunAttrs::Link(val) => {
                    if last_off == offset {
                        stack.push(("Link", last_off));
                        break;
                    }
                }
                LinkinfoIptunAttrs::Local(val) => {
                    if last_off == offset {
                        stack.push(("Local", last_off));
                        break;
                    }
                }
                LinkinfoIptunAttrs::Remote(val) => {
                    if last_off == offset {
                        stack.push(("Remote", last_off));
                        break;
                    }
                }
                LinkinfoIptunAttrs::Ttl(val) => {
                    if last_off == offset {
                        stack.push(("Ttl", last_off));
                        break;
                    }
                }
                LinkinfoIptunAttrs::Tos(val) => {
                    if last_off == offset {
                        stack.push(("Tos", last_off));
                        break;
                    }
                }
                LinkinfoIptunAttrs::EncapLimit(val) => {
                    if last_off == offset {
                        stack.push(("EncapLimit", last_off));
                        break;
                    }
                }
                LinkinfoIptunAttrs::Flowinfo(val) => {
                    if last_off == offset {
                        stack.push(("Flowinfo", last_off));
                        break;
                    }
                }
                LinkinfoIptunAttrs::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                LinkinfoIptunAttrs::Proto(val) => {
                    if last_off == offset {
                        stack.push(("Proto", last_off));
                        break;
                    }
                }
                LinkinfoIptunAttrs::Pmtudisc(val) => {
                    if last_off == offset {
                        stack.push(("Pmtudisc", last_off));
                        break;
                    }
                }
                LinkinfoIptunAttrs::_6rdPrefix(val) => {
                    if last_off == offset {
                        stack.push(("6rdPrefix", last_off));
                        break;
                    }
                }
                LinkinfoIptunAttrs::_6rdRelayPrefix(val) => {
                    if last_off == offset {
                        stack.push(("6rdRelayPrefix", last_off));
                        break;
                    }
                }
                LinkinfoIptunAttrs::_6rdPrefixlen(val) => {
                    if last_off == offset {
                        stack.push(("6rdPrefixlen", last_off));
                        break;
                    }
                }
                LinkinfoIptunAttrs::_6rdRelayPrefixlen(val) => {
                    if last_off == offset {
                        stack.push(("6rdRelayPrefixlen", last_off));
                        break;
                    }
                }
                LinkinfoIptunAttrs::EncapType(val) => {
                    if last_off == offset {
                        stack.push(("EncapType", last_off));
                        break;
                    }
                }
                LinkinfoIptunAttrs::EncapFlags(val) => {
                    if last_off == offset {
                        stack.push(("EncapFlags", last_off));
                        break;
                    }
                }
                LinkinfoIptunAttrs::EncapSport(val) => {
                    if last_off == offset {
                        stack.push(("EncapSport", last_off));
                        break;
                    }
                }
                LinkinfoIptunAttrs::EncapDport(val) => {
                    if last_off == offset {
                        stack.push(("EncapDport", last_off));
                        break;
                    }
                }
                LinkinfoIptunAttrs::CollectMetadata(val) => {
                    if last_off == offset {
                        stack.push(("CollectMetadata", last_off));
                        break;
                    }
                }
                LinkinfoIptunAttrs::Fwmark(val) => {
                    if last_off == offset {
                        stack.push(("Fwmark", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LinkinfoIptunAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LinkinfoIp6tnlAttrs<'a> {
    Link(u32),
    Local(&'a [u8]),
    Remote(&'a [u8]),
    Ttl(u8),
    EncapLimit(u8),
    Flowinfo(u32),
    Flags(u32),
    Proto(u8),
    EncapType(u16),
    EncapFlags(u16),
    EncapSport(u16),
    EncapDport(u16),
    CollectMetadata(()),
    Fwmark(u32),
}
impl<'a> IterableLinkinfoIp6tnlAttrs<'a> {
    pub fn get_link(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIp6tnlAttrs::Link(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIp6tnlAttrs",
            "Link",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_local(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIp6tnlAttrs::Local(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIp6tnlAttrs",
            "Local",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_remote(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIp6tnlAttrs::Remote(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIp6tnlAttrs",
            "Remote",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ttl(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIp6tnlAttrs::Ttl(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIp6tnlAttrs",
            "Ttl",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap_limit(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIp6tnlAttrs::EncapLimit(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIp6tnlAttrs",
            "EncapLimit",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flowinfo(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIp6tnlAttrs::Flowinfo(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIp6tnlAttrs",
            "Flowinfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIp6tnlAttrs::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIp6tnlAttrs",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_proto(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIp6tnlAttrs::Proto(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIp6tnlAttrs",
            "Proto",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap_type(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIp6tnlAttrs::EncapType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIp6tnlAttrs",
            "EncapType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap_flags(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIp6tnlAttrs::EncapFlags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIp6tnlAttrs",
            "EncapFlags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap_sport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIp6tnlAttrs::EncapSport(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIp6tnlAttrs",
            "EncapSport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_encap_dport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIp6tnlAttrs::EncapDport(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIp6tnlAttrs",
            "EncapDport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_collect_metadata(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIp6tnlAttrs::CollectMetadata(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIp6tnlAttrs",
            "CollectMetadata",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fwmark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoIp6tnlAttrs::Fwmark(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoIp6tnlAttrs",
            "Fwmark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LinkinfoIp6tnlAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLinkinfoIp6tnlAttrs<'a> {
        IterableLinkinfoIp6tnlAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        LinkinfoIptunAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLinkinfoIp6tnlAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLinkinfoIp6tnlAttrs<'a> {
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
impl<'a> Iterator for IterableLinkinfoIp6tnlAttrs<'a> {
    type Item = Result<LinkinfoIp6tnlAttrs<'a>, ErrorContext>;
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
                1u16 => LinkinfoIp6tnlAttrs::Link({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => LinkinfoIp6tnlAttrs::Local({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => LinkinfoIp6tnlAttrs::Remote({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => LinkinfoIp6tnlAttrs::Ttl({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => LinkinfoIp6tnlAttrs::EncapLimit({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => LinkinfoIp6tnlAttrs::Flowinfo({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => LinkinfoIp6tnlAttrs::Flags({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => LinkinfoIp6tnlAttrs::Proto({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => LinkinfoIp6tnlAttrs::EncapType({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => LinkinfoIp6tnlAttrs::EncapFlags({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => LinkinfoIp6tnlAttrs::EncapSport({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => LinkinfoIp6tnlAttrs::EncapDport({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => LinkinfoIp6tnlAttrs::CollectMetadata(()),
                20u16 => LinkinfoIp6tnlAttrs::Fwmark({
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
            "LinkinfoIp6tnlAttrs",
            r#type.and_then(|t| LinkinfoIp6tnlAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableLinkinfoIp6tnlAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LinkinfoIp6tnlAttrs");
        let mut iter = IterableLinkinfoIp6tnlAttrs::with_loc(&[], self.orig_loc);
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
                LinkinfoIp6tnlAttrs::Link(val) => fmt.field("Link", &val),
                LinkinfoIp6tnlAttrs::Local(val) => fmt.field("Local", &val),
                LinkinfoIp6tnlAttrs::Remote(val) => fmt.field("Remote", &val),
                LinkinfoIp6tnlAttrs::Ttl(val) => fmt.field("Ttl", &val),
                LinkinfoIp6tnlAttrs::EncapLimit(val) => fmt.field("EncapLimit", &val),
                LinkinfoIp6tnlAttrs::Flowinfo(val) => fmt.field("Flowinfo", &val),
                LinkinfoIp6tnlAttrs::Flags(val) => fmt.field("Flags", &val),
                LinkinfoIp6tnlAttrs::Proto(val) => fmt.field("Proto", &val),
                LinkinfoIp6tnlAttrs::EncapType(val) => fmt.field("EncapType", &val),
                LinkinfoIp6tnlAttrs::EncapFlags(val) => fmt.field("EncapFlags", &val),
                LinkinfoIp6tnlAttrs::EncapSport(val) => fmt.field("EncapSport", &val),
                LinkinfoIp6tnlAttrs::EncapDport(val) => fmt.field("EncapDport", &val),
                LinkinfoIp6tnlAttrs::CollectMetadata(val) => fmt.field("CollectMetadata", &val),
                LinkinfoIp6tnlAttrs::Fwmark(val) => fmt.field("Fwmark", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableLinkinfoIp6tnlAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LinkinfoIp6tnlAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| LinkinfoIp6tnlAttrs::attr_from_type(t)),
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
                LinkinfoIp6tnlAttrs::Link(val) => {
                    if last_off == offset {
                        stack.push(("Link", last_off));
                        break;
                    }
                }
                LinkinfoIp6tnlAttrs::Local(val) => {
                    if last_off == offset {
                        stack.push(("Local", last_off));
                        break;
                    }
                }
                LinkinfoIp6tnlAttrs::Remote(val) => {
                    if last_off == offset {
                        stack.push(("Remote", last_off));
                        break;
                    }
                }
                LinkinfoIp6tnlAttrs::Ttl(val) => {
                    if last_off == offset {
                        stack.push(("Ttl", last_off));
                        break;
                    }
                }
                LinkinfoIp6tnlAttrs::EncapLimit(val) => {
                    if last_off == offset {
                        stack.push(("EncapLimit", last_off));
                        break;
                    }
                }
                LinkinfoIp6tnlAttrs::Flowinfo(val) => {
                    if last_off == offset {
                        stack.push(("Flowinfo", last_off));
                        break;
                    }
                }
                LinkinfoIp6tnlAttrs::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                LinkinfoIp6tnlAttrs::Proto(val) => {
                    if last_off == offset {
                        stack.push(("Proto", last_off));
                        break;
                    }
                }
                LinkinfoIp6tnlAttrs::EncapType(val) => {
                    if last_off == offset {
                        stack.push(("EncapType", last_off));
                        break;
                    }
                }
                LinkinfoIp6tnlAttrs::EncapFlags(val) => {
                    if last_off == offset {
                        stack.push(("EncapFlags", last_off));
                        break;
                    }
                }
                LinkinfoIp6tnlAttrs::EncapSport(val) => {
                    if last_off == offset {
                        stack.push(("EncapSport", last_off));
                        break;
                    }
                }
                LinkinfoIp6tnlAttrs::EncapDport(val) => {
                    if last_off == offset {
                        stack.push(("EncapDport", last_off));
                        break;
                    }
                }
                LinkinfoIp6tnlAttrs::CollectMetadata(val) => {
                    if last_off == offset {
                        stack.push(("CollectMetadata", last_off));
                        break;
                    }
                }
                LinkinfoIp6tnlAttrs::Fwmark(val) => {
                    if last_off == offset {
                        stack.push(("Fwmark", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LinkinfoIp6tnlAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LinkinfoTunAttrs {
    Owner(u32),
    Group(u32),
    Type(u8),
    Pi(u8),
    VnetHdr(u8),
    Persist(u8),
    MultiQueue(u8),
    NumQueues(u32),
    NumDisabledQueues(u32),
}
impl<'a> IterableLinkinfoTunAttrs<'a> {
    pub fn get_owner(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoTunAttrs::Owner(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoTunAttrs",
            "Owner",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_group(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoTunAttrs::Group(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoTunAttrs",
            "Group",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_type(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoTunAttrs::Type(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoTunAttrs",
            "Type",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pi(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoTunAttrs::Pi(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoTunAttrs",
            "Pi",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vnet_hdr(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoTunAttrs::VnetHdr(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoTunAttrs",
            "VnetHdr",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_persist(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoTunAttrs::Persist(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoTunAttrs",
            "Persist",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_multi_queue(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoTunAttrs::MultiQueue(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoTunAttrs",
            "MultiQueue",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_num_queues(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoTunAttrs::NumQueues(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoTunAttrs",
            "NumQueues",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_num_disabled_queues(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoTunAttrs::NumDisabledQueues(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoTunAttrs",
            "NumDisabledQueues",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LinkinfoTunAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLinkinfoTunAttrs<'a> {
        IterableLinkinfoTunAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Owner",
            2u16 => "Group",
            3u16 => "Type",
            4u16 => "Pi",
            5u16 => "VnetHdr",
            6u16 => "Persist",
            7u16 => "MultiQueue",
            8u16 => "NumQueues",
            9u16 => "NumDisabledQueues",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLinkinfoTunAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLinkinfoTunAttrs<'a> {
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
impl<'a> Iterator for IterableLinkinfoTunAttrs<'a> {
    type Item = Result<LinkinfoTunAttrs, ErrorContext>;
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
                1u16 => LinkinfoTunAttrs::Owner({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => LinkinfoTunAttrs::Group({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => LinkinfoTunAttrs::Type({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => LinkinfoTunAttrs::Pi({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => LinkinfoTunAttrs::VnetHdr({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => LinkinfoTunAttrs::Persist({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => LinkinfoTunAttrs::MultiQueue({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => LinkinfoTunAttrs::NumQueues({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => LinkinfoTunAttrs::NumDisabledQueues({
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
            "LinkinfoTunAttrs",
            r#type.and_then(|t| LinkinfoTunAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableLinkinfoTunAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LinkinfoTunAttrs");
        let mut iter = IterableLinkinfoTunAttrs::with_loc(&[], self.orig_loc);
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
                LinkinfoTunAttrs::Owner(val) => fmt.field("Owner", &val),
                LinkinfoTunAttrs::Group(val) => fmt.field("Group", &val),
                LinkinfoTunAttrs::Type(val) => fmt.field("Type", &val),
                LinkinfoTunAttrs::Pi(val) => fmt.field("Pi", &val),
                LinkinfoTunAttrs::VnetHdr(val) => fmt.field("VnetHdr", &val),
                LinkinfoTunAttrs::Persist(val) => fmt.field("Persist", &val),
                LinkinfoTunAttrs::MultiQueue(val) => fmt.field("MultiQueue", &val),
                LinkinfoTunAttrs::NumQueues(val) => fmt.field("NumQueues", &val),
                LinkinfoTunAttrs::NumDisabledQueues(val) => fmt.field("NumDisabledQueues", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableLinkinfoTunAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LinkinfoTunAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| LinkinfoTunAttrs::attr_from_type(t)),
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
                LinkinfoTunAttrs::Owner(val) => {
                    if last_off == offset {
                        stack.push(("Owner", last_off));
                        break;
                    }
                }
                LinkinfoTunAttrs::Group(val) => {
                    if last_off == offset {
                        stack.push(("Group", last_off));
                        break;
                    }
                }
                LinkinfoTunAttrs::Type(val) => {
                    if last_off == offset {
                        stack.push(("Type", last_off));
                        break;
                    }
                }
                LinkinfoTunAttrs::Pi(val) => {
                    if last_off == offset {
                        stack.push(("Pi", last_off));
                        break;
                    }
                }
                LinkinfoTunAttrs::VnetHdr(val) => {
                    if last_off == offset {
                        stack.push(("VnetHdr", last_off));
                        break;
                    }
                }
                LinkinfoTunAttrs::Persist(val) => {
                    if last_off == offset {
                        stack.push(("Persist", last_off));
                        break;
                    }
                }
                LinkinfoTunAttrs::MultiQueue(val) => {
                    if last_off == offset {
                        stack.push(("MultiQueue", last_off));
                        break;
                    }
                }
                LinkinfoTunAttrs::NumQueues(val) => {
                    if last_off == offset {
                        stack.push(("NumQueues", last_off));
                        break;
                    }
                }
                LinkinfoTunAttrs::NumDisabledQueues(val) => {
                    if last_off == offset {
                        stack.push(("NumDisabledQueues", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LinkinfoTunAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LinkinfoVlanAttrs<'a> {
    Id(u16),
    Flags(IflaVlanFlags),
    EgressQos(IterableIflaVlanQos<'a>),
    IngressQos(IterableIflaVlanQos<'a>),
    #[doc = "Associated type: [`VlanProtocols`] (enum)"]
    Protocol(u16),
}
impl<'a> IterableLinkinfoVlanAttrs<'a> {
    pub fn get_id(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoVlanAttrs::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoVlanAttrs",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flags(&self) -> Result<IflaVlanFlags, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoVlanAttrs::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoVlanAttrs",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_egress_qos(&self) -> Result<IterableIflaVlanQos<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoVlanAttrs::EgressQos(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoVlanAttrs",
            "EgressQos",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ingress_qos(&self) -> Result<IterableIflaVlanQos<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoVlanAttrs::IngressQos(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoVlanAttrs",
            "IngressQos",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`VlanProtocols`] (enum)"]
    pub fn get_protocol(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoVlanAttrs::Protocol(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoVlanAttrs",
            "Protocol",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LinkinfoVlanAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLinkinfoVlanAttrs<'a> {
        IterableLinkinfoVlanAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Id",
            2u16 => "Flags",
            3u16 => "EgressQos",
            4u16 => "IngressQos",
            5u16 => "Protocol",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLinkinfoVlanAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLinkinfoVlanAttrs<'a> {
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
impl<'a> Iterator for IterableLinkinfoVlanAttrs<'a> {
    type Item = Result<LinkinfoVlanAttrs<'a>, ErrorContext>;
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
                1u16 => LinkinfoVlanAttrs::Id({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => LinkinfoVlanAttrs::Flags({
                    let res = Some(IflaVlanFlags::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => LinkinfoVlanAttrs::EgressQos({
                    let res = Some(IterableIflaVlanQos::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => LinkinfoVlanAttrs::IngressQos({
                    let res = Some(IterableIflaVlanQos::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => LinkinfoVlanAttrs::Protocol({
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
            "LinkinfoVlanAttrs",
            r#type.and_then(|t| LinkinfoVlanAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableLinkinfoVlanAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LinkinfoVlanAttrs");
        let mut iter = IterableLinkinfoVlanAttrs::with_loc(&[], self.orig_loc);
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
                LinkinfoVlanAttrs::Id(val) => fmt.field("Id", &val),
                LinkinfoVlanAttrs::Flags(val) => fmt.field("Flags", &val),
                LinkinfoVlanAttrs::EgressQos(val) => fmt.field("EgressQos", &val),
                LinkinfoVlanAttrs::IngressQos(val) => fmt.field("IngressQos", &val),
                LinkinfoVlanAttrs::Protocol(val) => fmt.field(
                    "Protocol",
                    &FormatEnum(val.into(), VlanProtocols::from_value),
                ),
            };
        }
        fmt.finish()
    }
}
impl IterableLinkinfoVlanAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LinkinfoVlanAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| LinkinfoVlanAttrs::attr_from_type(t)),
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
                LinkinfoVlanAttrs::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                LinkinfoVlanAttrs::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                LinkinfoVlanAttrs::EgressQos(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                LinkinfoVlanAttrs::IngressQos(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                LinkinfoVlanAttrs::Protocol(val) => {
                    if last_off == offset {
                        stack.push(("Protocol", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LinkinfoVlanAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum IflaVlanQos {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    Mapping(IflaVlanQosMapping),
}
impl<'a> IterableIflaVlanQos<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_mapping(&self) -> MultiAttrIterable<Self, IflaVlanQos, IflaVlanQosMapping> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let IflaVlanQos::Mapping(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl IflaVlanQos {
    pub fn new<'a>(buf: &'a [u8]) -> IterableIflaVlanQos<'a> {
        IterableIflaVlanQos::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Mapping",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableIflaVlanQos<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableIflaVlanQos<'a> {
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
impl<'a> Iterator for IterableIflaVlanQos<'a> {
    type Item = Result<IflaVlanQos, ErrorContext>;
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
                1u16 => IflaVlanQos::Mapping({
                    let res = Some(IflaVlanQosMapping::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "IflaVlanQos",
            r#type.and_then(|t| IflaVlanQos::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableIflaVlanQos<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("IflaVlanQos");
        let mut iter = IterableIflaVlanQos::with_loc(&[], self.orig_loc);
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
                IflaVlanQos::Mapping(val) => fmt.field("Mapping", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableIflaVlanQos<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("IflaVlanQos", offset));
            return (
                stack,
                missing_type.and_then(|t| IflaVlanQos::attr_from_type(t)),
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
                IflaVlanQos::Mapping(val) => {
                    if last_off == offset {
                        stack.push(("Mapping", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("IflaVlanQos", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LinkinfoVrfAttrs {
    Table(u32),
}
impl<'a> IterableLinkinfoVrfAttrs<'a> {
    pub fn get_table(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoVrfAttrs::Table(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoVrfAttrs",
            "Table",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LinkinfoVrfAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLinkinfoVrfAttrs<'a> {
        IterableLinkinfoVrfAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Table",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLinkinfoVrfAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLinkinfoVrfAttrs<'a> {
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
impl<'a> Iterator for IterableLinkinfoVrfAttrs<'a> {
    type Item = Result<LinkinfoVrfAttrs, ErrorContext>;
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
                1u16 => LinkinfoVrfAttrs::Table({
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
            "LinkinfoVrfAttrs",
            r#type.and_then(|t| LinkinfoVrfAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableLinkinfoVrfAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LinkinfoVrfAttrs");
        let mut iter = IterableLinkinfoVrfAttrs::with_loc(&[], self.orig_loc);
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
                LinkinfoVrfAttrs::Table(val) => fmt.field("Table", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableLinkinfoVrfAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LinkinfoVrfAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| LinkinfoVrfAttrs::attr_from_type(t)),
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
                LinkinfoVrfAttrs::Table(val) => {
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
            stack.push(("LinkinfoVrfAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum XdpAttrs {
    Fd(i32),
    Attached(u8),
    Flags(u32),
    ProgId(u32),
    DrvProgId(u32),
    SkbProgId(u32),
    HwProgId(u32),
    ExpectedFd(i32),
}
impl<'a> IterableXdpAttrs<'a> {
    pub fn get_fd(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(XdpAttrs::Fd(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "XdpAttrs",
            "Fd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_attached(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(XdpAttrs::Attached(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "XdpAttrs",
            "Attached",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(XdpAttrs::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "XdpAttrs",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_prog_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(XdpAttrs::ProgId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "XdpAttrs",
            "ProgId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_drv_prog_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(XdpAttrs::DrvProgId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "XdpAttrs",
            "DrvProgId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_skb_prog_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(XdpAttrs::SkbProgId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "XdpAttrs",
            "SkbProgId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_hw_prog_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(XdpAttrs::HwProgId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "XdpAttrs",
            "HwProgId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_expected_fd(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(XdpAttrs::ExpectedFd(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "XdpAttrs",
            "ExpectedFd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl XdpAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableXdpAttrs<'a> {
        IterableXdpAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Fd",
            2u16 => "Attached",
            3u16 => "Flags",
            4u16 => "ProgId",
            5u16 => "DrvProgId",
            6u16 => "SkbProgId",
            7u16 => "HwProgId",
            8u16 => "ExpectedFd",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableXdpAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableXdpAttrs<'a> {
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
impl<'a> Iterator for IterableXdpAttrs<'a> {
    type Item = Result<XdpAttrs, ErrorContext>;
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
                1u16 => XdpAttrs::Fd({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => XdpAttrs::Attached({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => XdpAttrs::Flags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => XdpAttrs::ProgId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => XdpAttrs::DrvProgId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => XdpAttrs::SkbProgId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => XdpAttrs::HwProgId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => XdpAttrs::ExpectedFd({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "XdpAttrs",
            r#type.and_then(|t| XdpAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableXdpAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("XdpAttrs");
        let mut iter = IterableXdpAttrs::with_loc(&[], self.orig_loc);
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
                XdpAttrs::Fd(val) => fmt.field("Fd", &val),
                XdpAttrs::Attached(val) => fmt.field("Attached", &val),
                XdpAttrs::Flags(val) => fmt.field("Flags", &val),
                XdpAttrs::ProgId(val) => fmt.field("ProgId", &val),
                XdpAttrs::DrvProgId(val) => fmt.field("DrvProgId", &val),
                XdpAttrs::SkbProgId(val) => fmt.field("SkbProgId", &val),
                XdpAttrs::HwProgId(val) => fmt.field("HwProgId", &val),
                XdpAttrs::ExpectedFd(val) => fmt.field("ExpectedFd", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableXdpAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("XdpAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| XdpAttrs::attr_from_type(t)),
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
                XdpAttrs::Fd(val) => {
                    if last_off == offset {
                        stack.push(("Fd", last_off));
                        break;
                    }
                }
                XdpAttrs::Attached(val) => {
                    if last_off == offset {
                        stack.push(("Attached", last_off));
                        break;
                    }
                }
                XdpAttrs::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                XdpAttrs::ProgId(val) => {
                    if last_off == offset {
                        stack.push(("ProgId", last_off));
                        break;
                    }
                }
                XdpAttrs::DrvProgId(val) => {
                    if last_off == offset {
                        stack.push(("DrvProgId", last_off));
                        break;
                    }
                }
                XdpAttrs::SkbProgId(val) => {
                    if last_off == offset {
                        stack.push(("SkbProgId", last_off));
                        break;
                    }
                }
                XdpAttrs::HwProgId(val) => {
                    if last_off == offset {
                        stack.push(("HwProgId", last_off));
                        break;
                    }
                }
                XdpAttrs::ExpectedFd(val) => {
                    if last_off == offset {
                        stack.push(("ExpectedFd", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("XdpAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum IflaAttrs<'a> {
    #[doc = "u32 indexed by ipv4-devconf - 1 on output, on input it\\'s a nest\n"]
    Conf(&'a [u8]),
}
impl<'a> IterableIflaAttrs<'a> {
    #[doc = "u32 indexed by ipv4-devconf - 1 on output, on input it\\'s a nest\n"]
    pub fn get_conf(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(IflaAttrs::Conf(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IflaAttrs",
            "Conf",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl IflaAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableIflaAttrs<'a> {
        IterableIflaAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Conf",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableIflaAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableIflaAttrs<'a> {
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
impl<'a> Iterator for IterableIflaAttrs<'a> {
    type Item = Result<IflaAttrs<'a>, ErrorContext>;
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
                1u16 => IflaAttrs::Conf({
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
            "IflaAttrs",
            r#type.and_then(|t| IflaAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableIflaAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("IflaAttrs");
        let mut iter = IterableIflaAttrs::with_loc(&[], self.orig_loc);
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
                IflaAttrs::Conf(val) => fmt.field("Conf", &FormatHexdump(val)),
            };
        }
        fmt.finish()
    }
}
impl IterableIflaAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("IflaAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| IflaAttrs::attr_from_type(t)),
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
                IflaAttrs::Conf(val) => {
                    if last_off == offset {
                        stack.push(("Conf", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("IflaAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum Ifla6Attrs<'a> {
    Flags(u32),
    #[doc = "u32 indexed by ipv6-devconf - 1 on output, on input it\\'s a nest\n"]
    Conf(&'a [u8]),
    Stats(&'a [u8]),
    Mcast(&'a [u8]),
    Cacheinfo(IflaCacheinfo),
    Icmp6stats(&'a [u8]),
    Token(&'a [u8]),
    AddrGenMode(u8),
    RaMtu(u32),
}
impl<'a> IterableIfla6Attrs<'a> {
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Ifla6Attrs::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Ifla6Attrs",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "u32 indexed by ipv6-devconf - 1 on output, on input it\\'s a nest\n"]
    pub fn get_conf(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Ifla6Attrs::Conf(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Ifla6Attrs",
            "Conf",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_stats(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Ifla6Attrs::Stats(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Ifla6Attrs",
            "Stats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Ifla6Attrs::Mcast(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Ifla6Attrs",
            "Mcast",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_cacheinfo(&self) -> Result<IflaCacheinfo, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Ifla6Attrs::Cacheinfo(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Ifla6Attrs",
            "Cacheinfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_icmp6stats(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Ifla6Attrs::Icmp6stats(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Ifla6Attrs",
            "Icmp6stats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_token(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Ifla6Attrs::Token(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Ifla6Attrs",
            "Token",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_addr_gen_mode(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Ifla6Attrs::AddrGenMode(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Ifla6Attrs",
            "AddrGenMode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ra_mtu(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Ifla6Attrs::RaMtu(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Ifla6Attrs",
            "RaMtu",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Ifla6Attrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableIfla6Attrs<'a> {
        IterableIfla6Attrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Flags",
            2u16 => "Conf",
            3u16 => "Stats",
            4u16 => "Mcast",
            5u16 => "Cacheinfo",
            6u16 => "Icmp6stats",
            7u16 => "Token",
            8u16 => "AddrGenMode",
            9u16 => "RaMtu",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableIfla6Attrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableIfla6Attrs<'a> {
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
impl<'a> Iterator for IterableIfla6Attrs<'a> {
    type Item = Result<Ifla6Attrs<'a>, ErrorContext>;
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
                1u16 => Ifla6Attrs::Flags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Ifla6Attrs::Conf({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Ifla6Attrs::Stats({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Ifla6Attrs::Mcast({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Ifla6Attrs::Cacheinfo({
                    let res = Some(IflaCacheinfo::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Ifla6Attrs::Icmp6stats({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => Ifla6Attrs::Token({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => Ifla6Attrs::AddrGenMode({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => Ifla6Attrs::RaMtu({
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
            "Ifla6Attrs",
            r#type.and_then(|t| Ifla6Attrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableIfla6Attrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Ifla6Attrs");
        let mut iter = IterableIfla6Attrs::with_loc(&[], self.orig_loc);
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
                Ifla6Attrs::Flags(val) => fmt.field("Flags", &val),
                Ifla6Attrs::Conf(val) => fmt.field("Conf", &FormatHexdump(val)),
                Ifla6Attrs::Stats(val) => fmt.field("Stats", &FormatHexdump(val)),
                Ifla6Attrs::Mcast(val) => fmt.field("Mcast", &FormatHexdump(val)),
                Ifla6Attrs::Cacheinfo(val) => fmt.field("Cacheinfo", &val),
                Ifla6Attrs::Icmp6stats(val) => fmt.field("Icmp6stats", &FormatHexdump(val)),
                Ifla6Attrs::Token(val) => fmt.field("Token", &FormatHexdump(val)),
                Ifla6Attrs::AddrGenMode(val) => fmt.field("AddrGenMode", &val),
                Ifla6Attrs::RaMtu(val) => fmt.field("RaMtu", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableIfla6Attrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Ifla6Attrs", offset));
            return (
                stack,
                missing_type.and_then(|t| Ifla6Attrs::attr_from_type(t)),
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
                Ifla6Attrs::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                Ifla6Attrs::Conf(val) => {
                    if last_off == offset {
                        stack.push(("Conf", last_off));
                        break;
                    }
                }
                Ifla6Attrs::Stats(val) => {
                    if last_off == offset {
                        stack.push(("Stats", last_off));
                        break;
                    }
                }
                Ifla6Attrs::Mcast(val) => {
                    if last_off == offset {
                        stack.push(("Mcast", last_off));
                        break;
                    }
                }
                Ifla6Attrs::Cacheinfo(val) => {
                    if last_off == offset {
                        stack.push(("Cacheinfo", last_off));
                        break;
                    }
                }
                Ifla6Attrs::Icmp6stats(val) => {
                    if last_off == offset {
                        stack.push(("Icmp6stats", last_off));
                        break;
                    }
                }
                Ifla6Attrs::Token(val) => {
                    if last_off == offset {
                        stack.push(("Token", last_off));
                        break;
                    }
                }
                Ifla6Attrs::AddrGenMode(val) => {
                    if last_off == offset {
                        stack.push(("AddrGenMode", last_off));
                        break;
                    }
                }
                Ifla6Attrs::RaMtu(val) => {
                    if last_off == offset {
                        stack.push(("RaMtu", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Ifla6Attrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum MctpAttrs {
    Net(u32),
    PhysBinding(u8),
}
impl<'a> IterableMctpAttrs<'a> {
    pub fn get_net(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(MctpAttrs::Net(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "MctpAttrs",
            "Net",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_phys_binding(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(MctpAttrs::PhysBinding(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "MctpAttrs",
            "PhysBinding",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl MctpAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableMctpAttrs<'a> {
        IterableMctpAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Net",
            2u16 => "PhysBinding",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableMctpAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableMctpAttrs<'a> {
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
impl<'a> Iterator for IterableMctpAttrs<'a> {
    type Item = Result<MctpAttrs, ErrorContext>;
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
                1u16 => MctpAttrs::Net({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => MctpAttrs::PhysBinding({
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
            "MctpAttrs",
            r#type.and_then(|t| MctpAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableMctpAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("MctpAttrs");
        let mut iter = IterableMctpAttrs::with_loc(&[], self.orig_loc);
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
                MctpAttrs::Net(val) => fmt.field("Net", &val),
                MctpAttrs::PhysBinding(val) => fmt.field("PhysBinding", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableMctpAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("MctpAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| MctpAttrs::attr_from_type(t)),
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
                MctpAttrs::Net(val) => {
                    if last_off == offset {
                        stack.push(("Net", last_off));
                        break;
                    }
                }
                MctpAttrs::PhysBinding(val) => {
                    if last_off == offset {
                        stack.push(("PhysBinding", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("MctpAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum StatsAttrs<'a> {
    Link64(RtnlLinkStats64),
    LinkXstats(&'a [u8]),
    LinkXstatsSlave(&'a [u8]),
    LinkOffloadXstats(IterableLinkOffloadXstats<'a>),
    AfSpec(&'a [u8]),
}
impl<'a> IterableStatsAttrs<'a> {
    pub fn get_link_64(&self) -> Result<RtnlLinkStats64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(StatsAttrs::Link64(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "StatsAttrs",
            "Link64",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_link_xstats(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(StatsAttrs::LinkXstats(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "StatsAttrs",
            "LinkXstats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_link_xstats_slave(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(StatsAttrs::LinkXstatsSlave(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "StatsAttrs",
            "LinkXstatsSlave",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_link_offload_xstats(&self) -> Result<IterableLinkOffloadXstats<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(StatsAttrs::LinkOffloadXstats(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "StatsAttrs",
            "LinkOffloadXstats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_af_spec(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(StatsAttrs::AfSpec(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "StatsAttrs",
            "AfSpec",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl StatsAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableStatsAttrs<'a> {
        IterableStatsAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Link64",
            2u16 => "LinkXstats",
            3u16 => "LinkXstatsSlave",
            4u16 => "LinkOffloadXstats",
            5u16 => "AfSpec",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableStatsAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableStatsAttrs<'a> {
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
impl<'a> Iterator for IterableStatsAttrs<'a> {
    type Item = Result<StatsAttrs<'a>, ErrorContext>;
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
                1u16 => StatsAttrs::Link64({
                    let res = Some(RtnlLinkStats64::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => StatsAttrs::LinkXstats({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => StatsAttrs::LinkXstatsSlave({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => StatsAttrs::LinkOffloadXstats({
                    let res = Some(IterableLinkOffloadXstats::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => StatsAttrs::AfSpec({
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
            "StatsAttrs",
            r#type.and_then(|t| StatsAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableStatsAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("StatsAttrs");
        let mut iter = IterableStatsAttrs::with_loc(&[], self.orig_loc);
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
                StatsAttrs::Link64(val) => fmt.field("Link64", &val),
                StatsAttrs::LinkXstats(val) => fmt.field("LinkXstats", &FormatHexdump(val)),
                StatsAttrs::LinkXstatsSlave(val) => {
                    fmt.field("LinkXstatsSlave", &FormatHexdump(val))
                }
                StatsAttrs::LinkOffloadXstats(val) => fmt.field("LinkOffloadXstats", &val),
                StatsAttrs::AfSpec(val) => fmt.field("AfSpec", &FormatHexdump(val)),
            };
        }
        fmt.finish()
    }
}
impl IterableStatsAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("StatsAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| StatsAttrs::attr_from_type(t)),
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
                StatsAttrs::Link64(val) => {
                    if last_off == offset {
                        stack.push(("Link64", last_off));
                        break;
                    }
                }
                StatsAttrs::LinkXstats(val) => {
                    if last_off == offset {
                        stack.push(("LinkXstats", last_off));
                        break;
                    }
                }
                StatsAttrs::LinkXstatsSlave(val) => {
                    if last_off == offset {
                        stack.push(("LinkXstatsSlave", last_off));
                        break;
                    }
                }
                StatsAttrs::LinkOffloadXstats(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                StatsAttrs::AfSpec(val) => {
                    if last_off == offset {
                        stack.push(("AfSpec", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("StatsAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum LinkOffloadXstats<'a> {
    CpuHit(&'a [u8]),
    HwSInfo(IterableArrayHwSInfoOne<'a>),
    L3Stats(&'a [u8]),
}
impl<'a> IterableLinkOffloadXstats<'a> {
    pub fn get_cpu_hit(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkOffloadXstats::CpuHit(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkOffloadXstats",
            "CpuHit",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_hw_s_info(
        &self,
    ) -> Result<ArrayIterable<IterableArrayHwSInfoOne<'a>, IterableHwSInfoOne<'a>>, ErrorContext>
    {
        for attr in self.clone() {
            if let Ok(LinkOffloadXstats::HwSInfo(val)) = attr {
                return Ok(ArrayIterable::new(val));
            }
        }
        Err(ErrorContext::new_missing(
            "LinkOffloadXstats",
            "HwSInfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_l3_stats(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkOffloadXstats::L3Stats(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkOffloadXstats",
            "L3Stats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl HwSInfoOne {
    pub fn new_array(buf: &[u8]) -> IterableArrayHwSInfoOne<'_> {
        IterableArrayHwSInfoOne::with_loc(buf, buf.as_ptr() as usize)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableArrayHwSInfoOne<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableArrayHwSInfoOne<'a> {
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
impl<'a> Iterator for IterableArrayHwSInfoOne<'a> {
    type Item = Result<IterableHwSInfoOne<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            {
                return Some(Ok(IterableHwSInfoOne::with_loc(next, self.orig_loc)));
            }
        }
        let pos = self.pos;
        self.pos = self.buf.len();
        Some(Err(ErrorContext::new(
            "HwSInfoOne",
            None,
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl LinkOffloadXstats<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLinkOffloadXstats<'a> {
        IterableLinkOffloadXstats::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "CpuHit",
            2u16 => "HwSInfo",
            3u16 => "L3Stats",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLinkOffloadXstats<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLinkOffloadXstats<'a> {
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
impl<'a> Iterator for IterableLinkOffloadXstats<'a> {
    type Item = Result<LinkOffloadXstats<'a>, ErrorContext>;
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
                1u16 => LinkOffloadXstats::CpuHit({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => LinkOffloadXstats::HwSInfo({
                    let res = Some(IterableArrayHwSInfoOne::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => LinkOffloadXstats::L3Stats({
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
            "LinkOffloadXstats",
            r#type.and_then(|t| LinkOffloadXstats::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableArrayHwSInfoOne<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_list()
            .entries(self.clone().map(FlattenErrorContext))
            .finish()
    }
}
impl<'a> std::fmt::Debug for IterableLinkOffloadXstats<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LinkOffloadXstats");
        let mut iter = IterableLinkOffloadXstats::with_loc(&[], self.orig_loc);
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
                LinkOffloadXstats::CpuHit(val) => fmt.field("CpuHit", &FormatHexdump(val)),
                LinkOffloadXstats::HwSInfo(val) => fmt.field("HwSInfo", &val),
                LinkOffloadXstats::L3Stats(val) => fmt.field("L3Stats", &FormatHexdump(val)),
            };
        }
        fmt.finish()
    }
}
impl IterableLinkOffloadXstats<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LinkOffloadXstats", offset));
            return (
                stack,
                missing_type.and_then(|t| LinkOffloadXstats::attr_from_type(t)),
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
                LinkOffloadXstats::CpuHit(val) => {
                    if last_off == offset {
                        stack.push(("CpuHit", last_off));
                        break;
                    }
                }
                LinkOffloadXstats::HwSInfo(val) => {
                    for entry in val {
                        let Ok(attr) = entry else { break };
                        (stack, missing) = attr.lookup_attr(offset, missing_type);
                        if !stack.is_empty() {
                            break;
                        }
                    }
                    if !stack.is_empty() {
                        stack.push(("HwSInfo", last_off));
                        break;
                    }
                }
                LinkOffloadXstats::L3Stats(val) => {
                    if last_off == offset {
                        stack.push(("L3Stats", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LinkOffloadXstats", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum HwSInfoOne {
    Request(u8),
    Used(u8),
}
impl<'a> IterableHwSInfoOne<'a> {
    pub fn get_request(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(HwSInfoOne::Request(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "HwSInfoOne",
            "Request",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_used(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(HwSInfoOne::Used(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "HwSInfoOne",
            "Used",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl HwSInfoOne {
    pub fn new<'a>(buf: &'a [u8]) -> IterableHwSInfoOne<'a> {
        IterableHwSInfoOne::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Request",
            2u16 => "Used",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableHwSInfoOne<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableHwSInfoOne<'a> {
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
impl<'a> Iterator for IterableHwSInfoOne<'a> {
    type Item = Result<HwSInfoOne, ErrorContext>;
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
                1u16 => HwSInfoOne::Request({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => HwSInfoOne::Used({
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
            "HwSInfoOne",
            r#type.and_then(|t| HwSInfoOne::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableHwSInfoOne<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("HwSInfoOne");
        let mut iter = IterableHwSInfoOne::with_loc(&[], self.orig_loc);
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
                HwSInfoOne::Request(val) => fmt.field("Request", &val),
                HwSInfoOne::Used(val) => fmt.field("Used", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableHwSInfoOne<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("HwSInfoOne", offset));
            return (
                stack,
                missing_type.and_then(|t| HwSInfoOne::attr_from_type(t)),
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
                HwSInfoOne::Request(val) => {
                    if last_off == offset {
                        stack.push(("Request", last_off));
                        break;
                    }
                }
                HwSInfoOne::Used(val) => {
                    if last_off == offset {
                        stack.push(("Used", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("HwSInfoOne", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LinkDpllPinAttrs {
    Id(u32),
}
impl<'a> IterableLinkDpllPinAttrs<'a> {
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkDpllPinAttrs::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkDpllPinAttrs",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LinkDpllPinAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLinkDpllPinAttrs<'a> {
        IterableLinkDpllPinAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Id",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLinkDpllPinAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLinkDpllPinAttrs<'a> {
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
impl<'a> Iterator for IterableLinkDpllPinAttrs<'a> {
    type Item = Result<LinkDpllPinAttrs, ErrorContext>;
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
                1u16 => LinkDpllPinAttrs::Id({
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
            "LinkDpllPinAttrs",
            r#type.and_then(|t| LinkDpllPinAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableLinkDpllPinAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LinkDpllPinAttrs");
        let mut iter = IterableLinkDpllPinAttrs::with_loc(&[], self.orig_loc);
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
                LinkDpllPinAttrs::Id(val) => fmt.field("Id", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableLinkDpllPinAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LinkDpllPinAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| LinkDpllPinAttrs::attr_from_type(t)),
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
                LinkDpllPinAttrs::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LinkDpllPinAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LinkinfoNetkitAttrs<'a> {
    PeerInfo(&'a [u8]),
    Primary(u8),
    #[doc = "Associated type: [`NetkitPolicy`] (enum)"]
    Policy(u32),
    #[doc = "Associated type: [`NetkitPolicy`] (enum)"]
    PeerPolicy(u32),
    #[doc = "Associated type: [`NetkitMode`] (enum)"]
    Mode(u32),
    #[doc = "Associated type: [`NetkitScrub`] (enum)"]
    Scrub(u32),
    #[doc = "Associated type: [`NetkitScrub`] (enum)"]
    PeerScrub(u32),
    Headroom(u16),
    Tailroom(u16),
    #[doc = "Associated type: [`NetkitPairing`] (enum)"]
    Pairing(u32),
}
impl<'a> IterableLinkinfoNetkitAttrs<'a> {
    pub fn get_peer_info(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoNetkitAttrs::PeerInfo(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoNetkitAttrs",
            "PeerInfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_primary(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoNetkitAttrs::Primary(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoNetkitAttrs",
            "Primary",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`NetkitPolicy`] (enum)"]
    pub fn get_policy(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoNetkitAttrs::Policy(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoNetkitAttrs",
            "Policy",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`NetkitPolicy`] (enum)"]
    pub fn get_peer_policy(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoNetkitAttrs::PeerPolicy(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoNetkitAttrs",
            "PeerPolicy",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`NetkitMode`] (enum)"]
    pub fn get_mode(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoNetkitAttrs::Mode(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoNetkitAttrs",
            "Mode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`NetkitScrub`] (enum)"]
    pub fn get_scrub(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoNetkitAttrs::Scrub(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoNetkitAttrs",
            "Scrub",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`NetkitScrub`] (enum)"]
    pub fn get_peer_scrub(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoNetkitAttrs::PeerScrub(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoNetkitAttrs",
            "PeerScrub",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_headroom(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoNetkitAttrs::Headroom(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoNetkitAttrs",
            "Headroom",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tailroom(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoNetkitAttrs::Tailroom(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoNetkitAttrs",
            "Tailroom",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`NetkitPairing`] (enum)"]
    pub fn get_pairing(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoNetkitAttrs::Pairing(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoNetkitAttrs",
            "Pairing",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LinkinfoNetkitAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLinkinfoNetkitAttrs<'a> {
        IterableLinkinfoNetkitAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "PeerInfo",
            2u16 => "Primary",
            3u16 => "Policy",
            4u16 => "PeerPolicy",
            5u16 => "Mode",
            6u16 => "Scrub",
            7u16 => "PeerScrub",
            8u16 => "Headroom",
            9u16 => "Tailroom",
            10u16 => "Pairing",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLinkinfoNetkitAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLinkinfoNetkitAttrs<'a> {
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
impl<'a> Iterator for IterableLinkinfoNetkitAttrs<'a> {
    type Item = Result<LinkinfoNetkitAttrs<'a>, ErrorContext>;
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
                1u16 => LinkinfoNetkitAttrs::PeerInfo({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => LinkinfoNetkitAttrs::Primary({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => LinkinfoNetkitAttrs::Policy({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => LinkinfoNetkitAttrs::PeerPolicy({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => LinkinfoNetkitAttrs::Mode({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => LinkinfoNetkitAttrs::Scrub({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => LinkinfoNetkitAttrs::PeerScrub({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => LinkinfoNetkitAttrs::Headroom({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => LinkinfoNetkitAttrs::Tailroom({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => LinkinfoNetkitAttrs::Pairing({
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
            "LinkinfoNetkitAttrs",
            r#type.and_then(|t| LinkinfoNetkitAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableLinkinfoNetkitAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LinkinfoNetkitAttrs");
        let mut iter = IterableLinkinfoNetkitAttrs::with_loc(&[], self.orig_loc);
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
                LinkinfoNetkitAttrs::PeerInfo(val) => fmt.field("PeerInfo", &FormatHexdump(val)),
                LinkinfoNetkitAttrs::Primary(val) => fmt.field("Primary", &val),
                LinkinfoNetkitAttrs::Policy(val) => {
                    fmt.field("Policy", &FormatEnum(val.into(), NetkitPolicy::from_value))
                }
                LinkinfoNetkitAttrs::PeerPolicy(val) => fmt.field(
                    "PeerPolicy",
                    &FormatEnum(val.into(), NetkitPolicy::from_value),
                ),
                LinkinfoNetkitAttrs::Mode(val) => {
                    fmt.field("Mode", &FormatEnum(val.into(), NetkitMode::from_value))
                }
                LinkinfoNetkitAttrs::Scrub(val) => {
                    fmt.field("Scrub", &FormatEnum(val.into(), NetkitScrub::from_value))
                }
                LinkinfoNetkitAttrs::PeerScrub(val) => fmt.field(
                    "PeerScrub",
                    &FormatEnum(val.into(), NetkitScrub::from_value),
                ),
                LinkinfoNetkitAttrs::Headroom(val) => fmt.field("Headroom", &val),
                LinkinfoNetkitAttrs::Tailroom(val) => fmt.field("Tailroom", &val),
                LinkinfoNetkitAttrs::Pairing(val) => fmt.field(
                    "Pairing",
                    &FormatEnum(val.into(), NetkitPairing::from_value),
                ),
            };
        }
        fmt.finish()
    }
}
impl IterableLinkinfoNetkitAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LinkinfoNetkitAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| LinkinfoNetkitAttrs::attr_from_type(t)),
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
                LinkinfoNetkitAttrs::PeerInfo(val) => {
                    if last_off == offset {
                        stack.push(("PeerInfo", last_off));
                        break;
                    }
                }
                LinkinfoNetkitAttrs::Primary(val) => {
                    if last_off == offset {
                        stack.push(("Primary", last_off));
                        break;
                    }
                }
                LinkinfoNetkitAttrs::Policy(val) => {
                    if last_off == offset {
                        stack.push(("Policy", last_off));
                        break;
                    }
                }
                LinkinfoNetkitAttrs::PeerPolicy(val) => {
                    if last_off == offset {
                        stack.push(("PeerPolicy", last_off));
                        break;
                    }
                }
                LinkinfoNetkitAttrs::Mode(val) => {
                    if last_off == offset {
                        stack.push(("Mode", last_off));
                        break;
                    }
                }
                LinkinfoNetkitAttrs::Scrub(val) => {
                    if last_off == offset {
                        stack.push(("Scrub", last_off));
                        break;
                    }
                }
                LinkinfoNetkitAttrs::PeerScrub(val) => {
                    if last_off == offset {
                        stack.push(("PeerScrub", last_off));
                        break;
                    }
                }
                LinkinfoNetkitAttrs::Headroom(val) => {
                    if last_off == offset {
                        stack.push(("Headroom", last_off));
                        break;
                    }
                }
                LinkinfoNetkitAttrs::Tailroom(val) => {
                    if last_off == offset {
                        stack.push(("Tailroom", last_off));
                        break;
                    }
                }
                LinkinfoNetkitAttrs::Pairing(val) => {
                    if last_off == offset {
                        stack.push(("Pairing", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LinkinfoNetkitAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LinkinfoOvpnAttrs {
    #[doc = "Associated type: [`OvpnMode`] (enum)"]
    Mode(u8),
}
impl<'a> IterableLinkinfoOvpnAttrs<'a> {
    #[doc = "Associated type: [`OvpnMode`] (enum)"]
    pub fn get_mode(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LinkinfoOvpnAttrs::Mode(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LinkinfoOvpnAttrs",
            "Mode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LinkinfoOvpnAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLinkinfoOvpnAttrs<'a> {
        IterableLinkinfoOvpnAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Mode",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLinkinfoOvpnAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLinkinfoOvpnAttrs<'a> {
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
impl<'a> Iterator for IterableLinkinfoOvpnAttrs<'a> {
    type Item = Result<LinkinfoOvpnAttrs, ErrorContext>;
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
                1u16 => LinkinfoOvpnAttrs::Mode({
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
            "LinkinfoOvpnAttrs",
            r#type.and_then(|t| LinkinfoOvpnAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableLinkinfoOvpnAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LinkinfoOvpnAttrs");
        let mut iter = IterableLinkinfoOvpnAttrs::with_loc(&[], self.orig_loc);
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
                LinkinfoOvpnAttrs::Mode(val) => {
                    fmt.field("Mode", &FormatEnum(val.into(), OvpnMode::from_value))
                }
            };
        }
        fmt.finish()
    }
}
impl IterableLinkinfoOvpnAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LinkinfoOvpnAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| LinkinfoOvpnAttrs::attr_from_type(t)),
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
                LinkinfoOvpnAttrs::Mode(val) => {
                    if last_off == offset {
                        stack.push(("Mode", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LinkinfoOvpnAttrs", cur));
        }
        (stack, None)
    }
}
pub struct PushLinkAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLinkAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLinkAttrs<Prev> {
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
    pub fn push_address(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_broadcast(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_ifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            3u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_ifname_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_mtu(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_link(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_qdisc(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            6u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_qdisc_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 6u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_stats(mut self, value: RtnlLinkStats) -> Self {
        push_header(self.as_vec_mut(), 7u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_master(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "struct iw_event\n"]
    pub fn push_wireless(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 11u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "A nest of ifla6-attrs or linkinfo-brport-attrs\n"]
    pub fn push_protinfo(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 12u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_txqlen(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 13u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_map(mut self, value: RtnlLinkIfmap) -> Self {
        push_header(self.as_vec_mut(), 14u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_weight(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 15u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_operstate(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 16u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_linkmode(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 17u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_linkinfo(mut self) -> PushLinkinfoAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 18u16);
        PushLinkinfoAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_net_ns_pid(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 19u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ifalias(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            20u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_ifalias_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 20u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_num_vf(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 21u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_vfinfo_list(mut self) -> PushVfinfoListAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 22u16);
        PushVfinfoListAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_stats64(mut self, value: RtnlLinkStats64) -> Self {
        push_header(self.as_vec_mut(), 23u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn nested_vf_ports(mut self) -> PushVfPortsAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 24u16);
        PushVfPortsAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_port_self(mut self) -> PushPortSelfAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 25u16);
        PushPortSelfAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_af_spec(mut self) -> PushAfSpecAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 26u16);
        PushAfSpecAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_group(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 27u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_net_ns_fd(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 28u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`RtextFilter`] (1 bit per enumeration)"]
    pub fn push_ext_mask(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 29u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_promiscuity(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 30u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_num_tx_queues(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 31u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_num_rx_queues(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 32u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_carrier(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 33u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_phys_port_id(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 34u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_carrier_changes(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 35u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_phys_switch_id(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 36u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_link_netnsid(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 37u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_phys_port_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            38u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_phys_port_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 38u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_proto_down(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 39u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_gso_max_segs(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 40u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_gso_max_size(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 41u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 42u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn nested_xdp(mut self) -> PushXdpAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 43u16);
        PushXdpAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_event(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 44u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_new_netnsid(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 45u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_target_netnsid(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 46u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_carrier_up_count(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 47u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_carrier_down_count(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 48u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_new_ifindex(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 49u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_min_mtu(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 50u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_mtu(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 51u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_prop_list(mut self) -> PushPropListLinkAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 52u16);
        PushPropListLinkAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_alt_ifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            53u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_alt_ifname_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 53u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_perm_address(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 54u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_proto_down_reason(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            55u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_proto_down_reason_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 55u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_parent_dev_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            56u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_parent_dev_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 56u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_parent_dev_bus_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            57u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_parent_dev_bus_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 57u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_gro_max_size(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 58u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tso_max_size(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 59u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tso_max_segs(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 60u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_allmulti(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 61u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_devlink_port(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 62u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_gso_ipv4_max_size(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 63u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_gro_ipv4_max_size(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 64u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_dpll_pin(mut self) -> PushLinkDpllPinAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 65u16);
        PushLinkDpllPinAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "EDT offload horizon supported by the device (in nsec).\n"]
    pub fn push_max_pacing_offload_horizon(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 66u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_netns_immutable(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 67u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_headroom(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 68u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tailroom(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 69u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushLinkAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPropListLinkAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushPropListLinkAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushPropListLinkAttrs<Prev> {
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
    pub fn push_alt_ifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            53u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn push_alt_ifname_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 53u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
}
impl<Prev: Pusher> Drop for PushPropListLinkAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushAfSpecAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushAfSpecAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushAfSpecAttrs<Prev> {
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
    pub fn nested_inet(mut self) -> PushIflaAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushIflaAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_inet6(mut self) -> PushIfla6Attrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 10u16);
        PushIfla6Attrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_mctp(mut self) -> PushMctpAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 45u16);
        PushMctpAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushAfSpecAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushVfinfoListAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushVfinfoListAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushVfinfoListAttrs<Prev> {
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
    pub fn nested_info(mut self) -> PushVfinfoAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushVfinfoAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushVfinfoListAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushVfinfoAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushVfinfoAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushVfinfoAttrs<Prev> {
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
    pub fn push_mac(mut self, value: IflaVfMac) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_vlan(mut self, value: IflaVfVlan) -> Self {
        push_header(self.as_vec_mut(), 2u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_tx_rate(mut self, value: IflaVfTxRate) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_spoofchk(mut self, value: IflaVfSpoofchk) -> Self {
        push_header(self.as_vec_mut(), 4u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_link_state(mut self, value: IflaVfLinkState) -> Self {
        push_header(self.as_vec_mut(), 5u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_rate(mut self, value: IflaVfRate) -> Self {
        push_header(self.as_vec_mut(), 6u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_rss_query_en(mut self, value: IflaVfRssQueryEn) -> Self {
        push_header(self.as_vec_mut(), 7u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn nested_stats(mut self) -> PushVfStatsAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 8u16);
        PushVfStatsAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_trust(mut self, value: IflaVfTrust) -> Self {
        push_header(self.as_vec_mut(), 9u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_ib_node_guid(mut self, value: IflaVfGuid) -> Self {
        push_header(self.as_vec_mut(), 10u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_ib_port_guid(mut self, value: IflaVfGuid) -> Self {
        push_header(self.as_vec_mut(), 11u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn nested_vlan_list(mut self) -> PushVfVlanAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 12u16);
        PushVfVlanAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_broadcast(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 13u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushVfinfoAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushVfStatsAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushVfStatsAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushVfStatsAttrs<Prev> {
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
    pub fn push_rx_packets(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 0u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tx_packets(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 1u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_rx_bytes(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 2u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tx_bytes(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 3u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_broadcast(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 4u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_multicast(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 5u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 6u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_rx_dropped(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 7u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tx_dropped(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 8u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushVfStatsAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushVfVlanAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushVfVlanAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushVfVlanAttrs<Prev> {
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
    pub fn push_info(mut self, value: IflaVfVlanInfo) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
}
impl<Prev: Pusher> Drop for PushVfVlanAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushVfPortsAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushVfPortsAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushVfPortsAttrs<Prev> {
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
}
impl<Prev: Pusher> Drop for PushVfPortsAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPortSelfAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushPortSelfAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushPortSelfAttrs<Prev> {
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
}
impl<Prev: Pusher> Drop for PushPortSelfAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLinkinfoAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLinkinfoAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLinkinfoAttrs<Prev> {
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
    pub fn push_kind(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_kind_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Selector attribute `kind` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_bond(mut self) -> PushLinkinfoBondAttrs<PushDummy<Prev>> {
        self = self.push_kind(c"bond");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushLinkinfoBondAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `kind` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_bridge(mut self) -> PushLinkinfoBridgeAttrs<PushDummy<Prev>> {
        self = self.push_kind(c"bridge");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushLinkinfoBridgeAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `kind` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_erspan(mut self) -> PushLinkinfoGreAttrs<PushDummy<Prev>> {
        self = self.push_kind(c"erspan");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushLinkinfoGreAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `kind` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_gre(mut self) -> PushLinkinfoGreAttrs<PushDummy<Prev>> {
        self = self.push_kind(c"gre");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushLinkinfoGreAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `kind` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_gretap(mut self) -> PushLinkinfoGreAttrs<PushDummy<Prev>> {
        self = self.push_kind(c"gretap");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushLinkinfoGreAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `kind` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_ip6gre(mut self) -> PushLinkinfoGre6Attrs<PushDummy<Prev>> {
        self = self.push_kind(c"ip6gre");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushLinkinfoGre6Attrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `kind` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_geneve(mut self) -> PushLinkinfoGeneveAttrs<PushDummy<Prev>> {
        self = self.push_kind(c"geneve");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushLinkinfoGeneveAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `kind` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_hsr(mut self) -> PushLinkinfoHsrAttrs<PushDummy<Prev>> {
        self = self.push_kind(c"hsr");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushLinkinfoHsrAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `kind` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_ipip(mut self) -> PushLinkinfoIptunAttrs<PushDummy<Prev>> {
        self = self.push_kind(c"ipip");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushLinkinfoIptunAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `kind` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_ip6tnl(mut self) -> PushLinkinfoIp6tnlAttrs<PushDummy<Prev>> {
        self = self.push_kind(c"ip6tnl");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushLinkinfoIp6tnlAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `kind` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_sit(mut self) -> PushLinkinfoIptunAttrs<PushDummy<Prev>> {
        self = self.push_kind(c"sit");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushLinkinfoIptunAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `kind` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_tun(mut self) -> PushLinkinfoTunAttrs<PushDummy<Prev>> {
        self = self.push_kind(c"tun");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushLinkinfoTunAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `kind` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_vlan(mut self) -> PushLinkinfoVlanAttrs<PushDummy<Prev>> {
        self = self.push_kind(c"vlan");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushLinkinfoVlanAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `kind` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_vrf(mut self) -> PushLinkinfoVrfAttrs<PushDummy<Prev>> {
        self = self.push_kind(c"vrf");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushLinkinfoVrfAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `kind` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_vti(mut self) -> PushLinkinfoVtiAttrs<PushDummy<Prev>> {
        self = self.push_kind(c"vti");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushLinkinfoVtiAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `kind` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_vti6(mut self) -> PushLinkinfoVti6Attrs<PushDummy<Prev>> {
        self = self.push_kind(c"vti6");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushLinkinfoVti6Attrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `kind` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_netkit(mut self) -> PushLinkinfoNetkitAttrs<PushDummy<Prev>> {
        self = self.push_kind(c"netkit");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushLinkinfoNetkitAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `kind` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_ovpn(mut self) -> PushLinkinfoOvpnAttrs<PushDummy<Prev>> {
        self = self.push_kind(c"ovpn");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushLinkinfoOvpnAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    pub fn push_xstats(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_slave_kind(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            4u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_slave_kind_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 4u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Selector attribute `slave-kind` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_slave_data_bridge(mut self) -> PushLinkinfoBrportAttrs<PushDummy<Prev>> {
        self = self.push_slave_kind(c"bridge");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 5u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushLinkinfoBrportAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `slave-kind` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_slave_data_bond(mut self) -> PushBondSlaveAttrs<PushDummy<Prev>> {
        self = self.push_slave_kind(c"bond");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 5u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushBondSlaveAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushLinkinfoAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLinkinfoBondAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLinkinfoBondAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
pub struct PushArrayU32<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
    pub(crate) counter: u16,
}
impl<Prev: Pusher> Pusher for PushArrayU32<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushArrayU32<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
            counter: 0,
        }
    }
    pub fn end_array(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_vec_mut(), *header_offset);
        }
        prev
    }
    pub fn entry(mut self, value: std::net::Ipv4Addr) -> Self {
        let index = self.counter;
        self.counter += 1;
        push_header(self.as_vec_mut(), index, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushArrayU32<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushArrayBinary<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
    pub(crate) counter: u16,
}
impl<Prev: Pusher> Pusher for PushArrayBinary<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushArrayBinary<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
            counter: 0,
        }
    }
    pub fn end_array(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_vec_mut(), *header_offset);
        }
        prev
    }
    pub fn entry(mut self, value: &[u8]) -> Self {
        let index = self.counter;
        self.counter += 1;
        push_header(self.as_vec_mut(), index, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushArrayBinary<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
impl<Prev: Pusher> PushLinkinfoBondAttrs<Prev> {
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
    pub fn push_active_slave(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_miimon(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_updelay(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_downdelay(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_use_carrier(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 6u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_arp_interval(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn array_arp_ip_target(mut self) -> PushArrayU32<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 8u16);
        PushArrayU32 {
            prev: Some(self),
            header_offset: Some(header_offset),
            counter: 0,
        }
    }
    pub fn push_arp_validate(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_arp_all_targets(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_primary(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 11u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_primary_reselect(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 12u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fail_over_mac(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 13u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_xmit_hash_policy(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 14u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_resend_igmp(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 15u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_num_peer_notif(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 16u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_all_slaves_active(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 17u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_min_links(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 18u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_lp_interval(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 19u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_packets_per_slave(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 20u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ad_lacp_rate(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 21u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ad_select(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 22u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_ad_info(mut self) -> PushBondAdInfoAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 23u16);
        PushBondAdInfoAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_ad_actor_sys_prio(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 24u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ad_user_port_key(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 25u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ad_actor_system(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 26u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_tlb_dynamic_lb(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 27u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_peer_notif_delay(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 28u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ad_lacp_active(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 29u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_missed_max(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 30u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn array_ns_ip6_target(mut self) -> PushArrayBinary<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 31u16);
        PushArrayBinary {
            prev: Some(self),
            header_offset: Some(header_offset),
            counter: 0,
        }
    }
    pub fn push_coupled_control(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 32u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_broadcast_neigh(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 33u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_lacp_strict(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 34u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushLinkinfoBondAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushBondAdInfoAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushBondAdInfoAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushBondAdInfoAttrs<Prev> {
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
    pub fn push_aggregator(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 1u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_num_ports(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 2u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_actor_key(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 3u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_partner_key(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 4u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_partner_mac(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 5u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushBondAdInfoAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushBondSlaveAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushBondSlaveAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushBondSlaveAttrs<Prev> {
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
    pub fn push_state(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 1u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mii_status(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 2u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_link_failure_count(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_perm_hwaddr(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 4u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_queue_id(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 5u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ad_aggregator_id(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 6u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ad_actor_oper_port_state(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 7u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ad_partner_oper_port_state(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 8u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_prio(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushBondSlaveAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLinkinfoBridgeAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLinkinfoBridgeAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLinkinfoBridgeAttrs<Prev> {
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
    pub fn push_forward_delay(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_hello_time(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_age(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ageing_time(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_stp_state(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_priority(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 6u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_vlan_filtering(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 7u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_vlan_protocol(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 8u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_group_fwd_mask(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 9u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_root_id(mut self, value: IflaBridgeId) -> Self {
        push_header(self.as_vec_mut(), 10u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_bridge_id(mut self, value: IflaBridgeId) -> Self {
        push_header(self.as_vec_mut(), 11u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_root_port(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 12u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_root_path_cost(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 13u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_topology_change(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 14u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_topology_change_detected(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 15u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_hello_timer(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 16u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tcn_timer(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 17u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_topology_change_timer(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 18u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_gc_timer(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 19u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_group_addr(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 20u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_fdb_flush(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 21u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_mcast_router(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 22u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mcast_snooping(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 23u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mcast_query_use_ifaddr(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 24u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mcast_querier(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 25u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mcast_hash_elasticity(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 26u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mcast_hash_max(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 27u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mcast_last_member_cnt(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 28u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mcast_startup_query_cnt(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 29u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mcast_last_member_intvl(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 30u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mcast_membership_intvl(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 31u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mcast_querier_intvl(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 32u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mcast_query_intvl(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 33u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mcast_query_response_intvl(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 34u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mcast_startup_query_intvl(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 35u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_nf_call_iptables(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 36u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_nf_call_ip6tables(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 37u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_nf_call_arptables(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 38u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_vlan_default_pvid(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 39u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 40u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_vlan_stats_enabled(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 41u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mcast_stats_enabled(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 42u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mcast_igmp_version(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 43u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mcast_mld_version(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 44u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_vlan_stats_per_port(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 45u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_multi_boolopt(mut self, value: BrBooloptMulti) -> Self {
        push_header(self.as_vec_mut(), 46u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_mcast_querier_state(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 47u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_fdb_n_learned(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 48u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fdb_max_learned(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 49u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`BrStpMode`] (enum)"]
    pub fn push_stp_mode(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 50u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushLinkinfoBridgeAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLinkinfoBrportAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLinkinfoBrportAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLinkinfoBrportAttrs<Prev> {
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
    pub fn push_state(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 1u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_priority(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 2u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_cost(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mode(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 4u16, 0 as u16);
        self
    }
    pub fn push_guard(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 5u16, 0 as u16);
        self
    }
    pub fn push_protect(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 6u16, 0 as u16);
        self
    }
    pub fn push_fast_leave(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 7u16, 0 as u16);
        self
    }
    pub fn push_learning(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 8u16, 0 as u16);
        self
    }
    pub fn push_unicast_flood(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 9u16, 0 as u16);
        self
    }
    pub fn push_proxyarp(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 10u16, 0 as u16);
        self
    }
    pub fn push_learning_sync(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 11u16, 0 as u16);
        self
    }
    pub fn push_proxyarp_wifi(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 12u16, 0 as u16);
        self
    }
    pub fn push_root_id(mut self, value: IflaBridgeId) -> Self {
        push_header(self.as_vec_mut(), 13u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_bridge_id(mut self, value: IflaBridgeId) -> Self {
        push_header(self.as_vec_mut(), 14u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_designated_port(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 15u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_designated_cost(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 16u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_id(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 17u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_no(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 18u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_topology_change_ack(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 19u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_config_pending(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 20u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_message_age_timer(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 21u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_forward_delay_timer(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 22u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_hold_timer(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 23u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flush(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 24u16, 0 as u16);
        self
    }
    pub fn push_multicast_router(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 25u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 26u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_mcast_flood(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 27u16, 0 as u16);
        self
    }
    pub fn push_mcast_to_ucast(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 28u16, 0 as u16);
        self
    }
    pub fn push_vlan_tunnel(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 29u16, 0 as u16);
        self
    }
    pub fn push_bcast_flood(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 30u16, 0 as u16);
        self
    }
    pub fn push_group_fwd_mask(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 31u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_neigh_suppress(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 32u16, 0 as u16);
        self
    }
    pub fn push_isolated(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 33u16, 0 as u16);
        self
    }
    pub fn push_backup_port(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 34u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mrp_ring_open(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 35u16, 0 as u16);
        self
    }
    pub fn push_mrp_in_open(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 36u16, 0 as u16);
        self
    }
    pub fn push_mcast_eht_hosts_limit(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 37u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mcast_eht_hosts_cnt(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 38u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_locked(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 39u16, 0 as u16);
        self
    }
    pub fn push_mab(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 40u16, 0 as u16);
        self
    }
    pub fn push_mcast_n_groups(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 41u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mcast_max_groups(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 42u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_neigh_vlan_suppress(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 43u16, 0 as u16);
        self
    }
    pub fn push_backup_nhid(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 44u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_neigh_forward_grat(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 45u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushLinkinfoBrportAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLinkinfoGreAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLinkinfoGreAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLinkinfoGreAttrs<Prev> {
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
    pub fn push_link(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_iflags(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 2u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_oflags(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 3u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_ikey(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_okey(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_local(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_vec_mut(), 6u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_vec_mut(), value);
        self
    }
    pub fn push_remote(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_vec_mut(), 7u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_vec_mut(), value);
        self
    }
    pub fn push_ttl(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 8u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tos(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 9u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pmtudisc(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 10u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap_limit(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 11u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flowinfo(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 12u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 13u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap_type(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 14u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap_flags(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 15u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap_sport(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 16u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_encap_dport(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 17u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_collect_metadata(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 18u16, 0 as u16);
        self
    }
    pub fn push_ignore_df(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 19u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fwmark(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 20u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_erspan_index(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 21u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_erspan_ver(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 22u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_erspan_dir(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 23u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_erspan_hwid(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 24u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushLinkinfoGreAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLinkinfoGre6Attrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLinkinfoGre6Attrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLinkinfoGre6Attrs<Prev> {
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
    pub fn push_link(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_iflags(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 2u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_oflags(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 3u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_ikey(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_okey(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_local(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 6u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_remote(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 7u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_ttl(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 8u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap_limit(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 11u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flowinfo(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 12u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 13u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap_type(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 14u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap_flags(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 15u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap_sport(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 16u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_encap_dport(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 17u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_collect_metadata(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 18u16, 0 as u16);
        self
    }
    pub fn push_fwmark(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 20u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_erspan_index(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 21u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_erspan_ver(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 22u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_erspan_dir(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 23u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_erspan_hwid(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 24u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushLinkinfoGre6Attrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLinkinfoVtiAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLinkinfoVtiAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLinkinfoVtiAttrs<Prev> {
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
    pub fn push_link(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ikey(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_okey(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_local(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_vec_mut(), 4u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_vec_mut(), value);
        self
    }
    pub fn push_remote(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_vec_mut(), 5u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_vec_mut(), value);
        self
    }
    pub fn push_fwmark(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushLinkinfoVtiAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLinkinfoVti6Attrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLinkinfoVti6Attrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLinkinfoVti6Attrs<Prev> {
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
    pub fn push_link(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ikey(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_okey(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_local(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 4u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_remote(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 5u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_fwmark(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushLinkinfoVti6Attrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLinkinfoGeneveAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLinkinfoGeneveAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLinkinfoGeneveAttrs<Prev> {
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
    pub fn push_remote(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    pub fn push_ttl(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 3u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tos(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 4u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_port(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 5u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_collect_metadata(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 6u16, 0 as u16);
        self
    }
    pub fn push_remote6(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 7u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_udp_csum(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 8u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_udp_zero_csum6_tx(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 9u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_udp_zero_csum6_rx(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 10u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_label(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 11u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_ttl_inherit(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 12u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_df(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 13u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_inner_proto_inherit(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 14u16, 0 as u16);
        self
    }
    pub fn push_port_range(mut self, value: IflaGenevePortRange) -> Self {
        push_header(self.as_vec_mut(), 15u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_gro_hint(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 16u16, 0 as u16);
        self
    }
    pub fn push_local(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_vec_mut(), 17u16, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    pub fn push_local6(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 18u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushLinkinfoGeneveAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLinkinfoHsrAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLinkinfoHsrAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLinkinfoHsrAttrs<Prev> {
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
    pub fn push_slave1(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_slave2(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_multicast_spec(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 3u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_supervision_addr(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 4u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_seq_nr(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 5u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_version(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 6u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_protocol(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 7u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_interlink(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushLinkinfoHsrAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLinkinfoIptunAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLinkinfoIptunAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLinkinfoIptunAttrs<Prev> {
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
    pub fn push_link(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_local(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_vec_mut(), 2u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_vec_mut(), value);
        self
    }
    pub fn push_remote(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_vec_mut(), 3u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_vec_mut(), value);
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
    pub fn push_encap_limit(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 6u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flowinfo(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_flags(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 8u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_proto(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 9u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pmtudisc(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 10u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_6rd_prefix(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 11u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_6rd_relay_prefix(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_vec_mut(), 12u16, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    pub fn push_6rd_prefixlen(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 13u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_6rd_relay_prefixlen(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 14u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap_type(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 15u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap_flags(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 16u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap_sport(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 17u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_encap_dport(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 18u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_collect_metadata(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 19u16, 0 as u16);
        self
    }
    pub fn push_fwmark(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 20u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushLinkinfoIptunAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLinkinfoIp6tnlAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLinkinfoIp6tnlAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLinkinfoIp6tnlAttrs<Prev> {
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
    pub fn push_link(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_local(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_remote(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_ttl(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 4u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap_limit(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 6u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flowinfo(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_proto(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 9u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap_type(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 15u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap_flags(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 16u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_encap_sport(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 17u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_encap_dport(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 18u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_collect_metadata(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 19u16, 0 as u16);
        self
    }
    pub fn push_fwmark(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 20u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushLinkinfoIp6tnlAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLinkinfoTunAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLinkinfoTunAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLinkinfoTunAttrs<Prev> {
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
    pub fn push_owner(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_group(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_type(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 3u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pi(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 4u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_vnet_hdr(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 5u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_persist(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 6u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_multi_queue(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 7u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_num_queues(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_num_disabled_queues(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushLinkinfoTunAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLinkinfoVlanAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLinkinfoVlanAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLinkinfoVlanAttrs<Prev> {
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
    pub fn push_id(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 1u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flags(mut self, value: IflaVlanFlags) -> Self {
        push_header(self.as_vec_mut(), 2u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn nested_egress_qos(mut self) -> PushIflaVlanQos<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 3u16);
        PushIflaVlanQos {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_ingress_qos(mut self) -> PushIflaVlanQos<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 4u16);
        PushIflaVlanQos {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Associated type: [`VlanProtocols`] (enum)"]
    pub fn push_protocol(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 5u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushLinkinfoVlanAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushIflaVlanQos<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushIflaVlanQos<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushIflaVlanQos<Prev> {
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
    pub fn push_mapping(mut self, value: IflaVlanQosMapping) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
}
impl<Prev: Pusher> Drop for PushIflaVlanQos<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLinkinfoVrfAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLinkinfoVrfAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLinkinfoVrfAttrs<Prev> {
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
    pub fn push_table(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushLinkinfoVrfAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushXdpAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushXdpAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushXdpAttrs<Prev> {
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
    pub fn push_fd(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_attached(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 2u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_prog_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_drv_prog_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_skb_prog_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_hw_prog_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_expected_fd(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushXdpAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushIflaAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushIflaAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushIflaAttrs<Prev> {
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
    #[doc = "u32 indexed by ipv4-devconf - 1 on output, on input it\\'s a nest\n"]
    pub fn push_conf(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushIflaAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushIfla6Attrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushIfla6Attrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushIfla6Attrs<Prev> {
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
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "u32 indexed by ipv6-devconf - 1 on output, on input it\\'s a nest\n"]
    pub fn push_conf(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_stats(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_mcast(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 4u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_cacheinfo(mut self, value: IflaCacheinfo) -> Self {
        push_header(self.as_vec_mut(), 5u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_icmp6stats(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 6u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_token(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 7u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_addr_gen_mode(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 8u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ra_mtu(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushIfla6Attrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushMctpAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushMctpAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushMctpAttrs<Prev> {
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
    pub fn push_net(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_phys_binding(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 2u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushMctpAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushStatsAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushStatsAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushStatsAttrs<Prev> {
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
    pub fn push_link_64(mut self, value: RtnlLinkStats64) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_link_xstats(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_link_xstats_slave(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn nested_link_offload_xstats(mut self) -> PushLinkOffloadXstats<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 4u16);
        PushLinkOffloadXstats {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_af_spec(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 5u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushStatsAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLinkOffloadXstats<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLinkOffloadXstats<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
pub struct PushArrayHwSInfoOne<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
    pub(crate) counter: u16,
}
impl<Prev: Pusher> Pusher for PushArrayHwSInfoOne<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushArrayHwSInfoOne<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
            counter: 0,
        }
    }
    pub fn end_array(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_vec_mut(), *header_offset);
        }
        prev
    }
    pub fn entry_nested(mut self) -> PushHwSInfoOne<Self> {
        let index = self.counter;
        self.counter += 1;
        let header_offset = push_nested_header(self.as_vec_mut(), index);
        PushHwSInfoOne {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushArrayHwSInfoOne<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
impl<Prev: Pusher> PushLinkOffloadXstats<Prev> {
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
    pub fn push_cpu_hit(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn array_hw_s_info(mut self) -> PushArrayHwSInfoOne<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushArrayHwSInfoOne {
            prev: Some(self),
            header_offset: Some(header_offset),
            counter: 0,
        }
    }
    pub fn push_l3_stats(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushLinkOffloadXstats<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushHwSInfoOne<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushHwSInfoOne<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushHwSInfoOne<Prev> {
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
    pub fn push_request(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 1u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_used(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 2u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushHwSInfoOne<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLinkDpllPinAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLinkDpllPinAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLinkDpllPinAttrs<Prev> {
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
}
impl<Prev: Pusher> Drop for PushLinkDpllPinAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLinkinfoNetkitAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLinkinfoNetkitAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLinkinfoNetkitAttrs<Prev> {
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
    pub fn push_peer_info(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_primary(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 2u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`NetkitPolicy`] (enum)"]
    pub fn push_policy(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`NetkitPolicy`] (enum)"]
    pub fn push_peer_policy(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`NetkitMode`] (enum)"]
    pub fn push_mode(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`NetkitScrub`] (enum)"]
    pub fn push_scrub(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`NetkitScrub`] (enum)"]
    pub fn push_peer_scrub(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_headroom(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 8u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tailroom(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 9u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`NetkitPairing`] (enum)"]
    pub fn push_pairing(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushLinkinfoNetkitAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLinkinfoOvpnAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLinkinfoOvpnAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLinkinfoOvpnAttrs<Prev> {
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
    #[doc = "Associated type: [`OvpnMode`] (enum)"]
    pub fn push_mode(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 1u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushLinkinfoOvpnAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Create a new link.\n\nRequest attributes:\n- [.push_address()](PushLinkAttrs::push_address)\n- [.push_broadcast()](PushLinkAttrs::push_broadcast)\n- [.push_ifname()](PushLinkAttrs::push_ifname)\n- [.push_mtu()](PushLinkAttrs::push_mtu)\n- [.push_txqlen()](PushLinkAttrs::push_txqlen)\n- [.push_operstate()](PushLinkAttrs::push_operstate)\n- [.push_linkmode()](PushLinkAttrs::push_linkmode)\n- [.nested_linkinfo()](PushLinkAttrs::nested_linkinfo)\n- [.push_net_ns_pid()](PushLinkAttrs::push_net_ns_pid)\n- [.nested_af_spec()](PushLinkAttrs::nested_af_spec)\n- [.push_group()](PushLinkAttrs::push_group)\n- [.push_net_ns_fd()](PushLinkAttrs::push_net_ns_fd)\n- [.push_num_tx_queues()](PushLinkAttrs::push_num_tx_queues)\n- [.push_num_rx_queues()](PushLinkAttrs::push_num_rx_queues)\n- [.push_link_netnsid()](PushLinkAttrs::push_link_netnsid)\n- [.push_gso_max_segs()](PushLinkAttrs::push_gso_max_segs)\n- [.push_gso_max_size()](PushLinkAttrs::push_gso_max_size)\n- [.push_target_netnsid()](PushLinkAttrs::push_target_netnsid)\n- [.push_gro_max_size()](PushLinkAttrs::push_gro_max_size)\n- [.push_gso_ipv4_max_size()](PushLinkAttrs::push_gso_ipv4_max_size)\n- [.push_gro_ipv4_max_size()](PushLinkAttrs::push_gro_ipv4_max_size)\n\n"]
#[derive(Debug)]
pub struct OpNewlinkDo<'r> {
    request: Request<'r>,
}
impl<'r> OpNewlinkDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Ifinfomsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Ifinfomsg,
    ) -> PushLinkAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushLinkAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushLinkAttrs<&mut Vec<u8>> {
        PushLinkAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushLinkAttrs<RequestBuf<'r>> {
        PushLinkAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Ifinfomsg, IterableLinkAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Ifinfomsg::len()));
        (
            Ifinfomsg::new_from_slice(header).unwrap_or_default(),
            IterableLinkAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Ifinfomsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Ifinfomsg {
        let pos = self.request.pos;
        Ifinfomsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Ifinfomsg {
        let pos = self.request.pos;
        Ifinfomsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpNewlinkDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 16u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Ifinfomsg, IterableLinkAttrs<'buf>);
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
#[doc = "Delete an existing link.\n\nRequest attributes:\n- [.push_ifname()](PushLinkAttrs::push_ifname)\n\n"]
#[derive(Debug)]
pub struct OpDellinkDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDellinkDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Ifinfomsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Ifinfomsg,
    ) -> PushLinkAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushLinkAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushLinkAttrs<&mut Vec<u8>> {
        PushLinkAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushLinkAttrs<RequestBuf<'r>> {
        PushLinkAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Ifinfomsg, IterableLinkAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Ifinfomsg::len()));
        (
            Ifinfomsg::new_from_slice(header).unwrap_or_default(),
            IterableLinkAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Ifinfomsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Ifinfomsg {
        let pos = self.request.pos;
        Ifinfomsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Ifinfomsg {
        let pos = self.request.pos;
        Ifinfomsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpDellinkDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 17u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Ifinfomsg, IterableLinkAttrs<'buf>);
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
#[doc = "Get / dump information about a link.\n\nRequest attributes:\n- [.push_master()](PushLinkAttrs::push_master)\n- [.nested_linkinfo()](PushLinkAttrs::nested_linkinfo)\n- [.push_ext_mask()](PushLinkAttrs::push_ext_mask)\n- [.push_target_netnsid()](PushLinkAttrs::push_target_netnsid)\n\nReply attributes:\n- [.get_address()](IterableLinkAttrs::get_address)\n- [.get_broadcast()](IterableLinkAttrs::get_broadcast)\n- [.get_ifname()](IterableLinkAttrs::get_ifname)\n- [.get_mtu()](IterableLinkAttrs::get_mtu)\n- [.get_link()](IterableLinkAttrs::get_link)\n- [.get_qdisc()](IterableLinkAttrs::get_qdisc)\n- [.get_stats()](IterableLinkAttrs::get_stats)\n- [.get_master()](IterableLinkAttrs::get_master)\n- [.get_wireless()](IterableLinkAttrs::get_wireless)\n- [.get_protinfo()](IterableLinkAttrs::get_protinfo)\n- [.get_txqlen()](IterableLinkAttrs::get_txqlen)\n- [.get_map()](IterableLinkAttrs::get_map)\n- [.get_weight()](IterableLinkAttrs::get_weight)\n- [.get_operstate()](IterableLinkAttrs::get_operstate)\n- [.get_linkmode()](IterableLinkAttrs::get_linkmode)\n- [.get_linkinfo()](IterableLinkAttrs::get_linkinfo)\n- [.get_net_ns_pid()](IterableLinkAttrs::get_net_ns_pid)\n- [.get_ifalias()](IterableLinkAttrs::get_ifalias)\n- [.get_num_vf()](IterableLinkAttrs::get_num_vf)\n- [.get_vfinfo_list()](IterableLinkAttrs::get_vfinfo_list)\n- [.get_stats64()](IterableLinkAttrs::get_stats64)\n- [.get_vf_ports()](IterableLinkAttrs::get_vf_ports)\n- [.get_port_self()](IterableLinkAttrs::get_port_self)\n- [.get_af_spec()](IterableLinkAttrs::get_af_spec)\n- [.get_group()](IterableLinkAttrs::get_group)\n- [.get_net_ns_fd()](IterableLinkAttrs::get_net_ns_fd)\n- [.get_ext_mask()](IterableLinkAttrs::get_ext_mask)\n- [.get_promiscuity()](IterableLinkAttrs::get_promiscuity)\n- [.get_num_tx_queues()](IterableLinkAttrs::get_num_tx_queues)\n- [.get_num_rx_queues()](IterableLinkAttrs::get_num_rx_queues)\n- [.get_carrier()](IterableLinkAttrs::get_carrier)\n- [.get_phys_port_id()](IterableLinkAttrs::get_phys_port_id)\n- [.get_carrier_changes()](IterableLinkAttrs::get_carrier_changes)\n- [.get_phys_switch_id()](IterableLinkAttrs::get_phys_switch_id)\n- [.get_link_netnsid()](IterableLinkAttrs::get_link_netnsid)\n- [.get_phys_port_name()](IterableLinkAttrs::get_phys_port_name)\n- [.get_proto_down()](IterableLinkAttrs::get_proto_down)\n- [.get_gso_max_segs()](IterableLinkAttrs::get_gso_max_segs)\n- [.get_gso_max_size()](IterableLinkAttrs::get_gso_max_size)\n- [.get_xdp()](IterableLinkAttrs::get_xdp)\n- [.get_event()](IterableLinkAttrs::get_event)\n- [.get_new_netnsid()](IterableLinkAttrs::get_new_netnsid)\n- [.get_target_netnsid()](IterableLinkAttrs::get_target_netnsid)\n- [.get_carrier_up_count()](IterableLinkAttrs::get_carrier_up_count)\n- [.get_carrier_down_count()](IterableLinkAttrs::get_carrier_down_count)\n- [.get_new_ifindex()](IterableLinkAttrs::get_new_ifindex)\n- [.get_min_mtu()](IterableLinkAttrs::get_min_mtu)\n- [.get_max_mtu()](IterableLinkAttrs::get_max_mtu)\n- [.get_prop_list()](IterableLinkAttrs::get_prop_list)\n- [.get_perm_address()](IterableLinkAttrs::get_perm_address)\n- [.get_proto_down_reason()](IterableLinkAttrs::get_proto_down_reason)\n- [.get_parent_dev_name()](IterableLinkAttrs::get_parent_dev_name)\n- [.get_parent_dev_bus_name()](IterableLinkAttrs::get_parent_dev_bus_name)\n- [.get_gro_max_size()](IterableLinkAttrs::get_gro_max_size)\n- [.get_tso_max_size()](IterableLinkAttrs::get_tso_max_size)\n- [.get_tso_max_segs()](IterableLinkAttrs::get_tso_max_segs)\n- [.get_allmulti()](IterableLinkAttrs::get_allmulti)\n- [.get_devlink_port()](IterableLinkAttrs::get_devlink_port)\n- [.get_gso_ipv4_max_size()](IterableLinkAttrs::get_gso_ipv4_max_size)\n- [.get_gro_ipv4_max_size()](IterableLinkAttrs::get_gro_ipv4_max_size)\n\n"]
#[derive(Debug)]
pub struct OpGetlinkDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetlinkDump<'r> {
    pub fn new(mut request: Request<'r>, header: &Ifinfomsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Ifinfomsg,
    ) -> PushLinkAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushLinkAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushLinkAttrs<&mut Vec<u8>> {
        PushLinkAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushLinkAttrs<RequestBuf<'r>> {
        PushLinkAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Ifinfomsg, IterableLinkAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Ifinfomsg::len()));
        (
            Ifinfomsg::new_from_slice(header).unwrap_or_default(),
            IterableLinkAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Ifinfomsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Ifinfomsg {
        let pos = self.request.pos;
        Ifinfomsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Ifinfomsg {
        let pos = self.request.pos;
        Ifinfomsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetlinkDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 18u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Ifinfomsg, IterableLinkAttrs<'buf>);
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
#[doc = "Get / dump information about a link.\n\nRequest attributes:\n- [.push_ifname()](PushLinkAttrs::push_ifname)\n- [.push_ext_mask()](PushLinkAttrs::push_ext_mask)\n- [.push_target_netnsid()](PushLinkAttrs::push_target_netnsid)\n- [.push_alt_ifname()](PushLinkAttrs::push_alt_ifname)\n\nReply attributes:\n- [.get_address()](IterableLinkAttrs::get_address)\n- [.get_broadcast()](IterableLinkAttrs::get_broadcast)\n- [.get_ifname()](IterableLinkAttrs::get_ifname)\n- [.get_mtu()](IterableLinkAttrs::get_mtu)\n- [.get_link()](IterableLinkAttrs::get_link)\n- [.get_qdisc()](IterableLinkAttrs::get_qdisc)\n- [.get_stats()](IterableLinkAttrs::get_stats)\n- [.get_master()](IterableLinkAttrs::get_master)\n- [.get_wireless()](IterableLinkAttrs::get_wireless)\n- [.get_protinfo()](IterableLinkAttrs::get_protinfo)\n- [.get_txqlen()](IterableLinkAttrs::get_txqlen)\n- [.get_map()](IterableLinkAttrs::get_map)\n- [.get_weight()](IterableLinkAttrs::get_weight)\n- [.get_operstate()](IterableLinkAttrs::get_operstate)\n- [.get_linkmode()](IterableLinkAttrs::get_linkmode)\n- [.get_linkinfo()](IterableLinkAttrs::get_linkinfo)\n- [.get_net_ns_pid()](IterableLinkAttrs::get_net_ns_pid)\n- [.get_ifalias()](IterableLinkAttrs::get_ifalias)\n- [.get_num_vf()](IterableLinkAttrs::get_num_vf)\n- [.get_vfinfo_list()](IterableLinkAttrs::get_vfinfo_list)\n- [.get_stats64()](IterableLinkAttrs::get_stats64)\n- [.get_vf_ports()](IterableLinkAttrs::get_vf_ports)\n- [.get_port_self()](IterableLinkAttrs::get_port_self)\n- [.get_af_spec()](IterableLinkAttrs::get_af_spec)\n- [.get_group()](IterableLinkAttrs::get_group)\n- [.get_net_ns_fd()](IterableLinkAttrs::get_net_ns_fd)\n- [.get_ext_mask()](IterableLinkAttrs::get_ext_mask)\n- [.get_promiscuity()](IterableLinkAttrs::get_promiscuity)\n- [.get_num_tx_queues()](IterableLinkAttrs::get_num_tx_queues)\n- [.get_num_rx_queues()](IterableLinkAttrs::get_num_rx_queues)\n- [.get_carrier()](IterableLinkAttrs::get_carrier)\n- [.get_phys_port_id()](IterableLinkAttrs::get_phys_port_id)\n- [.get_carrier_changes()](IterableLinkAttrs::get_carrier_changes)\n- [.get_phys_switch_id()](IterableLinkAttrs::get_phys_switch_id)\n- [.get_link_netnsid()](IterableLinkAttrs::get_link_netnsid)\n- [.get_phys_port_name()](IterableLinkAttrs::get_phys_port_name)\n- [.get_proto_down()](IterableLinkAttrs::get_proto_down)\n- [.get_gso_max_segs()](IterableLinkAttrs::get_gso_max_segs)\n- [.get_gso_max_size()](IterableLinkAttrs::get_gso_max_size)\n- [.get_xdp()](IterableLinkAttrs::get_xdp)\n- [.get_event()](IterableLinkAttrs::get_event)\n- [.get_new_netnsid()](IterableLinkAttrs::get_new_netnsid)\n- [.get_target_netnsid()](IterableLinkAttrs::get_target_netnsid)\n- [.get_carrier_up_count()](IterableLinkAttrs::get_carrier_up_count)\n- [.get_carrier_down_count()](IterableLinkAttrs::get_carrier_down_count)\n- [.get_new_ifindex()](IterableLinkAttrs::get_new_ifindex)\n- [.get_min_mtu()](IterableLinkAttrs::get_min_mtu)\n- [.get_max_mtu()](IterableLinkAttrs::get_max_mtu)\n- [.get_prop_list()](IterableLinkAttrs::get_prop_list)\n- [.get_perm_address()](IterableLinkAttrs::get_perm_address)\n- [.get_proto_down_reason()](IterableLinkAttrs::get_proto_down_reason)\n- [.get_parent_dev_name()](IterableLinkAttrs::get_parent_dev_name)\n- [.get_parent_dev_bus_name()](IterableLinkAttrs::get_parent_dev_bus_name)\n- [.get_gro_max_size()](IterableLinkAttrs::get_gro_max_size)\n- [.get_tso_max_size()](IterableLinkAttrs::get_tso_max_size)\n- [.get_tso_max_segs()](IterableLinkAttrs::get_tso_max_segs)\n- [.get_allmulti()](IterableLinkAttrs::get_allmulti)\n- [.get_devlink_port()](IterableLinkAttrs::get_devlink_port)\n- [.get_gso_ipv4_max_size()](IterableLinkAttrs::get_gso_ipv4_max_size)\n- [.get_gro_ipv4_max_size()](IterableLinkAttrs::get_gro_ipv4_max_size)\n\n"]
#[derive(Debug)]
pub struct OpGetlinkDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetlinkDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Ifinfomsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Ifinfomsg,
    ) -> PushLinkAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushLinkAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushLinkAttrs<&mut Vec<u8>> {
        PushLinkAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushLinkAttrs<RequestBuf<'r>> {
        PushLinkAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Ifinfomsg, IterableLinkAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Ifinfomsg::len()));
        (
            Ifinfomsg::new_from_slice(header).unwrap_or_default(),
            IterableLinkAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Ifinfomsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Ifinfomsg {
        let pos = self.request.pos;
        Ifinfomsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Ifinfomsg {
        let pos = self.request.pos;
        Ifinfomsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetlinkDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 18u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Ifinfomsg, IterableLinkAttrs<'buf>);
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
#[doc = "Set information about a link.\n\nRequest attributes:\n- [.push_address()](PushLinkAttrs::push_address)\n- [.push_broadcast()](PushLinkAttrs::push_broadcast)\n- [.push_ifname()](PushLinkAttrs::push_ifname)\n- [.push_mtu()](PushLinkAttrs::push_mtu)\n- [.push_link()](PushLinkAttrs::push_link)\n- [.push_qdisc()](PushLinkAttrs::push_qdisc)\n- [.push_stats()](PushLinkAttrs::push_stats)\n- [.push_master()](PushLinkAttrs::push_master)\n- [.push_wireless()](PushLinkAttrs::push_wireless)\n- [.push_protinfo()](PushLinkAttrs::push_protinfo)\n- [.push_txqlen()](PushLinkAttrs::push_txqlen)\n- [.push_map()](PushLinkAttrs::push_map)\n- [.push_weight()](PushLinkAttrs::push_weight)\n- [.push_operstate()](PushLinkAttrs::push_operstate)\n- [.push_linkmode()](PushLinkAttrs::push_linkmode)\n- [.nested_linkinfo()](PushLinkAttrs::nested_linkinfo)\n- [.push_net_ns_pid()](PushLinkAttrs::push_net_ns_pid)\n- [.push_ifalias()](PushLinkAttrs::push_ifalias)\n- [.push_num_vf()](PushLinkAttrs::push_num_vf)\n- [.nested_vfinfo_list()](PushLinkAttrs::nested_vfinfo_list)\n- [.push_stats64()](PushLinkAttrs::push_stats64)\n- [.nested_vf_ports()](PushLinkAttrs::nested_vf_ports)\n- [.nested_port_self()](PushLinkAttrs::nested_port_self)\n- [.nested_af_spec()](PushLinkAttrs::nested_af_spec)\n- [.push_group()](PushLinkAttrs::push_group)\n- [.push_net_ns_fd()](PushLinkAttrs::push_net_ns_fd)\n- [.push_ext_mask()](PushLinkAttrs::push_ext_mask)\n- [.push_promiscuity()](PushLinkAttrs::push_promiscuity)\n- [.push_num_tx_queues()](PushLinkAttrs::push_num_tx_queues)\n- [.push_num_rx_queues()](PushLinkAttrs::push_num_rx_queues)\n- [.push_carrier()](PushLinkAttrs::push_carrier)\n- [.push_phys_port_id()](PushLinkAttrs::push_phys_port_id)\n- [.push_carrier_changes()](PushLinkAttrs::push_carrier_changes)\n- [.push_phys_switch_id()](PushLinkAttrs::push_phys_switch_id)\n- [.push_link_netnsid()](PushLinkAttrs::push_link_netnsid)\n- [.push_phys_port_name()](PushLinkAttrs::push_phys_port_name)\n- [.push_proto_down()](PushLinkAttrs::push_proto_down)\n- [.push_gso_max_segs()](PushLinkAttrs::push_gso_max_segs)\n- [.push_gso_max_size()](PushLinkAttrs::push_gso_max_size)\n- [.nested_xdp()](PushLinkAttrs::nested_xdp)\n- [.push_event()](PushLinkAttrs::push_event)\n- [.push_new_netnsid()](PushLinkAttrs::push_new_netnsid)\n- [.push_target_netnsid()](PushLinkAttrs::push_target_netnsid)\n- [.push_carrier_up_count()](PushLinkAttrs::push_carrier_up_count)\n- [.push_carrier_down_count()](PushLinkAttrs::push_carrier_down_count)\n- [.push_new_ifindex()](PushLinkAttrs::push_new_ifindex)\n- [.push_min_mtu()](PushLinkAttrs::push_min_mtu)\n- [.push_max_mtu()](PushLinkAttrs::push_max_mtu)\n- [.nested_prop_list()](PushLinkAttrs::nested_prop_list)\n- [.push_perm_address()](PushLinkAttrs::push_perm_address)\n- [.push_proto_down_reason()](PushLinkAttrs::push_proto_down_reason)\n- [.push_parent_dev_name()](PushLinkAttrs::push_parent_dev_name)\n- [.push_parent_dev_bus_name()](PushLinkAttrs::push_parent_dev_bus_name)\n- [.push_gro_max_size()](PushLinkAttrs::push_gro_max_size)\n- [.push_tso_max_size()](PushLinkAttrs::push_tso_max_size)\n- [.push_tso_max_segs()](PushLinkAttrs::push_tso_max_segs)\n- [.push_allmulti()](PushLinkAttrs::push_allmulti)\n- [.push_devlink_port()](PushLinkAttrs::push_devlink_port)\n- [.push_gso_ipv4_max_size()](PushLinkAttrs::push_gso_ipv4_max_size)\n- [.push_gro_ipv4_max_size()](PushLinkAttrs::push_gro_ipv4_max_size)\n\n"]
#[derive(Debug)]
pub struct OpSetlinkDo<'r> {
    request: Request<'r>,
}
impl<'r> OpSetlinkDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Ifinfomsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Ifinfomsg,
    ) -> PushLinkAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushLinkAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushLinkAttrs<&mut Vec<u8>> {
        PushLinkAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushLinkAttrs<RequestBuf<'r>> {
        PushLinkAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Ifinfomsg, IterableLinkAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Ifinfomsg::len()));
        (
            Ifinfomsg::new_from_slice(header).unwrap_or_default(),
            IterableLinkAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Ifinfomsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Ifinfomsg {
        let pos = self.request.pos;
        Ifinfomsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Ifinfomsg {
        let pos = self.request.pos;
        Ifinfomsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpSetlinkDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 19u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Ifinfomsg, IterableLinkAttrs<'buf>);
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
#[doc = "Get / dump link stats.\n\nReply attributes:\n- [.get_link_64()](IterableStatsAttrs::get_link_64)\n- [.get_link_xstats()](IterableStatsAttrs::get_link_xstats)\n- [.get_link_xstats_slave()](IterableStatsAttrs::get_link_xstats_slave)\n- [.get_link_offload_xstats()](IterableStatsAttrs::get_link_offload_xstats)\n- [.get_af_spec()](IterableStatsAttrs::get_af_spec)\n\n"]
#[derive(Debug)]
pub struct OpGetstatsDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetstatsDump<'r> {
    pub fn new(mut request: Request<'r>, header: &IfStatsMsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &IfStatsMsg,
    ) -> PushStatsAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushStatsAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushStatsAttrs<&mut Vec<u8>> {
        PushStatsAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushStatsAttrs<RequestBuf<'r>> {
        PushStatsAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (IfStatsMsg, IterableStatsAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(IfStatsMsg::len()));
        (
            IfStatsMsg::new_from_slice(header).unwrap_or_default(),
            IterableStatsAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &IfStatsMsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &IfStatsMsg {
        let pos = self.request.pos;
        IfStatsMsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut IfStatsMsg {
        let pos = self.request.pos;
        IfStatsMsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetstatsDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 94u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (IfStatsMsg, IterableStatsAttrs<'buf>);
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
#[doc = "Get / dump link stats.\n\nReply attributes:\n- [.get_link_64()](IterableStatsAttrs::get_link_64)\n- [.get_link_xstats()](IterableStatsAttrs::get_link_xstats)\n- [.get_link_xstats_slave()](IterableStatsAttrs::get_link_xstats_slave)\n- [.get_link_offload_xstats()](IterableStatsAttrs::get_link_offload_xstats)\n- [.get_af_spec()](IterableStatsAttrs::get_af_spec)\n\n"]
#[derive(Debug)]
pub struct OpGetstatsDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetstatsDo<'r> {
    pub fn new(mut request: Request<'r>, header: &IfStatsMsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &IfStatsMsg,
    ) -> PushStatsAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushStatsAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushStatsAttrs<&mut Vec<u8>> {
        PushStatsAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushStatsAttrs<RequestBuf<'r>> {
        PushStatsAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (IfStatsMsg, IterableStatsAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(IfStatsMsg::len()));
        (
            IfStatsMsg::new_from_slice(header).unwrap_or_default(),
            IterableStatsAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &IfStatsMsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &IfStatsMsg {
        let pos = self.request.pos;
        IfStatsMsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut IfStatsMsg {
        let pos = self.request.pos;
        IfStatsMsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetstatsDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 94u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (IfStatsMsg, IterableStatsAttrs<'buf>);
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
    #[doc = "Create a new link.\n\nRequest attributes:\n- [.push_address()](PushLinkAttrs::push_address)\n- [.push_broadcast()](PushLinkAttrs::push_broadcast)\n- [.push_ifname()](PushLinkAttrs::push_ifname)\n- [.push_mtu()](PushLinkAttrs::push_mtu)\n- [.push_txqlen()](PushLinkAttrs::push_txqlen)\n- [.push_operstate()](PushLinkAttrs::push_operstate)\n- [.push_linkmode()](PushLinkAttrs::push_linkmode)\n- [.nested_linkinfo()](PushLinkAttrs::nested_linkinfo)\n- [.push_net_ns_pid()](PushLinkAttrs::push_net_ns_pid)\n- [.nested_af_spec()](PushLinkAttrs::nested_af_spec)\n- [.push_group()](PushLinkAttrs::push_group)\n- [.push_net_ns_fd()](PushLinkAttrs::push_net_ns_fd)\n- [.push_num_tx_queues()](PushLinkAttrs::push_num_tx_queues)\n- [.push_num_rx_queues()](PushLinkAttrs::push_num_rx_queues)\n- [.push_link_netnsid()](PushLinkAttrs::push_link_netnsid)\n- [.push_gso_max_segs()](PushLinkAttrs::push_gso_max_segs)\n- [.push_gso_max_size()](PushLinkAttrs::push_gso_max_size)\n- [.push_target_netnsid()](PushLinkAttrs::push_target_netnsid)\n- [.push_gro_max_size()](PushLinkAttrs::push_gro_max_size)\n- [.push_gso_ipv4_max_size()](PushLinkAttrs::push_gso_ipv4_max_size)\n- [.push_gro_ipv4_max_size()](PushLinkAttrs::push_gro_ipv4_max_size)\n\n"]
    pub fn op_newlink_do(self, header: &Ifinfomsg) -> OpNewlinkDo<'buf> {
        let mut res = OpNewlinkDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-newlink-do", OpNewlinkDo::lookup);
        res
    }
    #[doc = "Delete an existing link.\n\nRequest attributes:\n- [.push_ifname()](PushLinkAttrs::push_ifname)\n\n"]
    pub fn op_dellink_do(self, header: &Ifinfomsg) -> OpDellinkDo<'buf> {
        let mut res = OpDellinkDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-dellink-do", OpDellinkDo::lookup);
        res
    }
    #[doc = "Get / dump information about a link.\n\nRequest attributes:\n- [.push_master()](PushLinkAttrs::push_master)\n- [.nested_linkinfo()](PushLinkAttrs::nested_linkinfo)\n- [.push_ext_mask()](PushLinkAttrs::push_ext_mask)\n- [.push_target_netnsid()](PushLinkAttrs::push_target_netnsid)\n\nReply attributes:\n- [.get_address()](IterableLinkAttrs::get_address)\n- [.get_broadcast()](IterableLinkAttrs::get_broadcast)\n- [.get_ifname()](IterableLinkAttrs::get_ifname)\n- [.get_mtu()](IterableLinkAttrs::get_mtu)\n- [.get_link()](IterableLinkAttrs::get_link)\n- [.get_qdisc()](IterableLinkAttrs::get_qdisc)\n- [.get_stats()](IterableLinkAttrs::get_stats)\n- [.get_master()](IterableLinkAttrs::get_master)\n- [.get_wireless()](IterableLinkAttrs::get_wireless)\n- [.get_protinfo()](IterableLinkAttrs::get_protinfo)\n- [.get_txqlen()](IterableLinkAttrs::get_txqlen)\n- [.get_map()](IterableLinkAttrs::get_map)\n- [.get_weight()](IterableLinkAttrs::get_weight)\n- [.get_operstate()](IterableLinkAttrs::get_operstate)\n- [.get_linkmode()](IterableLinkAttrs::get_linkmode)\n- [.get_linkinfo()](IterableLinkAttrs::get_linkinfo)\n- [.get_net_ns_pid()](IterableLinkAttrs::get_net_ns_pid)\n- [.get_ifalias()](IterableLinkAttrs::get_ifalias)\n- [.get_num_vf()](IterableLinkAttrs::get_num_vf)\n- [.get_vfinfo_list()](IterableLinkAttrs::get_vfinfo_list)\n- [.get_stats64()](IterableLinkAttrs::get_stats64)\n- [.get_vf_ports()](IterableLinkAttrs::get_vf_ports)\n- [.get_port_self()](IterableLinkAttrs::get_port_self)\n- [.get_af_spec()](IterableLinkAttrs::get_af_spec)\n- [.get_group()](IterableLinkAttrs::get_group)\n- [.get_net_ns_fd()](IterableLinkAttrs::get_net_ns_fd)\n- [.get_ext_mask()](IterableLinkAttrs::get_ext_mask)\n- [.get_promiscuity()](IterableLinkAttrs::get_promiscuity)\n- [.get_num_tx_queues()](IterableLinkAttrs::get_num_tx_queues)\n- [.get_num_rx_queues()](IterableLinkAttrs::get_num_rx_queues)\n- [.get_carrier()](IterableLinkAttrs::get_carrier)\n- [.get_phys_port_id()](IterableLinkAttrs::get_phys_port_id)\n- [.get_carrier_changes()](IterableLinkAttrs::get_carrier_changes)\n- [.get_phys_switch_id()](IterableLinkAttrs::get_phys_switch_id)\n- [.get_link_netnsid()](IterableLinkAttrs::get_link_netnsid)\n- [.get_phys_port_name()](IterableLinkAttrs::get_phys_port_name)\n- [.get_proto_down()](IterableLinkAttrs::get_proto_down)\n- [.get_gso_max_segs()](IterableLinkAttrs::get_gso_max_segs)\n- [.get_gso_max_size()](IterableLinkAttrs::get_gso_max_size)\n- [.get_xdp()](IterableLinkAttrs::get_xdp)\n- [.get_event()](IterableLinkAttrs::get_event)\n- [.get_new_netnsid()](IterableLinkAttrs::get_new_netnsid)\n- [.get_target_netnsid()](IterableLinkAttrs::get_target_netnsid)\n- [.get_carrier_up_count()](IterableLinkAttrs::get_carrier_up_count)\n- [.get_carrier_down_count()](IterableLinkAttrs::get_carrier_down_count)\n- [.get_new_ifindex()](IterableLinkAttrs::get_new_ifindex)\n- [.get_min_mtu()](IterableLinkAttrs::get_min_mtu)\n- [.get_max_mtu()](IterableLinkAttrs::get_max_mtu)\n- [.get_prop_list()](IterableLinkAttrs::get_prop_list)\n- [.get_perm_address()](IterableLinkAttrs::get_perm_address)\n- [.get_proto_down_reason()](IterableLinkAttrs::get_proto_down_reason)\n- [.get_parent_dev_name()](IterableLinkAttrs::get_parent_dev_name)\n- [.get_parent_dev_bus_name()](IterableLinkAttrs::get_parent_dev_bus_name)\n- [.get_gro_max_size()](IterableLinkAttrs::get_gro_max_size)\n- [.get_tso_max_size()](IterableLinkAttrs::get_tso_max_size)\n- [.get_tso_max_segs()](IterableLinkAttrs::get_tso_max_segs)\n- [.get_allmulti()](IterableLinkAttrs::get_allmulti)\n- [.get_devlink_port()](IterableLinkAttrs::get_devlink_port)\n- [.get_gso_ipv4_max_size()](IterableLinkAttrs::get_gso_ipv4_max_size)\n- [.get_gro_ipv4_max_size()](IterableLinkAttrs::get_gro_ipv4_max_size)\n\n"]
    pub fn op_getlink_dump(self, header: &Ifinfomsg) -> OpGetlinkDump<'buf> {
        let mut res = OpGetlinkDump::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getlink-dump", OpGetlinkDump::lookup);
        res
    }
    #[doc = "Get / dump information about a link.\n\nRequest attributes:\n- [.push_ifname()](PushLinkAttrs::push_ifname)\n- [.push_ext_mask()](PushLinkAttrs::push_ext_mask)\n- [.push_target_netnsid()](PushLinkAttrs::push_target_netnsid)\n- [.push_alt_ifname()](PushLinkAttrs::push_alt_ifname)\n\nReply attributes:\n- [.get_address()](IterableLinkAttrs::get_address)\n- [.get_broadcast()](IterableLinkAttrs::get_broadcast)\n- [.get_ifname()](IterableLinkAttrs::get_ifname)\n- [.get_mtu()](IterableLinkAttrs::get_mtu)\n- [.get_link()](IterableLinkAttrs::get_link)\n- [.get_qdisc()](IterableLinkAttrs::get_qdisc)\n- [.get_stats()](IterableLinkAttrs::get_stats)\n- [.get_master()](IterableLinkAttrs::get_master)\n- [.get_wireless()](IterableLinkAttrs::get_wireless)\n- [.get_protinfo()](IterableLinkAttrs::get_protinfo)\n- [.get_txqlen()](IterableLinkAttrs::get_txqlen)\n- [.get_map()](IterableLinkAttrs::get_map)\n- [.get_weight()](IterableLinkAttrs::get_weight)\n- [.get_operstate()](IterableLinkAttrs::get_operstate)\n- [.get_linkmode()](IterableLinkAttrs::get_linkmode)\n- [.get_linkinfo()](IterableLinkAttrs::get_linkinfo)\n- [.get_net_ns_pid()](IterableLinkAttrs::get_net_ns_pid)\n- [.get_ifalias()](IterableLinkAttrs::get_ifalias)\n- [.get_num_vf()](IterableLinkAttrs::get_num_vf)\n- [.get_vfinfo_list()](IterableLinkAttrs::get_vfinfo_list)\n- [.get_stats64()](IterableLinkAttrs::get_stats64)\n- [.get_vf_ports()](IterableLinkAttrs::get_vf_ports)\n- [.get_port_self()](IterableLinkAttrs::get_port_self)\n- [.get_af_spec()](IterableLinkAttrs::get_af_spec)\n- [.get_group()](IterableLinkAttrs::get_group)\n- [.get_net_ns_fd()](IterableLinkAttrs::get_net_ns_fd)\n- [.get_ext_mask()](IterableLinkAttrs::get_ext_mask)\n- [.get_promiscuity()](IterableLinkAttrs::get_promiscuity)\n- [.get_num_tx_queues()](IterableLinkAttrs::get_num_tx_queues)\n- [.get_num_rx_queues()](IterableLinkAttrs::get_num_rx_queues)\n- [.get_carrier()](IterableLinkAttrs::get_carrier)\n- [.get_phys_port_id()](IterableLinkAttrs::get_phys_port_id)\n- [.get_carrier_changes()](IterableLinkAttrs::get_carrier_changes)\n- [.get_phys_switch_id()](IterableLinkAttrs::get_phys_switch_id)\n- [.get_link_netnsid()](IterableLinkAttrs::get_link_netnsid)\n- [.get_phys_port_name()](IterableLinkAttrs::get_phys_port_name)\n- [.get_proto_down()](IterableLinkAttrs::get_proto_down)\n- [.get_gso_max_segs()](IterableLinkAttrs::get_gso_max_segs)\n- [.get_gso_max_size()](IterableLinkAttrs::get_gso_max_size)\n- [.get_xdp()](IterableLinkAttrs::get_xdp)\n- [.get_event()](IterableLinkAttrs::get_event)\n- [.get_new_netnsid()](IterableLinkAttrs::get_new_netnsid)\n- [.get_target_netnsid()](IterableLinkAttrs::get_target_netnsid)\n- [.get_carrier_up_count()](IterableLinkAttrs::get_carrier_up_count)\n- [.get_carrier_down_count()](IterableLinkAttrs::get_carrier_down_count)\n- [.get_new_ifindex()](IterableLinkAttrs::get_new_ifindex)\n- [.get_min_mtu()](IterableLinkAttrs::get_min_mtu)\n- [.get_max_mtu()](IterableLinkAttrs::get_max_mtu)\n- [.get_prop_list()](IterableLinkAttrs::get_prop_list)\n- [.get_perm_address()](IterableLinkAttrs::get_perm_address)\n- [.get_proto_down_reason()](IterableLinkAttrs::get_proto_down_reason)\n- [.get_parent_dev_name()](IterableLinkAttrs::get_parent_dev_name)\n- [.get_parent_dev_bus_name()](IterableLinkAttrs::get_parent_dev_bus_name)\n- [.get_gro_max_size()](IterableLinkAttrs::get_gro_max_size)\n- [.get_tso_max_size()](IterableLinkAttrs::get_tso_max_size)\n- [.get_tso_max_segs()](IterableLinkAttrs::get_tso_max_segs)\n- [.get_allmulti()](IterableLinkAttrs::get_allmulti)\n- [.get_devlink_port()](IterableLinkAttrs::get_devlink_port)\n- [.get_gso_ipv4_max_size()](IterableLinkAttrs::get_gso_ipv4_max_size)\n- [.get_gro_ipv4_max_size()](IterableLinkAttrs::get_gro_ipv4_max_size)\n\n"]
    pub fn op_getlink_do(self, header: &Ifinfomsg) -> OpGetlinkDo<'buf> {
        let mut res = OpGetlinkDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getlink-do", OpGetlinkDo::lookup);
        res
    }
    #[doc = "Set information about a link.\n\nRequest attributes:\n- [.push_address()](PushLinkAttrs::push_address)\n- [.push_broadcast()](PushLinkAttrs::push_broadcast)\n- [.push_ifname()](PushLinkAttrs::push_ifname)\n- [.push_mtu()](PushLinkAttrs::push_mtu)\n- [.push_link()](PushLinkAttrs::push_link)\n- [.push_qdisc()](PushLinkAttrs::push_qdisc)\n- [.push_stats()](PushLinkAttrs::push_stats)\n- [.push_master()](PushLinkAttrs::push_master)\n- [.push_wireless()](PushLinkAttrs::push_wireless)\n- [.push_protinfo()](PushLinkAttrs::push_protinfo)\n- [.push_txqlen()](PushLinkAttrs::push_txqlen)\n- [.push_map()](PushLinkAttrs::push_map)\n- [.push_weight()](PushLinkAttrs::push_weight)\n- [.push_operstate()](PushLinkAttrs::push_operstate)\n- [.push_linkmode()](PushLinkAttrs::push_linkmode)\n- [.nested_linkinfo()](PushLinkAttrs::nested_linkinfo)\n- [.push_net_ns_pid()](PushLinkAttrs::push_net_ns_pid)\n- [.push_ifalias()](PushLinkAttrs::push_ifalias)\n- [.push_num_vf()](PushLinkAttrs::push_num_vf)\n- [.nested_vfinfo_list()](PushLinkAttrs::nested_vfinfo_list)\n- [.push_stats64()](PushLinkAttrs::push_stats64)\n- [.nested_vf_ports()](PushLinkAttrs::nested_vf_ports)\n- [.nested_port_self()](PushLinkAttrs::nested_port_self)\n- [.nested_af_spec()](PushLinkAttrs::nested_af_spec)\n- [.push_group()](PushLinkAttrs::push_group)\n- [.push_net_ns_fd()](PushLinkAttrs::push_net_ns_fd)\n- [.push_ext_mask()](PushLinkAttrs::push_ext_mask)\n- [.push_promiscuity()](PushLinkAttrs::push_promiscuity)\n- [.push_num_tx_queues()](PushLinkAttrs::push_num_tx_queues)\n- [.push_num_rx_queues()](PushLinkAttrs::push_num_rx_queues)\n- [.push_carrier()](PushLinkAttrs::push_carrier)\n- [.push_phys_port_id()](PushLinkAttrs::push_phys_port_id)\n- [.push_carrier_changes()](PushLinkAttrs::push_carrier_changes)\n- [.push_phys_switch_id()](PushLinkAttrs::push_phys_switch_id)\n- [.push_link_netnsid()](PushLinkAttrs::push_link_netnsid)\n- [.push_phys_port_name()](PushLinkAttrs::push_phys_port_name)\n- [.push_proto_down()](PushLinkAttrs::push_proto_down)\n- [.push_gso_max_segs()](PushLinkAttrs::push_gso_max_segs)\n- [.push_gso_max_size()](PushLinkAttrs::push_gso_max_size)\n- [.nested_xdp()](PushLinkAttrs::nested_xdp)\n- [.push_event()](PushLinkAttrs::push_event)\n- [.push_new_netnsid()](PushLinkAttrs::push_new_netnsid)\n- [.push_target_netnsid()](PushLinkAttrs::push_target_netnsid)\n- [.push_carrier_up_count()](PushLinkAttrs::push_carrier_up_count)\n- [.push_carrier_down_count()](PushLinkAttrs::push_carrier_down_count)\n- [.push_new_ifindex()](PushLinkAttrs::push_new_ifindex)\n- [.push_min_mtu()](PushLinkAttrs::push_min_mtu)\n- [.push_max_mtu()](PushLinkAttrs::push_max_mtu)\n- [.nested_prop_list()](PushLinkAttrs::nested_prop_list)\n- [.push_perm_address()](PushLinkAttrs::push_perm_address)\n- [.push_proto_down_reason()](PushLinkAttrs::push_proto_down_reason)\n- [.push_parent_dev_name()](PushLinkAttrs::push_parent_dev_name)\n- [.push_parent_dev_bus_name()](PushLinkAttrs::push_parent_dev_bus_name)\n- [.push_gro_max_size()](PushLinkAttrs::push_gro_max_size)\n- [.push_tso_max_size()](PushLinkAttrs::push_tso_max_size)\n- [.push_tso_max_segs()](PushLinkAttrs::push_tso_max_segs)\n- [.push_allmulti()](PushLinkAttrs::push_allmulti)\n- [.push_devlink_port()](PushLinkAttrs::push_devlink_port)\n- [.push_gso_ipv4_max_size()](PushLinkAttrs::push_gso_ipv4_max_size)\n- [.push_gro_ipv4_max_size()](PushLinkAttrs::push_gro_ipv4_max_size)\n\n"]
    pub fn op_setlink_do(self, header: &Ifinfomsg) -> OpSetlinkDo<'buf> {
        let mut res = OpSetlinkDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-setlink-do", OpSetlinkDo::lookup);
        res
    }
    #[doc = "Get / dump link stats.\n\nReply attributes:\n- [.get_link_64()](IterableStatsAttrs::get_link_64)\n- [.get_link_xstats()](IterableStatsAttrs::get_link_xstats)\n- [.get_link_xstats_slave()](IterableStatsAttrs::get_link_xstats_slave)\n- [.get_link_offload_xstats()](IterableStatsAttrs::get_link_offload_xstats)\n- [.get_af_spec()](IterableStatsAttrs::get_af_spec)\n\n"]
    pub fn op_getstats_dump(self, header: &IfStatsMsg) -> OpGetstatsDump<'buf> {
        let mut res = OpGetstatsDump::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getstats-dump", OpGetstatsDump::lookup);
        res
    }
    #[doc = "Get / dump link stats.\n\nReply attributes:\n- [.get_link_64()](IterableStatsAttrs::get_link_64)\n- [.get_link_xstats()](IterableStatsAttrs::get_link_xstats)\n- [.get_link_xstats_slave()](IterableStatsAttrs::get_link_xstats_slave)\n- [.get_link_offload_xstats()](IterableStatsAttrs::get_link_offload_xstats)\n- [.get_af_spec()](IterableStatsAttrs::get_af_spec)\n\n"]
    pub fn op_getstats_do(self, header: &IfStatsMsg) -> OpGetstatsDo<'buf> {
        let mut res = OpGetstatsDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getstats-do", OpGetstatsDo::lookup);
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableLinkAttrs::get_address;
        let _ = IterableLinkAttrs::get_af_spec;
        let _ = IterableLinkAttrs::get_allmulti;
        let _ = IterableLinkAttrs::get_broadcast;
        let _ = IterableLinkAttrs::get_carrier;
        let _ = IterableLinkAttrs::get_carrier_changes;
        let _ = IterableLinkAttrs::get_carrier_down_count;
        let _ = IterableLinkAttrs::get_carrier_up_count;
        let _ = IterableLinkAttrs::get_devlink_port;
        let _ = IterableLinkAttrs::get_event;
        let _ = IterableLinkAttrs::get_ext_mask;
        let _ = IterableLinkAttrs::get_gro_ipv4_max_size;
        let _ = IterableLinkAttrs::get_gro_max_size;
        let _ = IterableLinkAttrs::get_group;
        let _ = IterableLinkAttrs::get_gso_ipv4_max_size;
        let _ = IterableLinkAttrs::get_gso_max_segs;
        let _ = IterableLinkAttrs::get_gso_max_size;
        let _ = IterableLinkAttrs::get_ifalias;
        let _ = IterableLinkAttrs::get_ifname;
        let _ = IterableLinkAttrs::get_link;
        let _ = IterableLinkAttrs::get_link_netnsid;
        let _ = IterableLinkAttrs::get_linkinfo;
        let _ = IterableLinkAttrs::get_linkmode;
        let _ = IterableLinkAttrs::get_map;
        let _ = IterableLinkAttrs::get_master;
        let _ = IterableLinkAttrs::get_max_mtu;
        let _ = IterableLinkAttrs::get_min_mtu;
        let _ = IterableLinkAttrs::get_mtu;
        let _ = IterableLinkAttrs::get_net_ns_fd;
        let _ = IterableLinkAttrs::get_net_ns_pid;
        let _ = IterableLinkAttrs::get_new_ifindex;
        let _ = IterableLinkAttrs::get_new_netnsid;
        let _ = IterableLinkAttrs::get_num_rx_queues;
        let _ = IterableLinkAttrs::get_num_tx_queues;
        let _ = IterableLinkAttrs::get_num_vf;
        let _ = IterableLinkAttrs::get_operstate;
        let _ = IterableLinkAttrs::get_parent_dev_bus_name;
        let _ = IterableLinkAttrs::get_parent_dev_name;
        let _ = IterableLinkAttrs::get_perm_address;
        let _ = IterableLinkAttrs::get_phys_port_id;
        let _ = IterableLinkAttrs::get_phys_port_name;
        let _ = IterableLinkAttrs::get_phys_switch_id;
        let _ = IterableLinkAttrs::get_port_self;
        let _ = IterableLinkAttrs::get_promiscuity;
        let _ = IterableLinkAttrs::get_prop_list;
        let _ = IterableLinkAttrs::get_protinfo;
        let _ = IterableLinkAttrs::get_proto_down;
        let _ = IterableLinkAttrs::get_proto_down_reason;
        let _ = IterableLinkAttrs::get_qdisc;
        let _ = IterableLinkAttrs::get_stats64;
        let _ = IterableLinkAttrs::get_stats;
        let _ = IterableLinkAttrs::get_target_netnsid;
        let _ = IterableLinkAttrs::get_tso_max_segs;
        let _ = IterableLinkAttrs::get_tso_max_size;
        let _ = IterableLinkAttrs::get_txqlen;
        let _ = IterableLinkAttrs::get_vf_ports;
        let _ = IterableLinkAttrs::get_vfinfo_list;
        let _ = IterableLinkAttrs::get_weight;
        let _ = IterableLinkAttrs::get_wireless;
        let _ = IterableLinkAttrs::get_xdp;
        let _ = IterableStatsAttrs::get_af_spec;
        let _ = IterableStatsAttrs::get_link_64;
        let _ = IterableStatsAttrs::get_link_offload_xstats;
        let _ = IterableStatsAttrs::get_link_xstats;
        let _ = IterableStatsAttrs::get_link_xstats_slave;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::nested_af_spec;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::nested_linkinfo;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::nested_port_self;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::nested_prop_list;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::nested_vf_ports;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::nested_vfinfo_list;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::nested_xdp;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_address;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_allmulti;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_alt_ifname;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_broadcast;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_carrier;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_carrier_changes;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_carrier_down_count;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_carrier_up_count;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_devlink_port;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_event;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_ext_mask;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_gro_ipv4_max_size;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_gro_max_size;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_group;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_gso_ipv4_max_size;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_gso_max_segs;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_gso_max_size;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_ifalias;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_ifname;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_link;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_link_netnsid;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_linkmode;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_map;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_master;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_max_mtu;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_min_mtu;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_mtu;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_net_ns_fd;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_net_ns_pid;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_new_ifindex;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_new_netnsid;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_num_rx_queues;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_num_tx_queues;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_num_vf;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_operstate;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_parent_dev_bus_name;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_parent_dev_name;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_perm_address;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_phys_port_id;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_phys_port_name;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_phys_switch_id;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_promiscuity;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_protinfo;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_proto_down;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_proto_down_reason;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_qdisc;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_stats64;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_stats;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_target_netnsid;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_tso_max_segs;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_tso_max_size;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_txqlen;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_weight;
        let _ = PushLinkAttrs::<&mut Vec<u8>>::push_wireless;
    }
}
