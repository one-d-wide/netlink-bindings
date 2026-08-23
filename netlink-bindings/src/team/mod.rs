#![doc = "Network team device driver.\n"]
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
pub const PROTONAME: &str = "team";
pub const PROTONAME_CSTR: &CStr = c"team";
pub const STRING_MAX_LEN: u64 = 32u64;
pub const GENL_CHANGE_EVENT_MC_GRP_NAME: &str = "change_event";
pub const GENL_CHANGE_EVENT_MC_GRP_NAME_CSTR: &CStr = c"change_event";
#[derive(Clone)]
pub enum Team<'a> {
    TeamIfindex(u32),
    ListOption(IterableItemOption<'a>),
    ListPort(IterableItemPort<'a>),
}
impl<'a> IterableTeam<'a> {
    pub fn get_team_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Team::TeamIfindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Team",
            "TeamIfindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_list_option(&self) -> Result<IterableItemOption<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Team::ListOption(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Team",
            "ListOption",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_list_port(&self) -> Result<IterableItemPort<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Team::ListPort(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Team",
            "ListPort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Team<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableTeam<'a> {
        IterableTeam::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "Unspec",
            1u16 => "TeamIfindex",
            2u16 => "ListOption",
            3u16 => "ListPort",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableTeam<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableTeam<'a> {
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
impl<'a> Iterator for IterableTeam<'a> {
    type Item = Result<Team<'a>, ErrorContext>;
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
                1u16 => Team::TeamIfindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Team::ListOption({
                    let res = Some(IterableItemOption::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Team::ListPort({
                    let res = Some(IterableItemPort::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Team",
            r#type.and_then(|t| Team::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableTeam<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Team");
        let mut iter = IterableTeam::with_loc(&[], self.orig_loc);
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
                Team::TeamIfindex(val) => fmt.field("TeamIfindex", &val),
                Team::ListOption(val) => fmt.field("ListOption", &val),
                Team::ListPort(val) => fmt.field("ListPort", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableTeam<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Team", offset));
            return (stack, missing_type.and_then(|t| Team::attr_from_type(t)));
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
                Team::TeamIfindex(val) => {
                    if last_off == offset {
                        stack.push(("TeamIfindex", last_off));
                        break;
                    }
                }
                Team::ListOption(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Team::ListPort(val) => {
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
            stack.push(("Team", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum ItemOption<'a> {
    Option(IterableAttrOption<'a>),
}
impl<'a> IterableItemOption<'a> {
    pub fn get_option(&self) -> Result<IterableAttrOption<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ItemOption::Option(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ItemOption",
            "Option",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ItemOption<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableItemOption<'a> {
        IterableItemOption::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "OptionUnspec",
            1u16 => "Option",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableItemOption<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableItemOption<'a> {
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
impl<'a> Iterator for IterableItemOption<'a> {
    type Item = Result<ItemOption<'a>, ErrorContext>;
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
                1u16 => ItemOption::Option({
                    let res = Some(IterableAttrOption::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "ItemOption",
            r#type.and_then(|t| ItemOption::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableItemOption<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ItemOption");
        let mut iter = IterableItemOption::with_loc(&[], self.orig_loc);
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
                ItemOption::Option(val) => fmt.field("Option", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableItemOption<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ItemOption", offset));
            return (
                stack,
                missing_type.and_then(|t| ItemOption::attr_from_type(t)),
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
                ItemOption::Option(val) => {
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
            stack.push(("ItemOption", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum AttrOption<'a> {
    Name(&'a CStr),
    Changed(()),
    Type(u8),
    Data(&'a [u8]),
    Removed(()),
    #[doc = "for per-port options\n"]
    PortIfindex(u32),
    #[doc = "for array options\n"]
    ArrayIndex(u32),
}
impl<'a> IterableAttrOption<'a> {
    pub fn get_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AttrOption::Name(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AttrOption",
            "Name",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_changed(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AttrOption::Changed(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AttrOption",
            "Changed",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_type(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AttrOption::Type(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AttrOption",
            "Type",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_data(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AttrOption::Data(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AttrOption",
            "Data",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_removed(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AttrOption::Removed(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AttrOption",
            "Removed",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "for per-port options\n"]
    pub fn get_port_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AttrOption::PortIfindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AttrOption",
            "PortIfindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "for array options\n"]
    pub fn get_array_index(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AttrOption::ArrayIndex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AttrOption",
            "ArrayIndex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl AttrOption<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableAttrOption<'a> {
        IterableAttrOption::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "Unspec",
            1u16 => "Name",
            2u16 => "Changed",
            3u16 => "Type",
            4u16 => "Data",
            5u16 => "Removed",
            6u16 => "PortIfindex",
            7u16 => "ArrayIndex",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableAttrOption<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableAttrOption<'a> {
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
impl<'a> Iterator for IterableAttrOption<'a> {
    type Item = Result<AttrOption<'a>, ErrorContext>;
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
                1u16 => AttrOption::Name({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => AttrOption::Changed(()),
                3u16 => AttrOption::Type({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => AttrOption::Data({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => AttrOption::Removed(()),
                6u16 => AttrOption::PortIfindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => AttrOption::ArrayIndex({
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
            "AttrOption",
            r#type.and_then(|t| AttrOption::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableAttrOption<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("AttrOption");
        let mut iter = IterableAttrOption::with_loc(&[], self.orig_loc);
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
                AttrOption::Name(val) => fmt.field("Name", &val),
                AttrOption::Changed(val) => fmt.field("Changed", &val),
                AttrOption::Type(val) => fmt.field("Type", &val),
                AttrOption::Data(val) => fmt.field("Data", &FormatHexdump(val)),
                AttrOption::Removed(val) => fmt.field("Removed", &val),
                AttrOption::PortIfindex(val) => fmt.field("PortIfindex", &val),
                AttrOption::ArrayIndex(val) => fmt.field("ArrayIndex", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableAttrOption<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("AttrOption", offset));
            return (
                stack,
                missing_type.and_then(|t| AttrOption::attr_from_type(t)),
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
                AttrOption::Name(val) => {
                    if last_off == offset {
                        stack.push(("Name", last_off));
                        break;
                    }
                }
                AttrOption::Changed(val) => {
                    if last_off == offset {
                        stack.push(("Changed", last_off));
                        break;
                    }
                }
                AttrOption::Type(val) => {
                    if last_off == offset {
                        stack.push(("Type", last_off));
                        break;
                    }
                }
                AttrOption::Data(val) => {
                    if last_off == offset {
                        stack.push(("Data", last_off));
                        break;
                    }
                }
                AttrOption::Removed(val) => {
                    if last_off == offset {
                        stack.push(("Removed", last_off));
                        break;
                    }
                }
                AttrOption::PortIfindex(val) => {
                    if last_off == offset {
                        stack.push(("PortIfindex", last_off));
                        break;
                    }
                }
                AttrOption::ArrayIndex(val) => {
                    if last_off == offset {
                        stack.push(("ArrayIndex", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("AttrOption", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ItemPort<'a> {
    Port(IterableAttrPort<'a>),
}
impl<'a> IterableItemPort<'a> {
    pub fn get_port(&self) -> Result<IterableAttrPort<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ItemPort::Port(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ItemPort",
            "Port",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ItemPort<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableItemPort<'a> {
        IterableItemPort::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "PortUnspec",
            1u16 => "Port",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableItemPort<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableItemPort<'a> {
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
impl<'a> Iterator for IterableItemPort<'a> {
    type Item = Result<ItemPort<'a>, ErrorContext>;
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
                1u16 => ItemPort::Port({
                    let res = Some(IterableAttrPort::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "ItemPort",
            r#type.and_then(|t| ItemPort::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableItemPort<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ItemPort");
        let mut iter = IterableItemPort::with_loc(&[], self.orig_loc);
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
                ItemPort::Port(val) => fmt.field("Port", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableItemPort<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ItemPort", offset));
            return (
                stack,
                missing_type.and_then(|t| ItemPort::attr_from_type(t)),
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
                ItemPort::Port(val) => {
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
            stack.push(("ItemPort", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum AttrPort {
    Ifindex(u32),
    Changed(()),
    Linkup(()),
    Speed(u32),
    Duplex(u8),
    Removed(()),
}
impl<'a> IterableAttrPort<'a> {
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AttrPort::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AttrPort",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_changed(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AttrPort::Changed(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AttrPort",
            "Changed",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_linkup(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AttrPort::Linkup(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AttrPort",
            "Linkup",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_speed(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AttrPort::Speed(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AttrPort",
            "Speed",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_duplex(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AttrPort::Duplex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AttrPort",
            "Duplex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_removed(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(AttrPort::Removed(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "AttrPort",
            "Removed",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl AttrPort {
    pub fn new<'a>(buf: &'a [u8]) -> IterableAttrPort<'a> {
        IterableAttrPort::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "Unspec",
            1u16 => "Ifindex",
            2u16 => "Changed",
            3u16 => "Linkup",
            4u16 => "Speed",
            5u16 => "Duplex",
            6u16 => "Removed",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableAttrPort<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableAttrPort<'a> {
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
impl<'a> Iterator for IterableAttrPort<'a> {
    type Item = Result<AttrPort, ErrorContext>;
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
                1u16 => AttrPort::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => AttrPort::Changed(()),
                3u16 => AttrPort::Linkup(()),
                4u16 => AttrPort::Speed({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => AttrPort::Duplex({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => AttrPort::Removed(()),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "AttrPort",
            r#type.and_then(|t| AttrPort::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableAttrPort<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("AttrPort");
        let mut iter = IterableAttrPort::with_loc(&[], self.orig_loc);
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
                AttrPort::Ifindex(val) => fmt.field("Ifindex", &val),
                AttrPort::Changed(val) => fmt.field("Changed", &val),
                AttrPort::Linkup(val) => fmt.field("Linkup", &val),
                AttrPort::Speed(val) => fmt.field("Speed", &val),
                AttrPort::Duplex(val) => fmt.field("Duplex", &val),
                AttrPort::Removed(val) => fmt.field("Removed", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableAttrPort<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("AttrPort", offset));
            return (
                stack,
                missing_type.and_then(|t| AttrPort::attr_from_type(t)),
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
                AttrPort::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                AttrPort::Changed(val) => {
                    if last_off == offset {
                        stack.push(("Changed", last_off));
                        break;
                    }
                }
                AttrPort::Linkup(val) => {
                    if last_off == offset {
                        stack.push(("Linkup", last_off));
                        break;
                    }
                }
                AttrPort::Speed(val) => {
                    if last_off == offset {
                        stack.push(("Speed", last_off));
                        break;
                    }
                }
                AttrPort::Duplex(val) => {
                    if last_off == offset {
                        stack.push(("Duplex", last_off));
                        break;
                    }
                }
                AttrPort::Removed(val) => {
                    if last_off == offset {
                        stack.push(("Removed", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("AttrPort", cur));
        }
        (stack, None)
    }
}
pub struct PushTeam<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushTeam<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushTeam<Prev> {
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
    pub fn push_team_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_list_option(mut self) -> PushItemOption<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushItemOption {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_list_port(mut self) -> PushItemPort<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 3u16);
        PushItemPort {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushTeam<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushItemOption<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushItemOption<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushItemOption<Prev> {
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
    pub fn nested_option(mut self) -> PushAttrOption<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushAttrOption {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushItemOption<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushAttrOption<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushAttrOption<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushAttrOption<Prev> {
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
    pub fn push_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_changed(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 2u16, 0 as u16);
        self
    }
    pub fn push_type(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 3u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_data(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 4u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_removed(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 5u16, 0 as u16);
        self
    }
    #[doc = "for per-port options\n"]
    pub fn push_port_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "for array options\n"]
    pub fn push_array_index(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushAttrOption<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushItemPort<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushItemPort<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushItemPort<Prev> {
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
    pub fn nested_port(mut self) -> PushAttrPort<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushAttrPort {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushItemPort<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushAttrPort<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushAttrPort<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushAttrPort<Prev> {
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
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_changed(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 2u16, 0 as u16);
        self
    }
    pub fn push_linkup(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 3u16, 0 as u16);
        self
    }
    pub fn push_speed(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_duplex(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 5u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_removed(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 6u16, 0 as u16);
        self
    }
}
impl<Prev: Pusher> Drop for PushAttrPort<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "No operation\n\nReply attributes:\n- [.get_team_ifindex()](IterableTeam::get_team_ifindex)\n\n"]
#[derive(Debug)]
pub struct OpNoopDo<'r> {
    request: Request<'r>,
}
impl<'r> OpNoopDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushTeam<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushTeam::new(buf)
    }
    pub fn encode(&mut self) -> PushTeam<&mut Vec<u8>> {
        PushTeam::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushTeam<RequestBuf<'r>> {
        PushTeam::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableTeam<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableTeam::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 0u8;
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
impl NetlinkRequest for OpNoopDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("team".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableTeam<'buf>;
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
#[doc = "Set team options\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_team_ifindex()](PushTeam::push_team_ifindex)\n- [.nested_list_option()](PushTeam::nested_list_option)\n\nReply attributes:\n- [.get_team_ifindex()](IterableTeam::get_team_ifindex)\n- [.get_list_option()](IterableTeam::get_list_option)\n\n"]
#[derive(Debug)]
pub struct OpOptionsSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpOptionsSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushTeam<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushTeam::new(buf)
    }
    pub fn encode(&mut self) -> PushTeam<&mut Vec<u8>> {
        PushTeam::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushTeam<RequestBuf<'r>> {
        PushTeam::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableTeam<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableTeam::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpOptionsSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("team".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableTeam<'buf>;
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
#[doc = "Get team options info\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_team_ifindex()](PushTeam::push_team_ifindex)\n\nReply attributes:\n- [.get_team_ifindex()](IterableTeam::get_team_ifindex)\n- [.get_list_option()](IterableTeam::get_list_option)\n\n"]
#[derive(Debug)]
pub struct OpOptionsGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpOptionsGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushTeam<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushTeam::new(buf)
    }
    pub fn encode(&mut self) -> PushTeam<&mut Vec<u8>> {
        PushTeam::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushTeam<RequestBuf<'r>> {
        PushTeam::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableTeam<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableTeam::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpOptionsGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("team".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableTeam<'buf>;
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
#[doc = "Get team ports info\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_team_ifindex()](PushTeam::push_team_ifindex)\n\nReply attributes:\n- [.get_team_ifindex()](IterableTeam::get_team_ifindex)\n- [.get_list_port()](IterableTeam::get_list_port)\n\n"]
#[derive(Debug)]
pub struct OpPortListGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPortListGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushTeam<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushTeam::new(buf)
    }
    pub fn encode(&mut self) -> PushTeam<&mut Vec<u8>> {
        PushTeam::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushTeam<RequestBuf<'r>> {
        PushTeam::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableTeam<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableTeam::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpPortListGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("team".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableTeam<'buf>;
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
    #[doc = "No operation\n\nReply attributes:\n- [.get_team_ifindex()](IterableTeam::get_team_ifindex)\n\n"]
    pub fn op_noop_do(self) -> OpNoopDo<'buf> {
        let mut res = OpNoopDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-noop-do", OpNoopDo::lookup);
        res
    }
    #[doc = "Set team options\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_team_ifindex()](PushTeam::push_team_ifindex)\n- [.nested_list_option()](PushTeam::nested_list_option)\n\nReply attributes:\n- [.get_team_ifindex()](IterableTeam::get_team_ifindex)\n- [.get_list_option()](IterableTeam::get_list_option)\n\n"]
    pub fn op_options_set_do(self) -> OpOptionsSetDo<'buf> {
        let mut res = OpOptionsSetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-options-set-do", OpOptionsSetDo::lookup);
        res
    }
    #[doc = "Get team options info\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_team_ifindex()](PushTeam::push_team_ifindex)\n\nReply attributes:\n- [.get_team_ifindex()](IterableTeam::get_team_ifindex)\n- [.get_list_option()](IterableTeam::get_list_option)\n\n"]
    pub fn op_options_get_do(self) -> OpOptionsGetDo<'buf> {
        let mut res = OpOptionsGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-options-get-do", OpOptionsGetDo::lookup);
        res
    }
    #[doc = "Get team ports info\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_team_ifindex()](PushTeam::push_team_ifindex)\n\nReply attributes:\n- [.get_team_ifindex()](IterableTeam::get_team_ifindex)\n- [.get_list_port()](IterableTeam::get_list_port)\n\n"]
    pub fn op_port_list_get_do(self) -> OpPortListGetDo<'buf> {
        let mut res = OpPortListGetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-port-list-get-do",
            OpPortListGetDo::lookup,
        );
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableTeam::get_list_option;
        let _ = IterableTeam::get_list_port;
        let _ = IterableTeam::get_team_ifindex;
        let _ = PushTeam::<&mut Vec<u8>>::nested_list_option;
        let _ = PushTeam::<&mut Vec<u8>>::push_team_ifindex;
    }
}
