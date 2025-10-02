#![doc = "Address configuration over rtnetlink."]
#![allow(clippy::all)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]
#![allow(unreachable_code)]
#![allow(unreachable_patterns)]
use crate::builtin::{PushBuiltinBitfield32, PushBuiltinNfgenmsg};
use crate::consts;
use crate::utils::*;
use crate::{NetlinkRequest, Protocol};
pub const PROTONAME: &CStr = c"rt-addr";
pub const PROTONUM: u16 = 0u16;
#[doc = "Original name: \"ifa-flags\" (flags) - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
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
#[doc = "Original name: \"addr-attrs\""]
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
impl<'a> Iterable<'a, AddrAttrs<'a>> {
    pub fn get_address(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let AddrAttrs::Address(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("AddrAttrs", "Address"))
    }
    pub fn get_local(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let AddrAttrs::Local(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("AddrAttrs", "Local"))
    }
    pub fn get_label(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let AddrAttrs::Label(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("AddrAttrs", "Label"))
    }
    pub fn get_broadcast(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let AddrAttrs::Broadcast(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("AddrAttrs", "Broadcast"))
    }
    pub fn get_anycast(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let AddrAttrs::Anycast(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("AddrAttrs", "Anycast"))
    }
    pub fn get_cacheinfo(&self) -> Result<PushIfaCacheinfo, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let AddrAttrs::Cacheinfo(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("AddrAttrs", "Cacheinfo"))
    }
    pub fn get_multicast(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let AddrAttrs::Multicast(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("AddrAttrs", "Multicast"))
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
        Err(self.error_missing("AddrAttrs", "Flags"))
    }
    pub fn get_rt_priority(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let AddrAttrs::RtPriority(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("AddrAttrs", "RtPriority"))
    }
    pub fn get_target_netnsid(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let AddrAttrs::TargetNetnsid(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("AddrAttrs", "TargetNetnsid"))
    }
    pub fn get_proto(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let AddrAttrs::Proto(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("AddrAttrs", "Proto"))
    }
}
impl<'a> AddrAttrs<'a> {
    pub fn new(buf: &'a [u8]) -> Iterable<'a, AddrAttrs<'a>> {
        Iterable::new(buf)
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
impl<'a> Iterator for Iterable<'a, AddrAttrs<'a>> {
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
        Some(Err(self.error_context(
            "AddrAttrs",
            r#type.and_then(|t| AddrAttrs::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, AddrAttrs<'a>> {
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
impl<'a> Iterable<'a, AddrAttrs<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
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
    prev: Option<Prev>,
    header_offset: Option<usize>,
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
#[doc = "Original name: \"ifaddrmsg\""]
#[derive(Clone)]
pub struct PushIfaddrmsg {
    buf: [u8; 8usize],
}
impl PushIfaddrmsg {
    #[doc = "Create zero-initialized struct"]
    pub fn new() -> Self {
        Self {
            buf: [0u8; Self::len()],
        }
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
#[doc = "Original name: \"ifa-cacheinfo\""]
#[derive(Clone)]
pub struct PushIfaCacheinfo {
    buf: [u8; 16usize],
}
impl PushIfaCacheinfo {
    #[doc = "Create zero-initialized struct"]
    pub fn new() -> Self {
        Self {
            buf: [0u8; Self::len()],
        }
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
    prev: Option<Prev>,
    header_offset: Option<usize>,
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
#[doc = "Original name: \"op-newaddr-do-request\""]
#[derive(Clone)]
pub enum OpNewaddrDoRequest<'a> {
    Address(std::net::IpAddr),
    Local(std::net::IpAddr),
    Label(&'a CStr),
    Cacheinfo(PushIfaCacheinfo),
    #[doc = "Associated type: \"IfaFlags\" (1 bit per enumeration)"]
    Flags(u32),
}
impl<'a> Iterable<'a, OpNewaddrDoRequest<'a>> {
    pub fn get_address(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewaddrDoRequest::Address(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpNewaddrDoRequest", "Address"))
    }
    pub fn get_local(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewaddrDoRequest::Local(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpNewaddrDoRequest", "Local"))
    }
    pub fn get_label(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewaddrDoRequest::Label(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpNewaddrDoRequest", "Label"))
    }
    pub fn get_cacheinfo(&self) -> Result<PushIfaCacheinfo, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewaddrDoRequest::Cacheinfo(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpNewaddrDoRequest", "Cacheinfo"))
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
        Err(self.error_missing("OpNewaddrDoRequest", "Flags"))
    }
}
impl<'a> OpNewaddrDoRequest<'a> {
    pub fn new(buf: &'a [u8]) -> (PushIfaddrmsg, Iterable<'a, OpNewaddrDoRequest<'a>>) {
        let mut header = PushIfaddrmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushIfaddrmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushIfaddrmsg::len()..], buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        AddrAttrs::attr_from_type(r#type)
    }
}
impl<'a> Iterator for Iterable<'a, OpNewaddrDoRequest<'a>> {
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
        Some(Err(self.error_context(
            "OpNewaddrDoRequest",
            r#type.and_then(|t| OpNewaddrDoRequest::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpNewaddrDoRequest<'a>> {
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
impl<'a> Iterable<'a, OpNewaddrDoRequest<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
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
    prev: Option<Prev>,
    header_offset: Option<usize>,
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
#[doc = "Original name: \"op-newaddr-do-reply\""]
#[derive(Clone)]
pub enum OpNewaddrDoReply {}
impl<'a> Iterable<'a, OpNewaddrDoReply> {}
impl OpNewaddrDoReply {
    pub fn new(buf: &'_ [u8]) -> (PushIfaddrmsg, Iterable<'_, OpNewaddrDoReply>) {
        let mut header = PushIfaddrmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushIfaddrmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushIfaddrmsg::len()..], buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        AddrAttrs::attr_from_type(r#type)
    }
}
impl Iterator for Iterable<'_, OpNewaddrDoReply> {
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
        Some(Err(self.error_context(
            "OpNewaddrDoReply",
            r#type.and_then(|t| OpNewaddrDoReply::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl std::fmt::Debug for Iterable<'_, OpNewaddrDoReply> {
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
impl Iterable<'_, OpNewaddrDoReply> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
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
}
impl NetlinkRequest for RequestOpNewaddrDoRequest<'_> {
    type ReplyType<'buf> = (PushIfaddrmsg, Iterable<'buf, OpNewaddrDoReply>);
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
    prev: Option<Prev>,
    header_offset: Option<usize>,
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
#[doc = "Original name: \"op-deladdr-do-request\""]
#[derive(Clone)]
pub enum OpDeladdrDoRequest {
    Address(std::net::IpAddr),
    Local(std::net::IpAddr),
}
impl<'a> Iterable<'a, OpDeladdrDoRequest> {
    pub fn get_address(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDeladdrDoRequest::Address(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpDeladdrDoRequest", "Address"))
    }
    pub fn get_local(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDeladdrDoRequest::Local(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpDeladdrDoRequest", "Local"))
    }
}
impl OpDeladdrDoRequest {
    pub fn new(buf: &'_ [u8]) -> (PushIfaddrmsg, Iterable<'_, OpDeladdrDoRequest>) {
        let mut header = PushIfaddrmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushIfaddrmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushIfaddrmsg::len()..], buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        AddrAttrs::attr_from_type(r#type)
    }
}
impl Iterator for Iterable<'_, OpDeladdrDoRequest> {
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
        Some(Err(self.error_context(
            "OpDeladdrDoRequest",
            r#type.and_then(|t| OpDeladdrDoRequest::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl std::fmt::Debug for Iterable<'_, OpDeladdrDoRequest> {
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
impl Iterable<'_, OpDeladdrDoRequest> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
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
    prev: Option<Prev>,
    header_offset: Option<usize>,
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
#[doc = "Original name: \"op-deladdr-do-reply\""]
#[derive(Clone)]
pub enum OpDeladdrDoReply {}
impl<'a> Iterable<'a, OpDeladdrDoReply> {}
impl OpDeladdrDoReply {
    pub fn new(buf: &'_ [u8]) -> (PushIfaddrmsg, Iterable<'_, OpDeladdrDoReply>) {
        let mut header = PushIfaddrmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushIfaddrmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushIfaddrmsg::len()..], buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        AddrAttrs::attr_from_type(r#type)
    }
}
impl Iterator for Iterable<'_, OpDeladdrDoReply> {
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
        Some(Err(self.error_context(
            "OpDeladdrDoReply",
            r#type.and_then(|t| OpDeladdrDoReply::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl std::fmt::Debug for Iterable<'_, OpDeladdrDoReply> {
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
impl Iterable<'_, OpDeladdrDoReply> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
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
}
impl NetlinkRequest for RequestOpDeladdrDoRequest<'_> {
    type ReplyType<'buf> = (PushIfaddrmsg, Iterable<'buf, OpDeladdrDoReply>);
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
    prev: Option<Prev>,
    header_offset: Option<usize>,
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
#[doc = "Original name: \"op-getaddr-dump-request\""]
#[derive(Clone)]
pub enum OpGetaddrDumpRequest {}
impl<'a> Iterable<'a, OpGetaddrDumpRequest> {}
impl OpGetaddrDumpRequest {
    pub fn new(buf: &'_ [u8]) -> (PushIfaddrmsg, Iterable<'_, OpGetaddrDumpRequest>) {
        let mut header = PushIfaddrmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushIfaddrmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushIfaddrmsg::len()..], buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        AddrAttrs::attr_from_type(r#type)
    }
}
impl Iterator for Iterable<'_, OpGetaddrDumpRequest> {
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
        Some(Err(self.error_context(
            "OpGetaddrDumpRequest",
            r#type.and_then(|t| OpGetaddrDumpRequest::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl std::fmt::Debug for Iterable<'_, OpGetaddrDumpRequest> {
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
impl Iterable<'_, OpGetaddrDumpRequest> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
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
    prev: Option<Prev>,
    header_offset: Option<usize>,
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
#[doc = "Original name: \"op-getaddr-dump-reply\""]
#[derive(Clone)]
pub enum OpGetaddrDumpReply<'a> {
    Address(std::net::IpAddr),
    Local(std::net::IpAddr),
    Label(&'a CStr),
    Cacheinfo(PushIfaCacheinfo),
    #[doc = "Associated type: \"IfaFlags\" (1 bit per enumeration)"]
    Flags(u32),
}
impl<'a> Iterable<'a, OpGetaddrDumpReply<'a>> {
    pub fn get_address(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetaddrDumpReply::Address(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetaddrDumpReply", "Address"))
    }
    pub fn get_local(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetaddrDumpReply::Local(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetaddrDumpReply", "Local"))
    }
    pub fn get_label(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetaddrDumpReply::Label(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetaddrDumpReply", "Label"))
    }
    pub fn get_cacheinfo(&self) -> Result<PushIfaCacheinfo, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetaddrDumpReply::Cacheinfo(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetaddrDumpReply", "Cacheinfo"))
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
        Err(self.error_missing("OpGetaddrDumpReply", "Flags"))
    }
}
impl<'a> OpGetaddrDumpReply<'a> {
    pub fn new(buf: &'a [u8]) -> (PushIfaddrmsg, Iterable<'a, OpGetaddrDumpReply<'a>>) {
        let mut header = PushIfaddrmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushIfaddrmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushIfaddrmsg::len()..], buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        AddrAttrs::attr_from_type(r#type)
    }
}
impl<'a> Iterator for Iterable<'a, OpGetaddrDumpReply<'a>> {
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
        Some(Err(self.error_context(
            "OpGetaddrDumpReply",
            r#type.and_then(|t| OpGetaddrDumpReply::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpGetaddrDumpReply<'a>> {
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
impl<'a> Iterable<'a, OpGetaddrDumpReply<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
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
}
impl NetlinkRequest for RequestOpGetaddrDumpRequest<'_> {
    type ReplyType<'buf> = (PushIfaddrmsg, Iterable<'buf, OpGetaddrDumpReply<'buf>>);
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
    prev: Option<Prev>,
    header_offset: Option<usize>,
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
#[doc = "Original name: \"op-getmulticast-dump-request\""]
#[derive(Clone)]
pub enum OpGetmulticastDumpRequest {}
impl<'a> Iterable<'a, OpGetmulticastDumpRequest> {}
impl OpGetmulticastDumpRequest {
    pub fn new(buf: &'_ [u8]) -> (PushIfaddrmsg, Iterable<'_, OpGetmulticastDumpRequest>) {
        let mut header = PushIfaddrmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushIfaddrmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushIfaddrmsg::len()..], buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        AddrAttrs::attr_from_type(r#type)
    }
}
impl Iterator for Iterable<'_, OpGetmulticastDumpRequest> {
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
        Some(Err(self.error_context(
            "OpGetmulticastDumpRequest",
            r#type.and_then(|t| OpGetmulticastDumpRequest::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl std::fmt::Debug for Iterable<'_, OpGetmulticastDumpRequest> {
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
impl Iterable<'_, OpGetmulticastDumpRequest> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
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
    prev: Option<Prev>,
    header_offset: Option<usize>,
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
#[doc = "Original name: \"op-getmulticast-dump-reply\""]
#[derive(Clone)]
pub enum OpGetmulticastDumpReply<'a> {
    Cacheinfo(PushIfaCacheinfo),
    Multicast(&'a [u8]),
}
impl<'a> Iterable<'a, OpGetmulticastDumpReply<'a>> {
    pub fn get_cacheinfo(&self) -> Result<PushIfaCacheinfo, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetmulticastDumpReply::Cacheinfo(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetmulticastDumpReply", "Cacheinfo"))
    }
    pub fn get_multicast(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetmulticastDumpReply::Multicast(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetmulticastDumpReply", "Multicast"))
    }
}
impl<'a> OpGetmulticastDumpReply<'a> {
    pub fn new(buf: &'a [u8]) -> (PushIfaddrmsg, Iterable<'a, OpGetmulticastDumpReply<'a>>) {
        let mut header = PushIfaddrmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushIfaddrmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushIfaddrmsg::len()..], buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        AddrAttrs::attr_from_type(r#type)
    }
}
impl<'a> Iterator for Iterable<'a, OpGetmulticastDumpReply<'a>> {
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
        Some(Err(self.error_context(
            "OpGetmulticastDumpReply",
            r#type.and_then(|t| OpGetmulticastDumpReply::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpGetmulticastDumpReply<'a>> {
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
impl<'a> Iterable<'a, OpGetmulticastDumpReply<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
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
}
impl NetlinkRequest for RequestOpGetmulticastDumpRequest<'_> {
    type ReplyType<'buf> = (PushIfaddrmsg, Iterable<'buf, OpGetmulticastDumpReply<'buf>>);
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
    prev: Option<Prev>,
    header_offset: Option<usize>,
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
#[doc = "Original name: \"op-getmulticast-do-request\""]
#[derive(Clone)]
pub enum OpGetmulticastDoRequest {}
impl<'a> Iterable<'a, OpGetmulticastDoRequest> {}
impl OpGetmulticastDoRequest {
    pub fn new(buf: &'_ [u8]) -> (PushIfaddrmsg, Iterable<'_, OpGetmulticastDoRequest>) {
        let mut header = PushIfaddrmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushIfaddrmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushIfaddrmsg::len()..], buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        AddrAttrs::attr_from_type(r#type)
    }
}
impl Iterator for Iterable<'_, OpGetmulticastDoRequest> {
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
        Some(Err(self.error_context(
            "OpGetmulticastDoRequest",
            r#type.and_then(|t| OpGetmulticastDoRequest::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl std::fmt::Debug for Iterable<'_, OpGetmulticastDoRequest> {
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
impl Iterable<'_, OpGetmulticastDoRequest> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
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
    prev: Option<Prev>,
    header_offset: Option<usize>,
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
#[doc = "Original name: \"op-getmulticast-do-reply\""]
#[derive(Clone)]
pub enum OpGetmulticastDoReply<'a> {
    Cacheinfo(PushIfaCacheinfo),
    Multicast(&'a [u8]),
}
impl<'a> Iterable<'a, OpGetmulticastDoReply<'a>> {
    pub fn get_cacheinfo(&self) -> Result<PushIfaCacheinfo, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetmulticastDoReply::Cacheinfo(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetmulticastDoReply", "Cacheinfo"))
    }
    pub fn get_multicast(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetmulticastDoReply::Multicast(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetmulticastDoReply", "Multicast"))
    }
}
impl<'a> OpGetmulticastDoReply<'a> {
    pub fn new(buf: &'a [u8]) -> (PushIfaddrmsg, Iterable<'a, OpGetmulticastDoReply<'a>>) {
        let mut header = PushIfaddrmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushIfaddrmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushIfaddrmsg::len()..], buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        AddrAttrs::attr_from_type(r#type)
    }
}
impl<'a> Iterator for Iterable<'a, OpGetmulticastDoReply<'a>> {
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
        Some(Err(self.error_context(
            "OpGetmulticastDoReply",
            r#type.and_then(|t| OpGetmulticastDoReply::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpGetmulticastDoReply<'a>> {
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
impl<'a> Iterable<'a, OpGetmulticastDoReply<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
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
}
impl NetlinkRequest for RequestOpGetmulticastDoRequest<'_> {
    type ReplyType<'buf> = (PushIfaddrmsg, Iterable<'buf, OpGetmulticastDoReply<'buf>>);
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
enum RequestBuf<'buf> {
    Ref(&'buf mut Vec<u8>),
    Own(Vec<u8>),
}
#[derive(Debug)]
pub struct Request<'buf> {
    buf: RequestBuf<'buf>,
    flags: u16,
}
impl Request<'static> {
    pub fn new() -> Self {
        Self {
            flags: 0,
            buf: RequestBuf::Own(Vec::new()),
        }
    }
    pub fn from_buf(buf: Vec<u8>) -> Self {
        Self {
            flags: 0,
            buf: RequestBuf::Own(buf),
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
        Self {
            flags: 0,
            buf: RequestBuf::Ref(buf),
        }
    }
    fn buf(&self) -> &Vec<u8> {
        match &self.buf {
            RequestBuf::Ref(buf) => buf,
            RequestBuf::Own(buf) => buf,
        }
    }
    fn buf_mut(&mut self) -> &mut Vec<u8> {
        match &mut self.buf {
            RequestBuf::Ref(buf) => buf,
            RequestBuf::Own(buf) => buf,
        }
    }
    #[doc = "Set [`libc::NLM_F_CREATE`] flag"]
    pub fn set_create(mut self) -> Self {
        self.flags |= consts::NLM_F_CREATE as u16;
        self
    }
    #[doc = "Set [`libc::NLM_F_EXCL`] flag"]
    pub fn set_excl(mut self) -> Self {
        self.flags |= consts::NLM_F_EXCL as u16;
        self
    }
    #[doc = "Set [`libc::NLM_F_REPLACE`] flag"]
    pub fn set_replace(mut self) -> Self {
        self.flags |= consts::NLM_F_REPLACE as u16;
        self
    }
    #[doc = "Set [`libc::NLM_F_CREATE`] and [`libc::NLM_F_REPLACE`] flag"]
    pub fn set_change(self) -> Self {
        self.set_create().set_replace()
    }
    #[doc = "Set [`libc::NLM_F_APPEND`] flag"]
    pub fn set_append(mut self) -> Self {
        self.flags |= consts::NLM_F_APPEND as u16;
        self
    }
    #[doc = "Set [`libc::NLM_F_DUMP`] flag"]
    fn set_dump(mut self) -> Self {
        self.flags |= consts::NLM_F_DUMP as u16;
        self
    }
    pub fn op_newaddr_do_request(self, header: &PushIfaddrmsg) -> RequestOpNewaddrDoRequest<'buf> {
        RequestOpNewaddrDoRequest::new(self, header)
    }
    pub fn op_deladdr_do_request(self, header: &PushIfaddrmsg) -> RequestOpDeladdrDoRequest<'buf> {
        RequestOpDeladdrDoRequest::new(self, header)
    }
    pub fn op_getaddr_dump_request(
        self,
        header: &PushIfaddrmsg,
    ) -> RequestOpGetaddrDumpRequest<'buf> {
        RequestOpGetaddrDumpRequest::new(self, header)
    }
    pub fn op_getmulticast_dump_request(
        self,
        header: &PushIfaddrmsg,
    ) -> RequestOpGetmulticastDumpRequest<'buf> {
        RequestOpGetmulticastDumpRequest::new(self, header)
    }
    pub fn op_getmulticast_do_request(
        self,
        header: &PushIfaddrmsg,
    ) -> RequestOpGetmulticastDoRequest<'buf> {
        RequestOpGetmulticastDoRequest::new(self, header)
    }
}
