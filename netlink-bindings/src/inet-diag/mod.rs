#![doc = "Internet socket diagnostics\n"]
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
pub const PROTONAME: &str = "inet-diag";
pub const PROTONAME_CSTR: &CStr = c"inet-diag";
pub const PROTONUM: u16 = 4u16;
pub const TCPDIAG_GETSOCK_CONST: u64 = 18u64;
pub const DCCPDIAG_GETSOCK_CONST: u64 = 19u64;
pub const GETSOCK_MAX_CONST: u64 = 24u64;
pub const NOCOOKIE_CONST: u64 = 4294967295u64;
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum BytecodeOpCode {
    Nop = 0,
    #[doc = "unconditional jump. \\\"no\\\" value is ignored.\n"]
    Jmp = 1,
    #[doc = "sock.sport \\>= next_instruction.no (big endian)\n"]
    SportGe = 2,
    #[doc = "sock.sport \\<= next_instruction.no (big endian)\n"]
    SportLe = 3,
    #[doc = "sock.dport \\>= next_instruction.no (big endian)\n"]
    DportGe = 4,
    #[doc = "sock.dport \\<= next_instruction.no (big endian)\n"]
    DportLe = 5,
    #[doc = "check if sock is NOT bound to a port by user, i.e.\n`!(sk->userlocks & SOCK_BINDPORT_LOCK)`\n"]
    PortAuto = 6,
    #[doc = "Check aginst source socket addr packed as hostcond struct (hc), followed\nby big-endian ipv4 or ipv6 address (yes, it\\'s that cursed).\n\nThe check equivalent to the following (in order):\n\n``` raw\nno if hc.port != -1 && hc.port != sock.sport\nyes if hc.family == AF_INET && sock.family == AF_INET6\n    && &sock.saddr_u32[0..3] == &[0, 0, 0xffff.to_be()]\n    && bits_eq(&sock.saddr_u8[12..], &hc.addr[..], hc.prefix_len)\nno if hc.family != AF_UNSPEC && hc.family != family\nyes if hc.prefix_len == 0\nyes if bits_eq(&sock.addr[..], &hc.addr[..], hc.prefix_len)\nno\n```\n\nSee `inet_diag_bc_run()` in net/ipv4/inet_diag.c\n"]
    SaddrCond = 7,
    #[doc = "Check aginst source socket addr using hostcond struct. Same as\n[saddr-cond]{.title-ref}, see its description.\n"]
    DaddrCond = 8,
    #[doc = "socket ifindex == next_instruction (native endian u32)\n"]
    DevCond = 9,
    #[doc = "Check check socket mark bits against markcond struct (mc). The check is\nequivalent to: sock.mark & mc.mask == mc.mark\n"]
    MarkCond = 10,
    #[doc = "sock.sport == next_instruction.no (big endian)\n"]
    SportEq = 11,
    #[doc = "sock.dport == next_instruction.no (big endian)\n"]
    DportEq = 12,
    #[doc = "sock.cgroup_id == next_2_instructions (native endian u64)\n"]
    CgroupCond = 13,
}
impl BytecodeOpCode {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Nop,
            1 => Self::Jmp,
            2 => Self::SportGe,
            3 => Self::SportLe,
            4 => Self::DportGe,
            5 => Self::DportLe,
            6 => Self::PortAuto,
            7 => Self::SaddrCond,
            8 => Self::DaddrCond,
            9 => Self::DevCond,
            10 => Self::MarkCond,
            11 => Self::SportEq,
            12 => Self::DportEq,
            13 => Self::CgroupCond,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum SockoptFlag {
    Recverr = 1 << 0,
    IsIcsk = 1 << 1,
    Freebind = 1 << 2,
    Hdrincl = 1 << 3,
    McLoop = 1 << 4,
    Transparent = 1 << 5,
    McAll = 1 << 6,
    Nodefrag = 1 << 7,
    BindAddressNoPort = 1 << 8,
    RecverrRfc4884 = 1 << 9,
    DeferConnect = 1 << 10,
}
impl SockoptFlag {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::Recverr,
            n if n == 1 << 1 => Self::IsIcsk,
            n if n == 1 << 2 => Self::Freebind,
            n if n == 1 << 3 => Self::Hdrincl,
            n if n == 1 << 4 => Self::McLoop,
            n if n == 1 << 5 => Self::Transparent,
            n if n == 1 << 6 => Self::McAll,
            n if n == 1 << 7 => Self::Nodefrag,
            n if n == 1 << 8 => Self::BindAddressNoPort,
            n if n == 1 << 9 => Self::RecverrRfc4884,
            n if n == 1 << 10 => Self::DeferConnect,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum TcpState {
    Established = 1,
    SynSent = 2,
    SynRecv = 3,
    FinWait1 = 4,
    FinWait2 = 5,
    TimeWait = 6,
    Close = 7,
    CloseWait = 8,
    LastAck = 9,
    Listen = 10,
    #[doc = "Now a valid state\n"]
    Closing = 11,
    NewSynRecv = 12,
    #[doc = "Pseudo-state for inet_diag\n"]
    BoundInactive = 13,
}
impl TcpState {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            1 => Self::Established,
            2 => Self::SynSent,
            3 => Self::SynRecv,
            4 => Self::FinWait1,
            5 => Self::FinWait2,
            6 => Self::TimeWait,
            7 => Self::Close,
            8 => Self::CloseWait,
            9 => Self::LastAck,
            10 => Self::Listen,
            11 => Self::Closing,
            12 => Self::NewSynRecv,
            13 => Self::BoundInactive,
            _ => return None,
        })
    }
}
#[doc = "Socket identity\n"]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Sockid {
    pub _sport_be: u16,
    pub _dport_be: u16,
    pub src: [u8; 16usize],
    pub dst: [u8; 16usize],
    pub r#if: u32,
    pub cookie: [u8; 8usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for Sockid {
    fn default() -> Self {
        Self::new()
    }
}
impl Sockid {
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
    pub fn new_from_array(buf: [u8; 48usize]) -> Self {
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
    pub fn as_array(&self) -> &[u8; 48usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 48usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 48usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<Sockid>() == 48usize);
        const _: () = assert!(std::mem::align_of::<Sockid>() == 4usize);
        48usize
    }
    pub fn sport(&self) -> u16 {
        u16::from_be(self._sport_be)
    }
    pub fn set_sport(&mut self, value: u16) {
        self._sport_be = value.to_be();
    }
    pub fn dport(&self) -> u16 {
        u16::from_be(self._dport_be)
    }
    pub fn set_dport(&mut self, value: u16) {
        self._dport_be = value.to_be();
    }
}
impl std::fmt::Debug for Sockid {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Sockid")
            .field("sport", &self.sport())
            .field("dport", &self.dport())
            .field("src", &FormatHex(self.src))
            .field("dst", &FormatHex(self.dst))
            .field("if", &self.r#if)
            .field("cookie", &FormatHex(self.cookie))
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Req {
    #[doc = "Family of addresses\n"]
    pub family: u8,
    pub src_len: u8,
    pub dst_len: u8,
    #[doc = "Query extended information\n"]
    pub ext: u8,
    pub sockid: Sockid,
    #[doc = "States to dump\n\nAssociated type: [`TcpState`] (1 bit per enumeration)"]
    pub states: u32,
    #[doc = "Tables to dump (NI)\n"]
    pub dbs: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for Req {
    fn default() -> Self {
        Self::new()
    }
}
impl Req {
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
    pub fn new_from_array(buf: [u8; 60usize]) -> Self {
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
    pub fn as_array(&self) -> &[u8; 60usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 60usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 60usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<Req>() == 60usize);
        const _: () = assert!(std::mem::align_of::<Req>() == 4usize);
        60usize
    }
}
impl std::fmt::Debug for Req {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Req")
            .field("family", &self.family)
            .field("src_len", &self.src_len)
            .field("dst_len", &self.dst_len)
            .field("ext", &self.ext)
            .field("sockid", &self.sockid)
            .field(
                "states",
                &FormatFlags(self.states.into(), |val| {
                    TcpState::from_value(val.trailing_zeros().into())
                }),
            )
            .field("dbs", &self.dbs)
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct ReqV2 {
    pub family: u8,
    pub protocol: u8,
    pub ext: u8,
    pub pad: u8,
    #[doc = "Associated type: [`TcpState`] (1 bit per enumeration)"]
    pub states: u32,
    pub sockid: Sockid,
}
#[doc = "Create zero-initialized struct"]
impl Default for ReqV2 {
    fn default() -> Self {
        Self::new()
    }
}
impl ReqV2 {
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
    pub fn new_from_array(buf: [u8; 56usize]) -> Self {
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
    pub fn as_array(&self) -> &[u8; 56usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 56usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 56usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<ReqV2>() == 56usize);
        const _: () = assert!(std::mem::align_of::<ReqV2>() == 4usize);
        56usize
    }
}
impl std::fmt::Debug for ReqV2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("ReqV2")
            .field("family", &self.family)
            .field("protocol", &self.protocol)
            .field("ext", &self.ext)
            .field("pad", &self.pad)
            .field(
                "states",
                &FormatFlags(self.states.into(), |val| {
                    TcpState::from_value(val.trailing_zeros().into())
                }),
            )
            .field("sockid", &self.sockid)
            .finish()
    }
}
#[doc = "SOCK_RAW sockets require the underlied protocol to be additionally\nspecified so we can use \\@pad member for this, but we can\\'t rename it\nbecause userspace programs still may depend on this name. Instead lets\nuse another structure definition as an alias for struct\n\\@inet_diag_req_v2.\n"]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct ReqRaw {
    pub family: u8,
    pub protocol: u8,
    pub ext: u8,
    pub raw_protocol: u8,
    #[doc = "Associated type: [`TcpState`] (1 bit per enumeration)"]
    pub states: u32,
    pub sockid: Sockid,
}
#[doc = "Create zero-initialized struct"]
impl Default for ReqRaw {
    fn default() -> Self {
        Self::new()
    }
}
impl ReqRaw {
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
    pub fn new_from_array(buf: [u8; 56usize]) -> Self {
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
    pub fn as_array(&self) -> &[u8; 56usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 56usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 56usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<ReqRaw>() == 56usize);
        const _: () = assert!(std::mem::align_of::<ReqRaw>() == 4usize);
        56usize
    }
}
impl std::fmt::Debug for ReqRaw {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("ReqRaw")
            .field("family", &self.family)
            .field("protocol", &self.protocol)
            .field("ext", &self.ext)
            .field("raw_protocol", &self.raw_protocol)
            .field(
                "states",
                &FormatFlags(self.states.into(), |val| {
                    TcpState::from_value(val.trailing_zeros().into())
                }),
            )
            .field("sockid", &self.sockid)
            .finish()
    }
}
#[doc = "Base info structure. It contains socket identity (addrs/ports/cookie)\nand, alas, the information shown by netstat.\n"]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Msg {
    pub family: u8,
    #[doc = "Associated type: [`TcpState`] (enum)"]
    pub state: u8,
    pub timer: u8,
    pub retrans: u8,
    pub sockid: Sockid,
    pub expires: u32,
    pub rqueue: u32,
    pub wqueue: u32,
    pub uid: u32,
    pub inode: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for Msg {
    fn default() -> Self {
        Self::new()
    }
}
impl Msg {
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
    pub fn new_from_array(buf: [u8; 72usize]) -> Self {
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
    pub fn as_array(&self) -> &[u8; 72usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 72usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 72usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<Msg>() == 72usize);
        const _: () = assert!(std::mem::align_of::<Msg>() == 4usize);
        72usize
    }
}
impl std::fmt::Debug for Msg {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Msg")
            .field("family", &self.family)
            .field(
                "state",
                &FormatEnum(self.state.into(), TcpState::from_value),
            )
            .field("timer", &self.timer)
            .field("retrans", &self.retrans)
            .field("sockid", &self.sockid)
            .field("expires", &self.expires)
            .field("rqueue", &self.rqueue)
            .field("wqueue", &self.wqueue)
            .field("uid", &self.uid)
            .field("inode", &self.inode)
            .finish()
    }
}
#[doc = "Bytecode is sequence of 4 byte commands followed by variable arguments.\nAll the commands identified by \\\"code\\\" are conditional jumps forward:\nto offset cc+\\\"yes\\\" (bytes) or to offset cc+\\\"no\\\" (bytes). \\\"yes\\\" is\nsupposed to be length of the command and its arguments (in bytes).\n\nTermination condition is to land excactly on a len\\'th instruction (on\naddress of one after the last one), overshooting means an unsucessfull\ntermination.\n\nIf you reading this, for your own sanity, I advice you to first try\nreverse-lookup on the `ss` command with filters you need, and copy\nbytecode from there.\n"]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct BytecodeOp {
    #[doc = "Associated type: [`BytecodeOpCode`] (enum)"]
    pub code: u8,
    #[doc = "offset to jump on match\n"]
    pub yes: u8,
    #[doc = "offset to jump on non-match\n"]
    pub no: u16,
}
#[doc = "Create zero-initialized struct"]
impl Default for BytecodeOp {
    fn default() -> Self {
        Self::new()
    }
}
impl BytecodeOp {
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
        const _: () = assert!(std::mem::size_of::<BytecodeOp>() == 4usize);
        const _: () = assert!(std::mem::align_of::<BytecodeOp>() == 2usize);
        4usize
    }
}
impl std::fmt::Debug for BytecodeOp {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("BytecodeOp")
            .field(
                "code",
                &FormatEnum(self.code.into(), BytecodeOpCode::from_value),
            )
            .field("yes", &self.yes)
            .field("no", &self.no)
            .finish()
    }
}
#[doc = "Host condition to be placed directly into bytecode. Socket address bytes\nshould be appended right after this struct.\n"]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Hostcond {
    #[doc = "Socket address family\n"]
    pub family: u8,
    #[doc = "Number of bits to compare\n"]
    pub prefix_len: u8,
    pub _pad_2: [u8; 2usize],
    pub port: i32,
}
#[doc = "Create zero-initialized struct"]
impl Default for Hostcond {
    fn default() -> Self {
        Self::new()
    }
}
impl Hostcond {
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
        const _: () = assert!(std::mem::size_of::<Hostcond>() == 8usize);
        const _: () = assert!(std::mem::align_of::<Hostcond>() == 4usize);
        8usize
    }
}
impl std::fmt::Debug for Hostcond {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Hostcond")
            .field("family", &self.family)
            .field("prefix_len", &self.prefix_len)
            .field("port", &self.port)
            .finish()
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Markcond {
    pub mark: u32,
    pub mask: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for Markcond {
    fn default() -> Self {
        Self::new()
    }
}
impl Markcond {
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
        const _: () = assert!(std::mem::size_of::<Markcond>() == 8usize);
        const _: () = assert!(std::mem::align_of::<Markcond>() == 4usize);
        8usize
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Meminfo {
    pub rmem: u32,
    pub wmem: u32,
    pub fmem: u32,
    pub tmem: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for Meminfo {
    fn default() -> Self {
        Self::new()
    }
}
impl Meminfo {
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
        const _: () = assert!(std::mem::size_of::<Meminfo>() == 16usize);
        const _: () = assert!(std::mem::align_of::<Meminfo>() == 4usize);
        16usize
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct TcpvegasInfo {
    pub enabled: u32,
    pub rttcnt: u32,
    pub rtt: u32,
    pub minrtt: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for TcpvegasInfo {
    fn default() -> Self {
        Self::new()
    }
}
impl TcpvegasInfo {
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
        const _: () = assert!(std::mem::size_of::<TcpvegasInfo>() == 16usize);
        const _: () = assert!(std::mem::align_of::<TcpvegasInfo>() == 4usize);
        16usize
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct TcpDctcpInfo {
    pub enabled: u16,
    pub ce_state: u16,
    pub alpha: u32,
    pub ab_ecn: u32,
    pub ab_tot: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for TcpDctcpInfo {
    fn default() -> Self {
        Self::new()
    }
}
impl TcpDctcpInfo {
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
        const _: () = assert!(std::mem::size_of::<TcpDctcpInfo>() == 16usize);
        const _: () = assert!(std::mem::align_of::<TcpDctcpInfo>() == 4usize);
        16usize
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct TcpBbrInfo {
    #[doc = "lower 32 bits of bw\n"]
    pub bw_lo: u32,
    #[doc = "upper 32 bits of bw\n"]
    pub bw_hi: u32,
    #[doc = "min-filtered RTT in uSec\n"]
    pub min_rtt: u32,
    #[doc = "pacing gain shifted left 8 bits\n"]
    pub pacing_gain: u32,
    #[doc = "cwnd gain shifted left 8 bits\n"]
    pub cwnd_gain: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for TcpBbrInfo {
    fn default() -> Self {
        Self::new()
    }
}
impl TcpBbrInfo {
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
        const _: () = assert!(std::mem::size_of::<TcpBbrInfo>() == 20usize);
        const _: () = assert!(std::mem::align_of::<TcpBbrInfo>() == 4usize);
        20usize
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(packed(4))]
#[repr(C)]
pub struct TcpInfo {
    #[doc = "TCP state\n\nAssociated type: [`TcpState`] (enum)"]
    pub state: u8,
    #[doc = "Congestion avoidance state\n"]
    pub ca_state: u8,
    #[doc = "Number of retransmits\n"]
    pub retransmits: u8,
    #[doc = "Number of probes\n"]
    pub probes: u8,
    #[doc = "Backoff count\n"]
    pub backoff: u8,
    #[doc = "TCP options\n"]
    pub options: u8,
    pub _bits_snd_wscale: u8,
    pub _bits_delivery_rate_app_limited: u8,
    #[doc = "Retransmission timeout in microseconds\n"]
    pub rto: u32,
    #[doc = "Delayed ACK timeout in microseconds\n"]
    pub ato: u32,
    #[doc = "Send maximum segment size\n"]
    pub snd_mss: u32,
    #[doc = "Receive maximum segment size\n"]
    pub rcv_mss: u32,
    #[doc = "Number of unacknowledged segments\n"]
    pub unacked: u32,
    #[doc = "Number of SACKed segments\n"]
    pub sacked: u32,
    #[doc = "Number of lost segments\n"]
    pub lost: u32,
    #[doc = "Number of retransmitted segments\n"]
    pub retrans: u32,
    #[doc = "Forward Acknowledgment count\n"]
    pub fackets: u32,
    #[doc = "Time since last data sent (jiffies)\n"]
    pub last_data_sent: u32,
    #[doc = "Time since last ACK sent (jiffies, Not remembered, sorry.)\n"]
    pub last_ack_sent: u32,
    #[doc = "Time since last data received (jiffies)\n"]
    pub last_data_recv: u32,
    #[doc = "Time since last ACK received (jiffies)\n"]
    pub last_ack_recv: u32,
    #[doc = "Path MTU\n"]
    pub pmtu: u32,
    #[doc = "Receive slow start threshold\n"]
    pub rcv_ssthresh: u32,
    #[doc = "Smoothed round trip time in microseconds\n"]
    pub rtt: u32,
    #[doc = "Round trip time variation\n"]
    pub rttvar: u32,
    #[doc = "Send slow start threshold\n"]
    pub snd_ssthresh: u32,
    #[doc = "Send congestion window\n"]
    pub snd_cwnd: u32,
    #[doc = "Advertised MSS\n"]
    pub advmss: u32,
    #[doc = "Reordering threshold\n"]
    pub reordering: u32,
    #[doc = "Receiver side RTT\n"]
    pub rcv_rtt: u32,
    #[doc = "Receiver space\n"]
    pub rcv_space: u32,
    #[doc = "Total number of retransmitted segments\n"]
    pub total_retrans: u32,
    #[doc = "Pacing rate in bytes per second\n"]
    pub pacing_rate: u64,
    #[doc = "Maximum pacing rate in bytes per second\n"]
    pub max_pacing_rate: u64,
    #[doc = "RFC4898 tcpEStatsAppHCThruOctetsAcked\n"]
    pub bytes_acked: u64,
    #[doc = "RFC4898 tcpEStatsAppHCThruOctetsReceived\n"]
    pub bytes_received: u64,
    #[doc = "RFC4898 tcpEStatsPerfSegsOut\n"]
    pub segs_out: u32,
    #[doc = "RFC4898 tcpEStatsPerfSegsIn\n"]
    pub segs_in: u32,
    #[doc = "Bytes in write queue not yet sent\n"]
    pub notsent_bytes: u32,
    #[doc = "Minimum RTT observed in microseconds\n"]
    pub min_rtt: u32,
    #[doc = "RFC4898 tcpEStatsDataSegsIn\n"]
    pub data_segs_in: u32,
    #[doc = "RFC4898 tcpEStatsDataSegsOut\n"]
    pub data_segs_out: u32,
    #[doc = "Delivery rate in bytes per second\n"]
    pub delivery_rate: u64,
    #[doc = "Time (usec) busy sending data\n"]
    pub busy_time: u64,
    #[doc = "Time (usec) limited by receive window\n"]
    pub rwnd_limited: u64,
    #[doc = "Time (usec) limited by send buffer\n"]
    pub sndbuf_limited: u64,
    #[doc = "Packets delivered\n"]
    pub delivered: u32,
    #[doc = "Packets delivered with CE marks\n"]
    pub delivered_ce: u32,
    #[doc = "RFC4898 tcpEStatsPerfHCDataOctetsOut\n"]
    pub bytes_sent: u64,
    #[doc = "RFC4898 tcpEStatsPerfOctetsRetrans\n"]
    pub bytes_retrans: u64,
    #[doc = "RFC4898 tcpEStatsStackDSACKDups\n"]
    pub dsack_dups: u32,
    #[doc = "Reordering events seen\n"]
    pub reord_seen: u32,
    #[doc = "Out-of-order packets received\n"]
    pub rcv_ooopack: u32,
    #[doc = "Peer\\'s advertised receive window after scaling (bytes)\n"]
    pub snd_wnd: u32,
    #[doc = "Local advertised receive window after scaling (bytes)\n"]
    pub rcv_wnd: u32,
    #[doc = "PLB or timeout triggered rehash attempts\n"]
    pub rehash: u32,
    #[doc = "Total number of RTO timeouts, including SYN/SYN-ACK and recurring\ntimeouts\n"]
    pub total_rto: u16,
    #[doc = "Total number of RTO recoveries, including any unfinished recovery\n"]
    pub total_rto_recoveries: u16,
    #[doc = "Total time spent in RTO recoveries in milliseconds, including any\nunfinished recovery\n"]
    pub total_rto_time: u32,
    #[doc = "Number of CE marks received\n"]
    pub received_ce: u32,
    #[doc = "Accurate ECN byte counters for ECT(1)\n"]
    pub delivered_e1_bytes: u32,
    #[doc = "Accurate ECN byte counters for ECT(0)\n"]
    pub delivered_e0_bytes: u32,
    #[doc = "Accurate ECN byte counters for CE\n"]
    pub delivered_ce_bytes: u32,
    #[doc = "Received bytes with ECT(1) marks\n"]
    pub received_e1_bytes: u32,
    #[doc = "Received bytes with ECT(0) marks\n"]
    pub received_e0_bytes: u32,
    #[doc = "Received bytes with CE marks\n"]
    pub received_ce_bytes: u32,
    #[doc = "ACK ECN failure mode\n"]
    pub accecn_fail_mode: u16,
    #[doc = "ACK ECN option seen\n"]
    pub accecn_opt_seen: u16,
}
#[doc = "Create zero-initialized struct"]
impl Default for TcpInfo {
    fn default() -> Self {
        Self::new()
    }
}
impl TcpInfo {
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
    pub fn new_from_array(buf: [u8; 280usize]) -> Self {
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
    pub fn as_array(&self) -> &[u8; 280usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 280usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 280usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<TcpInfo>() == 280usize);
        const _: () = assert!(std::mem::align_of::<TcpInfo>() == 4usize);
        280usize
    }
    #[doc = "Send window scale\n"]
    pub fn snd_wscale(&self) -> u8 {
        (((self._bits_snd_wscale as u32) << 28u32) >> 28u32) as u8
    }
    #[doc = "Send window scale\n"]
    pub fn set_snd_wscale(&mut self, value: u8) {
        let mask = (1 << 4usize) - 1;
        self._bits_snd_wscale =
            (self._bits_snd_wscale & (!(mask << 0usize))) | ((value & mask) << 0usize);
    }
    #[doc = "Receive window scale\n"]
    pub fn rcv_wscale(&self) -> u8 {
        (((self._bits_snd_wscale as u32) << 24u32) >> 28u32) as u8
    }
    #[doc = "Receive window scale\n"]
    pub fn set_rcv_wscale(&mut self, value: u8) {
        let mask = (1 << 4usize) - 1;
        self._bits_snd_wscale =
            (self._bits_snd_wscale & (!(mask << 4usize))) | ((value & mask) << 4usize);
    }
    #[doc = "Delivery rate application limited flag\n"]
    pub fn delivery_rate_app_limited(&self) -> u8 {
        (((self._bits_delivery_rate_app_limited as u32) << 31u32) >> 31u32) as u8
    }
    #[doc = "Delivery rate application limited flag\n"]
    pub fn set_delivery_rate_app_limited(&mut self, value: u8) {
        let mask = (1 << 1usize) - 1;
        self._bits_delivery_rate_app_limited = (self._bits_delivery_rate_app_limited
            & (!(mask << 0usize)))
            | ((value & mask) << 0usize);
    }
    #[doc = "FastOpen client failure code\n"]
    pub fn fastopen_client_fail(&self) -> u8 {
        (((self._bits_delivery_rate_app_limited as u32) << 29u32) >> 30u32) as u8
    }
    #[doc = "FastOpen client failure code\n"]
    pub fn set_fastopen_client_fail(&mut self, value: u8) {
        let mask = (1 << 2usize) - 1;
        self._bits_delivery_rate_app_limited = (self._bits_delivery_rate_app_limited
            & (!(mask << 1usize)))
            | ((value & mask) << 1usize);
    }
}
impl std::fmt::Debug for TcpInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("TcpInfo")
            .field(
                "state",
                &FormatEnum(self.state.into(), TcpState::from_value),
            )
            .field("ca_state", &self.ca_state)
            .field("retransmits", &self.retransmits)
            .field("probes", &self.probes)
            .field("backoff", &self.backoff)
            .field("options", &self.options)
            .field("snd_wscale", &self.snd_wscale())
            .field("rcv_wscale", &self.rcv_wscale())
            .field(
                "delivery_rate_app_limited",
                &self.delivery_rate_app_limited(),
            )
            .field("fastopen_client_fail", &self.fastopen_client_fail())
            .field("rto", &self.rto)
            .field("ato", &self.ato)
            .field("snd_mss", &self.snd_mss)
            .field("rcv_mss", &self.rcv_mss)
            .field("unacked", &self.unacked)
            .field("sacked", &self.sacked)
            .field("lost", &self.lost)
            .field("retrans", &self.retrans)
            .field("fackets", &self.fackets)
            .field("last_data_sent", &self.last_data_sent)
            .field("last_ack_sent", &self.last_ack_sent)
            .field("last_data_recv", &self.last_data_recv)
            .field("last_ack_recv", &self.last_ack_recv)
            .field("pmtu", &self.pmtu)
            .field("rcv_ssthresh", &self.rcv_ssthresh)
            .field("rtt", &self.rtt)
            .field("rttvar", &self.rttvar)
            .field("snd_ssthresh", &self.snd_ssthresh)
            .field("snd_cwnd", &self.snd_cwnd)
            .field("advmss", &self.advmss)
            .field("reordering", &self.reordering)
            .field("rcv_rtt", &self.rcv_rtt)
            .field("rcv_space", &self.rcv_space)
            .field("total_retrans", &self.total_retrans)
            .field("pacing_rate", &{ self.pacing_rate })
            .field("max_pacing_rate", &{ self.max_pacing_rate })
            .field("bytes_acked", &{ self.bytes_acked })
            .field("bytes_received", &{ self.bytes_received })
            .field("segs_out", &self.segs_out)
            .field("segs_in", &self.segs_in)
            .field("notsent_bytes", &self.notsent_bytes)
            .field("min_rtt", &self.min_rtt)
            .field("data_segs_in", &self.data_segs_in)
            .field("data_segs_out", &self.data_segs_out)
            .field("delivery_rate", &{ self.delivery_rate })
            .field("busy_time", &{ self.busy_time })
            .field("rwnd_limited", &{ self.rwnd_limited })
            .field("sndbuf_limited", &{ self.sndbuf_limited })
            .field("delivered", &self.delivered)
            .field("delivered_ce", &self.delivered_ce)
            .field("bytes_sent", &{ self.bytes_sent })
            .field("bytes_retrans", &{ self.bytes_retrans })
            .field("dsack_dups", &self.dsack_dups)
            .field("reord_seen", &self.reord_seen)
            .field("rcv_ooopack", &self.rcv_ooopack)
            .field("snd_wnd", &self.snd_wnd)
            .field("rcv_wnd", &self.rcv_wnd)
            .field("rehash", &self.rehash)
            .field("total_rto", &self.total_rto)
            .field("total_rto_recoveries", &self.total_rto_recoveries)
            .field("total_rto_time", &self.total_rto_time)
            .field("received_ce", &self.received_ce)
            .field("delivered_e1_bytes", &self.delivered_e1_bytes)
            .field("delivered_e0_bytes", &self.delivered_e0_bytes)
            .field("delivered_ce_bytes", &self.delivered_ce_bytes)
            .field("received_e1_bytes", &self.received_e1_bytes)
            .field("received_e0_bytes", &self.received_e0_bytes)
            .field("received_ce_bytes", &self.received_ce_bytes)
            .field("accecn_fail_mode", &self.accecn_fail_mode)
            .field("accecn_opt_seen", &self.accecn_opt_seen)
            .finish()
    }
}
#[derive(Clone)]
pub enum UlpInfoAttrs<'a> {
    #[doc = "ULP name (e.g., \\\"tls\\\", \\\"mptcp\\\")\n"]
    Name(&'a CStr),
    #[doc = "TLS-specific information\n"]
    Tls(&'a [u8]),
    #[doc = "MPTCP-specific information\n"]
    Mptcp(&'a [u8]),
}
impl<'a> IterableUlpInfoAttrs<'a> {
    #[doc = "ULP name (e.g., \\\"tls\\\", \\\"mptcp\\\")\n"]
    pub fn get_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(UlpInfoAttrs::Name(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "UlpInfoAttrs",
            "Name",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "TLS-specific information\n"]
    pub fn get_tls(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(UlpInfoAttrs::Tls(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "UlpInfoAttrs",
            "Tls",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "MPTCP-specific information\n"]
    pub fn get_mptcp(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(UlpInfoAttrs::Mptcp(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "UlpInfoAttrs",
            "Mptcp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl UlpInfoAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableUlpInfoAttrs<'a> {
        IterableUlpInfoAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Name",
            2u16 => "Tls",
            3u16 => "Mptcp",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableUlpInfoAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableUlpInfoAttrs<'a> {
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
impl<'a> Iterator for IterableUlpInfoAttrs<'a> {
    type Item = Result<UlpInfoAttrs<'a>, ErrorContext>;
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
                1u16 => UlpInfoAttrs::Name({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => UlpInfoAttrs::Tls({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => UlpInfoAttrs::Mptcp({
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
            "UlpInfoAttrs",
            r#type.and_then(|t| UlpInfoAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableUlpInfoAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("UlpInfoAttrs");
        let mut iter = IterableUlpInfoAttrs::with_loc(&[], self.orig_loc);
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
                UlpInfoAttrs::Name(val) => fmt.field("Name", &val),
                UlpInfoAttrs::Tls(val) => fmt.field("Tls", &FormatHexdump(val)),
                UlpInfoAttrs::Mptcp(val) => fmt.field("Mptcp", &FormatHexdump(val)),
            };
        }
        fmt.finish()
    }
}
impl IterableUlpInfoAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("UlpInfoAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| UlpInfoAttrs::attr_from_type(t)),
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
                UlpInfoAttrs::Name(val) => {
                    if last_off == offset {
                        stack.push(("Name", last_off));
                        break;
                    }
                }
                UlpInfoAttrs::Tls(val) => {
                    if last_off == offset {
                        stack.push(("Tls", last_off));
                        break;
                    }
                }
                UlpInfoAttrs::Mptcp(val) => {
                    if last_off == offset {
                        stack.push(("Mptcp", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("UlpInfoAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum RequestAttrs<'a> {
    #[doc = "See bytecode-op\n"]
    Bytecode(&'a [u8]),
    BpfStorages(IterableBpfStorageReq<'a>),
    Protocol(u32),
}
impl<'a> IterableRequestAttrs<'a> {
    #[doc = "See bytecode-op\n"]
    pub fn get_bytecode(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RequestAttrs::Bytecode(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RequestAttrs",
            "Bytecode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_bpf_storages(&self) -> Result<IterableBpfStorageReq<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RequestAttrs::BpfStorages(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RequestAttrs",
            "BpfStorages",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_protocol(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RequestAttrs::Protocol(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RequestAttrs",
            "Protocol",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl RequestAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableRequestAttrs<'a> {
        IterableRequestAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Bytecode",
            2u16 => "BpfStorages",
            3u16 => "Protocol",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableRequestAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableRequestAttrs<'a> {
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
impl<'a> Iterator for IterableRequestAttrs<'a> {
    type Item = Result<RequestAttrs<'a>, ErrorContext>;
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
                1u16 => RequestAttrs::Bytecode({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => RequestAttrs::BpfStorages({
                    let res = Some(IterableBpfStorageReq::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => RequestAttrs::Protocol({
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
            "RequestAttrs",
            r#type.and_then(|t| RequestAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableRequestAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("RequestAttrs");
        let mut iter = IterableRequestAttrs::with_loc(&[], self.orig_loc);
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
                RequestAttrs::Bytecode(val) => {
                    let iter = val
                        .chunks(BytecodeOp::len())
                        .map(|b| BytecodeOp::new_from_zeroed(b));
                    fmt.field("Bytecode", &FormatIter(iter))
                }
                RequestAttrs::BpfStorages(val) => fmt.field("BpfStorages", &val),
                RequestAttrs::Protocol(val) => fmt.field("Protocol", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableRequestAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("RequestAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| RequestAttrs::attr_from_type(t)),
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
                RequestAttrs::Bytecode(val) => {
                    if last_off == offset {
                        stack.push(("Bytecode", last_off));
                        break;
                    }
                }
                RequestAttrs::BpfStorages(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                RequestAttrs::Protocol(val) => {
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
            stack.push(("RequestAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum ReplyAttrs<'a> {
    #[doc = "Memory information extension\n"]
    Meminfo(Meminfo),
    TcpInfo(TcpInfo),
    #[doc = "TCP Vegas information\n"]
    Vegasinfo(TcpvegasInfo),
    #[doc = "Congestion control algorithm name\n"]
    Cong(&'a CStr),
    #[doc = "Type of Service\n"]
    Tos(u8),
    #[doc = "Traffic Class\n"]
    Tclass(u8),
    #[doc = "Socket memory information\n"]
    Skmeminfo(&'a [u8]),
    #[doc = "Shutdown state\n"]
    Shutdown(u8),
    #[doc = "TCP DCTCP information (request as INET_DIAG_VEGASINFO)\n"]
    Dctcpinfo(TcpDctcpInfo),
    #[doc = "Raw socket protocol (response attribute only)\n"]
    Protocol(u8),
    #[doc = "IPv6-only socket flag\n"]
    Skv6only(()),
    #[doc = "Local addresses. SCTP thing.\n"]
    Locals(&'a [u8]),
    #[doc = "Peer addresses. SCTP thing.\n"]
    Peers(&'a [u8]),
    Pad(&'a [u8]),
    #[doc = "Socket mark (only with CAP_NET_ADMIN)\n"]
    Mark(u32),
    #[doc = "TCP BBR information (request as INET_DIAG_VEGASINFO)\n"]
    Bbritfo(TcpBbrInfo),
    #[doc = "Class ID (request as INET_DIAG_TCLASS)\n"]
    ClassId(u32),
    #[doc = "MD5 signature information\n"]
    Md5sig(&'a [u8]),
    #[doc = "Upper Layer Protocol information\n"]
    UlpInfo(IterableUlpInfoAttrs<'a>),
    #[doc = "BPF storage information\n\nAttribute may repeat multiple times (treat it as array)"]
    SkBpfStorages(IterableBpfStorageReply<'a>),
    #[doc = "Cgroup ID\n"]
    CgroupId(u64),
    #[doc = "Socket options\n\nAssociated type: [`SockoptFlag`] (enum)"]
    SockoptFlags(u16),
}
impl<'a> IterableReplyAttrs<'a> {
    #[doc = "Memory information extension\n"]
    pub fn get_meminfo(&self) -> Result<Meminfo, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ReplyAttrs::Meminfo(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ReplyAttrs",
            "Meminfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tcp_info(&self) -> Result<TcpInfo, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ReplyAttrs::TcpInfo(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ReplyAttrs",
            "TcpInfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "TCP Vegas information\n"]
    pub fn get_vegasinfo(&self) -> Result<TcpvegasInfo, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ReplyAttrs::Vegasinfo(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ReplyAttrs",
            "Vegasinfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Congestion control algorithm name\n"]
    pub fn get_cong(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ReplyAttrs::Cong(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ReplyAttrs",
            "Cong",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Type of Service\n"]
    pub fn get_tos(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ReplyAttrs::Tos(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ReplyAttrs",
            "Tos",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Traffic Class\n"]
    pub fn get_tclass(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ReplyAttrs::Tclass(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ReplyAttrs",
            "Tclass",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Socket memory information\n"]
    pub fn get_skmeminfo(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ReplyAttrs::Skmeminfo(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ReplyAttrs",
            "Skmeminfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Shutdown state\n"]
    pub fn get_shutdown(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ReplyAttrs::Shutdown(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ReplyAttrs",
            "Shutdown",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "TCP DCTCP information (request as INET_DIAG_VEGASINFO)\n"]
    pub fn get_dctcpinfo(&self) -> Result<TcpDctcpInfo, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ReplyAttrs::Dctcpinfo(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ReplyAttrs",
            "Dctcpinfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Raw socket protocol (response attribute only)\n"]
    pub fn get_protocol(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ReplyAttrs::Protocol(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ReplyAttrs",
            "Protocol",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "IPv6-only socket flag\n"]
    pub fn get_skv6only(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ReplyAttrs::Skv6only(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ReplyAttrs",
            "Skv6only",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Local addresses. SCTP thing.\n"]
    pub fn get_locals(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ReplyAttrs::Locals(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ReplyAttrs",
            "Locals",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Peer addresses. SCTP thing.\n"]
    pub fn get_peers(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ReplyAttrs::Peers(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ReplyAttrs",
            "Peers",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ReplyAttrs::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ReplyAttrs",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Socket mark (only with CAP_NET_ADMIN)\n"]
    pub fn get_mark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ReplyAttrs::Mark(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ReplyAttrs",
            "Mark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "TCP BBR information (request as INET_DIAG_VEGASINFO)\n"]
    pub fn get_bbritfo(&self) -> Result<TcpBbrInfo, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ReplyAttrs::Bbritfo(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ReplyAttrs",
            "Bbritfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Class ID (request as INET_DIAG_TCLASS)\n"]
    pub fn get_class_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ReplyAttrs::ClassId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ReplyAttrs",
            "ClassId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "MD5 signature information\n"]
    pub fn get_md5sig(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ReplyAttrs::Md5sig(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ReplyAttrs",
            "Md5sig",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Upper Layer Protocol information\n"]
    pub fn get_ulp_info(&self) -> Result<IterableUlpInfoAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ReplyAttrs::UlpInfo(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ReplyAttrs",
            "UlpInfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "BPF storage information\n\nAttribute may repeat multiple times (treat it as array)"]
    pub fn get_sk_bpf_storages(
        &self,
    ) -> MultiAttrIterable<Self, ReplyAttrs<'a>, IterableBpfStorageReply<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let ReplyAttrs::SkBpfStorages(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    #[doc = "Cgroup ID\n"]
    pub fn get_cgroup_id(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ReplyAttrs::CgroupId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ReplyAttrs",
            "CgroupId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Socket options\n\nAssociated type: [`SockoptFlag`] (enum)"]
    pub fn get_sockopt_flags(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ReplyAttrs::SockoptFlags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ReplyAttrs",
            "SockoptFlags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ReplyAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableReplyAttrs<'a> {
        IterableReplyAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Meminfo",
            2u16 => "TcpInfo",
            3u16 => "Vegasinfo",
            4u16 => "Cong",
            5u16 => "Tos",
            6u16 => "Tclass",
            7u16 => "Skmeminfo",
            8u16 => "Shutdown",
            9u16 => "Dctcpinfo",
            10u16 => "Protocol",
            11u16 => "Skv6only",
            12u16 => "Locals",
            13u16 => "Peers",
            14u16 => "Pad",
            15u16 => "Mark",
            16u16 => "Bbritfo",
            17u16 => "ClassId",
            18u16 => "Md5sig",
            19u16 => "UlpInfo",
            20u16 => "SkBpfStorages",
            21u16 => "CgroupId",
            22u16 => "SockoptFlags",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableReplyAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableReplyAttrs<'a> {
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
impl<'a> Iterator for IterableReplyAttrs<'a> {
    type Item = Result<ReplyAttrs<'a>, ErrorContext>;
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
                1u16 => ReplyAttrs::Meminfo({
                    let res = Some(Meminfo::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ReplyAttrs::TcpInfo({
                    let res = Some(TcpInfo::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ReplyAttrs::Vegasinfo({
                    let res = Some(TcpvegasInfo::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => ReplyAttrs::Cong({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => ReplyAttrs::Tos({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => ReplyAttrs::Tclass({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => ReplyAttrs::Skmeminfo({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => ReplyAttrs::Shutdown({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => ReplyAttrs::Dctcpinfo({
                    let res = Some(TcpDctcpInfo::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => ReplyAttrs::Protocol({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => ReplyAttrs::Skv6only(()),
                12u16 => ReplyAttrs::Locals({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => ReplyAttrs::Peers({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => ReplyAttrs::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => ReplyAttrs::Mark({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => ReplyAttrs::Bbritfo({
                    let res = Some(TcpBbrInfo::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => ReplyAttrs::ClassId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => ReplyAttrs::Md5sig({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => ReplyAttrs::UlpInfo({
                    let res = Some(IterableUlpInfoAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => ReplyAttrs::SkBpfStorages({
                    let res = Some(IterableBpfStorageReply::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => ReplyAttrs::CgroupId({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => ReplyAttrs::SockoptFlags({
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
            "ReplyAttrs",
            r#type.and_then(|t| ReplyAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableReplyAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ReplyAttrs");
        let mut iter = IterableReplyAttrs::with_loc(&[], self.orig_loc);
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
                ReplyAttrs::Meminfo(val) => fmt.field("Meminfo", &val),
                ReplyAttrs::TcpInfo(val) => fmt.field("TcpInfo", &val),
                ReplyAttrs::Vegasinfo(val) => fmt.field("Vegasinfo", &val),
                ReplyAttrs::Cong(val) => fmt.field("Cong", &val),
                ReplyAttrs::Tos(val) => fmt.field("Tos", &val),
                ReplyAttrs::Tclass(val) => fmt.field("Tclass", &val),
                ReplyAttrs::Skmeminfo(val) => fmt.field("Skmeminfo", &FormatHexdump(val)),
                ReplyAttrs::Shutdown(val) => fmt.field("Shutdown", &val),
                ReplyAttrs::Dctcpinfo(val) => fmt.field("Dctcpinfo", &val),
                ReplyAttrs::Protocol(val) => fmt.field("Protocol", &val),
                ReplyAttrs::Skv6only(val) => fmt.field("Skv6only", &val),
                ReplyAttrs::Locals(val) => fmt.field("Locals", &FormatHexdump(val)),
                ReplyAttrs::Peers(val) => fmt.field("Peers", &FormatHexdump(val)),
                ReplyAttrs::Pad(val) => fmt.field("Pad", &val),
                ReplyAttrs::Mark(val) => fmt.field("Mark", &val),
                ReplyAttrs::Bbritfo(val) => fmt.field("Bbritfo", &val),
                ReplyAttrs::ClassId(val) => fmt.field("ClassId", &val),
                ReplyAttrs::Md5sig(val) => fmt.field("Md5sig", &FormatHexdump(val)),
                ReplyAttrs::UlpInfo(val) => fmt.field("UlpInfo", &val),
                ReplyAttrs::SkBpfStorages(val) => fmt.field("SkBpfStorages", &val),
                ReplyAttrs::CgroupId(val) => fmt.field("CgroupId", &val),
                ReplyAttrs::SockoptFlags(val) => fmt.field(
                    "SockoptFlags",
                    &FormatFlags(val.into(), SockoptFlag::from_value),
                ),
            };
        }
        fmt.finish()
    }
}
impl IterableReplyAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ReplyAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ReplyAttrs::attr_from_type(t)),
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
                ReplyAttrs::Meminfo(val) => {
                    if last_off == offset {
                        stack.push(("Meminfo", last_off));
                        break;
                    }
                }
                ReplyAttrs::TcpInfo(val) => {
                    if last_off == offset {
                        stack.push(("TcpInfo", last_off));
                        break;
                    }
                }
                ReplyAttrs::Vegasinfo(val) => {
                    if last_off == offset {
                        stack.push(("Vegasinfo", last_off));
                        break;
                    }
                }
                ReplyAttrs::Cong(val) => {
                    if last_off == offset {
                        stack.push(("Cong", last_off));
                        break;
                    }
                }
                ReplyAttrs::Tos(val) => {
                    if last_off == offset {
                        stack.push(("Tos", last_off));
                        break;
                    }
                }
                ReplyAttrs::Tclass(val) => {
                    if last_off == offset {
                        stack.push(("Tclass", last_off));
                        break;
                    }
                }
                ReplyAttrs::Skmeminfo(val) => {
                    if last_off == offset {
                        stack.push(("Skmeminfo", last_off));
                        break;
                    }
                }
                ReplyAttrs::Shutdown(val) => {
                    if last_off == offset {
                        stack.push(("Shutdown", last_off));
                        break;
                    }
                }
                ReplyAttrs::Dctcpinfo(val) => {
                    if last_off == offset {
                        stack.push(("Dctcpinfo", last_off));
                        break;
                    }
                }
                ReplyAttrs::Protocol(val) => {
                    if last_off == offset {
                        stack.push(("Protocol", last_off));
                        break;
                    }
                }
                ReplyAttrs::Skv6only(val) => {
                    if last_off == offset {
                        stack.push(("Skv6only", last_off));
                        break;
                    }
                }
                ReplyAttrs::Locals(val) => {
                    if last_off == offset {
                        stack.push(("Locals", last_off));
                        break;
                    }
                }
                ReplyAttrs::Peers(val) => {
                    if last_off == offset {
                        stack.push(("Peers", last_off));
                        break;
                    }
                }
                ReplyAttrs::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                ReplyAttrs::Mark(val) => {
                    if last_off == offset {
                        stack.push(("Mark", last_off));
                        break;
                    }
                }
                ReplyAttrs::Bbritfo(val) => {
                    if last_off == offset {
                        stack.push(("Bbritfo", last_off));
                        break;
                    }
                }
                ReplyAttrs::ClassId(val) => {
                    if last_off == offset {
                        stack.push(("ClassId", last_off));
                        break;
                    }
                }
                ReplyAttrs::Md5sig(val) => {
                    if last_off == offset {
                        stack.push(("Md5sig", last_off));
                        break;
                    }
                }
                ReplyAttrs::UlpInfo(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ReplyAttrs::SkBpfStorages(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ReplyAttrs::CgroupId(val) => {
                    if last_off == offset {
                        stack.push(("CgroupId", last_off));
                        break;
                    }
                }
                ReplyAttrs::SockoptFlags(val) => {
                    if last_off == offset {
                        stack.push(("SockoptFlags", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ReplyAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum BpfStorageReq {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    MapFd(u32),
}
impl<'a> IterableBpfStorageReq<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_map_fd(&self) -> MultiAttrIterable<Self, BpfStorageReq, u32> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let BpfStorageReq::MapFd(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl BpfStorageReq {
    pub fn new<'a>(buf: &'a [u8]) -> IterableBpfStorageReq<'a> {
        IterableBpfStorageReq::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "MapFd",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableBpfStorageReq<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableBpfStorageReq<'a> {
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
impl<'a> Iterator for IterableBpfStorageReq<'a> {
    type Item = Result<BpfStorageReq, ErrorContext>;
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
                1u16 => BpfStorageReq::MapFd({
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
            "BpfStorageReq",
            r#type.and_then(|t| BpfStorageReq::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableBpfStorageReq<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("BpfStorageReq");
        let mut iter = IterableBpfStorageReq::with_loc(&[], self.orig_loc);
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
                BpfStorageReq::MapFd(val) => fmt.field("MapFd", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableBpfStorageReq<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("BpfStorageReq", offset));
            return (
                stack,
                missing_type.and_then(|t| BpfStorageReq::attr_from_type(t)),
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
                BpfStorageReq::MapFd(val) => {
                    if last_off == offset {
                        stack.push(("MapFd", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("BpfStorageReq", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum BpfStorageReply<'a> {
    Storage(IterableBpfStorage<'a>),
}
impl<'a> IterableBpfStorageReply<'a> {
    pub fn get_storage(&self) -> Result<IterableBpfStorage<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(BpfStorageReply::Storage(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BpfStorageReply",
            "Storage",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl BpfStorageReply<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableBpfStorageReply<'a> {
        IterableBpfStorageReply::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Storage",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableBpfStorageReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableBpfStorageReply<'a> {
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
impl<'a> Iterator for IterableBpfStorageReply<'a> {
    type Item = Result<BpfStorageReply<'a>, ErrorContext>;
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
                1u16 => BpfStorageReply::Storage({
                    let res = Some(IterableBpfStorage::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "BpfStorageReply",
            r#type.and_then(|t| BpfStorageReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableBpfStorageReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("BpfStorageReply");
        let mut iter = IterableBpfStorageReply::with_loc(&[], self.orig_loc);
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
                BpfStorageReply::Storage(val) => fmt.field("Storage", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableBpfStorageReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("BpfStorageReply", offset));
            return (
                stack,
                missing_type.and_then(|t| BpfStorageReply::attr_from_type(t)),
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
                BpfStorageReply::Storage(val) => {
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
            stack.push(("BpfStorageReply", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum BpfStorage<'a> {
    Pad(&'a [u8]),
    MapId(u32),
    MapValue(u64),
}
impl<'a> IterableBpfStorage<'a> {
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(BpfStorage::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BpfStorage",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_map_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(BpfStorage::MapId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BpfStorage",
            "MapId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_map_value(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(BpfStorage::MapValue(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "BpfStorage",
            "MapValue",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl BpfStorage<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableBpfStorage<'a> {
        IterableBpfStorage::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Pad",
            2u16 => "MapId",
            3u16 => "MapValue",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableBpfStorage<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableBpfStorage<'a> {
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
impl<'a> Iterator for IterableBpfStorage<'a> {
    type Item = Result<BpfStorage<'a>, ErrorContext>;
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
                1u16 => BpfStorage::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => BpfStorage::MapId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => BpfStorage::MapValue({
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
            "BpfStorage",
            r#type.and_then(|t| BpfStorage::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableBpfStorage<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("BpfStorage");
        let mut iter = IterableBpfStorage::with_loc(&[], self.orig_loc);
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
                BpfStorage::Pad(val) => fmt.field("Pad", &val),
                BpfStorage::MapId(val) => fmt.field("MapId", &val),
                BpfStorage::MapValue(val) => fmt.field("MapValue", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableBpfStorage<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("BpfStorage", offset));
            return (
                stack,
                missing_type.and_then(|t| BpfStorage::attr_from_type(t)),
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
                BpfStorage::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                BpfStorage::MapId(val) => {
                    if last_off == offset {
                        stack.push(("MapId", last_off));
                        break;
                    }
                }
                BpfStorage::MapValue(val) => {
                    if last_off == offset {
                        stack.push(("MapValue", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("BpfStorage", cur));
        }
        (stack, None)
    }
}
pub struct PushUlpInfoAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushUlpInfoAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushUlpInfoAttrs<Prev> {
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
    #[doc = "ULP name (e.g., \\\"tls\\\", \\\"mptcp\\\")\n"]
    pub fn push_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "ULP name (e.g., \\\"tls\\\", \\\"mptcp\\\")\n"]
    pub fn push_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "TLS-specific information\n"]
    pub fn push_tls(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "MPTCP-specific information\n"]
    pub fn push_mptcp(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushUlpInfoAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushRequestAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushRequestAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushRequestAttrs<Prev> {
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
    #[doc = "See bytecode-op\n"]
    pub fn push_bytecode(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn nested_bpf_storages(mut self) -> PushBpfStorageReq<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushBpfStorageReq {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_protocol(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushRequestAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushReplyAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushReplyAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushReplyAttrs<Prev> {
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
    #[doc = "Memory information extension\n"]
    pub fn push_meminfo(mut self, value: Meminfo) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_tcp_info(mut self, value: TcpInfo) -> Self {
        push_header(self.as_vec_mut(), 2u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    #[doc = "TCP Vegas information\n"]
    pub fn push_vegasinfo(mut self, value: TcpvegasInfo) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    #[doc = "Congestion control algorithm name\n"]
    pub fn push_cong(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            4u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "Congestion control algorithm name\n"]
    pub fn push_cong_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 4u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Type of Service\n"]
    pub fn push_tos(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 5u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Traffic Class\n"]
    pub fn push_tclass(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 6u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Socket memory information\n"]
    pub fn push_skmeminfo(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 7u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "Shutdown state\n"]
    pub fn push_shutdown(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 8u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "TCP DCTCP information (request as INET_DIAG_VEGASINFO)\n"]
    pub fn push_dctcpinfo(mut self, value: TcpDctcpInfo) -> Self {
        push_header(self.as_vec_mut(), 9u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    #[doc = "Raw socket protocol (response attribute only)\n"]
    pub fn push_protocol(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 10u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "IPv6-only socket flag\n"]
    pub fn push_skv6only(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 11u16, 0 as u16);
        self
    }
    #[doc = "Local addresses. SCTP thing.\n"]
    pub fn push_locals(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 12u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "Peer addresses. SCTP thing.\n"]
    pub fn push_peers(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 13u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 14u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "Socket mark (only with CAP_NET_ADMIN)\n"]
    pub fn push_mark(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 15u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "TCP BBR information (request as INET_DIAG_VEGASINFO)\n"]
    pub fn push_bbritfo(mut self, value: TcpBbrInfo) -> Self {
        push_header(self.as_vec_mut(), 16u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    #[doc = "Class ID (request as INET_DIAG_TCLASS)\n"]
    pub fn push_class_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 17u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "MD5 signature information\n"]
    pub fn push_md5sig(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 18u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "Upper Layer Protocol information\n"]
    pub fn nested_ulp_info(mut self) -> PushUlpInfoAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 19u16);
        PushUlpInfoAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "BPF storage information\n\nAttribute may repeat multiple times (treat it as array)"]
    pub fn nested_sk_bpf_storages(mut self) -> PushBpfStorageReply<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 20u16);
        PushBpfStorageReply {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Cgroup ID\n"]
    pub fn push_cgroup_id(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 21u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Socket options\n\nAssociated type: [`SockoptFlag`] (enum)"]
    pub fn push_sockopt_flags(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 22u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushReplyAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushBpfStorageReq<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushBpfStorageReq<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushBpfStorageReq<Prev> {
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
    pub fn push_map_fd(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushBpfStorageReq<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushBpfStorageReply<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushBpfStorageReply<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushBpfStorageReply<Prev> {
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
    pub fn nested_storage(mut self) -> PushBpfStorage<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushBpfStorage {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushBpfStorageReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushBpfStorage<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushBpfStorage<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushBpfStorage<Prev> {
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
    pub fn push_map_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_map_value(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 3u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushBpfStorage<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = ""]
#[derive(Debug)]
pub struct OpTcpDiagDump<'r> {
    request: Request<'r>,
}
impl<'r> OpTcpDiagDump<'r> {
    pub fn new(mut request: Request<'r>, header: &ReqV2) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &ReqV2,
    ) -> PushRequestAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushRequestAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushRequestAttrs<&mut Vec<u8>> {
        PushRequestAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushRequestAttrs<RequestBuf<'r>> {
        PushRequestAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (ReqV2, IterableRequestAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(ReqV2::len()));
        (
            ReqV2::new_from_slice(header).unwrap_or_default(),
            IterableRequestAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn decode_reply<'a>(buf: &'a [u8]) -> (Msg, IterableReplyAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Msg::len()));
        (
            Msg::new_from_slice(header).unwrap_or_default(),
            IterableReplyAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &ReqV2) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &ReqV2 {
        let pos = self.request.pos;
        ReqV2::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut ReqV2 {
        let pos = self.request.pos;
        ReqV2::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpTcpDiagDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 4u16,
            request_type: 20u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Msg, IterableReplyAttrs<'buf>);
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        Self::decode_reply(buf)
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
pub struct OpUdpDiagDump<'r> {
    request: Request<'r>,
}
impl<'r> OpUdpDiagDump<'r> {
    pub fn new(mut request: Request<'r>, header: &ReqV2) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &ReqV2,
    ) -> PushRequestAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushRequestAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushRequestAttrs<&mut Vec<u8>> {
        PushRequestAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushRequestAttrs<RequestBuf<'r>> {
        PushRequestAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (ReqV2, IterableRequestAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(ReqV2::len()));
        (
            ReqV2::new_from_slice(header).unwrap_or_default(),
            IterableRequestAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn decode_reply<'a>(buf: &'a [u8]) -> (Msg, IterableReplyAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Msg::len()));
        (
            Msg::new_from_slice(header).unwrap_or_default(),
            IterableReplyAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &ReqV2) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &ReqV2 {
        let pos = self.request.pos;
        ReqV2::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut ReqV2 {
        let pos = self.request.pos;
        ReqV2::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpUdpDiagDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 4u16,
            request_type: 20u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Msg, IterableReplyAttrs<'buf>);
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        Self::decode_reply(buf)
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
    #[doc = ""]
    pub fn op_tcp_diag_dump(self, header: &ReqV2) -> OpTcpDiagDump<'buf> {
        let mut res = OpTcpDiagDump::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-tcp-diag-dump", OpTcpDiagDump::lookup);
        res
    }
    #[doc = ""]
    pub fn op_udp_diag_dump(self, header: &ReqV2) -> OpUdpDiagDump<'buf> {
        let mut res = OpUdpDiagDump::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-udp-diag-dump", OpUdpDiagDump::lookup);
        res
    }
}
