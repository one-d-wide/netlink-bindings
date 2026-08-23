#![doc = "Address configuration over rtnetlink.\n"]
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
pub const PROTONAME: &str = "rt-addr";
pub const PROTONAME_CSTR: &CStr = c"rt-addr";
pub const PROTONUM: u16 = 0u16;
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum IfaFlags {
    Secondary = 1 << 0,
    Nodad = 1 << 1,
    Optimistic = 1 << 2,
    Dadfailed = 1 << 3,
    Homeaddress = 1 << 4,
    Deprecated = 1 << 5,
    Tentative = 1 << 6,
    Permanent = 1 << 7,
    Managetempaddr = 1 << 8,
    Noprefixroute = 1 << 9,
    Mcautojoin = 1 << 10,
    StablePrivacy = 1 << 11,
}
impl IfaFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::Secondary,
            n if n == 1 << 1 => Self::Nodad,
            n if n == 1 << 2 => Self::Optimistic,
            n if n == 1 << 3 => Self::Dadfailed,
            n if n == 1 << 4 => Self::Homeaddress,
            n if n == 1 << 5 => Self::Deprecated,
            n if n == 1 << 6 => Self::Tentative,
            n if n == 1 << 7 => Self::Permanent,
            n if n == 1 << 8 => Self::Managetempaddr,
            n if n == 1 << 9 => Self::Noprefixroute,
            n if n == 1 << 10 => Self::Mcautojoin,
            n if n == 1 << 11 => Self::StablePrivacy,
            _ => return None,
        })
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Ifaddrmsg {
    pub ifa_family: u8,
    pub ifa_prefixlen: u8,
    #[doc = "Associated type: [`IfaFlags`] (1 bit per enumeration)"]
    pub ifa_flags: u8,
    pub ifa_scope: u8,
    pub ifa_index: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for Ifaddrmsg {
    fn default() -> Self {
        Self::new()
    }
}
impl Ifaddrmsg {
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
        const _: () = assert!(std::mem::size_of::<Ifaddrmsg>() == 8usize);
        const _: () = assert!(std::mem::align_of::<Ifaddrmsg>() == 4usize);
        8usize
    }
}
impl std::fmt::Debug for Ifaddrmsg {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Ifaddrmsg")
            .field("ifa_family", &self.ifa_family)
            .field("ifa_prefixlen", &self.ifa_prefixlen)
            .field(
                "ifa_flags",
                &FormatFlags(self.ifa_flags.into(), IfaFlags::from_value),
            )
            .field("ifa_scope", &self.ifa_scope)
            .field("ifa_index", &self.ifa_index)
            .finish()
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct IfaCacheinfo {
    pub ifa_prefered: u32,
    pub ifa_valid: u32,
    pub cstamp: u32,
    pub tstamp: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for IfaCacheinfo {
    fn default() -> Self {
        Self::new()
    }
}
impl IfaCacheinfo {
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
        const _: () = assert!(std::mem::size_of::<IfaCacheinfo>() == 16usize);
        const _: () = assert!(std::mem::align_of::<IfaCacheinfo>() == 4usize);
        16usize
    }
}
#[derive(Clone)]
pub enum AddrAttrs<'a> {
    Address(std::net::IpAddr),
    Local(std::net::IpAddr),
    Label(&'a CStr),
    Broadcast(std::net::Ipv4Addr),
    Anycast(std::net::IpAddr),
    Cacheinfo(IfaCacheinfo),
    Multicast(std::net::IpAddr),
    #[doc = "Associated type: [`IfaFlags`] (1 bit per enumeration)"]
    Flags(u32),
    RtPriority(u32),
    TargetNetnsid(&'a [u8]),
    Proto(u8),
}
impl<'a> IterableAddrAttrs<'a> {
    pub fn get_address(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AddrAttrs::Address(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AddrAttrs",
            "Address",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_local(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AddrAttrs::Local(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AddrAttrs",
            "Local",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_label(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AddrAttrs::Label(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AddrAttrs",
            "Label",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_broadcast(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AddrAttrs::Broadcast(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AddrAttrs",
            "Broadcast",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_anycast(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AddrAttrs::Anycast(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AddrAttrs",
            "Anycast",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_cacheinfo(&self) -> Result<IfaCacheinfo, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AddrAttrs::Cacheinfo(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AddrAttrs",
            "Cacheinfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_multicast(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AddrAttrs::Multicast(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AddrAttrs",
            "Multicast",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`IfaFlags`] (1 bit per enumeration)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AddrAttrs::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AddrAttrs",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rt_priority(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AddrAttrs::RtPriority(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AddrAttrs",
            "RtPriority",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_target_netnsid(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AddrAttrs::TargetNetnsid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AddrAttrs",
            "TargetNetnsid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_proto(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AddrAttrs::Proto(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AddrAttrs",
            "Proto",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl AddrAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableAddrAttrs<'a> {
        IterableAddrAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Address",
            2u16 => "Local",
            3u16 => "Label",
            4u16 => "Broadcast",
            5u16 => "Anycast",
            6u16 => "Cacheinfo",
            7u16 => "Multicast",
            8u16 => "Flags",
            9u16 => "RtPriority",
            10u16 => "TargetNetnsid",
            11u16 => "Proto",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableAddrAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableAddrAttrs<'a> {
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
impl<'a> Iterator for IterableAddrAttrs<'a> {
    type Item = Result<AddrAttrs<'a>, ErrorContext>;
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
                1u16 => AddrAttrs::Address({
                    let res = parse_ip(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => AddrAttrs::Local({
                    let res = parse_ip(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => AddrAttrs::Label({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => AddrAttrs::Broadcast({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => AddrAttrs::Anycast({
                    let res = parse_ip(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => AddrAttrs::Cacheinfo({
                    let res = Some(IfaCacheinfo::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => AddrAttrs::Multicast({
                    let res = parse_ip(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => AddrAttrs::Flags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => AddrAttrs::RtPriority({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => AddrAttrs::TargetNetnsid({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => AddrAttrs::Proto({
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
            "AddrAttrs",
            r#type.and_then(|t| AddrAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableAddrAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("AddrAttrs");
        let mut iter = IterableAddrAttrs::with_loc(&[], self.orig_loc);
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
                AddrAttrs::Address(val) => fmt.field("Address", &val),
                AddrAttrs::Local(val) => fmt.field("Local", &val),
                AddrAttrs::Label(val) => fmt.field("Label", &val),
                AddrAttrs::Broadcast(val) => fmt.field("Broadcast", &val),
                AddrAttrs::Anycast(val) => fmt.field("Anycast", &val),
                AddrAttrs::Cacheinfo(val) => fmt.field("Cacheinfo", &val),
                AddrAttrs::Multicast(val) => fmt.field("Multicast", &val),
                AddrAttrs::Flags(val) => {
                    fmt.field("Flags", &FormatFlags(val.into(), IfaFlags::from_value))
                }
                AddrAttrs::RtPriority(val) => fmt.field("RtPriority", &val),
                AddrAttrs::TargetNetnsid(val) => fmt.field("TargetNetnsid", &FormatHexdump(val)),
                AddrAttrs::Proto(val) => fmt.field("Proto", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableAddrAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("AddrAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| AddrAttrs::attr_from_type(t)),
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
                AddrAttrs::Address(val) => {
                    if last_off == offset {
                        stack.push(("Address", last_off));
                        break;
                    }
                }
                AddrAttrs::Local(val) => {
                    if last_off == offset {
                        stack.push(("Local", last_off));
                        break;
                    }
                }
                AddrAttrs::Label(val) => {
                    if last_off == offset {
                        stack.push(("Label", last_off));
                        break;
                    }
                }
                AddrAttrs::Broadcast(val) => {
                    if last_off == offset {
                        stack.push(("Broadcast", last_off));
                        break;
                    }
                }
                AddrAttrs::Anycast(val) => {
                    if last_off == offset {
                        stack.push(("Anycast", last_off));
                        break;
                    }
                }
                AddrAttrs::Cacheinfo(val) => {
                    if last_off == offset {
                        stack.push(("Cacheinfo", last_off));
                        break;
                    }
                }
                AddrAttrs::Multicast(val) => {
                    if last_off == offset {
                        stack.push(("Multicast", last_off));
                        break;
                    }
                }
                AddrAttrs::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                AddrAttrs::RtPriority(val) => {
                    if last_off == offset {
                        stack.push(("RtPriority", last_off));
                        break;
                    }
                }
                AddrAttrs::TargetNetnsid(val) => {
                    if last_off == offset {
                        stack.push(("TargetNetnsid", last_off));
                        break;
                    }
                }
                AddrAttrs::Proto(val) => {
                    if last_off == offset {
                        stack.push(("Proto", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("AddrAttrs", cur));
        }
        (stack, None)
    }
}
pub struct PushAddrAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushAddrAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushAddrAttrs<Prev> {
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
    pub fn push_address(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_vec_mut(), 1u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_vec_mut(), value);
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
    pub fn push_label(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            3u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_label_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_broadcast(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    pub fn push_anycast(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_vec_mut(), 5u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_vec_mut(), value);
        self
    }
    pub fn push_cacheinfo(mut self, value: IfaCacheinfo) -> Self {
        push_header(self.as_vec_mut(), 6u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_multicast(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_vec_mut(), 7u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_vec_mut(), value);
        self
    }
    #[doc = "Associated type: [`IfaFlags`] (1 bit per enumeration)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_rt_priority(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_target_netnsid(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 10u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_proto(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 11u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushAddrAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Add new address\n\nRequest attributes:\n- [.push_address()](PushAddrAttrs::push_address)\n- [.push_local()](PushAddrAttrs::push_local)\n- [.push_label()](PushAddrAttrs::push_label)\n- [.push_cacheinfo()](PushAddrAttrs::push_cacheinfo)\n\n"]
#[derive(Debug)]
pub struct OpNewaddrDo<'r> {
    request: Request<'r>,
}
impl<'r> OpNewaddrDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Ifaddrmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Ifaddrmsg,
    ) -> PushAddrAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushAddrAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushAddrAttrs<&mut Vec<u8>> {
        PushAddrAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushAddrAttrs<RequestBuf<'r>> {
        PushAddrAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Ifaddrmsg, IterableAddrAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Ifaddrmsg::len()));
        (
            Ifaddrmsg::new_from_slice(header).unwrap_or_default(),
            IterableAddrAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Ifaddrmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Ifaddrmsg {
        let pos = self.request.pos;
        Ifaddrmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Ifaddrmsg {
        let pos = self.request.pos;
        Ifaddrmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpNewaddrDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 20u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Ifaddrmsg, IterableAddrAttrs<'buf>);
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
#[doc = "Remove address\n\nRequest attributes:\n- [.push_address()](PushAddrAttrs::push_address)\n- [.push_local()](PushAddrAttrs::push_local)\n\n"]
#[derive(Debug)]
pub struct OpDeladdrDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDeladdrDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Ifaddrmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Ifaddrmsg,
    ) -> PushAddrAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushAddrAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushAddrAttrs<&mut Vec<u8>> {
        PushAddrAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushAddrAttrs<RequestBuf<'r>> {
        PushAddrAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Ifaddrmsg, IterableAddrAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Ifaddrmsg::len()));
        (
            Ifaddrmsg::new_from_slice(header).unwrap_or_default(),
            IterableAddrAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Ifaddrmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Ifaddrmsg {
        let pos = self.request.pos;
        Ifaddrmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Ifaddrmsg {
        let pos = self.request.pos;
        Ifaddrmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpDeladdrDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 21u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Ifaddrmsg, IterableAddrAttrs<'buf>);
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
#[doc = "Dump address information.\n\nReply attributes:\n- [.get_address()](IterableAddrAttrs::get_address)\n- [.get_local()](IterableAddrAttrs::get_local)\n- [.get_label()](IterableAddrAttrs::get_label)\n- [.get_cacheinfo()](IterableAddrAttrs::get_cacheinfo)\n\n"]
#[derive(Debug)]
pub struct OpGetaddrDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetaddrDump<'r> {
    pub fn new(mut request: Request<'r>, header: &Ifaddrmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Ifaddrmsg,
    ) -> PushAddrAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushAddrAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushAddrAttrs<&mut Vec<u8>> {
        PushAddrAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushAddrAttrs<RequestBuf<'r>> {
        PushAddrAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Ifaddrmsg, IterableAddrAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Ifaddrmsg::len()));
        (
            Ifaddrmsg::new_from_slice(header).unwrap_or_default(),
            IterableAddrAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Ifaddrmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Ifaddrmsg {
        let pos = self.request.pos;
        Ifaddrmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Ifaddrmsg {
        let pos = self.request.pos;
        Ifaddrmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetaddrDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 22u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Ifaddrmsg, IterableAddrAttrs<'buf>);
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
#[doc = "Get / dump IPv4/IPv6 multicast addresses.\n\nReply attributes:\n- [.get_cacheinfo()](IterableAddrAttrs::get_cacheinfo)\n- [.get_multicast()](IterableAddrAttrs::get_multicast)\n\n"]
#[derive(Debug)]
pub struct OpGetmulticastDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetmulticastDump<'r> {
    pub fn new(mut request: Request<'r>, header: &Ifaddrmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Ifaddrmsg,
    ) -> PushAddrAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushAddrAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushAddrAttrs<&mut Vec<u8>> {
        PushAddrAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushAddrAttrs<RequestBuf<'r>> {
        PushAddrAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Ifaddrmsg, IterableAddrAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Ifaddrmsg::len()));
        (
            Ifaddrmsg::new_from_slice(header).unwrap_or_default(),
            IterableAddrAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Ifaddrmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Ifaddrmsg {
        let pos = self.request.pos;
        Ifaddrmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Ifaddrmsg {
        let pos = self.request.pos;
        Ifaddrmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetmulticastDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 58u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Ifaddrmsg, IterableAddrAttrs<'buf>);
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
#[doc = "Get / dump IPv4/IPv6 multicast addresses.\n\nReply attributes:\n- [.get_cacheinfo()](IterableAddrAttrs::get_cacheinfo)\n- [.get_multicast()](IterableAddrAttrs::get_multicast)\n\n"]
#[derive(Debug)]
pub struct OpGetmulticastDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetmulticastDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Ifaddrmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Ifaddrmsg,
    ) -> PushAddrAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushAddrAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushAddrAttrs<&mut Vec<u8>> {
        PushAddrAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushAddrAttrs<RequestBuf<'r>> {
        PushAddrAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Ifaddrmsg, IterableAddrAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Ifaddrmsg::len()));
        (
            Ifaddrmsg::new_from_slice(header).unwrap_or_default(),
            IterableAddrAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Ifaddrmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Ifaddrmsg {
        let pos = self.request.pos;
        Ifaddrmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Ifaddrmsg {
        let pos = self.request.pos;
        Ifaddrmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetmulticastDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 58u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Ifaddrmsg, IterableAddrAttrs<'buf>);
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
    #[doc = "Add new address\n\nRequest attributes:\n- [.push_address()](PushAddrAttrs::push_address)\n- [.push_local()](PushAddrAttrs::push_local)\n- [.push_label()](PushAddrAttrs::push_label)\n- [.push_cacheinfo()](PushAddrAttrs::push_cacheinfo)\n\n"]
    pub fn op_newaddr_do(self, header: &Ifaddrmsg) -> OpNewaddrDo<'buf> {
        let mut res = OpNewaddrDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-newaddr-do", OpNewaddrDo::lookup);
        res
    }
    #[doc = "Remove address\n\nRequest attributes:\n- [.push_address()](PushAddrAttrs::push_address)\n- [.push_local()](PushAddrAttrs::push_local)\n\n"]
    pub fn op_deladdr_do(self, header: &Ifaddrmsg) -> OpDeladdrDo<'buf> {
        let mut res = OpDeladdrDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-deladdr-do", OpDeladdrDo::lookup);
        res
    }
    #[doc = "Dump address information.\n\nReply attributes:\n- [.get_address()](IterableAddrAttrs::get_address)\n- [.get_local()](IterableAddrAttrs::get_local)\n- [.get_label()](IterableAddrAttrs::get_label)\n- [.get_cacheinfo()](IterableAddrAttrs::get_cacheinfo)\n\n"]
    pub fn op_getaddr_dump(self, header: &Ifaddrmsg) -> OpGetaddrDump<'buf> {
        let mut res = OpGetaddrDump::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getaddr-dump", OpGetaddrDump::lookup);
        res
    }
    #[doc = "Get / dump IPv4/IPv6 multicast addresses.\n\nReply attributes:\n- [.get_cacheinfo()](IterableAddrAttrs::get_cacheinfo)\n- [.get_multicast()](IterableAddrAttrs::get_multicast)\n\n"]
    pub fn op_getmulticast_dump(self, header: &Ifaddrmsg) -> OpGetmulticastDump<'buf> {
        let mut res = OpGetmulticastDump::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-getmulticast-dump",
            OpGetmulticastDump::lookup,
        );
        res
    }
    #[doc = "Get / dump IPv4/IPv6 multicast addresses.\n\nReply attributes:\n- [.get_cacheinfo()](IterableAddrAttrs::get_cacheinfo)\n- [.get_multicast()](IterableAddrAttrs::get_multicast)\n\n"]
    pub fn op_getmulticast_do(self, header: &Ifaddrmsg) -> OpGetmulticastDo<'buf> {
        let mut res = OpGetmulticastDo::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-getmulticast-do",
            OpGetmulticastDo::lookup,
        );
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableAddrAttrs::get_address;
        let _ = IterableAddrAttrs::get_cacheinfo;
        let _ = IterableAddrAttrs::get_label;
        let _ = IterableAddrAttrs::get_local;
        let _ = IterableAddrAttrs::get_multicast;
        let _ = PushAddrAttrs::<&mut Vec<u8>>::push_address;
        let _ = PushAddrAttrs::<&mut Vec<u8>>::push_cacheinfo;
        let _ = PushAddrAttrs::<&mut Vec<u8>>::push_label;
        let _ = PushAddrAttrs::<&mut Vec<u8>>::push_local;
    }
}
