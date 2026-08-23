#![doc = "Netlink protocol to request a transport layer security handshake.\n"]
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
pub const PROTONAME: &str = "handshake";
pub const PROTONAME_CSTR: &CStr = c"handshake";
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum HandlerClass {
    None = 0,
    Tlshd = 1,
    Max = 2,
}
impl HandlerClass {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::None,
            1 => Self::Tlshd,
            2 => Self::Max,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum MsgType {
    Unspec = 0,
    Clienthello = 1,
    Serverhello = 2,
}
impl MsgType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Unspec,
            1 => Self::Clienthello,
            2 => Self::Serverhello,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum Auth {
    Unspec = 0,
    Unauth = 1,
    Psk = 2,
    X509 = 3,
}
impl Auth {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Unspec,
            1 => Self::Unauth,
            2 => Self::Psk,
            3 => Self::X509,
            _ => return None,
        })
    }
}
#[derive(Clone)]
pub enum X509 {
    Cert(i32),
    Privkey(i32),
}
impl<'a> IterableX509<'a> {
    pub fn get_cert(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(X509::Cert(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "X509",
            "Cert",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_privkey(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(X509::Privkey(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "X509",
            "Privkey",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl X509 {
    pub fn new<'a>(buf: &'a [u8]) -> IterableX509<'a> {
        IterableX509::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Cert",
            2u16 => "Privkey",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableX509<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableX509<'a> {
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
impl<'a> Iterator for IterableX509<'a> {
    type Item = Result<X509, ErrorContext>;
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
                1u16 => X509::Cert({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => X509::Privkey({
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
            "X509",
            r#type.and_then(|t| X509::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableX509<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("X509");
        let mut iter = IterableX509::with_loc(&[], self.orig_loc);
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
                X509::Cert(val) => fmt.field("Cert", &val),
                X509::Privkey(val) => fmt.field("Privkey", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableX509<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("X509", offset));
            return (stack, missing_type.and_then(|t| X509::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                X509::Cert(val) => {
                    if last_off == offset {
                        stack.push(("Cert", last_off));
                        break;
                    }
                }
                X509::Privkey(val) => {
                    if last_off == offset {
                        stack.push(("Privkey", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("X509", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum Accept<'a> {
    Sockfd(i32),
    #[doc = "Associated type: [`HandlerClass`] (enum)"]
    HandlerClass(u32),
    #[doc = "Associated type: [`MsgType`] (enum)"]
    MessageType(u32),
    Timeout(u32),
    #[doc = "Associated type: [`Auth`] (enum)"]
    AuthMode(u32),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    PeerIdentity(u32),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    Certificate(IterableX509<'a>),
    Peername(&'a CStr),
    Keyring(u32),
}
impl<'a> IterableAccept<'a> {
    pub fn get_sockfd(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Accept::Sockfd(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Accept",
            "Sockfd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`HandlerClass`] (enum)"]
    pub fn get_handler_class(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Accept::HandlerClass(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Accept",
            "HandlerClass",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`MsgType`] (enum)"]
    pub fn get_message_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Accept::MessageType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Accept",
            "MessageType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_timeout(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Accept::Timeout(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Accept",
            "Timeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`Auth`] (enum)"]
    pub fn get_auth_mode(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Accept::AuthMode(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Accept",
            "AuthMode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_peer_identity(&self) -> MultiAttrIterable<Self, Accept<'a>, u32> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Accept::PeerIdentity(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_certificate(&self) -> MultiAttrIterable<Self, Accept<'a>, IterableX509<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Accept::Certificate(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    pub fn get_peername(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Accept::Peername(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Accept",
            "Peername",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_keyring(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Accept::Keyring(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Accept",
            "Keyring",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Accept<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableAccept<'a> {
        IterableAccept::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Sockfd",
            2u16 => "HandlerClass",
            3u16 => "MessageType",
            4u16 => "Timeout",
            5u16 => "AuthMode",
            6u16 => "PeerIdentity",
            7u16 => "Certificate",
            8u16 => "Peername",
            9u16 => "Keyring",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableAccept<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableAccept<'a> {
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
impl<'a> Iterator for IterableAccept<'a> {
    type Item = Result<Accept<'a>, ErrorContext>;
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
                1u16 => Accept::Sockfd({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Accept::HandlerClass({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Accept::MessageType({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Accept::Timeout({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Accept::AuthMode({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Accept::PeerIdentity({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => Accept::Certificate({
                    let res = Some(IterableX509::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => Accept::Peername({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => Accept::Keyring({
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
            "Accept",
            r#type.and_then(|t| Accept::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableAccept<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Accept");
        let mut iter = IterableAccept::with_loc(&[], self.orig_loc);
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
                Accept::Sockfd(val) => fmt.field("Sockfd", &val),
                Accept::HandlerClass(val) => fmt.field(
                    "HandlerClass",
                    &FormatEnum(val.into(), HandlerClass::from_value),
                ),
                Accept::MessageType(val) => {
                    fmt.field("MessageType", &FormatEnum(val.into(), MsgType::from_value))
                }
                Accept::Timeout(val) => fmt.field("Timeout", &val),
                Accept::AuthMode(val) => {
                    fmt.field("AuthMode", &FormatEnum(val.into(), Auth::from_value))
                }
                Accept::PeerIdentity(val) => fmt.field("PeerIdentity", &val),
                Accept::Certificate(val) => fmt.field("Certificate", &val),
                Accept::Peername(val) => fmt.field("Peername", &val),
                Accept::Keyring(val) => fmt.field("Keyring", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableAccept<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Accept", offset));
            return (stack, missing_type.and_then(|t| Accept::attr_from_type(t)));
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
                Accept::Sockfd(val) => {
                    if last_off == offset {
                        stack.push(("Sockfd", last_off));
                        break;
                    }
                }
                Accept::HandlerClass(val) => {
                    if last_off == offset {
                        stack.push(("HandlerClass", last_off));
                        break;
                    }
                }
                Accept::MessageType(val) => {
                    if last_off == offset {
                        stack.push(("MessageType", last_off));
                        break;
                    }
                }
                Accept::Timeout(val) => {
                    if last_off == offset {
                        stack.push(("Timeout", last_off));
                        break;
                    }
                }
                Accept::AuthMode(val) => {
                    if last_off == offset {
                        stack.push(("AuthMode", last_off));
                        break;
                    }
                }
                Accept::PeerIdentity(val) => {
                    if last_off == offset {
                        stack.push(("PeerIdentity", last_off));
                        break;
                    }
                }
                Accept::Certificate(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Accept::Peername(val) => {
                    if last_off == offset {
                        stack.push(("Peername", last_off));
                        break;
                    }
                }
                Accept::Keyring(val) => {
                    if last_off == offset {
                        stack.push(("Keyring", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Accept", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum Done {
    Status(u32),
    Sockfd(i32),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    RemoteAuth(u32),
}
impl<'a> IterableDone<'a> {
    pub fn get_status(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Done::Status(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Done",
            "Status",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sockfd(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Done::Sockfd(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Done",
            "Sockfd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_remote_auth(&self) -> MultiAttrIterable<Self, Done, u32> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Done::RemoteAuth(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl Done {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDone<'a> {
        IterableDone::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Status",
            2u16 => "Sockfd",
            3u16 => "RemoteAuth",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDone<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDone<'a> {
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
impl<'a> Iterator for IterableDone<'a> {
    type Item = Result<Done, ErrorContext>;
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
                1u16 => Done::Status({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Done::Sockfd({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Done::RemoteAuth({
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
            "Done",
            r#type.and_then(|t| Done::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableDone<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Done");
        let mut iter = IterableDone::with_loc(&[], self.orig_loc);
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
                Done::Status(val) => fmt.field("Status", &val),
                Done::Sockfd(val) => fmt.field("Sockfd", &val),
                Done::RemoteAuth(val) => fmt.field("RemoteAuth", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDone<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Done", offset));
            return (stack, missing_type.and_then(|t| Done::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Done::Status(val) => {
                    if last_off == offset {
                        stack.push(("Status", last_off));
                        break;
                    }
                }
                Done::Sockfd(val) => {
                    if last_off == offset {
                        stack.push(("Sockfd", last_off));
                        break;
                    }
                }
                Done::RemoteAuth(val) => {
                    if last_off == offset {
                        stack.push(("RemoteAuth", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Done", cur));
        }
        (stack, None)
    }
}
pub struct PushX509<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushX509<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushX509<Prev> {
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
    pub fn push_cert(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_privkey(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushX509<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushAccept<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushAccept<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushAccept<Prev> {
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
    pub fn push_sockfd(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`HandlerClass`] (enum)"]
    pub fn push_handler_class(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`MsgType`] (enum)"]
    pub fn push_message_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_timeout(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`Auth`] (enum)"]
    pub fn push_auth_mode(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn push_peer_identity(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_certificate(mut self) -> PushX509<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 7u16);
        PushX509 {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_peername(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            8u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_peername_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 8u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_keyring(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushAccept<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDone<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDone<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDone<Prev> {
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
    pub fn push_status(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_sockfd(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn push_remote_auth(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushDone<Prev> {
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
    pub const NONE: &str = "none";
    pub const NONE_CSTR: &CStr = c"none";
    pub const TLSHD: &str = "tlshd";
    pub const TLSHD_CSTR: &CStr = c"tlshd";
}
#[doc = "Handler retrieves next queued handshake request\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_handler_class()](PushAccept::push_handler_class)\n\nReply attributes:\n- [.get_sockfd()](IterableAccept::get_sockfd)\n- [.get_message_type()](IterableAccept::get_message_type)\n- [.get_timeout()](IterableAccept::get_timeout)\n- [.get_auth_mode()](IterableAccept::get_auth_mode)\n- [.get_peer_identity()](IterableAccept::get_peer_identity)\n- [.get_certificate()](IterableAccept::get_certificate)\n- [.get_peername()](IterableAccept::get_peername)\n- [.get_keyring()](IterableAccept::get_keyring)\n\n"]
#[derive(Debug)]
pub struct OpAcceptDo<'r> {
    request: Request<'r>,
}
impl<'r> OpAcceptDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushAccept<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushAccept::new(buf)
    }
    pub fn encode(&mut self) -> PushAccept<&mut Vec<u8>> {
        PushAccept::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushAccept<RequestBuf<'r>> {
        PushAccept::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableAccept<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableAccept::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpAcceptDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("handshake".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableAccept<'buf>;
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
#[doc = "Handler reports handshake completion\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_status()](PushDone::push_status)\n- [.push_sockfd()](PushDone::push_sockfd)\n- [.push_remote_auth()](PushDone::push_remote_auth)\n\n"]
#[derive(Debug)]
pub struct OpDoneDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDoneDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDone<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDone::new(buf)
    }
    pub fn encode(&mut self) -> PushDone<&mut Vec<u8>> {
        PushDone::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDone<RequestBuf<'r>> {
        PushDone::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDone<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDone::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpDoneDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("handshake".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDone<'buf>;
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
    #[doc = "Handler retrieves next queued handshake request\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_handler_class()](PushAccept::push_handler_class)\n\nReply attributes:\n- [.get_sockfd()](IterableAccept::get_sockfd)\n- [.get_message_type()](IterableAccept::get_message_type)\n- [.get_timeout()](IterableAccept::get_timeout)\n- [.get_auth_mode()](IterableAccept::get_auth_mode)\n- [.get_peer_identity()](IterableAccept::get_peer_identity)\n- [.get_certificate()](IterableAccept::get_certificate)\n- [.get_peername()](IterableAccept::get_peername)\n- [.get_keyring()](IterableAccept::get_keyring)\n\n"]
    pub fn op_accept_do(self) -> OpAcceptDo<'buf> {
        let mut res = OpAcceptDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-accept-do", OpAcceptDo::lookup);
        res
    }
    #[doc = "Handler reports handshake completion\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_status()](PushDone::push_status)\n- [.push_sockfd()](PushDone::push_sockfd)\n- [.push_remote_auth()](PushDone::push_remote_auth)\n\n"]
    pub fn op_done_do(self) -> OpDoneDo<'buf> {
        let mut res = OpDoneDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-done-do", OpDoneDo::lookup);
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableAccept::get_auth_mode;
        let _ = IterableAccept::get_certificate;
        let _ = IterableAccept::get_keyring;
        let _ = IterableAccept::get_message_type;
        let _ = IterableAccept::get_peer_identity;
        let _ = IterableAccept::get_peername;
        let _ = IterableAccept::get_sockfd;
        let _ = IterableAccept::get_timeout;
        let _ = PushAccept::<&mut Vec<u8>>::push_handler_class;
        let _ = PushDone::<&mut Vec<u8>>::push_remote_auth;
        let _ = PushDone::<&mut Vec<u8>>::push_sockfd;
        let _ = PushDone::<&mut Vec<u8>>::push_status;
    }
}
