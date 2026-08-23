#![doc = "Multipath TCP.\n"]
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
pub const PROTONAME: &str = "mptcp_pm";
pub const PROTONAME_CSTR: &CStr = c"mptcp_pm";
#[doc = "Netlink MPTCP event types\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum EventType {
    #[doc = "unused event\n"]
    Unspec = 0,
    #[doc = "A new MPTCP connection has been created. It is the good time to allocate\nmemory and send ADD_ADDR if needed. Depending on the traffic-patterns it\ncan take a long time until the MPTCP_EVENT_ESTABLISHED is sent.\nAttributes: token, family, saddr4 \\| saddr6, daddr4 \\| daddr6, sport,\ndport, \\[server-side\\], \\[flags\\].\n"]
    Created = 1,
    #[doc = "A MPTCP connection is established (can start new subflows). Attributes:\ntoken, family, saddr4 \\| saddr6, daddr4 \\| daddr6, sport, dport,\n\\[server-side\\], \\[flags\\].\n"]
    Established = 2,
    #[doc = "A MPTCP connection has stopped. Attribute: token.\n"]
    Closed = 3,
    #[doc = "A new address has been announced by the peer. Attributes: token, rem_id,\nfamily, daddr4 \\| daddr6 \\[, dport\\].\n"]
    Announced = 6,
    #[doc = "An address has been lost by the peer. Attributes: token, rem_id.\n"]
    Removed = 7,
    #[doc = "A new subflow has been established. \\'error\\' should not be set.\nAttributes: token, family, loc_id, rem_id, saddr4 \\| saddr6, daddr4 \\|\ndaddr6, sport, dport, backup, if-idx \\[, error\\].\n"]
    SubEstablished = 10,
    #[doc = "A subflow has been closed. An error (copy of sk_err) could be set if an\nerror has been detected for this subflow. Attributes: token, family,\nloc_id, rem_id, saddr4 \\| saddr6, daddr4 \\| daddr6, sport, dport,\nbackup, if-idx \\[, error\\].\n"]
    SubClosed = 11,
    #[doc = "The priority of a subflow has changed. \\'error\\' should not be set.\nAttributes: token, family, loc_id, rem_id, saddr4 \\| saddr6, daddr4 \\|\ndaddr6, sport, dport, backup, if-idx \\[, error\\].\n"]
    SubPriority = 13,
    #[doc = "A new PM listener is created. Attributes: family, sport, saddr4 \\|\nsaddr6.\n"]
    ListenerCreated = 15,
    #[doc = "A PM listener is closed. Attributes: family, sport, saddr4 \\| saddr6.\n"]
    ListenerClosed = 16,
}
impl EventType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Unspec,
            1 => Self::Created,
            2 => Self::Established,
            3 => Self::Closed,
            6 => Self::Announced,
            7 => Self::Removed,
            10 => Self::SubEstablished,
            11 => Self::SubClosed,
            13 => Self::SubPriority,
            15 => Self::ListenerCreated,
            16 => Self::ListenerClosed,
            _ => return None,
        })
    }
}
#[derive(Clone)]
pub enum Address<'a> {
    Family(u16),
    Id(u8),
    Addr4(u32),
    Addr6(&'a [u8]),
    Port(u16),
    Flags(u32),
    IfIdx(i32),
}
impl<'a> IterableAddress<'a> {
    pub fn get_family(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Address::Family(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Address",
            "Family",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_id(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Address::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Address",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_addr4(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Address::Addr4(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Address",
            "Addr4",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_addr6(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Address::Addr6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Address",
            "Addr6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Address::Port(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Address",
            "Port",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Address::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Address",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_if_idx(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Address::IfIdx(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Address",
            "IfIdx",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Address<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableAddress<'a> {
        IterableAddress::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "Unspec",
            1u16 => "Family",
            2u16 => "Id",
            3u16 => "Addr4",
            4u16 => "Addr6",
            5u16 => "Port",
            6u16 => "Flags",
            7u16 => "IfIdx",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableAddress<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableAddress<'a> {
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
impl<'a> Iterator for IterableAddress<'a> {
    type Item = Result<Address<'a>, ErrorContext>;
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
                1u16 => Address::Family({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Address::Id({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Address::Addr4({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Address::Addr6({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Address::Port({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Address::Flags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => Address::IfIdx({
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
            "Address",
            r#type.and_then(|t| Address::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableAddress<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Address");
        let mut iter = IterableAddress::with_loc(&[], self.orig_loc);
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
                Address::Family(val) => fmt.field("Family", &val),
                Address::Id(val) => fmt.field("Id", &val),
                Address::Addr4(val) => fmt.field("Addr4", &val),
                Address::Addr6(val) => fmt.field("Addr6", &FormatHexdump(val)),
                Address::Port(val) => fmt.field("Port", &val),
                Address::Flags(val) => fmt.field("Flags", &val),
                Address::IfIdx(val) => fmt.field("IfIdx", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableAddress<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Address", offset));
            return (stack, missing_type.and_then(|t| Address::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Address::Family(val) => {
                    if last_off == offset {
                        stack.push(("Family", last_off));
                        break;
                    }
                }
                Address::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                Address::Addr4(val) => {
                    if last_off == offset {
                        stack.push(("Addr4", last_off));
                        break;
                    }
                }
                Address::Addr6(val) => {
                    if last_off == offset {
                        stack.push(("Addr6", last_off));
                        break;
                    }
                }
                Address::Port(val) => {
                    if last_off == offset {
                        stack.push(("Port", last_off));
                        break;
                    }
                }
                Address::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                Address::IfIdx(val) => {
                    if last_off == offset {
                        stack.push(("IfIdx", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Address", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum SubflowAttribute<'a> {
    TokenRem(u32),
    TokenLoc(u32),
    RelwriteSeq(u32),
    MapSeq(u64),
    MapSfseq(u32),
    SsnOffset(u32),
    MapDatalen(u16),
    Flags(u32),
    IdRem(u8),
    IdLoc(u8),
    Pad(&'a [u8]),
}
impl<'a> IterableSubflowAttribute<'a> {
    pub fn get_token_rem(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SubflowAttribute::TokenRem(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SubflowAttribute",
            "TokenRem",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_token_loc(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SubflowAttribute::TokenLoc(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SubflowAttribute",
            "TokenLoc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_relwrite_seq(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SubflowAttribute::RelwriteSeq(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SubflowAttribute",
            "RelwriteSeq",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_map_seq(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SubflowAttribute::MapSeq(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SubflowAttribute",
            "MapSeq",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_map_sfseq(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SubflowAttribute::MapSfseq(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SubflowAttribute",
            "MapSfseq",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ssn_offset(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SubflowAttribute::SsnOffset(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SubflowAttribute",
            "SsnOffset",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_map_datalen(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SubflowAttribute::MapDatalen(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SubflowAttribute",
            "MapDatalen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SubflowAttribute::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SubflowAttribute",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_id_rem(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SubflowAttribute::IdRem(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SubflowAttribute",
            "IdRem",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_id_loc(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SubflowAttribute::IdLoc(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SubflowAttribute",
            "IdLoc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(SubflowAttribute::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "SubflowAttribute",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl SubflowAttribute<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableSubflowAttribute<'a> {
        IterableSubflowAttribute::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "Unspec",
            1u16 => "TokenRem",
            2u16 => "TokenLoc",
            3u16 => "RelwriteSeq",
            4u16 => "MapSeq",
            5u16 => "MapSfseq",
            6u16 => "SsnOffset",
            7u16 => "MapDatalen",
            8u16 => "Flags",
            9u16 => "IdRem",
            10u16 => "IdLoc",
            11u16 => "Pad",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableSubflowAttribute<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSubflowAttribute<'a> {
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
impl<'a> Iterator for IterableSubflowAttribute<'a> {
    type Item = Result<SubflowAttribute<'a>, ErrorContext>;
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
                1u16 => SubflowAttribute::TokenRem({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => SubflowAttribute::TokenLoc({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => SubflowAttribute::RelwriteSeq({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => SubflowAttribute::MapSeq({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => SubflowAttribute::MapSfseq({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => SubflowAttribute::SsnOffset({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => SubflowAttribute::MapDatalen({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => SubflowAttribute::Flags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => SubflowAttribute::IdRem({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => SubflowAttribute::IdLoc({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => SubflowAttribute::Pad({
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
            "SubflowAttribute",
            r#type.and_then(|t| SubflowAttribute::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableSubflowAttribute<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("SubflowAttribute");
        let mut iter = IterableSubflowAttribute::with_loc(&[], self.orig_loc);
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
                SubflowAttribute::TokenRem(val) => fmt.field("TokenRem", &val),
                SubflowAttribute::TokenLoc(val) => fmt.field("TokenLoc", &val),
                SubflowAttribute::RelwriteSeq(val) => fmt.field("RelwriteSeq", &val),
                SubflowAttribute::MapSeq(val) => fmt.field("MapSeq", &val),
                SubflowAttribute::MapSfseq(val) => fmt.field("MapSfseq", &val),
                SubflowAttribute::SsnOffset(val) => fmt.field("SsnOffset", &val),
                SubflowAttribute::MapDatalen(val) => fmt.field("MapDatalen", &val),
                SubflowAttribute::Flags(val) => fmt.field("Flags", &val),
                SubflowAttribute::IdRem(val) => fmt.field("IdRem", &val),
                SubflowAttribute::IdLoc(val) => fmt.field("IdLoc", &val),
                SubflowAttribute::Pad(val) => fmt.field("Pad", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSubflowAttribute<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("SubflowAttribute", offset));
            return (
                stack,
                missing_type.and_then(|t| SubflowAttribute::attr_from_type(t)),
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
                SubflowAttribute::TokenRem(val) => {
                    if last_off == offset {
                        stack.push(("TokenRem", last_off));
                        break;
                    }
                }
                SubflowAttribute::TokenLoc(val) => {
                    if last_off == offset {
                        stack.push(("TokenLoc", last_off));
                        break;
                    }
                }
                SubflowAttribute::RelwriteSeq(val) => {
                    if last_off == offset {
                        stack.push(("RelwriteSeq", last_off));
                        break;
                    }
                }
                SubflowAttribute::MapSeq(val) => {
                    if last_off == offset {
                        stack.push(("MapSeq", last_off));
                        break;
                    }
                }
                SubflowAttribute::MapSfseq(val) => {
                    if last_off == offset {
                        stack.push(("MapSfseq", last_off));
                        break;
                    }
                }
                SubflowAttribute::SsnOffset(val) => {
                    if last_off == offset {
                        stack.push(("SsnOffset", last_off));
                        break;
                    }
                }
                SubflowAttribute::MapDatalen(val) => {
                    if last_off == offset {
                        stack.push(("MapDatalen", last_off));
                        break;
                    }
                }
                SubflowAttribute::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                SubflowAttribute::IdRem(val) => {
                    if last_off == offset {
                        stack.push(("IdRem", last_off));
                        break;
                    }
                }
                SubflowAttribute::IdLoc(val) => {
                    if last_off == offset {
                        stack.push(("IdLoc", last_off));
                        break;
                    }
                }
                SubflowAttribute::Pad(val) => {
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
            stack.push(("SubflowAttribute", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum Endpoint<'a> {
    Addr(IterableAddress<'a>),
}
impl<'a> IterableEndpoint<'a> {
    pub fn get_addr(&self) -> Result<IterableAddress<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Endpoint::Addr(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Endpoint",
            "Addr",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Endpoint<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableEndpoint<'a> {
        IterableEndpoint::with_loc(buf, buf.as_ptr() as usize)
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
pub struct IterableEndpoint<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableEndpoint<'a> {
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
impl<'a> Iterator for IterableEndpoint<'a> {
    type Item = Result<Endpoint<'a>, ErrorContext>;
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
                1u16 => Endpoint::Addr({
                    let res = Some(IterableAddress::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Endpoint",
            r#type.and_then(|t| Endpoint::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableEndpoint<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Endpoint");
        let mut iter = IterableEndpoint::with_loc(&[], self.orig_loc);
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
                Endpoint::Addr(val) => fmt.field("Addr", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableEndpoint<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Endpoint", offset));
            return (
                stack,
                missing_type.and_then(|t| Endpoint::attr_from_type(t)),
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
                Endpoint::Addr(val) => {
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
            stack.push(("Endpoint", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum Attr<'a> {
    Addr(IterableAddress<'a>),
    RcvAddAddrs(u32),
    Subflows(u32),
    Token(u32),
    LocId(u8),
    AddrRemote(IterableAddress<'a>),
}
impl<'a> IterableAttr<'a> {
    pub fn get_addr(&self) -> Result<IterableAddress<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Attr::Addr(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Attr",
            "Addr",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rcv_add_addrs(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Attr::RcvAddAddrs(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Attr",
            "RcvAddAddrs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_subflows(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Attr::Subflows(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Attr",
            "Subflows",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_token(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Attr::Token(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Attr",
            "Token",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_loc_id(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Attr::LocId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Attr",
            "LocId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_addr_remote(&self) -> Result<IterableAddress<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Attr::AddrRemote(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Attr",
            "AddrRemote",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Attr<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableAttr<'a> {
        IterableAttr::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "Unspec",
            1u16 => "Addr",
            2u16 => "RcvAddAddrs",
            3u16 => "Subflows",
            4u16 => "Token",
            5u16 => "LocId",
            6u16 => "AddrRemote",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableAttr<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableAttr<'a> {
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
impl<'a> Iterator for IterableAttr<'a> {
    type Item = Result<Attr<'a>, ErrorContext>;
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
                1u16 => Attr::Addr({
                    let res = Some(IterableAddress::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Attr::RcvAddAddrs({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Attr::Subflows({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Attr::Token({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Attr::LocId({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Attr::AddrRemote({
                    let res = Some(IterableAddress::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Attr",
            r#type.and_then(|t| Attr::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableAttr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Attr");
        let mut iter = IterableAttr::with_loc(&[], self.orig_loc);
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
                Attr::Addr(val) => fmt.field("Addr", &val),
                Attr::RcvAddAddrs(val) => fmt.field("RcvAddAddrs", &val),
                Attr::Subflows(val) => fmt.field("Subflows", &val),
                Attr::Token(val) => fmt.field("Token", &val),
                Attr::LocId(val) => fmt.field("LocId", &val),
                Attr::AddrRemote(val) => fmt.field("AddrRemote", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableAttr<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Attr", offset));
            return (stack, missing_type.and_then(|t| Attr::attr_from_type(t)));
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
                Attr::Addr(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Attr::RcvAddAddrs(val) => {
                    if last_off == offset {
                        stack.push(("RcvAddAddrs", last_off));
                        break;
                    }
                }
                Attr::Subflows(val) => {
                    if last_off == offset {
                        stack.push(("Subflows", last_off));
                        break;
                    }
                }
                Attr::Token(val) => {
                    if last_off == offset {
                        stack.push(("Token", last_off));
                        break;
                    }
                }
                Attr::LocId(val) => {
                    if last_off == offset {
                        stack.push(("LocId", last_off));
                        break;
                    }
                }
                Attr::AddrRemote(val) => {
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
            stack.push(("Attr", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum EventAttr<'a> {
    Token(u32),
    Family(u16),
    LocId(u8),
    RemId(u8),
    Saddr4(u32),
    Saddr6(&'a [u8]),
    Daddr4(u32),
    Daddr6(&'a [u8]),
    Sport(u16),
    Dport(u16),
    Backup(u8),
    Error(u8),
    Flags(u16),
    Timeout(u32),
    IfIdx(i32),
    ResetReason(u32),
    ResetFlags(u32),
    #[doc = "Deprecated: use \\'flags\\'\n"]
    ServerSide(u8),
}
impl<'a> IterableEventAttr<'a> {
    pub fn get_token(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(EventAttr::Token(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "EventAttr",
            "Token",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_family(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(EventAttr::Family(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "EventAttr",
            "Family",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_loc_id(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(EventAttr::LocId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "EventAttr",
            "LocId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rem_id(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(EventAttr::RemId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "EventAttr",
            "RemId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_saddr4(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(EventAttr::Saddr4(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "EventAttr",
            "Saddr4",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_saddr6(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(EventAttr::Saddr6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "EventAttr",
            "Saddr6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_daddr4(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(EventAttr::Daddr4(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "EventAttr",
            "Daddr4",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_daddr6(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(EventAttr::Daddr6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "EventAttr",
            "Daddr6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(EventAttr::Sport(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "EventAttr",
            "Sport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(EventAttr::Dport(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "EventAttr",
            "Dport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_backup(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(EventAttr::Backup(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "EventAttr",
            "Backup",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_error(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(EventAttr::Error(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "EventAttr",
            "Error",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flags(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(EventAttr::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "EventAttr",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_timeout(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(EventAttr::Timeout(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "EventAttr",
            "Timeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_if_idx(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(EventAttr::IfIdx(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "EventAttr",
            "IfIdx",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_reset_reason(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(EventAttr::ResetReason(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "EventAttr",
            "ResetReason",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_reset_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(EventAttr::ResetFlags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "EventAttr",
            "ResetFlags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Deprecated: use \\'flags\\'\n"]
    pub fn get_server_side(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(EventAttr::ServerSide(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "EventAttr",
            "ServerSide",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl EventAttr<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableEventAttr<'a> {
        IterableEventAttr::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "Unspec",
            1u16 => "Token",
            2u16 => "Family",
            3u16 => "LocId",
            4u16 => "RemId",
            5u16 => "Saddr4",
            6u16 => "Saddr6",
            7u16 => "Daddr4",
            8u16 => "Daddr6",
            9u16 => "Sport",
            10u16 => "Dport",
            11u16 => "Backup",
            12u16 => "Error",
            13u16 => "Flags",
            14u16 => "Timeout",
            15u16 => "IfIdx",
            16u16 => "ResetReason",
            17u16 => "ResetFlags",
            18u16 => "ServerSide",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableEventAttr<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableEventAttr<'a> {
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
impl<'a> Iterator for IterableEventAttr<'a> {
    type Item = Result<EventAttr<'a>, ErrorContext>;
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
                1u16 => EventAttr::Token({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => EventAttr::Family({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => EventAttr::LocId({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => EventAttr::RemId({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => EventAttr::Saddr4({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => EventAttr::Saddr6({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => EventAttr::Daddr4({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => EventAttr::Daddr6({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => EventAttr::Sport({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => EventAttr::Dport({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => EventAttr::Backup({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => EventAttr::Error({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => EventAttr::Flags({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => EventAttr::Timeout({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => EventAttr::IfIdx({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => EventAttr::ResetReason({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => EventAttr::ResetFlags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => EventAttr::ServerSide({
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
            "EventAttr",
            r#type.and_then(|t| EventAttr::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableEventAttr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("EventAttr");
        let mut iter = IterableEventAttr::with_loc(&[], self.orig_loc);
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
                EventAttr::Token(val) => fmt.field("Token", &val),
                EventAttr::Family(val) => fmt.field("Family", &val),
                EventAttr::LocId(val) => fmt.field("LocId", &val),
                EventAttr::RemId(val) => fmt.field("RemId", &val),
                EventAttr::Saddr4(val) => fmt.field("Saddr4", &val),
                EventAttr::Saddr6(val) => fmt.field("Saddr6", &FormatHexdump(val)),
                EventAttr::Daddr4(val) => fmt.field("Daddr4", &val),
                EventAttr::Daddr6(val) => fmt.field("Daddr6", &FormatHexdump(val)),
                EventAttr::Sport(val) => fmt.field("Sport", &val),
                EventAttr::Dport(val) => fmt.field("Dport", &val),
                EventAttr::Backup(val) => fmt.field("Backup", &val),
                EventAttr::Error(val) => fmt.field("Error", &val),
                EventAttr::Flags(val) => fmt.field("Flags", &val),
                EventAttr::Timeout(val) => fmt.field("Timeout", &val),
                EventAttr::IfIdx(val) => fmt.field("IfIdx", &val),
                EventAttr::ResetReason(val) => fmt.field("ResetReason", &val),
                EventAttr::ResetFlags(val) => fmt.field("ResetFlags", &val),
                EventAttr::ServerSide(val) => fmt.field("ServerSide", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableEventAttr<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("EventAttr", offset));
            return (
                stack,
                missing_type.and_then(|t| EventAttr::attr_from_type(t)),
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
                EventAttr::Token(val) => {
                    if last_off == offset {
                        stack.push(("Token", last_off));
                        break;
                    }
                }
                EventAttr::Family(val) => {
                    if last_off == offset {
                        stack.push(("Family", last_off));
                        break;
                    }
                }
                EventAttr::LocId(val) => {
                    if last_off == offset {
                        stack.push(("LocId", last_off));
                        break;
                    }
                }
                EventAttr::RemId(val) => {
                    if last_off == offset {
                        stack.push(("RemId", last_off));
                        break;
                    }
                }
                EventAttr::Saddr4(val) => {
                    if last_off == offset {
                        stack.push(("Saddr4", last_off));
                        break;
                    }
                }
                EventAttr::Saddr6(val) => {
                    if last_off == offset {
                        stack.push(("Saddr6", last_off));
                        break;
                    }
                }
                EventAttr::Daddr4(val) => {
                    if last_off == offset {
                        stack.push(("Daddr4", last_off));
                        break;
                    }
                }
                EventAttr::Daddr6(val) => {
                    if last_off == offset {
                        stack.push(("Daddr6", last_off));
                        break;
                    }
                }
                EventAttr::Sport(val) => {
                    if last_off == offset {
                        stack.push(("Sport", last_off));
                        break;
                    }
                }
                EventAttr::Dport(val) => {
                    if last_off == offset {
                        stack.push(("Dport", last_off));
                        break;
                    }
                }
                EventAttr::Backup(val) => {
                    if last_off == offset {
                        stack.push(("Backup", last_off));
                        break;
                    }
                }
                EventAttr::Error(val) => {
                    if last_off == offset {
                        stack.push(("Error", last_off));
                        break;
                    }
                }
                EventAttr::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                EventAttr::Timeout(val) => {
                    if last_off == offset {
                        stack.push(("Timeout", last_off));
                        break;
                    }
                }
                EventAttr::IfIdx(val) => {
                    if last_off == offset {
                        stack.push(("IfIdx", last_off));
                        break;
                    }
                }
                EventAttr::ResetReason(val) => {
                    if last_off == offset {
                        stack.push(("ResetReason", last_off));
                        break;
                    }
                }
                EventAttr::ResetFlags(val) => {
                    if last_off == offset {
                        stack.push(("ResetFlags", last_off));
                        break;
                    }
                }
                EventAttr::ServerSide(val) => {
                    if last_off == offset {
                        stack.push(("ServerSide", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("EventAttr", cur));
        }
        (stack, None)
    }
}
pub struct PushAddress<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushAddress<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushAddress<Prev> {
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
    pub fn push_family(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 1u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_id(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 2u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_addr4(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_addr6(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 4u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_port(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 5u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_if_idx(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushAddress<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSubflowAttribute<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushSubflowAttribute<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushSubflowAttribute<Prev> {
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
    pub fn push_token_rem(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_token_loc(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_relwrite_seq(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_map_seq(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 4u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_map_sfseq(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ssn_offset(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_map_datalen(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 7u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_id_rem(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 9u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_id_loc(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 10u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 11u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushSubflowAttribute<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushEndpoint<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushEndpoint<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushEndpoint<Prev> {
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
    pub fn nested_addr(mut self) -> PushAddress<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushAddress {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushEndpoint<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushAttr<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushAttr<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushAttr<Prev> {
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
    pub fn nested_addr(mut self) -> PushAddress<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushAddress {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_rcv_add_addrs(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_subflows(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_token(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_loc_id(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 5u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_addr_remote(mut self) -> PushAddress<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 6u16);
        PushAddress {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushAttr<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushEventAttr<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushEventAttr<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushEventAttr<Prev> {
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
    pub fn push_token(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_family(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 2u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_loc_id(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 3u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_rem_id(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 4u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_saddr4(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_saddr6(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 6u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_daddr4(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_daddr6(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 8u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_sport(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 9u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_dport(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 10u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_backup(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 11u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_error(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 12u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_flags(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 13u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_timeout(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 14u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_if_idx(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 15u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_reset_reason(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 16u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_reset_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 17u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Deprecated: use \\'flags\\'\n"]
    pub fn push_server_side(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 18u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushEventAttr<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Add endpoint\n\nFlags: uns-admin-perm\n\nRequest attributes:\n- [.nested_addr()](PushEndpoint::nested_addr)\n\n"]
#[derive(Debug)]
pub struct OpAddAddrDo<'r> {
    request: Request<'r>,
}
impl<'r> OpAddAddrDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushEndpoint<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushEndpoint::new(buf)
    }
    pub fn encode(&mut self) -> PushEndpoint<&mut Vec<u8>> {
        PushEndpoint::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushEndpoint<RequestBuf<'r>> {
        PushEndpoint::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableEndpoint<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableEndpoint::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpAddAddrDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("mptcp_pm".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableEndpoint<'buf>;
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
#[doc = "Delete endpoint\n\nFlags: uns-admin-perm\n\nRequest attributes:\n- [.nested_addr()](PushEndpoint::nested_addr)\n\n"]
#[derive(Debug)]
pub struct OpDelAddrDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDelAddrDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushEndpoint<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushEndpoint::new(buf)
    }
    pub fn encode(&mut self) -> PushEndpoint<&mut Vec<u8>> {
        PushEndpoint::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushEndpoint<RequestBuf<'r>> {
        PushEndpoint::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableEndpoint<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableEndpoint::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpDelAddrDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("mptcp_pm".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableEndpoint<'buf>;
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
#[doc = "Get endpoint information\n\nReply attributes:\n- [.get_addr()](IterableAttr::get_addr)\n\n"]
#[derive(Debug)]
pub struct OpGetAddrDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetAddrDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushAttr<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushAttr::new(buf)
    }
    pub fn encode(&mut self) -> PushAttr<&mut Vec<u8>> {
        PushAttr::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushAttr<RequestBuf<'r>> {
        PushAttr::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableAttr<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableAttr::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpGetAddrDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("mptcp_pm".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableAttr<'buf>;
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
#[doc = "Get endpoint information\n\nRequest attributes:\n- [.nested_addr()](PushAttr::nested_addr)\n- [.push_token()](PushAttr::push_token)\n\nReply attributes:\n- [.get_addr()](IterableAttr::get_addr)\n\n"]
#[derive(Debug)]
pub struct OpGetAddrDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetAddrDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushAttr<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushAttr::new(buf)
    }
    pub fn encode(&mut self) -> PushAttr<&mut Vec<u8>> {
        PushAttr::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushAttr<RequestBuf<'r>> {
        PushAttr::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableAttr<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableAttr::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpGetAddrDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("mptcp_pm".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableAttr<'buf>;
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
#[doc = "Flush addresses\n\nFlags: uns-admin-perm\n\nRequest attributes:\n- [.nested_addr()](PushEndpoint::nested_addr)\n\n"]
#[derive(Debug)]
pub struct OpFlushAddrsDo<'r> {
    request: Request<'r>,
}
impl<'r> OpFlushAddrsDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushEndpoint<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushEndpoint::new(buf)
    }
    pub fn encode(&mut self) -> PushEndpoint<&mut Vec<u8>> {
        PushEndpoint::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushEndpoint<RequestBuf<'r>> {
        PushEndpoint::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableEndpoint<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableEndpoint::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpFlushAddrsDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("mptcp_pm".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableEndpoint<'buf>;
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
#[doc = "Set protocol limits\n\nFlags: uns-admin-perm\n\nRequest attributes:\n- [.push_rcv_add_addrs()](PushAttr::push_rcv_add_addrs)\n- [.push_subflows()](PushAttr::push_subflows)\n\n"]
#[derive(Debug)]
pub struct OpSetLimitsDo<'r> {
    request: Request<'r>,
}
impl<'r> OpSetLimitsDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushAttr<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushAttr::new(buf)
    }
    pub fn encode(&mut self) -> PushAttr<&mut Vec<u8>> {
        PushAttr::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushAttr<RequestBuf<'r>> {
        PushAttr::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableAttr<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableAttr::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpSetLimitsDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("mptcp_pm".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableAttr<'buf>;
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
#[doc = "Get protocol limits\n\nRequest attributes:\n- [.push_rcv_add_addrs()](PushAttr::push_rcv_add_addrs)\n- [.push_subflows()](PushAttr::push_subflows)\n\nReply attributes:\n- [.get_rcv_add_addrs()](IterableAttr::get_rcv_add_addrs)\n- [.get_subflows()](IterableAttr::get_subflows)\n\n"]
#[derive(Debug)]
pub struct OpGetLimitsDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetLimitsDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushAttr<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushAttr::new(buf)
    }
    pub fn encode(&mut self) -> PushAttr<&mut Vec<u8>> {
        PushAttr::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushAttr<RequestBuf<'r>> {
        PushAttr::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableAttr<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableAttr::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpGetLimitsDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("mptcp_pm".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableAttr<'buf>;
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
#[doc = "Change endpoint flags\n\nFlags: uns-admin-perm\n\nRequest attributes:\n- [.nested_addr()](PushAttr::nested_addr)\n- [.push_token()](PushAttr::push_token)\n- [.nested_addr_remote()](PushAttr::nested_addr_remote)\n\n"]
#[derive(Debug)]
pub struct OpSetFlagsDo<'r> {
    request: Request<'r>,
}
impl<'r> OpSetFlagsDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushAttr<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushAttr::new(buf)
    }
    pub fn encode(&mut self) -> PushAttr<&mut Vec<u8>> {
        PushAttr::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushAttr<RequestBuf<'r>> {
        PushAttr::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableAttr<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableAttr::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpSetFlagsDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("mptcp_pm".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableAttr<'buf>;
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
#[doc = "Announce new address\n\nFlags: uns-admin-perm\n\nRequest attributes:\n- [.nested_addr()](PushAttr::nested_addr)\n- [.push_token()](PushAttr::push_token)\n\n"]
#[derive(Debug)]
pub struct OpAnnounceDo<'r> {
    request: Request<'r>,
}
impl<'r> OpAnnounceDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushAttr<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushAttr::new(buf)
    }
    pub fn encode(&mut self) -> PushAttr<&mut Vec<u8>> {
        PushAttr::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushAttr<RequestBuf<'r>> {
        PushAttr::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableAttr<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableAttr::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpAnnounceDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("mptcp_pm".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableAttr<'buf>;
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
#[doc = "Announce removal\n\nFlags: uns-admin-perm\n\nRequest attributes:\n- [.push_token()](PushAttr::push_token)\n- [.push_loc_id()](PushAttr::push_loc_id)\n\n"]
#[derive(Debug)]
pub struct OpRemoveDo<'r> {
    request: Request<'r>,
}
impl<'r> OpRemoveDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushAttr<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushAttr::new(buf)
    }
    pub fn encode(&mut self) -> PushAttr<&mut Vec<u8>> {
        PushAttr::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushAttr<RequestBuf<'r>> {
        PushAttr::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableAttr<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableAttr::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpRemoveDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("mptcp_pm".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableAttr<'buf>;
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
#[doc = "Create subflow\n\nFlags: uns-admin-perm\n\nRequest attributes:\n- [.nested_addr()](PushAttr::nested_addr)\n- [.push_token()](PushAttr::push_token)\n- [.nested_addr_remote()](PushAttr::nested_addr_remote)\n\n"]
#[derive(Debug)]
pub struct OpSubflowCreateDo<'r> {
    request: Request<'r>,
}
impl<'r> OpSubflowCreateDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushAttr<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushAttr::new(buf)
    }
    pub fn encode(&mut self) -> PushAttr<&mut Vec<u8>> {
        PushAttr::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushAttr<RequestBuf<'r>> {
        PushAttr::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableAttr<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableAttr::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpSubflowCreateDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("mptcp_pm".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableAttr<'buf>;
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
#[doc = "Destroy subflow\n\nFlags: uns-admin-perm\n\nRequest attributes:\n- [.nested_addr()](PushAttr::nested_addr)\n- [.push_token()](PushAttr::push_token)\n- [.nested_addr_remote()](PushAttr::nested_addr_remote)\n\n"]
#[derive(Debug)]
pub struct OpSubflowDestroyDo<'r> {
    request: Request<'r>,
}
impl<'r> OpSubflowDestroyDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushAttr<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushAttr::new(buf)
    }
    pub fn encode(&mut self) -> PushAttr<&mut Vec<u8>> {
        PushAttr::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushAttr<RequestBuf<'r>> {
        PushAttr::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableAttr<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableAttr::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpSubflowDestroyDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("mptcp_pm".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableAttr<'buf>;
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
    #[doc = "Add endpoint\n\nFlags: uns-admin-perm\n\nRequest attributes:\n- [.nested_addr()](PushEndpoint::nested_addr)\n\n"]
    pub fn op_add_addr_do(self) -> OpAddAddrDo<'buf> {
        let mut res = OpAddAddrDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-add-addr-do", OpAddAddrDo::lookup);
        res
    }
    #[doc = "Delete endpoint\n\nFlags: uns-admin-perm\n\nRequest attributes:\n- [.nested_addr()](PushEndpoint::nested_addr)\n\n"]
    pub fn op_del_addr_do(self) -> OpDelAddrDo<'buf> {
        let mut res = OpDelAddrDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-del-addr-do", OpDelAddrDo::lookup);
        res
    }
    #[doc = "Get endpoint information\n\nReply attributes:\n- [.get_addr()](IterableAttr::get_addr)\n\n"]
    pub fn op_get_addr_dump(self) -> OpGetAddrDump<'buf> {
        let mut res = OpGetAddrDump::new(self);
        res.request
            .do_writeback(res.protocol(), "op-get-addr-dump", OpGetAddrDump::lookup);
        res
    }
    #[doc = "Get endpoint information\n\nRequest attributes:\n- [.nested_addr()](PushAttr::nested_addr)\n- [.push_token()](PushAttr::push_token)\n\nReply attributes:\n- [.get_addr()](IterableAttr::get_addr)\n\n"]
    pub fn op_get_addr_do(self) -> OpGetAddrDo<'buf> {
        let mut res = OpGetAddrDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-get-addr-do", OpGetAddrDo::lookup);
        res
    }
    #[doc = "Flush addresses\n\nFlags: uns-admin-perm\n\nRequest attributes:\n- [.nested_addr()](PushEndpoint::nested_addr)\n\n"]
    pub fn op_flush_addrs_do(self) -> OpFlushAddrsDo<'buf> {
        let mut res = OpFlushAddrsDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-flush-addrs-do", OpFlushAddrsDo::lookup);
        res
    }
    #[doc = "Set protocol limits\n\nFlags: uns-admin-perm\n\nRequest attributes:\n- [.push_rcv_add_addrs()](PushAttr::push_rcv_add_addrs)\n- [.push_subflows()](PushAttr::push_subflows)\n\n"]
    pub fn op_set_limits_do(self) -> OpSetLimitsDo<'buf> {
        let mut res = OpSetLimitsDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-set-limits-do", OpSetLimitsDo::lookup);
        res
    }
    #[doc = "Get protocol limits\n\nRequest attributes:\n- [.push_rcv_add_addrs()](PushAttr::push_rcv_add_addrs)\n- [.push_subflows()](PushAttr::push_subflows)\n\nReply attributes:\n- [.get_rcv_add_addrs()](IterableAttr::get_rcv_add_addrs)\n- [.get_subflows()](IterableAttr::get_subflows)\n\n"]
    pub fn op_get_limits_do(self) -> OpGetLimitsDo<'buf> {
        let mut res = OpGetLimitsDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-get-limits-do", OpGetLimitsDo::lookup);
        res
    }
    #[doc = "Change endpoint flags\n\nFlags: uns-admin-perm\n\nRequest attributes:\n- [.nested_addr()](PushAttr::nested_addr)\n- [.push_token()](PushAttr::push_token)\n- [.nested_addr_remote()](PushAttr::nested_addr_remote)\n\n"]
    pub fn op_set_flags_do(self) -> OpSetFlagsDo<'buf> {
        let mut res = OpSetFlagsDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-set-flags-do", OpSetFlagsDo::lookup);
        res
    }
    #[doc = "Announce new address\n\nFlags: uns-admin-perm\n\nRequest attributes:\n- [.nested_addr()](PushAttr::nested_addr)\n- [.push_token()](PushAttr::push_token)\n\n"]
    pub fn op_announce_do(self) -> OpAnnounceDo<'buf> {
        let mut res = OpAnnounceDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-announce-do", OpAnnounceDo::lookup);
        res
    }
    #[doc = "Announce removal\n\nFlags: uns-admin-perm\n\nRequest attributes:\n- [.push_token()](PushAttr::push_token)\n- [.push_loc_id()](PushAttr::push_loc_id)\n\n"]
    pub fn op_remove_do(self) -> OpRemoveDo<'buf> {
        let mut res = OpRemoveDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-remove-do", OpRemoveDo::lookup);
        res
    }
    #[doc = "Create subflow\n\nFlags: uns-admin-perm\n\nRequest attributes:\n- [.nested_addr()](PushAttr::nested_addr)\n- [.push_token()](PushAttr::push_token)\n- [.nested_addr_remote()](PushAttr::nested_addr_remote)\n\n"]
    pub fn op_subflow_create_do(self) -> OpSubflowCreateDo<'buf> {
        let mut res = OpSubflowCreateDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-subflow-create-do",
            OpSubflowCreateDo::lookup,
        );
        res
    }
    #[doc = "Destroy subflow\n\nFlags: uns-admin-perm\n\nRequest attributes:\n- [.nested_addr()](PushAttr::nested_addr)\n- [.push_token()](PushAttr::push_token)\n- [.nested_addr_remote()](PushAttr::nested_addr_remote)\n\n"]
    pub fn op_subflow_destroy_do(self) -> OpSubflowDestroyDo<'buf> {
        let mut res = OpSubflowDestroyDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-subflow-destroy-do",
            OpSubflowDestroyDo::lookup,
        );
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableAttr::get_addr;
        let _ = IterableAttr::get_rcv_add_addrs;
        let _ = IterableAttr::get_subflows;
        let _ = PushAttr::<&mut Vec<u8>>::nested_addr;
        let _ = PushAttr::<&mut Vec<u8>>::nested_addr_remote;
        let _ = PushAttr::<&mut Vec<u8>>::push_loc_id;
        let _ = PushAttr::<&mut Vec<u8>>::push_rcv_add_addrs;
        let _ = PushAttr::<&mut Vec<u8>>::push_subflows;
        let _ = PushAttr::<&mut Vec<u8>>::push_token;
        let _ = PushEndpoint::<&mut Vec<u8>>::nested_addr;
    }
}
