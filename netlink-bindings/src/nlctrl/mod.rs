#![doc = "genetlink meta-family that exposes information about all genetlink\nfamilies registered in the kernel (including itself).\n"]
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
pub const PROTONAME: &str = "nlctrl";
pub const PROTONAME_CSTR: &CStr = c"nlctrl";
pub const PROTONUM: u16 = 0x10;
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum OpFlags {
    AdminPerm = 1 << 0,
    CmdCapDo = 1 << 1,
    CmdCapDump = 1 << 2,
    CmdCapHaspol = 1 << 3,
    UnsAdminPerm = 1 << 4,
}
impl OpFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::AdminPerm,
            n if n == 1 << 1 => Self::CmdCapDo,
            n if n == 1 << 2 => Self::CmdCapDump,
            n if n == 1 << 3 => Self::CmdCapHaspol,
            n if n == 1 << 4 => Self::UnsAdminPerm,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum AttrType {
    Invalid = 0,
    Flag = 1,
    U8 = 2,
    U16 = 3,
    U32 = 4,
    U64 = 5,
    S8 = 6,
    S16 = 7,
    S32 = 8,
    S64 = 9,
    Binary = 10,
    String = 11,
    NulString = 12,
    Nested = 13,
    NestedArray = 14,
    Bitfield32 = 15,
    Sint = 16,
    Uint = 17,
}
impl AttrType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Invalid,
            1 => Self::Flag,
            2 => Self::U8,
            3 => Self::U16,
            4 => Self::U32,
            5 => Self::U64,
            6 => Self::S8,
            7 => Self::S16,
            8 => Self::S32,
            9 => Self::S64,
            10 => Self::Binary,
            11 => Self::String,
            12 => Self::NulString,
            13 => Self::Nested,
            14 => Self::NestedArray,
            15 => Self::Bitfield32,
            16 => Self::Sint,
            17 => Self::Uint,
            _ => return None,
        })
    }
}
#[derive(Clone)]
pub enum CtrlAttrs<'a> {
    FamilyId(u16),
    FamilyName(&'a CStr),
    Version(u32),
    Hdrsize(u32),
    Maxattr(u32),
    Ops(IterableArrayOpAttrs<'a>),
    McastGroups(IterableArrayMcastGroupAttrs<'a>),
    Policy(IterablePolicyAttrs<'a>),
    OpPolicy(IterableOpPolicyAttrs<'a>),
    Op(u32),
}
impl<'a> IterableCtrlAttrs<'a> {
    pub fn get_family_id(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CtrlAttrs::FamilyId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CtrlAttrs",
            "FamilyId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_family_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CtrlAttrs::FamilyName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CtrlAttrs",
            "FamilyName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_version(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CtrlAttrs::Version(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CtrlAttrs",
            "Version",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_hdrsize(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CtrlAttrs::Hdrsize(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CtrlAttrs",
            "Hdrsize",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_maxattr(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CtrlAttrs::Maxattr(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CtrlAttrs",
            "Maxattr",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ops(
        &self,
    ) -> Result<ArrayIterable<IterableArrayOpAttrs<'a>, IterableOpAttrs<'a>>, ErrorContext> {
        for attr in self.clone() {
            if let Ok(CtrlAttrs::Ops(val)) = attr {
                return Ok(ArrayIterable::new(val));
            }
        }
        Err(ErrorContext::new_missing(
            "CtrlAttrs",
            "Ops",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mcast_groups(
        &self,
    ) -> Result<
        ArrayIterable<IterableArrayMcastGroupAttrs<'a>, IterableMcastGroupAttrs<'a>>,
        ErrorContext,
    > {
        for attr in self.clone() {
            if let Ok(CtrlAttrs::McastGroups(val)) = attr {
                return Ok(ArrayIterable::new(val));
            }
        }
        Err(ErrorContext::new_missing(
            "CtrlAttrs",
            "McastGroups",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_policy(&self) -> Result<IterablePolicyAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CtrlAttrs::Policy(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CtrlAttrs",
            "Policy",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_op_policy(&self) -> Result<IterableOpPolicyAttrs<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CtrlAttrs::OpPolicy(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CtrlAttrs",
            "OpPolicy",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_op(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(CtrlAttrs::Op(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "CtrlAttrs",
            "Op",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpAttrs {
    pub fn new_array(buf: &[u8]) -> IterableArrayOpAttrs<'_> {
        IterableArrayOpAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableArrayOpAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableArrayOpAttrs<'a> {
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
impl<'a> Iterator for IterableArrayOpAttrs<'a> {
    type Item = Result<IterableOpAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            {
                return Some(Ok(IterableOpAttrs::with_loc(next, self.orig_loc)));
            }
        }
        let pos = self.pos;
        self.pos = self.buf.len();
        Some(Err(ErrorContext::new(
            "OpAttrs",
            None,
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> McastGroupAttrs<'a> {
    pub fn new_array(buf: &[u8]) -> IterableArrayMcastGroupAttrs<'_> {
        IterableArrayMcastGroupAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableArrayMcastGroupAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableArrayMcastGroupAttrs<'a> {
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
impl<'a> Iterator for IterableArrayMcastGroupAttrs<'a> {
    type Item = Result<IterableMcastGroupAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            {
                return Some(Ok(IterableMcastGroupAttrs::with_loc(next, self.orig_loc)));
            }
        }
        let pos = self.pos;
        self.pos = self.buf.len();
        Some(Err(ErrorContext::new(
            "McastGroupAttrs",
            None,
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl CtrlAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableCtrlAttrs<'a> {
        IterableCtrlAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "FamilyId",
            2u16 => "FamilyName",
            3u16 => "Version",
            4u16 => "Hdrsize",
            5u16 => "Maxattr",
            6u16 => "Ops",
            7u16 => "McastGroups",
            8u16 => "Policy",
            9u16 => "OpPolicy",
            10u16 => "Op",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableCtrlAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableCtrlAttrs<'a> {
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
impl<'a> Iterator for IterableCtrlAttrs<'a> {
    type Item = Result<CtrlAttrs<'a>, ErrorContext>;
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
                1u16 => CtrlAttrs::FamilyId({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => CtrlAttrs::FamilyName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => CtrlAttrs::Version({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => CtrlAttrs::Hdrsize({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => CtrlAttrs::Maxattr({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => CtrlAttrs::Ops({
                    let res = Some(IterableArrayOpAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => CtrlAttrs::McastGroups({
                    let res = Some(IterableArrayMcastGroupAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => CtrlAttrs::Policy({
                    let res = Some(IterablePolicyAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => CtrlAttrs::OpPolicy({
                    let res = Some(IterableOpPolicyAttrs::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => CtrlAttrs::Op({
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
            "CtrlAttrs",
            r#type.and_then(|t| CtrlAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableArrayOpAttrs<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_list()
            .entries(self.clone().map(FlattenErrorContext))
            .finish()
    }
}
impl std::fmt::Debug for IterableArrayMcastGroupAttrs<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_list()
            .entries(self.clone().map(FlattenErrorContext))
            .finish()
    }
}
impl<'a> std::fmt::Debug for IterableCtrlAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("CtrlAttrs");
        let mut iter = IterableCtrlAttrs::with_loc(&[], self.orig_loc);
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
                CtrlAttrs::FamilyId(val) => fmt.field("FamilyId", &val),
                CtrlAttrs::FamilyName(val) => fmt.field("FamilyName", &val),
                CtrlAttrs::Version(val) => fmt.field("Version", &val),
                CtrlAttrs::Hdrsize(val) => fmt.field("Hdrsize", &val),
                CtrlAttrs::Maxattr(val) => fmt.field("Maxattr", &val),
                CtrlAttrs::Ops(val) => fmt.field("Ops", &val),
                CtrlAttrs::McastGroups(val) => fmt.field("McastGroups", &val),
                CtrlAttrs::Policy(val) => fmt.field("Policy", &val),
                CtrlAttrs::OpPolicy(val) => fmt.field("OpPolicy", &val),
                CtrlAttrs::Op(val) => fmt.field("Op", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableCtrlAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("CtrlAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| CtrlAttrs::attr_from_type(t)),
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
                CtrlAttrs::FamilyId(val) => {
                    if last_off == offset {
                        stack.push(("FamilyId", last_off));
                        break;
                    }
                }
                CtrlAttrs::FamilyName(val) => {
                    if last_off == offset {
                        stack.push(("FamilyName", last_off));
                        break;
                    }
                }
                CtrlAttrs::Version(val) => {
                    if last_off == offset {
                        stack.push(("Version", last_off));
                        break;
                    }
                }
                CtrlAttrs::Hdrsize(val) => {
                    if last_off == offset {
                        stack.push(("Hdrsize", last_off));
                        break;
                    }
                }
                CtrlAttrs::Maxattr(val) => {
                    if last_off == offset {
                        stack.push(("Maxattr", last_off));
                        break;
                    }
                }
                CtrlAttrs::Ops(val) => {
                    for entry in val {
                        let Ok(attr) = entry else { break };
                        (stack, missing) = attr.lookup_attr(offset, missing_type);
                        if !stack.is_empty() {
                            break;
                        }
                    }
                    if !stack.is_empty() {
                        stack.push(("Ops", last_off));
                        break;
                    }
                }
                CtrlAttrs::McastGroups(val) => {
                    for entry in val {
                        let Ok(attr) = entry else { break };
                        (stack, missing) = attr.lookup_attr(offset, missing_type);
                        if !stack.is_empty() {
                            break;
                        }
                    }
                    if !stack.is_empty() {
                        stack.push(("McastGroups", last_off));
                        break;
                    }
                }
                CtrlAttrs::Policy(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                CtrlAttrs::OpPolicy(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                CtrlAttrs::Op(val) => {
                    if last_off == offset {
                        stack.push(("Op", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("CtrlAttrs", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum McastGroupAttrs<'a> {
    Name(&'a CStr),
    Id(u32),
}
impl<'a> IterableMcastGroupAttrs<'a> {
    pub fn get_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(McastGroupAttrs::Name(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "McastGroupAttrs",
            "Name",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(McastGroupAttrs::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "McastGroupAttrs",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl McastGroupAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableMcastGroupAttrs<'a> {
        IterableMcastGroupAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Name",
            2u16 => "Id",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableMcastGroupAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableMcastGroupAttrs<'a> {
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
impl<'a> Iterator for IterableMcastGroupAttrs<'a> {
    type Item = Result<McastGroupAttrs<'a>, ErrorContext>;
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
                1u16 => McastGroupAttrs::Name({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => McastGroupAttrs::Id({
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
            "McastGroupAttrs",
            r#type.and_then(|t| McastGroupAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableMcastGroupAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("McastGroupAttrs");
        let mut iter = IterableMcastGroupAttrs::with_loc(&[], self.orig_loc);
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
                McastGroupAttrs::Name(val) => fmt.field("Name", &val),
                McastGroupAttrs::Id(val) => fmt.field("Id", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableMcastGroupAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("McastGroupAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| McastGroupAttrs::attr_from_type(t)),
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
                McastGroupAttrs::Name(val) => {
                    if last_off == offset {
                        stack.push(("Name", last_off));
                        break;
                    }
                }
                McastGroupAttrs::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("McastGroupAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum OpAttrs {
    Id(u32),
    #[doc = "Associated type: [`OpFlags`] (1 bit per enumeration)"]
    Flags(u32),
}
impl<'a> IterableOpAttrs<'a> {
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(OpAttrs::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpAttrs",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`OpFlags`] (1 bit per enumeration)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(OpAttrs::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpAttrs",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpAttrs<'a> {
        IterableOpAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Id",
            2u16 => "Flags",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpAttrs<'a> {
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
impl<'a> Iterator for IterableOpAttrs<'a> {
    type Item = Result<OpAttrs, ErrorContext>;
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
                1u16 => OpAttrs::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpAttrs::Flags({
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
            "OpAttrs",
            r#type.and_then(|t| OpAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpAttrs");
        let mut iter = IterableOpAttrs::with_loc(&[], self.orig_loc);
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
                OpAttrs::Id(val) => fmt.field("Id", &val),
                OpAttrs::Flags(val) => {
                    fmt.field("Flags", &FormatFlags(val.into(), OpFlags::from_value))
                }
            };
        }
        fmt.finish()
    }
}
impl IterableOpAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("OpAttrs", offset));
            return (stack, missing_type.and_then(|t| OpAttrs::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                OpAttrs::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                OpAttrs::Flags(val) => {
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
            stack.push(("OpAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum PolicyAttrs<'a> {
    #[doc = "Associated type: [`AttrType`] (enum)"]
    Type(u32),
    MinValueS(i64),
    MaxValueS(i64),
    MinValueU(u64),
    MaxValueU(u64),
    MinLength(u32),
    MaxLength(u32),
    PolicyIdx(u32),
    PolicyMaxtype(u32),
    Bitfield32Mask(u32),
    Mask(u64),
    Pad(&'a [u8]),
}
impl<'a> IterablePolicyAttrs<'a> {
    #[doc = "Associated type: [`AttrType`] (enum)"]
    pub fn get_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PolicyAttrs::Type(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PolicyAttrs",
            "Type",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_min_value_s(&self) -> Result<i64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PolicyAttrs::MinValueS(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PolicyAttrs",
            "MinValueS",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_value_s(&self) -> Result<i64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PolicyAttrs::MaxValueS(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PolicyAttrs",
            "MaxValueS",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_min_value_u(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PolicyAttrs::MinValueU(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PolicyAttrs",
            "MinValueU",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_value_u(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PolicyAttrs::MaxValueU(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PolicyAttrs",
            "MaxValueU",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_min_length(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PolicyAttrs::MinLength(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PolicyAttrs",
            "MinLength",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_max_length(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PolicyAttrs::MaxLength(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PolicyAttrs",
            "MaxLength",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_policy_idx(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PolicyAttrs::PolicyIdx(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PolicyAttrs",
            "PolicyIdx",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_policy_maxtype(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PolicyAttrs::PolicyMaxtype(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PolicyAttrs",
            "PolicyMaxtype",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_bitfield32_mask(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PolicyAttrs::Bitfield32Mask(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PolicyAttrs",
            "Bitfield32Mask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_mask(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PolicyAttrs::Mask(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PolicyAttrs",
            "Mask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PolicyAttrs::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PolicyAttrs",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl PolicyAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePolicyAttrs<'a> {
        IterablePolicyAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Type",
            2u16 => "MinValueS",
            3u16 => "MaxValueS",
            4u16 => "MinValueU",
            5u16 => "MaxValueU",
            6u16 => "MinLength",
            7u16 => "MaxLength",
            8u16 => "PolicyIdx",
            9u16 => "PolicyMaxtype",
            10u16 => "Bitfield32Mask",
            11u16 => "Mask",
            12u16 => "Pad",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePolicyAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePolicyAttrs<'a> {
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
impl<'a> Iterator for IterablePolicyAttrs<'a> {
    type Item = Result<PolicyAttrs<'a>, ErrorContext>;
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
                1u16 => PolicyAttrs::Type({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => PolicyAttrs::MinValueS({
                    let res = parse_i64(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => PolicyAttrs::MaxValueS({
                    let res = parse_i64(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => PolicyAttrs::MinValueU({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => PolicyAttrs::MaxValueU({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => PolicyAttrs::MinLength({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => PolicyAttrs::MaxLength({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => PolicyAttrs::PolicyIdx({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => PolicyAttrs::PolicyMaxtype({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => PolicyAttrs::Bitfield32Mask({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => PolicyAttrs::Mask({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => PolicyAttrs::Pad({
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
            "PolicyAttrs",
            r#type.and_then(|t| PolicyAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterablePolicyAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("PolicyAttrs");
        let mut iter = IterablePolicyAttrs::with_loc(&[], self.orig_loc);
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
                PolicyAttrs::Type(val) => {
                    fmt.field("Type", &FormatEnum(val.into(), AttrType::from_value))
                }
                PolicyAttrs::MinValueS(val) => fmt.field("MinValueS", &val),
                PolicyAttrs::MaxValueS(val) => fmt.field("MaxValueS", &val),
                PolicyAttrs::MinValueU(val) => fmt.field("MinValueU", &val),
                PolicyAttrs::MaxValueU(val) => fmt.field("MaxValueU", &val),
                PolicyAttrs::MinLength(val) => fmt.field("MinLength", &val),
                PolicyAttrs::MaxLength(val) => fmt.field("MaxLength", &val),
                PolicyAttrs::PolicyIdx(val) => fmt.field("PolicyIdx", &val),
                PolicyAttrs::PolicyMaxtype(val) => fmt.field("PolicyMaxtype", &val),
                PolicyAttrs::Bitfield32Mask(val) => fmt.field("Bitfield32Mask", &val),
                PolicyAttrs::Mask(val) => fmt.field("Mask", &val),
                PolicyAttrs::Pad(val) => fmt.field("Pad", &val),
            };
        }
        fmt.finish()
    }
}
impl IterablePolicyAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("PolicyAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| PolicyAttrs::attr_from_type(t)),
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
                PolicyAttrs::Type(val) => {
                    if last_off == offset {
                        stack.push(("Type", last_off));
                        break;
                    }
                }
                PolicyAttrs::MinValueS(val) => {
                    if last_off == offset {
                        stack.push(("MinValueS", last_off));
                        break;
                    }
                }
                PolicyAttrs::MaxValueS(val) => {
                    if last_off == offset {
                        stack.push(("MaxValueS", last_off));
                        break;
                    }
                }
                PolicyAttrs::MinValueU(val) => {
                    if last_off == offset {
                        stack.push(("MinValueU", last_off));
                        break;
                    }
                }
                PolicyAttrs::MaxValueU(val) => {
                    if last_off == offset {
                        stack.push(("MaxValueU", last_off));
                        break;
                    }
                }
                PolicyAttrs::MinLength(val) => {
                    if last_off == offset {
                        stack.push(("MinLength", last_off));
                        break;
                    }
                }
                PolicyAttrs::MaxLength(val) => {
                    if last_off == offset {
                        stack.push(("MaxLength", last_off));
                        break;
                    }
                }
                PolicyAttrs::PolicyIdx(val) => {
                    if last_off == offset {
                        stack.push(("PolicyIdx", last_off));
                        break;
                    }
                }
                PolicyAttrs::PolicyMaxtype(val) => {
                    if last_off == offset {
                        stack.push(("PolicyMaxtype", last_off));
                        break;
                    }
                }
                PolicyAttrs::Bitfield32Mask(val) => {
                    if last_off == offset {
                        stack.push(("Bitfield32Mask", last_off));
                        break;
                    }
                }
                PolicyAttrs::Mask(val) => {
                    if last_off == offset {
                        stack.push(("Mask", last_off));
                        break;
                    }
                }
                PolicyAttrs::Pad(val) => {
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
            stack.push(("PolicyAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum OpPolicyAttrs {
    Do(u32),
    Dump(u32),
}
impl<'a> IterableOpPolicyAttrs<'a> {
    pub fn get_do(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(OpPolicyAttrs::Do(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPolicyAttrs",
            "Do",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dump(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(OpPolicyAttrs::Dump(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPolicyAttrs",
            "Dump",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpPolicyAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpPolicyAttrs<'a> {
        IterableOpPolicyAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Do",
            2u16 => "Dump",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpPolicyAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpPolicyAttrs<'a> {
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
impl<'a> Iterator for IterableOpPolicyAttrs<'a> {
    type Item = Result<OpPolicyAttrs, ErrorContext>;
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
                1u16 => OpPolicyAttrs::Do({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpPolicyAttrs::Dump({
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
            "OpPolicyAttrs",
            r#type.and_then(|t| OpPolicyAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpPolicyAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpPolicyAttrs");
        let mut iter = IterableOpPolicyAttrs::with_loc(&[], self.orig_loc);
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
                OpPolicyAttrs::Do(val) => fmt.field("Do", &val),
                OpPolicyAttrs::Dump(val) => fmt.field("Dump", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpPolicyAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("OpPolicyAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| OpPolicyAttrs::attr_from_type(t)),
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
                OpPolicyAttrs::Do(val) => {
                    if last_off == offset {
                        stack.push(("Do", last_off));
                        break;
                    }
                }
                OpPolicyAttrs::Dump(val) => {
                    if last_off == offset {
                        stack.push(("Dump", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpPolicyAttrs", cur));
        }
        (stack, None)
    }
}
pub struct PushCtrlAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushCtrlAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
pub struct PushArrayOpAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
    pub(crate) counter: u16,
}
impl<Prev: Pusher> Pusher for PushArrayOpAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushArrayOpAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
            counter: 0,
        }
    }
    pub fn end_array(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_vec_mut(), *header_offset);
        }
        prev
    }
    pub fn entry_nested(mut self) -> PushOpAttrs<Self> {
        let index = self.counter;
        self.counter += 1;
        let header_offset = push_nested_header(self.as_vec_mut(), index);
        PushOpAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushArrayOpAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushArrayMcastGroupAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
    pub(crate) counter: u16,
}
impl<Prev: Pusher> Pusher for PushArrayMcastGroupAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushArrayMcastGroupAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
            counter: 0,
        }
    }
    pub fn end_array(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_vec_mut(), *header_offset);
        }
        prev
    }
    pub fn entry_nested(mut self) -> PushMcastGroupAttrs<Self> {
        let index = self.counter;
        self.counter += 1;
        let header_offset = push_nested_header(self.as_vec_mut(), index);
        PushMcastGroupAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushArrayMcastGroupAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
impl<Prev: Pusher> PushCtrlAttrs<Prev> {
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
    pub fn push_family_id(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 1u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_family_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_family_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn push_version(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_hdrsize(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_maxattr(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn array_ops(mut self) -> PushArrayOpAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 6u16);
        PushArrayOpAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
            counter: 0,
        }
    }
    pub fn array_mcast_groups(mut self) -> PushArrayMcastGroupAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 7u16);
        PushArrayMcastGroupAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
            counter: 0,
        }
    }
    pub fn nested_policy(mut self) -> PushPolicyAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 8u16);
        PushPolicyAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_op_policy(mut self) -> PushOpPolicyAttrs<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 9u16);
        PushOpPolicyAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_op(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushCtrlAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushMcastGroupAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushMcastGroupAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushMcastGroupAttrs<Prev> {
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
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushMcastGroupAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushOpAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushOpAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushOpAttrs<Prev> {
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
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`OpFlags`] (1 bit per enumeration)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushOpAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPolicyAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushPolicyAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushPolicyAttrs<Prev> {
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
    #[doc = "Associated type: [`AttrType`] (enum)"]
    pub fn push_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_min_value_s(mut self, value: i64) -> Self {
        push_header(self.as_vec_mut(), 2u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_value_s(mut self, value: i64) -> Self {
        push_header(self.as_vec_mut(), 3u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_min_value_u(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 4u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_value_u(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 5u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_min_length(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_length(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_policy_idx(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_policy_maxtype(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_bitfield32_mask(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mask(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 11u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 12u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushPolicyAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushOpPolicyAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushOpPolicyAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushOpPolicyAttrs<Prev> {
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
    pub fn push_do(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dump(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushOpPolicyAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get / dump genetlink families\n\nReply attributes:\n- [.get_family_id()](IterableCtrlAttrs::get_family_id)\n- [.get_family_name()](IterableCtrlAttrs::get_family_name)\n- [.get_version()](IterableCtrlAttrs::get_version)\n- [.get_hdrsize()](IterableCtrlAttrs::get_hdrsize)\n- [.get_maxattr()](IterableCtrlAttrs::get_maxattr)\n- [.get_ops()](IterableCtrlAttrs::get_ops)\n- [.get_mcast_groups()](IterableCtrlAttrs::get_mcast_groups)\n\n"]
#[derive(Debug)]
pub struct OpGetfamilyDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetfamilyDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushCtrlAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushCtrlAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushCtrlAttrs<&mut Vec<u8>> {
        PushCtrlAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushCtrlAttrs<RequestBuf<'r>> {
        PushCtrlAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableCtrlAttrs<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableCtrlAttrs::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpGetfamilyDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0x10,
            request_type: 0x10,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableCtrlAttrs<'buf>;
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
#[doc = "Get / dump genetlink families\n\nRequest attributes:\n- [.push_family_name()](PushCtrlAttrs::push_family_name)\n\nReply attributes:\n- [.get_family_id()](IterableCtrlAttrs::get_family_id)\n- [.get_family_name()](IterableCtrlAttrs::get_family_name)\n- [.get_version()](IterableCtrlAttrs::get_version)\n- [.get_hdrsize()](IterableCtrlAttrs::get_hdrsize)\n- [.get_maxattr()](IterableCtrlAttrs::get_maxattr)\n- [.get_ops()](IterableCtrlAttrs::get_ops)\n- [.get_mcast_groups()](IterableCtrlAttrs::get_mcast_groups)\n\n"]
#[derive(Debug)]
pub struct OpGetfamilyDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetfamilyDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushCtrlAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushCtrlAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushCtrlAttrs<&mut Vec<u8>> {
        PushCtrlAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushCtrlAttrs<RequestBuf<'r>> {
        PushCtrlAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableCtrlAttrs<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableCtrlAttrs::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpGetfamilyDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0x10,
            request_type: 0x10,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableCtrlAttrs<'buf>;
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
#[doc = "Get / dump genetlink policies\n\nRequest attributes:\n- [.push_family_id()](PushCtrlAttrs::push_family_id)\n- [.push_family_name()](PushCtrlAttrs::push_family_name)\n- [.push_op()](PushCtrlAttrs::push_op)\n\nReply attributes:\n- [.get_family_id()](IterableCtrlAttrs::get_family_id)\n- [.get_policy()](IterableCtrlAttrs::get_policy)\n- [.get_op_policy()](IterableCtrlAttrs::get_op_policy)\n\n"]
#[derive(Debug)]
pub struct OpGetpolicyDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetpolicyDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushCtrlAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushCtrlAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushCtrlAttrs<&mut Vec<u8>> {
        PushCtrlAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushCtrlAttrs<RequestBuf<'r>> {
        PushCtrlAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableCtrlAttrs<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableCtrlAttrs::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpGetpolicyDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0x10,
            request_type: 0x10,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableCtrlAttrs<'buf>;
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
    #[doc = "Get / dump genetlink families\n\nReply attributes:\n- [.get_family_id()](IterableCtrlAttrs::get_family_id)\n- [.get_family_name()](IterableCtrlAttrs::get_family_name)\n- [.get_version()](IterableCtrlAttrs::get_version)\n- [.get_hdrsize()](IterableCtrlAttrs::get_hdrsize)\n- [.get_maxattr()](IterableCtrlAttrs::get_maxattr)\n- [.get_ops()](IterableCtrlAttrs::get_ops)\n- [.get_mcast_groups()](IterableCtrlAttrs::get_mcast_groups)\n\n"]
    pub fn op_getfamily_dump(self) -> OpGetfamilyDump<'buf> {
        let mut res = OpGetfamilyDump::new(self);
        res.request
            .do_writeback(res.protocol(), "op-getfamily-dump", OpGetfamilyDump::lookup);
        res
    }
    #[doc = "Get / dump genetlink families\n\nRequest attributes:\n- [.push_family_name()](PushCtrlAttrs::push_family_name)\n\nReply attributes:\n- [.get_family_id()](IterableCtrlAttrs::get_family_id)\n- [.get_family_name()](IterableCtrlAttrs::get_family_name)\n- [.get_version()](IterableCtrlAttrs::get_version)\n- [.get_hdrsize()](IterableCtrlAttrs::get_hdrsize)\n- [.get_maxattr()](IterableCtrlAttrs::get_maxattr)\n- [.get_ops()](IterableCtrlAttrs::get_ops)\n- [.get_mcast_groups()](IterableCtrlAttrs::get_mcast_groups)\n\n"]
    pub fn op_getfamily_do(self) -> OpGetfamilyDo<'buf> {
        let mut res = OpGetfamilyDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-getfamily-do", OpGetfamilyDo::lookup);
        res
    }
    #[doc = "Get / dump genetlink policies\n\nRequest attributes:\n- [.push_family_id()](PushCtrlAttrs::push_family_id)\n- [.push_family_name()](PushCtrlAttrs::push_family_name)\n- [.push_op()](PushCtrlAttrs::push_op)\n\nReply attributes:\n- [.get_family_id()](IterableCtrlAttrs::get_family_id)\n- [.get_policy()](IterableCtrlAttrs::get_policy)\n- [.get_op_policy()](IterableCtrlAttrs::get_op_policy)\n\n"]
    pub fn op_getpolicy_dump(self) -> OpGetpolicyDump<'buf> {
        let mut res = OpGetpolicyDump::new(self);
        res.request
            .do_writeback(res.protocol(), "op-getpolicy-dump", OpGetpolicyDump::lookup);
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableCtrlAttrs::get_family_id;
        let _ = IterableCtrlAttrs::get_family_name;
        let _ = IterableCtrlAttrs::get_hdrsize;
        let _ = IterableCtrlAttrs::get_maxattr;
        let _ = IterableCtrlAttrs::get_mcast_groups;
        let _ = IterableCtrlAttrs::get_op_policy;
        let _ = IterableCtrlAttrs::get_ops;
        let _ = IterableCtrlAttrs::get_policy;
        let _ = IterableCtrlAttrs::get_version;
        let _ = PushCtrlAttrs::<&mut Vec<u8>>::push_family_id;
        let _ = PushCtrlAttrs::<&mut Vec<u8>>::push_family_name;
        let _ = PushCtrlAttrs::<&mut Vec<u8>>::push_op;
    }
}
