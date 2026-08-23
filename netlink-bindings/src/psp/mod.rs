#![doc = "PSP Security Protocol Generic Netlink family.\n"]
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
pub const PROTONAME: &str = "psp";
pub const PROTONAME_CSTR: &CStr = c"psp";
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum Version {
    Hdr0AesGcm128 = 0,
    Hdr0AesGcm256 = 1,
    Hdr0AesGmac128 = 2,
    Hdr0AesGmac256 = 3,
}
impl Version {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Hdr0AesGcm128,
            1 => Self::Hdr0AesGcm256,
            2 => Self::Hdr0AesGmac128,
            3 => Self::Hdr0AesGmac256,
            _ => return None,
        })
    }
}
#[derive(Clone)]
pub enum AssocDevInfo {
    #[doc = "ifindex of an associated network device.\n"]
    Ifindex(u32),
    #[doc = "Network namespace ID of the associated device.\n"]
    Nsid(i32),
}
impl<'a> IterableAssocDevInfo<'a> {
    #[doc = "ifindex of an associated network device.\n"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AssocDevInfo::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AssocDevInfo",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Network namespace ID of the associated device.\n"]
    pub fn get_nsid(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AssocDevInfo::Nsid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AssocDevInfo",
            "Nsid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl AssocDevInfo {
    pub fn new<'a>(buf: &'a [u8]) -> IterableAssocDevInfo<'a> {
        IterableAssocDevInfo::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Ifindex",
            2u16 => "Nsid",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableAssocDevInfo<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableAssocDevInfo<'a> {
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
impl<'a> Iterator for IterableAssocDevInfo<'a> {
    type Item = Result<AssocDevInfo, ErrorContext>;
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
                1u16 => AssocDevInfo::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => AssocDevInfo::Nsid({
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
            "AssocDevInfo",
            r#type.and_then(|t| AssocDevInfo::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableAssocDevInfo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("AssocDevInfo");
        let mut iter = IterableAssocDevInfo::with_loc(&[], self.orig_loc);
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
                AssocDevInfo::Ifindex(val) => fmt.field("Ifindex", &val),
                AssocDevInfo::Nsid(val) => fmt.field("Nsid", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableAssocDevInfo<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("AssocDevInfo", offset));
            return (
                stack,
                missing_type.and_then(|t| AssocDevInfo::attr_from_type(t)),
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
                AssocDevInfo::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                AssocDevInfo::Nsid(val) => {
                    if last_off == offset {
                        stack.push(("Nsid", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("AssocDevInfo", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum Dev<'a> {
    #[doc = "PSP device ID.\n"]
    Id(u32),
    #[doc = "ifindex of the main netdevice linked to the PSP device, or the ifindex\nto associate with the PSP device.\n"]
    Ifindex(u32),
    #[doc = "Bitmask of PSP versions supported by the device.\n\nAssociated type: [`Version`] (1 bit per enumeration)"]
    PspVersionsCap(u32),
    #[doc = "Bitmask of currently enabled (accepted on Rx) PSP versions.\n\nAssociated type: [`Version`] (1 bit per enumeration)"]
    PspVersionsEna(u32),
    #[doc = "List of associated virtual devices.\n\nAttribute may repeat multiple times (treat it as array)"]
    AssocList(IterableAssocDevInfo<'a>),
    #[doc = "Network namespace ID for the device to associate/disassociate. Optional\nfor dev-assoc and dev-disassoc; if not present, the device is looked up\nin the caller\\'s network namespace.\n"]
    Nsid(i32),
    #[doc = "Flag indicating the PSP device is an associated device from a different\nnetwork namespace. Present when in associated namespace, absent when in\nprimary/host namespace.\n"]
    ByAssociation(()),
}
impl<'a> IterableDev<'a> {
    #[doc = "PSP device ID.\n"]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dev::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dev",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ifindex of the main netdevice linked to the PSP device, or the ifindex\nto associate with the PSP device.\n"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dev::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dev",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Bitmask of PSP versions supported by the device.\n\nAssociated type: [`Version`] (1 bit per enumeration)"]
    pub fn get_psp_versions_cap(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dev::PspVersionsCap(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dev",
            "PspVersionsCap",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Bitmask of currently enabled (accepted on Rx) PSP versions.\n\nAssociated type: [`Version`] (1 bit per enumeration)"]
    pub fn get_psp_versions_ena(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dev::PspVersionsEna(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dev",
            "PspVersionsEna",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "List of associated virtual devices.\n\nAttribute may repeat multiple times (treat it as array)"]
    pub fn get_assoc_list(&self) -> MultiAttrIterable<Self, Dev<'a>, IterableAssocDevInfo<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Dev::AssocList(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    #[doc = "Network namespace ID for the device to associate/disassociate. Optional\nfor dev-assoc and dev-disassoc; if not present, the device is looked up\nin the caller\\'s network namespace.\n"]
    pub fn get_nsid(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dev::Nsid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dev",
            "Nsid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Flag indicating the PSP device is an associated device from a different\nnetwork namespace. Present when in associated namespace, absent when in\nprimary/host namespace.\n"]
    pub fn get_by_association(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dev::ByAssociation(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dev",
            "ByAssociation",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Dev<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDev<'a> {
        IterableDev::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Id",
            2u16 => "Ifindex",
            3u16 => "PspVersionsCap",
            4u16 => "PspVersionsEna",
            5u16 => "AssocList",
            6u16 => "Nsid",
            7u16 => "ByAssociation",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDev<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDev<'a> {
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
impl<'a> Iterator for IterableDev<'a> {
    type Item = Result<Dev<'a>, ErrorContext>;
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
                1u16 => Dev::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Dev::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Dev::PspVersionsCap({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Dev::PspVersionsEna({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Dev::AssocList({
                    let res = Some(IterableAssocDevInfo::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Dev::Nsid({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => Dev::ByAssociation(()),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Dev",
            r#type.and_then(|t| Dev::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDev<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Dev");
        let mut iter = IterableDev::with_loc(&[], self.orig_loc);
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
                Dev::Id(val) => fmt.field("Id", &val),
                Dev::Ifindex(val) => fmt.field("Ifindex", &val),
                Dev::PspVersionsCap(val) => fmt.field(
                    "PspVersionsCap",
                    &FormatFlags(val.into(), |val| {
                        Version::from_value(val.trailing_zeros().into())
                    }),
                ),
                Dev::PspVersionsEna(val) => fmt.field(
                    "PspVersionsEna",
                    &FormatFlags(val.into(), |val| {
                        Version::from_value(val.trailing_zeros().into())
                    }),
                ),
                Dev::AssocList(val) => fmt.field("AssocList", &val),
                Dev::Nsid(val) => fmt.field("Nsid", &val),
                Dev::ByAssociation(val) => fmt.field("ByAssociation", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDev<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Dev", offset));
            return (stack, missing_type.and_then(|t| Dev::attr_from_type(t)));
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
                Dev::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                Dev::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                Dev::PspVersionsCap(val) => {
                    if last_off == offset {
                        stack.push(("PspVersionsCap", last_off));
                        break;
                    }
                }
                Dev::PspVersionsEna(val) => {
                    if last_off == offset {
                        stack.push(("PspVersionsEna", last_off));
                        break;
                    }
                }
                Dev::AssocList(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Dev::Nsid(val) => {
                    if last_off == offset {
                        stack.push(("Nsid", last_off));
                        break;
                    }
                }
                Dev::ByAssociation(val) => {
                    if last_off == offset {
                        stack.push(("ByAssociation", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Dev", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum Assoc<'a> {
    #[doc = "PSP device ID.\n"]
    DevId(u32),
    #[doc = "PSP versions (AEAD and protocol version) used by this association,\ndictates the size of the key.\n\nAssociated type: [`Version`] (enum)"]
    Version(u32),
    RxKey(IterableKeys<'a>),
    TxKey(IterableKeys<'a>),
    #[doc = "Sockets which should be bound to the association immediately.\n"]
    SockFd(u32),
}
impl<'a> IterableAssoc<'a> {
    #[doc = "PSP device ID.\n"]
    pub fn get_dev_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Assoc::DevId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Assoc",
            "DevId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "PSP versions (AEAD and protocol version) used by this association,\ndictates the size of the key.\n\nAssociated type: [`Version`] (enum)"]
    pub fn get_version(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Assoc::Version(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Assoc",
            "Version",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rx_key(&self) -> Result<IterableKeys<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Assoc::RxKey(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Assoc",
            "RxKey",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tx_key(&self) -> Result<IterableKeys<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Assoc::TxKey(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Assoc",
            "TxKey",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Sockets which should be bound to the association immediately.\n"]
    pub fn get_sock_fd(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Assoc::SockFd(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Assoc",
            "SockFd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Assoc<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableAssoc<'a> {
        IterableAssoc::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "DevId",
            2u16 => "Version",
            3u16 => "RxKey",
            4u16 => "TxKey",
            5u16 => "SockFd",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableAssoc<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableAssoc<'a> {
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
impl<'a> Iterator for IterableAssoc<'a> {
    type Item = Result<Assoc<'a>, ErrorContext>;
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
                1u16 => Assoc::DevId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Assoc::Version({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Assoc::RxKey({
                    let res = Some(IterableKeys::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Assoc::TxKey({
                    let res = Some(IterableKeys::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Assoc::SockFd({
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
            "Assoc",
            r#type.and_then(|t| Assoc::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableAssoc<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Assoc");
        let mut iter = IterableAssoc::with_loc(&[], self.orig_loc);
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
                Assoc::DevId(val) => fmt.field("DevId", &val),
                Assoc::Version(val) => {
                    fmt.field("Version", &FormatEnum(val.into(), Version::from_value))
                }
                Assoc::RxKey(val) => fmt.field("RxKey", &val),
                Assoc::TxKey(val) => fmt.field("TxKey", &val),
                Assoc::SockFd(val) => fmt.field("SockFd", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableAssoc<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Assoc", offset));
            return (stack, missing_type.and_then(|t| Assoc::attr_from_type(t)));
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
                Assoc::DevId(val) => {
                    if last_off == offset {
                        stack.push(("DevId", last_off));
                        break;
                    }
                }
                Assoc::Version(val) => {
                    if last_off == offset {
                        stack.push(("Version", last_off));
                        break;
                    }
                }
                Assoc::RxKey(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Assoc::TxKey(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Assoc::SockFd(val) => {
                    if last_off == offset {
                        stack.push(("SockFd", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Assoc", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum Keys<'a> {
    Key(&'a [u8]),
    #[doc = "Security Parameters Index (SPI) of the association.\n"]
    Spi(u32),
}
impl<'a> IterableKeys<'a> {
    pub fn get_key(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Keys::Key(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Keys",
            "Key",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Security Parameters Index (SPI) of the association.\n"]
    pub fn get_spi(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Keys::Spi(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Keys",
            "Spi",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Keys<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableKeys<'a> {
        IterableKeys::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Key",
            2u16 => "Spi",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableKeys<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableKeys<'a> {
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
impl<'a> Iterator for IterableKeys<'a> {
    type Item = Result<Keys<'a>, ErrorContext>;
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
                1u16 => Keys::Key({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Keys::Spi({
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
            "Keys",
            r#type.and_then(|t| Keys::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableKeys<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Keys");
        let mut iter = IterableKeys::with_loc(&[], self.orig_loc);
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
                Keys::Key(val) => fmt.field("Key", &FormatHexdump(val)),
                Keys::Spi(val) => fmt.field("Spi", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableKeys<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Keys", offset));
            return (stack, missing_type.and_then(|t| Keys::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Keys::Key(val) => {
                    if last_off == offset {
                        stack.push(("Key", last_off));
                        break;
                    }
                }
                Keys::Spi(val) => {
                    if last_off == offset {
                        stack.push(("Spi", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Keys", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum Stats {
    #[doc = "PSP device ID.\n"]
    DevId(u32),
    #[doc = "Number of key rotations during the lifetime of the device. Kernel\nstatistic.\n"]
    KeyRotations(u32),
    #[doc = "Number of times a socket\\'s Rx got shut down due to using a key which\nwent stale (fully rotated out). Kernel statistic.\n"]
    StaleEvents(u32),
    #[doc = "Number of successfully processed and authenticated PSP packets. Device\nstatistic (from the PSP spec).\n"]
    RxPackets(u32),
    #[doc = "Number of successfully authenticated PSP bytes received, counting from\nthe first byte after the IV through the last byte of payload. The fixed\ninitial portion of the PSP header (16 bytes) and the PSP trailer/ICV (16\nbytes) are not included in this count. Device statistic (from the PSP\nspec).\n"]
    RxBytes(u32),
    #[doc = "Number of received PSP packets with unsuccessful authentication. Device\nstatistic (from the PSP spec).\n"]
    RxAuthFail(u32),
    #[doc = "Number of received PSP packets with length/framing errors. Device\nstatistic (from the PSP spec).\n"]
    RxError(u32),
    #[doc = "Number of received PSP packets with miscellaneous errors (invalid master\nkey indicated by SPI, unsupported version, etc.) Device statistic (from\nthe PSP spec).\n"]
    RxBad(u32),
    #[doc = "Number of successfully processed PSP packets for transmission. Device\nstatistic (from the PSP spec).\n"]
    TxPackets(u32),
    #[doc = "Number of successfully processed PSP bytes for transmit, counting from\nthe first byte after the IV through the last byte of payload. The fixed\ninitial portion of the PSP header (16 bytes) and the PSP trailer/ICV (16\nbytes) are not included in this count. Device statistic (from the PSP\nspec).\n"]
    TxBytes(u32),
    #[doc = "Number of PSP packets for transmission with errors. Device statistic\n(from the PSP spec).\n"]
    TxError(u32),
}
impl<'a> IterableStats<'a> {
    #[doc = "PSP device ID.\n"]
    pub fn get_dev_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Stats::DevId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Stats",
            "DevId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of key rotations during the lifetime of the device. Kernel\nstatistic.\n"]
    pub fn get_key_rotations(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Stats::KeyRotations(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Stats",
            "KeyRotations",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of times a socket\\'s Rx got shut down due to using a key which\nwent stale (fully rotated out). Kernel statistic.\n"]
    pub fn get_stale_events(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Stats::StaleEvents(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Stats",
            "StaleEvents",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of successfully processed and authenticated PSP packets. Device\nstatistic (from the PSP spec).\n"]
    pub fn get_rx_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Stats::RxPackets(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Stats",
            "RxPackets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of successfully authenticated PSP bytes received, counting from\nthe first byte after the IV through the last byte of payload. The fixed\ninitial portion of the PSP header (16 bytes) and the PSP trailer/ICV (16\nbytes) are not included in this count. Device statistic (from the PSP\nspec).\n"]
    pub fn get_rx_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Stats::RxBytes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Stats",
            "RxBytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of received PSP packets with unsuccessful authentication. Device\nstatistic (from the PSP spec).\n"]
    pub fn get_rx_auth_fail(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Stats::RxAuthFail(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Stats",
            "RxAuthFail",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of received PSP packets with length/framing errors. Device\nstatistic (from the PSP spec).\n"]
    pub fn get_rx_error(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Stats::RxError(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Stats",
            "RxError",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of received PSP packets with miscellaneous errors (invalid master\nkey indicated by SPI, unsupported version, etc.) Device statistic (from\nthe PSP spec).\n"]
    pub fn get_rx_bad(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Stats::RxBad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Stats",
            "RxBad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of successfully processed PSP packets for transmission. Device\nstatistic (from the PSP spec).\n"]
    pub fn get_tx_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Stats::TxPackets(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Stats",
            "TxPackets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of successfully processed PSP bytes for transmit, counting from\nthe first byte after the IV through the last byte of payload. The fixed\ninitial portion of the PSP header (16 bytes) and the PSP trailer/ICV (16\nbytes) are not included in this count. Device statistic (from the PSP\nspec).\n"]
    pub fn get_tx_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Stats::TxBytes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Stats",
            "TxBytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of PSP packets for transmission with errors. Device statistic\n(from the PSP spec).\n"]
    pub fn get_tx_error(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Stats::TxError(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Stats",
            "TxError",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Stats {
    pub fn new<'a>(buf: &'a [u8]) -> IterableStats<'a> {
        IterableStats::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "DevId",
            2u16 => "KeyRotations",
            3u16 => "StaleEvents",
            4u16 => "RxPackets",
            5u16 => "RxBytes",
            6u16 => "RxAuthFail",
            7u16 => "RxError",
            8u16 => "RxBad",
            9u16 => "TxPackets",
            10u16 => "TxBytes",
            11u16 => "TxError",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableStats<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableStats<'a> {
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
impl<'a> Iterator for IterableStats<'a> {
    type Item = Result<Stats, ErrorContext>;
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
                1u16 => Stats::DevId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Stats::KeyRotations({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Stats::StaleEvents({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Stats::RxPackets({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Stats::RxBytes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Stats::RxAuthFail({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => Stats::RxError({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => Stats::RxBad({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => Stats::TxPackets({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => Stats::TxBytes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => Stats::TxError({
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
            "Stats",
            r#type.and_then(|t| Stats::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableStats<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Stats");
        let mut iter = IterableStats::with_loc(&[], self.orig_loc);
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
                Stats::DevId(val) => fmt.field("DevId", &val),
                Stats::KeyRotations(val) => fmt.field("KeyRotations", &val),
                Stats::StaleEvents(val) => fmt.field("StaleEvents", &val),
                Stats::RxPackets(val) => fmt.field("RxPackets", &val),
                Stats::RxBytes(val) => fmt.field("RxBytes", &val),
                Stats::RxAuthFail(val) => fmt.field("RxAuthFail", &val),
                Stats::RxError(val) => fmt.field("RxError", &val),
                Stats::RxBad(val) => fmt.field("RxBad", &val),
                Stats::TxPackets(val) => fmt.field("TxPackets", &val),
                Stats::TxBytes(val) => fmt.field("TxBytes", &val),
                Stats::TxError(val) => fmt.field("TxError", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableStats<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Stats", offset));
            return (stack, missing_type.and_then(|t| Stats::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Stats::DevId(val) => {
                    if last_off == offset {
                        stack.push(("DevId", last_off));
                        break;
                    }
                }
                Stats::KeyRotations(val) => {
                    if last_off == offset {
                        stack.push(("KeyRotations", last_off));
                        break;
                    }
                }
                Stats::StaleEvents(val) => {
                    if last_off == offset {
                        stack.push(("StaleEvents", last_off));
                        break;
                    }
                }
                Stats::RxPackets(val) => {
                    if last_off == offset {
                        stack.push(("RxPackets", last_off));
                        break;
                    }
                }
                Stats::RxBytes(val) => {
                    if last_off == offset {
                        stack.push(("RxBytes", last_off));
                        break;
                    }
                }
                Stats::RxAuthFail(val) => {
                    if last_off == offset {
                        stack.push(("RxAuthFail", last_off));
                        break;
                    }
                }
                Stats::RxError(val) => {
                    if last_off == offset {
                        stack.push(("RxError", last_off));
                        break;
                    }
                }
                Stats::RxBad(val) => {
                    if last_off == offset {
                        stack.push(("RxBad", last_off));
                        break;
                    }
                }
                Stats::TxPackets(val) => {
                    if last_off == offset {
                        stack.push(("TxPackets", last_off));
                        break;
                    }
                }
                Stats::TxBytes(val) => {
                    if last_off == offset {
                        stack.push(("TxBytes", last_off));
                        break;
                    }
                }
                Stats::TxError(val) => {
                    if last_off == offset {
                        stack.push(("TxError", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Stats", cur));
        }
        (stack, None)
    }
}
pub struct PushAssocDevInfo<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushAssocDevInfo<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushAssocDevInfo<Prev> {
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
    #[doc = "ifindex of an associated network device.\n"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Network namespace ID of the associated device.\n"]
    pub fn push_nsid(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushAssocDevInfo<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDev<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDev<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDev<Prev> {
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
    #[doc = "PSP device ID.\n"]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ifindex of the main netdevice linked to the PSP device, or the ifindex\nto associate with the PSP device.\n"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Bitmask of PSP versions supported by the device.\n\nAssociated type: [`Version`] (1 bit per enumeration)"]
    pub fn push_psp_versions_cap(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Bitmask of currently enabled (accepted on Rx) PSP versions.\n\nAssociated type: [`Version`] (1 bit per enumeration)"]
    pub fn push_psp_versions_ena(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "List of associated virtual devices.\n\nAttribute may repeat multiple times (treat it as array)"]
    pub fn nested_assoc_list(mut self) -> PushAssocDevInfo<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 5u16);
        PushAssocDevInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Network namespace ID for the device to associate/disassociate. Optional\nfor dev-assoc and dev-disassoc; if not present, the device is looked up\nin the caller\\'s network namespace.\n"]
    pub fn push_nsid(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Flag indicating the PSP device is an associated device from a different\nnetwork namespace. Present when in associated namespace, absent when in\nprimary/host namespace.\n"]
    pub fn push_by_association(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 7u16, 0 as u16);
        self
    }
}
impl<Prev: Pusher> Drop for PushDev<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushAssoc<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushAssoc<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushAssoc<Prev> {
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
    #[doc = "PSP device ID.\n"]
    pub fn push_dev_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "PSP versions (AEAD and protocol version) used by this association,\ndictates the size of the key.\n\nAssociated type: [`Version`] (enum)"]
    pub fn push_version(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_rx_key(mut self) -> PushKeys<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 3u16);
        PushKeys {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_tx_key(mut self) -> PushKeys<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 4u16);
        PushKeys {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Sockets which should be bound to the association immediately.\n"]
    pub fn push_sock_fd(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushAssoc<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushKeys<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushKeys<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushKeys<Prev> {
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
    pub fn push_key(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "Security Parameters Index (SPI) of the association.\n"]
    pub fn push_spi(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushKeys<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushStats<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushStats<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushStats<Prev> {
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
    #[doc = "PSP device ID.\n"]
    pub fn push_dev_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of key rotations during the lifetime of the device. Kernel\nstatistic.\n"]
    pub fn push_key_rotations(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of times a socket\\'s Rx got shut down due to using a key which\nwent stale (fully rotated out). Kernel statistic.\n"]
    pub fn push_stale_events(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of successfully processed and authenticated PSP packets. Device\nstatistic (from the PSP spec).\n"]
    pub fn push_rx_packets(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of successfully authenticated PSP bytes received, counting from\nthe first byte after the IV through the last byte of payload. The fixed\ninitial portion of the PSP header (16 bytes) and the PSP trailer/ICV (16\nbytes) are not included in this count. Device statistic (from the PSP\nspec).\n"]
    pub fn push_rx_bytes(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of received PSP packets with unsuccessful authentication. Device\nstatistic (from the PSP spec).\n"]
    pub fn push_rx_auth_fail(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of received PSP packets with length/framing errors. Device\nstatistic (from the PSP spec).\n"]
    pub fn push_rx_error(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of received PSP packets with miscellaneous errors (invalid master\nkey indicated by SPI, unsupported version, etc.) Device statistic (from\nthe PSP spec).\n"]
    pub fn push_rx_bad(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of successfully processed PSP packets for transmission. Device\nstatistic (from the PSP spec).\n"]
    pub fn push_tx_packets(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of successfully processed PSP bytes for transmit, counting from\nthe first byte after the IV through the last byte of payload. The fixed\ninitial portion of the PSP header (16 bytes) and the PSP trailer/ICV (16\nbytes) are not included in this count. Device statistic (from the PSP\nspec).\n"]
    pub fn push_tx_bytes(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of PSP packets for transmission with errors. Device statistic\n(from the PSP spec).\n"]
    pub fn push_tx_error(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 11u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushStats<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Notify attributes:\n- [`.get_id()`](IterableDev::get_id)\n- [`.get_ifindex()`](IterableDev::get_ifindex)\n- [`.get_psp_versions_cap()`](IterableDev::get_psp_versions_cap)\n- [`.get_psp_versions_ena()`](IterableDev::get_psp_versions_ena)\n- [`.get_assoc_list()`](IterableDev::get_assoc_list)\n- [`.get_by_association()`](IterableDev::get_by_association)\n"]
#[derive(Debug)]
pub struct OpDevAddNotif;
impl OpDevAddNotif {
    pub const CMD: u8 = 2u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterableDev<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDev::with_loc(attrs, buf.as_ptr() as usize)
    }
}
#[doc = "Notify attributes:\n- [`.get_id()`](IterableDev::get_id)\n- [`.get_ifindex()`](IterableDev::get_ifindex)\n- [`.get_psp_versions_cap()`](IterableDev::get_psp_versions_cap)\n- [`.get_psp_versions_ena()`](IterableDev::get_psp_versions_ena)\n- [`.get_assoc_list()`](IterableDev::get_assoc_list)\n- [`.get_by_association()`](IterableDev::get_by_association)\n"]
#[derive(Debug)]
pub struct OpDevDelNotif;
impl OpDevDelNotif {
    pub const CMD: u8 = 3u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterableDev<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDev::with_loc(attrs, buf.as_ptr() as usize)
    }
}
#[doc = "Notify attributes:\n- [`.get_id()`](IterableDev::get_id)\n- [`.get_ifindex()`](IterableDev::get_ifindex)\n- [`.get_psp_versions_cap()`](IterableDev::get_psp_versions_cap)\n- [`.get_psp_versions_ena()`](IterableDev::get_psp_versions_ena)\n- [`.get_assoc_list()`](IterableDev::get_assoc_list)\n- [`.get_by_association()`](IterableDev::get_by_association)\n"]
#[derive(Debug)]
pub struct OpDevChangeNotif;
impl OpDevChangeNotif {
    pub const CMD: u8 = 5u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterableDev<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDev::with_loc(attrs, buf.as_ptr() as usize)
    }
}
#[doc = "Notify attributes:\n- [`.get_id()`](IterableDev::get_id)\n"]
#[derive(Debug)]
pub struct OpKeyRotateNotif;
impl OpKeyRotateNotif {
    pub const CMD: u8 = 7u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterableDev<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDev::with_loc(attrs, buf.as_ptr() as usize)
    }
}
pub struct NotifGroup;
impl NotifGroup {
    #[doc = "Notifications:\n- [`OpDevAddNotif`]\n- [`OpDevDelNotif`]\n- [`OpDevChangeNotif`]\n"]
    pub const MGMT: &str = "mgmt";
    #[doc = "Notifications:\n- [`OpDevAddNotif`]\n- [`OpDevDelNotif`]\n- [`OpDevChangeNotif`]\n"]
    pub const MGMT_CSTR: &CStr = c"mgmt";
    #[doc = "Notifications:\n- [`OpKeyRotateNotif`]\n"]
    pub const USE: &str = "use";
    #[doc = "Notifications:\n- [`OpKeyRotateNotif`]\n"]
    pub const USE_CSTR: &CStr = c"use";
}
#[doc = "Get / dump information about PSP capable devices on the system.\n\nReply attributes:\n- [.get_id()](IterableDev::get_id)\n- [.get_ifindex()](IterableDev::get_ifindex)\n- [.get_psp_versions_cap()](IterableDev::get_psp_versions_cap)\n- [.get_psp_versions_ena()](IterableDev::get_psp_versions_ena)\n- [.get_assoc_list()](IterableDev::get_assoc_list)\n- [.get_by_association()](IterableDev::get_by_association)\n\n"]
#[derive(Debug)]
pub struct OpDevGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpDevGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDev<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDev::new(buf)
    }
    pub fn encode(&mut self) -> PushDev<&mut Vec<u8>> {
        PushDev::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDev<RequestBuf<'r>> {
        PushDev::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDev<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDev::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpDevGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("psp".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDev<'buf>;
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
#[doc = "Get / dump information about PSP capable devices on the system.\n\nRequest attributes:\n- [.push_id()](PushDev::push_id)\n\nReply attributes:\n- [.get_id()](IterableDev::get_id)\n- [.get_ifindex()](IterableDev::get_ifindex)\n- [.get_psp_versions_cap()](IterableDev::get_psp_versions_cap)\n- [.get_psp_versions_ena()](IterableDev::get_psp_versions_ena)\n- [.get_assoc_list()](IterableDev::get_assoc_list)\n- [.get_by_association()](IterableDev::get_by_association)\n\n"]
#[derive(Debug)]
pub struct OpDevGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDevGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDev<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDev::new(buf)
    }
    pub fn encode(&mut self) -> PushDev<&mut Vec<u8>> {
        PushDev::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDev<RequestBuf<'r>> {
        PushDev::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDev<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDev::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpDevGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("psp".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDev<'buf>;
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
#[doc = "Set the configuration of a PSP device.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_id()](PushDev::push_id)\n- [.push_psp_versions_ena()](PushDev::push_psp_versions_ena)\n\n"]
#[derive(Debug)]
pub struct OpDevSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDevSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDev<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDev::new(buf)
    }
    pub fn encode(&mut self) -> PushDev<&mut Vec<u8>> {
        PushDev::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDev<RequestBuf<'r>> {
        PushDev::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDev<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDev::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpDevSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("psp".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDev<'buf>;
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
#[doc = "Rotate the device key.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_id()](PushDev::push_id)\n\nReply attributes:\n- [.get_id()](IterableDev::get_id)\n\n"]
#[derive(Debug)]
pub struct OpKeyRotateDo<'r> {
    request: Request<'r>,
}
impl<'r> OpKeyRotateDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDev<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDev::new(buf)
    }
    pub fn encode(&mut self) -> PushDev<&mut Vec<u8>> {
        PushDev::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDev<RequestBuf<'r>> {
        PushDev::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDev<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDev::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpKeyRotateDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("psp".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDev<'buf>;
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
#[doc = "Allocate a new Rx key + SPI pair, associate it with a socket.\n\nRequest attributes:\n- [.push_dev_id()](PushAssoc::push_dev_id)\n- [.push_version()](PushAssoc::push_version)\n- [.push_sock_fd()](PushAssoc::push_sock_fd)\n\nReply attributes:\n- [.get_dev_id()](IterableAssoc::get_dev_id)\n- [.get_rx_key()](IterableAssoc::get_rx_key)\n\n"]
#[derive(Debug)]
pub struct OpRxAssocDo<'r> {
    request: Request<'r>,
}
impl<'r> OpRxAssocDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushAssoc<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushAssoc::new(buf)
    }
    pub fn encode(&mut self) -> PushAssoc<&mut Vec<u8>> {
        PushAssoc::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushAssoc<RequestBuf<'r>> {
        PushAssoc::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableAssoc<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableAssoc::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpRxAssocDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("psp".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableAssoc<'buf>;
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
#[doc = "Add a PSP Tx association.\n\nRequest attributes:\n- [.push_dev_id()](PushAssoc::push_dev_id)\n- [.push_version()](PushAssoc::push_version)\n- [.nested_tx_key()](PushAssoc::nested_tx_key)\n- [.push_sock_fd()](PushAssoc::push_sock_fd)\n\n"]
#[derive(Debug)]
pub struct OpTxAssocDo<'r> {
    request: Request<'r>,
}
impl<'r> OpTxAssocDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushAssoc<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushAssoc::new(buf)
    }
    pub fn encode(&mut self) -> PushAssoc<&mut Vec<u8>> {
        PushAssoc::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushAssoc<RequestBuf<'r>> {
        PushAssoc::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableAssoc<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableAssoc::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpTxAssocDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("psp".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableAssoc<'buf>;
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
#[doc = "Get device statistics.\n\nReply attributes:\n- [.get_dev_id()](IterableStats::get_dev_id)\n- [.get_key_rotations()](IterableStats::get_key_rotations)\n- [.get_stale_events()](IterableStats::get_stale_events)\n- [.get_rx_packets()](IterableStats::get_rx_packets)\n- [.get_rx_bytes()](IterableStats::get_rx_bytes)\n- [.get_rx_auth_fail()](IterableStats::get_rx_auth_fail)\n- [.get_rx_error()](IterableStats::get_rx_error)\n- [.get_rx_bad()](IterableStats::get_rx_bad)\n- [.get_tx_packets()](IterableStats::get_tx_packets)\n- [.get_tx_bytes()](IterableStats::get_tx_bytes)\n- [.get_tx_error()](IterableStats::get_tx_error)\n\n"]
#[derive(Debug)]
pub struct OpGetStatsDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetStatsDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushStats<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushStats::new(buf)
    }
    pub fn encode(&mut self) -> PushStats<&mut Vec<u8>> {
        PushStats::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushStats<RequestBuf<'r>> {
        PushStats::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableStats<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableStats::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 10u8;
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
impl NetlinkRequest for OpGetStatsDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("psp".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableStats<'buf>;
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
#[doc = "Get device statistics.\n\nRequest attributes:\n- [.push_dev_id()](PushStats::push_dev_id)\n\nReply attributes:\n- [.get_dev_id()](IterableStats::get_dev_id)\n- [.get_key_rotations()](IterableStats::get_key_rotations)\n- [.get_stale_events()](IterableStats::get_stale_events)\n- [.get_rx_packets()](IterableStats::get_rx_packets)\n- [.get_rx_bytes()](IterableStats::get_rx_bytes)\n- [.get_rx_auth_fail()](IterableStats::get_rx_auth_fail)\n- [.get_rx_error()](IterableStats::get_rx_error)\n- [.get_rx_bad()](IterableStats::get_rx_bad)\n- [.get_tx_packets()](IterableStats::get_tx_packets)\n- [.get_tx_bytes()](IterableStats::get_tx_bytes)\n- [.get_tx_error()](IterableStats::get_tx_error)\n\n"]
#[derive(Debug)]
pub struct OpGetStatsDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetStatsDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushStats<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushStats::new(buf)
    }
    pub fn encode(&mut self) -> PushStats<&mut Vec<u8>> {
        PushStats::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushStats<RequestBuf<'r>> {
        PushStats::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableStats<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableStats::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 10u8;
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
impl NetlinkRequest for OpGetStatsDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("psp".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableStats<'buf>;
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
#[doc = "Associate a network device with a PSP device.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_id()](PushDev::push_id)\n- [.push_ifindex()](PushDev::push_ifindex)\n- [.push_nsid()](PushDev::push_nsid)\n\n"]
#[derive(Debug)]
pub struct OpDevAssocDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDevAssocDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDev<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDev::new(buf)
    }
    pub fn encode(&mut self) -> PushDev<&mut Vec<u8>> {
        PushDev::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDev<RequestBuf<'r>> {
        PushDev::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDev<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDev::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpDevAssocDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("psp".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDev<'buf>;
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
#[doc = "Disassociate a network device from a PSP device.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_id()](PushDev::push_id)\n- [.push_ifindex()](PushDev::push_ifindex)\n- [.push_nsid()](PushDev::push_nsid)\n\n"]
#[derive(Debug)]
pub struct OpDevDisassocDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDevDisassocDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDev<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDev::new(buf)
    }
    pub fn encode(&mut self) -> PushDev<&mut Vec<u8>> {
        PushDev::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDev<RequestBuf<'r>> {
        PushDev::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDev<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDev::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpDevDisassocDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("psp".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDev<'buf>;
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
    #[doc = "Get / dump information about PSP capable devices on the system.\n\nReply attributes:\n- [.get_id()](IterableDev::get_id)\n- [.get_ifindex()](IterableDev::get_ifindex)\n- [.get_psp_versions_cap()](IterableDev::get_psp_versions_cap)\n- [.get_psp_versions_ena()](IterableDev::get_psp_versions_ena)\n- [.get_assoc_list()](IterableDev::get_assoc_list)\n- [.get_by_association()](IterableDev::get_by_association)\n\n"]
    pub fn op_dev_get_dump(self) -> OpDevGetDump<'buf> {
        let mut res = OpDevGetDump::new(self);
        res.request
            .do_writeback(res.protocol(), "op-dev-get-dump", OpDevGetDump::lookup);
        res
    }
    #[doc = "Get / dump information about PSP capable devices on the system.\n\nRequest attributes:\n- [.push_id()](PushDev::push_id)\n\nReply attributes:\n- [.get_id()](IterableDev::get_id)\n- [.get_ifindex()](IterableDev::get_ifindex)\n- [.get_psp_versions_cap()](IterableDev::get_psp_versions_cap)\n- [.get_psp_versions_ena()](IterableDev::get_psp_versions_ena)\n- [.get_assoc_list()](IterableDev::get_assoc_list)\n- [.get_by_association()](IterableDev::get_by_association)\n\n"]
    pub fn op_dev_get_do(self) -> OpDevGetDo<'buf> {
        let mut res = OpDevGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-dev-get-do", OpDevGetDo::lookup);
        res
    }
    #[doc = "Set the configuration of a PSP device.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_id()](PushDev::push_id)\n- [.push_psp_versions_ena()](PushDev::push_psp_versions_ena)\n\n"]
    pub fn op_dev_set_do(self) -> OpDevSetDo<'buf> {
        let mut res = OpDevSetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-dev-set-do", OpDevSetDo::lookup);
        res
    }
    #[doc = "Rotate the device key.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_id()](PushDev::push_id)\n\nReply attributes:\n- [.get_id()](IterableDev::get_id)\n\n"]
    pub fn op_key_rotate_do(self) -> OpKeyRotateDo<'buf> {
        let mut res = OpKeyRotateDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-key-rotate-do", OpKeyRotateDo::lookup);
        res
    }
    #[doc = "Allocate a new Rx key + SPI pair, associate it with a socket.\n\nRequest attributes:\n- [.push_dev_id()](PushAssoc::push_dev_id)\n- [.push_version()](PushAssoc::push_version)\n- [.push_sock_fd()](PushAssoc::push_sock_fd)\n\nReply attributes:\n- [.get_dev_id()](IterableAssoc::get_dev_id)\n- [.get_rx_key()](IterableAssoc::get_rx_key)\n\n"]
    pub fn op_rx_assoc_do(self) -> OpRxAssocDo<'buf> {
        let mut res = OpRxAssocDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-rx-assoc-do", OpRxAssocDo::lookup);
        res
    }
    #[doc = "Add a PSP Tx association.\n\nRequest attributes:\n- [.push_dev_id()](PushAssoc::push_dev_id)\n- [.push_version()](PushAssoc::push_version)\n- [.nested_tx_key()](PushAssoc::nested_tx_key)\n- [.push_sock_fd()](PushAssoc::push_sock_fd)\n\n"]
    pub fn op_tx_assoc_do(self) -> OpTxAssocDo<'buf> {
        let mut res = OpTxAssocDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-tx-assoc-do", OpTxAssocDo::lookup);
        res
    }
    #[doc = "Get device statistics.\n\nReply attributes:\n- [.get_dev_id()](IterableStats::get_dev_id)\n- [.get_key_rotations()](IterableStats::get_key_rotations)\n- [.get_stale_events()](IterableStats::get_stale_events)\n- [.get_rx_packets()](IterableStats::get_rx_packets)\n- [.get_rx_bytes()](IterableStats::get_rx_bytes)\n- [.get_rx_auth_fail()](IterableStats::get_rx_auth_fail)\n- [.get_rx_error()](IterableStats::get_rx_error)\n- [.get_rx_bad()](IterableStats::get_rx_bad)\n- [.get_tx_packets()](IterableStats::get_tx_packets)\n- [.get_tx_bytes()](IterableStats::get_tx_bytes)\n- [.get_tx_error()](IterableStats::get_tx_error)\n\n"]
    pub fn op_get_stats_dump(self) -> OpGetStatsDump<'buf> {
        let mut res = OpGetStatsDump::new(self);
        res.request
            .do_writeback(res.protocol(), "op-get-stats-dump", OpGetStatsDump::lookup);
        res
    }
    #[doc = "Get device statistics.\n\nRequest attributes:\n- [.push_dev_id()](PushStats::push_dev_id)\n\nReply attributes:\n- [.get_dev_id()](IterableStats::get_dev_id)\n- [.get_key_rotations()](IterableStats::get_key_rotations)\n- [.get_stale_events()](IterableStats::get_stale_events)\n- [.get_rx_packets()](IterableStats::get_rx_packets)\n- [.get_rx_bytes()](IterableStats::get_rx_bytes)\n- [.get_rx_auth_fail()](IterableStats::get_rx_auth_fail)\n- [.get_rx_error()](IterableStats::get_rx_error)\n- [.get_rx_bad()](IterableStats::get_rx_bad)\n- [.get_tx_packets()](IterableStats::get_tx_packets)\n- [.get_tx_bytes()](IterableStats::get_tx_bytes)\n- [.get_tx_error()](IterableStats::get_tx_error)\n\n"]
    pub fn op_get_stats_do(self) -> OpGetStatsDo<'buf> {
        let mut res = OpGetStatsDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-get-stats-do", OpGetStatsDo::lookup);
        res
    }
    #[doc = "Associate a network device with a PSP device.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_id()](PushDev::push_id)\n- [.push_ifindex()](PushDev::push_ifindex)\n- [.push_nsid()](PushDev::push_nsid)\n\n"]
    pub fn op_dev_assoc_do(self) -> OpDevAssocDo<'buf> {
        let mut res = OpDevAssocDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-dev-assoc-do", OpDevAssocDo::lookup);
        res
    }
    #[doc = "Disassociate a network device from a PSP device.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_id()](PushDev::push_id)\n- [.push_ifindex()](PushDev::push_ifindex)\n- [.push_nsid()](PushDev::push_nsid)\n\n"]
    pub fn op_dev_disassoc_do(self) -> OpDevDisassocDo<'buf> {
        let mut res = OpDevDisassocDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-dev-disassoc-do",
            OpDevDisassocDo::lookup,
        );
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableAssoc::get_dev_id;
        let _ = IterableAssoc::get_rx_key;
        let _ = IterableDev::get_assoc_list;
        let _ = IterableDev::get_by_association;
        let _ = IterableDev::get_id;
        let _ = IterableDev::get_ifindex;
        let _ = IterableDev::get_psp_versions_cap;
        let _ = IterableDev::get_psp_versions_ena;
        let _ = IterableStats::get_dev_id;
        let _ = IterableStats::get_key_rotations;
        let _ = IterableStats::get_rx_auth_fail;
        let _ = IterableStats::get_rx_bad;
        let _ = IterableStats::get_rx_bytes;
        let _ = IterableStats::get_rx_error;
        let _ = IterableStats::get_rx_packets;
        let _ = IterableStats::get_stale_events;
        let _ = IterableStats::get_tx_bytes;
        let _ = IterableStats::get_tx_error;
        let _ = IterableStats::get_tx_packets;
        let _ = OpDevAddNotif;
        let _ = OpDevChangeNotif;
        let _ = OpDevDelNotif;
        let _ = OpKeyRotateNotif;
        let _ = PushAssoc::<&mut Vec<u8>>::nested_tx_key;
        let _ = PushAssoc::<&mut Vec<u8>>::push_dev_id;
        let _ = PushAssoc::<&mut Vec<u8>>::push_sock_fd;
        let _ = PushAssoc::<&mut Vec<u8>>::push_version;
        let _ = PushDev::<&mut Vec<u8>>::push_id;
        let _ = PushDev::<&mut Vec<u8>>::push_ifindex;
        let _ = PushDev::<&mut Vec<u8>>::push_nsid;
        let _ = PushDev::<&mut Vec<u8>>::push_psp_versions_ena;
        let _ = PushStats::<&mut Vec<u8>>::push_dev_id;
    }
}
