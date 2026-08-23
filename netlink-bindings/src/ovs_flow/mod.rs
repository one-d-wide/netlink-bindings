#![doc = "OVS flow configuration over generic netlink.\n"]
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
pub const PROTONAME: &str = "ovs_flow";
pub const PROTONAME_CSTR: &CStr = c"ovs_flow";
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum OvsFragType {
    #[doc = "Packet is not a fragment.\n"]
    None = 0,
    #[doc = "Packet is a fragment with offset 0.\n"]
    First = 1,
    #[doc = "Packet is a fragment with nonzero offset.\n"]
    Later = 2,
    Any = 255,
}
impl OvsFragType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::None,
            1 => Self::First,
            2 => Self::Later,
            255 => Self::Any,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum OvsUfidFlags {
    OmitKey = 1 << 0,
    OmitMask = 1 << 1,
    OmitActions = 1 << 2,
}
impl OvsUfidFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::OmitKey,
            n if n == 1 << 1 => Self::OmitMask,
            n if n == 1 << 2 => Self::OmitActions,
            _ => return None,
        })
    }
}
#[doc = "Data path hash algorithm for computing Datapath hash. The algorithm type\nonly specifies the fields in a flow will be used as part of the hash.\nEach datapath is free to use its own hash algorithm. The hash value will\nbe opaque to the user space daemon.\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum OvsHashAlg {
    OvsHashAlgL4 = 0,
}
impl OvsHashAlg {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::OvsHashAlgL4,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum CtStateFlags {
    #[doc = "Beginning of a new connection.\n"]
    New = 1 << 0,
    #[doc = "Part of an existing connenction\n"]
    Established = 1 << 1,
    #[doc = "Related to an existing connection.\n"]
    Related = 1 << 2,
    #[doc = "Flow is in the reply direction.\n"]
    ReplyDir = 1 << 3,
    #[doc = "Could not track the connection.\n"]
    Invalid = 1 << 4,
    #[doc = "Conntrack has occurred.\n"]
    Tracked = 1 << 5,
    #[doc = "Packet\\'s source address/port was mangled by NAT.\n"]
    SrcNat = 1 << 6,
    #[doc = "Packet\\'s destination address/port was mangled by NAT.\n"]
    DstNat = 1 << 7,
}
impl CtStateFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::New,
            n if n == 1 << 1 => Self::Established,
            n if n == 1 << 2 => Self::Related,
            n if n == 1 << 3 => Self::ReplyDir,
            n if n == 1 << 4 => Self::Invalid,
            n if n == 1 << 5 => Self::Tracked,
            n if n == 1 << 6 => Self::SrcNat,
            n if n == 1 << 7 => Self::DstNat,
            _ => return None,
        })
    }
}
#[doc = "Header for OVS Generic Netlink messages.\n"]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct OvsHeader {
    #[doc = "ifindex of local port for datapath (0 to make a request not specific to\na datapath).\n"]
    pub dp_ifindex: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for OvsHeader {
    fn default() -> Self {
        Self::new()
    }
}
impl OvsHeader {
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
        const _: () = assert!(std::mem::size_of::<OvsHeader>() == 4usize);
        const _: () = assert!(std::mem::align_of::<OvsHeader>() == 4usize);
        4usize
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(packed(4))]
#[repr(C)]
pub struct OvsFlowStats {
    #[doc = "Number of matched packets.\n"]
    pub n_packets: u64,
    #[doc = "Number of matched bytes.\n"]
    pub n_bytes: u64,
}
#[doc = "Create zero-initialized struct"]
impl Default for OvsFlowStats {
    fn default() -> Self {
        Self::new()
    }
}
impl OvsFlowStats {
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
        const _: () = assert!(std::mem::size_of::<OvsFlowStats>() == 16usize);
        const _: () = assert!(std::mem::align_of::<OvsFlowStats>() == 4usize);
        16usize
    }
}
impl std::fmt::Debug for OvsFlowStats {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("OvsFlowStats")
            .field("n_packets", &{ self.n_packets })
            .field("n_bytes", &{ self.n_bytes })
            .finish()
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct OvsKeyEthernet {
    pub eth_src: [u8; 6usize],
    pub eth_dst: [u8; 6usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for OvsKeyEthernet {
    fn default() -> Self {
        Self::new()
    }
}
impl OvsKeyEthernet {
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
        const _: () = assert!(std::mem::size_of::<OvsKeyEthernet>() == 12usize);
        const _: () = assert!(std::mem::align_of::<OvsKeyEthernet>() == 1usize);
        12usize
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct OvsKeyMpls {
    pub _mpls_lse_be: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for OvsKeyMpls {
    fn default() -> Self {
        Self::new()
    }
}
impl OvsKeyMpls {
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
        const _: () = assert!(std::mem::size_of::<OvsKeyMpls>() == 4usize);
        const _: () = assert!(std::mem::align_of::<OvsKeyMpls>() == 4usize);
        4usize
    }
    pub fn mpls_lse(&self) -> u32 {
        u32::from_be(self._mpls_lse_be)
    }
    pub fn set_mpls_lse(&mut self, value: u32) {
        self._mpls_lse_be = value.to_be();
    }
}
impl std::fmt::Debug for OvsKeyMpls {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("OvsKeyMpls")
            .field("mpls_lse", &self.mpls_lse())
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct OvsKeyIpv4 {
    pub _ipv4_src_be: u32,
    pub _ipv4_dst_be: u32,
    pub ipv4_proto: u8,
    pub ipv4_tos: u8,
    pub ipv4_ttl: u8,
    #[doc = "Associated type: [`OvsFragType`] (enum)"]
    pub ipv4_frag: u8,
}
#[doc = "Create zero-initialized struct"]
impl Default for OvsKeyIpv4 {
    fn default() -> Self {
        Self::new()
    }
}
impl OvsKeyIpv4 {
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
        const _: () = assert!(std::mem::size_of::<OvsKeyIpv4>() == 12usize);
        const _: () = assert!(std::mem::align_of::<OvsKeyIpv4>() == 4usize);
        12usize
    }
    pub fn ipv4_src(&self) -> u32 {
        u32::from_be(self._ipv4_src_be)
    }
    pub fn set_ipv4_src(&mut self, value: u32) {
        self._ipv4_src_be = value.to_be();
    }
    pub fn ipv4_dst(&self) -> u32 {
        u32::from_be(self._ipv4_dst_be)
    }
    pub fn set_ipv4_dst(&mut self, value: u32) {
        self._ipv4_dst_be = value.to_be();
    }
}
impl std::fmt::Debug for OvsKeyIpv4 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("OvsKeyIpv4")
            .field("ipv4_src", &self.ipv4_src())
            .field("ipv4_dst", &self.ipv4_dst())
            .field("ipv4_proto", &self.ipv4_proto)
            .field("ipv4_tos", &self.ipv4_tos)
            .field("ipv4_ttl", &self.ipv4_ttl)
            .field(
                "ipv4_frag",
                &FormatEnum(self.ipv4_frag.into(), OvsFragType::from_value),
            )
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct OvsKeyIpv6 {
    pub ipv6_src: [u8; 16usize],
    pub ipv6_dst: [u8; 16usize],
    pub _ipv6_label_be: u32,
    pub ipv6_proto: u8,
    pub ipv6_tclass: u8,
    pub ipv6_hlimit: u8,
    pub ipv6_frag: u8,
}
#[doc = "Create zero-initialized struct"]
impl Default for OvsKeyIpv6 {
    fn default() -> Self {
        Self::new()
    }
}
impl OvsKeyIpv6 {
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
        const _: () = assert!(std::mem::size_of::<OvsKeyIpv6>() == 40usize);
        const _: () = assert!(std::mem::align_of::<OvsKeyIpv6>() == 4usize);
        40usize
    }
    pub fn ipv6_label(&self) -> u32 {
        u32::from_be(self._ipv6_label_be)
    }
    pub fn set_ipv6_label(&mut self, value: u32) {
        self._ipv6_label_be = value.to_be();
    }
}
impl std::fmt::Debug for OvsKeyIpv6 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("OvsKeyIpv6")
            .field("ipv6_src", &self.ipv6_src)
            .field("ipv6_dst", &self.ipv6_dst)
            .field("ipv6_label", &self.ipv6_label())
            .field("ipv6_proto", &self.ipv6_proto)
            .field("ipv6_tclass", &self.ipv6_tclass)
            .field("ipv6_hlimit", &self.ipv6_hlimit)
            .field("ipv6_frag", &self.ipv6_frag)
            .finish()
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct OvsKeyIpv6Exthdrs {
    pub hdrs: u16,
}
#[doc = "Create zero-initialized struct"]
impl Default for OvsKeyIpv6Exthdrs {
    fn default() -> Self {
        Self::new()
    }
}
impl OvsKeyIpv6Exthdrs {
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
        const _: () = assert!(std::mem::size_of::<OvsKeyIpv6Exthdrs>() == 2usize);
        const _: () = assert!(std::mem::align_of::<OvsKeyIpv6Exthdrs>() == 2usize);
        2usize
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct OvsKeyTcp {
    pub _tcp_src_be: u16,
    pub _tcp_dst_be: u16,
}
#[doc = "Create zero-initialized struct"]
impl Default for OvsKeyTcp {
    fn default() -> Self {
        Self::new()
    }
}
impl OvsKeyTcp {
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
        const _: () = assert!(std::mem::size_of::<OvsKeyTcp>() == 4usize);
        const _: () = assert!(std::mem::align_of::<OvsKeyTcp>() == 2usize);
        4usize
    }
    pub fn tcp_src(&self) -> u16 {
        u16::from_be(self._tcp_src_be)
    }
    pub fn set_tcp_src(&mut self, value: u16) {
        self._tcp_src_be = value.to_be();
    }
    pub fn tcp_dst(&self) -> u16 {
        u16::from_be(self._tcp_dst_be)
    }
    pub fn set_tcp_dst(&mut self, value: u16) {
        self._tcp_dst_be = value.to_be();
    }
}
impl std::fmt::Debug for OvsKeyTcp {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("OvsKeyTcp")
            .field("tcp_src", &self.tcp_src())
            .field("tcp_dst", &self.tcp_dst())
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct OvsKeyUdp {
    pub _udp_src_be: u16,
    pub _udp_dst_be: u16,
}
#[doc = "Create zero-initialized struct"]
impl Default for OvsKeyUdp {
    fn default() -> Self {
        Self::new()
    }
}
impl OvsKeyUdp {
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
        const _: () = assert!(std::mem::size_of::<OvsKeyUdp>() == 4usize);
        const _: () = assert!(std::mem::align_of::<OvsKeyUdp>() == 2usize);
        4usize
    }
    pub fn udp_src(&self) -> u16 {
        u16::from_be(self._udp_src_be)
    }
    pub fn set_udp_src(&mut self, value: u16) {
        self._udp_src_be = value.to_be();
    }
    pub fn udp_dst(&self) -> u16 {
        u16::from_be(self._udp_dst_be)
    }
    pub fn set_udp_dst(&mut self, value: u16) {
        self._udp_dst_be = value.to_be();
    }
}
impl std::fmt::Debug for OvsKeyUdp {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("OvsKeyUdp")
            .field("udp_src", &self.udp_src())
            .field("udp_dst", &self.udp_dst())
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct OvsKeySctp {
    pub _sctp_src_be: u16,
    pub _sctp_dst_be: u16,
}
#[doc = "Create zero-initialized struct"]
impl Default for OvsKeySctp {
    fn default() -> Self {
        Self::new()
    }
}
impl OvsKeySctp {
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
        const _: () = assert!(std::mem::size_of::<OvsKeySctp>() == 4usize);
        const _: () = assert!(std::mem::align_of::<OvsKeySctp>() == 2usize);
        4usize
    }
    pub fn sctp_src(&self) -> u16 {
        u16::from_be(self._sctp_src_be)
    }
    pub fn set_sctp_src(&mut self, value: u16) {
        self._sctp_src_be = value.to_be();
    }
    pub fn sctp_dst(&self) -> u16 {
        u16::from_be(self._sctp_dst_be)
    }
    pub fn set_sctp_dst(&mut self, value: u16) {
        self._sctp_dst_be = value.to_be();
    }
}
impl std::fmt::Debug for OvsKeySctp {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("OvsKeySctp")
            .field("sctp_src", &self.sctp_src())
            .field("sctp_dst", &self.sctp_dst())
            .finish()
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct OvsKeyIcmp {
    pub icmp_type: u8,
    pub icmp_code: u8,
}
#[doc = "Create zero-initialized struct"]
impl Default for OvsKeyIcmp {
    fn default() -> Self {
        Self::new()
    }
}
impl OvsKeyIcmp {
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
        const _: () = assert!(std::mem::size_of::<OvsKeyIcmp>() == 2usize);
        const _: () = assert!(std::mem::align_of::<OvsKeyIcmp>() == 1usize);
        2usize
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct OvsKeyArp {
    pub _arp_sip_be: u32,
    pub _arp_tip_be: u32,
    pub _arp_op_be: u16,
    pub arp_sha: [u8; 6usize],
    pub arp_tha: [u8; 6usize],
    pub _pad_22: [u8; 2usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for OvsKeyArp {
    fn default() -> Self {
        Self::new()
    }
}
impl OvsKeyArp {
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
    pub fn new_from_array(buf: [u8; 24usize]) -> Self {
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
    pub fn as_array(&self) -> &[u8; 24usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 24usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 24usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<OvsKeyArp>() == 24usize);
        const _: () = assert!(std::mem::align_of::<OvsKeyArp>() == 4usize);
        24usize
    }
    pub fn arp_sip(&self) -> u32 {
        u32::from_be(self._arp_sip_be)
    }
    pub fn set_arp_sip(&mut self, value: u32) {
        self._arp_sip_be = value.to_be();
    }
    pub fn arp_tip(&self) -> u32 {
        u32::from_be(self._arp_tip_be)
    }
    pub fn set_arp_tip(&mut self, value: u32) {
        self._arp_tip_be = value.to_be();
    }
    pub fn arp_op(&self) -> u16 {
        u16::from_be(self._arp_op_be)
    }
    pub fn set_arp_op(&mut self, value: u16) {
        self._arp_op_be = value.to_be();
    }
}
impl std::fmt::Debug for OvsKeyArp {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("OvsKeyArp")
            .field("arp_sip", &self.arp_sip())
            .field("arp_tip", &self.arp_tip())
            .field("arp_op", &self.arp_op())
            .field("arp_sha", &self.arp_sha)
            .field("arp_tha", &self.arp_tha)
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct OvsKeyNd {
    pub nd_target: [u8; 16usize],
    pub nd_sll: [u8; 6usize],
    pub nd_tll: [u8; 6usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for OvsKeyNd {
    fn default() -> Self {
        Self::new()
    }
}
impl OvsKeyNd {
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
    pub fn new_from_array(buf: [u8; 28usize]) -> Self {
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
    pub fn as_array(&self) -> &[u8; 28usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 28usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 28usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<OvsKeyNd>() == 28usize);
        const _: () = assert!(std::mem::align_of::<OvsKeyNd>() == 1usize);
        28usize
    }
}
impl std::fmt::Debug for OvsKeyNd {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("OvsKeyNd")
            .field("nd_target", &FormatHexdump(self.nd_target.as_slice()))
            .field("nd_sll", &self.nd_sll)
            .field("nd_tll", &self.nd_tll)
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct OvsKeyCtTupleIpv4 {
    pub _ipv4_src_be: u32,
    pub _ipv4_dst_be: u32,
    pub _src_port_be: u16,
    pub _dst_port_be: u16,
    pub ipv4_proto: u8,
    pub _pad_13: [u8; 3usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for OvsKeyCtTupleIpv4 {
    fn default() -> Self {
        Self::new()
    }
}
impl OvsKeyCtTupleIpv4 {
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
        const _: () = assert!(std::mem::size_of::<OvsKeyCtTupleIpv4>() == 16usize);
        const _: () = assert!(std::mem::align_of::<OvsKeyCtTupleIpv4>() == 4usize);
        16usize
    }
    pub fn ipv4_src(&self) -> u32 {
        u32::from_be(self._ipv4_src_be)
    }
    pub fn set_ipv4_src(&mut self, value: u32) {
        self._ipv4_src_be = value.to_be();
    }
    pub fn ipv4_dst(&self) -> u32 {
        u32::from_be(self._ipv4_dst_be)
    }
    pub fn set_ipv4_dst(&mut self, value: u32) {
        self._ipv4_dst_be = value.to_be();
    }
    pub fn src_port(&self) -> u16 {
        u16::from_be(self._src_port_be)
    }
    pub fn set_src_port(&mut self, value: u16) {
        self._src_port_be = value.to_be();
    }
    pub fn dst_port(&self) -> u16 {
        u16::from_be(self._dst_port_be)
    }
    pub fn set_dst_port(&mut self, value: u16) {
        self._dst_port_be = value.to_be();
    }
}
impl std::fmt::Debug for OvsKeyCtTupleIpv4 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("OvsKeyCtTupleIpv4")
            .field("ipv4_src", &self.ipv4_src())
            .field("ipv4_dst", &self.ipv4_dst())
            .field("src_port", &self.src_port())
            .field("dst_port", &self.dst_port())
            .field("ipv4_proto", &self.ipv4_proto)
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct OvsActionPushVlan {
    #[doc = "Tag protocol identifier (TPID) to push.\n"]
    pub _vlan_tpid_be: u16,
    #[doc = "Tag control identifier (TCI) to push.\n"]
    pub _vlan_tci_be: u16,
}
#[doc = "Create zero-initialized struct"]
impl Default for OvsActionPushVlan {
    fn default() -> Self {
        Self::new()
    }
}
impl OvsActionPushVlan {
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
        const _: () = assert!(std::mem::size_of::<OvsActionPushVlan>() == 4usize);
        const _: () = assert!(std::mem::align_of::<OvsActionPushVlan>() == 2usize);
        4usize
    }
    #[doc = "Tag protocol identifier (TPID) to push.\n"]
    pub fn vlan_tpid(&self) -> u16 {
        u16::from_be(self._vlan_tpid_be)
    }
    #[doc = "Tag protocol identifier (TPID) to push.\n"]
    pub fn set_vlan_tpid(&mut self, value: u16) {
        self._vlan_tpid_be = value.to_be();
    }
    #[doc = "Tag control identifier (TCI) to push.\n"]
    pub fn vlan_tci(&self) -> u16 {
        u16::from_be(self._vlan_tci_be)
    }
    #[doc = "Tag control identifier (TCI) to push.\n"]
    pub fn set_vlan_tci(&mut self, value: u16) {
        self._vlan_tci_be = value.to_be();
    }
}
impl std::fmt::Debug for OvsActionPushVlan {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("OvsActionPushVlan")
            .field("vlan_tpid", &self.vlan_tpid())
            .field("vlan_tci", &self.vlan_tci())
            .finish()
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct OvsActionHash {
    #[doc = "Algorithm used to compute hash prior to recirculation.\n"]
    pub hash_alg: u32,
    #[doc = "Basis used for computing hash.\n"]
    pub hash_basis: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for OvsActionHash {
    fn default() -> Self {
        Self::new()
    }
}
impl OvsActionHash {
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
        const _: () = assert!(std::mem::size_of::<OvsActionHash>() == 8usize);
        const _: () = assert!(std::mem::align_of::<OvsActionHash>() == 4usize);
        8usize
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct OvsActionPushMpls {
    #[doc = "MPLS label stack entry to push\n"]
    pub _mpls_lse_be: u32,
    #[doc = "Ethertype to set in the encapsulating ethernet frame. The only values\nethertype should ever be given are ETH_P_MPLS_UC and ETH_P_MPLS_MC,\nindicating MPLS unicast or multicast. Other are rejected.\n"]
    pub _mpls_ethertype_be: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for OvsActionPushMpls {
    fn default() -> Self {
        Self::new()
    }
}
impl OvsActionPushMpls {
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
        const _: () = assert!(std::mem::size_of::<OvsActionPushMpls>() == 8usize);
        const _: () = assert!(std::mem::align_of::<OvsActionPushMpls>() == 4usize);
        8usize
    }
    #[doc = "MPLS label stack entry to push\n"]
    pub fn mpls_lse(&self) -> u32 {
        u32::from_be(self._mpls_lse_be)
    }
    #[doc = "MPLS label stack entry to push\n"]
    pub fn set_mpls_lse(&mut self, value: u32) {
        self._mpls_lse_be = value.to_be();
    }
    #[doc = "Ethertype to set in the encapsulating ethernet frame. The only values\nethertype should ever be given are ETH_P_MPLS_UC and ETH_P_MPLS_MC,\nindicating MPLS unicast or multicast. Other are rejected.\n"]
    pub fn mpls_ethertype(&self) -> u32 {
        u32::from_be(self._mpls_ethertype_be)
    }
    #[doc = "Ethertype to set in the encapsulating ethernet frame. The only values\nethertype should ever be given are ETH_P_MPLS_UC and ETH_P_MPLS_MC,\nindicating MPLS unicast or multicast. Other are rejected.\n"]
    pub fn set_mpls_ethertype(&mut self, value: u32) {
        self._mpls_ethertype_be = value.to_be();
    }
}
impl std::fmt::Debug for OvsActionPushMpls {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("OvsActionPushMpls")
            .field("mpls_lse", &self.mpls_lse())
            .field("mpls_ethertype", &self.mpls_ethertype())
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct OvsActionAddMpls {
    #[doc = "MPLS label stack entry to push\n"]
    pub _mpls_lse_be: u32,
    #[doc = "Ethertype to set in the encapsulating ethernet frame. The only values\nethertype should ever be given are ETH_P_MPLS_UC and ETH_P_MPLS_MC,\nindicating MPLS unicast or multicast. Other are rejected.\n"]
    pub _mpls_ethertype_be: u32,
    #[doc = "MPLS tunnel attributes.\n"]
    pub tun_flags: u16,
    pub _pad_10: [u8; 2usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for OvsActionAddMpls {
    fn default() -> Self {
        Self::new()
    }
}
impl OvsActionAddMpls {
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
        const _: () = assert!(std::mem::size_of::<OvsActionAddMpls>() == 12usize);
        const _: () = assert!(std::mem::align_of::<OvsActionAddMpls>() == 4usize);
        12usize
    }
    #[doc = "MPLS label stack entry to push\n"]
    pub fn mpls_lse(&self) -> u32 {
        u32::from_be(self._mpls_lse_be)
    }
    #[doc = "MPLS label stack entry to push\n"]
    pub fn set_mpls_lse(&mut self, value: u32) {
        self._mpls_lse_be = value.to_be();
    }
    #[doc = "Ethertype to set in the encapsulating ethernet frame. The only values\nethertype should ever be given are ETH_P_MPLS_UC and ETH_P_MPLS_MC,\nindicating MPLS unicast or multicast. Other are rejected.\n"]
    pub fn mpls_ethertype(&self) -> u32 {
        u32::from_be(self._mpls_ethertype_be)
    }
    #[doc = "Ethertype to set in the encapsulating ethernet frame. The only values\nethertype should ever be given are ETH_P_MPLS_UC and ETH_P_MPLS_MC,\nindicating MPLS unicast or multicast. Other are rejected.\n"]
    pub fn set_mpls_ethertype(&mut self, value: u32) {
        self._mpls_ethertype_be = value.to_be();
    }
}
impl std::fmt::Debug for OvsActionAddMpls {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("OvsActionAddMpls")
            .field("mpls_lse", &self.mpls_lse())
            .field("mpls_ethertype", &self.mpls_ethertype())
            .field("tun_flags", &self.tun_flags)
            .finish()
    }
}
#[derive(Clone)]
pub enum FlowAttrs<'a> {
    #[doc = "Nested attributes specifying the flow key. Always present in\nnotifications. Required for all requests (except dumps).\n"]
    Key(IterableKeyAttrs<'a>),
    #[doc = "Nested attributes specifying the actions to take for packets that match\nthe key. Always present in notifications. Required for OVS_FLOW_CMD_NEW\nrequests, optional for OVS_FLOW_CMD_SET requests. An OVS_FLOW_CMD_SET\nwithout OVS_FLOW_ATTR_ACTIONS will not modify the actions. To clear the\nactions, an OVS_FLOW_ATTR_ACTIONS without any nested attributes must be\ngiven.\n"]
    Actions(IterableActionAttrs<'a>),
    #[doc = "Statistics for this flow. Present in notifications if the stats would be\nnonzero. Ignored in requests.\n"]
    Stats(OvsFlowStats),
    #[doc = "An 8-bit value giving the ORed value of all of the TCP flags seen on\npackets in this flow. Only present in notifications for TCP flows, and\nonly if it would be nonzero. Ignored in requests.\n"]
    TcpFlags(u8),
    #[doc = "A 64-bit integer giving the time, in milliseconds on the system\nmonotonic clock, at which a packet was last processed for this flow.\nOnly present in notifications if a packet has been processed for this\nflow. Ignored in requests.\n"]
    Used(u64),
    #[doc = "If present in a OVS_FLOW_CMD_SET request, clears the last-used time,\naccumulated TCP flags, and statistics for this flow. Otherwise ignored\nin requests. Never present in notifications.\n"]
    Clear(()),
    #[doc = "Nested attributes specifying the mask bits for wildcarded flow match.\nMask bit value \\'1\\' specifies exact match with corresponding flow key\nbit, while mask bit value \\'0\\' specifies a wildcarded match. Omitting\nattribute is treated as wildcarding all corresponding fields. Optional\nfor all requests. If not present, all flow key bits are exact match\nbits.\n"]
    Mask(IterableKeyAttrs<'a>),
    #[doc = "Flow operation is a feature probe, error logging should be suppressed.\n"]
    Probe(&'a [u8]),
    #[doc = "A value between 1-16 octets specifying a unique identifier for the flow.\nCauses the flow to be indexed by this value rather than the value of the\nOVS_FLOW_ATTR_KEY attribute. Optional for all requests. Present in\nnotifications if the flow was created with this attribute.\n"]
    Ufid(&'a [u8]),
    #[doc = "A 32-bit value of ORed flags that provide alternative semantics for flow\ninstallation and retrieval. Optional for all requests.\n\nAssociated type: [`OvsUfidFlags`] (enum)"]
    UfidFlags(u32),
    Pad(&'a [u8]),
}
impl<'a> IterableFlowAttrs<'a> {
    #[doc = "Nested attributes specifying the flow key. Always present in\nnotifications. Required for all requests (except dumps).\n"]
    pub fn get_key(&self) -> Result<IterableKeyAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FlowAttrs::Key(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FlowAttrs",
            "Key",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Nested attributes specifying the actions to take for packets that match\nthe key. Always present in notifications. Required for OVS_FLOW_CMD_NEW\nrequests, optional for OVS_FLOW_CMD_SET requests. An OVS_FLOW_CMD_SET\nwithout OVS_FLOW_ATTR_ACTIONS will not modify the actions. To clear the\nactions, an OVS_FLOW_ATTR_ACTIONS without any nested attributes must be\ngiven.\n"]
    pub fn get_actions(&self) -> Result<IterableActionAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FlowAttrs::Actions(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FlowAttrs",
            "Actions",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Statistics for this flow. Present in notifications if the stats would be\nnonzero. Ignored in requests.\n"]
    pub fn get_stats(&self) -> Result<OvsFlowStats, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FlowAttrs::Stats(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FlowAttrs",
            "Stats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "An 8-bit value giving the ORed value of all of the TCP flags seen on\npackets in this flow. Only present in notifications for TCP flows, and\nonly if it would be nonzero. Ignored in requests.\n"]
    pub fn get_tcp_flags(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FlowAttrs::TcpFlags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FlowAttrs",
            "TcpFlags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "A 64-bit integer giving the time, in milliseconds on the system\nmonotonic clock, at which a packet was last processed for this flow.\nOnly present in notifications if a packet has been processed for this\nflow. Ignored in requests.\n"]
    pub fn get_used(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FlowAttrs::Used(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FlowAttrs",
            "Used",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "If present in a OVS_FLOW_CMD_SET request, clears the last-used time,\naccumulated TCP flags, and statistics for this flow. Otherwise ignored\nin requests. Never present in notifications.\n"]
    pub fn get_clear(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FlowAttrs::Clear(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FlowAttrs",
            "Clear",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Nested attributes specifying the mask bits for wildcarded flow match.\nMask bit value \\'1\\' specifies exact match with corresponding flow key\nbit, while mask bit value \\'0\\' specifies a wildcarded match. Omitting\nattribute is treated as wildcarding all corresponding fields. Optional\nfor all requests. If not present, all flow key bits are exact match\nbits.\n"]
    pub fn get_mask(&self) -> Result<IterableKeyAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FlowAttrs::Mask(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FlowAttrs",
            "Mask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Flow operation is a feature probe, error logging should be suppressed.\n"]
    pub fn get_probe(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FlowAttrs::Probe(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FlowAttrs",
            "Probe",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "A value between 1-16 octets specifying a unique identifier for the flow.\nCauses the flow to be indexed by this value rather than the value of the\nOVS_FLOW_ATTR_KEY attribute. Optional for all requests. Present in\nnotifications if the flow was created with this attribute.\n"]
    pub fn get_ufid(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FlowAttrs::Ufid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FlowAttrs",
            "Ufid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "A 32-bit value of ORed flags that provide alternative semantics for flow\ninstallation and retrieval. Optional for all requests.\n\nAssociated type: [`OvsUfidFlags`] (enum)"]
    pub fn get_ufid_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FlowAttrs::UfidFlags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FlowAttrs",
            "UfidFlags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(FlowAttrs::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "FlowAttrs",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl FlowAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableFlowAttrs<'a> {
        IterableFlowAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Key",
            2u16 => "Actions",
            3u16 => "Stats",
            4u16 => "TcpFlags",
            5u16 => "Used",
            6u16 => "Clear",
            7u16 => "Mask",
            8u16 => "Probe",
            9u16 => "Ufid",
            10u16 => "UfidFlags",
            11u16 => "Pad",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableFlowAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableFlowAttrs<'a> {
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
impl<'a> Iterator for IterableFlowAttrs<'a> {
    type Item = Result<FlowAttrs<'a>, ErrorContext>;
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
                1u16 => FlowAttrs::Key({
                    let res = Some(IterableKeyAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => FlowAttrs::Actions({
                    let res = Some(IterableActionAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => FlowAttrs::Stats({
                    let res = Some(OvsFlowStats::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => FlowAttrs::TcpFlags({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => FlowAttrs::Used({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => FlowAttrs::Clear(()),
                7u16 => FlowAttrs::Mask({
                    let res = Some(IterableKeyAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => FlowAttrs::Probe({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => FlowAttrs::Ufid({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => FlowAttrs::UfidFlags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => FlowAttrs::Pad({
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
            "FlowAttrs",
            r#type.and_then(|t| FlowAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableFlowAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("FlowAttrs");
        let mut iter = IterableFlowAttrs::with_loc(&[], self.orig_loc);
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
                FlowAttrs::Key(val) => fmt.field("Key", &val),
                FlowAttrs::Actions(val) => fmt.field("Actions", &val),
                FlowAttrs::Stats(val) => fmt.field("Stats", &val),
                FlowAttrs::TcpFlags(val) => fmt.field("TcpFlags", &val),
                FlowAttrs::Used(val) => fmt.field("Used", &val),
                FlowAttrs::Clear(val) => fmt.field("Clear", &val),
                FlowAttrs::Mask(val) => fmt.field("Mask", &val),
                FlowAttrs::Probe(val) => fmt.field("Probe", &FormatHexdump(val)),
                FlowAttrs::Ufid(val) => fmt.field("Ufid", &val),
                FlowAttrs::UfidFlags(val) => fmt.field(
                    "UfidFlags",
                    &FormatFlags(val.into(), OvsUfidFlags::from_value),
                ),
                FlowAttrs::Pad(val) => fmt.field("Pad", &FormatHexdump(val)),
            };
        }
        fmt.finish()
    }
}
impl IterableFlowAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("FlowAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| FlowAttrs::attr_from_type(t)),
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
                FlowAttrs::Key(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                FlowAttrs::Actions(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                FlowAttrs::Stats(val) => {
                    if last_off == offset {
                        stack.push(("Stats", last_off));
                        break;
                    }
                }
                FlowAttrs::TcpFlags(val) => {
                    if last_off == offset {
                        stack.push(("TcpFlags", last_off));
                        break;
                    }
                }
                FlowAttrs::Used(val) => {
                    if last_off == offset {
                        stack.push(("Used", last_off));
                        break;
                    }
                }
                FlowAttrs::Clear(val) => {
                    if last_off == offset {
                        stack.push(("Clear", last_off));
                        break;
                    }
                }
                FlowAttrs::Mask(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                FlowAttrs::Probe(val) => {
                    if last_off == offset {
                        stack.push(("Probe", last_off));
                        break;
                    }
                }
                FlowAttrs::Ufid(val) => {
                    if last_off == offset {
                        stack.push(("Ufid", last_off));
                        break;
                    }
                }
                FlowAttrs::UfidFlags(val) => {
                    if last_off == offset {
                        stack.push(("UfidFlags", last_off));
                        break;
                    }
                }
                FlowAttrs::Pad(val) => {
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
            stack.push(("FlowAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum KeyAttrs<'a> {
    Encap(IterableKeyAttrs<'a>),
    Priority(u32),
    InPort(u32),
    #[doc = "struct ovs_key_ethernet\n"]
    Ethernet(OvsKeyEthernet),
    Vlan(u16),
    Ethertype(u16),
    Ipv4(OvsKeyIpv4),
    #[doc = "struct ovs_key_ipv6\n"]
    Ipv6(OvsKeyIpv6),
    Tcp(OvsKeyTcp),
    Udp(OvsKeyUdp),
    Icmp(OvsKeyIcmp),
    Icmpv6(OvsKeyIcmp),
    #[doc = "struct ovs_key_arp\n"]
    Arp(OvsKeyArp),
    #[doc = "struct ovs_key_nd\n"]
    Nd(OvsKeyNd),
    SkbMark(u32),
    Tunnel(IterableTunnelKeyAttrs<'a>),
    Sctp(OvsKeySctp),
    TcpFlags(u16),
    #[doc = "Value 0 indicates the hash is not computed by the datapath.\n"]
    DpHash(u32),
    RecircId(u32),
    Mpls(OvsKeyMpls),
    #[doc = "Associated type: [`CtStateFlags`] (1 bit per enumeration)"]
    CtState(u32),
    #[doc = "connection tracking zone\n"]
    CtZone(u16),
    #[doc = "connection tracking mark\n"]
    CtMark(u32),
    #[doc = "16-octet connection tracking label\n"]
    CtLabels(&'a [u8]),
    CtOrigTupleIpv4(OvsKeyCtTupleIpv4),
    #[doc = "struct ovs_key_ct_tuple_ipv6\n"]
    CtOrigTupleIpv6(&'a [u8]),
    Nsh(IterableOvsNshKeyAttrs<'a>),
    #[doc = "Should not be sent to the kernel\n"]
    PacketType(u32),
    #[doc = "Should not be sent to the kernel\n"]
    NdExtensions(&'a [u8]),
    #[doc = "struct ip_tunnel_info\n"]
    TunnelInfo(&'a [u8]),
    #[doc = "struct ovs_key_ipv6_exthdr\n"]
    Ipv6Exthdrs(OvsKeyIpv6Exthdrs),
}
impl<'a> IterableKeyAttrs<'a> {
    pub fn get_encap(&self) -> Result<IterableKeyAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::Encap(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "Encap",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_priority(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::Priority(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "Priority",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_in_port(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::InPort(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "InPort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "struct ovs_key_ethernet\n"]
    pub fn get_ethernet(&self) -> Result<OvsKeyEthernet, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::Ethernet(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "Ethernet",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vlan(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::Vlan(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "Vlan",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ethertype(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::Ethertype(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "Ethertype",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ipv4(&self) -> Result<OvsKeyIpv4, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::Ipv4(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "Ipv4",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "struct ovs_key_ipv6\n"]
    pub fn get_ipv6(&self) -> Result<OvsKeyIpv6, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::Ipv6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "Ipv6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tcp(&self) -> Result<OvsKeyTcp, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::Tcp(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "Tcp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_udp(&self) -> Result<OvsKeyUdp, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::Udp(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "Udp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_icmp(&self) -> Result<OvsKeyIcmp, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::Icmp(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "Icmp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_icmpv6(&self) -> Result<OvsKeyIcmp, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::Icmpv6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "Icmpv6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "struct ovs_key_arp\n"]
    pub fn get_arp(&self) -> Result<OvsKeyArp, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::Arp(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "Arp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "struct ovs_key_nd\n"]
    pub fn get_nd(&self) -> Result<OvsKeyNd, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::Nd(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "Nd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_skb_mark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::SkbMark(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "SkbMark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tunnel(&self) -> Result<IterableTunnelKeyAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::Tunnel(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "Tunnel",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sctp(&self) -> Result<OvsKeySctp, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::Sctp(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "Sctp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tcp_flags(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::TcpFlags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "TcpFlags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Value 0 indicates the hash is not computed by the datapath.\n"]
    pub fn get_dp_hash(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::DpHash(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "DpHash",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_recirc_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::RecircId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "RecircId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mpls(&self) -> Result<OvsKeyMpls, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::Mpls(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "Mpls",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`CtStateFlags`] (1 bit per enumeration)"]
    pub fn get_ct_state(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::CtState(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "CtState",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "connection tracking zone\n"]
    pub fn get_ct_zone(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::CtZone(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "CtZone",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "connection tracking mark\n"]
    pub fn get_ct_mark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::CtMark(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "CtMark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "16-octet connection tracking label\n"]
    pub fn get_ct_labels(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::CtLabels(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "CtLabels",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ct_orig_tuple_ipv4(&self) -> Result<OvsKeyCtTupleIpv4, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::CtOrigTupleIpv4(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "CtOrigTupleIpv4",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "struct ovs_key_ct_tuple_ipv6\n"]
    pub fn get_ct_orig_tuple_ipv6(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::CtOrigTupleIpv6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "CtOrigTupleIpv6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nsh(&self) -> Result<IterableOvsNshKeyAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::Nsh(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "Nsh",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Should not be sent to the kernel\n"]
    pub fn get_packet_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::PacketType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "PacketType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Should not be sent to the kernel\n"]
    pub fn get_nd_extensions(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::NdExtensions(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "NdExtensions",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "struct ip_tunnel_info\n"]
    pub fn get_tunnel_info(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::TunnelInfo(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "TunnelInfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "struct ovs_key_ipv6_exthdr\n"]
    pub fn get_ipv6_exthdrs(&self) -> Result<OvsKeyIpv6Exthdrs, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyAttrs::Ipv6Exthdrs(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyAttrs",
            "Ipv6Exthdrs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl KeyAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableKeyAttrs<'a> {
        IterableKeyAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Encap",
            2u16 => "Priority",
            3u16 => "InPort",
            4u16 => "Ethernet",
            5u16 => "Vlan",
            6u16 => "Ethertype",
            7u16 => "Ipv4",
            8u16 => "Ipv6",
            9u16 => "Tcp",
            10u16 => "Udp",
            11u16 => "Icmp",
            12u16 => "Icmpv6",
            13u16 => "Arp",
            14u16 => "Nd",
            15u16 => "SkbMark",
            16u16 => "Tunnel",
            17u16 => "Sctp",
            18u16 => "TcpFlags",
            19u16 => "DpHash",
            20u16 => "RecircId",
            21u16 => "Mpls",
            22u16 => "CtState",
            23u16 => "CtZone",
            24u16 => "CtMark",
            25u16 => "CtLabels",
            26u16 => "CtOrigTupleIpv4",
            27u16 => "CtOrigTupleIpv6",
            28u16 => "Nsh",
            29u16 => "PacketType",
            30u16 => "NdExtensions",
            31u16 => "TunnelInfo",
            32u16 => "Ipv6Exthdrs",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableKeyAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableKeyAttrs<'a> {
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
impl<'a> Iterator for IterableKeyAttrs<'a> {
    type Item = Result<KeyAttrs<'a>, ErrorContext>;
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
                1u16 => KeyAttrs::Encap({
                    let res = Some(IterableKeyAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => KeyAttrs::Priority({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => KeyAttrs::InPort({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => KeyAttrs::Ethernet({
                    let res = Some(OvsKeyEthernet::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => KeyAttrs::Vlan({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => KeyAttrs::Ethertype({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => KeyAttrs::Ipv4({
                    let res = Some(OvsKeyIpv4::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => KeyAttrs::Ipv6({
                    let res = Some(OvsKeyIpv6::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => KeyAttrs::Tcp({
                    let res = Some(OvsKeyTcp::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => KeyAttrs::Udp({
                    let res = Some(OvsKeyUdp::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => KeyAttrs::Icmp({
                    let res = Some(OvsKeyIcmp::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => KeyAttrs::Icmpv6({
                    let res = Some(OvsKeyIcmp::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => KeyAttrs::Arp({
                    let res = Some(OvsKeyArp::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => KeyAttrs::Nd({
                    let res = Some(OvsKeyNd::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => KeyAttrs::SkbMark({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => KeyAttrs::Tunnel({
                    let res = Some(IterableTunnelKeyAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => KeyAttrs::Sctp({
                    let res = Some(OvsKeySctp::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => KeyAttrs::TcpFlags({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => KeyAttrs::DpHash({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => KeyAttrs::RecircId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => KeyAttrs::Mpls({
                    let res = Some(OvsKeyMpls::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => KeyAttrs::CtState({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => KeyAttrs::CtZone({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => KeyAttrs::CtMark({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                25u16 => KeyAttrs::CtLabels({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                26u16 => KeyAttrs::CtOrigTupleIpv4({
                    let res = Some(OvsKeyCtTupleIpv4::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                27u16 => KeyAttrs::CtOrigTupleIpv6({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                28u16 => KeyAttrs::Nsh({
                    let res = Some(IterableOvsNshKeyAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                29u16 => KeyAttrs::PacketType({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                30u16 => KeyAttrs::NdExtensions({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                31u16 => KeyAttrs::TunnelInfo({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                32u16 => KeyAttrs::Ipv6Exthdrs({
                    let res = Some(OvsKeyIpv6Exthdrs::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "KeyAttrs",
            r#type.and_then(|t| KeyAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableKeyAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("KeyAttrs");
        let mut iter = IterableKeyAttrs::with_loc(&[], self.orig_loc);
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
                KeyAttrs::Encap(val) => fmt.field("Encap", &val),
                KeyAttrs::Priority(val) => fmt.field("Priority", &val),
                KeyAttrs::InPort(val) => fmt.field("InPort", &val),
                KeyAttrs::Ethernet(val) => fmt.field("Ethernet", &val),
                KeyAttrs::Vlan(val) => fmt.field("Vlan", &val),
                KeyAttrs::Ethertype(val) => fmt.field("Ethertype", &val),
                KeyAttrs::Ipv4(val) => fmt.field("Ipv4", &val),
                KeyAttrs::Ipv6(val) => fmt.field("Ipv6", &val),
                KeyAttrs::Tcp(val) => fmt.field("Tcp", &val),
                KeyAttrs::Udp(val) => fmt.field("Udp", &val),
                KeyAttrs::Icmp(val) => fmt.field("Icmp", &val),
                KeyAttrs::Icmpv6(val) => fmt.field("Icmpv6", &val),
                KeyAttrs::Arp(val) => fmt.field("Arp", &val),
                KeyAttrs::Nd(val) => fmt.field("Nd", &val),
                KeyAttrs::SkbMark(val) => fmt.field("SkbMark", &val),
                KeyAttrs::Tunnel(val) => fmt.field("Tunnel", &val),
                KeyAttrs::Sctp(val) => fmt.field("Sctp", &val),
                KeyAttrs::TcpFlags(val) => fmt.field("TcpFlags", &val),
                KeyAttrs::DpHash(val) => fmt.field("DpHash", &val),
                KeyAttrs::RecircId(val) => fmt.field("RecircId", &val),
                KeyAttrs::Mpls(val) => fmt.field("Mpls", &val),
                KeyAttrs::CtState(val) => fmt.field(
                    "CtState",
                    &FormatFlags(val.into(), CtStateFlags::from_value),
                ),
                KeyAttrs::CtZone(val) => fmt.field("CtZone", &val),
                KeyAttrs::CtMark(val) => fmt.field("CtMark", &val),
                KeyAttrs::CtLabels(val) => fmt.field("CtLabels", &FormatHex(val)),
                KeyAttrs::CtOrigTupleIpv4(val) => fmt.field("CtOrigTupleIpv4", &val),
                KeyAttrs::CtOrigTupleIpv6(val) => fmt.field("CtOrigTupleIpv6", &FormatHexdump(val)),
                KeyAttrs::Nsh(val) => fmt.field("Nsh", &val),
                KeyAttrs::PacketType(val) => fmt.field("PacketType", &val),
                KeyAttrs::NdExtensions(val) => fmt.field("NdExtensions", &FormatHexdump(val)),
                KeyAttrs::TunnelInfo(val) => fmt.field("TunnelInfo", &FormatHexdump(val)),
                KeyAttrs::Ipv6Exthdrs(val) => fmt.field("Ipv6Exthdrs", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableKeyAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("KeyAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| KeyAttrs::attr_from_type(t)),
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
                KeyAttrs::Encap(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                KeyAttrs::Priority(val) => {
                    if last_off == offset {
                        stack.push(("Priority", last_off));
                        break;
                    }
                }
                KeyAttrs::InPort(val) => {
                    if last_off == offset {
                        stack.push(("InPort", last_off));
                        break;
                    }
                }
                KeyAttrs::Ethernet(val) => {
                    if last_off == offset {
                        stack.push(("Ethernet", last_off));
                        break;
                    }
                }
                KeyAttrs::Vlan(val) => {
                    if last_off == offset {
                        stack.push(("Vlan", last_off));
                        break;
                    }
                }
                KeyAttrs::Ethertype(val) => {
                    if last_off == offset {
                        stack.push(("Ethertype", last_off));
                        break;
                    }
                }
                KeyAttrs::Ipv4(val) => {
                    if last_off == offset {
                        stack.push(("Ipv4", last_off));
                        break;
                    }
                }
                KeyAttrs::Ipv6(val) => {
                    if last_off == offset {
                        stack.push(("Ipv6", last_off));
                        break;
                    }
                }
                KeyAttrs::Tcp(val) => {
                    if last_off == offset {
                        stack.push(("Tcp", last_off));
                        break;
                    }
                }
                KeyAttrs::Udp(val) => {
                    if last_off == offset {
                        stack.push(("Udp", last_off));
                        break;
                    }
                }
                KeyAttrs::Icmp(val) => {
                    if last_off == offset {
                        stack.push(("Icmp", last_off));
                        break;
                    }
                }
                KeyAttrs::Icmpv6(val) => {
                    if last_off == offset {
                        stack.push(("Icmpv6", last_off));
                        break;
                    }
                }
                KeyAttrs::Arp(val) => {
                    if last_off == offset {
                        stack.push(("Arp", last_off));
                        break;
                    }
                }
                KeyAttrs::Nd(val) => {
                    if last_off == offset {
                        stack.push(("Nd", last_off));
                        break;
                    }
                }
                KeyAttrs::SkbMark(val) => {
                    if last_off == offset {
                        stack.push(("SkbMark", last_off));
                        break;
                    }
                }
                KeyAttrs::Tunnel(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                KeyAttrs::Sctp(val) => {
                    if last_off == offset {
                        stack.push(("Sctp", last_off));
                        break;
                    }
                }
                KeyAttrs::TcpFlags(val) => {
                    if last_off == offset {
                        stack.push(("TcpFlags", last_off));
                        break;
                    }
                }
                KeyAttrs::DpHash(val) => {
                    if last_off == offset {
                        stack.push(("DpHash", last_off));
                        break;
                    }
                }
                KeyAttrs::RecircId(val) => {
                    if last_off == offset {
                        stack.push(("RecircId", last_off));
                        break;
                    }
                }
                KeyAttrs::Mpls(val) => {
                    if last_off == offset {
                        stack.push(("Mpls", last_off));
                        break;
                    }
                }
                KeyAttrs::CtState(val) => {
                    if last_off == offset {
                        stack.push(("CtState", last_off));
                        break;
                    }
                }
                KeyAttrs::CtZone(val) => {
                    if last_off == offset {
                        stack.push(("CtZone", last_off));
                        break;
                    }
                }
                KeyAttrs::CtMark(val) => {
                    if last_off == offset {
                        stack.push(("CtMark", last_off));
                        break;
                    }
                }
                KeyAttrs::CtLabels(val) => {
                    if last_off == offset {
                        stack.push(("CtLabels", last_off));
                        break;
                    }
                }
                KeyAttrs::CtOrigTupleIpv4(val) => {
                    if last_off == offset {
                        stack.push(("CtOrigTupleIpv4", last_off));
                        break;
                    }
                }
                KeyAttrs::CtOrigTupleIpv6(val) => {
                    if last_off == offset {
                        stack.push(("CtOrigTupleIpv6", last_off));
                        break;
                    }
                }
                KeyAttrs::Nsh(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                KeyAttrs::PacketType(val) => {
                    if last_off == offset {
                        stack.push(("PacketType", last_off));
                        break;
                    }
                }
                KeyAttrs::NdExtensions(val) => {
                    if last_off == offset {
                        stack.push(("NdExtensions", last_off));
                        break;
                    }
                }
                KeyAttrs::TunnelInfo(val) => {
                    if last_off == offset {
                        stack.push(("TunnelInfo", last_off));
                        break;
                    }
                }
                KeyAttrs::Ipv6Exthdrs(val) => {
                    if last_off == offset {
                        stack.push(("Ipv6Exthdrs", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("KeyAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum ActionAttrs<'a> {
    #[doc = "ovs port number in datapath\n"]
    Output(u32),
    Userspace(IterableUserspaceAttrs<'a>),
    #[doc = "Replaces the contents of an existing header. The single nested attribute\nspecifies a header to modify and its value.\n"]
    Set(IterableKeyAttrs<'a>),
    #[doc = "Push a new outermost 802.1Q or 802.1ad header onto the packet.\n"]
    PushVlan(OvsActionPushVlan),
    #[doc = "Pop the outermost 802.1Q or 802.1ad header from the packet.\n"]
    PopVlan(()),
    #[doc = "Probabilistically executes actions, as specified in the nested\nattributes.\n"]
    Sample(IterableSampleAttrs<'a>),
    #[doc = "recirc id\n"]
    Recirc(u32),
    Hash(OvsActionHash),
    #[doc = "Push a new MPLS label stack entry onto the top of the packets MPLS label\nstack. Set the ethertype of the encapsulating frame to either\nETH_P_MPLS_UC or ETH_P_MPLS_MC to indicate the new packet contents.\n"]
    PushMpls(OvsActionPushMpls),
    #[doc = "ethertype\n"]
    PopMpls(u16),
    #[doc = "Replaces the contents of an existing header. A nested attribute\nspecifies a header to modify, its value, and a mask. For every bit set\nin the mask, the corresponding bit value is copied from the value to the\npacket header field, rest of the bits are left unchanged. The non-masked\nvalue bits must be passed in as zeroes. Masking is not supported for the\nOVS_KEY_ATTR_TUNNEL attribute.\n"]
    SetMasked(IterableKeyAttrs<'a>),
    #[doc = "Track the connection. Populate the conntrack-related entries in the flow\nkey.\n"]
    Ct(IterableCtAttrs<'a>),
    #[doc = "struct ovs_action_trunc is a u32 max length\n"]
    Trunc(u32),
    #[doc = "struct ovs_action_push_eth\n"]
    PushEth(&'a [u8]),
    PopEth(()),
    CtClear(()),
    #[doc = "Push NSH header to the packet.\n"]
    PushNsh(IterableOvsNshKeyAttrs<'a>),
    #[doc = "Pop the outermost NSH header off the packet.\n"]
    PopNsh(()),
    #[doc = "Run packet through a meter, which may drop the packet, or modify the\npacket (e.g., change the DSCP field)\n"]
    Meter(u32),
    #[doc = "Make a copy of the packet and execute a list of actions without\naffecting the original packet and key.\n"]
    Clone(IterableActionAttrs<'a>),
    #[doc = "Check the packet length and execute a set of actions if greater than the\nspecified packet length, else execute another set of actions.\n"]
    CheckPktLen(IterableCheckPktLenAttrs<'a>),
    #[doc = "Push a new MPLS label stack entry at the start of the packet or at the\nstart of the l3 header depending on the value of l3 tunnel flag in the\ntun_flags field of this OVS_ACTION_ATTR_ADD_MPLS argument.\n"]
    AddMpls(OvsActionAddMpls),
    DecTtl(IterableDecTtlAttrs<'a>),
    #[doc = "Sends a packet sample to psample for external observation.\n"]
    Psample(IterablePsampleAttrs<'a>),
}
impl<'a> IterableActionAttrs<'a> {
    #[doc = "ovs port number in datapath\n"]
    pub fn get_output(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ActionAttrs::Output(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ActionAttrs",
            "Output",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_userspace(&self) -> Result<IterableUserspaceAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ActionAttrs::Userspace(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ActionAttrs",
            "Userspace",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Replaces the contents of an existing header. The single nested attribute\nspecifies a header to modify and its value.\n"]
    pub fn get_set(&self) -> Result<IterableKeyAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ActionAttrs::Set(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ActionAttrs",
            "Set",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Push a new outermost 802.1Q or 802.1ad header onto the packet.\n"]
    pub fn get_push_vlan(&self) -> Result<OvsActionPushVlan, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ActionAttrs::PushVlan(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ActionAttrs",
            "PushVlan",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Pop the outermost 802.1Q or 802.1ad header from the packet.\n"]
    pub fn get_pop_vlan(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ActionAttrs::PopVlan(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ActionAttrs",
            "PopVlan",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Probabilistically executes actions, as specified in the nested\nattributes.\n"]
    pub fn get_sample(&self) -> Result<IterableSampleAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ActionAttrs::Sample(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ActionAttrs",
            "Sample",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "recirc id\n"]
    pub fn get_recirc(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ActionAttrs::Recirc(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ActionAttrs",
            "Recirc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_hash(&self) -> Result<OvsActionHash, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ActionAttrs::Hash(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ActionAttrs",
            "Hash",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Push a new MPLS label stack entry onto the top of the packets MPLS label\nstack. Set the ethertype of the encapsulating frame to either\nETH_P_MPLS_UC or ETH_P_MPLS_MC to indicate the new packet contents.\n"]
    pub fn get_push_mpls(&self) -> Result<OvsActionPushMpls, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ActionAttrs::PushMpls(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ActionAttrs",
            "PushMpls",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ethertype\n"]
    pub fn get_pop_mpls(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ActionAttrs::PopMpls(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ActionAttrs",
            "PopMpls",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Replaces the contents of an existing header. A nested attribute\nspecifies a header to modify, its value, and a mask. For every bit set\nin the mask, the corresponding bit value is copied from the value to the\npacket header field, rest of the bits are left unchanged. The non-masked\nvalue bits must be passed in as zeroes. Masking is not supported for the\nOVS_KEY_ATTR_TUNNEL attribute.\n"]
    pub fn get_set_masked(&self) -> Result<IterableKeyAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ActionAttrs::SetMasked(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ActionAttrs",
            "SetMasked",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Track the connection. Populate the conntrack-related entries in the flow\nkey.\n"]
    pub fn get_ct(&self) -> Result<IterableCtAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ActionAttrs::Ct(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ActionAttrs",
            "Ct",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "struct ovs_action_trunc is a u32 max length\n"]
    pub fn get_trunc(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ActionAttrs::Trunc(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ActionAttrs",
            "Trunc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "struct ovs_action_push_eth\n"]
    pub fn get_push_eth(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ActionAttrs::PushEth(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ActionAttrs",
            "PushEth",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pop_eth(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ActionAttrs::PopEth(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ActionAttrs",
            "PopEth",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ct_clear(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ActionAttrs::CtClear(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ActionAttrs",
            "CtClear",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Push NSH header to the packet.\n"]
    pub fn get_push_nsh(&self) -> Result<IterableOvsNshKeyAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ActionAttrs::PushNsh(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ActionAttrs",
            "PushNsh",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Pop the outermost NSH header off the packet.\n"]
    pub fn get_pop_nsh(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ActionAttrs::PopNsh(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ActionAttrs",
            "PopNsh",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Run packet through a meter, which may drop the packet, or modify the\npacket (e.g., change the DSCP field)\n"]
    pub fn get_meter(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ActionAttrs::Meter(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ActionAttrs",
            "Meter",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Make a copy of the packet and execute a list of actions without\naffecting the original packet and key.\n"]
    pub fn get_clone(&self) -> Result<IterableActionAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ActionAttrs::Clone(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ActionAttrs",
            "Clone",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Check the packet length and execute a set of actions if greater than the\nspecified packet length, else execute another set of actions.\n"]
    pub fn get_check_pkt_len(&self) -> Result<IterableCheckPktLenAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ActionAttrs::CheckPktLen(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ActionAttrs",
            "CheckPktLen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Push a new MPLS label stack entry at the start of the packet or at the\nstart of the l3 header depending on the value of l3 tunnel flag in the\ntun_flags field of this OVS_ACTION_ATTR_ADD_MPLS argument.\n"]
    pub fn get_add_mpls(&self) -> Result<OvsActionAddMpls, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ActionAttrs::AddMpls(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ActionAttrs",
            "AddMpls",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dec_ttl(&self) -> Result<IterableDecTtlAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ActionAttrs::DecTtl(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ActionAttrs",
            "DecTtl",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Sends a packet sample to psample for external observation.\n"]
    pub fn get_psample(&self) -> Result<IterablePsampleAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ActionAttrs::Psample(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ActionAttrs",
            "Psample",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ActionAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableActionAttrs<'a> {
        IterableActionAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Output",
            2u16 => "Userspace",
            3u16 => "Set",
            4u16 => "PushVlan",
            5u16 => "PopVlan",
            6u16 => "Sample",
            7u16 => "Recirc",
            8u16 => "Hash",
            9u16 => "PushMpls",
            10u16 => "PopMpls",
            11u16 => "SetMasked",
            12u16 => "Ct",
            13u16 => "Trunc",
            14u16 => "PushEth",
            15u16 => "PopEth",
            16u16 => "CtClear",
            17u16 => "PushNsh",
            18u16 => "PopNsh",
            19u16 => "Meter",
            20u16 => "Clone",
            21u16 => "CheckPktLen",
            22u16 => "AddMpls",
            23u16 => "DecTtl",
            24u16 => "Psample",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableActionAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableActionAttrs<'a> {
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
impl<'a> Iterator for IterableActionAttrs<'a> {
    type Item = Result<ActionAttrs<'a>, ErrorContext>;
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
                1u16 => ActionAttrs::Output({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ActionAttrs::Userspace({
                    let res = Some(IterableUserspaceAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ActionAttrs::Set({
                    let res = Some(IterableKeyAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => ActionAttrs::PushVlan({
                    let res = Some(OvsActionPushVlan::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => ActionAttrs::PopVlan(()),
                6u16 => ActionAttrs::Sample({
                    let res = Some(IterableSampleAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => ActionAttrs::Recirc({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => ActionAttrs::Hash({
                    let res = Some(OvsActionHash::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => ActionAttrs::PushMpls({
                    let res = Some(OvsActionPushMpls::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => ActionAttrs::PopMpls({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => ActionAttrs::SetMasked({
                    let res = Some(IterableKeyAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => ActionAttrs::Ct({
                    let res = Some(IterableCtAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => ActionAttrs::Trunc({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => ActionAttrs::PushEth({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => ActionAttrs::PopEth(()),
                16u16 => ActionAttrs::CtClear(()),
                17u16 => ActionAttrs::PushNsh({
                    let res = Some(IterableOvsNshKeyAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => ActionAttrs::PopNsh(()),
                19u16 => ActionAttrs::Meter({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => ActionAttrs::Clone({
                    let res = Some(IterableActionAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => ActionAttrs::CheckPktLen({
                    let res = Some(IterableCheckPktLenAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => ActionAttrs::AddMpls({
                    let res = Some(OvsActionAddMpls::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => ActionAttrs::DecTtl({
                    let res = Some(IterableDecTtlAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => ActionAttrs::Psample({
                    let res = Some(IterablePsampleAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "ActionAttrs",
            r#type.and_then(|t| ActionAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableActionAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ActionAttrs");
        let mut iter = IterableActionAttrs::with_loc(&[], self.orig_loc);
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
                ActionAttrs::Output(val) => fmt.field("Output", &val),
                ActionAttrs::Userspace(val) => fmt.field("Userspace", &val),
                ActionAttrs::Set(val) => fmt.field("Set", &val),
                ActionAttrs::PushVlan(val) => fmt.field("PushVlan", &val),
                ActionAttrs::PopVlan(val) => fmt.field("PopVlan", &val),
                ActionAttrs::Sample(val) => fmt.field("Sample", &val),
                ActionAttrs::Recirc(val) => fmt.field("Recirc", &val),
                ActionAttrs::Hash(val) => fmt.field("Hash", &val),
                ActionAttrs::PushMpls(val) => fmt.field("PushMpls", &val),
                ActionAttrs::PopMpls(val) => fmt.field("PopMpls", &val),
                ActionAttrs::SetMasked(val) => fmt.field("SetMasked", &val),
                ActionAttrs::Ct(val) => fmt.field("Ct", &val),
                ActionAttrs::Trunc(val) => fmt.field("Trunc", &val),
                ActionAttrs::PushEth(val) => fmt.field("PushEth", &FormatHexdump(val)),
                ActionAttrs::PopEth(val) => fmt.field("PopEth", &val),
                ActionAttrs::CtClear(val) => fmt.field("CtClear", &val),
                ActionAttrs::PushNsh(val) => fmt.field("PushNsh", &val),
                ActionAttrs::PopNsh(val) => fmt.field("PopNsh", &val),
                ActionAttrs::Meter(val) => fmt.field("Meter", &val),
                ActionAttrs::Clone(val) => fmt.field("Clone", &val),
                ActionAttrs::CheckPktLen(val) => fmt.field("CheckPktLen", &val),
                ActionAttrs::AddMpls(val) => fmt.field("AddMpls", &val),
                ActionAttrs::DecTtl(val) => fmt.field("DecTtl", &val),
                ActionAttrs::Psample(val) => fmt.field("Psample", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableActionAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ActionAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ActionAttrs::attr_from_type(t)),
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
                ActionAttrs::Output(val) => {
                    if last_off == offset {
                        stack.push(("Output", last_off));
                        break;
                    }
                }
                ActionAttrs::Userspace(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ActionAttrs::Set(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ActionAttrs::PushVlan(val) => {
                    if last_off == offset {
                        stack.push(("PushVlan", last_off));
                        break;
                    }
                }
                ActionAttrs::PopVlan(val) => {
                    if last_off == offset {
                        stack.push(("PopVlan", last_off));
                        break;
                    }
                }
                ActionAttrs::Sample(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ActionAttrs::Recirc(val) => {
                    if last_off == offset {
                        stack.push(("Recirc", last_off));
                        break;
                    }
                }
                ActionAttrs::Hash(val) => {
                    if last_off == offset {
                        stack.push(("Hash", last_off));
                        break;
                    }
                }
                ActionAttrs::PushMpls(val) => {
                    if last_off == offset {
                        stack.push(("PushMpls", last_off));
                        break;
                    }
                }
                ActionAttrs::PopMpls(val) => {
                    if last_off == offset {
                        stack.push(("PopMpls", last_off));
                        break;
                    }
                }
                ActionAttrs::SetMasked(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ActionAttrs::Ct(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ActionAttrs::Trunc(val) => {
                    if last_off == offset {
                        stack.push(("Trunc", last_off));
                        break;
                    }
                }
                ActionAttrs::PushEth(val) => {
                    if last_off == offset {
                        stack.push(("PushEth", last_off));
                        break;
                    }
                }
                ActionAttrs::PopEth(val) => {
                    if last_off == offset {
                        stack.push(("PopEth", last_off));
                        break;
                    }
                }
                ActionAttrs::CtClear(val) => {
                    if last_off == offset {
                        stack.push(("CtClear", last_off));
                        break;
                    }
                }
                ActionAttrs::PushNsh(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ActionAttrs::PopNsh(val) => {
                    if last_off == offset {
                        stack.push(("PopNsh", last_off));
                        break;
                    }
                }
                ActionAttrs::Meter(val) => {
                    if last_off == offset {
                        stack.push(("Meter", last_off));
                        break;
                    }
                }
                ActionAttrs::Clone(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ActionAttrs::CheckPktLen(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ActionAttrs::AddMpls(val) => {
                    if last_off == offset {
                        stack.push(("AddMpls", last_off));
                        break;
                    }
                }
                ActionAttrs::DecTtl(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                ActionAttrs::Psample(val) => {
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
            stack.push(("ActionAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum TunnelKeyAttrs<'a> {
    Id(u64),
    Ipv4Src(u32),
    Ipv4Dst(u32),
    Tos(u8),
    Ttl(u8),
    DontFragment(()),
    Csum(()),
    Oam(()),
    GeneveOpts(&'a [u8]),
    TpSrc(u16),
    TpDst(u16),
    VxlanOpts(IterableVxlanExtAttrs<'a>),
    #[doc = "struct in6_addr source IPv6 address\n"]
    Ipv6Src(&'a [u8]),
    #[doc = "struct in6_addr destination IPv6 address\n"]
    Ipv6Dst(&'a [u8]),
    Pad(&'a [u8]),
    #[doc = "struct erspan_metadata\n"]
    ErspanOpts(&'a [u8]),
    Ipv4InfoBridge(()),
}
impl<'a> IterableTunnelKeyAttrs<'a> {
    pub fn get_id(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TunnelKeyAttrs::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TunnelKeyAttrs",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ipv4_src(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TunnelKeyAttrs::Ipv4Src(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TunnelKeyAttrs",
            "Ipv4Src",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ipv4_dst(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TunnelKeyAttrs::Ipv4Dst(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TunnelKeyAttrs",
            "Ipv4Dst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tos(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TunnelKeyAttrs::Tos(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TunnelKeyAttrs",
            "Tos",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ttl(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TunnelKeyAttrs::Ttl(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TunnelKeyAttrs",
            "Ttl",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dont_fragment(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TunnelKeyAttrs::DontFragment(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TunnelKeyAttrs",
            "DontFragment",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_csum(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TunnelKeyAttrs::Csum(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TunnelKeyAttrs",
            "Csum",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_oam(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TunnelKeyAttrs::Oam(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TunnelKeyAttrs",
            "Oam",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_geneve_opts(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TunnelKeyAttrs::GeneveOpts(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TunnelKeyAttrs",
            "GeneveOpts",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tp_src(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TunnelKeyAttrs::TpSrc(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TunnelKeyAttrs",
            "TpSrc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tp_dst(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TunnelKeyAttrs::TpDst(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TunnelKeyAttrs",
            "TpDst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vxlan_opts(&self) -> Result<IterableVxlanExtAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TunnelKeyAttrs::VxlanOpts(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TunnelKeyAttrs",
            "VxlanOpts",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "struct in6_addr source IPv6 address\n"]
    pub fn get_ipv6_src(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TunnelKeyAttrs::Ipv6Src(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TunnelKeyAttrs",
            "Ipv6Src",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "struct in6_addr destination IPv6 address\n"]
    pub fn get_ipv6_dst(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TunnelKeyAttrs::Ipv6Dst(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TunnelKeyAttrs",
            "Ipv6Dst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TunnelKeyAttrs::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TunnelKeyAttrs",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "struct erspan_metadata\n"]
    pub fn get_erspan_opts(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TunnelKeyAttrs::ErspanOpts(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TunnelKeyAttrs",
            "ErspanOpts",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ipv4_info_bridge(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TunnelKeyAttrs::Ipv4InfoBridge(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TunnelKeyAttrs",
            "Ipv4InfoBridge",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl TunnelKeyAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableTunnelKeyAttrs<'a> {
        IterableTunnelKeyAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "Id",
            1u16 => "Ipv4Src",
            2u16 => "Ipv4Dst",
            3u16 => "Tos",
            4u16 => "Ttl",
            5u16 => "DontFragment",
            6u16 => "Csum",
            7u16 => "Oam",
            8u16 => "GeneveOpts",
            9u16 => "TpSrc",
            10u16 => "TpDst",
            11u16 => "VxlanOpts",
            12u16 => "Ipv6Src",
            13u16 => "Ipv6Dst",
            14u16 => "Pad",
            15u16 => "ErspanOpts",
            16u16 => "Ipv4InfoBridge",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableTunnelKeyAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableTunnelKeyAttrs<'a> {
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
impl<'a> Iterator for IterableTunnelKeyAttrs<'a> {
    type Item = Result<TunnelKeyAttrs<'a>, ErrorContext>;
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
                0u16 => TunnelKeyAttrs::Id({
                    let res = parse_be_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                1u16 => TunnelKeyAttrs::Ipv4Src({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => TunnelKeyAttrs::Ipv4Dst({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => TunnelKeyAttrs::Tos({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => TunnelKeyAttrs::Ttl({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => TunnelKeyAttrs::DontFragment(()),
                6u16 => TunnelKeyAttrs::Csum(()),
                7u16 => TunnelKeyAttrs::Oam(()),
                8u16 => TunnelKeyAttrs::GeneveOpts({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => TunnelKeyAttrs::TpSrc({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => TunnelKeyAttrs::TpDst({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => TunnelKeyAttrs::VxlanOpts({
                    let res = Some(IterableVxlanExtAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => TunnelKeyAttrs::Ipv6Src({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => TunnelKeyAttrs::Ipv6Dst({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => TunnelKeyAttrs::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => TunnelKeyAttrs::ErspanOpts({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => TunnelKeyAttrs::Ipv4InfoBridge(()),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "TunnelKeyAttrs",
            r#type.and_then(|t| TunnelKeyAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableTunnelKeyAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("TunnelKeyAttrs");
        let mut iter = IterableTunnelKeyAttrs::with_loc(&[], self.orig_loc);
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
                TunnelKeyAttrs::Id(val) => fmt.field("Id", &val),
                TunnelKeyAttrs::Ipv4Src(val) => fmt.field("Ipv4Src", &val),
                TunnelKeyAttrs::Ipv4Dst(val) => fmt.field("Ipv4Dst", &val),
                TunnelKeyAttrs::Tos(val) => fmt.field("Tos", &val),
                TunnelKeyAttrs::Ttl(val) => fmt.field("Ttl", &val),
                TunnelKeyAttrs::DontFragment(val) => fmt.field("DontFragment", &val),
                TunnelKeyAttrs::Csum(val) => fmt.field("Csum", &val),
                TunnelKeyAttrs::Oam(val) => fmt.field("Oam", &val),
                TunnelKeyAttrs::GeneveOpts(val) => fmt.field("GeneveOpts", &FormatHexdump(val)),
                TunnelKeyAttrs::TpSrc(val) => fmt.field("TpSrc", &val),
                TunnelKeyAttrs::TpDst(val) => fmt.field("TpDst", &val),
                TunnelKeyAttrs::VxlanOpts(val) => fmt.field("VxlanOpts", &val),
                TunnelKeyAttrs::Ipv6Src(val) => fmt.field("Ipv6Src", &FormatHexdump(val)),
                TunnelKeyAttrs::Ipv6Dst(val) => fmt.field("Ipv6Dst", &FormatHexdump(val)),
                TunnelKeyAttrs::Pad(val) => fmt.field("Pad", &FormatHexdump(val)),
                TunnelKeyAttrs::ErspanOpts(val) => fmt.field("ErspanOpts", &FormatHexdump(val)),
                TunnelKeyAttrs::Ipv4InfoBridge(val) => fmt.field("Ipv4InfoBridge", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableTunnelKeyAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("TunnelKeyAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| TunnelKeyAttrs::attr_from_type(t)),
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
                TunnelKeyAttrs::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                TunnelKeyAttrs::Ipv4Src(val) => {
                    if last_off == offset {
                        stack.push(("Ipv4Src", last_off));
                        break;
                    }
                }
                TunnelKeyAttrs::Ipv4Dst(val) => {
                    if last_off == offset {
                        stack.push(("Ipv4Dst", last_off));
                        break;
                    }
                }
                TunnelKeyAttrs::Tos(val) => {
                    if last_off == offset {
                        stack.push(("Tos", last_off));
                        break;
                    }
                }
                TunnelKeyAttrs::Ttl(val) => {
                    if last_off == offset {
                        stack.push(("Ttl", last_off));
                        break;
                    }
                }
                TunnelKeyAttrs::DontFragment(val) => {
                    if last_off == offset {
                        stack.push(("DontFragment", last_off));
                        break;
                    }
                }
                TunnelKeyAttrs::Csum(val) => {
                    if last_off == offset {
                        stack.push(("Csum", last_off));
                        break;
                    }
                }
                TunnelKeyAttrs::Oam(val) => {
                    if last_off == offset {
                        stack.push(("Oam", last_off));
                        break;
                    }
                }
                TunnelKeyAttrs::GeneveOpts(val) => {
                    if last_off == offset {
                        stack.push(("GeneveOpts", last_off));
                        break;
                    }
                }
                TunnelKeyAttrs::TpSrc(val) => {
                    if last_off == offset {
                        stack.push(("TpSrc", last_off));
                        break;
                    }
                }
                TunnelKeyAttrs::TpDst(val) => {
                    if last_off == offset {
                        stack.push(("TpDst", last_off));
                        break;
                    }
                }
                TunnelKeyAttrs::VxlanOpts(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                TunnelKeyAttrs::Ipv6Src(val) => {
                    if last_off == offset {
                        stack.push(("Ipv6Src", last_off));
                        break;
                    }
                }
                TunnelKeyAttrs::Ipv6Dst(val) => {
                    if last_off == offset {
                        stack.push(("Ipv6Dst", last_off));
                        break;
                    }
                }
                TunnelKeyAttrs::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                TunnelKeyAttrs::ErspanOpts(val) => {
                    if last_off == offset {
                        stack.push(("ErspanOpts", last_off));
                        break;
                    }
                }
                TunnelKeyAttrs::Ipv4InfoBridge(val) => {
                    if last_off == offset {
                        stack.push(("Ipv4InfoBridge", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("TunnelKeyAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum CheckPktLenAttrs<'a> {
    PktLen(u16),
    ActionsIfGreater(IterableActionAttrs<'a>),
    ActionsIfLessEqual(IterableActionAttrs<'a>),
}
impl<'a> IterableCheckPktLenAttrs<'a> {
    pub fn get_pkt_len(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CheckPktLenAttrs::PktLen(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CheckPktLenAttrs",
            "PktLen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_actions_if_greater(&self) -> Result<IterableActionAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CheckPktLenAttrs::ActionsIfGreater(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CheckPktLenAttrs",
            "ActionsIfGreater",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_actions_if_less_equal(&self) -> Result<IterableActionAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CheckPktLenAttrs::ActionsIfLessEqual(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CheckPktLenAttrs",
            "ActionsIfLessEqual",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl CheckPktLenAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableCheckPktLenAttrs<'a> {
        IterableCheckPktLenAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "PktLen",
            2u16 => "ActionsIfGreater",
            3u16 => "ActionsIfLessEqual",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableCheckPktLenAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableCheckPktLenAttrs<'a> {
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
impl<'a> Iterator for IterableCheckPktLenAttrs<'a> {
    type Item = Result<CheckPktLenAttrs<'a>, ErrorContext>;
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
                1u16 => CheckPktLenAttrs::PktLen({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => CheckPktLenAttrs::ActionsIfGreater({
                    let res = Some(IterableActionAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => CheckPktLenAttrs::ActionsIfLessEqual({
                    let res = Some(IterableActionAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "CheckPktLenAttrs",
            r#type.and_then(|t| CheckPktLenAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableCheckPktLenAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("CheckPktLenAttrs");
        let mut iter = IterableCheckPktLenAttrs::with_loc(&[], self.orig_loc);
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
                CheckPktLenAttrs::PktLen(val) => fmt.field("PktLen", &val),
                CheckPktLenAttrs::ActionsIfGreater(val) => fmt.field("ActionsIfGreater", &val),
                CheckPktLenAttrs::ActionsIfLessEqual(val) => fmt.field("ActionsIfLessEqual", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableCheckPktLenAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("CheckPktLenAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| CheckPktLenAttrs::attr_from_type(t)),
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
                CheckPktLenAttrs::PktLen(val) => {
                    if last_off == offset {
                        stack.push(("PktLen", last_off));
                        break;
                    }
                }
                CheckPktLenAttrs::ActionsIfGreater(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                CheckPktLenAttrs::ActionsIfLessEqual(val) => {
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
            stack.push(("CheckPktLenAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum SampleAttrs<'a> {
    Probability(u32),
    Actions(IterableActionAttrs<'a>),
}
impl<'a> IterableSampleAttrs<'a> {
    pub fn get_probability(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SampleAttrs::Probability(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SampleAttrs",
            "Probability",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_actions(&self) -> Result<IterableActionAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SampleAttrs::Actions(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SampleAttrs",
            "Actions",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl SampleAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableSampleAttrs<'a> {
        IterableSampleAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Probability",
            2u16 => "Actions",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableSampleAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSampleAttrs<'a> {
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
impl<'a> Iterator for IterableSampleAttrs<'a> {
    type Item = Result<SampleAttrs<'a>, ErrorContext>;
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
                1u16 => SampleAttrs::Probability({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => SampleAttrs::Actions({
                    let res = Some(IterableActionAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "SampleAttrs",
            r#type.and_then(|t| SampleAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableSampleAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("SampleAttrs");
        let mut iter = IterableSampleAttrs::with_loc(&[], self.orig_loc);
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
                SampleAttrs::Probability(val) => fmt.field("Probability", &val),
                SampleAttrs::Actions(val) => fmt.field("Actions", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSampleAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("SampleAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| SampleAttrs::attr_from_type(t)),
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
                SampleAttrs::Probability(val) => {
                    if last_off == offset {
                        stack.push(("Probability", last_off));
                        break;
                    }
                }
                SampleAttrs::Actions(val) => {
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
            stack.push(("SampleAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum UserspaceAttrs<'a> {
    Pid(u32),
    Userdata(&'a [u8]),
    EgressTunPort(u32),
    Actions(()),
}
impl<'a> IterableUserspaceAttrs<'a> {
    pub fn get_pid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(UserspaceAttrs::Pid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "UserspaceAttrs",
            "Pid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_userdata(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(UserspaceAttrs::Userdata(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "UserspaceAttrs",
            "Userdata",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_egress_tun_port(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(UserspaceAttrs::EgressTunPort(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "UserspaceAttrs",
            "EgressTunPort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_actions(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(UserspaceAttrs::Actions(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "UserspaceAttrs",
            "Actions",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl UserspaceAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableUserspaceAttrs<'a> {
        IterableUserspaceAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Pid",
            2u16 => "Userdata",
            3u16 => "EgressTunPort",
            4u16 => "Actions",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableUserspaceAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableUserspaceAttrs<'a> {
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
impl<'a> Iterator for IterableUserspaceAttrs<'a> {
    type Item = Result<UserspaceAttrs<'a>, ErrorContext>;
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
                1u16 => UserspaceAttrs::Pid({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => UserspaceAttrs::Userdata({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => UserspaceAttrs::EgressTunPort({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => UserspaceAttrs::Actions(()),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "UserspaceAttrs",
            r#type.and_then(|t| UserspaceAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableUserspaceAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("UserspaceAttrs");
        let mut iter = IterableUserspaceAttrs::with_loc(&[], self.orig_loc);
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
                UserspaceAttrs::Pid(val) => fmt.field("Pid", &val),
                UserspaceAttrs::Userdata(val) => fmt.field("Userdata", &FormatHexdump(val)),
                UserspaceAttrs::EgressTunPort(val) => fmt.field("EgressTunPort", &val),
                UserspaceAttrs::Actions(val) => fmt.field("Actions", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableUserspaceAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("UserspaceAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| UserspaceAttrs::attr_from_type(t)),
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
                UserspaceAttrs::Pid(val) => {
                    if last_off == offset {
                        stack.push(("Pid", last_off));
                        break;
                    }
                }
                UserspaceAttrs::Userdata(val) => {
                    if last_off == offset {
                        stack.push(("Userdata", last_off));
                        break;
                    }
                }
                UserspaceAttrs::EgressTunPort(val) => {
                    if last_off == offset {
                        stack.push(("EgressTunPort", last_off));
                        break;
                    }
                }
                UserspaceAttrs::Actions(val) => {
                    if last_off == offset {
                        stack.push(("Actions", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("UserspaceAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum OvsNshKeyAttrs<'a> {
    Base(&'a [u8]),
    Md1(&'a [u8]),
    Md2(&'a [u8]),
}
impl<'a> IterableOvsNshKeyAttrs<'a> {
    pub fn get_base(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(OvsNshKeyAttrs::Base(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OvsNshKeyAttrs",
            "Base",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_md1(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(OvsNshKeyAttrs::Md1(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OvsNshKeyAttrs",
            "Md1",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_md2(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(OvsNshKeyAttrs::Md2(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OvsNshKeyAttrs",
            "Md2",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OvsNshKeyAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOvsNshKeyAttrs<'a> {
        IterableOvsNshKeyAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Base",
            2u16 => "Md1",
            3u16 => "Md2",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOvsNshKeyAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOvsNshKeyAttrs<'a> {
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
impl<'a> Iterator for IterableOvsNshKeyAttrs<'a> {
    type Item = Result<OvsNshKeyAttrs<'a>, ErrorContext>;
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
                1u16 => OvsNshKeyAttrs::Base({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OvsNshKeyAttrs::Md1({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OvsNshKeyAttrs::Md2({
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
            "OvsNshKeyAttrs",
            r#type.and_then(|t| OvsNshKeyAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOvsNshKeyAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OvsNshKeyAttrs");
        let mut iter = IterableOvsNshKeyAttrs::with_loc(&[], self.orig_loc);
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
                OvsNshKeyAttrs::Base(val) => fmt.field("Base", &FormatHexdump(val)),
                OvsNshKeyAttrs::Md1(val) => fmt.field("Md1", &FormatHexdump(val)),
                OvsNshKeyAttrs::Md2(val) => fmt.field("Md2", &FormatHexdump(val)),
            };
        }
        fmt.finish()
    }
}
impl IterableOvsNshKeyAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("OvsNshKeyAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| OvsNshKeyAttrs::attr_from_type(t)),
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
                OvsNshKeyAttrs::Base(val) => {
                    if last_off == offset {
                        stack.push(("Base", last_off));
                        break;
                    }
                }
                OvsNshKeyAttrs::Md1(val) => {
                    if last_off == offset {
                        stack.push(("Md1", last_off));
                        break;
                    }
                }
                OvsNshKeyAttrs::Md2(val) => {
                    if last_off == offset {
                        stack.push(("Md2", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OvsNshKeyAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum CtAttrs<'a> {
    Commit(()),
    Zone(u16),
    Mark(&'a [u8]),
    Labels(&'a [u8]),
    Helper(&'a CStr),
    Nat(IterableNatAttrs<'a>),
    ForceCommit(()),
    Eventmask(u32),
    Timeout(&'a CStr),
}
impl<'a> IterableCtAttrs<'a> {
    pub fn get_commit(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CtAttrs::Commit(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CtAttrs",
            "Commit",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_zone(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CtAttrs::Zone(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CtAttrs",
            "Zone",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mark(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CtAttrs::Mark(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CtAttrs",
            "Mark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_labels(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CtAttrs::Labels(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CtAttrs",
            "Labels",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_helper(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CtAttrs::Helper(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CtAttrs",
            "Helper",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_nat(&self) -> Result<IterableNatAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CtAttrs::Nat(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CtAttrs",
            "Nat",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_force_commit(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CtAttrs::ForceCommit(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CtAttrs",
            "ForceCommit",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_eventmask(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CtAttrs::Eventmask(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CtAttrs",
            "Eventmask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_timeout(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CtAttrs::Timeout(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CtAttrs",
            "Timeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl CtAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableCtAttrs<'a> {
        IterableCtAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Commit",
            2u16 => "Zone",
            3u16 => "Mark",
            4u16 => "Labels",
            5u16 => "Helper",
            6u16 => "Nat",
            7u16 => "ForceCommit",
            8u16 => "Eventmask",
            9u16 => "Timeout",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableCtAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableCtAttrs<'a> {
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
impl<'a> Iterator for IterableCtAttrs<'a> {
    type Item = Result<CtAttrs<'a>, ErrorContext>;
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
                1u16 => CtAttrs::Commit(()),
                2u16 => CtAttrs::Zone({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => CtAttrs::Mark({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => CtAttrs::Labels({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => CtAttrs::Helper({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => CtAttrs::Nat({
                    let res = Some(IterableNatAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => CtAttrs::ForceCommit(()),
                8u16 => CtAttrs::Eventmask({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => CtAttrs::Timeout({
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
            "CtAttrs",
            r#type.and_then(|t| CtAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableCtAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("CtAttrs");
        let mut iter = IterableCtAttrs::with_loc(&[], self.orig_loc);
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
                CtAttrs::Commit(val) => fmt.field("Commit", &val),
                CtAttrs::Zone(val) => fmt.field("Zone", &val),
                CtAttrs::Mark(val) => fmt.field("Mark", &FormatHexdump(val)),
                CtAttrs::Labels(val) => fmt.field("Labels", &FormatHexdump(val)),
                CtAttrs::Helper(val) => fmt.field("Helper", &val),
                CtAttrs::Nat(val) => fmt.field("Nat", &val),
                CtAttrs::ForceCommit(val) => fmt.field("ForceCommit", &val),
                CtAttrs::Eventmask(val) => fmt.field("Eventmask", &val),
                CtAttrs::Timeout(val) => fmt.field("Timeout", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableCtAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("CtAttrs", offset));
            return (stack, missing_type.and_then(|t| CtAttrs::attr_from_type(t)));
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
                CtAttrs::Commit(val) => {
                    if last_off == offset {
                        stack.push(("Commit", last_off));
                        break;
                    }
                }
                CtAttrs::Zone(val) => {
                    if last_off == offset {
                        stack.push(("Zone", last_off));
                        break;
                    }
                }
                CtAttrs::Mark(val) => {
                    if last_off == offset {
                        stack.push(("Mark", last_off));
                        break;
                    }
                }
                CtAttrs::Labels(val) => {
                    if last_off == offset {
                        stack.push(("Labels", last_off));
                        break;
                    }
                }
                CtAttrs::Helper(val) => {
                    if last_off == offset {
                        stack.push(("Helper", last_off));
                        break;
                    }
                }
                CtAttrs::Nat(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                CtAttrs::ForceCommit(val) => {
                    if last_off == offset {
                        stack.push(("ForceCommit", last_off));
                        break;
                    }
                }
                CtAttrs::Eventmask(val) => {
                    if last_off == offset {
                        stack.push(("Eventmask", last_off));
                        break;
                    }
                }
                CtAttrs::Timeout(val) => {
                    if last_off == offset {
                        stack.push(("Timeout", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("CtAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum NatAttrs<'a> {
    Src(()),
    Dst(()),
    IpMin(&'a [u8]),
    IpMax(&'a [u8]),
    ProtoMin(u16),
    ProtoMax(u16),
    Persistent(()),
    ProtoHash(()),
    ProtoRandom(()),
}
impl<'a> IterableNatAttrs<'a> {
    pub fn get_src(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NatAttrs::Src(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NatAttrs",
            "Src",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dst(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NatAttrs::Dst(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NatAttrs",
            "Dst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ip_min(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NatAttrs::IpMin(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NatAttrs",
            "IpMin",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ip_max(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NatAttrs::IpMax(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NatAttrs",
            "IpMax",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_proto_min(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NatAttrs::ProtoMin(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NatAttrs",
            "ProtoMin",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_proto_max(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NatAttrs::ProtoMax(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NatAttrs",
            "ProtoMax",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_persistent(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NatAttrs::Persistent(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NatAttrs",
            "Persistent",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_proto_hash(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NatAttrs::ProtoHash(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NatAttrs",
            "ProtoHash",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_proto_random(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NatAttrs::ProtoRandom(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NatAttrs",
            "ProtoRandom",
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
            1u16 => "Src",
            2u16 => "Dst",
            3u16 => "IpMin",
            4u16 => "IpMax",
            5u16 => "ProtoMin",
            6u16 => "ProtoMax",
            7u16 => "Persistent",
            8u16 => "ProtoHash",
            9u16 => "ProtoRandom",
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
                1u16 => NatAttrs::Src(()),
                2u16 => NatAttrs::Dst(()),
                3u16 => NatAttrs::IpMin({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => NatAttrs::IpMax({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => NatAttrs::ProtoMin({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => NatAttrs::ProtoMax({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => NatAttrs::Persistent(()),
                8u16 => NatAttrs::ProtoHash(()),
                9u16 => NatAttrs::ProtoRandom(()),
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
                NatAttrs::Src(val) => fmt.field("Src", &val),
                NatAttrs::Dst(val) => fmt.field("Dst", &val),
                NatAttrs::IpMin(val) => fmt.field("IpMin", &FormatHexdump(val)),
                NatAttrs::IpMax(val) => fmt.field("IpMax", &FormatHexdump(val)),
                NatAttrs::ProtoMin(val) => fmt.field("ProtoMin", &val),
                NatAttrs::ProtoMax(val) => fmt.field("ProtoMax", &val),
                NatAttrs::Persistent(val) => fmt.field("Persistent", &val),
                NatAttrs::ProtoHash(val) => fmt.field("ProtoHash", &val),
                NatAttrs::ProtoRandom(val) => fmt.field("ProtoRandom", &val),
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
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                NatAttrs::Src(val) => {
                    if last_off == offset {
                        stack.push(("Src", last_off));
                        break;
                    }
                }
                NatAttrs::Dst(val) => {
                    if last_off == offset {
                        stack.push(("Dst", last_off));
                        break;
                    }
                }
                NatAttrs::IpMin(val) => {
                    if last_off == offset {
                        stack.push(("IpMin", last_off));
                        break;
                    }
                }
                NatAttrs::IpMax(val) => {
                    if last_off == offset {
                        stack.push(("IpMax", last_off));
                        break;
                    }
                }
                NatAttrs::ProtoMin(val) => {
                    if last_off == offset {
                        stack.push(("ProtoMin", last_off));
                        break;
                    }
                }
                NatAttrs::ProtoMax(val) => {
                    if last_off == offset {
                        stack.push(("ProtoMax", last_off));
                        break;
                    }
                }
                NatAttrs::Persistent(val) => {
                    if last_off == offset {
                        stack.push(("Persistent", last_off));
                        break;
                    }
                }
                NatAttrs::ProtoHash(val) => {
                    if last_off == offset {
                        stack.push(("ProtoHash", last_off));
                        break;
                    }
                }
                NatAttrs::ProtoRandom(val) => {
                    if last_off == offset {
                        stack.push(("ProtoRandom", last_off));
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
        (stack, None)
    }
}
#[derive(Clone)]
pub enum DecTtlAttrs<'a> {
    Action(IterableActionAttrs<'a>),
}
impl<'a> IterableDecTtlAttrs<'a> {
    pub fn get_action(&self) -> Result<IterableActionAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(DecTtlAttrs::Action(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "DecTtlAttrs",
            "Action",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl DecTtlAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDecTtlAttrs<'a> {
        IterableDecTtlAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Action",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDecTtlAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDecTtlAttrs<'a> {
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
impl<'a> Iterator for IterableDecTtlAttrs<'a> {
    type Item = Result<DecTtlAttrs<'a>, ErrorContext>;
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
                1u16 => DecTtlAttrs::Action({
                    let res = Some(IterableActionAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "DecTtlAttrs",
            r#type.and_then(|t| DecTtlAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDecTtlAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("DecTtlAttrs");
        let mut iter = IterableDecTtlAttrs::with_loc(&[], self.orig_loc);
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
                DecTtlAttrs::Action(val) => fmt.field("Action", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDecTtlAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("DecTtlAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| DecTtlAttrs::attr_from_type(t)),
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
                DecTtlAttrs::Action(val) => {
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
            stack.push(("DecTtlAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum VxlanExtAttrs {
    Gbp(u32),
}
impl<'a> IterableVxlanExtAttrs<'a> {
    pub fn get_gbp(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VxlanExtAttrs::Gbp(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VxlanExtAttrs",
            "Gbp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl VxlanExtAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableVxlanExtAttrs<'a> {
        IterableVxlanExtAttrs::with_loc(buf, buf.as_ptr() as usize)
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
pub struct IterableVxlanExtAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableVxlanExtAttrs<'a> {
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
impl<'a> Iterator for IterableVxlanExtAttrs<'a> {
    type Item = Result<VxlanExtAttrs, ErrorContext>;
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
                1u16 => VxlanExtAttrs::Gbp({
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
            "VxlanExtAttrs",
            r#type.and_then(|t| VxlanExtAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableVxlanExtAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("VxlanExtAttrs");
        let mut iter = IterableVxlanExtAttrs::with_loc(&[], self.orig_loc);
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
                VxlanExtAttrs::Gbp(val) => fmt.field("Gbp", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableVxlanExtAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("VxlanExtAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| VxlanExtAttrs::attr_from_type(t)),
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
                VxlanExtAttrs::Gbp(val) => {
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
            stack.push(("VxlanExtAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum PsampleAttrs<'a> {
    Group(u32),
    Cookie(&'a [u8]),
}
impl<'a> IterablePsampleAttrs<'a> {
    pub fn get_group(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PsampleAttrs::Group(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PsampleAttrs",
            "Group",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_cookie(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PsampleAttrs::Cookie(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PsampleAttrs",
            "Cookie",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl PsampleAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePsampleAttrs<'a> {
        IterablePsampleAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Group",
            2u16 => "Cookie",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePsampleAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePsampleAttrs<'a> {
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
impl<'a> Iterator for IterablePsampleAttrs<'a> {
    type Item = Result<PsampleAttrs<'a>, ErrorContext>;
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
                1u16 => PsampleAttrs::Group({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => PsampleAttrs::Cookie({
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
            "PsampleAttrs",
            r#type.and_then(|t| PsampleAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterablePsampleAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("PsampleAttrs");
        let mut iter = IterablePsampleAttrs::with_loc(&[], self.orig_loc);
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
                PsampleAttrs::Group(val) => fmt.field("Group", &val),
                PsampleAttrs::Cookie(val) => fmt.field("Cookie", &FormatHexdump(val)),
            };
        }
        fmt.finish()
    }
}
impl IterablePsampleAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("PsampleAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| PsampleAttrs::attr_from_type(t)),
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
                PsampleAttrs::Group(val) => {
                    if last_off == offset {
                        stack.push(("Group", last_off));
                        break;
                    }
                }
                PsampleAttrs::Cookie(val) => {
                    if last_off == offset {
                        stack.push(("Cookie", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("PsampleAttrs", cur));
        }
        (stack, None)
    }
}
pub struct PushFlowAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushFlowAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushFlowAttrs<Prev> {
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
    #[doc = "Nested attributes specifying the flow key. Always present in\nnotifications. Required for all requests (except dumps).\n"]
    pub fn nested_key(mut self) -> PushKeyAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushKeyAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Nested attributes specifying the actions to take for packets that match\nthe key. Always present in notifications. Required for OVS_FLOW_CMD_NEW\nrequests, optional for OVS_FLOW_CMD_SET requests. An OVS_FLOW_CMD_SET\nwithout OVS_FLOW_ATTR_ACTIONS will not modify the actions. To clear the\nactions, an OVS_FLOW_ATTR_ACTIONS without any nested attributes must be\ngiven.\n"]
    pub fn nested_actions(mut self) -> PushActionAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushActionAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Statistics for this flow. Present in notifications if the stats would be\nnonzero. Ignored in requests.\n"]
    pub fn push_stats(mut self, value: OvsFlowStats) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    #[doc = "An 8-bit value giving the ORed value of all of the TCP flags seen on\npackets in this flow. Only present in notifications for TCP flows, and\nonly if it would be nonzero. Ignored in requests.\n"]
    pub fn push_tcp_flags(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 4u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "A 64-bit integer giving the time, in milliseconds on the system\nmonotonic clock, at which a packet was last processed for this flow.\nOnly present in notifications if a packet has been processed for this\nflow. Ignored in requests.\n"]
    pub fn push_used(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 5u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "If present in a OVS_FLOW_CMD_SET request, clears the last-used time,\naccumulated TCP flags, and statistics for this flow. Otherwise ignored\nin requests. Never present in notifications.\n"]
    pub fn push_clear(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 6u16, 0 as u16);
        self
    }
    #[doc = "Nested attributes specifying the mask bits for wildcarded flow match.\nMask bit value \\'1\\' specifies exact match with corresponding flow key\nbit, while mask bit value \\'0\\' specifies a wildcarded match. Omitting\nattribute is treated as wildcarding all corresponding fields. Optional\nfor all requests. If not present, all flow key bits are exact match\nbits.\n"]
    pub fn nested_mask(mut self) -> PushKeyAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 7u16);
        PushKeyAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Flow operation is a feature probe, error logging should be suppressed.\n"]
    pub fn push_probe(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 8u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "A value between 1-16 octets specifying a unique identifier for the flow.\nCauses the flow to be indexed by this value rather than the value of the\nOVS_FLOW_ATTR_KEY attribute. Optional for all requests. Present in\nnotifications if the flow was created with this attribute.\n"]
    pub fn push_ufid(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 9u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "A 32-bit value of ORed flags that provide alternative semantics for flow\ninstallation and retrieval. Optional for all requests.\n\nAssociated type: [`OvsUfidFlags`] (enum)"]
    pub fn push_ufid_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 11u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushFlowAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushKeyAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushKeyAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushKeyAttrs<Prev> {
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
    pub fn nested_encap(mut self) -> PushKeyAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushKeyAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_priority(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_in_port(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "struct ovs_key_ethernet\n"]
    pub fn push_ethernet(mut self, value: OvsKeyEthernet) -> Self {
        push_header(self.as_vec_mut(), 4u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_vlan(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 5u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_ethertype(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 6u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_ipv4(mut self, value: OvsKeyIpv4) -> Self {
        push_header(self.as_vec_mut(), 7u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    #[doc = "struct ovs_key_ipv6\n"]
    pub fn push_ipv6(mut self, value: OvsKeyIpv6) -> Self {
        push_header(self.as_vec_mut(), 8u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_tcp(mut self, value: OvsKeyTcp) -> Self {
        push_header(self.as_vec_mut(), 9u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_udp(mut self, value: OvsKeyUdp) -> Self {
        push_header(self.as_vec_mut(), 10u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_icmp(mut self, value: OvsKeyIcmp) -> Self {
        push_header(self.as_vec_mut(), 11u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_icmpv6(mut self, value: OvsKeyIcmp) -> Self {
        push_header(self.as_vec_mut(), 12u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    #[doc = "struct ovs_key_arp\n"]
    pub fn push_arp(mut self, value: OvsKeyArp) -> Self {
        push_header(self.as_vec_mut(), 13u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    #[doc = "struct ovs_key_nd\n"]
    pub fn push_nd(mut self, value: OvsKeyNd) -> Self {
        push_header(self.as_vec_mut(), 14u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_skb_mark(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 15u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_tunnel(mut self) -> PushTunnelKeyAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 16u16);
        PushTunnelKeyAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_sctp(mut self, value: OvsKeySctp) -> Self {
        push_header(self.as_vec_mut(), 17u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_tcp_flags(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 18u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "Value 0 indicates the hash is not computed by the datapath.\n"]
    pub fn push_dp_hash(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 19u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_recirc_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 20u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mpls(mut self, value: OvsKeyMpls) -> Self {
        push_header(self.as_vec_mut(), 21u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    #[doc = "Associated type: [`CtStateFlags`] (1 bit per enumeration)"]
    pub fn push_ct_state(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 22u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "connection tracking zone\n"]
    pub fn push_ct_zone(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 23u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "connection tracking mark\n"]
    pub fn push_ct_mark(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 24u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "16-octet connection tracking label\n"]
    pub fn push_ct_labels(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 25u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_ct_orig_tuple_ipv4(mut self, value: OvsKeyCtTupleIpv4) -> Self {
        push_header(self.as_vec_mut(), 26u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    #[doc = "struct ovs_key_ct_tuple_ipv6\n"]
    pub fn push_ct_orig_tuple_ipv6(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 27u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn nested_nsh(mut self) -> PushOvsNshKeyAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 28u16);
        PushOvsNshKeyAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Should not be sent to the kernel\n"]
    pub fn push_packet_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 29u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "Should not be sent to the kernel\n"]
    pub fn push_nd_extensions(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 30u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "struct ip_tunnel_info\n"]
    pub fn push_tunnel_info(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 31u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "struct ovs_key_ipv6_exthdr\n"]
    pub fn push_ipv6_exthdrs(mut self, value: OvsKeyIpv6Exthdrs) -> Self {
        push_header(self.as_vec_mut(), 32u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
}
impl<Prev: Pusher> Drop for PushKeyAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushActionAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushActionAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushActionAttrs<Prev> {
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
    #[doc = "ovs port number in datapath\n"]
    pub fn push_output(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_userspace(mut self) -> PushUserspaceAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushUserspaceAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Replaces the contents of an existing header. The single nested attribute\nspecifies a header to modify and its value.\n"]
    pub fn nested_set(mut self) -> PushKeyAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 3u16);
        PushKeyAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Push a new outermost 802.1Q or 802.1ad header onto the packet.\n"]
    pub fn push_push_vlan(mut self, value: OvsActionPushVlan) -> Self {
        push_header(self.as_vec_mut(), 4u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    #[doc = "Pop the outermost 802.1Q or 802.1ad header from the packet.\n"]
    pub fn push_pop_vlan(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 5u16, 0 as u16);
        self
    }
    #[doc = "Probabilistically executes actions, as specified in the nested\nattributes.\n"]
    pub fn nested_sample(mut self) -> PushSampleAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 6u16);
        PushSampleAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "recirc id\n"]
    pub fn push_recirc(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_hash(mut self, value: OvsActionHash) -> Self {
        push_header(self.as_vec_mut(), 8u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    #[doc = "Push a new MPLS label stack entry onto the top of the packets MPLS label\nstack. Set the ethertype of the encapsulating frame to either\nETH_P_MPLS_UC or ETH_P_MPLS_MC to indicate the new packet contents.\n"]
    pub fn push_push_mpls(mut self, value: OvsActionPushMpls) -> Self {
        push_header(self.as_vec_mut(), 9u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    #[doc = "ethertype\n"]
    pub fn push_pop_mpls(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 10u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "Replaces the contents of an existing header. A nested attribute\nspecifies a header to modify, its value, and a mask. For every bit set\nin the mask, the corresponding bit value is copied from the value to the\npacket header field, rest of the bits are left unchanged. The non-masked\nvalue bits must be passed in as zeroes. Masking is not supported for the\nOVS_KEY_ATTR_TUNNEL attribute.\n"]
    pub fn nested_set_masked(mut self) -> PushKeyAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 11u16);
        PushKeyAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Track the connection. Populate the conntrack-related entries in the flow\nkey.\n"]
    pub fn nested_ct(mut self) -> PushCtAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 12u16);
        PushCtAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "struct ovs_action_trunc is a u32 max length\n"]
    pub fn push_trunc(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 13u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "struct ovs_action_push_eth\n"]
    pub fn push_push_eth(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 14u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_pop_eth(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 15u16, 0 as u16);
        self
    }
    pub fn push_ct_clear(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 16u16, 0 as u16);
        self
    }
    #[doc = "Push NSH header to the packet.\n"]
    pub fn nested_push_nsh(mut self) -> PushOvsNshKeyAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 17u16);
        PushOvsNshKeyAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Pop the outermost NSH header off the packet.\n"]
    pub fn push_pop_nsh(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 18u16, 0 as u16);
        self
    }
    #[doc = "Run packet through a meter, which may drop the packet, or modify the\npacket (e.g., change the DSCP field)\n"]
    pub fn push_meter(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 19u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Make a copy of the packet and execute a list of actions without\naffecting the original packet and key.\n"]
    pub fn nested_clone(mut self) -> PushActionAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 20u16);
        PushActionAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Check the packet length and execute a set of actions if greater than the\nspecified packet length, else execute another set of actions.\n"]
    pub fn nested_check_pkt_len(mut self) -> PushCheckPktLenAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 21u16);
        PushCheckPktLenAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Push a new MPLS label stack entry at the start of the packet or at the\nstart of the l3 header depending on the value of l3 tunnel flag in the\ntun_flags field of this OVS_ACTION_ATTR_ADD_MPLS argument.\n"]
    pub fn push_add_mpls(mut self, value: OvsActionAddMpls) -> Self {
        push_header(self.as_vec_mut(), 22u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn nested_dec_ttl(mut self) -> PushDecTtlAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 23u16);
        PushDecTtlAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Sends a packet sample to psample for external observation.\n"]
    pub fn nested_psample(mut self) -> PushPsampleAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 24u16);
        PushPsampleAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushActionAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushTunnelKeyAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushTunnelKeyAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushTunnelKeyAttrs<Prev> {
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
        push_header(self.as_vec_mut(), 0u16, 8 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_ipv4_src(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_ipv4_dst(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_tos(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 3u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ttl(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 4u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dont_fragment(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 5u16, 0 as u16);
        self
    }
    pub fn push_csum(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 6u16, 0 as u16);
        self
    }
    pub fn push_oam(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 7u16, 0 as u16);
        self
    }
    pub fn push_geneve_opts(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 8u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_tp_src(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 9u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_tp_dst(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 10u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn nested_vxlan_opts(mut self) -> PushVxlanExtAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 11u16);
        PushVxlanExtAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "struct in6_addr source IPv6 address\n"]
    pub fn push_ipv6_src(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 12u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "struct in6_addr destination IPv6 address\n"]
    pub fn push_ipv6_dst(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 13u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 14u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "struct erspan_metadata\n"]
    pub fn push_erspan_opts(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 15u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_ipv4_info_bridge(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 16u16, 0 as u16);
        self
    }
}
impl<Prev: Pusher> Drop for PushTunnelKeyAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushCheckPktLenAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushCheckPktLenAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushCheckPktLenAttrs<Prev> {
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
    pub fn push_pkt_len(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 1u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_actions_if_greater(mut self) -> PushActionAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushActionAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_actions_if_less_equal(mut self) -> PushActionAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 3u16);
        PushActionAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushCheckPktLenAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSampleAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushSampleAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushSampleAttrs<Prev> {
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
    pub fn push_probability(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_actions(mut self) -> PushActionAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushActionAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushSampleAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushUserspaceAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushUserspaceAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushUserspaceAttrs<Prev> {
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
    pub fn push_pid(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_userdata(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_egress_tun_port(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_actions(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 4u16, 0 as u16);
        self
    }
}
impl<Prev: Pusher> Drop for PushUserspaceAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushOvsNshKeyAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushOvsNshKeyAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushOvsNshKeyAttrs<Prev> {
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
    pub fn push_base(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_md1(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_md2(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushOvsNshKeyAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushCtAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushCtAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushCtAttrs<Prev> {
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
    pub fn push_commit(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 1u16, 0 as u16);
        self
    }
    pub fn push_zone(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 2u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mark(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_labels(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 4u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_helper(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            5u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_helper_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 5u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn nested_nat(mut self) -> PushNatAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 6u16);
        PushNatAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_force_commit(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 7u16, 0 as u16);
        self
    }
    pub fn push_eventmask(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_timeout(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            9u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_timeout_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 9u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
}
impl<Prev: Pusher> Drop for PushCtAttrs<Prev> {
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
    pub fn push_src(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 1u16, 0 as u16);
        self
    }
    pub fn push_dst(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 2u16, 0 as u16);
        self
    }
    pub fn push_ip_min(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_ip_max(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 4u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_proto_min(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 5u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_proto_max(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 6u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_persistent(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 7u16, 0 as u16);
        self
    }
    pub fn push_proto_hash(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 8u16, 0 as u16);
        self
    }
    pub fn push_proto_random(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 9u16, 0 as u16);
        self
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
pub struct PushDecTtlAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDecTtlAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDecTtlAttrs<Prev> {
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
    pub fn nested_action(mut self) -> PushActionAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushActionAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushDecTtlAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushVxlanExtAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushVxlanExtAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushVxlanExtAttrs<Prev> {
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
impl<Prev: Pusher> Drop for PushVxlanExtAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPsampleAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushPsampleAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushPsampleAttrs<Prev> {
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
    pub fn push_group(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_cookie(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushPsampleAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct NotifGroup;
impl NotifGroup {
    pub const OVS_FLOW: &str = "ovs_flow";
    pub const OVS_FLOW_CSTR: &CStr = c"ovs_flow";
}
#[doc = "Get / dump OVS flow configuration and state\n\nRequest attributes:\n- [.nested_key()](PushFlowAttrs::nested_key)\n- [.push_ufid()](PushFlowAttrs::push_ufid)\n- [.push_ufid_flags()](PushFlowAttrs::push_ufid_flags)\n\nReply attributes:\n- [.get_key()](IterableFlowAttrs::get_key)\n- [.get_actions()](IterableFlowAttrs::get_actions)\n- [.get_stats()](IterableFlowAttrs::get_stats)\n- [.get_mask()](IterableFlowAttrs::get_mask)\n- [.get_ufid()](IterableFlowAttrs::get_ufid)\n\n"]
#[derive(Debug)]
pub struct OpGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetDump<'r> {
    pub fn new(mut request: Request<'r>, header: &OvsHeader) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &OvsHeader,
    ) -> PushFlowAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushFlowAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushFlowAttrs<&mut Vec<u8>> {
        PushFlowAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushFlowAttrs<RequestBuf<'r>> {
        PushFlowAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (OvsHeader, IterableFlowAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(OvsHeader::len()));
        (
            OvsHeader::new_from_slice(header).unwrap_or_default(),
            IterableFlowAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &OvsHeader) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &OvsHeader {
        let pos = self.request.pos;
        OvsHeader::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut OvsHeader {
        let pos = self.request.pos;
        OvsHeader::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("ovs_flow".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (OvsHeader, IterableFlowAttrs<'buf>);
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
#[doc = "Get / dump OVS flow configuration and state\n\nRequest attributes:\n- [.nested_key()](PushFlowAttrs::nested_key)\n- [.push_ufid()](PushFlowAttrs::push_ufid)\n- [.push_ufid_flags()](PushFlowAttrs::push_ufid_flags)\n\nReply attributes:\n- [.get_key()](IterableFlowAttrs::get_key)\n- [.get_actions()](IterableFlowAttrs::get_actions)\n- [.get_stats()](IterableFlowAttrs::get_stats)\n- [.get_mask()](IterableFlowAttrs::get_mask)\n- [.get_ufid()](IterableFlowAttrs::get_ufid)\n\n"]
#[derive(Debug)]
pub struct OpGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetDo<'r> {
    pub fn new(mut request: Request<'r>, header: &OvsHeader) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &OvsHeader,
    ) -> PushFlowAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushFlowAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushFlowAttrs<&mut Vec<u8>> {
        PushFlowAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushFlowAttrs<RequestBuf<'r>> {
        PushFlowAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (OvsHeader, IterableFlowAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(OvsHeader::len()));
        (
            OvsHeader::new_from_slice(header).unwrap_or_default(),
            IterableFlowAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &OvsHeader) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &OvsHeader {
        let pos = self.request.pos;
        OvsHeader::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut OvsHeader {
        let pos = self.request.pos;
        OvsHeader::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("ovs_flow".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (OvsHeader, IterableFlowAttrs<'buf>);
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
#[doc = "Create OVS flow configuration in a data path\n\nRequest attributes:\n- [.nested_key()](PushFlowAttrs::nested_key)\n- [.nested_actions()](PushFlowAttrs::nested_actions)\n- [.nested_mask()](PushFlowAttrs::nested_mask)\n- [.push_ufid()](PushFlowAttrs::push_ufid)\n\n"]
#[derive(Debug)]
pub struct OpNewDo<'r> {
    request: Request<'r>,
}
impl<'r> OpNewDo<'r> {
    pub fn new(mut request: Request<'r>, header: &OvsHeader) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &OvsHeader,
    ) -> PushFlowAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushFlowAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushFlowAttrs<&mut Vec<u8>> {
        PushFlowAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushFlowAttrs<RequestBuf<'r>> {
        PushFlowAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (OvsHeader, IterableFlowAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(OvsHeader::len()));
        (
            OvsHeader::new_from_slice(header).unwrap_or_default(),
            IterableFlowAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &OvsHeader) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &OvsHeader {
        let pos = self.request.pos;
        OvsHeader::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut OvsHeader {
        let pos = self.request.pos;
        OvsHeader::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpNewDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("ovs_flow".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (OvsHeader, IterableFlowAttrs<'buf>);
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
    #[doc = "Get / dump OVS flow configuration and state\n\nRequest attributes:\n- [.nested_key()](PushFlowAttrs::nested_key)\n- [.push_ufid()](PushFlowAttrs::push_ufid)\n- [.push_ufid_flags()](PushFlowAttrs::push_ufid_flags)\n\nReply attributes:\n- [.get_key()](IterableFlowAttrs::get_key)\n- [.get_actions()](IterableFlowAttrs::get_actions)\n- [.get_stats()](IterableFlowAttrs::get_stats)\n- [.get_mask()](IterableFlowAttrs::get_mask)\n- [.get_ufid()](IterableFlowAttrs::get_ufid)\n\n"]
    pub fn op_get_dump(self, header: &OvsHeader) -> OpGetDump<'buf> {
        let mut res = OpGetDump::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-get-dump", OpGetDump::lookup);
        res
    }
    #[doc = "Get / dump OVS flow configuration and state\n\nRequest attributes:\n- [.nested_key()](PushFlowAttrs::nested_key)\n- [.push_ufid()](PushFlowAttrs::push_ufid)\n- [.push_ufid_flags()](PushFlowAttrs::push_ufid_flags)\n\nReply attributes:\n- [.get_key()](IterableFlowAttrs::get_key)\n- [.get_actions()](IterableFlowAttrs::get_actions)\n- [.get_stats()](IterableFlowAttrs::get_stats)\n- [.get_mask()](IterableFlowAttrs::get_mask)\n- [.get_ufid()](IterableFlowAttrs::get_ufid)\n\n"]
    pub fn op_get_do(self, header: &OvsHeader) -> OpGetDo<'buf> {
        let mut res = OpGetDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-get-do", OpGetDo::lookup);
        res
    }
    #[doc = "Create OVS flow configuration in a data path\n\nRequest attributes:\n- [.nested_key()](PushFlowAttrs::nested_key)\n- [.nested_actions()](PushFlowAttrs::nested_actions)\n- [.nested_mask()](PushFlowAttrs::nested_mask)\n- [.push_ufid()](PushFlowAttrs::push_ufid)\n\n"]
    pub fn op_new_do(self, header: &OvsHeader) -> OpNewDo<'buf> {
        let mut res = OpNewDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-new-do", OpNewDo::lookup);
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableFlowAttrs::get_actions;
        let _ = IterableFlowAttrs::get_key;
        let _ = IterableFlowAttrs::get_mask;
        let _ = IterableFlowAttrs::get_stats;
        let _ = IterableFlowAttrs::get_ufid;
        let _ = PushFlowAttrs::<&mut Vec<u8>>::nested_actions;
        let _ = PushFlowAttrs::<&mut Vec<u8>>::nested_key;
        let _ = PushFlowAttrs::<&mut Vec<u8>>::nested_mask;
        let _ = PushFlowAttrs::<&mut Vec<u8>>::push_ufid;
        let _ = PushFlowAttrs::<&mut Vec<u8>>::push_ufid_flags;
    }
}
