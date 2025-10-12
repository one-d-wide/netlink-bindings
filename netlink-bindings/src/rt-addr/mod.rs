#![doc = "Address configuration over rtnetlink."]
#![allow(clippy::all)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]
#![allow(unreachable_code)]
#![allow(unreachable_patterns)]
use crate::builtin::{PushBuiltinBitfield32, PushBuiltinNfgenmsg, PushDummy, PushNlmsghdr};
use crate::{
    consts,
    traits::{NetlinkRequest, Protocol},
    utils::*,
};
pub const PROTONAME: &CStr = c"rt-addr";
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
#[derive(Clone)]
pub enum AddrAttrs<'a> {
    Address(std::net::IpAddr),
    Local(std::net::IpAddr),
    Label(&'a CStr),
    Broadcast(std::net::IpAddr),
    Anycast(&'a [u8]),
    Cacheinfo(PushIfaCacheinfo),
    Multicast(&'a [u8]),
    #[doc = "Associated type: \"IfaFlags\" (1 bit per enumeration)"]
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
            if let AddrAttrs::Address(val) = attr? {
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
            if let AddrAttrs::Local(val) = attr? {
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
            if let AddrAttrs::Label(val) = attr? {
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
    pub fn get_broadcast(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let AddrAttrs::Broadcast(val) = attr? {
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
    pub fn get_anycast(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let AddrAttrs::Anycast(val) = attr? {
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
    pub fn get_cacheinfo(&self) -> Result<PushIfaCacheinfo, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let AddrAttrs::Cacheinfo(val) = attr? {
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
    pub fn get_multicast(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let AddrAttrs::Multicast(val) = attr? {
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
    #[doc = "Associated type: \"IfaFlags\" (1 bit per enumeration)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let AddrAttrs::Flags(val) = attr? {
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
            if let AddrAttrs::RtPriority(val) = attr? {
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
            if let AddrAttrs::TargetNetnsid(val) = attr? {
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
            if let AddrAttrs::Proto(val) = attr? {
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
impl<'a> AddrAttrs<'a> {
    pub fn new(buf: &'a [u8]) -> IterableAddrAttrs<'a> {
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
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
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
                    let res = parse_ip(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => AddrAttrs::Anycast({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => AddrAttrs::Cacheinfo({
                    let res = PushIfaCacheinfo::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => AddrAttrs::Multicast({
                    let res = Some(next);
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
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
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
        for attr in self.clone() {
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
                AddrAttrs::TargetNetnsid(val) => fmt.field("TargetNetnsid", &val),
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
        if cur == offset {
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
pub struct PushAddrAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushAddrAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushAddrAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_address(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_rec_mut(), 1u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_rec_mut(), value);
        self
    }
    pub fn push_local(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_rec_mut(), 2u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_rec_mut(), value);
        self
    }
    pub fn push_label(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            3u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_label_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 3u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_broadcast(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_rec_mut(), 4u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_rec_mut(), value);
        self
    }
    pub fn push_anycast(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 5u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_cacheinfo(mut self, value: PushIfaCacheinfo) -> Self {
        push_header(self.as_rec_mut(), 6u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_multicast(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 7u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    #[doc = "Associated type: \"IfaFlags\" (1 bit per enumeration)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_rt_priority(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 9u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_target_netnsid(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 10u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_proto(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 11u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushAddrAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[derive(Clone)]
pub struct PushIfaddrmsg {
    pub(crate) buf: [u8; 8usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for PushIfaddrmsg {
    fn default() -> Self {
        Self { buf: [0u8; 8usize] }
    }
}
impl PushIfaddrmsg {
    #[doc = "Create zero-initialized struct"]
    pub fn new() -> Self {
        Default::default()
    }
    #[doc = "Copy from contents from other slice"]
    pub fn new_from_slice(other: &[u8]) -> Option<Self> {
        if other.len() != Self::len() {
            return None;
        }
        let mut buf = [0u8; Self::len()];
        buf.clone_from_slice(other);
        Some(Self { buf })
    }
    pub fn as_slice(&self) -> &[u8] {
        &self.buf
    }
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.buf
    }
    pub const fn len() -> usize {
        8usize
    }
    pub fn ifa_family(&self) -> u8 {
        parse_u8(&self.buf[0usize..1usize]).unwrap()
    }
    pub fn set_ifa_family(&mut self, value: u8) {
        self.buf[0usize..1usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn ifa_prefixlen(&self) -> u8 {
        parse_u8(&self.buf[1usize..2usize]).unwrap()
    }
    pub fn set_ifa_prefixlen(&mut self, value: u8) {
        self.buf[1usize..2usize].copy_from_slice(&value.to_ne_bytes())
    }
    #[doc = "Associated type: \"IfaFlags\" (1 bit per enumeration)"]
    pub fn ifa_flags(&self) -> u8 {
        parse_u8(&self.buf[2usize..3usize]).unwrap()
    }
    #[doc = "Associated type: \"IfaFlags\" (1 bit per enumeration)"]
    pub fn set_ifa_flags(&mut self, value: u8) {
        self.buf[2usize..3usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn ifa_scope(&self) -> u8 {
        parse_u8(&self.buf[3usize..4usize]).unwrap()
    }
    pub fn set_ifa_scope(&mut self, value: u8) {
        self.buf[3usize..4usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn ifa_index(&self) -> u32 {
        parse_u32(&self.buf[4usize..8usize]).unwrap()
    }
    pub fn set_ifa_index(&mut self, value: u32) {
        self.buf[4usize..8usize].copy_from_slice(&value.to_ne_bytes())
    }
}
impl std::fmt::Debug for PushIfaddrmsg {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Ifaddrmsg")
            .field("ifa_family", &self.ifa_family())
            .field("ifa_prefixlen", &self.ifa_prefixlen())
            .field("ifa_flags", &self.ifa_flags())
            .field("ifa_scope", &self.ifa_scope())
            .field("ifa_index", &self.ifa_index())
            .finish()
    }
}
#[derive(Clone)]
pub struct PushIfaCacheinfo {
    pub(crate) buf: [u8; 16usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for PushIfaCacheinfo {
    fn default() -> Self {
        Self {
            buf: [0u8; 16usize],
        }
    }
}
impl PushIfaCacheinfo {
    #[doc = "Create zero-initialized struct"]
    pub fn new() -> Self {
        Default::default()
    }
    #[doc = "Copy from contents from other slice"]
    pub fn new_from_slice(other: &[u8]) -> Option<Self> {
        if other.len() != Self::len() {
            return None;
        }
        let mut buf = [0u8; Self::len()];
        buf.clone_from_slice(other);
        Some(Self { buf })
    }
    pub fn as_slice(&self) -> &[u8] {
        &self.buf
    }
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.buf
    }
    pub const fn len() -> usize {
        16usize
    }
    pub fn ifa_prefered(&self) -> u32 {
        parse_u32(&self.buf[0usize..4usize]).unwrap()
    }
    pub fn set_ifa_prefered(&mut self, value: u32) {
        self.buf[0usize..4usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn ifa_valid(&self) -> u32 {
        parse_u32(&self.buf[4usize..8usize]).unwrap()
    }
    pub fn set_ifa_valid(&mut self, value: u32) {
        self.buf[4usize..8usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn cstamp(&self) -> u32 {
        parse_u32(&self.buf[8usize..12usize]).unwrap()
    }
    pub fn set_cstamp(&mut self, value: u32) {
        self.buf[8usize..12usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn tstamp(&self) -> u32 {
        parse_u32(&self.buf[12usize..16usize]).unwrap()
    }
    pub fn set_tstamp(&mut self, value: u32) {
        self.buf[12usize..16usize].copy_from_slice(&value.to_ne_bytes())
    }
}
impl std::fmt::Debug for PushIfaCacheinfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("IfaCacheinfo")
            .field("ifa_prefered", &self.ifa_prefered())
            .field("ifa_valid", &self.ifa_valid())
            .field("cstamp", &self.cstamp())
            .field("tstamp", &self.tstamp())
            .finish()
    }
}
#[doc = "Add new address"]
pub struct PushOpNewaddrDoRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpNewaddrDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpNewaddrDoRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushIfaddrmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushIfaddrmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_address(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_rec_mut(), 1u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_rec_mut(), value);
        self
    }
    pub fn push_local(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_rec_mut(), 2u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_rec_mut(), value);
        self
    }
    pub fn push_label(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            3u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_label_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 3u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_cacheinfo(mut self, value: PushIfaCacheinfo) -> Self {
        push_header(self.as_rec_mut(), 6u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    #[doc = "Associated type: \"IfaFlags\" (1 bit per enumeration)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpNewaddrDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Add new address"]
#[derive(Clone)]
pub enum OpNewaddrDoRequest<'a> {
    Address(std::net::IpAddr),
    Local(std::net::IpAddr),
    Label(&'a CStr),
    Cacheinfo(PushIfaCacheinfo),
    #[doc = "Associated type: \"IfaFlags\" (1 bit per enumeration)"]
    Flags(u32),
}
impl<'a> IterableOpNewaddrDoRequest<'a> {
    pub fn get_address(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewaddrDoRequest::Address(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewaddrDoRequest",
            "Address",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_local(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewaddrDoRequest::Local(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewaddrDoRequest",
            "Local",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_label(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewaddrDoRequest::Label(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewaddrDoRequest",
            "Label",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_cacheinfo(&self) -> Result<PushIfaCacheinfo, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewaddrDoRequest::Cacheinfo(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewaddrDoRequest",
            "Cacheinfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: \"IfaFlags\" (1 bit per enumeration)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewaddrDoRequest::Flags(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNewaddrDoRequest",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> OpNewaddrDoRequest<'a> {
    pub fn new(buf: &'a [u8]) -> (PushIfaddrmsg, IterableOpNewaddrDoRequest<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushIfaddrmsg::len()));
        (
            PushIfaddrmsg::new_from_slice(header).unwrap_or_default(),
            IterableOpNewaddrDoRequest::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        AddrAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpNewaddrDoRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpNewaddrDoRequest<'a> {
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
impl<'a> Iterator for IterableOpNewaddrDoRequest<'a> {
    type Item = Result<OpNewaddrDoRequest<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpNewaddrDoRequest::Address({
                    let res = parse_ip(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpNewaddrDoRequest::Local({
                    let res = parse_ip(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpNewaddrDoRequest::Label({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpNewaddrDoRequest::Cacheinfo({
                    let res = PushIfaCacheinfo::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpNewaddrDoRequest::Flags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpNewaddrDoRequest",
            r#type.and_then(|t| OpNewaddrDoRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpNewaddrDoRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpNewaddrDoRequest");
        for attr in self.clone() {
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
                OpNewaddrDoRequest::Address(val) => fmt.field("Address", &val),
                OpNewaddrDoRequest::Local(val) => fmt.field("Local", &val),
                OpNewaddrDoRequest::Label(val) => fmt.field("Label", &val),
                OpNewaddrDoRequest::Cacheinfo(val) => fmt.field("Cacheinfo", &val),
                OpNewaddrDoRequest::Flags(val) => {
                    fmt.field("Flags", &FormatFlags(val.into(), IfaFlags::from_value))
                }
            };
        }
        fmt.finish()
    }
}
impl IterableOpNewaddrDoRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushIfaddrmsg::len() {
            stack.push(("OpNewaddrDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpNewaddrDoRequest::attr_from_type(t)),
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
                OpNewaddrDoRequest::Address(val) => {
                    if last_off == offset {
                        stack.push(("Address", last_off));
                        break;
                    }
                }
                OpNewaddrDoRequest::Local(val) => {
                    if last_off == offset {
                        stack.push(("Local", last_off));
                        break;
                    }
                }
                OpNewaddrDoRequest::Label(val) => {
                    if last_off == offset {
                        stack.push(("Label", last_off));
                        break;
                    }
                }
                OpNewaddrDoRequest::Cacheinfo(val) => {
                    if last_off == offset {
                        stack.push(("Cacheinfo", last_off));
                        break;
                    }
                }
                OpNewaddrDoRequest::Flags(val) => {
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
            stack.push(("OpNewaddrDoRequest", cur));
        }
        (stack, None)
    }
}
#[doc = "Add new address"]
pub struct PushOpNewaddrDoReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpNewaddrDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpNewaddrDoReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushIfaddrmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushIfaddrmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
}
impl<Prev: Rec> Drop for PushOpNewaddrDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Add new address"]
#[derive(Clone)]
pub enum OpNewaddrDoReply {}
impl<'a> IterableOpNewaddrDoReply<'a> {}
impl OpNewaddrDoReply {
    pub fn new(buf: &'_ [u8]) -> (PushIfaddrmsg, IterableOpNewaddrDoReply<'_>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushIfaddrmsg::len()));
        (
            PushIfaddrmsg::new_from_slice(header).unwrap_or_default(),
            IterableOpNewaddrDoReply::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        AddrAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpNewaddrDoReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpNewaddrDoReply<'a> {
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
impl<'a> Iterator for IterableOpNewaddrDoReply<'a> {
    type Item = Result<OpNewaddrDoReply, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpNewaddrDoReply",
            r#type.and_then(|t| OpNewaddrDoReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpNewaddrDoReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpNewaddrDoReply");
        for attr in self.clone() {
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
impl IterableOpNewaddrDoReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushIfaddrmsg::len() {
            stack.push(("OpNewaddrDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpNewaddrDoReply::attr_from_type(t)),
            );
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpNewaddrDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpNewaddrDoRequest<'r> {
    pub fn new(mut request: Request<'r>, header: &PushIfaddrmsg) -> Self {
        PushOpNewaddrDoRequest::write_header(&mut request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpNewaddrDoRequest<&mut Vec<u8>> {
        PushOpNewaddrDoRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpNewaddrDoRequest<RequestBuf<'r>> {
        PushOpNewaddrDoRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpNewaddrDoRequest<'_> {
    type ReplyType<'buf> = (PushIfaddrmsg, IterableOpNewaddrDoReply<'buf>);
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
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpNewaddrDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpNewaddrDoRequest::new(buf)
            .1
            .lookup_attr(offset, missing_type)
    }
}
#[doc = "Remove address"]
pub struct PushOpDeladdrDoRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpDeladdrDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpDeladdrDoRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushIfaddrmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushIfaddrmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_address(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_rec_mut(), 1u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_rec_mut(), value);
        self
    }
    pub fn push_local(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_rec_mut(), 2u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_rec_mut(), value);
        self
    }
}
impl<Prev: Rec> Drop for PushOpDeladdrDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Remove address"]
#[derive(Clone)]
pub enum OpDeladdrDoRequest {
    Address(std::net::IpAddr),
    Local(std::net::IpAddr),
}
impl<'a> IterableOpDeladdrDoRequest<'a> {
    pub fn get_address(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDeladdrDoRequest::Address(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDeladdrDoRequest",
            "Address",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_local(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDeladdrDoRequest::Local(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDeladdrDoRequest",
            "Local",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpDeladdrDoRequest {
    pub fn new(buf: &'_ [u8]) -> (PushIfaddrmsg, IterableOpDeladdrDoRequest<'_>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushIfaddrmsg::len()));
        (
            PushIfaddrmsg::new_from_slice(header).unwrap_or_default(),
            IterableOpDeladdrDoRequest::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        AddrAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpDeladdrDoRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpDeladdrDoRequest<'a> {
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
impl<'a> Iterator for IterableOpDeladdrDoRequest<'a> {
    type Item = Result<OpDeladdrDoRequest, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpDeladdrDoRequest::Address({
                    let res = parse_ip(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpDeladdrDoRequest::Local({
                    let res = parse_ip(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpDeladdrDoRequest",
            r#type.and_then(|t| OpDeladdrDoRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpDeladdrDoRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpDeladdrDoRequest");
        for attr in self.clone() {
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
                OpDeladdrDoRequest::Address(val) => fmt.field("Address", &val),
                OpDeladdrDoRequest::Local(val) => fmt.field("Local", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpDeladdrDoRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushIfaddrmsg::len() {
            stack.push(("OpDeladdrDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpDeladdrDoRequest::attr_from_type(t)),
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
                OpDeladdrDoRequest::Address(val) => {
                    if last_off == offset {
                        stack.push(("Address", last_off));
                        break;
                    }
                }
                OpDeladdrDoRequest::Local(val) => {
                    if last_off == offset {
                        stack.push(("Local", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpDeladdrDoRequest", cur));
        }
        (stack, None)
    }
}
#[doc = "Remove address"]
pub struct PushOpDeladdrDoReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpDeladdrDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpDeladdrDoReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushIfaddrmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushIfaddrmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
}
impl<Prev: Rec> Drop for PushOpDeladdrDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Remove address"]
#[derive(Clone)]
pub enum OpDeladdrDoReply {}
impl<'a> IterableOpDeladdrDoReply<'a> {}
impl OpDeladdrDoReply {
    pub fn new(buf: &'_ [u8]) -> (PushIfaddrmsg, IterableOpDeladdrDoReply<'_>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushIfaddrmsg::len()));
        (
            PushIfaddrmsg::new_from_slice(header).unwrap_or_default(),
            IterableOpDeladdrDoReply::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        AddrAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpDeladdrDoReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpDeladdrDoReply<'a> {
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
impl<'a> Iterator for IterableOpDeladdrDoReply<'a> {
    type Item = Result<OpDeladdrDoReply, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpDeladdrDoReply",
            r#type.and_then(|t| OpDeladdrDoReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpDeladdrDoReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpDeladdrDoReply");
        for attr in self.clone() {
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
impl IterableOpDeladdrDoReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushIfaddrmsg::len() {
            stack.push(("OpDeladdrDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpDeladdrDoReply::attr_from_type(t)),
            );
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpDeladdrDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpDeladdrDoRequest<'r> {
    pub fn new(mut request: Request<'r>, header: &PushIfaddrmsg) -> Self {
        PushOpDeladdrDoRequest::write_header(&mut request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpDeladdrDoRequest<&mut Vec<u8>> {
        PushOpDeladdrDoRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpDeladdrDoRequest<RequestBuf<'r>> {
        PushOpDeladdrDoRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpDeladdrDoRequest<'_> {
    type ReplyType<'buf> = (PushIfaddrmsg, IterableOpDeladdrDoReply<'buf>);
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
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpDeladdrDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpDeladdrDoRequest::new(buf)
            .1
            .lookup_attr(offset, missing_type)
    }
}
#[doc = "Dump address information."]
pub struct PushOpGetaddrDumpRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetaddrDumpRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetaddrDumpRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushIfaddrmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushIfaddrmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
}
impl<Prev: Rec> Drop for PushOpGetaddrDumpRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Dump address information."]
#[derive(Clone)]
pub enum OpGetaddrDumpRequest {}
impl<'a> IterableOpGetaddrDumpRequest<'a> {}
impl OpGetaddrDumpRequest {
    pub fn new(buf: &'_ [u8]) -> (PushIfaddrmsg, IterableOpGetaddrDumpRequest<'_>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushIfaddrmsg::len()));
        (
            PushIfaddrmsg::new_from_slice(header).unwrap_or_default(),
            IterableOpGetaddrDumpRequest::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        AddrAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetaddrDumpRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetaddrDumpRequest<'a> {
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
impl<'a> Iterator for IterableOpGetaddrDumpRequest<'a> {
    type Item = Result<OpGetaddrDumpRequest, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpGetaddrDumpRequest",
            r#type.and_then(|t| OpGetaddrDumpRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpGetaddrDumpRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetaddrDumpRequest");
        for attr in self.clone() {
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
impl IterableOpGetaddrDumpRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushIfaddrmsg::len() {
            stack.push(("OpGetaddrDumpRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetaddrDumpRequest::attr_from_type(t)),
            );
        }
        (stack, None)
    }
}
#[doc = "Dump address information."]
pub struct PushOpGetaddrDumpReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetaddrDumpReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetaddrDumpReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushIfaddrmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushIfaddrmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_address(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_rec_mut(), 1u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_rec_mut(), value);
        self
    }
    pub fn push_local(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_rec_mut(), 2u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_rec_mut(), value);
        self
    }
    pub fn push_label(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            3u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_label_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 3u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_cacheinfo(mut self, value: PushIfaCacheinfo) -> Self {
        push_header(self.as_rec_mut(), 6u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    #[doc = "Associated type: \"IfaFlags\" (1 bit per enumeration)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpGetaddrDumpReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Dump address information."]
#[derive(Clone)]
pub enum OpGetaddrDumpReply<'a> {
    Address(std::net::IpAddr),
    Local(std::net::IpAddr),
    Label(&'a CStr),
    Cacheinfo(PushIfaCacheinfo),
    #[doc = "Associated type: \"IfaFlags\" (1 bit per enumeration)"]
    Flags(u32),
}
impl<'a> IterableOpGetaddrDumpReply<'a> {
    pub fn get_address(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetaddrDumpReply::Address(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetaddrDumpReply",
            "Address",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_local(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetaddrDumpReply::Local(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetaddrDumpReply",
            "Local",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_label(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetaddrDumpReply::Label(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetaddrDumpReply",
            "Label",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_cacheinfo(&self) -> Result<PushIfaCacheinfo, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetaddrDumpReply::Cacheinfo(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetaddrDumpReply",
            "Cacheinfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: \"IfaFlags\" (1 bit per enumeration)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetaddrDumpReply::Flags(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetaddrDumpReply",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> OpGetaddrDumpReply<'a> {
    pub fn new(buf: &'a [u8]) -> (PushIfaddrmsg, IterableOpGetaddrDumpReply<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushIfaddrmsg::len()));
        (
            PushIfaddrmsg::new_from_slice(header).unwrap_or_default(),
            IterableOpGetaddrDumpReply::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        AddrAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetaddrDumpReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetaddrDumpReply<'a> {
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
impl<'a> Iterator for IterableOpGetaddrDumpReply<'a> {
    type Item = Result<OpGetaddrDumpReply<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpGetaddrDumpReply::Address({
                    let res = parse_ip(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpGetaddrDumpReply::Local({
                    let res = parse_ip(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpGetaddrDumpReply::Label({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpGetaddrDumpReply::Cacheinfo({
                    let res = PushIfaCacheinfo::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpGetaddrDumpReply::Flags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpGetaddrDumpReply",
            r#type.and_then(|t| OpGetaddrDumpReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpGetaddrDumpReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetaddrDumpReply");
        for attr in self.clone() {
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
                OpGetaddrDumpReply::Address(val) => fmt.field("Address", &val),
                OpGetaddrDumpReply::Local(val) => fmt.field("Local", &val),
                OpGetaddrDumpReply::Label(val) => fmt.field("Label", &val),
                OpGetaddrDumpReply::Cacheinfo(val) => fmt.field("Cacheinfo", &val),
                OpGetaddrDumpReply::Flags(val) => {
                    fmt.field("Flags", &FormatFlags(val.into(), IfaFlags::from_value))
                }
            };
        }
        fmt.finish()
    }
}
impl IterableOpGetaddrDumpReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushIfaddrmsg::len() {
            stack.push(("OpGetaddrDumpReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetaddrDumpReply::attr_from_type(t)),
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
                OpGetaddrDumpReply::Address(val) => {
                    if last_off == offset {
                        stack.push(("Address", last_off));
                        break;
                    }
                }
                OpGetaddrDumpReply::Local(val) => {
                    if last_off == offset {
                        stack.push(("Local", last_off));
                        break;
                    }
                }
                OpGetaddrDumpReply::Label(val) => {
                    if last_off == offset {
                        stack.push(("Label", last_off));
                        break;
                    }
                }
                OpGetaddrDumpReply::Cacheinfo(val) => {
                    if last_off == offset {
                        stack.push(("Cacheinfo", last_off));
                        break;
                    }
                }
                OpGetaddrDumpReply::Flags(val) => {
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
            stack.push(("OpGetaddrDumpReply", cur));
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpGetaddrDumpRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpGetaddrDumpRequest<'r> {
    pub fn new(mut request: Request<'r>, header: &PushIfaddrmsg) -> Self {
        PushOpGetaddrDumpRequest::write_header(&mut request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode(&mut self) -> PushOpGetaddrDumpRequest<&mut Vec<u8>> {
        PushOpGetaddrDumpRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpGetaddrDumpRequest<RequestBuf<'r>> {
        PushOpGetaddrDumpRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpGetaddrDumpRequest<'_> {
    type ReplyType<'buf> = (PushIfaddrmsg, IterableOpGetaddrDumpReply<'buf>);
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
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpGetaddrDumpReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpGetaddrDumpRequest::new(buf)
            .1
            .lookup_attr(offset, missing_type)
    }
}
#[doc = "Get / dump IPv4/IPv6 multicast addresses."]
pub struct PushOpGetmulticastDumpRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetmulticastDumpRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetmulticastDumpRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushIfaddrmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushIfaddrmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
}
impl<Prev: Rec> Drop for PushOpGetmulticastDumpRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get / dump IPv4/IPv6 multicast addresses."]
#[derive(Clone)]
pub enum OpGetmulticastDumpRequest {}
impl<'a> IterableOpGetmulticastDumpRequest<'a> {}
impl OpGetmulticastDumpRequest {
    pub fn new(buf: &'_ [u8]) -> (PushIfaddrmsg, IterableOpGetmulticastDumpRequest<'_>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushIfaddrmsg::len()));
        (
            PushIfaddrmsg::new_from_slice(header).unwrap_or_default(),
            IterableOpGetmulticastDumpRequest::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        AddrAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetmulticastDumpRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetmulticastDumpRequest<'a> {
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
impl<'a> Iterator for IterableOpGetmulticastDumpRequest<'a> {
    type Item = Result<OpGetmulticastDumpRequest, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpGetmulticastDumpRequest",
            r#type.and_then(|t| OpGetmulticastDumpRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpGetmulticastDumpRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetmulticastDumpRequest");
        for attr in self.clone() {
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
impl IterableOpGetmulticastDumpRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushIfaddrmsg::len() {
            stack.push(("OpGetmulticastDumpRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetmulticastDumpRequest::attr_from_type(t)),
            );
        }
        (stack, None)
    }
}
#[doc = "Get / dump IPv4/IPv6 multicast addresses."]
pub struct PushOpGetmulticastDumpReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetmulticastDumpReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetmulticastDumpReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushIfaddrmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushIfaddrmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_cacheinfo(mut self, value: PushIfaCacheinfo) -> Self {
        push_header(self.as_rec_mut(), 6u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_multicast(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 7u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
}
impl<Prev: Rec> Drop for PushOpGetmulticastDumpReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get / dump IPv4/IPv6 multicast addresses."]
#[derive(Clone)]
pub enum OpGetmulticastDumpReply<'a> {
    Cacheinfo(PushIfaCacheinfo),
    Multicast(&'a [u8]),
}
impl<'a> IterableOpGetmulticastDumpReply<'a> {
    pub fn get_cacheinfo(&self) -> Result<PushIfaCacheinfo, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetmulticastDumpReply::Cacheinfo(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetmulticastDumpReply",
            "Cacheinfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_multicast(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetmulticastDumpReply::Multicast(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetmulticastDumpReply",
            "Multicast",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> OpGetmulticastDumpReply<'a> {
    pub fn new(buf: &'a [u8]) -> (PushIfaddrmsg, IterableOpGetmulticastDumpReply<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushIfaddrmsg::len()));
        (
            PushIfaddrmsg::new_from_slice(header).unwrap_or_default(),
            IterableOpGetmulticastDumpReply::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        AddrAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetmulticastDumpReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetmulticastDumpReply<'a> {
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
impl<'a> Iterator for IterableOpGetmulticastDumpReply<'a> {
    type Item = Result<OpGetmulticastDumpReply<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                6u16 => OpGetmulticastDumpReply::Cacheinfo({
                    let res = PushIfaCacheinfo::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => OpGetmulticastDumpReply::Multicast({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpGetmulticastDumpReply",
            r#type.and_then(|t| OpGetmulticastDumpReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpGetmulticastDumpReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetmulticastDumpReply");
        for attr in self.clone() {
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
                OpGetmulticastDumpReply::Cacheinfo(val) => fmt.field("Cacheinfo", &val),
                OpGetmulticastDumpReply::Multicast(val) => fmt.field("Multicast", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpGetmulticastDumpReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushIfaddrmsg::len() {
            stack.push(("OpGetmulticastDumpReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetmulticastDumpReply::attr_from_type(t)),
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
                OpGetmulticastDumpReply::Cacheinfo(val) => {
                    if last_off == offset {
                        stack.push(("Cacheinfo", last_off));
                        break;
                    }
                }
                OpGetmulticastDumpReply::Multicast(val) => {
                    if last_off == offset {
                        stack.push(("Multicast", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetmulticastDumpReply", cur));
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpGetmulticastDumpRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpGetmulticastDumpRequest<'r> {
    pub fn new(mut request: Request<'r>, header: &PushIfaddrmsg) -> Self {
        PushOpGetmulticastDumpRequest::write_header(&mut request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode(&mut self) -> PushOpGetmulticastDumpRequest<&mut Vec<u8>> {
        PushOpGetmulticastDumpRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpGetmulticastDumpRequest<RequestBuf<'r>> {
        PushOpGetmulticastDumpRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpGetmulticastDumpRequest<'_> {
    type ReplyType<'buf> = (PushIfaddrmsg, IterableOpGetmulticastDumpReply<'buf>);
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
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpGetmulticastDumpReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpGetmulticastDumpRequest::new(buf)
            .1
            .lookup_attr(offset, missing_type)
    }
}
#[doc = "Get / dump IPv4/IPv6 multicast addresses."]
pub struct PushOpGetmulticastDoRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetmulticastDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetmulticastDoRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushIfaddrmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushIfaddrmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
}
impl<Prev: Rec> Drop for PushOpGetmulticastDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get / dump IPv4/IPv6 multicast addresses."]
#[derive(Clone)]
pub enum OpGetmulticastDoRequest {}
impl<'a> IterableOpGetmulticastDoRequest<'a> {}
impl OpGetmulticastDoRequest {
    pub fn new(buf: &'_ [u8]) -> (PushIfaddrmsg, IterableOpGetmulticastDoRequest<'_>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushIfaddrmsg::len()));
        (
            PushIfaddrmsg::new_from_slice(header).unwrap_or_default(),
            IterableOpGetmulticastDoRequest::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        AddrAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetmulticastDoRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetmulticastDoRequest<'a> {
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
impl<'a> Iterator for IterableOpGetmulticastDoRequest<'a> {
    type Item = Result<OpGetmulticastDoRequest, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpGetmulticastDoRequest",
            r#type.and_then(|t| OpGetmulticastDoRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpGetmulticastDoRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetmulticastDoRequest");
        for attr in self.clone() {
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
impl IterableOpGetmulticastDoRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushIfaddrmsg::len() {
            stack.push(("OpGetmulticastDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetmulticastDoRequest::attr_from_type(t)),
            );
        }
        (stack, None)
    }
}
#[doc = "Get / dump IPv4/IPv6 multicast addresses."]
pub struct PushOpGetmulticastDoReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetmulticastDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetmulticastDoReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushIfaddrmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushIfaddrmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_cacheinfo(mut self, value: PushIfaCacheinfo) -> Self {
        push_header(self.as_rec_mut(), 6u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_multicast(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 7u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
}
impl<Prev: Rec> Drop for PushOpGetmulticastDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get / dump IPv4/IPv6 multicast addresses."]
#[derive(Clone)]
pub enum OpGetmulticastDoReply<'a> {
    Cacheinfo(PushIfaCacheinfo),
    Multicast(&'a [u8]),
}
impl<'a> IterableOpGetmulticastDoReply<'a> {
    pub fn get_cacheinfo(&self) -> Result<PushIfaCacheinfo, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetmulticastDoReply::Cacheinfo(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetmulticastDoReply",
            "Cacheinfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_multicast(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetmulticastDoReply::Multicast(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpGetmulticastDoReply",
            "Multicast",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> OpGetmulticastDoReply<'a> {
    pub fn new(buf: &'a [u8]) -> (PushIfaddrmsg, IterableOpGetmulticastDoReply<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(PushIfaddrmsg::len()));
        (
            PushIfaddrmsg::new_from_slice(header).unwrap_or_default(),
            IterableOpGetmulticastDoReply::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        AddrAttrs::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpGetmulticastDoReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpGetmulticastDoReply<'a> {
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
impl<'a> Iterator for IterableOpGetmulticastDoReply<'a> {
    type Item = Result<OpGetmulticastDoReply<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                6u16 => OpGetmulticastDoReply::Cacheinfo({
                    let res = PushIfaCacheinfo::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => OpGetmulticastDoReply::Multicast({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpGetmulticastDoReply",
            r#type.and_then(|t| OpGetmulticastDoReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpGetmulticastDoReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetmulticastDoReply");
        for attr in self.clone() {
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
                OpGetmulticastDoReply::Cacheinfo(val) => fmt.field("Cacheinfo", &val),
                OpGetmulticastDoReply::Multicast(val) => fmt.field("Multicast", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpGetmulticastDoReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushIfaddrmsg::len() {
            stack.push(("OpGetmulticastDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetmulticastDoReply::attr_from_type(t)),
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
                OpGetmulticastDoReply::Cacheinfo(val) => {
                    if last_off == offset {
                        stack.push(("Cacheinfo", last_off));
                        break;
                    }
                }
                OpGetmulticastDoReply::Multicast(val) => {
                    if last_off == offset {
                        stack.push(("Multicast", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetmulticastDoReply", cur));
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpGetmulticastDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpGetmulticastDoRequest<'r> {
    pub fn new(mut request: Request<'r>, header: &PushIfaddrmsg) -> Self {
        PushOpGetmulticastDoRequest::write_header(&mut request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpGetmulticastDoRequest<&mut Vec<u8>> {
        PushOpGetmulticastDoRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpGetmulticastDoRequest<RequestBuf<'r>> {
        PushOpGetmulticastDoRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpGetmulticastDoRequest<'_> {
    type ReplyType<'buf> = (PushIfaddrmsg, IterableOpGetmulticastDoReply<'buf>);
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
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpGetmulticastDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpGetmulticastDoRequest::new(buf)
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
    pub fn new_from_buf(buf: Vec<u8>, first_seq: u32) -> Self {
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
        self.buf_mut()
            .extend_from_slice(PushNlmsghdr::new().as_slice());
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
                assert_eq!(
                    self.last_header_offset + PushNlmsghdr::len(),
                    self.buf().len()
                );
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
        let mut header = PushNlmsghdr::new();
        header.set_len((buf.len() - header_offset) as u32);
        header.set_type(request_type);
        header.set_flags(flags | consts::NLM_F_REQUEST as u16 | consts::NLM_F_ACK as u16);
        header.set_seq(seq);
        buf[header_offset..(header_offset + 16)].clone_from_slice(header.as_slice());
    }
}
use crate::traits::LookupFn;
use crate::utils::RequestBuf;
#[derive(Debug)]
pub struct Request<'buf> {
    buf: RequestBuf<'buf>,
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
    pub fn new_from_buf(buf: Vec<u8>) -> Self {
        Self {
            flags: 0,
            buf: RequestBuf::Own(buf),
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
        Self {
            flags: 0,
            buf: RequestBuf::Ref(buf),
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
    #[doc = "Set `NLM_F_DUMP` flag"]
    fn set_dump(mut self) -> Self {
        self.flags |= consts::NLM_F_DUMP as u16;
        self
    }
    pub fn op_newaddr_do_request(self, header: &PushIfaddrmsg) -> RequestOpNewaddrDoRequest<'buf> {
        let mut res = RequestOpNewaddrDoRequest::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-newaddr-do-request",
            RequestOpNewaddrDoRequest::lookup,
        );
        res
    }
    pub fn op_deladdr_do_request(self, header: &PushIfaddrmsg) -> RequestOpDeladdrDoRequest<'buf> {
        let mut res = RequestOpDeladdrDoRequest::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-deladdr-do-request",
            RequestOpDeladdrDoRequest::lookup,
        );
        res
    }
    pub fn op_getaddr_dump_request(
        self,
        header: &PushIfaddrmsg,
    ) -> RequestOpGetaddrDumpRequest<'buf> {
        let mut res = RequestOpGetaddrDumpRequest::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-getaddr-dump-request",
            RequestOpGetaddrDumpRequest::lookup,
        );
        res
    }
    pub fn op_getmulticast_dump_request(
        self,
        header: &PushIfaddrmsg,
    ) -> RequestOpGetmulticastDumpRequest<'buf> {
        let mut res = RequestOpGetmulticastDumpRequest::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-getmulticast-dump-request",
            RequestOpGetmulticastDumpRequest::lookup,
        );
        res
    }
    pub fn op_getmulticast_do_request(
        self,
        header: &PushIfaddrmsg,
    ) -> RequestOpGetmulticastDoRequest<'buf> {
        let mut res = RequestOpGetmulticastDoRequest::new(self, header);
        res.request.do_writeback(
            res.protocol(),
            "op-getmulticast-do-request",
            RequestOpGetmulticastDoRequest::lookup,
        );
        res
    }
}
