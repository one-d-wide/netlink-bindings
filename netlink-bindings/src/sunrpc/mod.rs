#![doc = "SUNRPC cache upcall support over generic netlink.\n"]
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
pub const PROTONAME: &str = "sunrpc";
pub const PROTONAME_CSTR: &CStr = c"sunrpc";
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum CacheType {
    IpMap = 1 << 0,
    UnixGid = 1 << 1,
}
impl CacheType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::IpMap,
            n if n == 1 << 1 => Self::UnixGid,
            _ => return None,
        })
    }
}
#[derive(Clone)]
pub enum CacheNotify {
    #[doc = "Associated type: [`CacheType`] (enum)"]
    CacheType(u32),
}
impl<'a> IterableCacheNotify<'a> {
    #[doc = "Associated type: [`CacheType`] (enum)"]
    pub fn get_cache_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CacheNotify::CacheType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CacheNotify",
            "CacheType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl CacheNotify {
    pub fn new<'a>(buf: &'a [u8]) -> IterableCacheNotify<'a> {
        IterableCacheNotify::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "CacheType",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableCacheNotify<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableCacheNotify<'a> {
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
impl<'a> Iterator for IterableCacheNotify<'a> {
    type Item = Result<CacheNotify, ErrorContext>;
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
                1u16 => CacheNotify::CacheType({
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
            "CacheNotify",
            r#type.and_then(|t| CacheNotify::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableCacheNotify<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("CacheNotify");
        let mut iter = IterableCacheNotify::with_loc(&[], self.orig_loc);
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
                CacheNotify::CacheType(val) => {
                    fmt.field("CacheType", &FormatFlags(val.into(), CacheType::from_value))
                }
            };
        }
        fmt.finish()
    }
}
impl IterableCacheNotify<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("CacheNotify", offset));
            return (
                stack,
                missing_type.and_then(|t| CacheNotify::attr_from_type(t)),
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
                CacheNotify::CacheType(val) => {
                    if last_off == offset {
                        stack.push(("CacheType", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("CacheNotify", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum IpMap<'a> {
    Seqno(u64),
    Class(&'a CStr),
    Addr(&'a CStr),
    Domain(&'a CStr),
    Negative(()),
    Expiry(u64),
}
impl<'a> IterableIpMap<'a> {
    pub fn get_seqno(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(IpMap::Seqno(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IpMap",
            "Seqno",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_class(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(IpMap::Class(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IpMap",
            "Class",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_addr(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(IpMap::Addr(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IpMap",
            "Addr",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_domain(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(IpMap::Domain(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IpMap",
            "Domain",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_negative(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(IpMap::Negative(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IpMap",
            "Negative",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_expiry(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(IpMap::Expiry(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IpMap",
            "Expiry",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl IpMap<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableIpMap<'a> {
        IterableIpMap::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Seqno",
            2u16 => "Class",
            3u16 => "Addr",
            4u16 => "Domain",
            5u16 => "Negative",
            6u16 => "Expiry",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableIpMap<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableIpMap<'a> {
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
impl<'a> Iterator for IterableIpMap<'a> {
    type Item = Result<IpMap<'a>, ErrorContext>;
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
                1u16 => IpMap::Seqno({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => IpMap::Class({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => IpMap::Addr({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => IpMap::Domain({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => IpMap::Negative(()),
                6u16 => IpMap::Expiry({
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
            "IpMap",
            r#type.and_then(|t| IpMap::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableIpMap<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("IpMap");
        let mut iter = IterableIpMap::with_loc(&[], self.orig_loc);
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
                IpMap::Seqno(val) => fmt.field("Seqno", &val),
                IpMap::Class(val) => fmt.field("Class", &val),
                IpMap::Addr(val) => fmt.field("Addr", &val),
                IpMap::Domain(val) => fmt.field("Domain", &val),
                IpMap::Negative(val) => fmt.field("Negative", &val),
                IpMap::Expiry(val) => fmt.field("Expiry", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableIpMap<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("IpMap", offset));
            return (stack, missing_type.and_then(|t| IpMap::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                IpMap::Seqno(val) => {
                    if last_off == offset {
                        stack.push(("Seqno", last_off));
                        break;
                    }
                }
                IpMap::Class(val) => {
                    if last_off == offset {
                        stack.push(("Class", last_off));
                        break;
                    }
                }
                IpMap::Addr(val) => {
                    if last_off == offset {
                        stack.push(("Addr", last_off));
                        break;
                    }
                }
                IpMap::Domain(val) => {
                    if last_off == offset {
                        stack.push(("Domain", last_off));
                        break;
                    }
                }
                IpMap::Negative(val) => {
                    if last_off == offset {
                        stack.push(("Negative", last_off));
                        break;
                    }
                }
                IpMap::Expiry(val) => {
                    if last_off == offset {
                        stack.push(("Expiry", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("IpMap", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum IpMapReqs<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    Requests(IterableIpMap<'a>),
}
impl<'a> IterableIpMapReqs<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_requests(&self) -> MultiAttrIterable<Self, IpMapReqs<'a>, IterableIpMap<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let IpMapReqs::Requests(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl IpMapReqs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableIpMapReqs<'a> {
        IterableIpMapReqs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Requests",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableIpMapReqs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableIpMapReqs<'a> {
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
impl<'a> Iterator for IterableIpMapReqs<'a> {
    type Item = Result<IpMapReqs<'a>, ErrorContext>;
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
                1u16 => IpMapReqs::Requests({
                    let res = Some(IterableIpMap::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "IpMapReqs",
            r#type.and_then(|t| IpMapReqs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableIpMapReqs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("IpMapReqs");
        let mut iter = IterableIpMapReqs::with_loc(&[], self.orig_loc);
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
                IpMapReqs::Requests(val) => fmt.field("Requests", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableIpMapReqs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("IpMapReqs", offset));
            return (
                stack,
                missing_type.and_then(|t| IpMapReqs::attr_from_type(t)),
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
                IpMapReqs::Requests(val) => {
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
            stack.push(("IpMapReqs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum UnixGid {
    Seqno(u64),
    Uid(u32),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    Gids(u32),
    Negative(()),
    Expiry(u64),
}
impl<'a> IterableUnixGid<'a> {
    pub fn get_seqno(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(UnixGid::Seqno(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "UnixGid",
            "Seqno",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_uid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(UnixGid::Uid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "UnixGid",
            "Uid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_gids(&self) -> MultiAttrIterable<Self, UnixGid, u32> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let UnixGid::Gids(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    pub fn get_negative(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(UnixGid::Negative(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "UnixGid",
            "Negative",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_expiry(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(UnixGid::Expiry(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "UnixGid",
            "Expiry",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl UnixGid {
    pub fn new<'a>(buf: &'a [u8]) -> IterableUnixGid<'a> {
        IterableUnixGid::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Seqno",
            2u16 => "Uid",
            3u16 => "Gids",
            4u16 => "Negative",
            5u16 => "Expiry",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableUnixGid<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableUnixGid<'a> {
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
impl<'a> Iterator for IterableUnixGid<'a> {
    type Item = Result<UnixGid, ErrorContext>;
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
                1u16 => UnixGid::Seqno({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => UnixGid::Uid({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => UnixGid::Gids({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => UnixGid::Negative(()),
                5u16 => UnixGid::Expiry({
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
            "UnixGid",
            r#type.and_then(|t| UnixGid::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableUnixGid<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("UnixGid");
        let mut iter = IterableUnixGid::with_loc(&[], self.orig_loc);
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
                UnixGid::Seqno(val) => fmt.field("Seqno", &val),
                UnixGid::Uid(val) => fmt.field("Uid", &val),
                UnixGid::Gids(val) => fmt.field("Gids", &val),
                UnixGid::Negative(val) => fmt.field("Negative", &val),
                UnixGid::Expiry(val) => fmt.field("Expiry", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableUnixGid<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("UnixGid", offset));
            return (stack, missing_type.and_then(|t| UnixGid::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                UnixGid::Seqno(val) => {
                    if last_off == offset {
                        stack.push(("Seqno", last_off));
                        break;
                    }
                }
                UnixGid::Uid(val) => {
                    if last_off == offset {
                        stack.push(("Uid", last_off));
                        break;
                    }
                }
                UnixGid::Gids(val) => {
                    if last_off == offset {
                        stack.push(("Gids", last_off));
                        break;
                    }
                }
                UnixGid::Negative(val) => {
                    if last_off == offset {
                        stack.push(("Negative", last_off));
                        break;
                    }
                }
                UnixGid::Expiry(val) => {
                    if last_off == offset {
                        stack.push(("Expiry", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("UnixGid", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum UnixGidReqs<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    Requests(IterableUnixGid<'a>),
}
impl<'a> IterableUnixGidReqs<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_requests(&self) -> MultiAttrIterable<Self, UnixGidReqs<'a>, IterableUnixGid<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let UnixGidReqs::Requests(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl UnixGidReqs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableUnixGidReqs<'a> {
        IterableUnixGidReqs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Requests",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableUnixGidReqs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableUnixGidReqs<'a> {
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
impl<'a> Iterator for IterableUnixGidReqs<'a> {
    type Item = Result<UnixGidReqs<'a>, ErrorContext>;
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
                1u16 => UnixGidReqs::Requests({
                    let res = Some(IterableUnixGid::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "UnixGidReqs",
            r#type.and_then(|t| UnixGidReqs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableUnixGidReqs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("UnixGidReqs");
        let mut iter = IterableUnixGidReqs::with_loc(&[], self.orig_loc);
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
                UnixGidReqs::Requests(val) => fmt.field("Requests", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableUnixGidReqs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("UnixGidReqs", offset));
            return (
                stack,
                missing_type.and_then(|t| UnixGidReqs::attr_from_type(t)),
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
                UnixGidReqs::Requests(val) => {
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
            stack.push(("UnixGidReqs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum CacheFlush {
    #[doc = "Associated type: [`CacheType`] (1 bit per enumeration)"]
    Mask(u32),
}
impl<'a> IterableCacheFlush<'a> {
    #[doc = "Associated type: [`CacheType`] (1 bit per enumeration)"]
    pub fn get_mask(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CacheFlush::Mask(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CacheFlush",
            "Mask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl CacheFlush {
    pub fn new<'a>(buf: &'a [u8]) -> IterableCacheFlush<'a> {
        IterableCacheFlush::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Mask",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableCacheFlush<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableCacheFlush<'a> {
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
impl<'a> Iterator for IterableCacheFlush<'a> {
    type Item = Result<CacheFlush, ErrorContext>;
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
                1u16 => CacheFlush::Mask({
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
            "CacheFlush",
            r#type.and_then(|t| CacheFlush::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableCacheFlush<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("CacheFlush");
        let mut iter = IterableCacheFlush::with_loc(&[], self.orig_loc);
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
                CacheFlush::Mask(val) => {
                    fmt.field("Mask", &FormatFlags(val.into(), CacheType::from_value))
                }
            };
        }
        fmt.finish()
    }
}
impl IterableCacheFlush<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("CacheFlush", offset));
            return (
                stack,
                missing_type.and_then(|t| CacheFlush::attr_from_type(t)),
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
                CacheFlush::Mask(val) => {
                    if last_off == offset {
                        stack.push(("Mask", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("CacheFlush", cur));
        }
        (stack, None)
    }
}
pub struct PushCacheNotify<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushCacheNotify<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushCacheNotify<Prev> {
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
    #[doc = "Associated type: [`CacheType`] (enum)"]
    pub fn push_cache_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushCacheNotify<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushIpMap<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushIpMap<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushIpMap<Prev> {
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
    pub fn push_seqno(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 1u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_class(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_class_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_addr(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            3u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_addr_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_domain(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            4u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_domain_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 4u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_negative(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 5u16, 0 as u16);
        self
    }
    pub fn push_expiry(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 6u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushIpMap<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushIpMapReqs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushIpMapReqs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushIpMapReqs<Prev> {
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
    pub fn nested_requests(mut self) -> PushIpMap<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushIpMap {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushIpMapReqs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushUnixGid<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushUnixGid<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushUnixGid<Prev> {
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
    pub fn push_seqno(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 1u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_uid(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn push_gids(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_negative(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 4u16, 0 as u16);
        self
    }
    pub fn push_expiry(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 5u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushUnixGid<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushUnixGidReqs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushUnixGidReqs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushUnixGidReqs<Prev> {
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
    pub fn nested_requests(mut self) -> PushUnixGid<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushUnixGid {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushUnixGidReqs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushCacheFlush<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushCacheFlush<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushCacheFlush<Prev> {
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
    #[doc = "Associated type: [`CacheType`] (1 bit per enumeration)"]
    pub fn push_mask(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushCacheFlush<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Notify attributes:\n- [`.get_cache_type()`](IterableCacheNotify::get_cache_type)\n"]
#[derive(Debug)]
pub struct OpCacheNotifyNotif;
impl OpCacheNotifyNotif {
    pub const CMD: u8 = 1u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterableCacheNotify<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableCacheNotify::with_loc(attrs, buf.as_ptr() as usize)
    }
}
pub struct NotifGroup;
impl NotifGroup {
    pub const NONE: &str = "none";
    pub const NONE_CSTR: &CStr = c"none";
    #[doc = "Notifications:\n- [`OpCacheNotifyNotif`]\n"]
    pub const EXPORTD: &str = "exportd";
    #[doc = "Notifications:\n- [`OpCacheNotifyNotif`]\n"]
    pub const EXPORTD_CSTR: &CStr = c"exportd";
}
#[doc = "Dump all pending ip_map requests\n\nFlags: admin-perm\n\nReply attributes:\n- [.get_requests()](IterableIpMapReqs::get_requests)\n\n"]
#[derive(Debug)]
pub struct OpIpMapGetReqsDump<'r> {
    request: Request<'r>,
}
impl<'r> OpIpMapGetReqsDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushIpMapReqs<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushIpMapReqs::new(buf)
    }
    pub fn encode(&mut self) -> PushIpMapReqs<&mut Vec<u8>> {
        PushIpMapReqs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushIpMapReqs<RequestBuf<'r>> {
        PushIpMapReqs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableIpMapReqs<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableIpMapReqs::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 2u8;
        header.version = 1u8;
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &BuiltinNfgenmsg {
        let pos = self.request.pos;
        BuiltinNfgenmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut BuiltinNfgenmsg {
        let pos = self.request.pos;
        BuiltinNfgenmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpIpMapGetReqsDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("sunrpc".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableIpMapReqs<'buf>;
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        Self::decode_request(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        Self::decode_request(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "Respond to one or more ip_map requests\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_requests()](PushIpMapReqs::nested_requests)\n\n"]
#[derive(Debug)]
pub struct OpIpMapSetReqsDo<'r> {
    request: Request<'r>,
}
impl<'r> OpIpMapSetReqsDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushIpMapReqs<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushIpMapReqs::new(buf)
    }
    pub fn encode(&mut self) -> PushIpMapReqs<&mut Vec<u8>> {
        PushIpMapReqs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushIpMapReqs<RequestBuf<'r>> {
        PushIpMapReqs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableIpMapReqs<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableIpMapReqs::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 3u8;
        header.version = 1u8;
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &BuiltinNfgenmsg {
        let pos = self.request.pos;
        BuiltinNfgenmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut BuiltinNfgenmsg {
        let pos = self.request.pos;
        BuiltinNfgenmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpIpMapSetReqsDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("sunrpc".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableIpMapReqs<'buf>;
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        Self::decode_request(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        Self::decode_request(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "Dump all pending unix_gid requests\n\nFlags: admin-perm\n\nReply attributes:\n- [.get_requests()](IterableUnixGidReqs::get_requests)\n\n"]
#[derive(Debug)]
pub struct OpUnixGidGetReqsDump<'r> {
    request: Request<'r>,
}
impl<'r> OpUnixGidGetReqsDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushUnixGidReqs<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushUnixGidReqs::new(buf)
    }
    pub fn encode(&mut self) -> PushUnixGidReqs<&mut Vec<u8>> {
        PushUnixGidReqs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushUnixGidReqs<RequestBuf<'r>> {
        PushUnixGidReqs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableUnixGidReqs<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableUnixGidReqs::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 4u8;
        header.version = 1u8;
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &BuiltinNfgenmsg {
        let pos = self.request.pos;
        BuiltinNfgenmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut BuiltinNfgenmsg {
        let pos = self.request.pos;
        BuiltinNfgenmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpUnixGidGetReqsDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("sunrpc".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableUnixGidReqs<'buf>;
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        Self::decode_request(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        Self::decode_request(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "Respond to one or more unix_gid requests\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_requests()](PushUnixGidReqs::nested_requests)\n\n"]
#[derive(Debug)]
pub struct OpUnixGidSetReqsDo<'r> {
    request: Request<'r>,
}
impl<'r> OpUnixGidSetReqsDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushUnixGidReqs<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushUnixGidReqs::new(buf)
    }
    pub fn encode(&mut self) -> PushUnixGidReqs<&mut Vec<u8>> {
        PushUnixGidReqs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushUnixGidReqs<RequestBuf<'r>> {
        PushUnixGidReqs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableUnixGidReqs<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableUnixGidReqs::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 5u8;
        header.version = 1u8;
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &BuiltinNfgenmsg {
        let pos = self.request.pos;
        BuiltinNfgenmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut BuiltinNfgenmsg {
        let pos = self.request.pos;
        BuiltinNfgenmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpUnixGidSetReqsDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("sunrpc".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableUnixGidReqs<'buf>;
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        Self::decode_request(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        Self::decode_request(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "Flush sunrpc caches (ip_map and/or unix_gid)\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_mask()](PushCacheFlush::push_mask)\n\n"]
#[derive(Debug)]
pub struct OpCacheFlushDo<'r> {
    request: Request<'r>,
}
impl<'r> OpCacheFlushDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushCacheFlush<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushCacheFlush::new(buf)
    }
    pub fn encode(&mut self) -> PushCacheFlush<&mut Vec<u8>> {
        PushCacheFlush::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushCacheFlush<RequestBuf<'r>> {
        PushCacheFlush::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableCacheFlush<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableCacheFlush::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 6u8;
        header.version = 1u8;
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &BuiltinNfgenmsg {
        let pos = self.request.pos;
        BuiltinNfgenmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut BuiltinNfgenmsg {
        let pos = self.request.pos;
        BuiltinNfgenmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpCacheFlushDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("sunrpc".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableCacheFlush<'buf>;
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        Self::decode_request(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        Self::decode_request(buf).lookup_attr(offset, missing_type)
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
    #[doc = "Dump all pending ip_map requests\n\nFlags: admin-perm\n\nReply attributes:\n- [.get_requests()](IterableIpMapReqs::get_requests)\n\n"]
    pub fn op_ip_map_get_reqs_dump(self) -> OpIpMapGetReqsDump<'buf> {
        let mut res = OpIpMapGetReqsDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-ip-map-get-reqs-dump",
            OpIpMapGetReqsDump::lookup,
        );
        res
    }
    #[doc = "Respond to one or more ip_map requests\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_requests()](PushIpMapReqs::nested_requests)\n\n"]
    pub fn op_ip_map_set_reqs_do(self) -> OpIpMapSetReqsDo<'buf> {
        let mut res = OpIpMapSetReqsDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-ip-map-set-reqs-do",
            OpIpMapSetReqsDo::lookup,
        );
        res
    }
    #[doc = "Dump all pending unix_gid requests\n\nFlags: admin-perm\n\nReply attributes:\n- [.get_requests()](IterableUnixGidReqs::get_requests)\n\n"]
    pub fn op_unix_gid_get_reqs_dump(self) -> OpUnixGidGetReqsDump<'buf> {
        let mut res = OpUnixGidGetReqsDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-unix-gid-get-reqs-dump",
            OpUnixGidGetReqsDump::lookup,
        );
        res
    }
    #[doc = "Respond to one or more unix_gid requests\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_requests()](PushUnixGidReqs::nested_requests)\n\n"]
    pub fn op_unix_gid_set_reqs_do(self) -> OpUnixGidSetReqsDo<'buf> {
        let mut res = OpUnixGidSetReqsDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-unix-gid-set-reqs-do",
            OpUnixGidSetReqsDo::lookup,
        );
        res
    }
    #[doc = "Flush sunrpc caches (ip_map and/or unix_gid)\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_mask()](PushCacheFlush::push_mask)\n\n"]
    pub fn op_cache_flush_do(self) -> OpCacheFlushDo<'buf> {
        let mut res = OpCacheFlushDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-cache-flush-do", OpCacheFlushDo::lookup);
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableCacheNotify::get_cache_type;
        let _ = IterableIpMapReqs::get_requests;
        let _ = IterableUnixGidReqs::get_requests;
        let _ = OpCacheNotifyNotif;
        let _ = PushCacheFlush::<&mut Vec<u8>>::push_mask;
        let _ = PushIpMapReqs::<&mut Vec<u8>>::nested_requests;
        let _ = PushUnixGidReqs::<&mut Vec<u8>>::nested_requests;
    }
}
