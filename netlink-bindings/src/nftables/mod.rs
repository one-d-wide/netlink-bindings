#![doc = "Netfilter nftables configuration over netlink.\n"]
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
pub const PROTONAME: &str = "nftables";
pub const PROTONAME_CSTR: &CStr = c"nftables";
pub const PROTONUM: u16 = 12u16;
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum MetaKeys {
    Len = 0,
    Protocol = 1,
    Priority = 2,
    Mark = 3,
    Iif = 4,
    Oif = 5,
    Iifname = 6,
    Oifname = 7,
    Iftype = 8,
    Oiftype = 9,
    Skuid = 10,
    Skgid = 11,
    Nftrace = 12,
    Rtclassid = 13,
    Secmark = 14,
    Nfproto = 15,
    L4Proto = 16,
    BriIifname = 17,
    BriOifname = 18,
    Pkttype = 19,
    Cpu = 20,
    Iifgroup = 21,
    Oifgroup = 22,
    Cgroup = 23,
    Prandom = 24,
    Secpath = 25,
    Iifkind = 26,
    Oifkind = 27,
    BriIifpvid = 28,
    BriIifvproto = 29,
    TimeNs = 30,
    TimeDay = 31,
    TimeHour = 32,
    Sdif = 33,
    Sdifname = 34,
    BriBroute = 35,
}
impl MetaKeys {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Len,
            1 => Self::Protocol,
            2 => Self::Priority,
            3 => Self::Mark,
            4 => Self::Iif,
            5 => Self::Oif,
            6 => Self::Iifname,
            7 => Self::Oifname,
            8 => Self::Iftype,
            9 => Self::Oiftype,
            10 => Self::Skuid,
            11 => Self::Skgid,
            12 => Self::Nftrace,
            13 => Self::Rtclassid,
            14 => Self::Secmark,
            15 => Self::Nfproto,
            16 => Self::L4Proto,
            17 => Self::BriIifname,
            18 => Self::BriOifname,
            19 => Self::Pkttype,
            20 => Self::Cpu,
            21 => Self::Iifgroup,
            22 => Self::Oifgroup,
            23 => Self::Cgroup,
            24 => Self::Prandom,
            25 => Self::Secpath,
            26 => Self::Iifkind,
            27 => Self::Oifkind,
            28 => Self::BriIifpvid,
            29 => Self::BriIifvproto,
            30 => Self::TimeNs,
            31 => Self::TimeDay,
            32 => Self::TimeHour,
            33 => Self::Sdif,
            34 => Self::Sdifname,
            35 => Self::BriBroute,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum BitwiseOps {
    #[doc = "mask-and-xor operation used to implement NOT, AND, OR and XOR boolean\noperations\n"]
    MaskXor = 0,
    Lshift = 1,
    Rshift = 2,
    And = 3,
    Or = 4,
    Xor = 5,
}
impl BitwiseOps {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::MaskXor,
            1 => Self::Lshift,
            2 => Self::Rshift,
            3 => Self::And,
            4 => Self::Or,
            5 => Self::Xor,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum CmpOps {
    Eq = 0,
    Neq = 1,
    Lt = 2,
    Lte = 3,
    Gt = 4,
    Gte = 5,
}
impl CmpOps {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Eq,
            1 => Self::Neq,
            2 => Self::Lt,
            3 => Self::Lte,
            4 => Self::Gt,
            5 => Self::Gte,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum ObjectType {
    Unspec = 0,
    Counter = 1,
    Quota = 2,
    CtHelper = 3,
    Limit = 4,
    Connlimit = 5,
    Tunnel = 6,
    CtTimeout = 7,
    Secmark = 8,
    CtExpect = 9,
    Synproxy = 10,
}
impl ObjectType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Unspec,
            1 => Self::Counter,
            2 => Self::Quota,
            3 => Self::CtHelper,
            4 => Self::Limit,
            5 => Self::Connlimit,
            6 => Self::Tunnel,
            7 => Self::CtTimeout,
            8 => Self::Secmark,
            9 => Self::CtExpect,
            10 => Self::Synproxy,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum NatRangeFlags {
    MapIps = 1 << 0,
    ProtoSpecified = 1 << 1,
    ProtoRandom = 1 << 2,
    Persistent = 1 << 3,
    ProtoRandomFully = 1 << 4,
    ProtoOffset = 1 << 5,
    Netmap = 1 << 6,
}
impl NatRangeFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::MapIps,
            n if n == 1 << 1 => Self::ProtoSpecified,
            n if n == 1 << 2 => Self::ProtoRandom,
            n if n == 1 << 3 => Self::Persistent,
            n if n == 1 << 4 => Self::ProtoRandomFully,
            n if n == 1 << 5 => Self::ProtoOffset,
            n if n == 1 << 6 => Self::Netmap,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum TableFlags {
    Dormant = 1 << 0,
    Owner = 1 << 1,
    Persist = 1 << 2,
}
impl TableFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::Dormant,
            n if n == 1 << 1 => Self::Owner,
            n if n == 1 << 2 => Self::Persist,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum ChainFlags {
    Base = 1 << 0,
    HwOffload = 1 << 1,
    Binding = 1 << 2,
}
impl ChainFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::Base,
            n if n == 1 << 1 => Self::HwOffload,
            n if n == 1 << 2 => Self::Binding,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum SetFlags {
    Anonymous = 1 << 0,
    Constant = 1 << 1,
    Interval = 1 << 2,
    Map = 1 << 3,
    Timeout = 1 << 4,
    Eval = 1 << 5,
    Object = 1 << 6,
    Concat = 1 << 7,
    Expr = 1 << 8,
}
impl SetFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::Anonymous,
            n if n == 1 << 1 => Self::Constant,
            n if n == 1 << 2 => Self::Interval,
            n if n == 1 << 3 => Self::Map,
            n if n == 1 << 4 => Self::Timeout,
            n if n == 1 << 5 => Self::Eval,
            n if n == 1 << 6 => Self::Object,
            n if n == 1 << 7 => Self::Concat,
            n if n == 1 << 8 => Self::Expr,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum SetElemFlags {
    IntervalEnd = 1 << 0,
    Catchall = 1 << 1,
}
impl SetElemFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::IntervalEnd,
            n if n == 1 << 1 => Self::Catchall,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum LookupFlags {
    Invert = 1 << 0,
}
impl LookupFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::Invert,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum CtKeys {
    State = 0,
    Direction = 1,
    Status = 2,
    Mark = 3,
    Secmark = 4,
    Expiration = 5,
    Helper = 6,
    L3protocol = 7,
    Src = 8,
    Dst = 9,
    Protocol = 10,
    ProtoSrc = 11,
    ProtoDst = 12,
    Labels = 13,
    Pkts = 14,
    Bytes = 15,
    Avgpkt = 16,
    Zone = 17,
    Eventmask = 18,
    SrcIp = 19,
    DstIp = 20,
    SrcIp6 = 21,
    DstIp6 = 22,
    CtId = 23,
}
impl CtKeys {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::State,
            1 => Self::Direction,
            2 => Self::Status,
            3 => Self::Mark,
            4 => Self::Secmark,
            5 => Self::Expiration,
            6 => Self::Helper,
            7 => Self::L3protocol,
            8 => Self::Src,
            9 => Self::Dst,
            10 => Self::Protocol,
            11 => Self::ProtoSrc,
            12 => Self::ProtoDst,
            13 => Self::Labels,
            14 => Self::Pkts,
            15 => Self::Bytes,
            16 => Self::Avgpkt,
            17 => Self::Zone,
            18 => Self::Eventmask,
            19 => Self::SrcIp,
            20 => Self::DstIp,
            21 => Self::SrcIp6,
            22 => Self::DstIp6,
            23 => Self::CtId,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum CtDirection {
    Original = 0,
    Reply = 1,
}
impl CtDirection {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Original,
            1 => Self::Reply,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum QuotaFlags {
    Invert = 1 << 0,
    Depleted = 1 << 1,
}
impl QuotaFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::Invert,
            n if n == 1 << 1 => Self::Depleted,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum VerdictCode {
    Continue = 4294967295,
    Break = 4294967294,
    Jump = 4294967293,
    Goto = 4294967292,
    Return = 4294967291,
    Drop = 0,
    Accept = 1,
    Stolen = 2,
    Queue = 3,
    Repeat = 4,
}
impl VerdictCode {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            4294967295 => Self::Continue,
            4294967294 => Self::Break,
            4294967293 => Self::Jump,
            4294967292 => Self::Goto,
            4294967291 => Self::Return,
            0 => Self::Drop,
            1 => Self::Accept,
            2 => Self::Stolen,
            3 => Self::Queue,
            4 => Self::Repeat,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum FibResult {
    Oif = 0,
    Oifname = 1,
    Addrtype = 2,
}
impl FibResult {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Oif,
            1 => Self::Oifname,
            2 => Self::Addrtype,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum FibFlags {
    Saddr = 1 << 0,
    Daddr = 1 << 1,
    Mark = 1 << 2,
    Iif = 1 << 3,
    Oif = 1 << 4,
    Present = 1 << 5,
}
impl FibFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::Saddr,
            n if n == 1 << 1 => Self::Daddr,
            n if n == 1 << 2 => Self::Mark,
            n if n == 1 << 3 => Self::Iif,
            n if n == 1 << 4 => Self::Oif,
            n if n == 1 << 5 => Self::Present,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum RejectTypes {
    IcmpUnreach = 0,
    TcpRst = 1,
    IcmpxUnreach = 2,
}
impl RejectTypes {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::IcmpUnreach,
            1 => Self::TcpRst,
            2 => Self::IcmpxUnreach,
            _ => return None,
        })
    }
}
#[doc = "These codes are mapped to real ICMP and ICMPv6 codes.\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum RejectInetCode {
    IcmpxNoRoute = 0,
    IcmpxPortUnreach = 1,
    IcmpxHostUnreach = 2,
    IcmpxAdminProhibited = 3,
}
impl RejectInetCode {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::IcmpxNoRoute,
            1 => Self::IcmpxPortUnreach,
            2 => Self::IcmpxHostUnreach,
            3 => Self::IcmpxAdminProhibited,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum PayloadBase {
    LinkLayerHeader = 0,
    NetworkHeader = 1,
    TransportHeader = 2,
    InnerHeader = 3,
    TunHeader = 4,
}
impl PayloadBase {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::LinkLayerHeader,
            1 => Self::NetworkHeader,
            2 => Self::TransportHeader,
            3 => Self::InnerHeader,
            4 => Self::TunHeader,
            _ => return None,
        })
    }
}
#[doc = "Range operator\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum RangeOps {
    Eq = 0,
    Neq = 1,
}
impl RangeOps {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Eq,
            1 => Self::Neq,
            _ => return None,
        })
    }
}
#[doc = "nf_tables registers. nf_tables used to have five registers: a verdict\nregister and four data registers of size 16. The data registers have\nbeen changed to 16 registers of size 4. For compatibility reasons, the\n[NFT_REG]()\\[1-4\\] registers still map to areas of size 16, the 4 byte\nregisters are addressed using NFT_REG32_00 - NFT_REG32_15.\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum Registers {
    RegVerdict = 0,
    Reg1 = 1,
    Reg2 = 2,
    Reg3 = 3,
    Reg4 = 4,
    Reg3200 = 8,
    Reg3201 = 9,
    Reg3202 = 10,
    Reg3203 = 11,
    Reg3204 = 12,
    Reg3205 = 13,
    Reg3206 = 14,
    Reg3207 = 15,
    Reg3208 = 16,
    Reg3209 = 17,
    Reg3210 = 18,
    Reg3211 = 19,
    Reg3212 = 20,
    Reg3213 = 21,
    Reg3214 = 22,
    Reg3215 = 23,
}
impl Registers {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::RegVerdict,
            1 => Self::Reg1,
            2 => Self::Reg2,
            3 => Self::Reg3,
            4 => Self::Reg4,
            8 => Self::Reg3200,
            9 => Self::Reg3201,
            10 => Self::Reg3202,
            11 => Self::Reg3203,
            12 => Self::Reg3204,
            13 => Self::Reg3205,
            14 => Self::Reg3206,
            15 => Self::Reg3207,
            16 => Self::Reg3208,
            17 => Self::Reg3209,
            18 => Self::Reg3210,
            19 => Self::Reg3211,
            20 => Self::Reg3212,
            21 => Self::Reg3213,
            22 => Self::Reg3214,
            23 => Self::Reg3215,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum NumgenTypes {
    Incremental = 0,
    Random = 1,
}
impl NumgenTypes {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Incremental,
            1 => Self::Random,
            _ => return None,
        })
    }
}
#[doc = "nf_tables log levels\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    #[doc = "system is unusable\n"]
    Emerg = 0,
    #[doc = "action must be taken immediately\n"]
    Alert = 1,
    #[doc = "critical conditions\n"]
    Crit = 2,
    #[doc = "error conditions\n"]
    Err = 3,
    #[doc = "warning conditions\n"]
    Warning = 4,
    #[doc = "normal but significant condition\n"]
    Notice = 5,
    #[doc = "informational\n"]
    Info = 6,
    #[doc = "debug-level messages\n"]
    Debug = 7,
    #[doc = "enabling audit logging\n"]
    Audit = 8,
}
impl LogLevel {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Emerg,
            1 => Self::Alert,
            2 => Self::Crit,
            3 => Self::Err,
            4 => Self::Warning,
            5 => Self::Notice,
            6 => Self::Info,
            7 => Self::Debug,
            8 => Self::Audit,
            _ => return None,
        })
    }
}
#[doc = "nf_tables log flags\n"]
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum LogFlags {
    #[doc = "Log TCP sequence numbers\n"]
    Tcpseq = 1 << 0,
    #[doc = "Log TCP options\n"]
    Tcpopt = 1 << 1,
    #[doc = "Log IP options\n"]
    Ipopt = 1 << 2,
    #[doc = "Log UID owning local socket\n"]
    Uid = 1 << 3,
    #[doc = "Unsupported, don\\'t reuse\n"]
    Nflog = 1 << 4,
    #[doc = "Decode MAC header\n"]
    Macdecode = 1 << 5,
}
impl LogFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::Tcpseq,
            n if n == 1 << 1 => Self::Tcpopt,
            n if n == 1 << 2 => Self::Ipopt,
            n if n == 1 << 3 => Self::Uid,
            n if n == 1 << 4 => Self::Nflog,
            n if n == 1 << 5 => Self::Macdecode,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum XtLogFlag {
    #[doc = "Log TCP sequence numbers\n"]
    Tcpseq = 1 << 0,
    #[doc = "Log TCP options\n"]
    Tcpopt = 1 << 1,
    #[doc = "Log IP options\n"]
    Ipopt = 1 << 2,
    #[doc = "UID owning local socket\n"]
    Uid = 1 << 3,
    #[doc = "Unsupported, don\\'t reuse\n"]
    Nflog = 1 << 4,
    #[doc = "Decode MAC header\n"]
    Macdecode = 1 << 5,
}
impl XtLogFlag {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::Tcpseq,
            n if n == 1 << 1 => Self::Tcpopt,
            n if n == 1 << 2 => Self::Ipopt,
            n if n == 1 << 3 => Self::Uid,
            n if n == 1 << 4 => Self::Nflog,
            n if n == 1 << 5 => Self::Macdecode,
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
pub struct NatRange2 {
    pub flags: u32,
    pub min_addr: [u8; 16usize],
    pub max_addr: [u8; 16usize],
    pub _min_port_be: u16,
    pub _max_port_be: u16,
    pub _base_port_be: u16,
    pub _pad_42: [u8; 2usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for NatRange2 {
    fn default() -> Self {
        Self::new()
    }
}
impl NatRange2 {
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
    pub fn new_from_array(buf: [u8; 44usize]) -> Self {
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
    pub fn as_array(&self) -> &[u8; 44usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 44usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 44usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<NatRange2>() == 44usize);
        const _: () = assert!(std::mem::align_of::<NatRange2>() == 4usize);
        44usize
    }
    pub fn min_port(&self) -> u16 {
        u16::from_be(self._min_port_be)
    }
    pub fn set_min_port(&mut self, value: u16) {
        self._min_port_be = value.to_be();
    }
    pub fn max_port(&self) -> u16 {
        u16::from_be(self._max_port_be)
    }
    pub fn set_max_port(&mut self, value: u16) {
        self._max_port_be = value.to_be();
    }
    pub fn base_port(&self) -> u16 {
        u16::from_be(self._base_port_be)
    }
    pub fn set_base_port(&mut self, value: u16) {
        self._base_port_be = value.to_be();
    }
}
impl std::fmt::Debug for NatRange2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("NatRange2")
            .field("flags", &self.flags)
            .field("min_addr", &FormatHexdump(self.min_addr.as_slice()))
            .field("max_addr", &FormatHexdump(self.max_addr.as_slice()))
            .field("min_port", &self.min_port())
            .field("max_port", &self.max_port())
            .field("base_port", &self.base_port())
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct NatIpv4MultiRange {
    #[doc = "Always set to 1. Multiple ranges no longer supported.\n"]
    pub range_size: u32,
    pub range: NatIpv4Range,
}
#[doc = "Create zero-initialized struct"]
impl Default for NatIpv4MultiRange {
    fn default() -> Self {
        Self::new()
    }
}
impl NatIpv4MultiRange {
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
        const _: () = assert!(std::mem::size_of::<NatIpv4MultiRange>() == 20usize);
        const _: () = assert!(std::mem::align_of::<NatIpv4MultiRange>() == 4usize);
        20usize
    }
}
impl std::fmt::Debug for NatIpv4MultiRange {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("NatIpv4MultiRange")
            .field("range_size", &self.range_size)
            .field("range", &self.range)
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct NatIpv4Range {
    #[doc = "Associated type: [`NatRangeFlags`] (enum)"]
    pub flags: u32,
    pub _min_ip_be: u32,
    pub _max_ip_be: u32,
    pub _min_port_be: u16,
    pub _max_port_be: u16,
}
#[doc = "Create zero-initialized struct"]
impl Default for NatIpv4Range {
    fn default() -> Self {
        Self::new()
    }
}
impl NatIpv4Range {
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
        const _: () = assert!(std::mem::size_of::<NatIpv4Range>() == 16usize);
        const _: () = assert!(std::mem::align_of::<NatIpv4Range>() == 4usize);
        16usize
    }
    pub fn min_ip(&self) -> u32 {
        u32::from_be(self._min_ip_be)
    }
    pub fn set_min_ip(&mut self, value: u32) {
        self._min_ip_be = value.to_be();
    }
    pub fn max_ip(&self) -> u32 {
        u32::from_be(self._max_ip_be)
    }
    pub fn set_max_ip(&mut self, value: u32) {
        self._max_ip_be = value.to_be();
    }
    pub fn min_port(&self) -> u16 {
        u16::from_be(self._min_port_be)
    }
    pub fn set_min_port(&mut self, value: u16) {
        self._min_port_be = value.to_be();
    }
    pub fn max_port(&self) -> u16 {
        u16::from_be(self._max_port_be)
    }
    pub fn set_max_port(&mut self, value: u16) {
        self._max_port_be = value.to_be();
    }
}
impl std::fmt::Debug for NatIpv4Range {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("NatIpv4Range")
            .field(
                "flags",
                &FormatFlags(self.flags.into(), NatRangeFlags::from_value),
            )
            .field("min_ip", &self.min_ip())
            .field("max_ip", &self.max_ip())
            .field("min_port", &self.min_port())
            .field("max_port", &self.max_port())
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct NatRange {
    pub flags: u32,
    pub min_addr: [u8; 16usize],
    pub max_addr: [u8; 16usize],
    pub _min_port_be: u16,
    pub _max_port_be: u16,
}
#[doc = "Create zero-initialized struct"]
impl Default for NatRange {
    fn default() -> Self {
        Self::new()
    }
}
impl NatRange {
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
    pub fn new_from_array(buf: [u8; 40usize]) -> Self {
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
    pub fn as_array(&self) -> &[u8; 40usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 40usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 40usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<NatRange>() == 40usize);
        const _: () = assert!(std::mem::align_of::<NatRange>() == 4usize);
        40usize
    }
    pub fn min_port(&self) -> u16 {
        u16::from_be(self._min_port_be)
    }
    pub fn set_min_port(&mut self, value: u16) {
        self._min_port_be = value.to_be();
    }
    pub fn max_port(&self) -> u16 {
        u16::from_be(self._max_port_be)
    }
    pub fn set_max_port(&mut self, value: u16) {
        self._max_port_be = value.to_be();
    }
}
impl std::fmt::Debug for NatRange {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("NatRange")
            .field("flags", &self.flags)
            .field("min_addr", &FormatHexdump(self.min_addr.as_slice()))
            .field("max_addr", &FormatHexdump(self.max_addr.as_slice()))
            .field("min_port", &self.min_port())
            .field("max_port", &self.max_port())
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct XtLogInfo {
    #[doc = "Associated type: [`LogLevel`] (enum)"]
    pub level: u8,
    #[doc = "Associated type: [`XtLogFlag`] (enum)"]
    pub logflags: u8,
    #[doc = "null-terminated C-string\n"]
    pub prefix: [u8; 30usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for XtLogInfo {
    fn default() -> Self {
        Self::new()
    }
}
impl XtLogInfo {
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
        const _: () = assert!(std::mem::size_of::<XtLogInfo>() == 32usize);
        const _: () = assert!(std::mem::align_of::<XtLogInfo>() == 1usize);
        32usize
    }
}
impl std::fmt::Debug for XtLogInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("XtLogInfo")
            .field(
                "level",
                &FormatEnum(self.level.into(), LogLevel::from_value),
            )
            .field(
                "logflags",
                &FormatFlags(self.logflags.into(), XtLogFlag::from_value),
            )
            .field("prefix", &FormatBinStr(self.prefix))
            .finish()
    }
}
#[derive(Clone)]
pub enum LogAttrs<'a> {
    #[doc = "netlink group to send messages to\n"]
    Group(u16),
    #[doc = "prefix to prepend to log messages\n"]
    Prefix(&'a CStr),
    #[doc = "length of payload to include in netlink message\n"]
    Snaplen(u32),
    #[doc = "queue threshold\n"]
    Qthreshold(u16),
    #[doc = "log level\n\nAssociated type: [`LogLevel`] (enum)"]
    Level(u32),
    #[doc = "logging flags\n\nAssociated type: [`LogFlags`] (enum)"]
    Flags(u32),
}
impl<'a> IterableLogAttrs<'a> {
    #[doc = "netlink group to send messages to\n"]
    pub fn get_group(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LogAttrs::Group(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LogAttrs",
            "Group",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "prefix to prepend to log messages\n"]
    pub fn get_prefix(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LogAttrs::Prefix(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LogAttrs",
            "Prefix",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "length of payload to include in netlink message\n"]
    pub fn get_snaplen(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LogAttrs::Snaplen(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LogAttrs",
            "Snaplen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "queue threshold\n"]
    pub fn get_qthreshold(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LogAttrs::Qthreshold(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LogAttrs",
            "Qthreshold",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "log level\n\nAssociated type: [`LogLevel`] (enum)"]
    pub fn get_level(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LogAttrs::Level(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LogAttrs",
            "Level",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "logging flags\n\nAssociated type: [`LogFlags`] (enum)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LogAttrs::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LogAttrs",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LogAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLogAttrs<'a> {
        IterableLogAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Group",
            2u16 => "Prefix",
            3u16 => "Snaplen",
            4u16 => "Qthreshold",
            5u16 => "Level",
            6u16 => "Flags",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLogAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLogAttrs<'a> {
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
impl<'a> Iterator for IterableLogAttrs<'a> {
    type Item = Result<LogAttrs<'a>, ErrorContext>;
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
                1u16 => LogAttrs::Group({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => LogAttrs::Prefix({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => LogAttrs::Snaplen({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => LogAttrs::Qthreshold({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => LogAttrs::Level({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => LogAttrs::Flags({
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
            "LogAttrs",
            r#type.and_then(|t| LogAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableLogAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LogAttrs");
        let mut iter = IterableLogAttrs::with_loc(&[], self.orig_loc);
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
                LogAttrs::Group(val) => fmt.field("Group", &val),
                LogAttrs::Prefix(val) => fmt.field("Prefix", &val),
                LogAttrs::Snaplen(val) => fmt.field("Snaplen", &val),
                LogAttrs::Qthreshold(val) => fmt.field("Qthreshold", &val),
                LogAttrs::Level(val) => {
                    fmt.field("Level", &FormatEnum(val.into(), LogLevel::from_value))
                }
                LogAttrs::Flags(val) => {
                    fmt.field("Flags", &FormatFlags(val.into(), LogFlags::from_value))
                }
            };
        }
        fmt.finish()
    }
}
impl IterableLogAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LogAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| LogAttrs::attr_from_type(t)),
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
                LogAttrs::Group(val) => {
                    if last_off == offset {
                        stack.push(("Group", last_off));
                        break;
                    }
                }
                LogAttrs::Prefix(val) => {
                    if last_off == offset {
                        stack.push(("Prefix", last_off));
                        break;
                    }
                }
                LogAttrs::Snaplen(val) => {
                    if last_off == offset {
                        stack.push(("Snaplen", last_off));
                        break;
                    }
                }
                LogAttrs::Qthreshold(val) => {
                    if last_off == offset {
                        stack.push(("Qthreshold", last_off));
                        break;
                    }
                }
                LogAttrs::Level(val) => {
                    if last_off == offset {
                        stack.push(("Level", last_off));
                        break;
                    }
                }
                LogAttrs::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LogAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum NumgenAttrs {
    #[doc = "destination register\n\nAssociated type: [`Registers`] (enum)"]
    Dreg(u32),
    #[doc = "maximum counter value\n"]
    Modulus(u32),
    #[doc = "operation type\n\nAssociated type: [`NumgenTypes`] (enum)"]
    Type(u32),
    #[doc = "offset to be added to the counter\n"]
    Offset(u32),
}
impl<'a> IterableNumgenAttrs<'a> {
    #[doc = "destination register\n\nAssociated type: [`Registers`] (enum)"]
    pub fn get_dreg(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NumgenAttrs::Dreg(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NumgenAttrs",
            "Dreg",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "maximum counter value\n"]
    pub fn get_modulus(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NumgenAttrs::Modulus(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NumgenAttrs",
            "Modulus",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "operation type\n\nAssociated type: [`NumgenTypes`] (enum)"]
    pub fn get_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NumgenAttrs::Type(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NumgenAttrs",
            "Type",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "offset to be added to the counter\n"]
    pub fn get_offset(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NumgenAttrs::Offset(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NumgenAttrs",
            "Offset",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl NumgenAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableNumgenAttrs<'a> {
        IterableNumgenAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Dreg",
            2u16 => "Modulus",
            3u16 => "Type",
            4u16 => "Offset",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableNumgenAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableNumgenAttrs<'a> {
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
impl<'a> Iterator for IterableNumgenAttrs<'a> {
    type Item = Result<NumgenAttrs, ErrorContext>;
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
                1u16 => NumgenAttrs::Dreg({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => NumgenAttrs::Modulus({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => NumgenAttrs::Type({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => NumgenAttrs::Offset({
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
            "NumgenAttrs",
            r#type.and_then(|t| NumgenAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableNumgenAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("NumgenAttrs");
        let mut iter = IterableNumgenAttrs::with_loc(&[], self.orig_loc);
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
                NumgenAttrs::Dreg(val) => {
                    fmt.field("Dreg", &FormatEnum(val.into(), Registers::from_value))
                }
                NumgenAttrs::Modulus(val) => fmt.field("Modulus", &val),
                NumgenAttrs::Type(val) => {
                    fmt.field("Type", &FormatEnum(val.into(), NumgenTypes::from_value))
                }
                NumgenAttrs::Offset(val) => fmt.field("Offset", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableNumgenAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("NumgenAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| NumgenAttrs::attr_from_type(t)),
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
                NumgenAttrs::Dreg(val) => {
                    if last_off == offset {
                        stack.push(("Dreg", last_off));
                        break;
                    }
                }
                NumgenAttrs::Modulus(val) => {
                    if last_off == offset {
                        stack.push(("Modulus", last_off));
                        break;
                    }
                }
                NumgenAttrs::Type(val) => {
                    if last_off == offset {
                        stack.push(("Type", last_off));
                        break;
                    }
                }
                NumgenAttrs::Offset(val) => {
                    if last_off == offset {
                        stack.push(("Offset", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("NumgenAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum RangeAttrs<'a> {
    #[doc = "source register of data to compare\n\nAssociated type: [`Registers`] (enum)"]
    Sreg(u32),
    #[doc = "cmp operation\n\nAssociated type: [`RangeOps`] (enum)"]
    Op(u32),
    #[doc = "data range from\n"]
    FromData(IterableDataAttrs<'a>),
    #[doc = "data range to\n"]
    ToData(IterableDataAttrs<'a>),
}
impl<'a> IterableRangeAttrs<'a> {
    #[doc = "source register of data to compare\n\nAssociated type: [`Registers`] (enum)"]
    pub fn get_sreg(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RangeAttrs::Sreg(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RangeAttrs",
            "Sreg",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "cmp operation\n\nAssociated type: [`RangeOps`] (enum)"]
    pub fn get_op(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RangeAttrs::Op(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RangeAttrs",
            "Op",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "data range from\n"]
    pub fn get_from_data(&self) -> Result<IterableDataAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RangeAttrs::FromData(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RangeAttrs",
            "FromData",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "data range to\n"]
    pub fn get_to_data(&self) -> Result<IterableDataAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RangeAttrs::ToData(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RangeAttrs",
            "ToData",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl RangeAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableRangeAttrs<'a> {
        IterableRangeAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Sreg",
            2u16 => "Op",
            3u16 => "FromData",
            4u16 => "ToData",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableRangeAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableRangeAttrs<'a> {
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
impl<'a> Iterator for IterableRangeAttrs<'a> {
    type Item = Result<RangeAttrs<'a>, ErrorContext>;
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
                1u16 => RangeAttrs::Sreg({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => RangeAttrs::Op({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => RangeAttrs::FromData({
                    let res = Some(IterableDataAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => RangeAttrs::ToData({
                    let res = Some(IterableDataAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "RangeAttrs",
            r#type.and_then(|t| RangeAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableRangeAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("RangeAttrs");
        let mut iter = IterableRangeAttrs::with_loc(&[], self.orig_loc);
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
                RangeAttrs::Sreg(val) => {
                    fmt.field("Sreg", &FormatEnum(val.into(), Registers::from_value))
                }
                RangeAttrs::Op(val) => {
                    fmt.field("Op", &FormatEnum(val.into(), RangeOps::from_value))
                }
                RangeAttrs::FromData(val) => fmt.field("FromData", &val),
                RangeAttrs::ToData(val) => fmt.field("ToData", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableRangeAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("RangeAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| RangeAttrs::attr_from_type(t)),
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
                RangeAttrs::Sreg(val) => {
                    if last_off == offset {
                        stack.push(("Sreg", last_off));
                        break;
                    }
                }
                RangeAttrs::Op(val) => {
                    if last_off == offset {
                        stack.push(("Op", last_off));
                        break;
                    }
                }
                RangeAttrs::FromData(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                RangeAttrs::ToData(val) => {
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
            stack.push(("RangeAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum BatchAttrs {
    #[doc = "generation ID for this changeset\n"]
    Genid(u32),
}
impl<'a> IterableBatchAttrs<'a> {
    #[doc = "generation ID for this changeset\n"]
    pub fn get_genid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(BatchAttrs::Genid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BatchAttrs",
            "Genid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl BatchAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableBatchAttrs<'a> {
        IterableBatchAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Genid",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableBatchAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableBatchAttrs<'a> {
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
impl<'a> Iterator for IterableBatchAttrs<'a> {
    type Item = Result<BatchAttrs, ErrorContext>;
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
                1u16 => BatchAttrs::Genid({
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
            "BatchAttrs",
            r#type.and_then(|t| BatchAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableBatchAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("BatchAttrs");
        let mut iter = IterableBatchAttrs::with_loc(&[], self.orig_loc);
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
                BatchAttrs::Genid(val) => fmt.field("Genid", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableBatchAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("BatchAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| BatchAttrs::attr_from_type(t)),
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
                BatchAttrs::Genid(val) => {
                    if last_off == offset {
                        stack.push(("Genid", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("BatchAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum TableAttrs<'a> {
    #[doc = "name of the table\n"]
    Name(&'a CStr),
    #[doc = "bitmask of flags\n\nAssociated type: [`TableFlags`] (1 bit per enumeration)"]
    Flags(u32),
    #[doc = "number of chains in this table\n"]
    Use(u32),
    #[doc = "numeric handle of the table\n"]
    Handle(u64),
    Pad(&'a [u8]),
    #[doc = "user data\n"]
    Userdata(&'a [u8]),
    #[doc = "owner of this table through netlink portID\n"]
    Owner(u32),
}
impl<'a> IterableTableAttrs<'a> {
    #[doc = "name of the table\n"]
    pub fn get_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TableAttrs::Name(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TableAttrs",
            "Name",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "bitmask of flags\n\nAssociated type: [`TableFlags`] (1 bit per enumeration)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TableAttrs::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TableAttrs",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "number of chains in this table\n"]
    pub fn get_use(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TableAttrs::Use(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TableAttrs",
            "Use",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "numeric handle of the table\n"]
    pub fn get_handle(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TableAttrs::Handle(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TableAttrs",
            "Handle",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TableAttrs::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TableAttrs",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "user data\n"]
    pub fn get_userdata(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TableAttrs::Userdata(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TableAttrs",
            "Userdata",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "owner of this table through netlink portID\n"]
    pub fn get_owner(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TableAttrs::Owner(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TableAttrs",
            "Owner",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl TableAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableTableAttrs<'a> {
        IterableTableAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Name",
            2u16 => "Flags",
            3u16 => "Use",
            4u16 => "Handle",
            5u16 => "Pad",
            6u16 => "Userdata",
            7u16 => "Owner",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableTableAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableTableAttrs<'a> {
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
impl<'a> Iterator for IterableTableAttrs<'a> {
    type Item = Result<TableAttrs<'a>, ErrorContext>;
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
                1u16 => TableAttrs::Name({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => TableAttrs::Flags({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => TableAttrs::Use({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => TableAttrs::Handle({
                    let res = parse_be_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => TableAttrs::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => TableAttrs::Userdata({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => TableAttrs::Owner({
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
            "TableAttrs",
            r#type.and_then(|t| TableAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableTableAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("TableAttrs");
        let mut iter = IterableTableAttrs::with_loc(&[], self.orig_loc);
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
                TableAttrs::Name(val) => fmt.field("Name", &val),
                TableAttrs::Flags(val) => {
                    fmt.field("Flags", &FormatFlags(val.into(), TableFlags::from_value))
                }
                TableAttrs::Use(val) => fmt.field("Use", &val),
                TableAttrs::Handle(val) => fmt.field("Handle", &val),
                TableAttrs::Pad(val) => fmt.field("Pad", &val),
                TableAttrs::Userdata(val) => fmt.field("Userdata", &FormatHexdump(val)),
                TableAttrs::Owner(val) => fmt.field("Owner", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableTableAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("TableAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| TableAttrs::attr_from_type(t)),
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
                TableAttrs::Name(val) => {
                    if last_off == offset {
                        stack.push(("Name", last_off));
                        break;
                    }
                }
                TableAttrs::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                TableAttrs::Use(val) => {
                    if last_off == offset {
                        stack.push(("Use", last_off));
                        break;
                    }
                }
                TableAttrs::Handle(val) => {
                    if last_off == offset {
                        stack.push(("Handle", last_off));
                        break;
                    }
                }
                TableAttrs::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                TableAttrs::Userdata(val) => {
                    if last_off == offset {
                        stack.push(("Userdata", last_off));
                        break;
                    }
                }
                TableAttrs::Owner(val) => {
                    if last_off == offset {
                        stack.push(("Owner", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("TableAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ChainAttrs<'a> {
    #[doc = "name of the table containing the chain\n"]
    Table(&'a CStr),
    #[doc = "numeric handle of the chain\n"]
    Handle(u64),
    #[doc = "name of the chain\n"]
    Name(&'a CStr),
    #[doc = "hook specification for basechains\n"]
    Hook(IterableNftHookAttrs<'a>),
    #[doc = "numeric policy of the chain\n"]
    Policy(u32),
    #[doc = "number of references to this chain\n"]
    Use(u32),
    #[doc = "type name of the chain\n"]
    Type(&'a CStr),
    #[doc = "counter specification of the chain\n"]
    Counters(IterableNftCounterAttrs<'a>),
    #[doc = "chain flags\n\nAssociated type: [`ChainFlags`] (1 bit per enumeration)"]
    Flags(u32),
    #[doc = "uniquely identifies a chain in a transaction\n"]
    Id(u32),
    #[doc = "user data\n"]
    Userdata(&'a [u8]),
}
impl<'a> IterableChainAttrs<'a> {
    #[doc = "name of the table containing the chain\n"]
    pub fn get_table(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ChainAttrs::Table(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ChainAttrs",
            "Table",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "numeric handle of the chain\n"]
    pub fn get_handle(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ChainAttrs::Handle(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ChainAttrs",
            "Handle",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "name of the chain\n"]
    pub fn get_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ChainAttrs::Name(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ChainAttrs",
            "Name",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "hook specification for basechains\n"]
    pub fn get_hook(&self) -> Result<IterableNftHookAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ChainAttrs::Hook(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ChainAttrs",
            "Hook",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "numeric policy of the chain\n"]
    pub fn get_policy(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ChainAttrs::Policy(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ChainAttrs",
            "Policy",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "number of references to this chain\n"]
    pub fn get_use(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ChainAttrs::Use(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ChainAttrs",
            "Use",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "type name of the chain\n"]
    pub fn get_type(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ChainAttrs::Type(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ChainAttrs",
            "Type",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "counter specification of the chain\n"]
    pub fn get_counters(&self) -> Result<IterableNftCounterAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ChainAttrs::Counters(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ChainAttrs",
            "Counters",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "chain flags\n\nAssociated type: [`ChainFlags`] (1 bit per enumeration)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ChainAttrs::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ChainAttrs",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "uniquely identifies a chain in a transaction\n"]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ChainAttrs::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ChainAttrs",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "user data\n"]
    pub fn get_userdata(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ChainAttrs::Userdata(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ChainAttrs",
            "Userdata",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ChainAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableChainAttrs<'a> {
        IterableChainAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Table",
            2u16 => "Handle",
            3u16 => "Name",
            4u16 => "Hook",
            5u16 => "Policy",
            6u16 => "Use",
            7u16 => "Type",
            8u16 => "Counters",
            9u16 => "Flags",
            10u16 => "Id",
            11u16 => "Userdata",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableChainAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableChainAttrs<'a> {
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
impl<'a> Iterator for IterableChainAttrs<'a> {
    type Item = Result<ChainAttrs<'a>, ErrorContext>;
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
                1u16 => ChainAttrs::Table({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ChainAttrs::Handle({
                    let res = parse_be_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ChainAttrs::Name({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => ChainAttrs::Hook({
                    let res = Some(IterableNftHookAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => ChainAttrs::Policy({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => ChainAttrs::Use({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => ChainAttrs::Type({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => ChainAttrs::Counters({
                    let res = Some(IterableNftCounterAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => ChainAttrs::Flags({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => ChainAttrs::Id({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => ChainAttrs::Userdata({
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
            "ChainAttrs",
            r#type.and_then(|t| ChainAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableChainAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ChainAttrs");
        let mut iter = IterableChainAttrs::with_loc(&[], self.orig_loc);
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
                ChainAttrs::Table(val) => fmt.field("Table", &val),
                ChainAttrs::Handle(val) => fmt.field("Handle", &val),
                ChainAttrs::Name(val) => fmt.field("Name", &val),
                ChainAttrs::Hook(val) => fmt.field("Hook", &val),
                ChainAttrs::Policy(val) => fmt.field("Policy", &val),
                ChainAttrs::Use(val) => fmt.field("Use", &val),
                ChainAttrs::Type(val) => fmt.field("Type", &val),
                ChainAttrs::Counters(val) => fmt.field("Counters", &val),
                ChainAttrs::Flags(val) => {
                    fmt.field("Flags", &FormatFlags(val.into(), ChainFlags::from_value))
                }
                ChainAttrs::Id(val) => fmt.field("Id", &val),
                ChainAttrs::Userdata(val) => fmt.field("Userdata", &FormatHexdump(val)),
            };
        }
        fmt.finish()
    }
}
impl IterableChainAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ChainAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ChainAttrs::attr_from_type(t)),
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
                ChainAttrs::Table(val) => {
                    if last_off == offset {
                        stack.push(("Table", last_off));
                        break;
                    }
                }
                ChainAttrs::Handle(val) => {
                    if last_off == offset {
                        stack.push(("Handle", last_off));
                        break;
                    }
                }
                ChainAttrs::Name(val) => {
                    if last_off == offset {
                        stack.push(("Name", last_off));
                        break;
                    }
                }
                ChainAttrs::Hook(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ChainAttrs::Policy(val) => {
                    if last_off == offset {
                        stack.push(("Policy", last_off));
                        break;
                    }
                }
                ChainAttrs::Use(val) => {
                    if last_off == offset {
                        stack.push(("Use", last_off));
                        break;
                    }
                }
                ChainAttrs::Type(val) => {
                    if last_off == offset {
                        stack.push(("Type", last_off));
                        break;
                    }
                }
                ChainAttrs::Counters(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ChainAttrs::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                ChainAttrs::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                ChainAttrs::Userdata(val) => {
                    if last_off == offset {
                        stack.push(("Userdata", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ChainAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum CounterAttrs<'a> {
    Bytes(u64),
    Packets(u64),
    Pad(&'a [u8]),
}
impl<'a> IterableCounterAttrs<'a> {
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
            1u16 => "Bytes",
            2u16 => "Packets",
            3u16 => "Pad",
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
                1u16 => CounterAttrs::Bytes({
                    let res = parse_be_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => CounterAttrs::Packets({
                    let res = parse_be_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => CounterAttrs::Pad({
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
                CounterAttrs::Bytes(val) => fmt.field("Bytes", &val),
                CounterAttrs::Packets(val) => fmt.field("Packets", &val),
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
                CounterAttrs::Bytes(val) => {
                    if last_off == offset {
                        stack.push(("Bytes", last_off));
                        break;
                    }
                }
                CounterAttrs::Packets(val) => {
                    if last_off == offset {
                        stack.push(("Packets", last_off));
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
pub enum NftHookAttrs<'a> {
    Num(u32),
    Priority(i32),
    #[doc = "net device name\n"]
    Dev(&'a CStr),
    #[doc = "list of net devices\n"]
    Devs(IterableHookDevAttrs<'a>),
}
impl<'a> IterableNftHookAttrs<'a> {
    pub fn get_num(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NftHookAttrs::Num(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NftHookAttrs",
            "Num",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_priority(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NftHookAttrs::Priority(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NftHookAttrs",
            "Priority",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "net device name\n"]
    pub fn get_dev(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NftHookAttrs::Dev(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NftHookAttrs",
            "Dev",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "list of net devices\n"]
    pub fn get_devs(&self) -> Result<IterableHookDevAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NftHookAttrs::Devs(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NftHookAttrs",
            "Devs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl NftHookAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableNftHookAttrs<'a> {
        IterableNftHookAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Num",
            2u16 => "Priority",
            3u16 => "Dev",
            4u16 => "Devs",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableNftHookAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableNftHookAttrs<'a> {
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
impl<'a> Iterator for IterableNftHookAttrs<'a> {
    type Item = Result<NftHookAttrs<'a>, ErrorContext>;
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
                1u16 => NftHookAttrs::Num({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => NftHookAttrs::Priority({
                    let res = parse_be_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => NftHookAttrs::Dev({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => NftHookAttrs::Devs({
                    let res = Some(IterableHookDevAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "NftHookAttrs",
            r#type.and_then(|t| NftHookAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableNftHookAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("NftHookAttrs");
        let mut iter = IterableNftHookAttrs::with_loc(&[], self.orig_loc);
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
                NftHookAttrs::Num(val) => fmt.field("Num", &val),
                NftHookAttrs::Priority(val) => fmt.field("Priority", &val),
                NftHookAttrs::Dev(val) => fmt.field("Dev", &val),
                NftHookAttrs::Devs(val) => fmt.field("Devs", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableNftHookAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("NftHookAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| NftHookAttrs::attr_from_type(t)),
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
                NftHookAttrs::Num(val) => {
                    if last_off == offset {
                        stack.push(("Num", last_off));
                        break;
                    }
                }
                NftHookAttrs::Priority(val) => {
                    if last_off == offset {
                        stack.push(("Priority", last_off));
                        break;
                    }
                }
                NftHookAttrs::Dev(val) => {
                    if last_off == offset {
                        stack.push(("Dev", last_off));
                        break;
                    }
                }
                NftHookAttrs::Devs(val) => {
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
            stack.push(("NftHookAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum HookDevAttrs<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    Name(&'a CStr),
}
impl<'a> IterableHookDevAttrs<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_name(&self) -> MultiAttrIterable<Self, HookDevAttrs<'a>, &'a CStr> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let HookDevAttrs::Name(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl HookDevAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableHookDevAttrs<'a> {
        IterableHookDevAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Name",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableHookDevAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableHookDevAttrs<'a> {
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
impl<'a> Iterator for IterableHookDevAttrs<'a> {
    type Item = Result<HookDevAttrs<'a>, ErrorContext>;
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
                1u16 => HookDevAttrs::Name({
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
            "HookDevAttrs",
            r#type.and_then(|t| HookDevAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableHookDevAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("HookDevAttrs");
        let mut iter = IterableHookDevAttrs::with_loc(&[], self.orig_loc);
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
                HookDevAttrs::Name(val) => fmt.field("Name", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableHookDevAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("HookDevAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| HookDevAttrs::attr_from_type(t)),
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
                HookDevAttrs::Name(val) => {
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
            stack.push(("HookDevAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum NftCounterAttrs {
    Bytes(u64),
    Packets(u64),
}
impl<'a> IterableNftCounterAttrs<'a> {
    pub fn get_bytes(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NftCounterAttrs::Bytes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NftCounterAttrs",
            "Bytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_packets(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NftCounterAttrs::Packets(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NftCounterAttrs",
            "Packets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl NftCounterAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableNftCounterAttrs<'a> {
        IterableNftCounterAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Bytes",
            2u16 => "Packets",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableNftCounterAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableNftCounterAttrs<'a> {
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
impl<'a> Iterator for IterableNftCounterAttrs<'a> {
    type Item = Result<NftCounterAttrs, ErrorContext>;
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
                1u16 => NftCounterAttrs::Bytes({
                    let res = parse_be_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => NftCounterAttrs::Packets({
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
            "NftCounterAttrs",
            r#type.and_then(|t| NftCounterAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableNftCounterAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("NftCounterAttrs");
        let mut iter = IterableNftCounterAttrs::with_loc(&[], self.orig_loc);
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
                NftCounterAttrs::Bytes(val) => fmt.field("Bytes", &val),
                NftCounterAttrs::Packets(val) => fmt.field("Packets", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableNftCounterAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("NftCounterAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| NftCounterAttrs::attr_from_type(t)),
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
                NftCounterAttrs::Bytes(val) => {
                    if last_off == offset {
                        stack.push(("Bytes", last_off));
                        break;
                    }
                }
                NftCounterAttrs::Packets(val) => {
                    if last_off == offset {
                        stack.push(("Packets", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("NftCounterAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum RuleAttrs<'a> {
    #[doc = "name of the table containing the rule\n"]
    Table(&'a CStr),
    #[doc = "name of the chain containing the rule\n"]
    Chain(&'a CStr),
    #[doc = "numeric handle of the rule\n"]
    Handle(u64),
    #[doc = "list of expressions\n"]
    Expressions(IterableExprListAttrs<'a>),
    #[doc = "compatibility specifications of the rule\n"]
    Compat(IterableRuleCompatAttrs<'a>),
    #[doc = "numeric handle of the previous rule\n"]
    Position(u64),
    #[doc = "user data\n"]
    Userdata(&'a [u8]),
    #[doc = "uniquely identifies a rule in a transaction\n"]
    Id(u32),
    #[doc = "transaction unique identifier of the previous rule\n"]
    PositionId(u32),
    #[doc = "add the rule to chain by ID, alternative to chain name\n"]
    ChainId(u32),
}
impl<'a> IterableRuleAttrs<'a> {
    #[doc = "name of the table containing the rule\n"]
    pub fn get_table(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RuleAttrs::Table(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RuleAttrs",
            "Table",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "name of the chain containing the rule\n"]
    pub fn get_chain(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RuleAttrs::Chain(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RuleAttrs",
            "Chain",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "numeric handle of the rule\n"]
    pub fn get_handle(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RuleAttrs::Handle(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RuleAttrs",
            "Handle",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "list of expressions\n"]
    pub fn get_expressions(&self) -> Result<IterableExprListAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RuleAttrs::Expressions(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RuleAttrs",
            "Expressions",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "compatibility specifications of the rule\n"]
    pub fn get_compat(&self) -> Result<IterableRuleCompatAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RuleAttrs::Compat(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RuleAttrs",
            "Compat",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "numeric handle of the previous rule\n"]
    pub fn get_position(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RuleAttrs::Position(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RuleAttrs",
            "Position",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "user data\n"]
    pub fn get_userdata(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RuleAttrs::Userdata(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RuleAttrs",
            "Userdata",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "uniquely identifies a rule in a transaction\n"]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RuleAttrs::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RuleAttrs",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "transaction unique identifier of the previous rule\n"]
    pub fn get_position_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RuleAttrs::PositionId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RuleAttrs",
            "PositionId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "add the rule to chain by ID, alternative to chain name\n"]
    pub fn get_chain_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RuleAttrs::ChainId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RuleAttrs",
            "ChainId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl RuleAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableRuleAttrs<'a> {
        IterableRuleAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Table",
            2u16 => "Chain",
            3u16 => "Handle",
            4u16 => "Expressions",
            5u16 => "Compat",
            6u16 => "Position",
            7u16 => "Userdata",
            8u16 => "Id",
            9u16 => "PositionId",
            10u16 => "ChainId",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableRuleAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableRuleAttrs<'a> {
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
impl<'a> Iterator for IterableRuleAttrs<'a> {
    type Item = Result<RuleAttrs<'a>, ErrorContext>;
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
                1u16 => RuleAttrs::Table({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => RuleAttrs::Chain({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => RuleAttrs::Handle({
                    let res = parse_be_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => RuleAttrs::Expressions({
                    let res = Some(IterableExprListAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => RuleAttrs::Compat({
                    let res = Some(IterableRuleCompatAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => RuleAttrs::Position({
                    let res = parse_be_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => RuleAttrs::Userdata({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => RuleAttrs::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => RuleAttrs::PositionId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => RuleAttrs::ChainId({
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
            "RuleAttrs",
            r#type.and_then(|t| RuleAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableRuleAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("RuleAttrs");
        let mut iter = IterableRuleAttrs::with_loc(&[], self.orig_loc);
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
                RuleAttrs::Table(val) => fmt.field("Table", &val),
                RuleAttrs::Chain(val) => fmt.field("Chain", &val),
                RuleAttrs::Handle(val) => fmt.field("Handle", &val),
                RuleAttrs::Expressions(val) => fmt.field("Expressions", &val),
                RuleAttrs::Compat(val) => fmt.field("Compat", &val),
                RuleAttrs::Position(val) => fmt.field("Position", &val),
                RuleAttrs::Userdata(val) => fmt.field("Userdata", &FormatHexdump(val)),
                RuleAttrs::Id(val) => fmt.field("Id", &val),
                RuleAttrs::PositionId(val) => fmt.field("PositionId", &val),
                RuleAttrs::ChainId(val) => fmt.field("ChainId", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableRuleAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("RuleAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| RuleAttrs::attr_from_type(t)),
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
                RuleAttrs::Table(val) => {
                    if last_off == offset {
                        stack.push(("Table", last_off));
                        break;
                    }
                }
                RuleAttrs::Chain(val) => {
                    if last_off == offset {
                        stack.push(("Chain", last_off));
                        break;
                    }
                }
                RuleAttrs::Handle(val) => {
                    if last_off == offset {
                        stack.push(("Handle", last_off));
                        break;
                    }
                }
                RuleAttrs::Expressions(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                RuleAttrs::Compat(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                RuleAttrs::Position(val) => {
                    if last_off == offset {
                        stack.push(("Position", last_off));
                        break;
                    }
                }
                RuleAttrs::Userdata(val) => {
                    if last_off == offset {
                        stack.push(("Userdata", last_off));
                        break;
                    }
                }
                RuleAttrs::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                RuleAttrs::PositionId(val) => {
                    if last_off == offset {
                        stack.push(("PositionId", last_off));
                        break;
                    }
                }
                RuleAttrs::ChainId(val) => {
                    if last_off == offset {
                        stack.push(("ChainId", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("RuleAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum ExprListAttrs<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    Elem(IterableExprAttrs<'a>),
}
impl<'a> IterableExprListAttrs<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_elem(&self) -> MultiAttrIterable<Self, ExprListAttrs<'a>, IterableExprAttrs<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let ExprListAttrs::Elem(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl ExprListAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableExprListAttrs<'a> {
        IterableExprListAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Elem",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableExprListAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableExprListAttrs<'a> {
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
impl<'a> Iterator for IterableExprListAttrs<'a> {
    type Item = Result<ExprListAttrs<'a>, ErrorContext>;
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
                1u16 => ExprListAttrs::Elem({
                    let res = Some(IterableExprAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "ExprListAttrs",
            r#type.and_then(|t| ExprListAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableExprListAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ExprListAttrs");
        let mut iter = IterableExprListAttrs::with_loc(&[], self.orig_loc);
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
                ExprListAttrs::Elem(val) => fmt.field("Elem", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableExprListAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ExprListAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ExprListAttrs::attr_from_type(t)),
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
                ExprListAttrs::Elem(val) => {
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
            stack.push(("ExprListAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum ExprAttrs<'a> {
    #[doc = "name of the expression type\n"]
    Name(&'a CStr),
    #[doc = "type specific data\n"]
    Data(ExprOps<'a>),
}
impl<'a> IterableExprAttrs<'a> {
    #[doc = "name of the expression type\n"]
    pub fn get_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprAttrs::Name(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprAttrs",
            "Name",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "type specific data\n"]
    pub fn get_data(&self) -> Result<ExprOps<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprAttrs::Data(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprAttrs",
            "Data",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
#[derive(Debug, Clone)]
pub enum ExprOps<'a> {
    Bitwise(IterableExprBitwiseAttrs<'a>),
    Cmp(IterableExprCmpAttrs<'a>),
    Counter(IterableExprCounterAttrs<'a>),
    Ct(IterableExprCtAttrs<'a>),
    Fib(IterableExprFibAttrs<'a>),
    FlowOffload(IterableExprFlowOffloadAttrs<'a>),
    Immediate(IterableExprImmediateAttrs<'a>),
    Log(IterableLogAttrs<'a>),
    Lookup(IterableExprLookupAttrs<'a>),
    Match(IterableCompatMatchAttrs<'a>),
    Meta(IterableExprMetaAttrs<'a>),
    Nat(IterableExprNatAttrs<'a>),
    Numgen(IterableNumgenAttrs<'a>),
    Objref(IterableExprObjrefAttrs<'a>),
    Payload(IterableExprPayloadAttrs<'a>),
    Quota(IterableQuotaAttrs<'a>),
    Range(IterableRangeAttrs<'a>),
    Reject(IterableExprRejectAttrs<'a>),
    Target(IterableExprTargetAttrs<'a>),
    Tproxy(IterableExprTproxyAttrs<'a>),
}
impl<'a> ExprOps<'a> {
    fn select_with_loc(selector: &'_ [u8], buf: &'a [u8], loc: usize) -> Option<Self> {
        match selector {
            b"bitwise" => Some(ExprOps::Bitwise(IterableExprBitwiseAttrs::with_loc(
                buf, loc,
            ))),
            b"cmp" => Some(ExprOps::Cmp(IterableExprCmpAttrs::with_loc(buf, loc))),
            b"counter" => Some(ExprOps::Counter(IterableExprCounterAttrs::with_loc(
                buf, loc,
            ))),
            b"ct" => Some(ExprOps::Ct(IterableExprCtAttrs::with_loc(buf, loc))),
            b"fib" => Some(ExprOps::Fib(IterableExprFibAttrs::with_loc(buf, loc))),
            b"flow_offload" => Some(ExprOps::FlowOffload(
                IterableExprFlowOffloadAttrs::with_loc(buf, loc),
            )),
            b"immediate" => Some(ExprOps::Immediate(IterableExprImmediateAttrs::with_loc(
                buf, loc,
            ))),
            b"log" => Some(ExprOps::Log(IterableLogAttrs::with_loc(buf, loc))),
            b"lookup" => Some(ExprOps::Lookup(IterableExprLookupAttrs::with_loc(buf, loc))),
            b"match" => Some(ExprOps::Match(IterableCompatMatchAttrs::with_loc(buf, loc))),
            b"meta" => Some(ExprOps::Meta(IterableExprMetaAttrs::with_loc(buf, loc))),
            b"nat" => Some(ExprOps::Nat(IterableExprNatAttrs::with_loc(buf, loc))),
            b"numgen" => Some(ExprOps::Numgen(IterableNumgenAttrs::with_loc(buf, loc))),
            b"objref" => Some(ExprOps::Objref(IterableExprObjrefAttrs::with_loc(buf, loc))),
            b"payload" => Some(ExprOps::Payload(IterableExprPayloadAttrs::with_loc(
                buf, loc,
            ))),
            b"quota" => Some(ExprOps::Quota(IterableQuotaAttrs::with_loc(buf, loc))),
            b"range" => Some(ExprOps::Range(IterableRangeAttrs::with_loc(buf, loc))),
            b"reject" => Some(ExprOps::Reject(IterableExprRejectAttrs::with_loc(buf, loc))),
            b"target" => Some(ExprOps::Target(IterableExprTargetAttrs::with_loc(buf, loc))),
            b"tproxy" => Some(ExprOps::Tproxy(IterableExprTproxyAttrs::with_loc(buf, loc))),
            _ => None,
        }
    }
}
impl ExprAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableExprAttrs<'a> {
        IterableExprAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Name",
            2u16 => "Data",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableExprAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
    selector_name: Option<&'a [u8]>,
}
impl<'a> IterableExprAttrs<'a> {
    fn with_loc(buf: &'a [u8], orig_loc: usize) -> Self {
        Self {
            buf,
            pos: 0,
            orig_loc,
            selector_name: None,
        }
    }
    pub fn get_buf(&self) -> &'a [u8] {
        self.buf
    }
}
impl<'a> Iterator for IterableExprAttrs<'a> {
    type Item = Result<ExprAttrs<'a>, ErrorContext>;
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
                1u16 => ExprAttrs::Name({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ExprAttrs::Data({
                    let res = {
                        let Some(selector) = self.selector_name else {
                            break;
                        };
                        match ExprOps::select_with_loc(selector, next, self.orig_loc) {
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
            if let ExprAttrs::Name(sel) = &res {
                self.selector_name = Some(sel.to_bytes());
            }
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "ExprAttrs",
            r#type.and_then(|t| ExprAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableExprAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ExprAttrs");
        let mut iter = IterableExprAttrs::with_loc(&[], self.orig_loc);
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
                ExprAttrs::Name(val) => fmt.field("Name", &val),
                ExprAttrs::Data(val) => fmt.field("Data", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableExprAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ExprAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ExprAttrs::attr_from_type(t)),
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
                ExprAttrs::Name(val) => {
                    if last_off == offset {
                        stack.push(("Name", last_off));
                        break;
                    }
                }
                ExprAttrs::Data(val) => {
                    if last_off == offset {
                        stack.push(("Data", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ExprAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum RuleCompatAttrs {
    #[doc = "numeric value of the handled protocol\n"]
    Proto(u32),
    #[doc = "bitmask of flags\n"]
    Flags(u32),
}
impl<'a> IterableRuleCompatAttrs<'a> {
    #[doc = "numeric value of the handled protocol\n"]
    pub fn get_proto(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RuleCompatAttrs::Proto(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RuleCompatAttrs",
            "Proto",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "bitmask of flags\n"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RuleCompatAttrs::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RuleCompatAttrs",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl RuleCompatAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableRuleCompatAttrs<'a> {
        IterableRuleCompatAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Proto",
            2u16 => "Flags",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableRuleCompatAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableRuleCompatAttrs<'a> {
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
impl<'a> Iterator for IterableRuleCompatAttrs<'a> {
    type Item = Result<RuleCompatAttrs, ErrorContext>;
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
                1u16 => RuleCompatAttrs::Proto({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => RuleCompatAttrs::Flags({
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
            "RuleCompatAttrs",
            r#type.and_then(|t| RuleCompatAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableRuleCompatAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("RuleCompatAttrs");
        let mut iter = IterableRuleCompatAttrs::with_loc(&[], self.orig_loc);
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
                RuleCompatAttrs::Proto(val) => fmt.field("Proto", &val),
                RuleCompatAttrs::Flags(val) => fmt.field("Flags", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableRuleCompatAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("RuleCompatAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| RuleCompatAttrs::attr_from_type(t)),
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
                RuleCompatAttrs::Proto(val) => {
                    if last_off == offset {
                        stack.push(("Proto", last_off));
                        break;
                    }
                }
                RuleCompatAttrs::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("RuleCompatAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum SetAttrs<'a> {
    #[doc = "table name\n"]
    Table(&'a CStr),
    #[doc = "set name\n"]
    Name(&'a CStr),
    #[doc = "bitmask of enum nft_set_flags\n\nAssociated type: [`SetFlags`] (enum)"]
    Flags(u32),
    #[doc = "key data type, informational purpose only\n"]
    KeyType(u32),
    #[doc = "key data length\n"]
    KeyLen(u32),
    #[doc = "mapping data type\n"]
    DataType(u32),
    #[doc = "mapping data length\n"]
    DataLen(u32),
    #[doc = "selection policy\n"]
    Policy(u32),
    #[doc = "set description\n"]
    Desc(IterableSetDescAttrs<'a>),
    #[doc = "uniquely identifies a set in a transaction\n"]
    Id(u32),
    #[doc = "default timeout value\n"]
    Timeout(u64),
    #[doc = "garbage collection interval\n"]
    GcInterval(u32),
    #[doc = "user data\n"]
    Userdata(&'a [u8]),
    Pad(&'a [u8]),
    #[doc = "stateful object type\n"]
    ObjType(u32),
    #[doc = "set handle\n"]
    Handle(u64),
    #[doc = "set expression\n\nAttribute may repeat multiple times (treat it as array)"]
    Expr(IterableExprAttrs<'a>),
    #[doc = "list of expressions\n"]
    Expressions(IterableSetListAttrs<'a>),
    #[doc = "set backend type\n"]
    Type(&'a CStr),
    #[doc = "number of set elements\n"]
    Count(u32),
}
impl<'a> IterableSetAttrs<'a> {
    #[doc = "table name\n"]
    pub fn get_table(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetAttrs::Table(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetAttrs",
            "Table",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "set name\n"]
    pub fn get_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetAttrs::Name(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetAttrs",
            "Name",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "bitmask of enum nft_set_flags\n\nAssociated type: [`SetFlags`] (enum)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetAttrs::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetAttrs",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "key data type, informational purpose only\n"]
    pub fn get_key_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetAttrs::KeyType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetAttrs",
            "KeyType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "key data length\n"]
    pub fn get_key_len(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetAttrs::KeyLen(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetAttrs",
            "KeyLen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "mapping data type\n"]
    pub fn get_data_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetAttrs::DataType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetAttrs",
            "DataType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "mapping data length\n"]
    pub fn get_data_len(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetAttrs::DataLen(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetAttrs",
            "DataLen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "selection policy\n"]
    pub fn get_policy(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetAttrs::Policy(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetAttrs",
            "Policy",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "set description\n"]
    pub fn get_desc(&self) -> Result<IterableSetDescAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetAttrs::Desc(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetAttrs",
            "Desc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "uniquely identifies a set in a transaction\n"]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetAttrs::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetAttrs",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "default timeout value\n"]
    pub fn get_timeout(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetAttrs::Timeout(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetAttrs",
            "Timeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "garbage collection interval\n"]
    pub fn get_gc_interval(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetAttrs::GcInterval(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetAttrs",
            "GcInterval",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "user data\n"]
    pub fn get_userdata(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetAttrs::Userdata(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetAttrs",
            "Userdata",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetAttrs::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetAttrs",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "stateful object type\n"]
    pub fn get_obj_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetAttrs::ObjType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetAttrs",
            "ObjType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "set handle\n"]
    pub fn get_handle(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetAttrs::Handle(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetAttrs",
            "Handle",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "set expression\n\nAttribute may repeat multiple times (treat it as array)"]
    pub fn get_expr(&self) -> MultiAttrIterable<Self, SetAttrs<'a>, IterableExprAttrs<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let SetAttrs::Expr(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    #[doc = "list of expressions\n"]
    pub fn get_expressions(&self) -> Result<IterableSetListAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetAttrs::Expressions(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetAttrs",
            "Expressions",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "set backend type\n"]
    pub fn get_type(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetAttrs::Type(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetAttrs",
            "Type",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "number of set elements\n"]
    pub fn get_count(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetAttrs::Count(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetAttrs",
            "Count",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl SetAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableSetAttrs<'a> {
        IterableSetAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Table",
            2u16 => "Name",
            3u16 => "Flags",
            4u16 => "KeyType",
            5u16 => "KeyLen",
            6u16 => "DataType",
            7u16 => "DataLen",
            8u16 => "Policy",
            9u16 => "Desc",
            10u16 => "Id",
            11u16 => "Timeout",
            12u16 => "GcInterval",
            13u16 => "Userdata",
            14u16 => "Pad",
            15u16 => "ObjType",
            16u16 => "Handle",
            17u16 => "Expr",
            18u16 => "Expressions",
            19u16 => "Type",
            20u16 => "Count",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableSetAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSetAttrs<'a> {
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
impl<'a> Iterator for IterableSetAttrs<'a> {
    type Item = Result<SetAttrs<'a>, ErrorContext>;
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
                1u16 => SetAttrs::Table({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => SetAttrs::Name({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => SetAttrs::Flags({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => SetAttrs::KeyType({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => SetAttrs::KeyLen({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => SetAttrs::DataType({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => SetAttrs::DataLen({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => SetAttrs::Policy({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => SetAttrs::Desc({
                    let res = Some(IterableSetDescAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => SetAttrs::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => SetAttrs::Timeout({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => SetAttrs::GcInterval({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => SetAttrs::Userdata({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => SetAttrs::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => SetAttrs::ObjType({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => SetAttrs::Handle({
                    let res = parse_be_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => SetAttrs::Expr({
                    let res = Some(IterableExprAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => SetAttrs::Expressions({
                    let res = Some(IterableSetListAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => SetAttrs::Type({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => SetAttrs::Count({
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
            "SetAttrs",
            r#type.and_then(|t| SetAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableSetAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("SetAttrs");
        let mut iter = IterableSetAttrs::with_loc(&[], self.orig_loc);
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
                SetAttrs::Table(val) => fmt.field("Table", &val),
                SetAttrs::Name(val) => fmt.field("Name", &val),
                SetAttrs::Flags(val) => {
                    fmt.field("Flags", &FormatFlags(val.into(), SetFlags::from_value))
                }
                SetAttrs::KeyType(val) => fmt.field("KeyType", &val),
                SetAttrs::KeyLen(val) => fmt.field("KeyLen", &val),
                SetAttrs::DataType(val) => fmt.field("DataType", &val),
                SetAttrs::DataLen(val) => fmt.field("DataLen", &val),
                SetAttrs::Policy(val) => fmt.field("Policy", &val),
                SetAttrs::Desc(val) => fmt.field("Desc", &val),
                SetAttrs::Id(val) => fmt.field("Id", &val),
                SetAttrs::Timeout(val) => fmt.field("Timeout", &val),
                SetAttrs::GcInterval(val) => fmt.field("GcInterval", &val),
                SetAttrs::Userdata(val) => fmt.field("Userdata", &FormatHexdump(val)),
                SetAttrs::Pad(val) => fmt.field("Pad", &val),
                SetAttrs::ObjType(val) => fmt.field("ObjType", &val),
                SetAttrs::Handle(val) => fmt.field("Handle", &val),
                SetAttrs::Expr(val) => fmt.field("Expr", &val),
                SetAttrs::Expressions(val) => fmt.field("Expressions", &val),
                SetAttrs::Type(val) => fmt.field("Type", &val),
                SetAttrs::Count(val) => fmt.field("Count", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSetAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("SetAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| SetAttrs::attr_from_type(t)),
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
                SetAttrs::Table(val) => {
                    if last_off == offset {
                        stack.push(("Table", last_off));
                        break;
                    }
                }
                SetAttrs::Name(val) => {
                    if last_off == offset {
                        stack.push(("Name", last_off));
                        break;
                    }
                }
                SetAttrs::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                SetAttrs::KeyType(val) => {
                    if last_off == offset {
                        stack.push(("KeyType", last_off));
                        break;
                    }
                }
                SetAttrs::KeyLen(val) => {
                    if last_off == offset {
                        stack.push(("KeyLen", last_off));
                        break;
                    }
                }
                SetAttrs::DataType(val) => {
                    if last_off == offset {
                        stack.push(("DataType", last_off));
                        break;
                    }
                }
                SetAttrs::DataLen(val) => {
                    if last_off == offset {
                        stack.push(("DataLen", last_off));
                        break;
                    }
                }
                SetAttrs::Policy(val) => {
                    if last_off == offset {
                        stack.push(("Policy", last_off));
                        break;
                    }
                }
                SetAttrs::Desc(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                SetAttrs::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                SetAttrs::Timeout(val) => {
                    if last_off == offset {
                        stack.push(("Timeout", last_off));
                        break;
                    }
                }
                SetAttrs::GcInterval(val) => {
                    if last_off == offset {
                        stack.push(("GcInterval", last_off));
                        break;
                    }
                }
                SetAttrs::Userdata(val) => {
                    if last_off == offset {
                        stack.push(("Userdata", last_off));
                        break;
                    }
                }
                SetAttrs::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                SetAttrs::ObjType(val) => {
                    if last_off == offset {
                        stack.push(("ObjType", last_off));
                        break;
                    }
                }
                SetAttrs::Handle(val) => {
                    if last_off == offset {
                        stack.push(("Handle", last_off));
                        break;
                    }
                }
                SetAttrs::Expr(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                SetAttrs::Expressions(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                SetAttrs::Type(val) => {
                    if last_off == offset {
                        stack.push(("Type", last_off));
                        break;
                    }
                }
                SetAttrs::Count(val) => {
                    if last_off == offset {
                        stack.push(("Count", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("SetAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum SetDescAttrs<'a> {
    #[doc = "number of elements in set\n"]
    Size(u32),
    #[doc = "description of field concatenation\n\nAttribute may repeat multiple times (treat it as array)"]
    Concat(IterableSetDescConcatAttrs<'a>),
}
impl<'a> IterableSetDescAttrs<'a> {
    #[doc = "number of elements in set\n"]
    pub fn get_size(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetDescAttrs::Size(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetDescAttrs",
            "Size",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "description of field concatenation\n\nAttribute may repeat multiple times (treat it as array)"]
    pub fn get_concat(
        &self,
    ) -> MultiAttrIterable<Self, SetDescAttrs<'a>, IterableSetDescConcatAttrs<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let SetDescAttrs::Concat(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl SetDescAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableSetDescAttrs<'a> {
        IterableSetDescAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Size",
            2u16 => "Concat",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableSetDescAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSetDescAttrs<'a> {
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
impl<'a> Iterator for IterableSetDescAttrs<'a> {
    type Item = Result<SetDescAttrs<'a>, ErrorContext>;
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
                1u16 => SetDescAttrs::Size({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => SetDescAttrs::Concat({
                    let res = Some(IterableSetDescConcatAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "SetDescAttrs",
            r#type.and_then(|t| SetDescAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableSetDescAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("SetDescAttrs");
        let mut iter = IterableSetDescAttrs::with_loc(&[], self.orig_loc);
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
                SetDescAttrs::Size(val) => fmt.field("Size", &val),
                SetDescAttrs::Concat(val) => fmt.field("Concat", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSetDescAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("SetDescAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| SetDescAttrs::attr_from_type(t)),
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
                SetDescAttrs::Size(val) => {
                    if last_off == offset {
                        stack.push(("Size", last_off));
                        break;
                    }
                }
                SetDescAttrs::Concat(val) => {
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
            stack.push(("SetDescAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum SetDescConcatAttrs<'a> {
    Elem(IterableSetFieldAttrs<'a>),
}
impl<'a> IterableSetDescConcatAttrs<'a> {
    pub fn get_elem(&self) -> Result<IterableSetFieldAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetDescConcatAttrs::Elem(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetDescConcatAttrs",
            "Elem",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl SetDescConcatAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableSetDescConcatAttrs<'a> {
        IterableSetDescConcatAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Elem",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableSetDescConcatAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSetDescConcatAttrs<'a> {
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
impl<'a> Iterator for IterableSetDescConcatAttrs<'a> {
    type Item = Result<SetDescConcatAttrs<'a>, ErrorContext>;
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
                1u16 => SetDescConcatAttrs::Elem({
                    let res = Some(IterableSetFieldAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "SetDescConcatAttrs",
            r#type.and_then(|t| SetDescConcatAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableSetDescConcatAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("SetDescConcatAttrs");
        let mut iter = IterableSetDescConcatAttrs::with_loc(&[], self.orig_loc);
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
                SetDescConcatAttrs::Elem(val) => fmt.field("Elem", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSetDescConcatAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("SetDescConcatAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| SetDescConcatAttrs::attr_from_type(t)),
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
                SetDescConcatAttrs::Elem(val) => {
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
            stack.push(("SetDescConcatAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum SetFieldAttrs {
    Len(u32),
}
impl<'a> IterableSetFieldAttrs<'a> {
    pub fn get_len(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetFieldAttrs::Len(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetFieldAttrs",
            "Len",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl SetFieldAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableSetFieldAttrs<'a> {
        IterableSetFieldAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Len",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableSetFieldAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSetFieldAttrs<'a> {
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
impl<'a> Iterator for IterableSetFieldAttrs<'a> {
    type Item = Result<SetFieldAttrs, ErrorContext>;
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
                1u16 => SetFieldAttrs::Len({
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
            "SetFieldAttrs",
            r#type.and_then(|t| SetFieldAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableSetFieldAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("SetFieldAttrs");
        let mut iter = IterableSetFieldAttrs::with_loc(&[], self.orig_loc);
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
                SetFieldAttrs::Len(val) => fmt.field("Len", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSetFieldAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("SetFieldAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| SetFieldAttrs::attr_from_type(t)),
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
                SetFieldAttrs::Len(val) => {
                    if last_off == offset {
                        stack.push(("Len", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("SetFieldAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum SetListAttrs<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    Elem(IterableExprAttrs<'a>),
}
impl<'a> IterableSetListAttrs<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_elem(&self) -> MultiAttrIterable<Self, SetListAttrs<'a>, IterableExprAttrs<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let SetListAttrs::Elem(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl SetListAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableSetListAttrs<'a> {
        IterableSetListAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Elem",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableSetListAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSetListAttrs<'a> {
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
impl<'a> Iterator for IterableSetListAttrs<'a> {
    type Item = Result<SetListAttrs<'a>, ErrorContext>;
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
                1u16 => SetListAttrs::Elem({
                    let res = Some(IterableExprAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "SetListAttrs",
            r#type.and_then(|t| SetListAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableSetListAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("SetListAttrs");
        let mut iter = IterableSetListAttrs::with_loc(&[], self.orig_loc);
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
                SetListAttrs::Elem(val) => fmt.field("Elem", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSetListAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("SetListAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| SetListAttrs::attr_from_type(t)),
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
                SetListAttrs::Elem(val) => {
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
            stack.push(("SetListAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum SetelemAttrs<'a> {
    #[doc = "key value\n"]
    Key(IterableDataAttrs<'a>),
    #[doc = "data value of mapping\n"]
    Data(IterableDataAttrs<'a>),
    #[doc = "bitmask of nft_set_elem_flags\n"]
    Flags(&'a [u8]),
    #[doc = "timeout value\n"]
    Timeout(u64),
    #[doc = "expiration time\n"]
    Expiration(u64),
    #[doc = "user data\n"]
    Userdata(&'a [u8]),
    #[doc = "expression\n"]
    Expr(IterableExprAttrs<'a>),
    #[doc = "stateful object reference\n"]
    Objref(&'a CStr),
    #[doc = "closing key value\n"]
    KeyEnd(IterableDataAttrs<'a>),
    #[doc = "list of expressions\n"]
    Expressions(IterableExprListAttrs<'a>),
}
impl<'a> IterableSetelemAttrs<'a> {
    #[doc = "key value\n"]
    pub fn get_key(&self) -> Result<IterableDataAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetelemAttrs::Key(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetelemAttrs",
            "Key",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "data value of mapping\n"]
    pub fn get_data(&self) -> Result<IterableDataAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetelemAttrs::Data(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetelemAttrs",
            "Data",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "bitmask of nft_set_elem_flags\n"]
    pub fn get_flags(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetelemAttrs::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetelemAttrs",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "timeout value\n"]
    pub fn get_timeout(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetelemAttrs::Timeout(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetelemAttrs",
            "Timeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "expiration time\n"]
    pub fn get_expiration(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetelemAttrs::Expiration(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetelemAttrs",
            "Expiration",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "user data\n"]
    pub fn get_userdata(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetelemAttrs::Userdata(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetelemAttrs",
            "Userdata",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "expression\n"]
    pub fn get_expr(&self) -> Result<IterableExprAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetelemAttrs::Expr(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetelemAttrs",
            "Expr",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "stateful object reference\n"]
    pub fn get_objref(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetelemAttrs::Objref(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetelemAttrs",
            "Objref",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "closing key value\n"]
    pub fn get_key_end(&self) -> Result<IterableDataAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetelemAttrs::KeyEnd(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetelemAttrs",
            "KeyEnd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "list of expressions\n"]
    pub fn get_expressions(&self) -> Result<IterableExprListAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetelemAttrs::Expressions(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetelemAttrs",
            "Expressions",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl SetelemAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableSetelemAttrs<'a> {
        IterableSetelemAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Key",
            2u16 => "Data",
            3u16 => "Flags",
            4u16 => "Timeout",
            5u16 => "Expiration",
            6u16 => "Userdata",
            7u16 => "Expr",
            8u16 => "Objref",
            9u16 => "KeyEnd",
            10u16 => "Expressions",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableSetelemAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSetelemAttrs<'a> {
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
impl<'a> Iterator for IterableSetelemAttrs<'a> {
    type Item = Result<SetelemAttrs<'a>, ErrorContext>;
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
                1u16 => SetelemAttrs::Key({
                    let res = Some(IterableDataAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => SetelemAttrs::Data({
                    let res = Some(IterableDataAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => SetelemAttrs::Flags({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => SetelemAttrs::Timeout({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => SetelemAttrs::Expiration({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => SetelemAttrs::Userdata({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => SetelemAttrs::Expr({
                    let res = Some(IterableExprAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => SetelemAttrs::Objref({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => SetelemAttrs::KeyEnd({
                    let res = Some(IterableDataAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => SetelemAttrs::Expressions({
                    let res = Some(IterableExprListAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "SetelemAttrs",
            r#type.and_then(|t| SetelemAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableSetelemAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("SetelemAttrs");
        let mut iter = IterableSetelemAttrs::with_loc(&[], self.orig_loc);
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
                SetelemAttrs::Key(val) => fmt.field("Key", &val),
                SetelemAttrs::Data(val) => fmt.field("Data", &val),
                SetelemAttrs::Flags(val) => fmt.field("Flags", &FormatHexdump(val)),
                SetelemAttrs::Timeout(val) => fmt.field("Timeout", &val),
                SetelemAttrs::Expiration(val) => fmt.field("Expiration", &val),
                SetelemAttrs::Userdata(val) => fmt.field("Userdata", &FormatHexdump(val)),
                SetelemAttrs::Expr(val) => fmt.field("Expr", &val),
                SetelemAttrs::Objref(val) => fmt.field("Objref", &val),
                SetelemAttrs::KeyEnd(val) => fmt.field("KeyEnd", &val),
                SetelemAttrs::Expressions(val) => fmt.field("Expressions", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSetelemAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("SetelemAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| SetelemAttrs::attr_from_type(t)),
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
                SetelemAttrs::Key(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                SetelemAttrs::Data(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                SetelemAttrs::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                SetelemAttrs::Timeout(val) => {
                    if last_off == offset {
                        stack.push(("Timeout", last_off));
                        break;
                    }
                }
                SetelemAttrs::Expiration(val) => {
                    if last_off == offset {
                        stack.push(("Expiration", last_off));
                        break;
                    }
                }
                SetelemAttrs::Userdata(val) => {
                    if last_off == offset {
                        stack.push(("Userdata", last_off));
                        break;
                    }
                }
                SetelemAttrs::Expr(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                SetelemAttrs::Objref(val) => {
                    if last_off == offset {
                        stack.push(("Objref", last_off));
                        break;
                    }
                }
                SetelemAttrs::KeyEnd(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                SetelemAttrs::Expressions(val) => {
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
            stack.push(("SetelemAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum SetelemListElemAttrs<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    Elem(IterableSetelemAttrs<'a>),
}
impl<'a> IterableSetelemListElemAttrs<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_elem(
        &self,
    ) -> MultiAttrIterable<Self, SetelemListElemAttrs<'a>, IterableSetelemAttrs<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let SetelemListElemAttrs::Elem(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl SetelemListElemAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableSetelemListElemAttrs<'a> {
        IterableSetelemListElemAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Elem",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableSetelemListElemAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSetelemListElemAttrs<'a> {
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
impl<'a> Iterator for IterableSetelemListElemAttrs<'a> {
    type Item = Result<SetelemListElemAttrs<'a>, ErrorContext>;
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
                1u16 => SetelemListElemAttrs::Elem({
                    let res = Some(IterableSetelemAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "SetelemListElemAttrs",
            r#type.and_then(|t| SetelemListElemAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableSetelemListElemAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("SetelemListElemAttrs");
        let mut iter = IterableSetelemListElemAttrs::with_loc(&[], self.orig_loc);
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
                SetelemListElemAttrs::Elem(val) => fmt.field("Elem", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSetelemListElemAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("SetelemListElemAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| SetelemListElemAttrs::attr_from_type(t)),
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
                SetelemListElemAttrs::Elem(val) => {
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
            stack.push(("SetelemListElemAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum SetelemListAttrs<'a> {
    Table(&'a CStr),
    Set(&'a CStr),
    Elements(IterableSetelemListElemAttrs<'a>),
    SetId(u32),
}
impl<'a> IterableSetelemListAttrs<'a> {
    pub fn get_table(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetelemListAttrs::Table(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetelemListAttrs",
            "Table",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_set(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetelemListAttrs::Set(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetelemListAttrs",
            "Set",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_elements(&self) -> Result<IterableSetelemListElemAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetelemListAttrs::Elements(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetelemListAttrs",
            "Elements",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_set_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SetelemListAttrs::SetId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SetelemListAttrs",
            "SetId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl SetelemListAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableSetelemListAttrs<'a> {
        IterableSetelemListAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Table",
            2u16 => "Set",
            3u16 => "Elements",
            4u16 => "SetId",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableSetelemListAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSetelemListAttrs<'a> {
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
impl<'a> Iterator for IterableSetelemListAttrs<'a> {
    type Item = Result<SetelemListAttrs<'a>, ErrorContext>;
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
                1u16 => SetelemListAttrs::Table({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => SetelemListAttrs::Set({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => SetelemListAttrs::Elements({
                    let res = Some(IterableSetelemListElemAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => SetelemListAttrs::SetId({
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
            "SetelemListAttrs",
            r#type.and_then(|t| SetelemListAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableSetelemListAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("SetelemListAttrs");
        let mut iter = IterableSetelemListAttrs::with_loc(&[], self.orig_loc);
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
                SetelemListAttrs::Table(val) => fmt.field("Table", &val),
                SetelemListAttrs::Set(val) => fmt.field("Set", &val),
                SetelemListAttrs::Elements(val) => fmt.field("Elements", &val),
                SetelemListAttrs::SetId(val) => fmt.field("SetId", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSetelemListAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("SetelemListAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| SetelemListAttrs::attr_from_type(t)),
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
                SetelemListAttrs::Table(val) => {
                    if last_off == offset {
                        stack.push(("Table", last_off));
                        break;
                    }
                }
                SetelemListAttrs::Set(val) => {
                    if last_off == offset {
                        stack.push(("Set", last_off));
                        break;
                    }
                }
                SetelemListAttrs::Elements(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                SetelemListAttrs::SetId(val) => {
                    if last_off == offset {
                        stack.push(("SetId", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("SetelemListAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum GenAttrs<'a> {
    #[doc = "ruleset generation id\n"]
    Id(u32),
    ProcPid(u32),
    ProcName(&'a CStr),
}
impl<'a> IterableGenAttrs<'a> {
    #[doc = "ruleset generation id\n"]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(GenAttrs::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "GenAttrs",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_proc_pid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(GenAttrs::ProcPid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "GenAttrs",
            "ProcPid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_proc_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(GenAttrs::ProcName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "GenAttrs",
            "ProcName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl GenAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableGenAttrs<'a> {
        IterableGenAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Id",
            2u16 => "ProcPid",
            3u16 => "ProcName",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableGenAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableGenAttrs<'a> {
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
impl<'a> Iterator for IterableGenAttrs<'a> {
    type Item = Result<GenAttrs<'a>, ErrorContext>;
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
                1u16 => GenAttrs::Id({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => GenAttrs::ProcPid({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => GenAttrs::ProcName({
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
            "GenAttrs",
            r#type.and_then(|t| GenAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableGenAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("GenAttrs");
        let mut iter = IterableGenAttrs::with_loc(&[], self.orig_loc);
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
                GenAttrs::Id(val) => fmt.field("Id", &val),
                GenAttrs::ProcPid(val) => fmt.field("ProcPid", &val),
                GenAttrs::ProcName(val) => fmt.field("ProcName", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableGenAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("GenAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| GenAttrs::attr_from_type(t)),
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
                GenAttrs::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                GenAttrs::ProcPid(val) => {
                    if last_off == offset {
                        stack.push(("ProcPid", last_off));
                        break;
                    }
                }
                GenAttrs::ProcName(val) => {
                    if last_off == offset {
                        stack.push(("ProcName", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("GenAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ObjAttrs<'a> {
    #[doc = "name of the table containing the expression\n"]
    Table(&'a CStr),
    #[doc = "name of this expression type\n"]
    Name(&'a CStr),
    #[doc = "stateful object type\n\nAssociated type: [`ObjectType`] (enum)"]
    Type(u32),
    #[doc = "stateful object data\n"]
    Data(ObjData<'a>),
    #[doc = "number of references to this expression\n"]
    Use(u32),
    #[doc = "object handle\n"]
    Handle(u64),
    Pad(&'a [u8]),
    #[doc = "user data\n"]
    Userdata(&'a [u8]),
}
impl<'a> IterableObjAttrs<'a> {
    #[doc = "name of the table containing the expression\n"]
    pub fn get_table(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ObjAttrs::Table(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ObjAttrs",
            "Table",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "name of this expression type\n"]
    pub fn get_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ObjAttrs::Name(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ObjAttrs",
            "Name",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "stateful object type\n\nAssociated type: [`ObjectType`] (enum)"]
    pub fn get_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ObjAttrs::Type(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ObjAttrs",
            "Type",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "stateful object data\n"]
    pub fn get_data(&self) -> Result<ObjData<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ObjAttrs::Data(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ObjAttrs",
            "Data",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "number of references to this expression\n"]
    pub fn get_use(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ObjAttrs::Use(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ObjAttrs",
            "Use",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "object handle\n"]
    pub fn get_handle(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ObjAttrs::Handle(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ObjAttrs",
            "Handle",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ObjAttrs::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ObjAttrs",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "user data\n"]
    pub fn get_userdata(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ObjAttrs::Userdata(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ObjAttrs",
            "Userdata",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
#[derive(Debug, Clone)]
pub enum ObjData<'a> {
    Counter(IterableCounterAttrs<'a>),
    Quota(IterableQuotaAttrs<'a>),
}
impl<'a> ObjData<'a> {
    fn select_with_loc(selector: u32, buf: &'a [u8], loc: usize) -> Option<Self> {
        match selector {
            val if val == ObjectType::Counter as u32 => {
                Some(ObjData::Counter(IterableCounterAttrs::with_loc(buf, loc)))
            }
            val if val == ObjectType::Quota as u32 => {
                Some(ObjData::Quota(IterableQuotaAttrs::with_loc(buf, loc)))
            }
            _ => None,
        }
    }
}
impl ObjAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableObjAttrs<'a> {
        IterableObjAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Table",
            2u16 => "Name",
            3u16 => "Type",
            4u16 => "Data",
            5u16 => "Use",
            6u16 => "Handle",
            7u16 => "Pad",
            8u16 => "Userdata",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableObjAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
    selector_type: Option<u32>,
}
impl<'a> IterableObjAttrs<'a> {
    fn with_loc(buf: &'a [u8], orig_loc: usize) -> Self {
        Self {
            buf,
            pos: 0,
            orig_loc,
            selector_type: None,
        }
    }
    pub fn get_buf(&self) -> &'a [u8] {
        self.buf
    }
}
impl<'a> Iterator for IterableObjAttrs<'a> {
    type Item = Result<ObjAttrs<'a>, ErrorContext>;
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
                1u16 => ObjAttrs::Table({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ObjAttrs::Name({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ObjAttrs::Type({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => ObjAttrs::Data({
                    let res = {
                        let Some(selector) = self.selector_type else {
                            break;
                        };
                        match ObjData::select_with_loc(selector, next, self.orig_loc) {
                            Some(sub) => Some(sub),
                            None if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                            None => continue,
                        }
                    };
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => ObjAttrs::Use({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => ObjAttrs::Handle({
                    let res = parse_be_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => ObjAttrs::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => ObjAttrs::Userdata({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            if let ObjAttrs::Type(sel) = &res {
                self.selector_type = Some(*sel);
            }
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "ObjAttrs",
            r#type.and_then(|t| ObjAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableObjAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ObjAttrs");
        let mut iter = IterableObjAttrs::with_loc(&[], self.orig_loc);
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
                ObjAttrs::Table(val) => fmt.field("Table", &val),
                ObjAttrs::Name(val) => fmt.field("Name", &val),
                ObjAttrs::Type(val) => {
                    fmt.field("Type", &FormatEnum(val.into(), ObjectType::from_value))
                }
                ObjAttrs::Data(val) => fmt.field("Data", &val),
                ObjAttrs::Use(val) => fmt.field("Use", &val),
                ObjAttrs::Handle(val) => fmt.field("Handle", &val),
                ObjAttrs::Pad(val) => fmt.field("Pad", &val),
                ObjAttrs::Userdata(val) => fmt.field("Userdata", &FormatHexdump(val)),
            };
        }
        fmt.finish()
    }
}
impl IterableObjAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ObjAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ObjAttrs::attr_from_type(t)),
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
                ObjAttrs::Table(val) => {
                    if last_off == offset {
                        stack.push(("Table", last_off));
                        break;
                    }
                }
                ObjAttrs::Name(val) => {
                    if last_off == offset {
                        stack.push(("Name", last_off));
                        break;
                    }
                }
                ObjAttrs::Type(val) => {
                    if last_off == offset {
                        stack.push(("Type", last_off));
                        break;
                    }
                }
                ObjAttrs::Data(val) => {
                    if last_off == offset {
                        stack.push(("Data", last_off));
                        break;
                    }
                }
                ObjAttrs::Use(val) => {
                    if last_off == offset {
                        stack.push(("Use", last_off));
                        break;
                    }
                }
                ObjAttrs::Handle(val) => {
                    if last_off == offset {
                        stack.push(("Handle", last_off));
                        break;
                    }
                }
                ObjAttrs::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                ObjAttrs::Userdata(val) => {
                    if last_off == offset {
                        stack.push(("Userdata", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ObjAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum QuotaAttrs<'a> {
    Bytes(u64),
    #[doc = "Associated type: [`QuotaFlags`] (enum)"]
    Flags(u32),
    Pad(&'a [u8]),
    Consumed(u64),
}
impl<'a> IterableQuotaAttrs<'a> {
    pub fn get_bytes(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(QuotaAttrs::Bytes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "QuotaAttrs",
            "Bytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`QuotaFlags`] (enum)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(QuotaAttrs::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "QuotaAttrs",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(QuotaAttrs::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "QuotaAttrs",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_consumed(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(QuotaAttrs::Consumed(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "QuotaAttrs",
            "Consumed",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl QuotaAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableQuotaAttrs<'a> {
        IterableQuotaAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Bytes",
            2u16 => "Flags",
            3u16 => "Pad",
            4u16 => "Consumed",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableQuotaAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableQuotaAttrs<'a> {
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
impl<'a> Iterator for IterableQuotaAttrs<'a> {
    type Item = Result<QuotaAttrs<'a>, ErrorContext>;
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
                1u16 => QuotaAttrs::Bytes({
                    let res = parse_be_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => QuotaAttrs::Flags({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => QuotaAttrs::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => QuotaAttrs::Consumed({
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
            "QuotaAttrs",
            r#type.and_then(|t| QuotaAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableQuotaAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("QuotaAttrs");
        let mut iter = IterableQuotaAttrs::with_loc(&[], self.orig_loc);
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
                QuotaAttrs::Bytes(val) => fmt.field("Bytes", &val),
                QuotaAttrs::Flags(val) => {
                    fmt.field("Flags", &FormatFlags(val.into(), QuotaFlags::from_value))
                }
                QuotaAttrs::Pad(val) => fmt.field("Pad", &val),
                QuotaAttrs::Consumed(val) => fmt.field("Consumed", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableQuotaAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("QuotaAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| QuotaAttrs::attr_from_type(t)),
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
                QuotaAttrs::Bytes(val) => {
                    if last_off == offset {
                        stack.push(("Bytes", last_off));
                        break;
                    }
                }
                QuotaAttrs::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                QuotaAttrs::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                QuotaAttrs::Consumed(val) => {
                    if last_off == offset {
                        stack.push(("Consumed", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("QuotaAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum FlowtableAttrs<'a> {
    Table(&'a CStr),
    Name(&'a CStr),
    Hook(IterableFlowtableHookAttrs<'a>),
    Use(u32),
    Handle(u64),
    Pad(&'a [u8]),
    Flags(u32),
}
impl<'a> IterableFlowtableAttrs<'a> {
    pub fn get_table(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FlowtableAttrs::Table(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FlowtableAttrs",
            "Table",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FlowtableAttrs::Name(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FlowtableAttrs",
            "Name",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_hook(&self) -> Result<IterableFlowtableHookAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FlowtableAttrs::Hook(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FlowtableAttrs",
            "Hook",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_use(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FlowtableAttrs::Use(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FlowtableAttrs",
            "Use",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_handle(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FlowtableAttrs::Handle(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FlowtableAttrs",
            "Handle",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FlowtableAttrs::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FlowtableAttrs",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FlowtableAttrs::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FlowtableAttrs",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl FlowtableAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableFlowtableAttrs<'a> {
        IterableFlowtableAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Table",
            2u16 => "Name",
            3u16 => "Hook",
            4u16 => "Use",
            5u16 => "Handle",
            6u16 => "Pad",
            7u16 => "Flags",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableFlowtableAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableFlowtableAttrs<'a> {
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
impl<'a> Iterator for IterableFlowtableAttrs<'a> {
    type Item = Result<FlowtableAttrs<'a>, ErrorContext>;
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
                1u16 => FlowtableAttrs::Table({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => FlowtableAttrs::Name({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => FlowtableAttrs::Hook({
                    let res = Some(IterableFlowtableHookAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => FlowtableAttrs::Use({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => FlowtableAttrs::Handle({
                    let res = parse_be_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => FlowtableAttrs::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => FlowtableAttrs::Flags({
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
            "FlowtableAttrs",
            r#type.and_then(|t| FlowtableAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableFlowtableAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("FlowtableAttrs");
        let mut iter = IterableFlowtableAttrs::with_loc(&[], self.orig_loc);
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
                FlowtableAttrs::Table(val) => fmt.field("Table", &val),
                FlowtableAttrs::Name(val) => fmt.field("Name", &val),
                FlowtableAttrs::Hook(val) => fmt.field("Hook", &val),
                FlowtableAttrs::Use(val) => fmt.field("Use", &val),
                FlowtableAttrs::Handle(val) => fmt.field("Handle", &val),
                FlowtableAttrs::Pad(val) => fmt.field("Pad", &val),
                FlowtableAttrs::Flags(val) => fmt.field("Flags", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableFlowtableAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("FlowtableAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| FlowtableAttrs::attr_from_type(t)),
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
                FlowtableAttrs::Table(val) => {
                    if last_off == offset {
                        stack.push(("Table", last_off));
                        break;
                    }
                }
                FlowtableAttrs::Name(val) => {
                    if last_off == offset {
                        stack.push(("Name", last_off));
                        break;
                    }
                }
                FlowtableAttrs::Hook(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                FlowtableAttrs::Use(val) => {
                    if last_off == offset {
                        stack.push(("Use", last_off));
                        break;
                    }
                }
                FlowtableAttrs::Handle(val) => {
                    if last_off == offset {
                        stack.push(("Handle", last_off));
                        break;
                    }
                }
                FlowtableAttrs::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                FlowtableAttrs::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("FlowtableAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum FlowtableHookAttrs<'a> {
    Num(u32),
    Priority(u32),
    Devs(IterableHookDevAttrs<'a>),
}
impl<'a> IterableFlowtableHookAttrs<'a> {
    pub fn get_num(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FlowtableHookAttrs::Num(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FlowtableHookAttrs",
            "Num",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_priority(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FlowtableHookAttrs::Priority(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FlowtableHookAttrs",
            "Priority",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_devs(&self) -> Result<IterableHookDevAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FlowtableHookAttrs::Devs(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FlowtableHookAttrs",
            "Devs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl FlowtableHookAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableFlowtableHookAttrs<'a> {
        IterableFlowtableHookAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Num",
            2u16 => "Priority",
            3u16 => "Devs",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableFlowtableHookAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableFlowtableHookAttrs<'a> {
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
impl<'a> Iterator for IterableFlowtableHookAttrs<'a> {
    type Item = Result<FlowtableHookAttrs<'a>, ErrorContext>;
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
                1u16 => FlowtableHookAttrs::Num({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => FlowtableHookAttrs::Priority({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => FlowtableHookAttrs::Devs({
                    let res = Some(IterableHookDevAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "FlowtableHookAttrs",
            r#type.and_then(|t| FlowtableHookAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableFlowtableHookAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("FlowtableHookAttrs");
        let mut iter = IterableFlowtableHookAttrs::with_loc(&[], self.orig_loc);
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
                FlowtableHookAttrs::Num(val) => fmt.field("Num", &val),
                FlowtableHookAttrs::Priority(val) => fmt.field("Priority", &val),
                FlowtableHookAttrs::Devs(val) => fmt.field("Devs", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableFlowtableHookAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("FlowtableHookAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| FlowtableHookAttrs::attr_from_type(t)),
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
                FlowtableHookAttrs::Num(val) => {
                    if last_off == offset {
                        stack.push(("Num", last_off));
                        break;
                    }
                }
                FlowtableHookAttrs::Priority(val) => {
                    if last_off == offset {
                        stack.push(("Priority", last_off));
                        break;
                    }
                }
                FlowtableHookAttrs::Devs(val) => {
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
            stack.push(("FlowtableHookAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum ExprBitwiseAttrs<'a> {
    Sreg(u32),
    Dreg(u32),
    Len(u32),
    Mask(IterableDataAttrs<'a>),
    Xor(IterableDataAttrs<'a>),
    #[doc = "Associated type: [`BitwiseOps`] (enum)"]
    Op(u32),
    Data(IterableDataAttrs<'a>),
}
impl<'a> IterableExprBitwiseAttrs<'a> {
    pub fn get_sreg(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprBitwiseAttrs::Sreg(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprBitwiseAttrs",
            "Sreg",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dreg(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprBitwiseAttrs::Dreg(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprBitwiseAttrs",
            "Dreg",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_len(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprBitwiseAttrs::Len(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprBitwiseAttrs",
            "Len",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mask(&self) -> Result<IterableDataAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprBitwiseAttrs::Mask(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprBitwiseAttrs",
            "Mask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_xor(&self) -> Result<IterableDataAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprBitwiseAttrs::Xor(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprBitwiseAttrs",
            "Xor",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`BitwiseOps`] (enum)"]
    pub fn get_op(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprBitwiseAttrs::Op(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprBitwiseAttrs",
            "Op",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_data(&self) -> Result<IterableDataAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprBitwiseAttrs::Data(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprBitwiseAttrs",
            "Data",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ExprBitwiseAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableExprBitwiseAttrs<'a> {
        IterableExprBitwiseAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Sreg",
            2u16 => "Dreg",
            3u16 => "Len",
            4u16 => "Mask",
            5u16 => "Xor",
            6u16 => "Op",
            7u16 => "Data",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableExprBitwiseAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableExprBitwiseAttrs<'a> {
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
impl<'a> Iterator for IterableExprBitwiseAttrs<'a> {
    type Item = Result<ExprBitwiseAttrs<'a>, ErrorContext>;
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
                1u16 => ExprBitwiseAttrs::Sreg({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ExprBitwiseAttrs::Dreg({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ExprBitwiseAttrs::Len({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => ExprBitwiseAttrs::Mask({
                    let res = Some(IterableDataAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => ExprBitwiseAttrs::Xor({
                    let res = Some(IterableDataAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => ExprBitwiseAttrs::Op({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => ExprBitwiseAttrs::Data({
                    let res = Some(IterableDataAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "ExprBitwiseAttrs",
            r#type.and_then(|t| ExprBitwiseAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableExprBitwiseAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ExprBitwiseAttrs");
        let mut iter = IterableExprBitwiseAttrs::with_loc(&[], self.orig_loc);
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
                ExprBitwiseAttrs::Sreg(val) => fmt.field("Sreg", &val),
                ExprBitwiseAttrs::Dreg(val) => fmt.field("Dreg", &val),
                ExprBitwiseAttrs::Len(val) => fmt.field("Len", &val),
                ExprBitwiseAttrs::Mask(val) => fmt.field("Mask", &val),
                ExprBitwiseAttrs::Xor(val) => fmt.field("Xor", &val),
                ExprBitwiseAttrs::Op(val) => {
                    fmt.field("Op", &FormatEnum(val.into(), BitwiseOps::from_value))
                }
                ExprBitwiseAttrs::Data(val) => fmt.field("Data", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableExprBitwiseAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ExprBitwiseAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ExprBitwiseAttrs::attr_from_type(t)),
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
                ExprBitwiseAttrs::Sreg(val) => {
                    if last_off == offset {
                        stack.push(("Sreg", last_off));
                        break;
                    }
                }
                ExprBitwiseAttrs::Dreg(val) => {
                    if last_off == offset {
                        stack.push(("Dreg", last_off));
                        break;
                    }
                }
                ExprBitwiseAttrs::Len(val) => {
                    if last_off == offset {
                        stack.push(("Len", last_off));
                        break;
                    }
                }
                ExprBitwiseAttrs::Mask(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ExprBitwiseAttrs::Xor(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ExprBitwiseAttrs::Op(val) => {
                    if last_off == offset {
                        stack.push(("Op", last_off));
                        break;
                    }
                }
                ExprBitwiseAttrs::Data(val) => {
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
            stack.push(("ExprBitwiseAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum ExprCmpAttrs<'a> {
    Sreg(u32),
    #[doc = "Associated type: [`CmpOps`] (enum)"]
    Op(u32),
    Data(IterableDataAttrs<'a>),
}
impl<'a> IterableExprCmpAttrs<'a> {
    pub fn get_sreg(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprCmpAttrs::Sreg(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprCmpAttrs",
            "Sreg",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`CmpOps`] (enum)"]
    pub fn get_op(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprCmpAttrs::Op(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprCmpAttrs",
            "Op",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_data(&self) -> Result<IterableDataAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprCmpAttrs::Data(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprCmpAttrs",
            "Data",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ExprCmpAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableExprCmpAttrs<'a> {
        IterableExprCmpAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Sreg",
            2u16 => "Op",
            3u16 => "Data",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableExprCmpAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableExprCmpAttrs<'a> {
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
impl<'a> Iterator for IterableExprCmpAttrs<'a> {
    type Item = Result<ExprCmpAttrs<'a>, ErrorContext>;
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
                1u16 => ExprCmpAttrs::Sreg({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ExprCmpAttrs::Op({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ExprCmpAttrs::Data({
                    let res = Some(IterableDataAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "ExprCmpAttrs",
            r#type.and_then(|t| ExprCmpAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableExprCmpAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ExprCmpAttrs");
        let mut iter = IterableExprCmpAttrs::with_loc(&[], self.orig_loc);
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
                ExprCmpAttrs::Sreg(val) => fmt.field("Sreg", &val),
                ExprCmpAttrs::Op(val) => {
                    fmt.field("Op", &FormatEnum(val.into(), CmpOps::from_value))
                }
                ExprCmpAttrs::Data(val) => fmt.field("Data", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableExprCmpAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ExprCmpAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ExprCmpAttrs::attr_from_type(t)),
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
                ExprCmpAttrs::Sreg(val) => {
                    if last_off == offset {
                        stack.push(("Sreg", last_off));
                        break;
                    }
                }
                ExprCmpAttrs::Op(val) => {
                    if last_off == offset {
                        stack.push(("Op", last_off));
                        break;
                    }
                }
                ExprCmpAttrs::Data(val) => {
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
            stack.push(("ExprCmpAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum DataAttrs<'a> {
    Value(&'a [u8]),
    Verdict(IterableVerdictAttrs<'a>),
}
impl<'a> IterableDataAttrs<'a> {
    pub fn get_value(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DataAttrs::Value(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DataAttrs",
            "Value",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_verdict(&self) -> Result<IterableVerdictAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DataAttrs::Verdict(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DataAttrs",
            "Verdict",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DataAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDataAttrs<'a> {
        IterableDataAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Value",
            2u16 => "Verdict",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDataAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDataAttrs<'a> {
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
impl<'a> Iterator for IterableDataAttrs<'a> {
    type Item = Result<DataAttrs<'a>, ErrorContext>;
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
                1u16 => DataAttrs::Value({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => DataAttrs::Verdict({
                    let res = Some(IterableVerdictAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DataAttrs",
            r#type.and_then(|t| DataAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDataAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DataAttrs");
        let mut iter = IterableDataAttrs::with_loc(&[], self.orig_loc);
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
                DataAttrs::Value(val) => fmt.field("Value", &FormatHexdump(val)),
                DataAttrs::Verdict(val) => fmt.field("Verdict", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDataAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DataAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| DataAttrs::attr_from_type(t)),
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
                DataAttrs::Value(val) => {
                    if last_off == offset {
                        stack.push(("Value", last_off));
                        break;
                    }
                }
                DataAttrs::Verdict(val) => {
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
            stack.push(("DataAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum VerdictAttrs<'a> {
    #[doc = "nf_tables verdict\n\nAssociated type: [`VerdictCode`] (enum)"]
    Code(u32),
    #[doc = "jump target chain name\n"]
    Chain(&'a CStr),
    #[doc = "jump target chain ID\n"]
    ChainId(u32),
}
impl<'a> IterableVerdictAttrs<'a> {
    #[doc = "nf_tables verdict\n\nAssociated type: [`VerdictCode`] (enum)"]
    pub fn get_code(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VerdictAttrs::Code(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VerdictAttrs",
            "Code",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "jump target chain name\n"]
    pub fn get_chain(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VerdictAttrs::Chain(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VerdictAttrs",
            "Chain",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "jump target chain ID\n"]
    pub fn get_chain_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VerdictAttrs::ChainId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VerdictAttrs",
            "ChainId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl VerdictAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableVerdictAttrs<'a> {
        IterableVerdictAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Code",
            2u16 => "Chain",
            3u16 => "ChainId",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableVerdictAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableVerdictAttrs<'a> {
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
impl<'a> Iterator for IterableVerdictAttrs<'a> {
    type Item = Result<VerdictAttrs<'a>, ErrorContext>;
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
                1u16 => VerdictAttrs::Code({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => VerdictAttrs::Chain({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => VerdictAttrs::ChainId({
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
            "VerdictAttrs",
            r#type.and_then(|t| VerdictAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableVerdictAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("VerdictAttrs");
        let mut iter = IterableVerdictAttrs::with_loc(&[], self.orig_loc);
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
                VerdictAttrs::Code(val) => {
                    fmt.field("Code", &FormatEnum(val.into(), VerdictCode::from_value))
                }
                VerdictAttrs::Chain(val) => fmt.field("Chain", &val),
                VerdictAttrs::ChainId(val) => fmt.field("ChainId", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableVerdictAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("VerdictAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| VerdictAttrs::attr_from_type(t)),
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
                VerdictAttrs::Code(val) => {
                    if last_off == offset {
                        stack.push(("Code", last_off));
                        break;
                    }
                }
                VerdictAttrs::Chain(val) => {
                    if last_off == offset {
                        stack.push(("Chain", last_off));
                        break;
                    }
                }
                VerdictAttrs::ChainId(val) => {
                    if last_off == offset {
                        stack.push(("ChainId", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("VerdictAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ExprCounterAttrs<'a> {
    #[doc = "Number of bytes\n"]
    Bytes(u64),
    #[doc = "Number of packets\n"]
    Packets(u64),
    Pad(&'a [u8]),
}
impl<'a> IterableExprCounterAttrs<'a> {
    #[doc = "Number of bytes\n"]
    pub fn get_bytes(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprCounterAttrs::Bytes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprCounterAttrs",
            "Bytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of packets\n"]
    pub fn get_packets(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprCounterAttrs::Packets(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprCounterAttrs",
            "Packets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprCounterAttrs::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprCounterAttrs",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ExprCounterAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableExprCounterAttrs<'a> {
        IterableExprCounterAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Bytes",
            2u16 => "Packets",
            3u16 => "Pad",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableExprCounterAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableExprCounterAttrs<'a> {
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
impl<'a> Iterator for IterableExprCounterAttrs<'a> {
    type Item = Result<ExprCounterAttrs<'a>, ErrorContext>;
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
                1u16 => ExprCounterAttrs::Bytes({
                    let res = parse_be_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ExprCounterAttrs::Packets({
                    let res = parse_be_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ExprCounterAttrs::Pad({
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
            "ExprCounterAttrs",
            r#type.and_then(|t| ExprCounterAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableExprCounterAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ExprCounterAttrs");
        let mut iter = IterableExprCounterAttrs::with_loc(&[], self.orig_loc);
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
                ExprCounterAttrs::Bytes(val) => fmt.field("Bytes", &val),
                ExprCounterAttrs::Packets(val) => fmt.field("Packets", &val),
                ExprCounterAttrs::Pad(val) => fmt.field("Pad", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableExprCounterAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ExprCounterAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ExprCounterAttrs::attr_from_type(t)),
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
                ExprCounterAttrs::Bytes(val) => {
                    if last_off == offset {
                        stack.push(("Bytes", last_off));
                        break;
                    }
                }
                ExprCounterAttrs::Packets(val) => {
                    if last_off == offset {
                        stack.push(("Packets", last_off));
                        break;
                    }
                }
                ExprCounterAttrs::Pad(val) => {
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
            stack.push(("ExprCounterAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ExprFibAttrs {
    Dreg(u32),
    #[doc = "Associated type: [`FibResult`] (enum)"]
    Result(u32),
    #[doc = "Associated type: [`FibFlags`] (enum)"]
    Flags(u32),
}
impl<'a> IterableExprFibAttrs<'a> {
    pub fn get_dreg(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprFibAttrs::Dreg(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprFibAttrs",
            "Dreg",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`FibResult`] (enum)"]
    pub fn get_result(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprFibAttrs::Result(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprFibAttrs",
            "Result",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`FibFlags`] (enum)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprFibAttrs::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprFibAttrs",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ExprFibAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableExprFibAttrs<'a> {
        IterableExprFibAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Dreg",
            2u16 => "Result",
            3u16 => "Flags",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableExprFibAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableExprFibAttrs<'a> {
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
impl<'a> Iterator for IterableExprFibAttrs<'a> {
    type Item = Result<ExprFibAttrs, ErrorContext>;
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
                1u16 => ExprFibAttrs::Dreg({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ExprFibAttrs::Result({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ExprFibAttrs::Flags({
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
            "ExprFibAttrs",
            r#type.and_then(|t| ExprFibAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableExprFibAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ExprFibAttrs");
        let mut iter = IterableExprFibAttrs::with_loc(&[], self.orig_loc);
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
                ExprFibAttrs::Dreg(val) => fmt.field("Dreg", &val),
                ExprFibAttrs::Result(val) => {
                    fmt.field("Result", &FormatEnum(val.into(), FibResult::from_value))
                }
                ExprFibAttrs::Flags(val) => {
                    fmt.field("Flags", &FormatFlags(val.into(), FibFlags::from_value))
                }
            };
        }
        fmt.finish()
    }
}
impl IterableExprFibAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ExprFibAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ExprFibAttrs::attr_from_type(t)),
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
                ExprFibAttrs::Dreg(val) => {
                    if last_off == offset {
                        stack.push(("Dreg", last_off));
                        break;
                    }
                }
                ExprFibAttrs::Result(val) => {
                    if last_off == offset {
                        stack.push(("Result", last_off));
                        break;
                    }
                }
                ExprFibAttrs::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ExprFibAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ExprCtAttrs {
    Dreg(u32),
    #[doc = "Associated type: [`CtKeys`] (enum)"]
    Key(u32),
    #[doc = "Associated type: [`CtDirection`] (enum)"]
    Direction(u8),
    Sreg(u32),
}
impl<'a> IterableExprCtAttrs<'a> {
    pub fn get_dreg(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprCtAttrs::Dreg(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprCtAttrs",
            "Dreg",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`CtKeys`] (enum)"]
    pub fn get_key(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprCtAttrs::Key(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprCtAttrs",
            "Key",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`CtDirection`] (enum)"]
    pub fn get_direction(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprCtAttrs::Direction(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprCtAttrs",
            "Direction",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sreg(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprCtAttrs::Sreg(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprCtAttrs",
            "Sreg",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ExprCtAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableExprCtAttrs<'a> {
        IterableExprCtAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Dreg",
            2u16 => "Key",
            3u16 => "Direction",
            4u16 => "Sreg",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableExprCtAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableExprCtAttrs<'a> {
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
impl<'a> Iterator for IterableExprCtAttrs<'a> {
    type Item = Result<ExprCtAttrs, ErrorContext>;
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
                1u16 => ExprCtAttrs::Dreg({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ExprCtAttrs::Key({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ExprCtAttrs::Direction({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => ExprCtAttrs::Sreg({
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
            "ExprCtAttrs",
            r#type.and_then(|t| ExprCtAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableExprCtAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ExprCtAttrs");
        let mut iter = IterableExprCtAttrs::with_loc(&[], self.orig_loc);
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
                ExprCtAttrs::Dreg(val) => fmt.field("Dreg", &val),
                ExprCtAttrs::Key(val) => {
                    fmt.field("Key", &FormatEnum(val.into(), CtKeys::from_value))
                }
                ExprCtAttrs::Direction(val) => fmt.field(
                    "Direction",
                    &FormatEnum(val.into(), CtDirection::from_value),
                ),
                ExprCtAttrs::Sreg(val) => fmt.field("Sreg", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableExprCtAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ExprCtAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ExprCtAttrs::attr_from_type(t)),
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
                ExprCtAttrs::Dreg(val) => {
                    if last_off == offset {
                        stack.push(("Dreg", last_off));
                        break;
                    }
                }
                ExprCtAttrs::Key(val) => {
                    if last_off == offset {
                        stack.push(("Key", last_off));
                        break;
                    }
                }
                ExprCtAttrs::Direction(val) => {
                    if last_off == offset {
                        stack.push(("Direction", last_off));
                        break;
                    }
                }
                ExprCtAttrs::Sreg(val) => {
                    if last_off == offset {
                        stack.push(("Sreg", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ExprCtAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ExprFlowOffloadAttrs<'a> {
    #[doc = "Flow offload table name\n"]
    Name(&'a CStr),
}
impl<'a> IterableExprFlowOffloadAttrs<'a> {
    #[doc = "Flow offload table name\n"]
    pub fn get_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprFlowOffloadAttrs::Name(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprFlowOffloadAttrs",
            "Name",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ExprFlowOffloadAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableExprFlowOffloadAttrs<'a> {
        IterableExprFlowOffloadAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Name",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableExprFlowOffloadAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableExprFlowOffloadAttrs<'a> {
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
impl<'a> Iterator for IterableExprFlowOffloadAttrs<'a> {
    type Item = Result<ExprFlowOffloadAttrs<'a>, ErrorContext>;
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
                1u16 => ExprFlowOffloadAttrs::Name({
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
            "ExprFlowOffloadAttrs",
            r#type.and_then(|t| ExprFlowOffloadAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableExprFlowOffloadAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ExprFlowOffloadAttrs");
        let mut iter = IterableExprFlowOffloadAttrs::with_loc(&[], self.orig_loc);
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
                ExprFlowOffloadAttrs::Name(val) => fmt.field("Name", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableExprFlowOffloadAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ExprFlowOffloadAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ExprFlowOffloadAttrs::attr_from_type(t)),
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
                ExprFlowOffloadAttrs::Name(val) => {
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
            stack.push(("ExprFlowOffloadAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ExprImmediateAttrs<'a> {
    Dreg(u32),
    Data(IterableDataAttrs<'a>),
}
impl<'a> IterableExprImmediateAttrs<'a> {
    pub fn get_dreg(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprImmediateAttrs::Dreg(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprImmediateAttrs",
            "Dreg",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_data(&self) -> Result<IterableDataAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprImmediateAttrs::Data(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprImmediateAttrs",
            "Data",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ExprImmediateAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableExprImmediateAttrs<'a> {
        IterableExprImmediateAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Dreg",
            2u16 => "Data",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableExprImmediateAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableExprImmediateAttrs<'a> {
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
impl<'a> Iterator for IterableExprImmediateAttrs<'a> {
    type Item = Result<ExprImmediateAttrs<'a>, ErrorContext>;
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
                1u16 => ExprImmediateAttrs::Dreg({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ExprImmediateAttrs::Data({
                    let res = Some(IterableDataAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "ExprImmediateAttrs",
            r#type.and_then(|t| ExprImmediateAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableExprImmediateAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ExprImmediateAttrs");
        let mut iter = IterableExprImmediateAttrs::with_loc(&[], self.orig_loc);
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
                ExprImmediateAttrs::Dreg(val) => fmt.field("Dreg", &val),
                ExprImmediateAttrs::Data(val) => fmt.field("Data", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableExprImmediateAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ExprImmediateAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ExprImmediateAttrs::attr_from_type(t)),
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
                ExprImmediateAttrs::Dreg(val) => {
                    if last_off == offset {
                        stack.push(("Dreg", last_off));
                        break;
                    }
                }
                ExprImmediateAttrs::Data(val) => {
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
            stack.push(("ExprImmediateAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum ExprLookupAttrs<'a> {
    #[doc = "Name of set to use\n"]
    Set(&'a CStr),
    #[doc = "ID of set to use\n"]
    SetId(u32),
    Sreg(u32),
    Dreg(u32),
    #[doc = "Associated type: [`LookupFlags`] (enum)"]
    Flags(u32),
}
impl<'a> IterableExprLookupAttrs<'a> {
    #[doc = "Name of set to use\n"]
    pub fn get_set(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprLookupAttrs::Set(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprLookupAttrs",
            "Set",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ID of set to use\n"]
    pub fn get_set_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprLookupAttrs::SetId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprLookupAttrs",
            "SetId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sreg(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprLookupAttrs::Sreg(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprLookupAttrs",
            "Sreg",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dreg(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprLookupAttrs::Dreg(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprLookupAttrs",
            "Dreg",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`LookupFlags`] (enum)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprLookupAttrs::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprLookupAttrs",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ExprLookupAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableExprLookupAttrs<'a> {
        IterableExprLookupAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Set",
            2u16 => "SetId",
            3u16 => "Sreg",
            4u16 => "Dreg",
            5u16 => "Flags",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableExprLookupAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableExprLookupAttrs<'a> {
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
impl<'a> Iterator for IterableExprLookupAttrs<'a> {
    type Item = Result<ExprLookupAttrs<'a>, ErrorContext>;
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
                1u16 => ExprLookupAttrs::Set({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ExprLookupAttrs::SetId({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ExprLookupAttrs::Sreg({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => ExprLookupAttrs::Dreg({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => ExprLookupAttrs::Flags({
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
            "ExprLookupAttrs",
            r#type.and_then(|t| ExprLookupAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableExprLookupAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ExprLookupAttrs");
        let mut iter = IterableExprLookupAttrs::with_loc(&[], self.orig_loc);
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
                ExprLookupAttrs::Set(val) => fmt.field("Set", &val),
                ExprLookupAttrs::SetId(val) => fmt.field("SetId", &val),
                ExprLookupAttrs::Sreg(val) => fmt.field("Sreg", &val),
                ExprLookupAttrs::Dreg(val) => fmt.field("Dreg", &val),
                ExprLookupAttrs::Flags(val) => {
                    fmt.field("Flags", &FormatFlags(val.into(), LookupFlags::from_value))
                }
            };
        }
        fmt.finish()
    }
}
impl IterableExprLookupAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ExprLookupAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ExprLookupAttrs::attr_from_type(t)),
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
                ExprLookupAttrs::Set(val) => {
                    if last_off == offset {
                        stack.push(("Set", last_off));
                        break;
                    }
                }
                ExprLookupAttrs::SetId(val) => {
                    if last_off == offset {
                        stack.push(("SetId", last_off));
                        break;
                    }
                }
                ExprLookupAttrs::Sreg(val) => {
                    if last_off == offset {
                        stack.push(("Sreg", last_off));
                        break;
                    }
                }
                ExprLookupAttrs::Dreg(val) => {
                    if last_off == offset {
                        stack.push(("Dreg", last_off));
                        break;
                    }
                }
                ExprLookupAttrs::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ExprLookupAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ExprMasqAttrs {
    #[doc = "Associated type: [`NatRangeFlags`] (1 bit per enumeration)"]
    Flags(u32),
    #[doc = "Associated type: [`Registers`] (enum)"]
    RegProtoMin(u32),
    #[doc = "Associated type: [`Registers`] (enum)"]
    RegProtoMax(u32),
}
impl<'a> IterableExprMasqAttrs<'a> {
    #[doc = "Associated type: [`NatRangeFlags`] (1 bit per enumeration)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprMasqAttrs::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprMasqAttrs",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`Registers`] (enum)"]
    pub fn get_reg_proto_min(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprMasqAttrs::RegProtoMin(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprMasqAttrs",
            "RegProtoMin",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`Registers`] (enum)"]
    pub fn get_reg_proto_max(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprMasqAttrs::RegProtoMax(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprMasqAttrs",
            "RegProtoMax",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ExprMasqAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableExprMasqAttrs<'a> {
        IterableExprMasqAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Flags",
            2u16 => "RegProtoMin",
            3u16 => "RegProtoMax",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableExprMasqAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableExprMasqAttrs<'a> {
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
impl<'a> Iterator for IterableExprMasqAttrs<'a> {
    type Item = Result<ExprMasqAttrs, ErrorContext>;
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
                1u16 => ExprMasqAttrs::Flags({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ExprMasqAttrs::RegProtoMin({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ExprMasqAttrs::RegProtoMax({
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
            "ExprMasqAttrs",
            r#type.and_then(|t| ExprMasqAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableExprMasqAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ExprMasqAttrs");
        let mut iter = IterableExprMasqAttrs::with_loc(&[], self.orig_loc);
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
                ExprMasqAttrs::Flags(val) => {
                    fmt.field("Flags", &FormatFlags(val.into(), NatRangeFlags::from_value))
                }
                ExprMasqAttrs::RegProtoMin(val) => fmt.field(
                    "RegProtoMin",
                    &FormatEnum(val.into(), Registers::from_value),
                ),
                ExprMasqAttrs::RegProtoMax(val) => fmt.field(
                    "RegProtoMax",
                    &FormatEnum(val.into(), Registers::from_value),
                ),
            };
        }
        fmt.finish()
    }
}
impl IterableExprMasqAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ExprMasqAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ExprMasqAttrs::attr_from_type(t)),
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
                ExprMasqAttrs::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                ExprMasqAttrs::RegProtoMin(val) => {
                    if last_off == offset {
                        stack.push(("RegProtoMin", last_off));
                        break;
                    }
                }
                ExprMasqAttrs::RegProtoMax(val) => {
                    if last_off == offset {
                        stack.push(("RegProtoMax", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ExprMasqAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ExprMetaAttrs {
    Dreg(u32),
    #[doc = "Associated type: [`MetaKeys`] (enum)"]
    Key(u32),
    Sreg(u32),
}
impl<'a> IterableExprMetaAttrs<'a> {
    pub fn get_dreg(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprMetaAttrs::Dreg(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprMetaAttrs",
            "Dreg",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`MetaKeys`] (enum)"]
    pub fn get_key(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprMetaAttrs::Key(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprMetaAttrs",
            "Key",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sreg(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprMetaAttrs::Sreg(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprMetaAttrs",
            "Sreg",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ExprMetaAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableExprMetaAttrs<'a> {
        IterableExprMetaAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Dreg",
            2u16 => "Key",
            3u16 => "Sreg",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableExprMetaAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableExprMetaAttrs<'a> {
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
impl<'a> Iterator for IterableExprMetaAttrs<'a> {
    type Item = Result<ExprMetaAttrs, ErrorContext>;
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
                1u16 => ExprMetaAttrs::Dreg({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ExprMetaAttrs::Key({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ExprMetaAttrs::Sreg({
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
            "ExprMetaAttrs",
            r#type.and_then(|t| ExprMetaAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableExprMetaAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ExprMetaAttrs");
        let mut iter = IterableExprMetaAttrs::with_loc(&[], self.orig_loc);
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
                ExprMetaAttrs::Dreg(val) => fmt.field("Dreg", &val),
                ExprMetaAttrs::Key(val) => {
                    fmt.field("Key", &FormatEnum(val.into(), MetaKeys::from_value))
                }
                ExprMetaAttrs::Sreg(val) => fmt.field("Sreg", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableExprMetaAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ExprMetaAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ExprMetaAttrs::attr_from_type(t)),
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
                ExprMetaAttrs::Dreg(val) => {
                    if last_off == offset {
                        stack.push(("Dreg", last_off));
                        break;
                    }
                }
                ExprMetaAttrs::Key(val) => {
                    if last_off == offset {
                        stack.push(("Key", last_off));
                        break;
                    }
                }
                ExprMetaAttrs::Sreg(val) => {
                    if last_off == offset {
                        stack.push(("Sreg", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ExprMetaAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ExprNatAttrs {
    Type(u32),
    Family(u32),
    RegAddrMin(u32),
    RegAddrMax(u32),
    RegProtoMin(u32),
    RegProtoMax(u32),
    #[doc = "Associated type: [`NatRangeFlags`] (1 bit per enumeration)"]
    Flags(u32),
}
impl<'a> IterableExprNatAttrs<'a> {
    pub fn get_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprNatAttrs::Type(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprNatAttrs",
            "Type",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_family(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprNatAttrs::Family(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprNatAttrs",
            "Family",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_reg_addr_min(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprNatAttrs::RegAddrMin(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprNatAttrs",
            "RegAddrMin",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_reg_addr_max(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprNatAttrs::RegAddrMax(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprNatAttrs",
            "RegAddrMax",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_reg_proto_min(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprNatAttrs::RegProtoMin(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprNatAttrs",
            "RegProtoMin",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_reg_proto_max(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprNatAttrs::RegProtoMax(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprNatAttrs",
            "RegProtoMax",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`NatRangeFlags`] (1 bit per enumeration)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprNatAttrs::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprNatAttrs",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ExprNatAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableExprNatAttrs<'a> {
        IterableExprNatAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Type",
            2u16 => "Family",
            3u16 => "RegAddrMin",
            4u16 => "RegAddrMax",
            5u16 => "RegProtoMin",
            6u16 => "RegProtoMax",
            7u16 => "Flags",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableExprNatAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableExprNatAttrs<'a> {
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
impl<'a> Iterator for IterableExprNatAttrs<'a> {
    type Item = Result<ExprNatAttrs, ErrorContext>;
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
                1u16 => ExprNatAttrs::Type({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ExprNatAttrs::Family({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ExprNatAttrs::RegAddrMin({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => ExprNatAttrs::RegAddrMax({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => ExprNatAttrs::RegProtoMin({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => ExprNatAttrs::RegProtoMax({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => ExprNatAttrs::Flags({
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
            "ExprNatAttrs",
            r#type.and_then(|t| ExprNatAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableExprNatAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ExprNatAttrs");
        let mut iter = IterableExprNatAttrs::with_loc(&[], self.orig_loc);
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
                ExprNatAttrs::Type(val) => fmt.field("Type", &val),
                ExprNatAttrs::Family(val) => fmt.field("Family", &val),
                ExprNatAttrs::RegAddrMin(val) => fmt.field("RegAddrMin", &val),
                ExprNatAttrs::RegAddrMax(val) => fmt.field("RegAddrMax", &val),
                ExprNatAttrs::RegProtoMin(val) => fmt.field("RegProtoMin", &val),
                ExprNatAttrs::RegProtoMax(val) => fmt.field("RegProtoMax", &val),
                ExprNatAttrs::Flags(val) => {
                    fmt.field("Flags", &FormatFlags(val.into(), NatRangeFlags::from_value))
                }
            };
        }
        fmt.finish()
    }
}
impl IterableExprNatAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ExprNatAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ExprNatAttrs::attr_from_type(t)),
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
                ExprNatAttrs::Type(val) => {
                    if last_off == offset {
                        stack.push(("Type", last_off));
                        break;
                    }
                }
                ExprNatAttrs::Family(val) => {
                    if last_off == offset {
                        stack.push(("Family", last_off));
                        break;
                    }
                }
                ExprNatAttrs::RegAddrMin(val) => {
                    if last_off == offset {
                        stack.push(("RegAddrMin", last_off));
                        break;
                    }
                }
                ExprNatAttrs::RegAddrMax(val) => {
                    if last_off == offset {
                        stack.push(("RegAddrMax", last_off));
                        break;
                    }
                }
                ExprNatAttrs::RegProtoMin(val) => {
                    if last_off == offset {
                        stack.push(("RegProtoMin", last_off));
                        break;
                    }
                }
                ExprNatAttrs::RegProtoMax(val) => {
                    if last_off == offset {
                        stack.push(("RegProtoMax", last_off));
                        break;
                    }
                }
                ExprNatAttrs::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ExprNatAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ExprPayloadAttrs {
    #[doc = "destination register to load data into\n\nAssociated type: [`Registers`] (enum)"]
    Dreg(u32),
    #[doc = "payload base\n\nAssociated type: [`PayloadBase`] (enum)"]
    Base(u32),
    #[doc = "payload offset relative to base\n"]
    Offset(u32),
    #[doc = "payload length\n"]
    Len(u32),
    #[doc = "source register to load data from\n\nAssociated type: [`Registers`] (enum)"]
    Sreg(u32),
    #[doc = "checksum type\n"]
    CsumType(u32),
    #[doc = "checksum offset relative to base\n"]
    CsumOffset(u32),
    #[doc = "checksum flags\n"]
    CsumFlags(u32),
}
impl<'a> IterableExprPayloadAttrs<'a> {
    #[doc = "destination register to load data into\n\nAssociated type: [`Registers`] (enum)"]
    pub fn get_dreg(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprPayloadAttrs::Dreg(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprPayloadAttrs",
            "Dreg",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "payload base\n\nAssociated type: [`PayloadBase`] (enum)"]
    pub fn get_base(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprPayloadAttrs::Base(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprPayloadAttrs",
            "Base",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "payload offset relative to base\n"]
    pub fn get_offset(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprPayloadAttrs::Offset(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprPayloadAttrs",
            "Offset",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "payload length\n"]
    pub fn get_len(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprPayloadAttrs::Len(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprPayloadAttrs",
            "Len",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "source register to load data from\n\nAssociated type: [`Registers`] (enum)"]
    pub fn get_sreg(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprPayloadAttrs::Sreg(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprPayloadAttrs",
            "Sreg",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "checksum type\n"]
    pub fn get_csum_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprPayloadAttrs::CsumType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprPayloadAttrs",
            "CsumType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "checksum offset relative to base\n"]
    pub fn get_csum_offset(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprPayloadAttrs::CsumOffset(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprPayloadAttrs",
            "CsumOffset",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "checksum flags\n"]
    pub fn get_csum_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprPayloadAttrs::CsumFlags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprPayloadAttrs",
            "CsumFlags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ExprPayloadAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableExprPayloadAttrs<'a> {
        IterableExprPayloadAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Dreg",
            2u16 => "Base",
            3u16 => "Offset",
            4u16 => "Len",
            5u16 => "Sreg",
            6u16 => "CsumType",
            7u16 => "CsumOffset",
            8u16 => "CsumFlags",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableExprPayloadAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableExprPayloadAttrs<'a> {
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
impl<'a> Iterator for IterableExprPayloadAttrs<'a> {
    type Item = Result<ExprPayloadAttrs, ErrorContext>;
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
                1u16 => ExprPayloadAttrs::Dreg({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ExprPayloadAttrs::Base({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ExprPayloadAttrs::Offset({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => ExprPayloadAttrs::Len({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => ExprPayloadAttrs::Sreg({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => ExprPayloadAttrs::CsumType({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => ExprPayloadAttrs::CsumOffset({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => ExprPayloadAttrs::CsumFlags({
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
            "ExprPayloadAttrs",
            r#type.and_then(|t| ExprPayloadAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableExprPayloadAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ExprPayloadAttrs");
        let mut iter = IterableExprPayloadAttrs::with_loc(&[], self.orig_loc);
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
                ExprPayloadAttrs::Dreg(val) => {
                    fmt.field("Dreg", &FormatEnum(val.into(), Registers::from_value))
                }
                ExprPayloadAttrs::Base(val) => {
                    fmt.field("Base", &FormatEnum(val.into(), PayloadBase::from_value))
                }
                ExprPayloadAttrs::Offset(val) => fmt.field("Offset", &val),
                ExprPayloadAttrs::Len(val) => fmt.field("Len", &val),
                ExprPayloadAttrs::Sreg(val) => {
                    fmt.field("Sreg", &FormatEnum(val.into(), Registers::from_value))
                }
                ExprPayloadAttrs::CsumType(val) => fmt.field("CsumType", &val),
                ExprPayloadAttrs::CsumOffset(val) => fmt.field("CsumOffset", &val),
                ExprPayloadAttrs::CsumFlags(val) => fmt.field("CsumFlags", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableExprPayloadAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ExprPayloadAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ExprPayloadAttrs::attr_from_type(t)),
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
                ExprPayloadAttrs::Dreg(val) => {
                    if last_off == offset {
                        stack.push(("Dreg", last_off));
                        break;
                    }
                }
                ExprPayloadAttrs::Base(val) => {
                    if last_off == offset {
                        stack.push(("Base", last_off));
                        break;
                    }
                }
                ExprPayloadAttrs::Offset(val) => {
                    if last_off == offset {
                        stack.push(("Offset", last_off));
                        break;
                    }
                }
                ExprPayloadAttrs::Len(val) => {
                    if last_off == offset {
                        stack.push(("Len", last_off));
                        break;
                    }
                }
                ExprPayloadAttrs::Sreg(val) => {
                    if last_off == offset {
                        stack.push(("Sreg", last_off));
                        break;
                    }
                }
                ExprPayloadAttrs::CsumType(val) => {
                    if last_off == offset {
                        stack.push(("CsumType", last_off));
                        break;
                    }
                }
                ExprPayloadAttrs::CsumOffset(val) => {
                    if last_off == offset {
                        stack.push(("CsumOffset", last_off));
                        break;
                    }
                }
                ExprPayloadAttrs::CsumFlags(val) => {
                    if last_off == offset {
                        stack.push(("CsumFlags", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ExprPayloadAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ExprRejectAttrs {
    #[doc = "Associated type: [`RejectTypes`] (enum)"]
    Type(u32),
    IcmpCode(u8),
}
impl<'a> IterableExprRejectAttrs<'a> {
    #[doc = "Associated type: [`RejectTypes`] (enum)"]
    pub fn get_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprRejectAttrs::Type(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprRejectAttrs",
            "Type",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_icmp_code(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprRejectAttrs::IcmpCode(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprRejectAttrs",
            "IcmpCode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ExprRejectAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableExprRejectAttrs<'a> {
        IterableExprRejectAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Type",
            2u16 => "IcmpCode",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableExprRejectAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableExprRejectAttrs<'a> {
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
impl<'a> Iterator for IterableExprRejectAttrs<'a> {
    type Item = Result<ExprRejectAttrs, ErrorContext>;
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
                1u16 => ExprRejectAttrs::Type({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ExprRejectAttrs::IcmpCode({
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
            "ExprRejectAttrs",
            r#type.and_then(|t| ExprRejectAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableExprRejectAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ExprRejectAttrs");
        let mut iter = IterableExprRejectAttrs::with_loc(&[], self.orig_loc);
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
                ExprRejectAttrs::Type(val) => {
                    fmt.field("Type", &FormatEnum(val.into(), RejectTypes::from_value))
                }
                ExprRejectAttrs::IcmpCode(val) => fmt.field("IcmpCode", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableExprRejectAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ExprRejectAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ExprRejectAttrs::attr_from_type(t)),
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
                ExprRejectAttrs::Type(val) => {
                    if last_off == offset {
                        stack.push(("Type", last_off));
                        break;
                    }
                }
                ExprRejectAttrs::IcmpCode(val) => {
                    if last_off == offset {
                        stack.push(("IcmpCode", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ExprRejectAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ExprTargetAttrs<'a> {
    Name(&'a CStr),
    Rev(u32),
    #[doc = "This attribute is usually carries struct. Unlike other sub-messages, its\ntype depend on values of **both** name and rev.\n\nGrep for `static struct xt_target` in `linux/net/nftables`.\n"]
    Info(&'a [u8]),
}
impl<'a> IterableExprTargetAttrs<'a> {
    pub fn get_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprTargetAttrs::Name(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprTargetAttrs",
            "Name",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rev(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprTargetAttrs::Rev(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprTargetAttrs",
            "Rev",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "This attribute is usually carries struct. Unlike other sub-messages, its\ntype depend on values of **both** name and rev.\n\nGrep for `static struct xt_target` in `linux/net/nftables`.\n"]
    pub fn get_info(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprTargetAttrs::Info(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprTargetAttrs",
            "Info",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ExprTargetAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableExprTargetAttrs<'a> {
        IterableExprTargetAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Name",
            2u16 => "Rev",
            3u16 => "Info",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableExprTargetAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableExprTargetAttrs<'a> {
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
impl<'a> Iterator for IterableExprTargetAttrs<'a> {
    type Item = Result<ExprTargetAttrs<'a>, ErrorContext>;
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
                1u16 => ExprTargetAttrs::Name({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ExprTargetAttrs::Rev({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ExprTargetAttrs::Info({
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
            "ExprTargetAttrs",
            r#type.and_then(|t| ExprTargetAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableExprTargetAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ExprTargetAttrs");
        let mut iter = IterableExprTargetAttrs::with_loc(&[], self.orig_loc);
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
                ExprTargetAttrs::Name(val) => fmt.field("Name", &val),
                ExprTargetAttrs::Rev(val) => fmt.field("Rev", &val),
                ExprTargetAttrs::Info(val) => fmt.field("Info", &FormatHexdump(val)),
            };
        }
        fmt.finish()
    }
}
impl IterableExprTargetAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ExprTargetAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ExprTargetAttrs::attr_from_type(t)),
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
                ExprTargetAttrs::Name(val) => {
                    if last_off == offset {
                        stack.push(("Name", last_off));
                        break;
                    }
                }
                ExprTargetAttrs::Rev(val) => {
                    if last_off == offset {
                        stack.push(("Rev", last_off));
                        break;
                    }
                }
                ExprTargetAttrs::Info(val) => {
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
            stack.push(("ExprTargetAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ExprTproxyAttrs {
    Family(u32),
    RegAddr(u32),
    RegPort(u32),
}
impl<'a> IterableExprTproxyAttrs<'a> {
    pub fn get_family(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprTproxyAttrs::Family(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprTproxyAttrs",
            "Family",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_reg_addr(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprTproxyAttrs::RegAddr(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprTproxyAttrs",
            "RegAddr",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_reg_port(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprTproxyAttrs::RegPort(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprTproxyAttrs",
            "RegPort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ExprTproxyAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableExprTproxyAttrs<'a> {
        IterableExprTproxyAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Family",
            2u16 => "RegAddr",
            3u16 => "RegPort",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableExprTproxyAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableExprTproxyAttrs<'a> {
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
impl<'a> Iterator for IterableExprTproxyAttrs<'a> {
    type Item = Result<ExprTproxyAttrs, ErrorContext>;
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
                1u16 => ExprTproxyAttrs::Family({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ExprTproxyAttrs::RegAddr({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ExprTproxyAttrs::RegPort({
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
            "ExprTproxyAttrs",
            r#type.and_then(|t| ExprTproxyAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableExprTproxyAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ExprTproxyAttrs");
        let mut iter = IterableExprTproxyAttrs::with_loc(&[], self.orig_loc);
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
                ExprTproxyAttrs::Family(val) => fmt.field("Family", &val),
                ExprTproxyAttrs::RegAddr(val) => fmt.field("RegAddr", &val),
                ExprTproxyAttrs::RegPort(val) => fmt.field("RegPort", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableExprTproxyAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ExprTproxyAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ExprTproxyAttrs::attr_from_type(t)),
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
                ExprTproxyAttrs::Family(val) => {
                    if last_off == offset {
                        stack.push(("Family", last_off));
                        break;
                    }
                }
                ExprTproxyAttrs::RegAddr(val) => {
                    if last_off == offset {
                        stack.push(("RegAddr", last_off));
                        break;
                    }
                }
                ExprTproxyAttrs::RegPort(val) => {
                    if last_off == offset {
                        stack.push(("RegPort", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ExprTproxyAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ExprObjrefAttrs<'a> {
    ImmType(u32),
    #[doc = "object name\n"]
    ImmName(&'a CStr),
    SetSreg(u32),
    #[doc = "name of object map\n"]
    SetName(&'a CStr),
    #[doc = "id of object map\n"]
    SetId(u32),
}
impl<'a> IterableExprObjrefAttrs<'a> {
    pub fn get_imm_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprObjrefAttrs::ImmType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprObjrefAttrs",
            "ImmType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "object name\n"]
    pub fn get_imm_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprObjrefAttrs::ImmName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprObjrefAttrs",
            "ImmName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_set_sreg(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprObjrefAttrs::SetSreg(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprObjrefAttrs",
            "SetSreg",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "name of object map\n"]
    pub fn get_set_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprObjrefAttrs::SetName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprObjrefAttrs",
            "SetName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "id of object map\n"]
    pub fn get_set_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ExprObjrefAttrs::SetId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ExprObjrefAttrs",
            "SetId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ExprObjrefAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableExprObjrefAttrs<'a> {
        IterableExprObjrefAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "ImmType",
            2u16 => "ImmName",
            3u16 => "SetSreg",
            4u16 => "SetName",
            5u16 => "SetId",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableExprObjrefAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableExprObjrefAttrs<'a> {
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
impl<'a> Iterator for IterableExprObjrefAttrs<'a> {
    type Item = Result<ExprObjrefAttrs<'a>, ErrorContext>;
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
                1u16 => ExprObjrefAttrs::ImmType({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ExprObjrefAttrs::ImmName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ExprObjrefAttrs::SetSreg({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => ExprObjrefAttrs::SetName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => ExprObjrefAttrs::SetId({
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
            "ExprObjrefAttrs",
            r#type.and_then(|t| ExprObjrefAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableExprObjrefAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ExprObjrefAttrs");
        let mut iter = IterableExprObjrefAttrs::with_loc(&[], self.orig_loc);
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
                ExprObjrefAttrs::ImmType(val) => fmt.field("ImmType", &val),
                ExprObjrefAttrs::ImmName(val) => fmt.field("ImmName", &val),
                ExprObjrefAttrs::SetSreg(val) => fmt.field("SetSreg", &val),
                ExprObjrefAttrs::SetName(val) => fmt.field("SetName", &val),
                ExprObjrefAttrs::SetId(val) => fmt.field("SetId", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableExprObjrefAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ExprObjrefAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ExprObjrefAttrs::attr_from_type(t)),
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
                ExprObjrefAttrs::ImmType(val) => {
                    if last_off == offset {
                        stack.push(("ImmType", last_off));
                        break;
                    }
                }
                ExprObjrefAttrs::ImmName(val) => {
                    if last_off == offset {
                        stack.push(("ImmName", last_off));
                        break;
                    }
                }
                ExprObjrefAttrs::SetSreg(val) => {
                    if last_off == offset {
                        stack.push(("SetSreg", last_off));
                        break;
                    }
                }
                ExprObjrefAttrs::SetName(val) => {
                    if last_off == offset {
                        stack.push(("SetName", last_off));
                        break;
                    }
                }
                ExprObjrefAttrs::SetId(val) => {
                    if last_off == offset {
                        stack.push(("SetId", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ExprObjrefAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum CompatTargetAttrs<'a> {
    Name(&'a CStr),
    Rev(u32),
    Info(&'a [u8]),
}
impl<'a> IterableCompatTargetAttrs<'a> {
    pub fn get_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CompatTargetAttrs::Name(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CompatTargetAttrs",
            "Name",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rev(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CompatTargetAttrs::Rev(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CompatTargetAttrs",
            "Rev",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_info(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CompatTargetAttrs::Info(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CompatTargetAttrs",
            "Info",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl CompatTargetAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableCompatTargetAttrs<'a> {
        IterableCompatTargetAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Name",
            2u16 => "Rev",
            3u16 => "Info",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableCompatTargetAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableCompatTargetAttrs<'a> {
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
impl<'a> Iterator for IterableCompatTargetAttrs<'a> {
    type Item = Result<CompatTargetAttrs<'a>, ErrorContext>;
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
                1u16 => CompatTargetAttrs::Name({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => CompatTargetAttrs::Rev({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => CompatTargetAttrs::Info({
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
            "CompatTargetAttrs",
            r#type.and_then(|t| CompatTargetAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableCompatTargetAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("CompatTargetAttrs");
        let mut iter = IterableCompatTargetAttrs::with_loc(&[], self.orig_loc);
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
                CompatTargetAttrs::Name(val) => fmt.field("Name", &val),
                CompatTargetAttrs::Rev(val) => fmt.field("Rev", &val),
                CompatTargetAttrs::Info(val) => fmt.field("Info", &FormatHexdump(val)),
            };
        }
        fmt.finish()
    }
}
impl IterableCompatTargetAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("CompatTargetAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| CompatTargetAttrs::attr_from_type(t)),
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
                CompatTargetAttrs::Name(val) => {
                    if last_off == offset {
                        stack.push(("Name", last_off));
                        break;
                    }
                }
                CompatTargetAttrs::Rev(val) => {
                    if last_off == offset {
                        stack.push(("Rev", last_off));
                        break;
                    }
                }
                CompatTargetAttrs::Info(val) => {
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
            stack.push(("CompatTargetAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum CompatMatchAttrs<'a> {
    Name(&'a CStr),
    Rev(u32),
    Info(&'a [u8]),
}
impl<'a> IterableCompatMatchAttrs<'a> {
    pub fn get_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CompatMatchAttrs::Name(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CompatMatchAttrs",
            "Name",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rev(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CompatMatchAttrs::Rev(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CompatMatchAttrs",
            "Rev",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_info(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CompatMatchAttrs::Info(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CompatMatchAttrs",
            "Info",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl CompatMatchAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableCompatMatchAttrs<'a> {
        IterableCompatMatchAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Name",
            2u16 => "Rev",
            3u16 => "Info",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableCompatMatchAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableCompatMatchAttrs<'a> {
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
impl<'a> Iterator for IterableCompatMatchAttrs<'a> {
    type Item = Result<CompatMatchAttrs<'a>, ErrorContext>;
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
                1u16 => CompatMatchAttrs::Name({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => CompatMatchAttrs::Rev({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => CompatMatchAttrs::Info({
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
            "CompatMatchAttrs",
            r#type.and_then(|t| CompatMatchAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableCompatMatchAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("CompatMatchAttrs");
        let mut iter = IterableCompatMatchAttrs::with_loc(&[], self.orig_loc);
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
                CompatMatchAttrs::Name(val) => fmt.field("Name", &val),
                CompatMatchAttrs::Rev(val) => fmt.field("Rev", &val),
                CompatMatchAttrs::Info(val) => fmt.field("Info", &FormatHexdump(val)),
            };
        }
        fmt.finish()
    }
}
impl IterableCompatMatchAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("CompatMatchAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| CompatMatchAttrs::attr_from_type(t)),
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
                CompatMatchAttrs::Name(val) => {
                    if last_off == offset {
                        stack.push(("Name", last_off));
                        break;
                    }
                }
                CompatMatchAttrs::Rev(val) => {
                    if last_off == offset {
                        stack.push(("Rev", last_off));
                        break;
                    }
                }
                CompatMatchAttrs::Info(val) => {
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
            stack.push(("CompatMatchAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum CompatAttrs<'a> {
    Name(&'a CStr),
    Rev(u32),
    Type(u32),
}
impl<'a> IterableCompatAttrs<'a> {
    pub fn get_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CompatAttrs::Name(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CompatAttrs",
            "Name",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rev(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CompatAttrs::Rev(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CompatAttrs",
            "Rev",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CompatAttrs::Type(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CompatAttrs",
            "Type",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl CompatAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableCompatAttrs<'a> {
        IterableCompatAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Name",
            2u16 => "Rev",
            3u16 => "Type",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableCompatAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableCompatAttrs<'a> {
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
impl<'a> Iterator for IterableCompatAttrs<'a> {
    type Item = Result<CompatAttrs<'a>, ErrorContext>;
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
                1u16 => CompatAttrs::Name({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => CompatAttrs::Rev({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => CompatAttrs::Type({
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
            "CompatAttrs",
            r#type.and_then(|t| CompatAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableCompatAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("CompatAttrs");
        let mut iter = IterableCompatAttrs::with_loc(&[], self.orig_loc);
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
                CompatAttrs::Name(val) => fmt.field("Name", &val),
                CompatAttrs::Rev(val) => fmt.field("Rev", &val),
                CompatAttrs::Type(val) => fmt.field("Type", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableCompatAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("CompatAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| CompatAttrs::attr_from_type(t)),
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
                CompatAttrs::Name(val) => {
                    if last_off == offset {
                        stack.push(("Name", last_off));
                        break;
                    }
                }
                CompatAttrs::Rev(val) => {
                    if last_off == offset {
                        stack.push(("Rev", last_off));
                        break;
                    }
                }
                CompatAttrs::Type(val) => {
                    if last_off == offset {
                        stack.push(("Type", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("CompatAttrs", cur));
        }
        (stack, None)
    }
}
pub struct PushLogAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLogAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLogAttrs<Prev> {
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
    #[doc = "netlink group to send messages to\n"]
    pub fn push_group(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 1u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "prefix to prepend to log messages\n"]
    pub fn push_prefix(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "prefix to prepend to log messages\n"]
    pub fn push_prefix_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "length of payload to include in netlink message\n"]
    pub fn push_snaplen(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "queue threshold\n"]
    pub fn push_qthreshold(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 4u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "log level\n\nAssociated type: [`LogLevel`] (enum)"]
    pub fn push_level(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "logging flags\n\nAssociated type: [`LogFlags`] (enum)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushLogAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushNumgenAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushNumgenAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushNumgenAttrs<Prev> {
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
    #[doc = "destination register\n\nAssociated type: [`Registers`] (enum)"]
    pub fn push_dreg(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "maximum counter value\n"]
    pub fn push_modulus(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "operation type\n\nAssociated type: [`NumgenTypes`] (enum)"]
    pub fn push_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "offset to be added to the counter\n"]
    pub fn push_offset(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushNumgenAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushRangeAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushRangeAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushRangeAttrs<Prev> {
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
    #[doc = "source register of data to compare\n\nAssociated type: [`Registers`] (enum)"]
    pub fn push_sreg(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "cmp operation\n\nAssociated type: [`RangeOps`] (enum)"]
    pub fn push_op(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "data range from\n"]
    pub fn nested_from_data(mut self) -> PushDataAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 3u16);
        PushDataAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "data range to\n"]
    pub fn nested_to_data(mut self) -> PushDataAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 4u16);
        PushDataAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushRangeAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushBatchAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushBatchAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushBatchAttrs<Prev> {
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
    #[doc = "generation ID for this changeset\n"]
    pub fn push_genid(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushBatchAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushTableAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushTableAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushTableAttrs<Prev> {
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
    #[doc = "name of the table\n"]
    pub fn push_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "name of the table\n"]
    pub fn push_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "bitmask of flags\n\nAssociated type: [`TableFlags`] (1 bit per enumeration)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "number of chains in this table\n"]
    pub fn push_use(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "numeric handle of the table\n"]
    pub fn push_handle(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 4u16, 8 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 5u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "user data\n"]
    pub fn push_userdata(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 6u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "owner of this table through netlink portID\n"]
    pub fn push_owner(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushTableAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushChainAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushChainAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushChainAttrs<Prev> {
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
    #[doc = "name of the table containing the chain\n"]
    pub fn push_table(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "name of the table containing the chain\n"]
    pub fn push_table_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "numeric handle of the chain\n"]
    pub fn push_handle(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 2u16, 8 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "name of the chain\n"]
    pub fn push_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            3u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "name of the chain\n"]
    pub fn push_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "hook specification for basechains\n"]
    pub fn nested_hook(mut self) -> PushNftHookAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 4u16);
        PushNftHookAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "numeric policy of the chain\n"]
    pub fn push_policy(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "number of references to this chain\n"]
    pub fn push_use(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "type name of the chain\n"]
    pub fn push_type(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            7u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "type name of the chain\n"]
    pub fn push_type_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 7u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "counter specification of the chain\n"]
    pub fn nested_counters(mut self) -> PushNftCounterAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 8u16);
        PushNftCounterAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "chain flags\n\nAssociated type: [`ChainFlags`] (1 bit per enumeration)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "uniquely identifies a chain in a transaction\n"]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "user data\n"]
    pub fn push_userdata(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 11u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushChainAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
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
    pub fn push_bytes(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 1u16, 8 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_packets(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 2u16, 8 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.len() as u16);
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
pub struct PushNftHookAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushNftHookAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushNftHookAttrs<Prev> {
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
    pub fn push_num(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_priority(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "net device name\n"]
    pub fn push_dev(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            3u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "net device name\n"]
    pub fn push_dev_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "list of net devices\n"]
    pub fn nested_devs(mut self) -> PushHookDevAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 4u16);
        PushHookDevAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushNftHookAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushHookDevAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushHookDevAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushHookDevAttrs<Prev> {
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
    pub fn push_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn push_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
}
impl<Prev: Pusher> Drop for PushHookDevAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushNftCounterAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushNftCounterAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushNftCounterAttrs<Prev> {
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
    pub fn push_bytes(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 1u16, 8 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_packets(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 2u16, 8 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushNftCounterAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushRuleAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushRuleAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushRuleAttrs<Prev> {
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
    #[doc = "name of the table containing the rule\n"]
    pub fn push_table(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "name of the table containing the rule\n"]
    pub fn push_table_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "name of the chain containing the rule\n"]
    pub fn push_chain(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "name of the chain containing the rule\n"]
    pub fn push_chain_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "numeric handle of the rule\n"]
    pub fn push_handle(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 3u16, 8 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "list of expressions\n"]
    pub fn nested_expressions(mut self) -> PushExprListAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 4u16);
        PushExprListAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "compatibility specifications of the rule\n"]
    pub fn nested_compat(mut self) -> PushRuleCompatAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 5u16);
        PushRuleCompatAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "numeric handle of the previous rule\n"]
    pub fn push_position(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 6u16, 8 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "user data\n"]
    pub fn push_userdata(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 7u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "uniquely identifies a rule in a transaction\n"]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "transaction unique identifier of the previous rule\n"]
    pub fn push_position_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "add the rule to chain by ID, alternative to chain name\n"]
    pub fn push_chain_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushRuleAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushExprListAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushExprListAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushExprListAttrs<Prev> {
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
    pub fn nested_elem(mut self) -> PushExprAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushExprAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushExprListAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushExprAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushExprAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushExprAttrs<Prev> {
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
    #[doc = "name of the expression type\n"]
    pub fn push_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "name of the expression type\n"]
    pub fn push_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "type specific data\n"]
    #[doc = "Selector attribute `name` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_bitwise(mut self) -> PushExprBitwiseAttrs<PushDummy<Prev>> {
        self = self.push_name(c"bitwise");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushExprBitwiseAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `name` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_cmp(mut self) -> PushExprCmpAttrs<PushDummy<Prev>> {
        self = self.push_name(c"cmp");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushExprCmpAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `name` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_counter(mut self) -> PushExprCounterAttrs<PushDummy<Prev>> {
        self = self.push_name(c"counter");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushExprCounterAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `name` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_ct(mut self) -> PushExprCtAttrs<PushDummy<Prev>> {
        self = self.push_name(c"ct");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushExprCtAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `name` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_fib(mut self) -> PushExprFibAttrs<PushDummy<Prev>> {
        self = self.push_name(c"fib");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushExprFibAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `name` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_flow_offload(mut self) -> PushExprFlowOffloadAttrs<PushDummy<Prev>> {
        self = self.push_name(c"flow_offload");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushExprFlowOffloadAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `name` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_immediate(mut self) -> PushExprImmediateAttrs<PushDummy<Prev>> {
        self = self.push_name(c"immediate");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushExprImmediateAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `name` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_log(mut self) -> PushLogAttrs<PushDummy<Prev>> {
        self = self.push_name(c"log");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushLogAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `name` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_lookup(mut self) -> PushExprLookupAttrs<PushDummy<Prev>> {
        self = self.push_name(c"lookup");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushExprLookupAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `name` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_match(mut self) -> PushCompatMatchAttrs<PushDummy<Prev>> {
        self = self.push_name(c"match");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushCompatMatchAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `name` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_meta(mut self) -> PushExprMetaAttrs<PushDummy<Prev>> {
        self = self.push_name(c"meta");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushExprMetaAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `name` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_nat(mut self) -> PushExprNatAttrs<PushDummy<Prev>> {
        self = self.push_name(c"nat");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushExprNatAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `name` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_numgen(mut self) -> PushNumgenAttrs<PushDummy<Prev>> {
        self = self.push_name(c"numgen");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushNumgenAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `name` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_objref(mut self) -> PushExprObjrefAttrs<PushDummy<Prev>> {
        self = self.push_name(c"objref");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushExprObjrefAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `name` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_payload(mut self) -> PushExprPayloadAttrs<PushDummy<Prev>> {
        self = self.push_name(c"payload");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushExprPayloadAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `name` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_quota(mut self) -> PushQuotaAttrs<PushDummy<Prev>> {
        self = self.push_name(c"quota");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushQuotaAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `name` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_range(mut self) -> PushRangeAttrs<PushDummy<Prev>> {
        self = self.push_name(c"range");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushRangeAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `name` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_reject(mut self) -> PushExprRejectAttrs<PushDummy<Prev>> {
        self = self.push_name(c"reject");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushExprRejectAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `name` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_target(mut self) -> PushExprTargetAttrs<PushDummy<Prev>> {
        self = self.push_name(c"target");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushExprTargetAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `name` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_tproxy(mut self) -> PushExprTproxyAttrs<PushDummy<Prev>> {
        self = self.push_name(c"tproxy");
        let new_header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushExprTproxyAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushExprAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushRuleCompatAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushRuleCompatAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushRuleCompatAttrs<Prev> {
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
    #[doc = "numeric value of the handled protocol\n"]
    pub fn push_proto(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "bitmask of flags\n"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushRuleCompatAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSetAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushSetAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushSetAttrs<Prev> {
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
    #[doc = "table name\n"]
    pub fn push_table(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "table name\n"]
    pub fn push_table_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "set name\n"]
    pub fn push_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "set name\n"]
    pub fn push_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "bitmask of enum nft_set_flags\n\nAssociated type: [`SetFlags`] (enum)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "key data type, informational purpose only\n"]
    pub fn push_key_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "key data length\n"]
    pub fn push_key_len(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "mapping data type\n"]
    pub fn push_data_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "mapping data length\n"]
    pub fn push_data_len(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "selection policy\n"]
    pub fn push_policy(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "set description\n"]
    pub fn nested_desc(mut self) -> PushSetDescAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 9u16);
        PushSetDescAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "uniquely identifies a set in a transaction\n"]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "default timeout value\n"]
    pub fn push_timeout(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 11u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "garbage collection interval\n"]
    pub fn push_gc_interval(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 12u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "user data\n"]
    pub fn push_userdata(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 13u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 14u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "stateful object type\n"]
    pub fn push_obj_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 15u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "set handle\n"]
    pub fn push_handle(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 16u16, 8 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "set expression\n\nAttribute may repeat multiple times (treat it as array)"]
    pub fn nested_expr(mut self) -> PushExprAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 17u16);
        PushExprAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "list of expressions\n"]
    pub fn nested_expressions(mut self) -> PushSetListAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 18u16);
        PushSetListAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "set backend type\n"]
    pub fn push_type(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            19u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "set backend type\n"]
    pub fn push_type_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 19u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "number of set elements\n"]
    pub fn push_count(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 20u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushSetAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSetDescAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushSetDescAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushSetDescAttrs<Prev> {
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
    #[doc = "number of elements in set\n"]
    pub fn push_size(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "description of field concatenation\n\nAttribute may repeat multiple times (treat it as array)"]
    pub fn nested_concat(mut self) -> PushSetDescConcatAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushSetDescConcatAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushSetDescAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSetDescConcatAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushSetDescConcatAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushSetDescConcatAttrs<Prev> {
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
    pub fn nested_elem(mut self) -> PushSetFieldAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushSetFieldAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushSetDescConcatAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSetFieldAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushSetFieldAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushSetFieldAttrs<Prev> {
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
    pub fn push_len(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushSetFieldAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSetListAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushSetListAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushSetListAttrs<Prev> {
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
    pub fn nested_elem(mut self) -> PushExprAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushExprAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushSetListAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSetelemAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushSetelemAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushSetelemAttrs<Prev> {
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
    #[doc = "key value\n"]
    pub fn nested_key(mut self) -> PushDataAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushDataAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "data value of mapping\n"]
    pub fn nested_data(mut self) -> PushDataAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushDataAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "bitmask of nft_set_elem_flags\n"]
    pub fn push_flags(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "timeout value\n"]
    pub fn push_timeout(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 4u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "expiration time\n"]
    pub fn push_expiration(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 5u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "user data\n"]
    pub fn push_userdata(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 6u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "expression\n"]
    pub fn nested_expr(mut self) -> PushExprAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 7u16);
        PushExprAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "stateful object reference\n"]
    pub fn push_objref(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            8u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "stateful object reference\n"]
    pub fn push_objref_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 8u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "closing key value\n"]
    pub fn nested_key_end(mut self) -> PushDataAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 9u16);
        PushDataAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "list of expressions\n"]
    pub fn nested_expressions(mut self) -> PushExprListAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 10u16);
        PushExprListAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushSetelemAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSetelemListElemAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushSetelemListElemAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushSetelemListElemAttrs<Prev> {
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
    pub fn nested_elem(mut self) -> PushSetelemAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushSetelemAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushSetelemListElemAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSetelemListAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushSetelemListAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushSetelemListAttrs<Prev> {
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
    pub fn push_table(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_table_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_set(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_set_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn nested_elements(mut self) -> PushSetelemListElemAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 3u16);
        PushSetelemListElemAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_set_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushSetelemListAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushGenAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushGenAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushGenAttrs<Prev> {
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
    #[doc = "ruleset generation id\n"]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_proc_pid(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_proc_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            3u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_proc_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
}
impl<Prev: Pusher> Drop for PushGenAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushObjAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushObjAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushObjAttrs<Prev> {
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
    #[doc = "name of the table containing the expression\n"]
    pub fn push_table(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "name of the table containing the expression\n"]
    pub fn push_table_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "name of this expression type\n"]
    pub fn push_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "name of this expression type\n"]
    pub fn push_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "stateful object type\n\nAssociated type: [`ObjectType`] (enum)"]
    pub fn push_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "stateful object data\n"]
    #[doc = "Selector attribute `type` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_counter(mut self) -> PushCounterAttrs<PushDummy<Prev>> {
        self = self.push_type(ObjectType::Counter as u32);
        let new_header_offset = push_nested_header(self.as_vec_mut(), 4u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushCounterAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "Selector attribute `type` is inserted automatically."]
    #[doc = "At most one sub-message attribute is expected per attribute set."]
    pub fn nested_data_quota(mut self) -> PushQuotaAttrs<PushDummy<Prev>> {
        self = self.push_type(ObjectType::Quota as u32);
        let new_header_offset = push_nested_header(self.as_vec_mut(), 4u16);
        let dummy = PushDummy {
            prev: self.prev.take(),
            header_offset: self.header_offset.take(),
        };
        PushQuotaAttrs {
            prev: Some(dummy),
            header_offset: Some(new_header_offset),
        }
    }
    #[doc = "number of references to this expression\n"]
    pub fn push_use(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "object handle\n"]
    pub fn push_handle(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 6u16, 8 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 7u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "user data\n"]
    pub fn push_userdata(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 8u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushObjAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushQuotaAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushQuotaAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushQuotaAttrs<Prev> {
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
    pub fn push_bytes(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 1u16, 8 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "Associated type: [`QuotaFlags`] (enum)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_consumed(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 4u16, 8 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushQuotaAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushFlowtableAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushFlowtableAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushFlowtableAttrs<Prev> {
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
    pub fn push_table(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_table_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
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
    pub fn nested_hook(mut self) -> PushFlowtableHookAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 3u16);
        PushFlowtableHookAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_use(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_handle(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 5u16, 8 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 6u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushFlowtableAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushFlowtableHookAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushFlowtableHookAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushFlowtableHookAttrs<Prev> {
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
    pub fn push_num(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_priority(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn nested_devs(mut self) -> PushHookDevAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 3u16);
        PushHookDevAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushFlowtableHookAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushExprBitwiseAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushExprBitwiseAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushExprBitwiseAttrs<Prev> {
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
    pub fn push_sreg(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_dreg(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_len(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn nested_mask(mut self) -> PushDataAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 4u16);
        PushDataAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_xor(mut self) -> PushDataAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 5u16);
        PushDataAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Associated type: [`BitwiseOps`] (enum)"]
    pub fn push_op(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn nested_data(mut self) -> PushDataAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 7u16);
        PushDataAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushExprBitwiseAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushExprCmpAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushExprCmpAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushExprCmpAttrs<Prev> {
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
    pub fn push_sreg(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "Associated type: [`CmpOps`] (enum)"]
    pub fn push_op(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn nested_data(mut self) -> PushDataAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 3u16);
        PushDataAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushExprCmpAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDataAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDataAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDataAttrs<Prev> {
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
    pub fn push_value(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn nested_verdict(mut self) -> PushVerdictAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushVerdictAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushDataAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushVerdictAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushVerdictAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushVerdictAttrs<Prev> {
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
    #[doc = "nf_tables verdict\n\nAssociated type: [`VerdictCode`] (enum)"]
    pub fn push_code(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "jump target chain name\n"]
    pub fn push_chain(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "jump target chain name\n"]
    pub fn push_chain_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "jump target chain ID\n"]
    pub fn push_chain_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushVerdictAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushExprCounterAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushExprCounterAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushExprCounterAttrs<Prev> {
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
    #[doc = "Number of bytes\n"]
    pub fn push_bytes(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 1u16, 8 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "Number of packets\n"]
    pub fn push_packets(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 2u16, 8 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushExprCounterAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushExprFibAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushExprFibAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushExprFibAttrs<Prev> {
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
    pub fn push_dreg(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "Associated type: [`FibResult`] (enum)"]
    pub fn push_result(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "Associated type: [`FibFlags`] (enum)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushExprFibAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushExprCtAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushExprCtAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushExprCtAttrs<Prev> {
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
    pub fn push_dreg(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "Associated type: [`CtKeys`] (enum)"]
    pub fn push_key(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "Associated type: [`CtDirection`] (enum)"]
    pub fn push_direction(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 3u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sreg(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushExprCtAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushExprFlowOffloadAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushExprFlowOffloadAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushExprFlowOffloadAttrs<Prev> {
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
    #[doc = "Flow offload table name\n"]
    pub fn push_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "Flow offload table name\n"]
    pub fn push_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
}
impl<Prev: Pusher> Drop for PushExprFlowOffloadAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushExprImmediateAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushExprImmediateAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushExprImmediateAttrs<Prev> {
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
    pub fn push_dreg(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn nested_data(mut self) -> PushDataAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushDataAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushExprImmediateAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushExprLookupAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushExprLookupAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushExprLookupAttrs<Prev> {
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
    #[doc = "Name of set to use\n"]
    pub fn push_set(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "Name of set to use\n"]
    pub fn push_set_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "ID of set to use\n"]
    pub fn push_set_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_sreg(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_dreg(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "Associated type: [`LookupFlags`] (enum)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushExprLookupAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushExprMasqAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushExprMasqAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushExprMasqAttrs<Prev> {
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
    #[doc = "Associated type: [`NatRangeFlags`] (1 bit per enumeration)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "Associated type: [`Registers`] (enum)"]
    pub fn push_reg_proto_min(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "Associated type: [`Registers`] (enum)"]
    pub fn push_reg_proto_max(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushExprMasqAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushExprMetaAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushExprMetaAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushExprMetaAttrs<Prev> {
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
    pub fn push_dreg(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "Associated type: [`MetaKeys`] (enum)"]
    pub fn push_key(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_sreg(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushExprMetaAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushExprNatAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushExprNatAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushExprNatAttrs<Prev> {
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
    pub fn push_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_family(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_reg_addr_min(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_reg_addr_max(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_reg_proto_min(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_reg_proto_max(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "Associated type: [`NatRangeFlags`] (1 bit per enumeration)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushExprNatAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushExprPayloadAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushExprPayloadAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushExprPayloadAttrs<Prev> {
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
    #[doc = "destination register to load data into\n\nAssociated type: [`Registers`] (enum)"]
    pub fn push_dreg(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "payload base\n\nAssociated type: [`PayloadBase`] (enum)"]
    pub fn push_base(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "payload offset relative to base\n"]
    pub fn push_offset(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "payload length\n"]
    pub fn push_len(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "source register to load data from\n\nAssociated type: [`Registers`] (enum)"]
    pub fn push_sreg(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "checksum type\n"]
    pub fn push_csum_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "checksum offset relative to base\n"]
    pub fn push_csum_offset(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "checksum flags\n"]
    pub fn push_csum_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushExprPayloadAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushExprRejectAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushExprRejectAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushExprRejectAttrs<Prev> {
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
    #[doc = "Associated type: [`RejectTypes`] (enum)"]
    pub fn push_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_icmp_code(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 2u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushExprRejectAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushExprTargetAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushExprTargetAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushExprTargetAttrs<Prev> {
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
    pub fn push_rev(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "This attribute is usually carries struct. Unlike other sub-messages, its\ntype depend on values of **both** name and rev.\n\nGrep for `static struct xt_target` in `linux/net/nftables`.\n"]
    pub fn push_info(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushExprTargetAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushExprTproxyAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushExprTproxyAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushExprTproxyAttrs<Prev> {
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
    pub fn push_family(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_reg_addr(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_reg_port(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushExprTproxyAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushExprObjrefAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushExprObjrefAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushExprObjrefAttrs<Prev> {
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
    pub fn push_imm_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "object name\n"]
    pub fn push_imm_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "object name\n"]
    pub fn push_imm_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_set_sreg(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "name of object map\n"]
    pub fn push_set_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            4u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "name of object map\n"]
    pub fn push_set_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 4u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "id of object map\n"]
    pub fn push_set_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushExprObjrefAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushCompatTargetAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushCompatTargetAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushCompatTargetAttrs<Prev> {
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
    pub fn push_rev(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_info(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushCompatTargetAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushCompatMatchAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushCompatMatchAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushCompatMatchAttrs<Prev> {
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
    pub fn push_rev(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_info(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushCompatMatchAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushCompatAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushCompatAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushCompatAttrs<Prev> {
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
    pub fn push_rev(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushCompatAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Start a batch of operations\n\nRequest attributes:\n- [.push_genid()](PushBatchAttrs::push_genid)\n\nReply attributes:\n- [.get_genid()](IterableBatchAttrs::get_genid)\n\n"]
#[derive(Debug)]
pub struct OpBatchBeginDo<'r> {
    request: Request<'r>,
}
impl<'r> OpBatchBeginDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushBatchAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushBatchAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushBatchAttrs<&mut Vec<u8>> {
        PushBatchAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushBatchAttrs<RequestBuf<'r>> {
        PushBatchAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableBatchAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableBatchAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpBatchBeginDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 16u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableBatchAttrs<'buf>);
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
#[doc = "Finish a batch of operations\n\nRequest attributes:\n- [.push_genid()](PushBatchAttrs::push_genid)\n\n"]
#[derive(Debug)]
pub struct OpBatchEndDo<'r> {
    request: Request<'r>,
}
impl<'r> OpBatchEndDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushBatchAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushBatchAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushBatchAttrs<&mut Vec<u8>> {
        PushBatchAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushBatchAttrs<RequestBuf<'r>> {
        PushBatchAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableBatchAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableBatchAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpBatchEndDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 17u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableBatchAttrs<'buf>);
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
#[doc = "Create a new table.\n\nRequest attributes:\n- [.push_name()](PushTableAttrs::push_name)\n- [.push_flags()](PushTableAttrs::push_flags)\n- [.push_userdata()](PushTableAttrs::push_userdata)\n\n"]
#[derive(Debug)]
pub struct OpNewtableDo<'r> {
    request: Request<'r>,
}
impl<'r> OpNewtableDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushTableAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushTableAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushTableAttrs<&mut Vec<u8>> {
        PushTableAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushTableAttrs<RequestBuf<'r>> {
        PushTableAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableTableAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableTableAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpNewtableDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2560u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableTableAttrs<'buf>);
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
#[doc = "Get / dump tables.\n\nReply attributes:\n- [.get_name()](IterableTableAttrs::get_name)\n- [.get_flags()](IterableTableAttrs::get_flags)\n- [.get_use()](IterableTableAttrs::get_use)\n- [.get_handle()](IterableTableAttrs::get_handle)\n- [.get_userdata()](IterableTableAttrs::get_userdata)\n- [.get_owner()](IterableTableAttrs::get_owner)\n\n"]
#[derive(Debug)]
pub struct OpGettableDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGettableDump<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushTableAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushTableAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushTableAttrs<&mut Vec<u8>> {
        PushTableAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushTableAttrs<RequestBuf<'r>> {
        PushTableAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableTableAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableTableAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpGettableDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2561u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableTableAttrs<'buf>);
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
#[doc = "Get / dump tables.\n\nRequest attributes:\n- [.push_name()](PushTableAttrs::push_name)\n\nReply attributes:\n- [.get_name()](IterableTableAttrs::get_name)\n- [.get_flags()](IterableTableAttrs::get_flags)\n- [.get_use()](IterableTableAttrs::get_use)\n- [.get_handle()](IterableTableAttrs::get_handle)\n- [.get_userdata()](IterableTableAttrs::get_userdata)\n- [.get_owner()](IterableTableAttrs::get_owner)\n\n"]
#[derive(Debug)]
pub struct OpGettableDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGettableDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushTableAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushTableAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushTableAttrs<&mut Vec<u8>> {
        PushTableAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushTableAttrs<RequestBuf<'r>> {
        PushTableAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableTableAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableTableAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpGettableDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2561u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableTableAttrs<'buf>);
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
#[doc = "Delete an existing table.\n\nRequest attributes:\n- [.push_name()](PushTableAttrs::push_name)\n- [.push_handle()](PushTableAttrs::push_handle)\n\n"]
#[derive(Debug)]
pub struct OpDeltableDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDeltableDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushTableAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushTableAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushTableAttrs<&mut Vec<u8>> {
        PushTableAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushTableAttrs<RequestBuf<'r>> {
        PushTableAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableTableAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableTableAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpDeltableDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2562u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableTableAttrs<'buf>);
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
#[doc = "Delete an existing table with destroy semantics (ignoring ENOENT\nerrors).\n\nRequest attributes:\n- [.push_name()](PushTableAttrs::push_name)\n- [.push_handle()](PushTableAttrs::push_handle)\n\n"]
#[derive(Debug)]
pub struct OpDestroytableDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDestroytableDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushTableAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushTableAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushTableAttrs<&mut Vec<u8>> {
        PushTableAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushTableAttrs<RequestBuf<'r>> {
        PushTableAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableTableAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableTableAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpDestroytableDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2586u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableTableAttrs<'buf>);
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
#[doc = "Create a new chain.\n\nRequest attributes:\n- [.push_table()](PushChainAttrs::push_table)\n- [.push_handle()](PushChainAttrs::push_handle)\n- [.push_name()](PushChainAttrs::push_name)\n- [.nested_hook()](PushChainAttrs::nested_hook)\n- [.push_policy()](PushChainAttrs::push_policy)\n- [.push_type()](PushChainAttrs::push_type)\n- [.nested_counters()](PushChainAttrs::nested_counters)\n- [.push_flags()](PushChainAttrs::push_flags)\n- [.push_userdata()](PushChainAttrs::push_userdata)\n\n"]
#[derive(Debug)]
pub struct OpNewchainDo<'r> {
    request: Request<'r>,
}
impl<'r> OpNewchainDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushChainAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushChainAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushChainAttrs<&mut Vec<u8>> {
        PushChainAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushChainAttrs<RequestBuf<'r>> {
        PushChainAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableChainAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableChainAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpNewchainDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2563u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableChainAttrs<'buf>);
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
#[doc = "Get / dump chains.\n\nReply attributes:\n- [.get_table()](IterableChainAttrs::get_table)\n- [.get_handle()](IterableChainAttrs::get_handle)\n- [.get_name()](IterableChainAttrs::get_name)\n- [.get_hook()](IterableChainAttrs::get_hook)\n- [.get_policy()](IterableChainAttrs::get_policy)\n- [.get_use()](IterableChainAttrs::get_use)\n- [.get_type()](IterableChainAttrs::get_type)\n- [.get_counters()](IterableChainAttrs::get_counters)\n- [.get_flags()](IterableChainAttrs::get_flags)\n- [.get_id()](IterableChainAttrs::get_id)\n- [.get_userdata()](IterableChainAttrs::get_userdata)\n\n"]
#[derive(Debug)]
pub struct OpGetchainDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetchainDump<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushChainAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushChainAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushChainAttrs<&mut Vec<u8>> {
        PushChainAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushChainAttrs<RequestBuf<'r>> {
        PushChainAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableChainAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableChainAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpGetchainDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2564u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableChainAttrs<'buf>);
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
#[doc = "Get / dump chains.\n\nRequest attributes:\n- [.push_table()](PushChainAttrs::push_table)\n- [.push_name()](PushChainAttrs::push_name)\n\nReply attributes:\n- [.get_table()](IterableChainAttrs::get_table)\n- [.get_handle()](IterableChainAttrs::get_handle)\n- [.get_name()](IterableChainAttrs::get_name)\n- [.get_hook()](IterableChainAttrs::get_hook)\n- [.get_policy()](IterableChainAttrs::get_policy)\n- [.get_use()](IterableChainAttrs::get_use)\n- [.get_type()](IterableChainAttrs::get_type)\n- [.get_counters()](IterableChainAttrs::get_counters)\n- [.get_flags()](IterableChainAttrs::get_flags)\n- [.get_id()](IterableChainAttrs::get_id)\n- [.get_userdata()](IterableChainAttrs::get_userdata)\n\n"]
#[derive(Debug)]
pub struct OpGetchainDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetchainDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushChainAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushChainAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushChainAttrs<&mut Vec<u8>> {
        PushChainAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushChainAttrs<RequestBuf<'r>> {
        PushChainAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableChainAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableChainAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpGetchainDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2564u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableChainAttrs<'buf>);
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
#[doc = "Delete an existing chain.\n\nRequest attributes:\n- [.push_table()](PushChainAttrs::push_table)\n- [.push_handle()](PushChainAttrs::push_handle)\n- [.push_name()](PushChainAttrs::push_name)\n- [.nested_hook()](PushChainAttrs::nested_hook)\n\n"]
#[derive(Debug)]
pub struct OpDelchainDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDelchainDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushChainAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushChainAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushChainAttrs<&mut Vec<u8>> {
        PushChainAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushChainAttrs<RequestBuf<'r>> {
        PushChainAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableChainAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableChainAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpDelchainDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2565u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableChainAttrs<'buf>);
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
#[doc = "Delete an existing chain with destroy semantics (ignoring ENOENT\nerrors).\n\nRequest attributes:\n- [.push_table()](PushChainAttrs::push_table)\n- [.push_handle()](PushChainAttrs::push_handle)\n- [.push_name()](PushChainAttrs::push_name)\n- [.nested_hook()](PushChainAttrs::nested_hook)\n\n"]
#[derive(Debug)]
pub struct OpDestroychainDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDestroychainDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushChainAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushChainAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushChainAttrs<&mut Vec<u8>> {
        PushChainAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushChainAttrs<RequestBuf<'r>> {
        PushChainAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableChainAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableChainAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpDestroychainDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2587u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableChainAttrs<'buf>);
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
#[doc = "Create a new rule.\n\nRequest attributes:\n- [.push_table()](PushRuleAttrs::push_table)\n- [.push_chain()](PushRuleAttrs::push_chain)\n- [.push_handle()](PushRuleAttrs::push_handle)\n- [.nested_expressions()](PushRuleAttrs::nested_expressions)\n- [.nested_compat()](PushRuleAttrs::nested_compat)\n- [.push_position()](PushRuleAttrs::push_position)\n- [.push_userdata()](PushRuleAttrs::push_userdata)\n- [.push_position_id()](PushRuleAttrs::push_position_id)\n- [.push_chain_id()](PushRuleAttrs::push_chain_id)\n\n"]
#[derive(Debug)]
pub struct OpNewruleDo<'r> {
    request: Request<'r>,
}
impl<'r> OpNewruleDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushRuleAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushRuleAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushRuleAttrs<&mut Vec<u8>> {
        PushRuleAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushRuleAttrs<RequestBuf<'r>> {
        PushRuleAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableRuleAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableRuleAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpNewruleDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2566u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableRuleAttrs<'buf>);
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
#[doc = "Get / dump rules.\n\nRequest attributes:\n- [.push_table()](PushRuleAttrs::push_table)\n- [.push_chain()](PushRuleAttrs::push_chain)\n\nReply attributes:\n- [.get_table()](IterableRuleAttrs::get_table)\n- [.get_chain()](IterableRuleAttrs::get_chain)\n- [.get_handle()](IterableRuleAttrs::get_handle)\n- [.get_expressions()](IterableRuleAttrs::get_expressions)\n- [.get_position()](IterableRuleAttrs::get_position)\n- [.get_userdata()](IterableRuleAttrs::get_userdata)\n\n"]
#[derive(Debug)]
pub struct OpGetruleDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetruleDump<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushRuleAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushRuleAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushRuleAttrs<&mut Vec<u8>> {
        PushRuleAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushRuleAttrs<RequestBuf<'r>> {
        PushRuleAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableRuleAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableRuleAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpGetruleDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2567u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableRuleAttrs<'buf>);
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
#[doc = "Get / dump rules.\n\nRequest attributes:\n- [.push_table()](PushRuleAttrs::push_table)\n- [.push_chain()](PushRuleAttrs::push_chain)\n- [.push_handle()](PushRuleAttrs::push_handle)\n\nReply attributes:\n- [.get_table()](IterableRuleAttrs::get_table)\n- [.get_chain()](IterableRuleAttrs::get_chain)\n- [.get_handle()](IterableRuleAttrs::get_handle)\n- [.get_expressions()](IterableRuleAttrs::get_expressions)\n- [.get_position()](IterableRuleAttrs::get_position)\n- [.get_userdata()](IterableRuleAttrs::get_userdata)\n\n"]
#[derive(Debug)]
pub struct OpGetruleDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetruleDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushRuleAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushRuleAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushRuleAttrs<&mut Vec<u8>> {
        PushRuleAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushRuleAttrs<RequestBuf<'r>> {
        PushRuleAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableRuleAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableRuleAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpGetruleDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2567u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableRuleAttrs<'buf>);
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
#[doc = "Get / dump rules and reset stateful expressions.\n\nRequest attributes:\n- [.push_table()](PushRuleAttrs::push_table)\n- [.push_chain()](PushRuleAttrs::push_chain)\n- [.push_handle()](PushRuleAttrs::push_handle)\n\nReply attributes:\n- [.get_table()](IterableRuleAttrs::get_table)\n- [.get_chain()](IterableRuleAttrs::get_chain)\n- [.get_handle()](IterableRuleAttrs::get_handle)\n- [.get_expressions()](IterableRuleAttrs::get_expressions)\n- [.get_position()](IterableRuleAttrs::get_position)\n- [.get_userdata()](IterableRuleAttrs::get_userdata)\n\n"]
#[derive(Debug)]
pub struct OpGetruleResetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetruleResetDump<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushRuleAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushRuleAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushRuleAttrs<&mut Vec<u8>> {
        PushRuleAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushRuleAttrs<RequestBuf<'r>> {
        PushRuleAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableRuleAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableRuleAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpGetruleResetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2585u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableRuleAttrs<'buf>);
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
#[doc = "Get / dump rules and reset stateful expressions.\n\nRequest attributes:\n- [.push_table()](PushRuleAttrs::push_table)\n- [.push_chain()](PushRuleAttrs::push_chain)\n- [.push_handle()](PushRuleAttrs::push_handle)\n\nReply attributes:\n- [.get_table()](IterableRuleAttrs::get_table)\n- [.get_chain()](IterableRuleAttrs::get_chain)\n- [.get_handle()](IterableRuleAttrs::get_handle)\n- [.get_expressions()](IterableRuleAttrs::get_expressions)\n- [.get_position()](IterableRuleAttrs::get_position)\n- [.get_userdata()](IterableRuleAttrs::get_userdata)\n\n"]
#[derive(Debug)]
pub struct OpGetruleResetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetruleResetDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushRuleAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushRuleAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushRuleAttrs<&mut Vec<u8>> {
        PushRuleAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushRuleAttrs<RequestBuf<'r>> {
        PushRuleAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableRuleAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableRuleAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpGetruleResetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2585u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableRuleAttrs<'buf>);
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
#[doc = "Delete an existing rule.\n\nRequest attributes:\n- [.push_table()](PushRuleAttrs::push_table)\n- [.push_chain()](PushRuleAttrs::push_chain)\n- [.push_handle()](PushRuleAttrs::push_handle)\n- [.push_id()](PushRuleAttrs::push_id)\n\n"]
#[derive(Debug)]
pub struct OpDelruleDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDelruleDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushRuleAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushRuleAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushRuleAttrs<&mut Vec<u8>> {
        PushRuleAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushRuleAttrs<RequestBuf<'r>> {
        PushRuleAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableRuleAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableRuleAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpDelruleDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2568u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableRuleAttrs<'buf>);
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
#[doc = "Delete an existing rule with destroy semantics (ignoring ENOENT errors).\n\nRequest attributes:\n- [.push_table()](PushRuleAttrs::push_table)\n- [.push_chain()](PushRuleAttrs::push_chain)\n- [.push_handle()](PushRuleAttrs::push_handle)\n- [.push_id()](PushRuleAttrs::push_id)\n\n"]
#[derive(Debug)]
pub struct OpDestroyruleDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDestroyruleDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushRuleAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushRuleAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushRuleAttrs<&mut Vec<u8>> {
        PushRuleAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushRuleAttrs<RequestBuf<'r>> {
        PushRuleAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableRuleAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableRuleAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpDestroyruleDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2588u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableRuleAttrs<'buf>);
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
#[doc = "Create a new set.\n\nRequest attributes:\n- [.push_table()](PushSetAttrs::push_table)\n- [.push_name()](PushSetAttrs::push_name)\n- [.push_flags()](PushSetAttrs::push_flags)\n- [.push_key_type()](PushSetAttrs::push_key_type)\n- [.push_key_len()](PushSetAttrs::push_key_len)\n- [.push_data_type()](PushSetAttrs::push_data_type)\n- [.push_data_len()](PushSetAttrs::push_data_len)\n- [.push_policy()](PushSetAttrs::push_policy)\n- [.nested_desc()](PushSetAttrs::nested_desc)\n- [.push_id()](PushSetAttrs::push_id)\n- [.push_timeout()](PushSetAttrs::push_timeout)\n- [.push_gc_interval()](PushSetAttrs::push_gc_interval)\n- [.push_userdata()](PushSetAttrs::push_userdata)\n- [.push_obj_type()](PushSetAttrs::push_obj_type)\n\n"]
#[derive(Debug)]
pub struct OpNewsetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpNewsetDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushSetAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushSetAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushSetAttrs<&mut Vec<u8>> {
        PushSetAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushSetAttrs<RequestBuf<'r>> {
        PushSetAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableSetAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableSetAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpNewsetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2569u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableSetAttrs<'buf>);
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
#[doc = "Get / dump sets.\n\nRequest attributes:\n- [.push_table()](PushSetAttrs::push_table)\n\nReply attributes:\n- [.get_table()](IterableSetAttrs::get_table)\n- [.get_name()](IterableSetAttrs::get_name)\n- [.get_flags()](IterableSetAttrs::get_flags)\n- [.get_key_type()](IterableSetAttrs::get_key_type)\n- [.get_key_len()](IterableSetAttrs::get_key_len)\n- [.get_data_type()](IterableSetAttrs::get_data_type)\n- [.get_data_len()](IterableSetAttrs::get_data_len)\n- [.get_policy()](IterableSetAttrs::get_policy)\n- [.get_desc()](IterableSetAttrs::get_desc)\n- [.get_gc_interval()](IterableSetAttrs::get_gc_interval)\n- [.get_userdata()](IterableSetAttrs::get_userdata)\n- [.get_obj_type()](IterableSetAttrs::get_obj_type)\n- [.get_handle()](IterableSetAttrs::get_handle)\n- [.get_expr()](IterableSetAttrs::get_expr)\n- [.get_expressions()](IterableSetAttrs::get_expressions)\n\n"]
#[derive(Debug)]
pub struct OpGetsetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetsetDump<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushSetAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushSetAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushSetAttrs<&mut Vec<u8>> {
        PushSetAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushSetAttrs<RequestBuf<'r>> {
        PushSetAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableSetAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableSetAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpGetsetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2570u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableSetAttrs<'buf>);
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
#[doc = "Get / dump sets.\n\nRequest attributes:\n- [.push_table()](PushSetAttrs::push_table)\n- [.push_name()](PushSetAttrs::push_name)\n\nReply attributes:\n- [.get_table()](IterableSetAttrs::get_table)\n- [.get_name()](IterableSetAttrs::get_name)\n- [.get_flags()](IterableSetAttrs::get_flags)\n- [.get_key_type()](IterableSetAttrs::get_key_type)\n- [.get_key_len()](IterableSetAttrs::get_key_len)\n- [.get_data_type()](IterableSetAttrs::get_data_type)\n- [.get_data_len()](IterableSetAttrs::get_data_len)\n- [.get_policy()](IterableSetAttrs::get_policy)\n- [.get_desc()](IterableSetAttrs::get_desc)\n- [.get_gc_interval()](IterableSetAttrs::get_gc_interval)\n- [.get_userdata()](IterableSetAttrs::get_userdata)\n- [.get_obj_type()](IterableSetAttrs::get_obj_type)\n- [.get_handle()](IterableSetAttrs::get_handle)\n- [.get_expr()](IterableSetAttrs::get_expr)\n- [.get_expressions()](IterableSetAttrs::get_expressions)\n\n"]
#[derive(Debug)]
pub struct OpGetsetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetsetDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushSetAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushSetAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushSetAttrs<&mut Vec<u8>> {
        PushSetAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushSetAttrs<RequestBuf<'r>> {
        PushSetAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableSetAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableSetAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpGetsetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2570u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableSetAttrs<'buf>);
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
#[doc = "Delete an existing set.\n\nRequest attributes:\n- [.push_table()](PushSetAttrs::push_table)\n- [.push_name()](PushSetAttrs::push_name)\n- [.push_handle()](PushSetAttrs::push_handle)\n\n"]
#[derive(Debug)]
pub struct OpDelsetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDelsetDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushSetAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushSetAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushSetAttrs<&mut Vec<u8>> {
        PushSetAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushSetAttrs<RequestBuf<'r>> {
        PushSetAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableSetAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableSetAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpDelsetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2571u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableSetAttrs<'buf>);
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
#[doc = "Delete an existing set with destroy semantics (ignoring ENOENT errors).\n\nRequest attributes:\n- [.push_table()](PushSetAttrs::push_table)\n- [.push_name()](PushSetAttrs::push_name)\n- [.push_handle()](PushSetAttrs::push_handle)\n\n"]
#[derive(Debug)]
pub struct OpDestroysetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDestroysetDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushSetAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushSetAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushSetAttrs<&mut Vec<u8>> {
        PushSetAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushSetAttrs<RequestBuf<'r>> {
        PushSetAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableSetAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableSetAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpDestroysetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2589u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableSetAttrs<'buf>);
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
#[doc = "Create a new set element.\n\nRequest attributes:\n- [.push_table()](PushSetelemListAttrs::push_table)\n- [.push_set()](PushSetelemListAttrs::push_set)\n- [.nested_elements()](PushSetelemListAttrs::nested_elements)\n- [.push_set_id()](PushSetelemListAttrs::push_set_id)\n\n"]
#[derive(Debug)]
pub struct OpNewsetelemDo<'r> {
    request: Request<'r>,
}
impl<'r> OpNewsetelemDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushSetelemListAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushSetelemListAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushSetelemListAttrs<&mut Vec<u8>> {
        PushSetelemListAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushSetelemListAttrs<RequestBuf<'r>> {
        PushSetelemListAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableSetelemListAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableSetelemListAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpNewsetelemDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2572u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableSetelemListAttrs<'buf>);
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
#[doc = "Get / dump set elements.\n\nRequest attributes:\n- [.push_table()](PushSetelemListAttrs::push_table)\n- [.push_set()](PushSetelemListAttrs::push_set)\n\nReply attributes:\n- [.get_table()](IterableSetelemListAttrs::get_table)\n- [.get_set()](IterableSetelemListAttrs::get_set)\n- [.get_elements()](IterableSetelemListAttrs::get_elements)\n\n"]
#[derive(Debug)]
pub struct OpGetsetelemDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetsetelemDump<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushSetelemListAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushSetelemListAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushSetelemListAttrs<&mut Vec<u8>> {
        PushSetelemListAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushSetelemListAttrs<RequestBuf<'r>> {
        PushSetelemListAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableSetelemListAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableSetelemListAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpGetsetelemDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2573u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableSetelemListAttrs<'buf>);
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
#[doc = "Get / dump set elements.\n\nRequest attributes:\n- [.push_table()](PushSetelemListAttrs::push_table)\n- [.push_set()](PushSetelemListAttrs::push_set)\n- [.nested_elements()](PushSetelemListAttrs::nested_elements)\n\nReply attributes:\n- [.get_elements()](IterableSetelemListAttrs::get_elements)\n\n"]
#[derive(Debug)]
pub struct OpGetsetelemDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetsetelemDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushSetelemListAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushSetelemListAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushSetelemListAttrs<&mut Vec<u8>> {
        PushSetelemListAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushSetelemListAttrs<RequestBuf<'r>> {
        PushSetelemListAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableSetelemListAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableSetelemListAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpGetsetelemDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2573u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableSetelemListAttrs<'buf>);
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
#[doc = "Get / dump set elements and reset stateful expressions.\n\nRequest attributes:\n- [.push_table()](PushSetelemListAttrs::push_table)\n- [.push_set()](PushSetelemListAttrs::push_set)\n\nReply attributes:\n- [.get_table()](IterableSetelemListAttrs::get_table)\n- [.get_set()](IterableSetelemListAttrs::get_set)\n- [.get_elements()](IterableSetelemListAttrs::get_elements)\n\n"]
#[derive(Debug)]
pub struct OpGetsetelemResetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetsetelemResetDump<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushSetelemListAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushSetelemListAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushSetelemListAttrs<&mut Vec<u8>> {
        PushSetelemListAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushSetelemListAttrs<RequestBuf<'r>> {
        PushSetelemListAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableSetelemListAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableSetelemListAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpGetsetelemResetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2593u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableSetelemListAttrs<'buf>);
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
#[doc = "Get / dump set elements and reset stateful expressions.\n\nRequest attributes:\n- [.nested_elements()](PushSetelemListAttrs::nested_elements)\n\nReply attributes:\n- [.get_table()](IterableSetelemListAttrs::get_table)\n- [.get_set()](IterableSetelemListAttrs::get_set)\n- [.get_elements()](IterableSetelemListAttrs::get_elements)\n\n"]
#[derive(Debug)]
pub struct OpGetsetelemResetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetsetelemResetDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushSetelemListAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushSetelemListAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushSetelemListAttrs<&mut Vec<u8>> {
        PushSetelemListAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushSetelemListAttrs<RequestBuf<'r>> {
        PushSetelemListAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableSetelemListAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableSetelemListAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpGetsetelemResetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2593u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableSetelemListAttrs<'buf>);
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
#[doc = "Delete an existing set element.\n\nRequest attributes:\n- [.push_table()](PushSetelemListAttrs::push_table)\n- [.push_set()](PushSetelemListAttrs::push_set)\n- [.nested_elements()](PushSetelemListAttrs::nested_elements)\n\n"]
#[derive(Debug)]
pub struct OpDelsetelemDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDelsetelemDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushSetelemListAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushSetelemListAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushSetelemListAttrs<&mut Vec<u8>> {
        PushSetelemListAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushSetelemListAttrs<RequestBuf<'r>> {
        PushSetelemListAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableSetelemListAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableSetelemListAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpDelsetelemDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2574u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableSetelemListAttrs<'buf>);
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
#[doc = "Delete an existing set element with destroy semantics.\n\nRequest attributes:\n- [.push_table()](PushSetelemListAttrs::push_table)\n- [.push_set()](PushSetelemListAttrs::push_set)\n- [.nested_elements()](PushSetelemListAttrs::nested_elements)\n\n"]
#[derive(Debug)]
pub struct OpDestroysetelemDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDestroysetelemDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushSetelemListAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushSetelemListAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushSetelemListAttrs<&mut Vec<u8>> {
        PushSetelemListAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushSetelemListAttrs<RequestBuf<'r>> {
        PushSetelemListAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableSetelemListAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableSetelemListAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpDestroysetelemDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2590u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableSetelemListAttrs<'buf>);
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
#[doc = "Get / dump rule-set generation.\n\nReply attributes:\n- [.get_id()](IterableGenAttrs::get_id)\n- [.get_proc_pid()](IterableGenAttrs::get_proc_pid)\n- [.get_proc_name()](IterableGenAttrs::get_proc_name)\n\n"]
#[derive(Debug)]
pub struct OpGetgenDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetgenDump<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushGenAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushGenAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushGenAttrs<&mut Vec<u8>> {
        PushGenAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushGenAttrs<RequestBuf<'r>> {
        PushGenAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableGenAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableGenAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpGetgenDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2576u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableGenAttrs<'buf>);
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
#[doc = "Get / dump rule-set generation.\n\nReply attributes:\n- [.get_id()](IterableGenAttrs::get_id)\n- [.get_proc_pid()](IterableGenAttrs::get_proc_pid)\n- [.get_proc_name()](IterableGenAttrs::get_proc_name)\n\n"]
#[derive(Debug)]
pub struct OpGetgenDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetgenDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushGenAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushGenAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushGenAttrs<&mut Vec<u8>> {
        PushGenAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushGenAttrs<RequestBuf<'r>> {
        PushGenAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableGenAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableGenAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpGetgenDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2576u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableGenAttrs<'buf>);
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
#[doc = "Create a new stateful object.\n\nRequest attributes:\n- [.push_table()](PushObjAttrs::push_table)\n- [.push_name()](PushObjAttrs::push_name)\n- [.push_type()](PushObjAttrs::push_type)\n- [.nested_data_counter()](PushObjAttrs::nested_data_counter)\n- [.nested_data_quota()](PushObjAttrs::nested_data_quota)\n- [.push_userdata()](PushObjAttrs::push_userdata)\n\n"]
#[derive(Debug)]
pub struct OpNewobjDo<'r> {
    request: Request<'r>,
}
impl<'r> OpNewobjDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushObjAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushObjAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushObjAttrs<&mut Vec<u8>> {
        PushObjAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushObjAttrs<RequestBuf<'r>> {
        PushObjAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableObjAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableObjAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpNewobjDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2578u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableObjAttrs<'buf>);
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
#[doc = "Get / dump stateful objects.\n\nRequest attributes:\n- [.push_table()](PushObjAttrs::push_table)\n- [.push_type()](PushObjAttrs::push_type)\n\nReply attributes:\n- [.get_table()](IterableObjAttrs::get_table)\n- [.get_name()](IterableObjAttrs::get_name)\n- [.get_type()](IterableObjAttrs::get_type)\n- [.get_data()](IterableObjAttrs::get_data)\n- [.get_use()](IterableObjAttrs::get_use)\n- [.get_handle()](IterableObjAttrs::get_handle)\n- [.get_userdata()](IterableObjAttrs::get_userdata)\n\n"]
#[derive(Debug)]
pub struct OpGetobjDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetobjDump<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushObjAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushObjAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushObjAttrs<&mut Vec<u8>> {
        PushObjAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushObjAttrs<RequestBuf<'r>> {
        PushObjAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableObjAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableObjAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpGetobjDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2579u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableObjAttrs<'buf>);
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
#[doc = "Get / dump stateful objects.\n\nRequest attributes:\n- [.push_table()](PushObjAttrs::push_table)\n- [.push_name()](PushObjAttrs::push_name)\n- [.push_type()](PushObjAttrs::push_type)\n\nReply attributes:\n- [.get_table()](IterableObjAttrs::get_table)\n- [.get_name()](IterableObjAttrs::get_name)\n- [.get_type()](IterableObjAttrs::get_type)\n- [.get_data()](IterableObjAttrs::get_data)\n- [.get_use()](IterableObjAttrs::get_use)\n- [.get_handle()](IterableObjAttrs::get_handle)\n- [.get_userdata()](IterableObjAttrs::get_userdata)\n\n"]
#[derive(Debug)]
pub struct OpGetobjDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetobjDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushObjAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushObjAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushObjAttrs<&mut Vec<u8>> {
        PushObjAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushObjAttrs<RequestBuf<'r>> {
        PushObjAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableObjAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableObjAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpGetobjDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2579u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableObjAttrs<'buf>);
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
#[doc = "Delete an existing stateful object.\n\nRequest attributes:\n- [.push_table()](PushObjAttrs::push_table)\n- [.push_name()](PushObjAttrs::push_name)\n- [.push_type()](PushObjAttrs::push_type)\n- [.push_handle()](PushObjAttrs::push_handle)\n\n"]
#[derive(Debug)]
pub struct OpDelobjDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDelobjDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushObjAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushObjAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushObjAttrs<&mut Vec<u8>> {
        PushObjAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushObjAttrs<RequestBuf<'r>> {
        PushObjAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableObjAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableObjAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpDelobjDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2580u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableObjAttrs<'buf>);
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
#[doc = "Delete an existing stateful object with destroy semantics.\n\nRequest attributes:\n- [.push_table()](PushObjAttrs::push_table)\n- [.push_name()](PushObjAttrs::push_name)\n- [.push_type()](PushObjAttrs::push_type)\n- [.push_handle()](PushObjAttrs::push_handle)\n\n"]
#[derive(Debug)]
pub struct OpDestroyobjDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDestroyobjDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushObjAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushObjAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushObjAttrs<&mut Vec<u8>> {
        PushObjAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushObjAttrs<RequestBuf<'r>> {
        PushObjAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableObjAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableObjAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpDestroyobjDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2591u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableObjAttrs<'buf>);
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
#[doc = "Create a new flow table.\n\nRequest attributes:\n- [.push_table()](PushFlowtableAttrs::push_table)\n- [.push_name()](PushFlowtableAttrs::push_name)\n- [.nested_hook()](PushFlowtableAttrs::nested_hook)\n- [.push_flags()](PushFlowtableAttrs::push_flags)\n\n"]
#[derive(Debug)]
pub struct OpNewflowtableDo<'r> {
    request: Request<'r>,
}
impl<'r> OpNewflowtableDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushFlowtableAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushFlowtableAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushFlowtableAttrs<&mut Vec<u8>> {
        PushFlowtableAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushFlowtableAttrs<RequestBuf<'r>> {
        PushFlowtableAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableFlowtableAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableFlowtableAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpNewflowtableDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2582u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableFlowtableAttrs<'buf>);
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
#[doc = "Get / dump flow tables.\n\nReply attributes:\n- [.get_table()](IterableFlowtableAttrs::get_table)\n- [.get_name()](IterableFlowtableAttrs::get_name)\n- [.get_hook()](IterableFlowtableAttrs::get_hook)\n- [.get_use()](IterableFlowtableAttrs::get_use)\n- [.get_handle()](IterableFlowtableAttrs::get_handle)\n- [.get_flags()](IterableFlowtableAttrs::get_flags)\n\n"]
#[derive(Debug)]
pub struct OpGetflowtableDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetflowtableDump<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushFlowtableAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushFlowtableAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushFlowtableAttrs<&mut Vec<u8>> {
        PushFlowtableAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushFlowtableAttrs<RequestBuf<'r>> {
        PushFlowtableAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableFlowtableAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableFlowtableAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpGetflowtableDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2583u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableFlowtableAttrs<'buf>);
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
#[doc = "Get / dump flow tables.\n\nRequest attributes:\n- [.push_table()](PushFlowtableAttrs::push_table)\n- [.push_name()](PushFlowtableAttrs::push_name)\n\nReply attributes:\n- [.get_table()](IterableFlowtableAttrs::get_table)\n- [.get_name()](IterableFlowtableAttrs::get_name)\n- [.get_hook()](IterableFlowtableAttrs::get_hook)\n- [.get_use()](IterableFlowtableAttrs::get_use)\n- [.get_handle()](IterableFlowtableAttrs::get_handle)\n- [.get_flags()](IterableFlowtableAttrs::get_flags)\n\n"]
#[derive(Debug)]
pub struct OpGetflowtableDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetflowtableDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushFlowtableAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushFlowtableAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushFlowtableAttrs<&mut Vec<u8>> {
        PushFlowtableAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushFlowtableAttrs<RequestBuf<'r>> {
        PushFlowtableAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableFlowtableAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableFlowtableAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpGetflowtableDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2583u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableFlowtableAttrs<'buf>);
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
#[doc = "Delete an existing flow table.\n\nRequest attributes:\n- [.push_table()](PushFlowtableAttrs::push_table)\n- [.push_name()](PushFlowtableAttrs::push_name)\n- [.nested_hook()](PushFlowtableAttrs::nested_hook)\n- [.push_handle()](PushFlowtableAttrs::push_handle)\n\n"]
#[derive(Debug)]
pub struct OpDelflowtableDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDelflowtableDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushFlowtableAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushFlowtableAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushFlowtableAttrs<&mut Vec<u8>> {
        PushFlowtableAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushFlowtableAttrs<RequestBuf<'r>> {
        PushFlowtableAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableFlowtableAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableFlowtableAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpDelflowtableDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2584u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableFlowtableAttrs<'buf>);
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
#[doc = "Delete an existing flow table with destroy semantics.\n\nRequest attributes:\n- [.push_table()](PushFlowtableAttrs::push_table)\n- [.push_name()](PushFlowtableAttrs::push_name)\n- [.nested_hook()](PushFlowtableAttrs::nested_hook)\n- [.push_handle()](PushFlowtableAttrs::push_handle)\n\n"]
#[derive(Debug)]
pub struct OpDestroyflowtableDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDestroyflowtableDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushFlowtableAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushFlowtableAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushFlowtableAttrs<&mut Vec<u8>> {
        PushFlowtableAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushFlowtableAttrs<RequestBuf<'r>> {
        PushFlowtableAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableFlowtableAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableFlowtableAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpDestroyflowtableDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2592u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableFlowtableAttrs<'buf>);
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
#[doc = "Get / dump nft_compat info\n\nReply attributes:\n- [.get_name()](IterableCompatAttrs::get_name)\n- [.get_rev()](IterableCompatAttrs::get_rev)\n- [.get_type()](IterableCompatAttrs::get_type)\n\n"]
#[derive(Debug)]
pub struct OpGetcompatDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetcompatDump<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushCompatAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushCompatAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushCompatAttrs<&mut Vec<u8>> {
        PushCompatAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushCompatAttrs<RequestBuf<'r>> {
        PushCompatAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableCompatAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableCompatAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpGetcompatDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2816u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableCompatAttrs<'buf>);
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
#[doc = "Get / dump nft_compat info\n\nRequest attributes:\n- [.push_name()](PushCompatAttrs::push_name)\n- [.push_rev()](PushCompatAttrs::push_rev)\n- [.push_type()](PushCompatAttrs::push_type)\n\nReply attributes:\n- [.get_name()](IterableCompatAttrs::get_name)\n- [.get_rev()](IterableCompatAttrs::get_rev)\n- [.get_type()](IterableCompatAttrs::get_type)\n\n"]
#[derive(Debug)]
pub struct OpGetcompatDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetcompatDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Nfgenmsg,
    ) -> PushCompatAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushCompatAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushCompatAttrs<&mut Vec<u8>> {
        PushCompatAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushCompatAttrs<RequestBuf<'r>> {
        PushCompatAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableCompatAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableCompatAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpGetcompatDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: 2816u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableCompatAttrs<'buf>);
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
#[doc = ""]
#[derive(Debug)]
pub struct OpDump<'r> {
    request: Request<'r>,
    request_type: u16,
}
impl<'r> OpDump<'r> {
    pub fn new(mut request: Request<'r>, request_type: u16, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
            request_type,
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        request_type: u16,
        header: &Nfgenmsg,
    ) -> PushRuleAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushRuleAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushRuleAttrs<&mut Vec<u8>> {
        PushRuleAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushRuleAttrs<RequestBuf<'r>> {
        PushRuleAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableRuleAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableRuleAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: self.request_type,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableRuleAttrs<'buf>);
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
#[doc = ""]
#[derive(Debug)]
pub struct OpDo<'r> {
    request: Request<'r>,
    request_type: u16,
}
impl<'r> OpDo<'r> {
    pub fn new(mut request: Request<'r>, request_type: u16, header: &Nfgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request,
            request_type,
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        request_type: u16,
        header: &Nfgenmsg,
    ) -> PushRuleAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushRuleAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushRuleAttrs<&mut Vec<u8>> {
        PushRuleAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushRuleAttrs<RequestBuf<'r>> {
        PushRuleAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Nfgenmsg, IterableRuleAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Nfgenmsg::len()));
        (
            Nfgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableRuleAttrs::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 12u16,
            request_type: self.request_type,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Nfgenmsg, IterableRuleAttrs<'buf>);
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
    fn supports_ack(&self, index: usize) -> Option<bool> {
        Some(match self.inner.lookups.get(index)?.1 as *const LookupFn {
            f if f == OpBatchBeginDo::lookup as *const LookupFn => false,
            f if f == OpBatchEndDo::lookup as *const LookupFn => false,
            _ => true,
        })
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
    #[doc = "Start a batch of operations\n\nRequest attributes:\n- [.push_genid()](PushBatchAttrs::push_genid)\n\nReply attributes:\n- [.get_genid()](IterableBatchAttrs::get_genid)\n\n"]
    pub fn op_batch_begin_do(self, header: &Nfgenmsg) -> OpBatchBeginDo<'buf> {
        let mut res = OpBatchBeginDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-batch-begin-do", OpBatchBeginDo::lookup);
        res
    }
    #[doc = "Finish a batch of operations\n\nRequest attributes:\n- [.push_genid()](PushBatchAttrs::push_genid)\n\n"]
    pub fn op_batch_end_do(self, header: &Nfgenmsg) -> OpBatchEndDo<'buf> {
        let mut res = OpBatchEndDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-batch-end-do", OpBatchEndDo::lookup);
        res
    }
    #[doc = "Create a new table.\n\nRequest attributes:\n- [.push_name()](PushTableAttrs::push_name)\n- [.push_flags()](PushTableAttrs::push_flags)\n- [.push_userdata()](PushTableAttrs::push_userdata)\n\n"]
    pub fn op_newtable_do(self, header: &Nfgenmsg) -> OpNewtableDo<'buf> {
        let mut res = OpNewtableDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-newtable-do", OpNewtableDo::lookup);
        res
    }
    #[doc = "Get / dump tables.\n\nReply attributes:\n- [.get_name()](IterableTableAttrs::get_name)\n- [.get_flags()](IterableTableAttrs::get_flags)\n- [.get_use()](IterableTableAttrs::get_use)\n- [.get_handle()](IterableTableAttrs::get_handle)\n- [.get_userdata()](IterableTableAttrs::get_userdata)\n- [.get_owner()](IterableTableAttrs::get_owner)\n\n"]
    pub fn op_gettable_dump(self, header: &Nfgenmsg) -> OpGettableDump<'buf> {
        let mut res = OpGettableDump::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-gettable-dump", OpGettableDump::lookup);
        res
    }
    #[doc = "Get / dump tables.\n\nRequest attributes:\n- [.push_name()](PushTableAttrs::push_name)\n\nReply attributes:\n- [.get_name()](IterableTableAttrs::get_name)\n- [.get_flags()](IterableTableAttrs::get_flags)\n- [.get_use()](IterableTableAttrs::get_use)\n- [.get_handle()](IterableTableAttrs::get_handle)\n- [.get_userdata()](IterableTableAttrs::get_userdata)\n- [.get_owner()](IterableTableAttrs::get_owner)\n\n"]
    pub fn op_gettable_do(self, header: &Nfgenmsg) -> OpGettableDo<'buf> {
        let mut res = OpGettableDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-gettable-do", OpGettableDo::lookup);
        res
    }
    #[doc = "Delete an existing table.\n\nRequest attributes:\n- [.push_name()](PushTableAttrs::push_name)\n- [.push_handle()](PushTableAttrs::push_handle)\n\n"]
    pub fn op_deltable_do(self, header: &Nfgenmsg) -> OpDeltableDo<'buf> {
        let mut res = OpDeltableDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-deltable-do", OpDeltableDo::lookup);
        res
    }
    #[doc = "Delete an existing table with destroy semantics (ignoring ENOENT\nerrors).\n\nRequest attributes:\n- [.push_name()](PushTableAttrs::push_name)\n- [.push_handle()](PushTableAttrs::push_handle)\n\n"]
    pub fn op_destroytable_do(self, header: &Nfgenmsg) -> OpDestroytableDo<'buf> {
        let mut res = OpDestroytableDo::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-destroytable-do",
            OpDestroytableDo::lookup,
        );
        res
    }
    #[doc = "Create a new chain.\n\nRequest attributes:\n- [.push_table()](PushChainAttrs::push_table)\n- [.push_handle()](PushChainAttrs::push_handle)\n- [.push_name()](PushChainAttrs::push_name)\n- [.nested_hook()](PushChainAttrs::nested_hook)\n- [.push_policy()](PushChainAttrs::push_policy)\n- [.push_type()](PushChainAttrs::push_type)\n- [.nested_counters()](PushChainAttrs::nested_counters)\n- [.push_flags()](PushChainAttrs::push_flags)\n- [.push_userdata()](PushChainAttrs::push_userdata)\n\n"]
    pub fn op_newchain_do(self, header: &Nfgenmsg) -> OpNewchainDo<'buf> {
        let mut res = OpNewchainDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-newchain-do", OpNewchainDo::lookup);
        res
    }
    #[doc = "Get / dump chains.\n\nReply attributes:\n- [.get_table()](IterableChainAttrs::get_table)\n- [.get_handle()](IterableChainAttrs::get_handle)\n- [.get_name()](IterableChainAttrs::get_name)\n- [.get_hook()](IterableChainAttrs::get_hook)\n- [.get_policy()](IterableChainAttrs::get_policy)\n- [.get_use()](IterableChainAttrs::get_use)\n- [.get_type()](IterableChainAttrs::get_type)\n- [.get_counters()](IterableChainAttrs::get_counters)\n- [.get_flags()](IterableChainAttrs::get_flags)\n- [.get_id()](IterableChainAttrs::get_id)\n- [.get_userdata()](IterableChainAttrs::get_userdata)\n\n"]
    pub fn op_getchain_dump(self, header: &Nfgenmsg) -> OpGetchainDump<'buf> {
        let mut res = OpGetchainDump::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getchain-dump", OpGetchainDump::lookup);
        res
    }
    #[doc = "Get / dump chains.\n\nRequest attributes:\n- [.push_table()](PushChainAttrs::push_table)\n- [.push_name()](PushChainAttrs::push_name)\n\nReply attributes:\n- [.get_table()](IterableChainAttrs::get_table)\n- [.get_handle()](IterableChainAttrs::get_handle)\n- [.get_name()](IterableChainAttrs::get_name)\n- [.get_hook()](IterableChainAttrs::get_hook)\n- [.get_policy()](IterableChainAttrs::get_policy)\n- [.get_use()](IterableChainAttrs::get_use)\n- [.get_type()](IterableChainAttrs::get_type)\n- [.get_counters()](IterableChainAttrs::get_counters)\n- [.get_flags()](IterableChainAttrs::get_flags)\n- [.get_id()](IterableChainAttrs::get_id)\n- [.get_userdata()](IterableChainAttrs::get_userdata)\n\n"]
    pub fn op_getchain_do(self, header: &Nfgenmsg) -> OpGetchainDo<'buf> {
        let mut res = OpGetchainDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getchain-do", OpGetchainDo::lookup);
        res
    }
    #[doc = "Delete an existing chain.\n\nRequest attributes:\n- [.push_table()](PushChainAttrs::push_table)\n- [.push_handle()](PushChainAttrs::push_handle)\n- [.push_name()](PushChainAttrs::push_name)\n- [.nested_hook()](PushChainAttrs::nested_hook)\n\n"]
    pub fn op_delchain_do(self, header: &Nfgenmsg) -> OpDelchainDo<'buf> {
        let mut res = OpDelchainDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-delchain-do", OpDelchainDo::lookup);
        res
    }
    #[doc = "Delete an existing chain with destroy semantics (ignoring ENOENT\nerrors).\n\nRequest attributes:\n- [.push_table()](PushChainAttrs::push_table)\n- [.push_handle()](PushChainAttrs::push_handle)\n- [.push_name()](PushChainAttrs::push_name)\n- [.nested_hook()](PushChainAttrs::nested_hook)\n\n"]
    pub fn op_destroychain_do(self, header: &Nfgenmsg) -> OpDestroychainDo<'buf> {
        let mut res = OpDestroychainDo::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-destroychain-do",
            OpDestroychainDo::lookup,
        );
        res
    }
    #[doc = "Create a new rule.\n\nRequest attributes:\n- [.push_table()](PushRuleAttrs::push_table)\n- [.push_chain()](PushRuleAttrs::push_chain)\n- [.push_handle()](PushRuleAttrs::push_handle)\n- [.nested_expressions()](PushRuleAttrs::nested_expressions)\n- [.nested_compat()](PushRuleAttrs::nested_compat)\n- [.push_position()](PushRuleAttrs::push_position)\n- [.push_userdata()](PushRuleAttrs::push_userdata)\n- [.push_position_id()](PushRuleAttrs::push_position_id)\n- [.push_chain_id()](PushRuleAttrs::push_chain_id)\n\n"]
    pub fn op_newrule_do(self, header: &Nfgenmsg) -> OpNewruleDo<'buf> {
        let mut res = OpNewruleDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-newrule-do", OpNewruleDo::lookup);
        res
    }
    #[doc = "Get / dump rules.\n\nRequest attributes:\n- [.push_table()](PushRuleAttrs::push_table)\n- [.push_chain()](PushRuleAttrs::push_chain)\n\nReply attributes:\n- [.get_table()](IterableRuleAttrs::get_table)\n- [.get_chain()](IterableRuleAttrs::get_chain)\n- [.get_handle()](IterableRuleAttrs::get_handle)\n- [.get_expressions()](IterableRuleAttrs::get_expressions)\n- [.get_position()](IterableRuleAttrs::get_position)\n- [.get_userdata()](IterableRuleAttrs::get_userdata)\n\n"]
    pub fn op_getrule_dump(self, header: &Nfgenmsg) -> OpGetruleDump<'buf> {
        let mut res = OpGetruleDump::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getrule-dump", OpGetruleDump::lookup);
        res
    }
    #[doc = "Get / dump rules.\n\nRequest attributes:\n- [.push_table()](PushRuleAttrs::push_table)\n- [.push_chain()](PushRuleAttrs::push_chain)\n- [.push_handle()](PushRuleAttrs::push_handle)\n\nReply attributes:\n- [.get_table()](IterableRuleAttrs::get_table)\n- [.get_chain()](IterableRuleAttrs::get_chain)\n- [.get_handle()](IterableRuleAttrs::get_handle)\n- [.get_expressions()](IterableRuleAttrs::get_expressions)\n- [.get_position()](IterableRuleAttrs::get_position)\n- [.get_userdata()](IterableRuleAttrs::get_userdata)\n\n"]
    pub fn op_getrule_do(self, header: &Nfgenmsg) -> OpGetruleDo<'buf> {
        let mut res = OpGetruleDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getrule-do", OpGetruleDo::lookup);
        res
    }
    #[doc = "Get / dump rules and reset stateful expressions.\n\nRequest attributes:\n- [.push_table()](PushRuleAttrs::push_table)\n- [.push_chain()](PushRuleAttrs::push_chain)\n- [.push_handle()](PushRuleAttrs::push_handle)\n\nReply attributes:\n- [.get_table()](IterableRuleAttrs::get_table)\n- [.get_chain()](IterableRuleAttrs::get_chain)\n- [.get_handle()](IterableRuleAttrs::get_handle)\n- [.get_expressions()](IterableRuleAttrs::get_expressions)\n- [.get_position()](IterableRuleAttrs::get_position)\n- [.get_userdata()](IterableRuleAttrs::get_userdata)\n\n"]
    pub fn op_getrule_reset_dump(self, header: &Nfgenmsg) -> OpGetruleResetDump<'buf> {
        let mut res = OpGetruleResetDump::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-getrule-reset-dump",
            OpGetruleResetDump::lookup,
        );
        res
    }
    #[doc = "Get / dump rules and reset stateful expressions.\n\nRequest attributes:\n- [.push_table()](PushRuleAttrs::push_table)\n- [.push_chain()](PushRuleAttrs::push_chain)\n- [.push_handle()](PushRuleAttrs::push_handle)\n\nReply attributes:\n- [.get_table()](IterableRuleAttrs::get_table)\n- [.get_chain()](IterableRuleAttrs::get_chain)\n- [.get_handle()](IterableRuleAttrs::get_handle)\n- [.get_expressions()](IterableRuleAttrs::get_expressions)\n- [.get_position()](IterableRuleAttrs::get_position)\n- [.get_userdata()](IterableRuleAttrs::get_userdata)\n\n"]
    pub fn op_getrule_reset_do(self, header: &Nfgenmsg) -> OpGetruleResetDo<'buf> {
        let mut res = OpGetruleResetDo::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-getrule-reset-do",
            OpGetruleResetDo::lookup,
        );
        res
    }
    #[doc = "Delete an existing rule.\n\nRequest attributes:\n- [.push_table()](PushRuleAttrs::push_table)\n- [.push_chain()](PushRuleAttrs::push_chain)\n- [.push_handle()](PushRuleAttrs::push_handle)\n- [.push_id()](PushRuleAttrs::push_id)\n\n"]
    pub fn op_delrule_do(self, header: &Nfgenmsg) -> OpDelruleDo<'buf> {
        let mut res = OpDelruleDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-delrule-do", OpDelruleDo::lookup);
        res
    }
    #[doc = "Delete an existing rule with destroy semantics (ignoring ENOENT errors).\n\nRequest attributes:\n- [.push_table()](PushRuleAttrs::push_table)\n- [.push_chain()](PushRuleAttrs::push_chain)\n- [.push_handle()](PushRuleAttrs::push_handle)\n- [.push_id()](PushRuleAttrs::push_id)\n\n"]
    pub fn op_destroyrule_do(self, header: &Nfgenmsg) -> OpDestroyruleDo<'buf> {
        let mut res = OpDestroyruleDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-destroyrule-do", OpDestroyruleDo::lookup);
        res
    }
    #[doc = "Create a new set.\n\nRequest attributes:\n- [.push_table()](PushSetAttrs::push_table)\n- [.push_name()](PushSetAttrs::push_name)\n- [.push_flags()](PushSetAttrs::push_flags)\n- [.push_key_type()](PushSetAttrs::push_key_type)\n- [.push_key_len()](PushSetAttrs::push_key_len)\n- [.push_data_type()](PushSetAttrs::push_data_type)\n- [.push_data_len()](PushSetAttrs::push_data_len)\n- [.push_policy()](PushSetAttrs::push_policy)\n- [.nested_desc()](PushSetAttrs::nested_desc)\n- [.push_id()](PushSetAttrs::push_id)\n- [.push_timeout()](PushSetAttrs::push_timeout)\n- [.push_gc_interval()](PushSetAttrs::push_gc_interval)\n- [.push_userdata()](PushSetAttrs::push_userdata)\n- [.push_obj_type()](PushSetAttrs::push_obj_type)\n\n"]
    pub fn op_newset_do(self, header: &Nfgenmsg) -> OpNewsetDo<'buf> {
        let mut res = OpNewsetDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-newset-do", OpNewsetDo::lookup);
        res
    }
    #[doc = "Get / dump sets.\n\nRequest attributes:\n- [.push_table()](PushSetAttrs::push_table)\n\nReply attributes:\n- [.get_table()](IterableSetAttrs::get_table)\n- [.get_name()](IterableSetAttrs::get_name)\n- [.get_flags()](IterableSetAttrs::get_flags)\n- [.get_key_type()](IterableSetAttrs::get_key_type)\n- [.get_key_len()](IterableSetAttrs::get_key_len)\n- [.get_data_type()](IterableSetAttrs::get_data_type)\n- [.get_data_len()](IterableSetAttrs::get_data_len)\n- [.get_policy()](IterableSetAttrs::get_policy)\n- [.get_desc()](IterableSetAttrs::get_desc)\n- [.get_gc_interval()](IterableSetAttrs::get_gc_interval)\n- [.get_userdata()](IterableSetAttrs::get_userdata)\n- [.get_obj_type()](IterableSetAttrs::get_obj_type)\n- [.get_handle()](IterableSetAttrs::get_handle)\n- [.get_expr()](IterableSetAttrs::get_expr)\n- [.get_expressions()](IterableSetAttrs::get_expressions)\n\n"]
    pub fn op_getset_dump(self, header: &Nfgenmsg) -> OpGetsetDump<'buf> {
        let mut res = OpGetsetDump::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getset-dump", OpGetsetDump::lookup);
        res
    }
    #[doc = "Get / dump sets.\n\nRequest attributes:\n- [.push_table()](PushSetAttrs::push_table)\n- [.push_name()](PushSetAttrs::push_name)\n\nReply attributes:\n- [.get_table()](IterableSetAttrs::get_table)\n- [.get_name()](IterableSetAttrs::get_name)\n- [.get_flags()](IterableSetAttrs::get_flags)\n- [.get_key_type()](IterableSetAttrs::get_key_type)\n- [.get_key_len()](IterableSetAttrs::get_key_len)\n- [.get_data_type()](IterableSetAttrs::get_data_type)\n- [.get_data_len()](IterableSetAttrs::get_data_len)\n- [.get_policy()](IterableSetAttrs::get_policy)\n- [.get_desc()](IterableSetAttrs::get_desc)\n- [.get_gc_interval()](IterableSetAttrs::get_gc_interval)\n- [.get_userdata()](IterableSetAttrs::get_userdata)\n- [.get_obj_type()](IterableSetAttrs::get_obj_type)\n- [.get_handle()](IterableSetAttrs::get_handle)\n- [.get_expr()](IterableSetAttrs::get_expr)\n- [.get_expressions()](IterableSetAttrs::get_expressions)\n\n"]
    pub fn op_getset_do(self, header: &Nfgenmsg) -> OpGetsetDo<'buf> {
        let mut res = OpGetsetDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getset-do", OpGetsetDo::lookup);
        res
    }
    #[doc = "Delete an existing set.\n\nRequest attributes:\n- [.push_table()](PushSetAttrs::push_table)\n- [.push_name()](PushSetAttrs::push_name)\n- [.push_handle()](PushSetAttrs::push_handle)\n\n"]
    pub fn op_delset_do(self, header: &Nfgenmsg) -> OpDelsetDo<'buf> {
        let mut res = OpDelsetDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-delset-do", OpDelsetDo::lookup);
        res
    }
    #[doc = "Delete an existing set with destroy semantics (ignoring ENOENT errors).\n\nRequest attributes:\n- [.push_table()](PushSetAttrs::push_table)\n- [.push_name()](PushSetAttrs::push_name)\n- [.push_handle()](PushSetAttrs::push_handle)\n\n"]
    pub fn op_destroyset_do(self, header: &Nfgenmsg) -> OpDestroysetDo<'buf> {
        let mut res = OpDestroysetDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-destroyset-do", OpDestroysetDo::lookup);
        res
    }
    #[doc = "Create a new set element.\n\nRequest attributes:\n- [.push_table()](PushSetelemListAttrs::push_table)\n- [.push_set()](PushSetelemListAttrs::push_set)\n- [.nested_elements()](PushSetelemListAttrs::nested_elements)\n- [.push_set_id()](PushSetelemListAttrs::push_set_id)\n\n"]
    pub fn op_newsetelem_do(self, header: &Nfgenmsg) -> OpNewsetelemDo<'buf> {
        let mut res = OpNewsetelemDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-newsetelem-do", OpNewsetelemDo::lookup);
        res
    }
    #[doc = "Get / dump set elements.\n\nRequest attributes:\n- [.push_table()](PushSetelemListAttrs::push_table)\n- [.push_set()](PushSetelemListAttrs::push_set)\n\nReply attributes:\n- [.get_table()](IterableSetelemListAttrs::get_table)\n- [.get_set()](IterableSetelemListAttrs::get_set)\n- [.get_elements()](IterableSetelemListAttrs::get_elements)\n\n"]
    pub fn op_getsetelem_dump(self, header: &Nfgenmsg) -> OpGetsetelemDump<'buf> {
        let mut res = OpGetsetelemDump::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-getsetelem-dump",
            OpGetsetelemDump::lookup,
        );
        res
    }
    #[doc = "Get / dump set elements.\n\nRequest attributes:\n- [.push_table()](PushSetelemListAttrs::push_table)\n- [.push_set()](PushSetelemListAttrs::push_set)\n- [.nested_elements()](PushSetelemListAttrs::nested_elements)\n\nReply attributes:\n- [.get_elements()](IterableSetelemListAttrs::get_elements)\n\n"]
    pub fn op_getsetelem_do(self, header: &Nfgenmsg) -> OpGetsetelemDo<'buf> {
        let mut res = OpGetsetelemDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getsetelem-do", OpGetsetelemDo::lookup);
        res
    }
    #[doc = "Get / dump set elements and reset stateful expressions.\n\nRequest attributes:\n- [.push_table()](PushSetelemListAttrs::push_table)\n- [.push_set()](PushSetelemListAttrs::push_set)\n\nReply attributes:\n- [.get_table()](IterableSetelemListAttrs::get_table)\n- [.get_set()](IterableSetelemListAttrs::get_set)\n- [.get_elements()](IterableSetelemListAttrs::get_elements)\n\n"]
    pub fn op_getsetelem_reset_dump(self, header: &Nfgenmsg) -> OpGetsetelemResetDump<'buf> {
        let mut res = OpGetsetelemResetDump::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-getsetelem-reset-dump",
            OpGetsetelemResetDump::lookup,
        );
        res
    }
    #[doc = "Get / dump set elements and reset stateful expressions.\n\nRequest attributes:\n- [.nested_elements()](PushSetelemListAttrs::nested_elements)\n\nReply attributes:\n- [.get_table()](IterableSetelemListAttrs::get_table)\n- [.get_set()](IterableSetelemListAttrs::get_set)\n- [.get_elements()](IterableSetelemListAttrs::get_elements)\n\n"]
    pub fn op_getsetelem_reset_do(self, header: &Nfgenmsg) -> OpGetsetelemResetDo<'buf> {
        let mut res = OpGetsetelemResetDo::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-getsetelem-reset-do",
            OpGetsetelemResetDo::lookup,
        );
        res
    }
    #[doc = "Delete an existing set element.\n\nRequest attributes:\n- [.push_table()](PushSetelemListAttrs::push_table)\n- [.push_set()](PushSetelemListAttrs::push_set)\n- [.nested_elements()](PushSetelemListAttrs::nested_elements)\n\n"]
    pub fn op_delsetelem_do(self, header: &Nfgenmsg) -> OpDelsetelemDo<'buf> {
        let mut res = OpDelsetelemDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-delsetelem-do", OpDelsetelemDo::lookup);
        res
    }
    #[doc = "Delete an existing set element with destroy semantics.\n\nRequest attributes:\n- [.push_table()](PushSetelemListAttrs::push_table)\n- [.push_set()](PushSetelemListAttrs::push_set)\n- [.nested_elements()](PushSetelemListAttrs::nested_elements)\n\n"]
    pub fn op_destroysetelem_do(self, header: &Nfgenmsg) -> OpDestroysetelemDo<'buf> {
        let mut res = OpDestroysetelemDo::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-destroysetelem-do",
            OpDestroysetelemDo::lookup,
        );
        res
    }
    #[doc = "Get / dump rule-set generation.\n\nReply attributes:\n- [.get_id()](IterableGenAttrs::get_id)\n- [.get_proc_pid()](IterableGenAttrs::get_proc_pid)\n- [.get_proc_name()](IterableGenAttrs::get_proc_name)\n\n"]
    pub fn op_getgen_dump(self, header: &Nfgenmsg) -> OpGetgenDump<'buf> {
        let mut res = OpGetgenDump::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getgen-dump", OpGetgenDump::lookup);
        res
    }
    #[doc = "Get / dump rule-set generation.\n\nReply attributes:\n- [.get_id()](IterableGenAttrs::get_id)\n- [.get_proc_pid()](IterableGenAttrs::get_proc_pid)\n- [.get_proc_name()](IterableGenAttrs::get_proc_name)\n\n"]
    pub fn op_getgen_do(self, header: &Nfgenmsg) -> OpGetgenDo<'buf> {
        let mut res = OpGetgenDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getgen-do", OpGetgenDo::lookup);
        res
    }
    #[doc = "Create a new stateful object.\n\nRequest attributes:\n- [.push_table()](PushObjAttrs::push_table)\n- [.push_name()](PushObjAttrs::push_name)\n- [.push_type()](PushObjAttrs::push_type)\n- [.nested_data_counter()](PushObjAttrs::nested_data_counter)\n- [.nested_data_quota()](PushObjAttrs::nested_data_quota)\n- [.push_userdata()](PushObjAttrs::push_userdata)\n\n"]
    pub fn op_newobj_do(self, header: &Nfgenmsg) -> OpNewobjDo<'buf> {
        let mut res = OpNewobjDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-newobj-do", OpNewobjDo::lookup);
        res
    }
    #[doc = "Get / dump stateful objects.\n\nRequest attributes:\n- [.push_table()](PushObjAttrs::push_table)\n- [.push_type()](PushObjAttrs::push_type)\n\nReply attributes:\n- [.get_table()](IterableObjAttrs::get_table)\n- [.get_name()](IterableObjAttrs::get_name)\n- [.get_type()](IterableObjAttrs::get_type)\n- [.get_data()](IterableObjAttrs::get_data)\n- [.get_use()](IterableObjAttrs::get_use)\n- [.get_handle()](IterableObjAttrs::get_handle)\n- [.get_userdata()](IterableObjAttrs::get_userdata)\n\n"]
    pub fn op_getobj_dump(self, header: &Nfgenmsg) -> OpGetobjDump<'buf> {
        let mut res = OpGetobjDump::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getobj-dump", OpGetobjDump::lookup);
        res
    }
    #[doc = "Get / dump stateful objects.\n\nRequest attributes:\n- [.push_table()](PushObjAttrs::push_table)\n- [.push_name()](PushObjAttrs::push_name)\n- [.push_type()](PushObjAttrs::push_type)\n\nReply attributes:\n- [.get_table()](IterableObjAttrs::get_table)\n- [.get_name()](IterableObjAttrs::get_name)\n- [.get_type()](IterableObjAttrs::get_type)\n- [.get_data()](IterableObjAttrs::get_data)\n- [.get_use()](IterableObjAttrs::get_use)\n- [.get_handle()](IterableObjAttrs::get_handle)\n- [.get_userdata()](IterableObjAttrs::get_userdata)\n\n"]
    pub fn op_getobj_do(self, header: &Nfgenmsg) -> OpGetobjDo<'buf> {
        let mut res = OpGetobjDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getobj-do", OpGetobjDo::lookup);
        res
    }
    #[doc = "Delete an existing stateful object.\n\nRequest attributes:\n- [.push_table()](PushObjAttrs::push_table)\n- [.push_name()](PushObjAttrs::push_name)\n- [.push_type()](PushObjAttrs::push_type)\n- [.push_handle()](PushObjAttrs::push_handle)\n\n"]
    pub fn op_delobj_do(self, header: &Nfgenmsg) -> OpDelobjDo<'buf> {
        let mut res = OpDelobjDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-delobj-do", OpDelobjDo::lookup);
        res
    }
    #[doc = "Delete an existing stateful object with destroy semantics.\n\nRequest attributes:\n- [.push_table()](PushObjAttrs::push_table)\n- [.push_name()](PushObjAttrs::push_name)\n- [.push_type()](PushObjAttrs::push_type)\n- [.push_handle()](PushObjAttrs::push_handle)\n\n"]
    pub fn op_destroyobj_do(self, header: &Nfgenmsg) -> OpDestroyobjDo<'buf> {
        let mut res = OpDestroyobjDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-destroyobj-do", OpDestroyobjDo::lookup);
        res
    }
    #[doc = "Create a new flow table.\n\nRequest attributes:\n- [.push_table()](PushFlowtableAttrs::push_table)\n- [.push_name()](PushFlowtableAttrs::push_name)\n- [.nested_hook()](PushFlowtableAttrs::nested_hook)\n- [.push_flags()](PushFlowtableAttrs::push_flags)\n\n"]
    pub fn op_newflowtable_do(self, header: &Nfgenmsg) -> OpNewflowtableDo<'buf> {
        let mut res = OpNewflowtableDo::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-newflowtable-do",
            OpNewflowtableDo::lookup,
        );
        res
    }
    #[doc = "Get / dump flow tables.\n\nReply attributes:\n- [.get_table()](IterableFlowtableAttrs::get_table)\n- [.get_name()](IterableFlowtableAttrs::get_name)\n- [.get_hook()](IterableFlowtableAttrs::get_hook)\n- [.get_use()](IterableFlowtableAttrs::get_use)\n- [.get_handle()](IterableFlowtableAttrs::get_handle)\n- [.get_flags()](IterableFlowtableAttrs::get_flags)\n\n"]
    pub fn op_getflowtable_dump(self, header: &Nfgenmsg) -> OpGetflowtableDump<'buf> {
        let mut res = OpGetflowtableDump::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-getflowtable-dump",
            OpGetflowtableDump::lookup,
        );
        res
    }
    #[doc = "Get / dump flow tables.\n\nRequest attributes:\n- [.push_table()](PushFlowtableAttrs::push_table)\n- [.push_name()](PushFlowtableAttrs::push_name)\n\nReply attributes:\n- [.get_table()](IterableFlowtableAttrs::get_table)\n- [.get_name()](IterableFlowtableAttrs::get_name)\n- [.get_hook()](IterableFlowtableAttrs::get_hook)\n- [.get_use()](IterableFlowtableAttrs::get_use)\n- [.get_handle()](IterableFlowtableAttrs::get_handle)\n- [.get_flags()](IterableFlowtableAttrs::get_flags)\n\n"]
    pub fn op_getflowtable_do(self, header: &Nfgenmsg) -> OpGetflowtableDo<'buf> {
        let mut res = OpGetflowtableDo::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-getflowtable-do",
            OpGetflowtableDo::lookup,
        );
        res
    }
    #[doc = "Delete an existing flow table.\n\nRequest attributes:\n- [.push_table()](PushFlowtableAttrs::push_table)\n- [.push_name()](PushFlowtableAttrs::push_name)\n- [.nested_hook()](PushFlowtableAttrs::nested_hook)\n- [.push_handle()](PushFlowtableAttrs::push_handle)\n\n"]
    pub fn op_delflowtable_do(self, header: &Nfgenmsg) -> OpDelflowtableDo<'buf> {
        let mut res = OpDelflowtableDo::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-delflowtable-do",
            OpDelflowtableDo::lookup,
        );
        res
    }
    #[doc = "Delete an existing flow table with destroy semantics.\n\nRequest attributes:\n- [.push_table()](PushFlowtableAttrs::push_table)\n- [.push_name()](PushFlowtableAttrs::push_name)\n- [.nested_hook()](PushFlowtableAttrs::nested_hook)\n- [.push_handle()](PushFlowtableAttrs::push_handle)\n\n"]
    pub fn op_destroyflowtable_do(self, header: &Nfgenmsg) -> OpDestroyflowtableDo<'buf> {
        let mut res = OpDestroyflowtableDo::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-destroyflowtable-do",
            OpDestroyflowtableDo::lookup,
        );
        res
    }
    #[doc = "Get / dump nft_compat info\n\nReply attributes:\n- [.get_name()](IterableCompatAttrs::get_name)\n- [.get_rev()](IterableCompatAttrs::get_rev)\n- [.get_type()](IterableCompatAttrs::get_type)\n\n"]
    pub fn op_getcompat_dump(self, header: &Nfgenmsg) -> OpGetcompatDump<'buf> {
        let mut res = OpGetcompatDump::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getcompat-dump", OpGetcompatDump::lookup);
        res
    }
    #[doc = "Get / dump nft_compat info\n\nRequest attributes:\n- [.push_name()](PushCompatAttrs::push_name)\n- [.push_rev()](PushCompatAttrs::push_rev)\n- [.push_type()](PushCompatAttrs::push_type)\n\nReply attributes:\n- [.get_name()](IterableCompatAttrs::get_name)\n- [.get_rev()](IterableCompatAttrs::get_rev)\n- [.get_type()](IterableCompatAttrs::get_type)\n\n"]
    pub fn op_getcompat_do(self, header: &Nfgenmsg) -> OpGetcompatDo<'buf> {
        let mut res = OpGetcompatDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getcompat-do", OpGetcompatDo::lookup);
        res
    }
    #[doc = ""]
    pub fn op_dump(self, request_type: u16, header: &Nfgenmsg) -> OpDump<'buf> {
        let mut res = OpDump::new(self, request_type, header);
        res.request
            .do_writeback(res.protocol(), "op-dump", OpDump::lookup);
        res
    }
    #[doc = ""]
    pub fn op_do(self, request_type: u16, header: &Nfgenmsg) -> OpDo<'buf> {
        let mut res = OpDo::new(self, request_type, header);
        res.request
            .do_writeback(res.protocol(), "op-do", OpDo::lookup);
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableBatchAttrs::get_genid;
        let _ = IterableChainAttrs::get_counters;
        let _ = IterableChainAttrs::get_flags;
        let _ = IterableChainAttrs::get_handle;
        let _ = IterableChainAttrs::get_hook;
        let _ = IterableChainAttrs::get_id;
        let _ = IterableChainAttrs::get_name;
        let _ = IterableChainAttrs::get_policy;
        let _ = IterableChainAttrs::get_table;
        let _ = IterableChainAttrs::get_type;
        let _ = IterableChainAttrs::get_use;
        let _ = IterableChainAttrs::get_userdata;
        let _ = IterableCompatAttrs::get_name;
        let _ = IterableCompatAttrs::get_rev;
        let _ = IterableCompatAttrs::get_type;
        let _ = IterableFlowtableAttrs::get_flags;
        let _ = IterableFlowtableAttrs::get_handle;
        let _ = IterableFlowtableAttrs::get_hook;
        let _ = IterableFlowtableAttrs::get_name;
        let _ = IterableFlowtableAttrs::get_table;
        let _ = IterableFlowtableAttrs::get_use;
        let _ = IterableGenAttrs::get_id;
        let _ = IterableGenAttrs::get_proc_name;
        let _ = IterableGenAttrs::get_proc_pid;
        let _ = IterableObjAttrs::get_data;
        let _ = IterableObjAttrs::get_handle;
        let _ = IterableObjAttrs::get_name;
        let _ = IterableObjAttrs::get_table;
        let _ = IterableObjAttrs::get_type;
        let _ = IterableObjAttrs::get_use;
        let _ = IterableObjAttrs::get_userdata;
        let _ = IterableRuleAttrs::get_chain;
        let _ = IterableRuleAttrs::get_expressions;
        let _ = IterableRuleAttrs::get_handle;
        let _ = IterableRuleAttrs::get_position;
        let _ = IterableRuleAttrs::get_table;
        let _ = IterableRuleAttrs::get_userdata;
        let _ = IterableSetAttrs::get_data_len;
        let _ = IterableSetAttrs::get_data_type;
        let _ = IterableSetAttrs::get_desc;
        let _ = IterableSetAttrs::get_expr;
        let _ = IterableSetAttrs::get_expressions;
        let _ = IterableSetAttrs::get_flags;
        let _ = IterableSetAttrs::get_gc_interval;
        let _ = IterableSetAttrs::get_handle;
        let _ = IterableSetAttrs::get_key_len;
        let _ = IterableSetAttrs::get_key_type;
        let _ = IterableSetAttrs::get_name;
        let _ = IterableSetAttrs::get_obj_type;
        let _ = IterableSetAttrs::get_policy;
        let _ = IterableSetAttrs::get_table;
        let _ = IterableSetAttrs::get_userdata;
        let _ = IterableSetelemListAttrs::get_elements;
        let _ = IterableSetelemListAttrs::get_set;
        let _ = IterableSetelemListAttrs::get_table;
        let _ = IterableTableAttrs::get_flags;
        let _ = IterableTableAttrs::get_handle;
        let _ = IterableTableAttrs::get_name;
        let _ = IterableTableAttrs::get_owner;
        let _ = IterableTableAttrs::get_use;
        let _ = IterableTableAttrs::get_userdata;
        let _ = PushBatchAttrs::<&mut Vec<u8>>::push_genid;
        let _ = PushChainAttrs::<&mut Vec<u8>>::nested_counters;
        let _ = PushChainAttrs::<&mut Vec<u8>>::nested_hook;
        let _ = PushChainAttrs::<&mut Vec<u8>>::push_flags;
        let _ = PushChainAttrs::<&mut Vec<u8>>::push_handle;
        let _ = PushChainAttrs::<&mut Vec<u8>>::push_name;
        let _ = PushChainAttrs::<&mut Vec<u8>>::push_policy;
        let _ = PushChainAttrs::<&mut Vec<u8>>::push_table;
        let _ = PushChainAttrs::<&mut Vec<u8>>::push_type;
        let _ = PushChainAttrs::<&mut Vec<u8>>::push_userdata;
        let _ = PushCompatAttrs::<&mut Vec<u8>>::push_name;
        let _ = PushCompatAttrs::<&mut Vec<u8>>::push_rev;
        let _ = PushCompatAttrs::<&mut Vec<u8>>::push_type;
        let _ = PushFlowtableAttrs::<&mut Vec<u8>>::nested_hook;
        let _ = PushFlowtableAttrs::<&mut Vec<u8>>::push_flags;
        let _ = PushFlowtableAttrs::<&mut Vec<u8>>::push_handle;
        let _ = PushFlowtableAttrs::<&mut Vec<u8>>::push_name;
        let _ = PushFlowtableAttrs::<&mut Vec<u8>>::push_table;
        let _ = PushObjAttrs::<&mut Vec<u8>>::nested_data_counter;
        let _ = PushObjAttrs::<&mut Vec<u8>>::nested_data_quota;
        let _ = PushObjAttrs::<&mut Vec<u8>>::push_handle;
        let _ = PushObjAttrs::<&mut Vec<u8>>::push_name;
        let _ = PushObjAttrs::<&mut Vec<u8>>::push_table;
        let _ = PushObjAttrs::<&mut Vec<u8>>::push_type;
        let _ = PushObjAttrs::<&mut Vec<u8>>::push_userdata;
        let _ = PushRuleAttrs::<&mut Vec<u8>>::nested_compat;
        let _ = PushRuleAttrs::<&mut Vec<u8>>::nested_expressions;
        let _ = PushRuleAttrs::<&mut Vec<u8>>::push_chain;
        let _ = PushRuleAttrs::<&mut Vec<u8>>::push_chain_id;
        let _ = PushRuleAttrs::<&mut Vec<u8>>::push_handle;
        let _ = PushRuleAttrs::<&mut Vec<u8>>::push_id;
        let _ = PushRuleAttrs::<&mut Vec<u8>>::push_position;
        let _ = PushRuleAttrs::<&mut Vec<u8>>::push_position_id;
        let _ = PushRuleAttrs::<&mut Vec<u8>>::push_table;
        let _ = PushRuleAttrs::<&mut Vec<u8>>::push_userdata;
        let _ = PushSetAttrs::<&mut Vec<u8>>::nested_desc;
        let _ = PushSetAttrs::<&mut Vec<u8>>::push_data_len;
        let _ = PushSetAttrs::<&mut Vec<u8>>::push_data_type;
        let _ = PushSetAttrs::<&mut Vec<u8>>::push_flags;
        let _ = PushSetAttrs::<&mut Vec<u8>>::push_gc_interval;
        let _ = PushSetAttrs::<&mut Vec<u8>>::push_handle;
        let _ = PushSetAttrs::<&mut Vec<u8>>::push_id;
        let _ = PushSetAttrs::<&mut Vec<u8>>::push_key_len;
        let _ = PushSetAttrs::<&mut Vec<u8>>::push_key_type;
        let _ = PushSetAttrs::<&mut Vec<u8>>::push_name;
        let _ = PushSetAttrs::<&mut Vec<u8>>::push_obj_type;
        let _ = PushSetAttrs::<&mut Vec<u8>>::push_policy;
        let _ = PushSetAttrs::<&mut Vec<u8>>::push_table;
        let _ = PushSetAttrs::<&mut Vec<u8>>::push_timeout;
        let _ = PushSetAttrs::<&mut Vec<u8>>::push_userdata;
        let _ = PushSetelemListAttrs::<&mut Vec<u8>>::nested_elements;
        let _ = PushSetelemListAttrs::<&mut Vec<u8>>::push_set;
        let _ = PushSetelemListAttrs::<&mut Vec<u8>>::push_set_id;
        let _ = PushSetelemListAttrs::<&mut Vec<u8>>::push_table;
        let _ = PushTableAttrs::<&mut Vec<u8>>::push_flags;
        let _ = PushTableAttrs::<&mut Vec<u8>>::push_handle;
        let _ = PushTableAttrs::<&mut Vec<u8>>::push_name;
        let _ = PushTableAttrs::<&mut Vec<u8>>::push_userdata;
    }
}
