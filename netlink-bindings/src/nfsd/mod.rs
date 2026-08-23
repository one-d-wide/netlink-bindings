#![doc = "NFSD configuration over generic netlink.\n"]
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
pub const PROTONAME: &str = "nfsd";
pub const PROTONAME_CSTR: &CStr = c"nfsd";
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum CacheType {
    SvcExport = 1 << 0,
    Expkey = 1 << 1,
}
impl CacheType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::SvcExport,
            n if n == 1 << 1 => Self::Expkey,
            _ => return None,
        })
    }
}
#[doc = "These flags are ordered to match the [NFSEXP]()\\* flags in\ninclude/linux/nfsd/export.h\n"]
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum ExportFlags {
    Readonly = 1 << 0,
    InsecurePort = 1 << 1,
    Rootsquash = 1 << 2,
    Allsquash = 1 << 3,
    Async = 1 << 4,
    GatheredWrites = 1 << 5,
    Noreaddirplus = 1 << 6,
    SecurityLabel = 1 << 7,
    SignFh = 1 << 8,
    Nohide = 1 << 9,
    Nosubtreecheck = 1 << 10,
    Noauthnlm = 1 << 11,
    Msnfs = 1 << 12,
    Fsid = 1 << 13,
    Crossmount = 1 << 14,
    Noacl = 1 << 15,
    V4root = 1 << 16,
    Pnfs = 1 << 17,
}
impl ExportFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::Readonly,
            n if n == 1 << 1 => Self::InsecurePort,
            n if n == 1 << 2 => Self::Rootsquash,
            n if n == 1 << 3 => Self::Allsquash,
            n if n == 1 << 4 => Self::Async,
            n if n == 1 << 5 => Self::GatheredWrites,
            n if n == 1 << 6 => Self::Noreaddirplus,
            n if n == 1 << 7 => Self::SecurityLabel,
            n if n == 1 << 8 => Self::SignFh,
            n if n == 1 << 9 => Self::Nohide,
            n if n == 1 << 10 => Self::Nosubtreecheck,
            n if n == 1 << 11 => Self::Noauthnlm,
            n if n == 1 << 12 => Self::Msnfs,
            n if n == 1 << 13 => Self::Fsid,
            n if n == 1 << 14 => Self::Crossmount,
            n if n == 1 << 15 => Self::Noacl,
            n if n == 1 << 16 => Self::V4root,
            n if n == 1 << 17 => Self::Pnfs,
            _ => return None,
        })
    }
}
#[doc = "These flags are ordered to match the [NFSEXP_XPRTSEC]()\\* flags in\ninclude/linux/nfsd/export.h\n"]
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum XprtsecMode {
    None = 1 << 0,
    Tls = 1 << 1,
    Mtls = 1 << 2,
}
impl XprtsecMode {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::None,
            n if n == 1 << 1 => Self::Tls,
            n if n == 1 << 2 => Self::Mtls,
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
pub enum RpcStatus<'a> {
    Xid(u32),
    Flags(u32),
    Prog(u32),
    Version(u8),
    Proc(u32),
    ServiceTime(i64),
    Pad(&'a [u8]),
    Saddr4(std::net::Ipv4Addr),
    Daddr4(std::net::Ipv4Addr),
    Saddr6(&'a [u8]),
    Daddr6(&'a [u8]),
    Sport(u16),
    Dport(u16),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    CompoundOps(u32),
}
impl<'a> IterableRpcStatus<'a> {
    pub fn get_xid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RpcStatus::Xid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "Xid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RpcStatus::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_prog(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RpcStatus::Prog(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "Prog",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_version(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RpcStatus::Version(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "Version",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_proc(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RpcStatus::Proc(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "Proc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_service_time(&self) -> Result<i64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RpcStatus::ServiceTime(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "ServiceTime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RpcStatus::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_saddr4(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RpcStatus::Saddr4(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "Saddr4",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_daddr4(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RpcStatus::Daddr4(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "Daddr4",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_saddr6(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RpcStatus::Saddr6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "Saddr6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_daddr6(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RpcStatus::Daddr6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "Daddr6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RpcStatus::Sport(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "Sport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(RpcStatus::Dport(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "Dport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_compound_ops(&self) -> MultiAttrIterable<Self, RpcStatus<'a>, u32> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let RpcStatus::CompoundOps(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl RpcStatus<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableRpcStatus<'a> {
        IterableRpcStatus::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Xid",
            2u16 => "Flags",
            3u16 => "Prog",
            4u16 => "Version",
            5u16 => "Proc",
            6u16 => "ServiceTime",
            7u16 => "Pad",
            8u16 => "Saddr4",
            9u16 => "Daddr4",
            10u16 => "Saddr6",
            11u16 => "Daddr6",
            12u16 => "Sport",
            13u16 => "Dport",
            14u16 => "CompoundOps",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableRpcStatus<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableRpcStatus<'a> {
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
impl<'a> Iterator for IterableRpcStatus<'a> {
    type Item = Result<RpcStatus<'a>, ErrorContext>;
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
                1u16 => RpcStatus::Xid({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => RpcStatus::Flags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => RpcStatus::Prog({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => RpcStatus::Version({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => RpcStatus::Proc({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => RpcStatus::ServiceTime({
                    let res = parse_i64(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => RpcStatus::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => RpcStatus::Saddr4({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => RpcStatus::Daddr4({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => RpcStatus::Saddr6({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => RpcStatus::Daddr6({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => RpcStatus::Sport({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => RpcStatus::Dport({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => RpcStatus::CompoundOps({
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
            "RpcStatus",
            r#type.and_then(|t| RpcStatus::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableRpcStatus<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("RpcStatus");
        let mut iter = IterableRpcStatus::with_loc(&[], self.orig_loc);
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
                RpcStatus::Xid(val) => fmt.field("Xid", &val),
                RpcStatus::Flags(val) => fmt.field("Flags", &val),
                RpcStatus::Prog(val) => fmt.field("Prog", &val),
                RpcStatus::Version(val) => fmt.field("Version", &val),
                RpcStatus::Proc(val) => fmt.field("Proc", &val),
                RpcStatus::ServiceTime(val) => fmt.field("ServiceTime", &val),
                RpcStatus::Pad(val) => fmt.field("Pad", &val),
                RpcStatus::Saddr4(val) => fmt.field("Saddr4", &val),
                RpcStatus::Daddr4(val) => fmt.field("Daddr4", &val),
                RpcStatus::Saddr6(val) => fmt.field("Saddr6", &val),
                RpcStatus::Daddr6(val) => fmt.field("Daddr6", &val),
                RpcStatus::Sport(val) => fmt.field("Sport", &val),
                RpcStatus::Dport(val) => fmt.field("Dport", &val),
                RpcStatus::CompoundOps(val) => fmt.field("CompoundOps", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableRpcStatus<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("RpcStatus", offset));
            return (
                stack,
                missing_type.and_then(|t| RpcStatus::attr_from_type(t)),
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
                RpcStatus::Xid(val) => {
                    if last_off == offset {
                        stack.push(("Xid", last_off));
                        break;
                    }
                }
                RpcStatus::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                RpcStatus::Prog(val) => {
                    if last_off == offset {
                        stack.push(("Prog", last_off));
                        break;
                    }
                }
                RpcStatus::Version(val) => {
                    if last_off == offset {
                        stack.push(("Version", last_off));
                        break;
                    }
                }
                RpcStatus::Proc(val) => {
                    if last_off == offset {
                        stack.push(("Proc", last_off));
                        break;
                    }
                }
                RpcStatus::ServiceTime(val) => {
                    if last_off == offset {
                        stack.push(("ServiceTime", last_off));
                        break;
                    }
                }
                RpcStatus::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                RpcStatus::Saddr4(val) => {
                    if last_off == offset {
                        stack.push(("Saddr4", last_off));
                        break;
                    }
                }
                RpcStatus::Daddr4(val) => {
                    if last_off == offset {
                        stack.push(("Daddr4", last_off));
                        break;
                    }
                }
                RpcStatus::Saddr6(val) => {
                    if last_off == offset {
                        stack.push(("Saddr6", last_off));
                        break;
                    }
                }
                RpcStatus::Daddr6(val) => {
                    if last_off == offset {
                        stack.push(("Daddr6", last_off));
                        break;
                    }
                }
                RpcStatus::Sport(val) => {
                    if last_off == offset {
                        stack.push(("Sport", last_off));
                        break;
                    }
                }
                RpcStatus::Dport(val) => {
                    if last_off == offset {
                        stack.push(("Dport", last_off));
                        break;
                    }
                }
                RpcStatus::CompoundOps(val) => {
                    if last_off == offset {
                        stack.push(("CompoundOps", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("RpcStatus", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum Server<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    Threads(u32),
    Gracetime(u32),
    Leasetime(u32),
    Scope(&'a CStr),
    MinThreads(u32),
    FhKey(&'a [u8]),
}
impl<'a> IterableServer<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_threads(&self) -> MultiAttrIterable<Self, Server<'a>, u32> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Server::Threads(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    pub fn get_gracetime(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Server::Gracetime(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Server",
            "Gracetime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_leasetime(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Server::Leasetime(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Server",
            "Leasetime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_scope(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Server::Scope(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Server",
            "Scope",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_min_threads(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Server::MinThreads(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Server",
            "MinThreads",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fh_key(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Server::FhKey(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Server",
            "FhKey",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Server<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableServer<'a> {
        IterableServer::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Threads",
            2u16 => "Gracetime",
            3u16 => "Leasetime",
            4u16 => "Scope",
            5u16 => "MinThreads",
            6u16 => "FhKey",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableServer<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableServer<'a> {
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
impl<'a> Iterator for IterableServer<'a> {
    type Item = Result<Server<'a>, ErrorContext>;
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
                1u16 => Server::Threads({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Server::Gracetime({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Server::Leasetime({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Server::Scope({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Server::MinThreads({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Server::FhKey({
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
            "Server",
            r#type.and_then(|t| Server::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableServer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Server");
        let mut iter = IterableServer::with_loc(&[], self.orig_loc);
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
                Server::Threads(val) => fmt.field("Threads", &val),
                Server::Gracetime(val) => fmt.field("Gracetime", &val),
                Server::Leasetime(val) => fmt.field("Leasetime", &val),
                Server::Scope(val) => fmt.field("Scope", &val),
                Server::MinThreads(val) => fmt.field("MinThreads", &val),
                Server::FhKey(val) => fmt.field("FhKey", &FormatHexdump(val)),
            };
        }
        fmt.finish()
    }
}
impl IterableServer<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Server", offset));
            return (stack, missing_type.and_then(|t| Server::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Server::Threads(val) => {
                    if last_off == offset {
                        stack.push(("Threads", last_off));
                        break;
                    }
                }
                Server::Gracetime(val) => {
                    if last_off == offset {
                        stack.push(("Gracetime", last_off));
                        break;
                    }
                }
                Server::Leasetime(val) => {
                    if last_off == offset {
                        stack.push(("Leasetime", last_off));
                        break;
                    }
                }
                Server::Scope(val) => {
                    if last_off == offset {
                        stack.push(("Scope", last_off));
                        break;
                    }
                }
                Server::MinThreads(val) => {
                    if last_off == offset {
                        stack.push(("MinThreads", last_off));
                        break;
                    }
                }
                Server::FhKey(val) => {
                    if last_off == offset {
                        stack.push(("FhKey", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Server", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum Version {
    Major(u32),
    Minor(u32),
    Enabled(()),
}
impl<'a> IterableVersion<'a> {
    pub fn get_major(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Version::Major(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Version",
            "Major",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_minor(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Version::Minor(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Version",
            "Minor",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_enabled(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Version::Enabled(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Version",
            "Enabled",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Version {
    pub fn new<'a>(buf: &'a [u8]) -> IterableVersion<'a> {
        IterableVersion::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Major",
            2u16 => "Minor",
            3u16 => "Enabled",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableVersion<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableVersion<'a> {
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
impl<'a> Iterator for IterableVersion<'a> {
    type Item = Result<Version, ErrorContext>;
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
                1u16 => Version::Major({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Version::Minor({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Version::Enabled(()),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Version",
            r#type.and_then(|t| Version::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableVersion<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Version");
        let mut iter = IterableVersion::with_loc(&[], self.orig_loc);
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
                Version::Major(val) => fmt.field("Major", &val),
                Version::Minor(val) => fmt.field("Minor", &val),
                Version::Enabled(val) => fmt.field("Enabled", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableVersion<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Version", offset));
            return (stack, missing_type.and_then(|t| Version::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Version::Major(val) => {
                    if last_off == offset {
                        stack.push(("Major", last_off));
                        break;
                    }
                }
                Version::Minor(val) => {
                    if last_off == offset {
                        stack.push(("Minor", last_off));
                        break;
                    }
                }
                Version::Enabled(val) => {
                    if last_off == offset {
                        stack.push(("Enabled", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Version", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ServerProto<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    Version(IterableVersion<'a>),
}
impl<'a> IterableServerProto<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_version(&self) -> MultiAttrIterable<Self, ServerProto<'a>, IterableVersion<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let ServerProto::Version(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl ServerProto<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableServerProto<'a> {
        IterableServerProto::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Version",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableServerProto<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableServerProto<'a> {
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
impl<'a> Iterator for IterableServerProto<'a> {
    type Item = Result<ServerProto<'a>, ErrorContext>;
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
                1u16 => ServerProto::Version({
                    let res = Some(IterableVersion::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "ServerProto",
            r#type.and_then(|t| ServerProto::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableServerProto<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ServerProto");
        let mut iter = IterableServerProto::with_loc(&[], self.orig_loc);
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
                ServerProto::Version(val) => fmt.field("Version", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableServerProto<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ServerProto", offset));
            return (
                stack,
                missing_type.and_then(|t| ServerProto::attr_from_type(t)),
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
                ServerProto::Version(val) => {
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
            stack.push(("ServerProto", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum Sock<'a> {
    Addr(&'a [u8]),
    TransportName(&'a CStr),
}
impl<'a> IterableSock<'a> {
    pub fn get_addr(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Sock::Addr(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Sock",
            "Addr",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_transport_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Sock::TransportName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Sock",
            "TransportName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Sock<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableSock<'a> {
        IterableSock::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Addr",
            2u16 => "TransportName",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableSock<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSock<'a> {
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
impl<'a> Iterator for IterableSock<'a> {
    type Item = Result<Sock<'a>, ErrorContext>;
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
                1u16 => Sock::Addr({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Sock::TransportName({
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
            "Sock",
            r#type.and_then(|t| Sock::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableSock<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Sock");
        let mut iter = IterableSock::with_loc(&[], self.orig_loc);
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
                Sock::Addr(val) => fmt.field("Addr", &FormatHexdump(val)),
                Sock::TransportName(val) => fmt.field("TransportName", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSock<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Sock", offset));
            return (stack, missing_type.and_then(|t| Sock::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Sock::Addr(val) => {
                    if last_off == offset {
                        stack.push(("Addr", last_off));
                        break;
                    }
                }
                Sock::TransportName(val) => {
                    if last_off == offset {
                        stack.push(("TransportName", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Sock", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ServerSock<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    Addr(IterableSock<'a>),
}
impl<'a> IterableServerSock<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_addr(&self) -> MultiAttrIterable<Self, ServerSock<'a>, IterableSock<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let ServerSock::Addr(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl ServerSock<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableServerSock<'a> {
        IterableServerSock::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Addr",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableServerSock<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableServerSock<'a> {
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
impl<'a> Iterator for IterableServerSock<'a> {
    type Item = Result<ServerSock<'a>, ErrorContext>;
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
                1u16 => ServerSock::Addr({
                    let res = Some(IterableSock::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "ServerSock",
            r#type.and_then(|t| ServerSock::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableServerSock<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ServerSock");
        let mut iter = IterableServerSock::with_loc(&[], self.orig_loc);
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
                ServerSock::Addr(val) => fmt.field("Addr", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableServerSock<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ServerSock", offset));
            return (
                stack,
                missing_type.and_then(|t| ServerSock::attr_from_type(t)),
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
                ServerSock::Addr(val) => {
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
            stack.push(("ServerSock", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum PoolMode<'a> {
    Mode(&'a CStr),
    Npools(u32),
}
impl<'a> IterablePoolMode<'a> {
    pub fn get_mode(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PoolMode::Mode(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PoolMode",
            "Mode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_npools(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PoolMode::Npools(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PoolMode",
            "Npools",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl PoolMode<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePoolMode<'a> {
        IterablePoolMode::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Mode",
            2u16 => "Npools",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePoolMode<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePoolMode<'a> {
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
impl<'a> Iterator for IterablePoolMode<'a> {
    type Item = Result<PoolMode<'a>, ErrorContext>;
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
                1u16 => PoolMode::Mode({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => PoolMode::Npools({
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
            "PoolMode",
            r#type.and_then(|t| PoolMode::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterablePoolMode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("PoolMode");
        let mut iter = IterablePoolMode::with_loc(&[], self.orig_loc);
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
                PoolMode::Mode(val) => fmt.field("Mode", &val),
                PoolMode::Npools(val) => fmt.field("Npools", &val),
            };
        }
        fmt.finish()
    }
}
impl IterablePoolMode<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("PoolMode", offset));
            return (
                stack,
                missing_type.and_then(|t| PoolMode::attr_from_type(t)),
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
                PoolMode::Mode(val) => {
                    if last_off == offset {
                        stack.push(("Mode", last_off));
                        break;
                    }
                }
                PoolMode::Npools(val) => {
                    if last_off == offset {
                        stack.push(("Npools", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("PoolMode", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum Fslocation<'a> {
    Host(&'a CStr),
    Path(&'a CStr),
}
impl<'a> IterableFslocation<'a> {
    pub fn get_host(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Fslocation::Host(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Fslocation",
            "Host",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_path(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Fslocation::Path(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Fslocation",
            "Path",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Fslocation<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableFslocation<'a> {
        IterableFslocation::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Host",
            2u16 => "Path",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableFslocation<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableFslocation<'a> {
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
impl<'a> Iterator for IterableFslocation<'a> {
    type Item = Result<Fslocation<'a>, ErrorContext>;
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
                1u16 => Fslocation::Host({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Fslocation::Path({
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
            "Fslocation",
            r#type.and_then(|t| Fslocation::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableFslocation<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Fslocation");
        let mut iter = IterableFslocation::with_loc(&[], self.orig_loc);
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
                Fslocation::Host(val) => fmt.field("Host", &val),
                Fslocation::Path(val) => fmt.field("Path", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableFslocation<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Fslocation", offset));
            return (
                stack,
                missing_type.and_then(|t| Fslocation::attr_from_type(t)),
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
                Fslocation::Host(val) => {
                    if last_off == offset {
                        stack.push(("Host", last_off));
                        break;
                    }
                }
                Fslocation::Path(val) => {
                    if last_off == offset {
                        stack.push(("Path", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Fslocation", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum Fslocations<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    Location(IterableFslocation<'a>),
}
impl<'a> IterableFslocations<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_location(&self) -> MultiAttrIterable<Self, Fslocations<'a>, IterableFslocation<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Fslocations::Location(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl Fslocations<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableFslocations<'a> {
        IterableFslocations::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Location",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableFslocations<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableFslocations<'a> {
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
impl<'a> Iterator for IterableFslocations<'a> {
    type Item = Result<Fslocations<'a>, ErrorContext>;
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
                1u16 => Fslocations::Location({
                    let res = Some(IterableFslocation::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Fslocations",
            r#type.and_then(|t| Fslocations::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableFslocations<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Fslocations");
        let mut iter = IterableFslocations::with_loc(&[], self.orig_loc);
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
                Fslocations::Location(val) => fmt.field("Location", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableFslocations<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Fslocations", offset));
            return (
                stack,
                missing_type.and_then(|t| Fslocations::attr_from_type(t)),
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
                Fslocations::Location(val) => {
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
            stack.push(("Fslocations", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum AuthFlavor {
    Pseudoflavor(u32),
    #[doc = "Associated type: [`ExportFlags`] (1 bit per enumeration)"]
    Flags(u32),
}
impl<'a> IterableAuthFlavor<'a> {
    pub fn get_pseudoflavor(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AuthFlavor::Pseudoflavor(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AuthFlavor",
            "Pseudoflavor",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`ExportFlags`] (1 bit per enumeration)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AuthFlavor::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AuthFlavor",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl AuthFlavor {
    pub fn new<'a>(buf: &'a [u8]) -> IterableAuthFlavor<'a> {
        IterableAuthFlavor::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Pseudoflavor",
            2u16 => "Flags",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableAuthFlavor<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableAuthFlavor<'a> {
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
impl<'a> Iterator for IterableAuthFlavor<'a> {
    type Item = Result<AuthFlavor, ErrorContext>;
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
                1u16 => AuthFlavor::Pseudoflavor({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => AuthFlavor::Flags({
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
            "AuthFlavor",
            r#type.and_then(|t| AuthFlavor::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableAuthFlavor<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("AuthFlavor");
        let mut iter = IterableAuthFlavor::with_loc(&[], self.orig_loc);
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
                AuthFlavor::Pseudoflavor(val) => fmt.field("Pseudoflavor", &val),
                AuthFlavor::Flags(val) => {
                    fmt.field("Flags", &FormatFlags(val.into(), ExportFlags::from_value))
                }
            };
        }
        fmt.finish()
    }
}
impl IterableAuthFlavor<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("AuthFlavor", offset));
            return (
                stack,
                missing_type.and_then(|t| AuthFlavor::attr_from_type(t)),
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
                AuthFlavor::Pseudoflavor(val) => {
                    if last_off == offset {
                        stack.push(("Pseudoflavor", last_off));
                        break;
                    }
                }
                AuthFlavor::Flags(val) => {
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
            stack.push(("AuthFlavor", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum SvcExport<'a> {
    Seqno(u64),
    Client(&'a CStr),
    Path(&'a CStr),
    Negative(()),
    Expiry(u64),
    AnonUid(u32),
    AnonGid(u32),
    Fslocations(IterableFslocations<'a>),
    Uuid(&'a [u8]),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    Secinfo(IterableAuthFlavor<'a>),
    #[doc = "Associated type: [`XprtsecMode`] (enum)\nAttribute may repeat multiple times (treat it as array)"]
    Xprtsec(u32),
    #[doc = "Associated type: [`ExportFlags`] (1 bit per enumeration)"]
    Flags(u32),
    Fsid(i32),
}
impl<'a> IterableSvcExport<'a> {
    pub fn get_seqno(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SvcExport::Seqno(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SvcExport",
            "Seqno",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_client(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SvcExport::Client(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SvcExport",
            "Client",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_path(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SvcExport::Path(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SvcExport",
            "Path",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_negative(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SvcExport::Negative(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SvcExport",
            "Negative",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_expiry(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SvcExport::Expiry(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SvcExport",
            "Expiry",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_anon_uid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SvcExport::AnonUid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SvcExport",
            "AnonUid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_anon_gid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SvcExport::AnonGid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SvcExport",
            "AnonGid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fslocations(&self) -> Result<IterableFslocations<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SvcExport::Fslocations(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SvcExport",
            "Fslocations",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_uuid(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SvcExport::Uuid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SvcExport",
            "Uuid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_secinfo(&self) -> MultiAttrIterable<Self, SvcExport<'a>, IterableAuthFlavor<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let SvcExport::Secinfo(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    #[doc = "Associated type: [`XprtsecMode`] (enum)\nAttribute may repeat multiple times (treat it as array)"]
    pub fn get_xprtsec(&self) -> MultiAttrIterable<Self, SvcExport<'a>, u32> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let SvcExport::Xprtsec(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    #[doc = "Associated type: [`ExportFlags`] (1 bit per enumeration)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SvcExport::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SvcExport",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fsid(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SvcExport::Fsid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SvcExport",
            "Fsid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl SvcExport<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableSvcExport<'a> {
        IterableSvcExport::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Seqno",
            2u16 => "Client",
            3u16 => "Path",
            4u16 => "Negative",
            5u16 => "Expiry",
            6u16 => "AnonUid",
            7u16 => "AnonGid",
            8u16 => "Fslocations",
            9u16 => "Uuid",
            10u16 => "Secinfo",
            11u16 => "Xprtsec",
            12u16 => "Flags",
            13u16 => "Fsid",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableSvcExport<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSvcExport<'a> {
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
impl<'a> Iterator for IterableSvcExport<'a> {
    type Item = Result<SvcExport<'a>, ErrorContext>;
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
                1u16 => SvcExport::Seqno({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => SvcExport::Client({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => SvcExport::Path({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => SvcExport::Negative(()),
                5u16 => SvcExport::Expiry({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => SvcExport::AnonUid({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => SvcExport::AnonGid({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => SvcExport::Fslocations({
                    let res = Some(IterableFslocations::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => SvcExport::Uuid({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => SvcExport::Secinfo({
                    let res = Some(IterableAuthFlavor::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => SvcExport::Xprtsec({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => SvcExport::Flags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => SvcExport::Fsid({
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
            "SvcExport",
            r#type.and_then(|t| SvcExport::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableSvcExport<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("SvcExport");
        let mut iter = IterableSvcExport::with_loc(&[], self.orig_loc);
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
                SvcExport::Seqno(val) => fmt.field("Seqno", &val),
                SvcExport::Client(val) => fmt.field("Client", &val),
                SvcExport::Path(val) => fmt.field("Path", &val),
                SvcExport::Negative(val) => fmt.field("Negative", &val),
                SvcExport::Expiry(val) => fmt.field("Expiry", &val),
                SvcExport::AnonUid(val) => fmt.field("AnonUid", &val),
                SvcExport::AnonGid(val) => fmt.field("AnonGid", &val),
                SvcExport::Fslocations(val) => fmt.field("Fslocations", &val),
                SvcExport::Uuid(val) => fmt.field("Uuid", &FormatHexdump(val)),
                SvcExport::Secinfo(val) => fmt.field("Secinfo", &val),
                SvcExport::Xprtsec(val) => {
                    fmt.field("Xprtsec", &FormatFlags(val.into(), XprtsecMode::from_value))
                }
                SvcExport::Flags(val) => {
                    fmt.field("Flags", &FormatFlags(val.into(), ExportFlags::from_value))
                }
                SvcExport::Fsid(val) => fmt.field("Fsid", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSvcExport<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("SvcExport", offset));
            return (
                stack,
                missing_type.and_then(|t| SvcExport::attr_from_type(t)),
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
                SvcExport::Seqno(val) => {
                    if last_off == offset {
                        stack.push(("Seqno", last_off));
                        break;
                    }
                }
                SvcExport::Client(val) => {
                    if last_off == offset {
                        stack.push(("Client", last_off));
                        break;
                    }
                }
                SvcExport::Path(val) => {
                    if last_off == offset {
                        stack.push(("Path", last_off));
                        break;
                    }
                }
                SvcExport::Negative(val) => {
                    if last_off == offset {
                        stack.push(("Negative", last_off));
                        break;
                    }
                }
                SvcExport::Expiry(val) => {
                    if last_off == offset {
                        stack.push(("Expiry", last_off));
                        break;
                    }
                }
                SvcExport::AnonUid(val) => {
                    if last_off == offset {
                        stack.push(("AnonUid", last_off));
                        break;
                    }
                }
                SvcExport::AnonGid(val) => {
                    if last_off == offset {
                        stack.push(("AnonGid", last_off));
                        break;
                    }
                }
                SvcExport::Fslocations(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                SvcExport::Uuid(val) => {
                    if last_off == offset {
                        stack.push(("Uuid", last_off));
                        break;
                    }
                }
                SvcExport::Secinfo(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                SvcExport::Xprtsec(val) => {
                    if last_off == offset {
                        stack.push(("Xprtsec", last_off));
                        break;
                    }
                }
                SvcExport::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                SvcExport::Fsid(val) => {
                    if last_off == offset {
                        stack.push(("Fsid", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("SvcExport", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum SvcExportReqs<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    Requests(IterableSvcExport<'a>),
}
impl<'a> IterableSvcExportReqs<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_requests(
        &self,
    ) -> MultiAttrIterable<Self, SvcExportReqs<'a>, IterableSvcExport<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let SvcExportReqs::Requests(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl SvcExportReqs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableSvcExportReqs<'a> {
        IterableSvcExportReqs::with_loc(buf, buf.as_ptr() as usize)
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
pub struct IterableSvcExportReqs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSvcExportReqs<'a> {
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
impl<'a> Iterator for IterableSvcExportReqs<'a> {
    type Item = Result<SvcExportReqs<'a>, ErrorContext>;
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
                1u16 => SvcExportReqs::Requests({
                    let res = Some(IterableSvcExport::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "SvcExportReqs",
            r#type.and_then(|t| SvcExportReqs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableSvcExportReqs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("SvcExportReqs");
        let mut iter = IterableSvcExportReqs::with_loc(&[], self.orig_loc);
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
                SvcExportReqs::Requests(val) => fmt.field("Requests", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSvcExportReqs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("SvcExportReqs", offset));
            return (
                stack,
                missing_type.and_then(|t| SvcExportReqs::attr_from_type(t)),
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
                SvcExportReqs::Requests(val) => {
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
            stack.push(("SvcExportReqs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum Expkey<'a> {
    Seqno(u64),
    Client(&'a CStr),
    Fsidtype(u8),
    Fsid(&'a [u8]),
    Negative(()),
    Expiry(u64),
    Path(&'a CStr),
}
impl<'a> IterableExpkey<'a> {
    pub fn get_seqno(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Expkey::Seqno(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Expkey",
            "Seqno",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_client(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Expkey::Client(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Expkey",
            "Client",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fsidtype(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Expkey::Fsidtype(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Expkey",
            "Fsidtype",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fsid(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Expkey::Fsid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Expkey",
            "Fsid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_negative(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Expkey::Negative(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Expkey",
            "Negative",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_expiry(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Expkey::Expiry(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Expkey",
            "Expiry",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_path(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Expkey::Path(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Expkey",
            "Path",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Expkey<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableExpkey<'a> {
        IterableExpkey::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Seqno",
            2u16 => "Client",
            3u16 => "Fsidtype",
            4u16 => "Fsid",
            5u16 => "Negative",
            6u16 => "Expiry",
            7u16 => "Path",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableExpkey<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableExpkey<'a> {
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
impl<'a> Iterator for IterableExpkey<'a> {
    type Item = Result<Expkey<'a>, ErrorContext>;
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
                1u16 => Expkey::Seqno({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Expkey::Client({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Expkey::Fsidtype({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Expkey::Fsid({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Expkey::Negative(()),
                6u16 => Expkey::Expiry({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => Expkey::Path({
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
            "Expkey",
            r#type.and_then(|t| Expkey::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableExpkey<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Expkey");
        let mut iter = IterableExpkey::with_loc(&[], self.orig_loc);
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
                Expkey::Seqno(val) => fmt.field("Seqno", &val),
                Expkey::Client(val) => fmt.field("Client", &val),
                Expkey::Fsidtype(val) => fmt.field("Fsidtype", &val),
                Expkey::Fsid(val) => fmt.field("Fsid", &FormatHexdump(val)),
                Expkey::Negative(val) => fmt.field("Negative", &val),
                Expkey::Expiry(val) => fmt.field("Expiry", &val),
                Expkey::Path(val) => fmt.field("Path", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableExpkey<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Expkey", offset));
            return (stack, missing_type.and_then(|t| Expkey::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Expkey::Seqno(val) => {
                    if last_off == offset {
                        stack.push(("Seqno", last_off));
                        break;
                    }
                }
                Expkey::Client(val) => {
                    if last_off == offset {
                        stack.push(("Client", last_off));
                        break;
                    }
                }
                Expkey::Fsidtype(val) => {
                    if last_off == offset {
                        stack.push(("Fsidtype", last_off));
                        break;
                    }
                }
                Expkey::Fsid(val) => {
                    if last_off == offset {
                        stack.push(("Fsid", last_off));
                        break;
                    }
                }
                Expkey::Negative(val) => {
                    if last_off == offset {
                        stack.push(("Negative", last_off));
                        break;
                    }
                }
                Expkey::Expiry(val) => {
                    if last_off == offset {
                        stack.push(("Expiry", last_off));
                        break;
                    }
                }
                Expkey::Path(val) => {
                    if last_off == offset {
                        stack.push(("Path", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Expkey", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ExpkeyReqs<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    Requests(IterableExpkey<'a>),
}
impl<'a> IterableExpkeyReqs<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_requests(&self) -> MultiAttrIterable<Self, ExpkeyReqs<'a>, IterableExpkey<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let ExpkeyReqs::Requests(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl ExpkeyReqs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableExpkeyReqs<'a> {
        IterableExpkeyReqs::with_loc(buf, buf.as_ptr() as usize)
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
pub struct IterableExpkeyReqs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableExpkeyReqs<'a> {
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
impl<'a> Iterator for IterableExpkeyReqs<'a> {
    type Item = Result<ExpkeyReqs<'a>, ErrorContext>;
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
                1u16 => ExpkeyReqs::Requests({
                    let res = Some(IterableExpkey::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "ExpkeyReqs",
            r#type.and_then(|t| ExpkeyReqs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableExpkeyReqs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ExpkeyReqs");
        let mut iter = IterableExpkeyReqs::with_loc(&[], self.orig_loc);
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
                ExpkeyReqs::Requests(val) => fmt.field("Requests", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableExpkeyReqs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ExpkeyReqs", offset));
            return (
                stack,
                missing_type.and_then(|t| ExpkeyReqs::attr_from_type(t)),
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
                ExpkeyReqs::Requests(val) => {
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
            stack.push(("ExpkeyReqs", cur));
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
#[derive(Clone)]
pub enum UnlockIp<'a> {
    #[doc = "struct sockaddr_in or struct sockaddr_in6.\n"]
    Address(&'a [u8]),
}
impl<'a> IterableUnlockIp<'a> {
    #[doc = "struct sockaddr_in or struct sockaddr_in6.\n"]
    pub fn get_address(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(UnlockIp::Address(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "UnlockIp",
            "Address",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl UnlockIp<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableUnlockIp<'a> {
        IterableUnlockIp::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Address",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableUnlockIp<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableUnlockIp<'a> {
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
impl<'a> Iterator for IterableUnlockIp<'a> {
    type Item = Result<UnlockIp<'a>, ErrorContext>;
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
                1u16 => UnlockIp::Address({
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
            "UnlockIp",
            r#type.and_then(|t| UnlockIp::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableUnlockIp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("UnlockIp");
        let mut iter = IterableUnlockIp::with_loc(&[], self.orig_loc);
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
                UnlockIp::Address(val) => fmt.field("Address", &FormatHexdump(val)),
            };
        }
        fmt.finish()
    }
}
impl IterableUnlockIp<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("UnlockIp", offset));
            return (
                stack,
                missing_type.and_then(|t| UnlockIp::attr_from_type(t)),
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
                UnlockIp::Address(val) => {
                    if last_off == offset {
                        stack.push(("Address", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("UnlockIp", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum UnlockFilesystem<'a> {
    #[doc = "Filesystem path whose state should be released.\n"]
    Path(&'a CStr),
}
impl<'a> IterableUnlockFilesystem<'a> {
    #[doc = "Filesystem path whose state should be released.\n"]
    pub fn get_path(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(UnlockFilesystem::Path(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "UnlockFilesystem",
            "Path",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl UnlockFilesystem<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableUnlockFilesystem<'a> {
        IterableUnlockFilesystem::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Path",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableUnlockFilesystem<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableUnlockFilesystem<'a> {
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
impl<'a> Iterator for IterableUnlockFilesystem<'a> {
    type Item = Result<UnlockFilesystem<'a>, ErrorContext>;
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
                1u16 => UnlockFilesystem::Path({
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
            "UnlockFilesystem",
            r#type.and_then(|t| UnlockFilesystem::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableUnlockFilesystem<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("UnlockFilesystem");
        let mut iter = IterableUnlockFilesystem::with_loc(&[], self.orig_loc);
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
                UnlockFilesystem::Path(val) => fmt.field("Path", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableUnlockFilesystem<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("UnlockFilesystem", offset));
            return (
                stack,
                missing_type.and_then(|t| UnlockFilesystem::attr_from_type(t)),
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
                UnlockFilesystem::Path(val) => {
                    if last_off == offset {
                        stack.push(("Path", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("UnlockFilesystem", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum UnlockExport<'a> {
    #[doc = "Export path whose NFSv4 state should be revoked. All state (opens,\nlocks, delegations, layouts) acquired through any export of this path is\nrevoked, regardless of which client holds the state. Intended for use\nafter all clients have been unexported from a given path, enabling the\nunderlying filesystem to be unmounted.\n"]
    Path(&'a CStr),
}
impl<'a> IterableUnlockExport<'a> {
    #[doc = "Export path whose NFSv4 state should be revoked. All state (opens,\nlocks, delegations, layouts) acquired through any export of this path is\nrevoked, regardless of which client holds the state. Intended for use\nafter all clients have been unexported from a given path, enabling the\nunderlying filesystem to be unmounted.\n"]
    pub fn get_path(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(UnlockExport::Path(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "UnlockExport",
            "Path",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl UnlockExport<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableUnlockExport<'a> {
        IterableUnlockExport::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Path",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableUnlockExport<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableUnlockExport<'a> {
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
impl<'a> Iterator for IterableUnlockExport<'a> {
    type Item = Result<UnlockExport<'a>, ErrorContext>;
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
                1u16 => UnlockExport::Path({
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
            "UnlockExport",
            r#type.and_then(|t| UnlockExport::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableUnlockExport<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("UnlockExport");
        let mut iter = IterableUnlockExport::with_loc(&[], self.orig_loc);
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
                UnlockExport::Path(val) => fmt.field("Path", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableUnlockExport<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("UnlockExport", offset));
            return (
                stack,
                missing_type.and_then(|t| UnlockExport::attr_from_type(t)),
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
                UnlockExport::Path(val) => {
                    if last_off == offset {
                        stack.push(("Path", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("UnlockExport", cur));
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
pub struct PushRpcStatus<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushRpcStatus<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushRpcStatus<Prev> {
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
    pub fn push_xid(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_prog(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_version(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 4u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_proc(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_service_time(mut self, value: i64) -> Self {
        push_header(self.as_vec_mut(), 6u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 7u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_saddr4(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    pub fn push_daddr4(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    pub fn push_saddr6(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 10u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_daddr6(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 11u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_sport(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 12u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_dport(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 13u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn push_compound_ops(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 14u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushRpcStatus<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushServer<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushServer<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushServer<Prev> {
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
    pub fn push_threads(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_gracetime(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_leasetime(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_scope(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            4u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_scope_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 4u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_min_threads(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fh_key(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 6u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushServer<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushVersion<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushVersion<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushVersion<Prev> {
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
    pub fn push_major(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_minor(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_enabled(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 3u16, 0 as u16);
        self
    }
}
impl<Prev: Pusher> Drop for PushVersion<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushServerProto<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushServerProto<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushServerProto<Prev> {
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
    pub fn nested_version(mut self) -> PushVersion<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushVersion {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushServerProto<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSock<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushSock<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushSock<Prev> {
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
    pub fn push_addr(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_transport_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_transport_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
}
impl<Prev: Pusher> Drop for PushSock<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushServerSock<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushServerSock<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushServerSock<Prev> {
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
    pub fn nested_addr(mut self) -> PushSock<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushSock {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushServerSock<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPoolMode<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushPoolMode<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushPoolMode<Prev> {
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
    pub fn push_mode(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_mode_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_npools(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushPoolMode<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushFslocation<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushFslocation<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushFslocation<Prev> {
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
    pub fn push_host(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_host_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_path(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_path_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
}
impl<Prev: Pusher> Drop for PushFslocation<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushFslocations<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushFslocations<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushFslocations<Prev> {
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
    pub fn nested_location(mut self) -> PushFslocation<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushFslocation {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushFslocations<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushAuthFlavor<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushAuthFlavor<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushAuthFlavor<Prev> {
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
    pub fn push_pseudoflavor(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`ExportFlags`] (1 bit per enumeration)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushAuthFlavor<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSvcExport<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushSvcExport<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushSvcExport<Prev> {
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
    pub fn push_client(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_client_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_path(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            3u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_path_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
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
    pub fn push_anon_uid(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_anon_gid(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_fslocations(mut self) -> PushFslocations<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 8u16);
        PushFslocations {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_uuid(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 9u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_secinfo(mut self) -> PushAuthFlavor<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 10u16);
        PushAuthFlavor {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Associated type: [`XprtsecMode`] (enum)\nAttribute may repeat multiple times (treat it as array)"]
    pub fn push_xprtsec(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 11u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`ExportFlags`] (1 bit per enumeration)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 12u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fsid(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 13u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushSvcExport<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSvcExportReqs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushSvcExportReqs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushSvcExportReqs<Prev> {
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
    pub fn nested_requests(mut self) -> PushSvcExport<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushSvcExport {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushSvcExportReqs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushExpkey<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushExpkey<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushExpkey<Prev> {
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
    pub fn push_client(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_client_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_fsidtype(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 3u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fsid(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 4u16, value.len() as u16);
        self.as_vec_mut().extend(value);
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
    pub fn push_path(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            7u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_path_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 7u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
}
impl<Prev: Pusher> Drop for PushExpkey<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushExpkeyReqs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushExpkeyReqs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushExpkeyReqs<Prev> {
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
    pub fn nested_requests(mut self) -> PushExpkey<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushExpkey {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushExpkeyReqs<Prev> {
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
pub struct PushUnlockIp<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushUnlockIp<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushUnlockIp<Prev> {
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
    #[doc = "struct sockaddr_in or struct sockaddr_in6.\n"]
    pub fn push_address(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushUnlockIp<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushUnlockFilesystem<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushUnlockFilesystem<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushUnlockFilesystem<Prev> {
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
    #[doc = "Filesystem path whose state should be released.\n"]
    pub fn push_path(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "Filesystem path whose state should be released.\n"]
    pub fn push_path_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
}
impl<Prev: Pusher> Drop for PushUnlockFilesystem<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushUnlockExport<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushUnlockExport<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushUnlockExport<Prev> {
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
    #[doc = "Export path whose NFSv4 state should be revoked. All state (opens,\nlocks, delegations, layouts) acquired through any export of this path is\nrevoked, regardless of which client holds the state. Intended for use\nafter all clients have been unexported from a given path, enabling the\nunderlying filesystem to be unmounted.\n"]
    pub fn push_path(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "Export path whose NFSv4 state should be revoked. All state (opens,\nlocks, delegations, layouts) acquired through any export of this path is\nrevoked, regardless of which client holds the state. Intended for use\nafter all clients have been unexported from a given path, enabling the\nunderlying filesystem to be unmounted.\n"]
    pub fn push_path_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
}
impl<Prev: Pusher> Drop for PushUnlockExport<Prev> {
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
    pub const CMD: u8 = 10u8;
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
#[doc = "dump pending nfsd rpc\n\nReply attributes:\n- [.get_xid()](IterableRpcStatus::get_xid)\n- [.get_flags()](IterableRpcStatus::get_flags)\n- [.get_prog()](IterableRpcStatus::get_prog)\n- [.get_version()](IterableRpcStatus::get_version)\n- [.get_proc()](IterableRpcStatus::get_proc)\n- [.get_service_time()](IterableRpcStatus::get_service_time)\n- [.get_saddr4()](IterableRpcStatus::get_saddr4)\n- [.get_daddr4()](IterableRpcStatus::get_daddr4)\n- [.get_saddr6()](IterableRpcStatus::get_saddr6)\n- [.get_daddr6()](IterableRpcStatus::get_daddr6)\n- [.get_sport()](IterableRpcStatus::get_sport)\n- [.get_dport()](IterableRpcStatus::get_dport)\n- [.get_compound_ops()](IterableRpcStatus::get_compound_ops)\n\n"]
#[derive(Debug)]
pub struct OpRpcStatusGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpRpcStatusGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushRpcStatus<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushRpcStatus::new(buf)
    }
    pub fn encode(&mut self) -> PushRpcStatus<&mut Vec<u8>> {
        PushRpcStatus::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushRpcStatus<RequestBuf<'r>> {
        PushRpcStatus::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableRpcStatus<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableRpcStatus::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 1u8;
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
impl NetlinkRequest for OpRpcStatusGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nfsd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableRpcStatus<'buf>;
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
#[doc = "set the maximum number of running threads\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_threads()](PushServer::push_threads)\n- [.push_gracetime()](PushServer::push_gracetime)\n- [.push_leasetime()](PushServer::push_leasetime)\n- [.push_scope()](PushServer::push_scope)\n- [.push_min_threads()](PushServer::push_min_threads)\n- [.push_fh_key()](PushServer::push_fh_key)\n\n"]
#[derive(Debug)]
pub struct OpThreadsSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpThreadsSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushServer<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushServer::new(buf)
    }
    pub fn encode(&mut self) -> PushServer<&mut Vec<u8>> {
        PushServer::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushServer<RequestBuf<'r>> {
        PushServer::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableServer<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableServer::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpThreadsSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nfsd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableServer<'buf>;
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
#[doc = "get the maximum number of running threads\n\nReply attributes:\n- [.get_threads()](IterableServer::get_threads)\n- [.get_gracetime()](IterableServer::get_gracetime)\n- [.get_leasetime()](IterableServer::get_leasetime)\n- [.get_scope()](IterableServer::get_scope)\n- [.get_min_threads()](IterableServer::get_min_threads)\n\n"]
#[derive(Debug)]
pub struct OpThreadsGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpThreadsGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushServer<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushServer::new(buf)
    }
    pub fn encode(&mut self) -> PushServer<&mut Vec<u8>> {
        PushServer::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushServer<RequestBuf<'r>> {
        PushServer::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableServer<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableServer::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpThreadsGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nfsd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableServer<'buf>;
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
#[doc = "set nfs enabled versions\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_version()](PushServerProto::nested_version)\n\n"]
#[derive(Debug)]
pub struct OpVersionSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpVersionSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushServerProto<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushServerProto::new(buf)
    }
    pub fn encode(&mut self) -> PushServerProto<&mut Vec<u8>> {
        PushServerProto::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushServerProto<RequestBuf<'r>> {
        PushServerProto::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableServerProto<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableServerProto::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpVersionSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nfsd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableServerProto<'buf>;
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
#[doc = "get nfs enabled versions\n\nReply attributes:\n- [.get_version()](IterableServerProto::get_version)\n\n"]
#[derive(Debug)]
pub struct OpVersionGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpVersionGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushServerProto<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushServerProto::new(buf)
    }
    pub fn encode(&mut self) -> PushServerProto<&mut Vec<u8>> {
        PushServerProto::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushServerProto<RequestBuf<'r>> {
        PushServerProto::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableServerProto<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableServerProto::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpVersionGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nfsd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableServerProto<'buf>;
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
#[doc = "set nfs running sockets\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_addr()](PushServerSock::nested_addr)\n\n"]
#[derive(Debug)]
pub struct OpListenerSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpListenerSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushServerSock<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushServerSock::new(buf)
    }
    pub fn encode(&mut self) -> PushServerSock<&mut Vec<u8>> {
        PushServerSock::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushServerSock<RequestBuf<'r>> {
        PushServerSock::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableServerSock<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableServerSock::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpListenerSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nfsd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableServerSock<'buf>;
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
#[doc = "get nfs running listeners\n\nReply attributes:\n- [.get_addr()](IterableServerSock::get_addr)\n\n"]
#[derive(Debug)]
pub struct OpListenerGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpListenerGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushServerSock<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushServerSock::new(buf)
    }
    pub fn encode(&mut self) -> PushServerSock<&mut Vec<u8>> {
        PushServerSock::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushServerSock<RequestBuf<'r>> {
        PushServerSock::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableServerSock<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableServerSock::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 7u8;
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
impl NetlinkRequest for OpListenerGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nfsd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableServerSock<'buf>;
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
#[doc = "set the current server pool-mode\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_mode()](PushPoolMode::push_mode)\n\n"]
#[derive(Debug)]
pub struct OpPoolModeSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPoolModeSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushPoolMode<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushPoolMode::new(buf)
    }
    pub fn encode(&mut self) -> PushPoolMode<&mut Vec<u8>> {
        PushPoolMode::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushPoolMode<RequestBuf<'r>> {
        PushPoolMode::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterablePoolMode<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterablePoolMode::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 8u8;
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
impl NetlinkRequest for OpPoolModeSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nfsd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterablePoolMode<'buf>;
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
#[doc = "get info about server pool-mode\n\nReply attributes:\n- [.get_mode()](IterablePoolMode::get_mode)\n- [.get_npools()](IterablePoolMode::get_npools)\n\n"]
#[derive(Debug)]
pub struct OpPoolModeGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPoolModeGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushPoolMode<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushPoolMode::new(buf)
    }
    pub fn encode(&mut self) -> PushPoolMode<&mut Vec<u8>> {
        PushPoolMode::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushPoolMode<RequestBuf<'r>> {
        PushPoolMode::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterablePoolMode<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterablePoolMode::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 9u8;
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
impl NetlinkRequest for OpPoolModeGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nfsd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterablePoolMode<'buf>;
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
#[doc = "Dump all pending svc_export requests\n\nFlags: admin-perm\n\nReply attributes:\n- [.get_requests()](IterableSvcExportReqs::get_requests)\n\n"]
#[derive(Debug)]
pub struct OpSvcExportGetReqsDump<'r> {
    request: Request<'r>,
}
impl<'r> OpSvcExportGetReqsDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushSvcExportReqs<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushSvcExportReqs::new(buf)
    }
    pub fn encode(&mut self) -> PushSvcExportReqs<&mut Vec<u8>> {
        PushSvcExportReqs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushSvcExportReqs<RequestBuf<'r>> {
        PushSvcExportReqs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableSvcExportReqs<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableSvcExportReqs::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 11u8;
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
impl NetlinkRequest for OpSvcExportGetReqsDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nfsd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableSvcExportReqs<'buf>;
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
#[doc = "Respond to one or more svc_export requests\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_requests()](PushSvcExportReqs::nested_requests)\n\n"]
#[derive(Debug)]
pub struct OpSvcExportSetReqsDo<'r> {
    request: Request<'r>,
}
impl<'r> OpSvcExportSetReqsDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushSvcExportReqs<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushSvcExportReqs::new(buf)
    }
    pub fn encode(&mut self) -> PushSvcExportReqs<&mut Vec<u8>> {
        PushSvcExportReqs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushSvcExportReqs<RequestBuf<'r>> {
        PushSvcExportReqs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableSvcExportReqs<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableSvcExportReqs::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 12u8;
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
impl NetlinkRequest for OpSvcExportSetReqsDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nfsd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableSvcExportReqs<'buf>;
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
#[doc = "Dump all pending expkey requests\n\nFlags: admin-perm\n\nReply attributes:\n- [.get_requests()](IterableExpkeyReqs::get_requests)\n\n"]
#[derive(Debug)]
pub struct OpExpkeyGetReqsDump<'r> {
    request: Request<'r>,
}
impl<'r> OpExpkeyGetReqsDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushExpkeyReqs<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushExpkeyReqs::new(buf)
    }
    pub fn encode(&mut self) -> PushExpkeyReqs<&mut Vec<u8>> {
        PushExpkeyReqs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushExpkeyReqs<RequestBuf<'r>> {
        PushExpkeyReqs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableExpkeyReqs<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableExpkeyReqs::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 13u8;
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
impl NetlinkRequest for OpExpkeyGetReqsDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nfsd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableExpkeyReqs<'buf>;
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
#[doc = "Respond to one or more expkey requests\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_requests()](PushExpkeyReqs::nested_requests)\n\n"]
#[derive(Debug)]
pub struct OpExpkeySetReqsDo<'r> {
    request: Request<'r>,
}
impl<'r> OpExpkeySetReqsDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushExpkeyReqs<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushExpkeyReqs::new(buf)
    }
    pub fn encode(&mut self) -> PushExpkeyReqs<&mut Vec<u8>> {
        PushExpkeyReqs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushExpkeyReqs<RequestBuf<'r>> {
        PushExpkeyReqs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableExpkeyReqs<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableExpkeyReqs::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 14u8;
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
impl NetlinkRequest for OpExpkeySetReqsDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nfsd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableExpkeyReqs<'buf>;
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
#[doc = "Flush nfsd caches (svc_export and/or expkey)\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_mask()](PushCacheFlush::push_mask)\n\n"]
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
        header.cmd = 15u8;
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
        Protocol::Generic("nfsd".as_bytes())
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
#[doc = "release NLM locks held by an IP address\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_address()](PushUnlockIp::push_address)\n\n"]
#[derive(Debug)]
pub struct OpUnlockIpDo<'r> {
    request: Request<'r>,
}
impl<'r> OpUnlockIpDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushUnlockIp<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushUnlockIp::new(buf)
    }
    pub fn encode(&mut self) -> PushUnlockIp<&mut Vec<u8>> {
        PushUnlockIp::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushUnlockIp<RequestBuf<'r>> {
        PushUnlockIp::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableUnlockIp<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableUnlockIp::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 16u8;
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
impl NetlinkRequest for OpUnlockIpDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nfsd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableUnlockIp<'buf>;
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
#[doc = "revoke NFS state under a filesystem path\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_path()](PushUnlockFilesystem::push_path)\n\n"]
#[derive(Debug)]
pub struct OpUnlockFilesystemDo<'r> {
    request: Request<'r>,
}
impl<'r> OpUnlockFilesystemDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushUnlockFilesystem<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushUnlockFilesystem::new(buf)
    }
    pub fn encode(&mut self) -> PushUnlockFilesystem<&mut Vec<u8>> {
        PushUnlockFilesystem::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushUnlockFilesystem<RequestBuf<'r>> {
        PushUnlockFilesystem::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableUnlockFilesystem<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableUnlockFilesystem::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 17u8;
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
impl NetlinkRequest for OpUnlockFilesystemDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nfsd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableUnlockFilesystem<'buf>;
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
#[doc = "Revoke NFSv4 state acquired through exports of a given path. Unlike\nunlock-filesystem, which operates at superblock granularity, this\ncommand targets only state associated with a specific export path.\nUserspace (exportfs -u) sends this after removing the last client for a\npath so the underlying filesystem can be unmounted.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_path()](PushUnlockExport::push_path)\n\n"]
#[derive(Debug)]
pub struct OpUnlockExportDo<'r> {
    request: Request<'r>,
}
impl<'r> OpUnlockExportDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushUnlockExport<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushUnlockExport::new(buf)
    }
    pub fn encode(&mut self) -> PushUnlockExport<&mut Vec<u8>> {
        PushUnlockExport::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushUnlockExport<RequestBuf<'r>> {
        PushUnlockExport::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableUnlockExport<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableUnlockExport::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 18u8;
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
impl NetlinkRequest for OpUnlockExportDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nfsd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableUnlockExport<'buf>;
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
    #[doc = "dump pending nfsd rpc\n\nReply attributes:\n- [.get_xid()](IterableRpcStatus::get_xid)\n- [.get_flags()](IterableRpcStatus::get_flags)\n- [.get_prog()](IterableRpcStatus::get_prog)\n- [.get_version()](IterableRpcStatus::get_version)\n- [.get_proc()](IterableRpcStatus::get_proc)\n- [.get_service_time()](IterableRpcStatus::get_service_time)\n- [.get_saddr4()](IterableRpcStatus::get_saddr4)\n- [.get_daddr4()](IterableRpcStatus::get_daddr4)\n- [.get_saddr6()](IterableRpcStatus::get_saddr6)\n- [.get_daddr6()](IterableRpcStatus::get_daddr6)\n- [.get_sport()](IterableRpcStatus::get_sport)\n- [.get_dport()](IterableRpcStatus::get_dport)\n- [.get_compound_ops()](IterableRpcStatus::get_compound_ops)\n\n"]
    pub fn op_rpc_status_get_dump(self) -> OpRpcStatusGetDump<'buf> {
        let mut res = OpRpcStatusGetDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-rpc-status-get-dump",
            OpRpcStatusGetDump::lookup,
        );
        res
    }
    #[doc = "set the maximum number of running threads\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_threads()](PushServer::push_threads)\n- [.push_gracetime()](PushServer::push_gracetime)\n- [.push_leasetime()](PushServer::push_leasetime)\n- [.push_scope()](PushServer::push_scope)\n- [.push_min_threads()](PushServer::push_min_threads)\n- [.push_fh_key()](PushServer::push_fh_key)\n\n"]
    pub fn op_threads_set_do(self) -> OpThreadsSetDo<'buf> {
        let mut res = OpThreadsSetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-threads-set-do", OpThreadsSetDo::lookup);
        res
    }
    #[doc = "get the maximum number of running threads\n\nReply attributes:\n- [.get_threads()](IterableServer::get_threads)\n- [.get_gracetime()](IterableServer::get_gracetime)\n- [.get_leasetime()](IterableServer::get_leasetime)\n- [.get_scope()](IterableServer::get_scope)\n- [.get_min_threads()](IterableServer::get_min_threads)\n\n"]
    pub fn op_threads_get_do(self) -> OpThreadsGetDo<'buf> {
        let mut res = OpThreadsGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-threads-get-do", OpThreadsGetDo::lookup);
        res
    }
    #[doc = "set nfs enabled versions\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_version()](PushServerProto::nested_version)\n\n"]
    pub fn op_version_set_do(self) -> OpVersionSetDo<'buf> {
        let mut res = OpVersionSetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-version-set-do", OpVersionSetDo::lookup);
        res
    }
    #[doc = "get nfs enabled versions\n\nReply attributes:\n- [.get_version()](IterableServerProto::get_version)\n\n"]
    pub fn op_version_get_do(self) -> OpVersionGetDo<'buf> {
        let mut res = OpVersionGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-version-get-do", OpVersionGetDo::lookup);
        res
    }
    #[doc = "set nfs running sockets\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_addr()](PushServerSock::nested_addr)\n\n"]
    pub fn op_listener_set_do(self) -> OpListenerSetDo<'buf> {
        let mut res = OpListenerSetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-listener-set-do",
            OpListenerSetDo::lookup,
        );
        res
    }
    #[doc = "get nfs running listeners\n\nReply attributes:\n- [.get_addr()](IterableServerSock::get_addr)\n\n"]
    pub fn op_listener_get_do(self) -> OpListenerGetDo<'buf> {
        let mut res = OpListenerGetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-listener-get-do",
            OpListenerGetDo::lookup,
        );
        res
    }
    #[doc = "set the current server pool-mode\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_mode()](PushPoolMode::push_mode)\n\n"]
    pub fn op_pool_mode_set_do(self) -> OpPoolModeSetDo<'buf> {
        let mut res = OpPoolModeSetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-pool-mode-set-do",
            OpPoolModeSetDo::lookup,
        );
        res
    }
    #[doc = "get info about server pool-mode\n\nReply attributes:\n- [.get_mode()](IterablePoolMode::get_mode)\n- [.get_npools()](IterablePoolMode::get_npools)\n\n"]
    pub fn op_pool_mode_get_do(self) -> OpPoolModeGetDo<'buf> {
        let mut res = OpPoolModeGetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-pool-mode-get-do",
            OpPoolModeGetDo::lookup,
        );
        res
    }
    #[doc = "Dump all pending svc_export requests\n\nFlags: admin-perm\n\nReply attributes:\n- [.get_requests()](IterableSvcExportReqs::get_requests)\n\n"]
    pub fn op_svc_export_get_reqs_dump(self) -> OpSvcExportGetReqsDump<'buf> {
        let mut res = OpSvcExportGetReqsDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-svc-export-get-reqs-dump",
            OpSvcExportGetReqsDump::lookup,
        );
        res
    }
    #[doc = "Respond to one or more svc_export requests\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_requests()](PushSvcExportReqs::nested_requests)\n\n"]
    pub fn op_svc_export_set_reqs_do(self) -> OpSvcExportSetReqsDo<'buf> {
        let mut res = OpSvcExportSetReqsDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-svc-export-set-reqs-do",
            OpSvcExportSetReqsDo::lookup,
        );
        res
    }
    #[doc = "Dump all pending expkey requests\n\nFlags: admin-perm\n\nReply attributes:\n- [.get_requests()](IterableExpkeyReqs::get_requests)\n\n"]
    pub fn op_expkey_get_reqs_dump(self) -> OpExpkeyGetReqsDump<'buf> {
        let mut res = OpExpkeyGetReqsDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-expkey-get-reqs-dump",
            OpExpkeyGetReqsDump::lookup,
        );
        res
    }
    #[doc = "Respond to one or more expkey requests\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_requests()](PushExpkeyReqs::nested_requests)\n\n"]
    pub fn op_expkey_set_reqs_do(self) -> OpExpkeySetReqsDo<'buf> {
        let mut res = OpExpkeySetReqsDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-expkey-set-reqs-do",
            OpExpkeySetReqsDo::lookup,
        );
        res
    }
    #[doc = "Flush nfsd caches (svc_export and/or expkey)\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_mask()](PushCacheFlush::push_mask)\n\n"]
    pub fn op_cache_flush_do(self) -> OpCacheFlushDo<'buf> {
        let mut res = OpCacheFlushDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-cache-flush-do", OpCacheFlushDo::lookup);
        res
    }
    #[doc = "release NLM locks held by an IP address\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_address()](PushUnlockIp::push_address)\n\n"]
    pub fn op_unlock_ip_do(self) -> OpUnlockIpDo<'buf> {
        let mut res = OpUnlockIpDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-unlock-ip-do", OpUnlockIpDo::lookup);
        res
    }
    #[doc = "revoke NFS state under a filesystem path\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_path()](PushUnlockFilesystem::push_path)\n\n"]
    pub fn op_unlock_filesystem_do(self) -> OpUnlockFilesystemDo<'buf> {
        let mut res = OpUnlockFilesystemDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-unlock-filesystem-do",
            OpUnlockFilesystemDo::lookup,
        );
        res
    }
    #[doc = "Revoke NFSv4 state acquired through exports of a given path. Unlike\nunlock-filesystem, which operates at superblock granularity, this\ncommand targets only state associated with a specific export path.\nUserspace (exportfs -u) sends this after removing the last client for a\npath so the underlying filesystem can be unmounted.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_path()](PushUnlockExport::push_path)\n\n"]
    pub fn op_unlock_export_do(self) -> OpUnlockExportDo<'buf> {
        let mut res = OpUnlockExportDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-unlock-export-do",
            OpUnlockExportDo::lookup,
        );
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableCacheNotify::get_cache_type;
        let _ = IterableExpkeyReqs::get_requests;
        let _ = IterablePoolMode::get_mode;
        let _ = IterablePoolMode::get_npools;
        let _ = IterableRpcStatus::get_compound_ops;
        let _ = IterableRpcStatus::get_daddr4;
        let _ = IterableRpcStatus::get_daddr6;
        let _ = IterableRpcStatus::get_dport;
        let _ = IterableRpcStatus::get_flags;
        let _ = IterableRpcStatus::get_proc;
        let _ = IterableRpcStatus::get_prog;
        let _ = IterableRpcStatus::get_saddr4;
        let _ = IterableRpcStatus::get_saddr6;
        let _ = IterableRpcStatus::get_service_time;
        let _ = IterableRpcStatus::get_sport;
        let _ = IterableRpcStatus::get_version;
        let _ = IterableRpcStatus::get_xid;
        let _ = IterableServer::get_gracetime;
        let _ = IterableServer::get_leasetime;
        let _ = IterableServer::get_min_threads;
        let _ = IterableServer::get_scope;
        let _ = IterableServer::get_threads;
        let _ = IterableServerProto::get_version;
        let _ = IterableServerSock::get_addr;
        let _ = IterableSvcExportReqs::get_requests;
        let _ = OpCacheNotifyNotif;
        let _ = PushCacheFlush::<&mut Vec<u8>>::push_mask;
        let _ = PushExpkeyReqs::<&mut Vec<u8>>::nested_requests;
        let _ = PushPoolMode::<&mut Vec<u8>>::push_mode;
        let _ = PushServer::<&mut Vec<u8>>::push_fh_key;
        let _ = PushServer::<&mut Vec<u8>>::push_gracetime;
        let _ = PushServer::<&mut Vec<u8>>::push_leasetime;
        let _ = PushServer::<&mut Vec<u8>>::push_min_threads;
        let _ = PushServer::<&mut Vec<u8>>::push_scope;
        let _ = PushServer::<&mut Vec<u8>>::push_threads;
        let _ = PushServerProto::<&mut Vec<u8>>::nested_version;
        let _ = PushServerSock::<&mut Vec<u8>>::nested_addr;
        let _ = PushSvcExportReqs::<&mut Vec<u8>>::nested_requests;
        let _ = PushUnlockExport::<&mut Vec<u8>>::push_path;
        let _ = PushUnlockFilesystem::<&mut Vec<u8>>::push_path;
        let _ = PushUnlockIp::<&mut Vec<u8>>::push_address;
    }
}
