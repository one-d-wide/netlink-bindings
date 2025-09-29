#![doc = "genetlink meta-family that exposes information about all genetlink\nfamilies registered in the kernel (including itself).\n"]
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
pub const PROTONAME: &CStr = c"nlctrl";
#[doc = "Original name: \"op-flags\" (flags) - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Clone)]
pub enum OpFlags {
    AdminPerm = 1 << 0,
    CmdCapDo = 1 << 1,
    CmdCapDump = 1 << 2,
    CmdCapHaspol = 1 << 3,
    UnsAdminPerm = 1 << 4,
}
#[doc = "Original name: \"attr-type\" (enum) - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Clone)]
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
#[doc = "Original name: \"ctrl-attrs\""]
#[derive(Clone)]
pub enum CtrlAttrs<'a> {
    FamilyId(u16),
    FamilyName(&'a CStr),
    Version(u32),
    Hdrsize(u32),
    Maxattr(u32),
    Ops(Iterable<'a, Iterable<'a, OpAttrs>>),
    McastGroups(Iterable<'a, Iterable<'a, McastGroupAttrs<'a>>>),
    Policy(Iterable<'a, PolicyAttrs<'a>>),
    OpPolicy(Iterable<'a, OpPolicyAttrs>),
    Op(u32),
}
impl<'a> Iterable<'a, CtrlAttrs<'a>> {
    pub fn get_family_id(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let CtrlAttrs::FamilyId(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("CtrlAttrs", "FamilyId"))
    }
    pub fn get_family_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let CtrlAttrs::FamilyName(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("CtrlAttrs", "FamilyName"))
    }
    pub fn get_version(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let CtrlAttrs::Version(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("CtrlAttrs", "Version"))
    }
    pub fn get_hdrsize(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let CtrlAttrs::Hdrsize(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("CtrlAttrs", "Hdrsize"))
    }
    pub fn get_maxattr(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let CtrlAttrs::Maxattr(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("CtrlAttrs", "Maxattr"))
    }
    pub fn get_ops(
        &self,
    ) -> Result<
        ArrayIterable<Iterable<'a, Iterable<'a, OpAttrs>>, Iterable<'a, OpAttrs>>,
        ErrorContext,
    > {
        for attr in self.clone() {
            if let CtrlAttrs::Ops(val) = attr? {
                return Ok(ArrayIterable::new(val));
            }
        }
        Err(self.error_missing("CtrlAttrs", "Ops"))
    }
    pub fn get_mcast_groups(
        &self,
    ) -> Result<
        ArrayIterable<
            Iterable<'a, Iterable<'a, McastGroupAttrs<'a>>>,
            Iterable<'a, McastGroupAttrs<'a>>,
        >,
        ErrorContext,
    > {
        for attr in self.clone() {
            if let CtrlAttrs::McastGroups(val) = attr? {
                return Ok(ArrayIterable::new(val));
            }
        }
        Err(self.error_missing("CtrlAttrs", "McastGroups"))
    }
    pub fn get_policy(&self) -> Result<Iterable<'a, PolicyAttrs<'a>>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let CtrlAttrs::Policy(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("CtrlAttrs", "Policy"))
    }
    pub fn get_op_policy(&self) -> Result<Iterable<'a, OpPolicyAttrs>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let CtrlAttrs::OpPolicy(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("CtrlAttrs", "OpPolicy"))
    }
    pub fn get_op(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let CtrlAttrs::Op(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("CtrlAttrs", "Op"))
    }
}
impl<'a> OpAttrs {
    pub fn new_array(buf: &'a [u8]) -> Iterable<'a, Iterable<'a, OpAttrs>> {
        Iterable::new(buf)
    }
}
impl<'a> Iterator for Iterable<'a, Iterable<'a, OpAttrs>> {
    type Item = Result<Iterable<'a, OpAttrs>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            {
                return Some(Ok(Iterable::with_loc(next, self.orig_loc)));
            }
        }
        Some(Err(self.error_context(
            "OpAttrs",
            None,
            self.buf.as_ptr().wrapping_add(self.pos),
        )))
    }
}
impl<'a> McastGroupAttrs<'a> {
    pub fn new_array(buf: &'a [u8]) -> Iterable<'a, Iterable<'a, McastGroupAttrs<'a>>> {
        Iterable::new(buf)
    }
}
impl<'a> Iterator for Iterable<'a, Iterable<'a, McastGroupAttrs<'a>>> {
    type Item = Result<Iterable<'a, McastGroupAttrs<'a>>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            {
                return Some(Ok(Iterable::with_loc(next, self.orig_loc)));
            }
        }
        Some(Err(self.error_context(
            "McastGroupAttrs",
            None,
            self.buf.as_ptr().wrapping_add(self.pos),
        )))
    }
}
impl<'a> CtrlAttrs<'a> {
    pub fn new(buf: &'a [u8]) -> Iterable<'a, CtrlAttrs<'a>> {
        Iterable::new(buf)
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
impl<'a> Iterator for Iterable<'a, CtrlAttrs<'a>> {
    type Item = Result<CtrlAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
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
                    let res = Some(Iterable::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => CtrlAttrs::McastGroups({
                    let res = Some(Iterable::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => CtrlAttrs::Policy({
                    let res = Some(Iterable::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => CtrlAttrs::OpPolicy({
                    let res = Some(Iterable::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => CtrlAttrs::Op({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "CtrlAttrs",
            r#type.and_then(|t| CtrlAttrs::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, Iterable<'a, OpAttrs>> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_list()
            .entries(self.clone().map(FlattenErrorContext))
            .finish()
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, Iterable<'a, McastGroupAttrs<'a>>> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_list()
            .entries(self.clone().map(FlattenErrorContext))
            .finish()
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, CtrlAttrs<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("CtrlAttrs");
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
impl<'a> Iterable<'a, CtrlAttrs<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset {
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
#[doc = "Original name: \"mcast-group-attrs\""]
#[derive(Clone)]
pub enum McastGroupAttrs<'a> {
    Name(&'a CStr),
    Id(u32),
}
impl<'a> Iterable<'a, McastGroupAttrs<'a>> {
    pub fn get_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let McastGroupAttrs::Name(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("McastGroupAttrs", "Name"))
    }
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let McastGroupAttrs::Id(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("McastGroupAttrs", "Id"))
    }
}
impl<'a> McastGroupAttrs<'a> {
    pub fn new(buf: &'a [u8]) -> Iterable<'a, McastGroupAttrs<'a>> {
        Iterable::new(buf)
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
impl<'a> Iterator for Iterable<'a, McastGroupAttrs<'a>> {
    type Item = Result<McastGroupAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
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
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "McastGroupAttrs",
            r#type.and_then(|t| McastGroupAttrs::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, McastGroupAttrs<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("McastGroupAttrs");
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
                McastGroupAttrs::Name(val) => fmt.field("Name", &val),
                McastGroupAttrs::Id(val) => fmt.field("Id", &val),
            };
        }
        fmt.finish()
    }
}
impl<'a> Iterable<'a, McastGroupAttrs<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset {
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
#[doc = "Original name: \"op-attrs\""]
#[derive(Clone)]
pub enum OpAttrs {
    Id(u32),
    #[doc = "Associated type: \"OpFlags\" (1 bit per enumeration)"]
    Flags(u32),
}
impl<'a> Iterable<'a, OpAttrs> {
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpAttrs::Id(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpAttrs", "Id"))
    }
    #[doc = "Associated type: \"OpFlags\" (1 bit per enumeration)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpAttrs::Flags(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpAttrs", "Flags"))
    }
}
impl OpAttrs {
    pub fn new(buf: &'_ [u8]) -> Iterable<'_, OpAttrs> {
        Iterable::new(buf)
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
impl Iterator for Iterable<'_, OpAttrs> {
    type Item = Result<OpAttrs, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
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
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpAttrs",
            r#type.and_then(|t| OpAttrs::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl std::fmt::Debug for Iterable<'_, OpAttrs> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpAttrs");
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
                OpAttrs::Id(val) => fmt.field("Id", &val),
                OpAttrs::Flags(val) => fmt.field("Flags", &val),
            };
        }
        fmt.finish()
    }
}
impl Iterable<'_, OpAttrs> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset {
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
#[doc = "Original name: \"policy-attrs\""]
#[derive(Clone)]
pub enum PolicyAttrs<'a> {
    #[doc = "Associated type: \"AttrType\" (enum)"]
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
impl<'a> Iterable<'a, PolicyAttrs<'a>> {
    #[doc = "Associated type: \"AttrType\" (enum)"]
    pub fn get_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PolicyAttrs::Type(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("PolicyAttrs", "Type"))
    }
    pub fn get_min_value_s(&self) -> Result<i64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PolicyAttrs::MinValueS(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("PolicyAttrs", "MinValueS"))
    }
    pub fn get_max_value_s(&self) -> Result<i64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PolicyAttrs::MaxValueS(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("PolicyAttrs", "MaxValueS"))
    }
    pub fn get_min_value_u(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PolicyAttrs::MinValueU(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("PolicyAttrs", "MinValueU"))
    }
    pub fn get_max_value_u(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PolicyAttrs::MaxValueU(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("PolicyAttrs", "MaxValueU"))
    }
    pub fn get_min_length(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PolicyAttrs::MinLength(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("PolicyAttrs", "MinLength"))
    }
    pub fn get_max_length(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PolicyAttrs::MaxLength(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("PolicyAttrs", "MaxLength"))
    }
    pub fn get_policy_idx(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PolicyAttrs::PolicyIdx(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("PolicyAttrs", "PolicyIdx"))
    }
    pub fn get_policy_maxtype(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PolicyAttrs::PolicyMaxtype(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("PolicyAttrs", "PolicyMaxtype"))
    }
    pub fn get_bitfield32_mask(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PolicyAttrs::Bitfield32Mask(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("PolicyAttrs", "Bitfield32Mask"))
    }
    pub fn get_mask(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PolicyAttrs::Mask(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("PolicyAttrs", "Mask"))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PolicyAttrs::Pad(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("PolicyAttrs", "Pad"))
    }
}
impl<'a> PolicyAttrs<'a> {
    pub fn new(buf: &'a [u8]) -> Iterable<'a, PolicyAttrs<'a>> {
        Iterable::new(buf)
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
impl<'a> Iterator for Iterable<'a, PolicyAttrs<'a>> {
    type Item = Result<PolicyAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
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
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "PolicyAttrs",
            r#type.and_then(|t| PolicyAttrs::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, PolicyAttrs<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("PolicyAttrs");
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
                PolicyAttrs::Type(val) => fmt.field("Type", &val),
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
impl<'a> Iterable<'a, PolicyAttrs<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset {
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
#[doc = "Original name: \"op-policy-attrs\""]
#[derive(Clone)]
pub enum OpPolicyAttrs {
    Do(u32),
    Dump(u32),
}
impl<'a> Iterable<'a, OpPolicyAttrs> {
    pub fn get_do(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPolicyAttrs::Do(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpPolicyAttrs", "Do"))
    }
    pub fn get_dump(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPolicyAttrs::Dump(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpPolicyAttrs", "Dump"))
    }
}
impl OpPolicyAttrs {
    pub fn new(buf: &'_ [u8]) -> Iterable<'_, OpPolicyAttrs> {
        Iterable::new(buf)
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
impl Iterator for Iterable<'_, OpPolicyAttrs> {
    type Item = Result<OpPolicyAttrs, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
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
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpPolicyAttrs",
            r#type.and_then(|t| OpPolicyAttrs::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl std::fmt::Debug for Iterable<'_, OpPolicyAttrs> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpPolicyAttrs");
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
                OpPolicyAttrs::Do(val) => fmt.field("Do", &val),
                OpPolicyAttrs::Dump(val) => fmt.field("Dump", &val),
            };
        }
        fmt.finish()
    }
}
impl Iterable<'_, OpPolicyAttrs> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset {
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
pub struct PushCtrlAttrs<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushCtrlAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
pub struct PushArrayOpAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
    pub(crate) counter: u16,
}
impl<Prev: Rec> Rec for PushArrayOpAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushArrayOpAttrs<Prev> {
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
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn entry_nested(mut self) -> PushOpAttrs<Self> {
        let index = self.counter;
        self.counter += 1;
        let header_offset = push_nested_header(self.as_rec_mut(), index);
        PushOpAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushArrayOpAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushArrayMcastGroupAttrs<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
    pub(crate) counter: u16,
}
impl<Prev: Rec> Rec for PushArrayMcastGroupAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushArrayMcastGroupAttrs<Prev> {
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
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn entry_nested(mut self) -> PushMcastGroupAttrs<Self> {
        let index = self.counter;
        self.counter += 1;
        let header_offset = push_nested_header(self.as_rec_mut(), index);
        PushMcastGroupAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushArrayMcastGroupAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
impl<Prev: Rec> PushCtrlAttrs<Prev> {
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
    pub fn push_family_id(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 1u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_family_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_family_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_version(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_hdrsize(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_maxattr(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 5u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn array_ops(mut self) -> PushArrayOpAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 6u16);
        PushArrayOpAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
            counter: 0,
        }
    }
    pub fn array_mcast_groups(mut self) -> PushArrayMcastGroupAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 7u16);
        PushArrayMcastGroupAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
            counter: 0,
        }
    }
    pub fn nested_policy(mut self) -> PushPolicyAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 8u16);
        PushPolicyAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_op_policy(mut self) -> PushOpPolicyAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 9u16);
        PushOpPolicyAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_op(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 10u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushCtrlAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushMcastGroupAttrs<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushMcastGroupAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushMcastGroupAttrs<Prev> {
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
    pub fn push_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushMcastGroupAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushOpAttrs<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpAttrs<Prev> {
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
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: \"OpFlags\" (1 bit per enumeration)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPolicyAttrs<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushPolicyAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushPolicyAttrs<Prev> {
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
    #[doc = "Associated type: \"AttrType\" (enum)"]
    pub fn push_type(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_min_value_s(mut self, value: i64) -> Self {
        push_header(self.as_rec_mut(), 2u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_value_s(mut self, value: i64) -> Self {
        push_header(self.as_rec_mut(), 3u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_min_value_u(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 4u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_value_u(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 5u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_min_length(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 6u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_max_length(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 7u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_policy_idx(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_policy_maxtype(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 9u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_bitfield32_mask(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 10u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mask(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 11u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 12u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
}
impl<Prev: Rec> Drop for PushPolicyAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushOpPolicyAttrs<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpPolicyAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpPolicyAttrs<Prev> {
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
    pub fn push_do(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_dump(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpPolicyAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get / dump genetlink families"]
pub struct PushOpGetfamilyDumpRequest<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetfamilyDumpRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetfamilyDumpRequest<Prev> {
    pub fn new(mut prev: Prev) -> Self {
        Self::write_header(&mut prev);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev) {
        let mut header = PushBuiltinNfgenmsg::new();
        header.set_cmd(3u8);
        header.set_version(1u8);
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
impl<Prev: Rec> Drop for PushOpGetfamilyDumpRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get / dump genetlink families"]
#[doc = "Original name: \"op-getfamily-dump-request\""]
#[derive(Clone)]
pub enum OpGetfamilyDumpRequest {}
impl<'a> Iterable<'a, OpGetfamilyDumpRequest> {}
impl OpGetfamilyDumpRequest {
    pub fn new(buf: &'_ [u8]) -> Iterable<'_, OpGetfamilyDumpRequest> {
        let mut header = PushBuiltinNfgenmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushBuiltinNfgenmsg::len()]);
        Iterable::with_loc(&buf[PushBuiltinNfgenmsg::len()..], buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        CtrlAttrs::attr_from_type(r#type)
    }
}
impl Iterator for Iterable<'_, OpGetfamilyDumpRequest> {
    type Item = Result<OpGetfamilyDumpRequest, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpGetfamilyDumpRequest",
            r#type.and_then(|t| OpGetfamilyDumpRequest::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl std::fmt::Debug for Iterable<'_, OpGetfamilyDumpRequest> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetfamilyDumpRequest");
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
impl Iterable<'_, OpGetfamilyDumpRequest> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpGetfamilyDumpRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetfamilyDumpRequest::attr_from_type(t)),
            );
        }
        (stack, None)
    }
}
#[doc = "Get / dump genetlink families"]
pub struct PushOpGetfamilyDumpReply<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetfamilyDumpReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetfamilyDumpReply<Prev> {
    pub fn new(mut prev: Prev) -> Self {
        Self::write_header(&mut prev);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev) {
        let mut header = PushBuiltinNfgenmsg::new();
        header.set_cmd(1u8);
        header.set_version(1u8);
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_family_id(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 1u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_family_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_family_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_version(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_hdrsize(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_maxattr(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 5u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn array_ops(mut self) -> PushArrayOpAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 6u16);
        PushArrayOpAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
            counter: 0,
        }
    }
    pub fn array_mcast_groups(mut self) -> PushArrayMcastGroupAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 7u16);
        PushArrayMcastGroupAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
            counter: 0,
        }
    }
}
impl<Prev: Rec> Drop for PushOpGetfamilyDumpReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get / dump genetlink families"]
#[doc = "Original name: \"op-getfamily-dump-reply\""]
#[derive(Clone)]
pub enum OpGetfamilyDumpReply<'a> {
    FamilyId(u16),
    FamilyName(&'a CStr),
    Version(u32),
    Hdrsize(u32),
    Maxattr(u32),
    Ops(Iterable<'a, Iterable<'a, OpAttrs>>),
    McastGroups(Iterable<'a, Iterable<'a, McastGroupAttrs<'a>>>),
}
impl<'a> Iterable<'a, OpGetfamilyDumpReply<'a>> {
    pub fn get_family_id(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetfamilyDumpReply::FamilyId(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetfamilyDumpReply", "FamilyId"))
    }
    pub fn get_family_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetfamilyDumpReply::FamilyName(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetfamilyDumpReply", "FamilyName"))
    }
    pub fn get_version(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetfamilyDumpReply::Version(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetfamilyDumpReply", "Version"))
    }
    pub fn get_hdrsize(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetfamilyDumpReply::Hdrsize(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetfamilyDumpReply", "Hdrsize"))
    }
    pub fn get_maxattr(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetfamilyDumpReply::Maxattr(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetfamilyDumpReply", "Maxattr"))
    }
    pub fn get_ops(
        &self,
    ) -> Result<
        ArrayIterable<Iterable<'a, Iterable<'a, OpAttrs>>, Iterable<'a, OpAttrs>>,
        ErrorContext,
    > {
        for attr in self.clone() {
            if let OpGetfamilyDumpReply::Ops(val) = attr? {
                return Ok(ArrayIterable::new(val));
            }
        }
        Err(self.error_missing("OpGetfamilyDumpReply", "Ops"))
    }
    pub fn get_mcast_groups(
        &self,
    ) -> Result<
        ArrayIterable<
            Iterable<'a, Iterable<'a, McastGroupAttrs<'a>>>,
            Iterable<'a, McastGroupAttrs<'a>>,
        >,
        ErrorContext,
    > {
        for attr in self.clone() {
            if let OpGetfamilyDumpReply::McastGroups(val) = attr? {
                return Ok(ArrayIterable::new(val));
            }
        }
        Err(self.error_missing("OpGetfamilyDumpReply", "McastGroups"))
    }
}
impl<'a> OpGetfamilyDumpReply<'a> {
    pub fn new(buf: &'a [u8]) -> Iterable<'a, OpGetfamilyDumpReply<'a>> {
        let mut header = PushBuiltinNfgenmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushBuiltinNfgenmsg::len()]);
        Iterable::with_loc(&buf[PushBuiltinNfgenmsg::len()..], buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        CtrlAttrs::attr_from_type(r#type)
    }
}
impl<'a> Iterator for Iterable<'a, OpGetfamilyDumpReply<'a>> {
    type Item = Result<OpGetfamilyDumpReply<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpGetfamilyDumpReply::FamilyId({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpGetfamilyDumpReply::FamilyName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpGetfamilyDumpReply::Version({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpGetfamilyDumpReply::Hdrsize({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpGetfamilyDumpReply::Maxattr({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpGetfamilyDumpReply::Ops({
                    let res = Some(Iterable::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => OpGetfamilyDumpReply::McastGroups({
                    let res = Some(Iterable::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpGetfamilyDumpReply",
            r#type.and_then(|t| OpGetfamilyDumpReply::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpGetfamilyDumpReply<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetfamilyDumpReply");
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
                OpGetfamilyDumpReply::FamilyId(val) => fmt.field("FamilyId", &val),
                OpGetfamilyDumpReply::FamilyName(val) => fmt.field("FamilyName", &val),
                OpGetfamilyDumpReply::Version(val) => fmt.field("Version", &val),
                OpGetfamilyDumpReply::Hdrsize(val) => fmt.field("Hdrsize", &val),
                OpGetfamilyDumpReply::Maxattr(val) => fmt.field("Maxattr", &val),
                OpGetfamilyDumpReply::Ops(val) => fmt.field("Ops", &val),
                OpGetfamilyDumpReply::McastGroups(val) => fmt.field("McastGroups", &val),
            };
        }
        fmt.finish()
    }
}
impl<'a> Iterable<'a, OpGetfamilyDumpReply<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpGetfamilyDumpReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetfamilyDumpReply::attr_from_type(t)),
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
                OpGetfamilyDumpReply::FamilyId(val) => {
                    if last_off == offset {
                        stack.push(("FamilyId", last_off));
                        break;
                    }
                }
                OpGetfamilyDumpReply::FamilyName(val) => {
                    if last_off == offset {
                        stack.push(("FamilyName", last_off));
                        break;
                    }
                }
                OpGetfamilyDumpReply::Version(val) => {
                    if last_off == offset {
                        stack.push(("Version", last_off));
                        break;
                    }
                }
                OpGetfamilyDumpReply::Hdrsize(val) => {
                    if last_off == offset {
                        stack.push(("Hdrsize", last_off));
                        break;
                    }
                }
                OpGetfamilyDumpReply::Maxattr(val) => {
                    if last_off == offset {
                        stack.push(("Maxattr", last_off));
                        break;
                    }
                }
                OpGetfamilyDumpReply::Ops(val) => {
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
                OpGetfamilyDumpReply::McastGroups(val) => {
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
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetfamilyDumpReply", cur));
        }
        (stack, missing)
    }
}
#[derive(Debug)]
pub struct RequestOpGetfamilyDumpRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpGetfamilyDumpRequest<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        PushOpGetfamilyDumpRequest::write_header(&mut request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode(&mut self) -> PushOpGetfamilyDumpRequest<&mut Vec<u8>> {
        PushOpGetfamilyDumpRequest::new_without_header(self.request.buf_mut())
    }
}
impl NetlinkRequest for RequestOpGetfamilyDumpRequest<'_> {
    type ReplyType<'buf> = Iterable<'buf, OpGetfamilyDumpReply<'buf>>;
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
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpGetfamilyDumpReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpGetfamilyDumpRequest::new(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "Get / dump genetlink families"]
pub struct PushOpGetfamilyDoRequest<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetfamilyDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetfamilyDoRequest<Prev> {
    pub fn new(mut prev: Prev) -> Self {
        Self::write_header(&mut prev);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev) {
        let mut header = PushBuiltinNfgenmsg::new();
        header.set_cmd(3u8);
        header.set_version(1u8);
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_family_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_family_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
}
impl<Prev: Rec> Drop for PushOpGetfamilyDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get / dump genetlink families"]
#[doc = "Original name: \"op-getfamily-do-request\""]
#[derive(Clone)]
pub enum OpGetfamilyDoRequest<'a> {
    FamilyName(&'a CStr),
}
impl<'a> Iterable<'a, OpGetfamilyDoRequest<'a>> {
    pub fn get_family_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetfamilyDoRequest::FamilyName(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetfamilyDoRequest", "FamilyName"))
    }
}
impl<'a> OpGetfamilyDoRequest<'a> {
    pub fn new(buf: &'a [u8]) -> Iterable<'a, OpGetfamilyDoRequest<'a>> {
        let mut header = PushBuiltinNfgenmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushBuiltinNfgenmsg::len()]);
        Iterable::with_loc(&buf[PushBuiltinNfgenmsg::len()..], buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        CtrlAttrs::attr_from_type(r#type)
    }
}
impl<'a> Iterator for Iterable<'a, OpGetfamilyDoRequest<'a>> {
    type Item = Result<OpGetfamilyDoRequest<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                2u16 => OpGetfamilyDoRequest::FamilyName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpGetfamilyDoRequest",
            r#type.and_then(|t| OpGetfamilyDoRequest::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpGetfamilyDoRequest<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetfamilyDoRequest");
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
                OpGetfamilyDoRequest::FamilyName(val) => fmt.field("FamilyName", &val),
            };
        }
        fmt.finish()
    }
}
impl<'a> Iterable<'a, OpGetfamilyDoRequest<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpGetfamilyDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetfamilyDoRequest::attr_from_type(t)),
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
                OpGetfamilyDoRequest::FamilyName(val) => {
                    if last_off == offset {
                        stack.push(("FamilyName", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetfamilyDoRequest", cur));
        }
        (stack, None)
    }
}
#[doc = "Get / dump genetlink families"]
pub struct PushOpGetfamilyDoReply<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetfamilyDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetfamilyDoReply<Prev> {
    pub fn new(mut prev: Prev) -> Self {
        Self::write_header(&mut prev);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev) {
        let mut header = PushBuiltinNfgenmsg::new();
        header.set_cmd(1u8);
        header.set_version(1u8);
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_family_id(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 1u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_family_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_family_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_version(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_hdrsize(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_maxattr(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 5u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn array_ops(mut self) -> PushArrayOpAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 6u16);
        PushArrayOpAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
            counter: 0,
        }
    }
    pub fn array_mcast_groups(mut self) -> PushArrayMcastGroupAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 7u16);
        PushArrayMcastGroupAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
            counter: 0,
        }
    }
}
impl<Prev: Rec> Drop for PushOpGetfamilyDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get / dump genetlink families"]
#[doc = "Original name: \"op-getfamily-do-reply\""]
#[derive(Clone)]
pub enum OpGetfamilyDoReply<'a> {
    FamilyId(u16),
    FamilyName(&'a CStr),
    Version(u32),
    Hdrsize(u32),
    Maxattr(u32),
    Ops(Iterable<'a, Iterable<'a, OpAttrs>>),
    McastGroups(Iterable<'a, Iterable<'a, McastGroupAttrs<'a>>>),
}
impl<'a> Iterable<'a, OpGetfamilyDoReply<'a>> {
    pub fn get_family_id(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetfamilyDoReply::FamilyId(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetfamilyDoReply", "FamilyId"))
    }
    pub fn get_family_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetfamilyDoReply::FamilyName(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetfamilyDoReply", "FamilyName"))
    }
    pub fn get_version(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetfamilyDoReply::Version(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetfamilyDoReply", "Version"))
    }
    pub fn get_hdrsize(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetfamilyDoReply::Hdrsize(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetfamilyDoReply", "Hdrsize"))
    }
    pub fn get_maxattr(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetfamilyDoReply::Maxattr(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetfamilyDoReply", "Maxattr"))
    }
    pub fn get_ops(
        &self,
    ) -> Result<
        ArrayIterable<Iterable<'a, Iterable<'a, OpAttrs>>, Iterable<'a, OpAttrs>>,
        ErrorContext,
    > {
        for attr in self.clone() {
            if let OpGetfamilyDoReply::Ops(val) = attr? {
                return Ok(ArrayIterable::new(val));
            }
        }
        Err(self.error_missing("OpGetfamilyDoReply", "Ops"))
    }
    pub fn get_mcast_groups(
        &self,
    ) -> Result<
        ArrayIterable<
            Iterable<'a, Iterable<'a, McastGroupAttrs<'a>>>,
            Iterable<'a, McastGroupAttrs<'a>>,
        >,
        ErrorContext,
    > {
        for attr in self.clone() {
            if let OpGetfamilyDoReply::McastGroups(val) = attr? {
                return Ok(ArrayIterable::new(val));
            }
        }
        Err(self.error_missing("OpGetfamilyDoReply", "McastGroups"))
    }
}
impl<'a> OpGetfamilyDoReply<'a> {
    pub fn new(buf: &'a [u8]) -> Iterable<'a, OpGetfamilyDoReply<'a>> {
        let mut header = PushBuiltinNfgenmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushBuiltinNfgenmsg::len()]);
        Iterable::with_loc(&buf[PushBuiltinNfgenmsg::len()..], buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        CtrlAttrs::attr_from_type(r#type)
    }
}
impl<'a> Iterator for Iterable<'a, OpGetfamilyDoReply<'a>> {
    type Item = Result<OpGetfamilyDoReply<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpGetfamilyDoReply::FamilyId({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpGetfamilyDoReply::FamilyName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpGetfamilyDoReply::Version({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpGetfamilyDoReply::Hdrsize({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpGetfamilyDoReply::Maxattr({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpGetfamilyDoReply::Ops({
                    let res = Some(Iterable::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => OpGetfamilyDoReply::McastGroups({
                    let res = Some(Iterable::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpGetfamilyDoReply",
            r#type.and_then(|t| OpGetfamilyDoReply::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpGetfamilyDoReply<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetfamilyDoReply");
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
                OpGetfamilyDoReply::FamilyId(val) => fmt.field("FamilyId", &val),
                OpGetfamilyDoReply::FamilyName(val) => fmt.field("FamilyName", &val),
                OpGetfamilyDoReply::Version(val) => fmt.field("Version", &val),
                OpGetfamilyDoReply::Hdrsize(val) => fmt.field("Hdrsize", &val),
                OpGetfamilyDoReply::Maxattr(val) => fmt.field("Maxattr", &val),
                OpGetfamilyDoReply::Ops(val) => fmt.field("Ops", &val),
                OpGetfamilyDoReply::McastGroups(val) => fmt.field("McastGroups", &val),
            };
        }
        fmt.finish()
    }
}
impl<'a> Iterable<'a, OpGetfamilyDoReply<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpGetfamilyDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetfamilyDoReply::attr_from_type(t)),
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
                OpGetfamilyDoReply::FamilyId(val) => {
                    if last_off == offset {
                        stack.push(("FamilyId", last_off));
                        break;
                    }
                }
                OpGetfamilyDoReply::FamilyName(val) => {
                    if last_off == offset {
                        stack.push(("FamilyName", last_off));
                        break;
                    }
                }
                OpGetfamilyDoReply::Version(val) => {
                    if last_off == offset {
                        stack.push(("Version", last_off));
                        break;
                    }
                }
                OpGetfamilyDoReply::Hdrsize(val) => {
                    if last_off == offset {
                        stack.push(("Hdrsize", last_off));
                        break;
                    }
                }
                OpGetfamilyDoReply::Maxattr(val) => {
                    if last_off == offset {
                        stack.push(("Maxattr", last_off));
                        break;
                    }
                }
                OpGetfamilyDoReply::Ops(val) => {
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
                OpGetfamilyDoReply::McastGroups(val) => {
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
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetfamilyDoReply", cur));
        }
        (stack, missing)
    }
}
#[derive(Debug)]
pub struct RequestOpGetfamilyDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpGetfamilyDoRequest<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        PushOpGetfamilyDoRequest::write_header(&mut request.buf_mut());
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpGetfamilyDoRequest<&mut Vec<u8>> {
        PushOpGetfamilyDoRequest::new_without_header(self.request.buf_mut())
    }
}
impl NetlinkRequest for RequestOpGetfamilyDoRequest<'_> {
    type ReplyType<'buf> = Iterable<'buf, OpGetfamilyDoReply<'buf>>;
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
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpGetfamilyDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpGetfamilyDoRequest::new(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "Get / dump genetlink policies"]
pub struct PushOpGetpolicyDumpRequest<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetpolicyDumpRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetpolicyDumpRequest<Prev> {
    pub fn new(mut prev: Prev) -> Self {
        Self::write_header(&mut prev);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev) {
        let mut header = PushBuiltinNfgenmsg::new();
        header.set_cmd(10u8);
        header.set_version(1u8);
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_family_id(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 1u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_family_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_family_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_op(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 10u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpGetpolicyDumpRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get / dump genetlink policies"]
#[doc = "Original name: \"op-getpolicy-dump-request\""]
#[derive(Clone)]
pub enum OpGetpolicyDumpRequest<'a> {
    FamilyId(u16),
    FamilyName(&'a CStr),
    Op(u32),
}
impl<'a> Iterable<'a, OpGetpolicyDumpRequest<'a>> {
    pub fn get_family_id(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetpolicyDumpRequest::FamilyId(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetpolicyDumpRequest", "FamilyId"))
    }
    pub fn get_family_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetpolicyDumpRequest::FamilyName(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetpolicyDumpRequest", "FamilyName"))
    }
    pub fn get_op(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetpolicyDumpRequest::Op(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetpolicyDumpRequest", "Op"))
    }
}
impl<'a> OpGetpolicyDumpRequest<'a> {
    pub fn new(buf: &'a [u8]) -> Iterable<'a, OpGetpolicyDumpRequest<'a>> {
        let mut header = PushBuiltinNfgenmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushBuiltinNfgenmsg::len()]);
        Iterable::with_loc(&buf[PushBuiltinNfgenmsg::len()..], buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        CtrlAttrs::attr_from_type(r#type)
    }
}
impl<'a> Iterator for Iterable<'a, OpGetpolicyDumpRequest<'a>> {
    type Item = Result<OpGetpolicyDumpRequest<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpGetpolicyDumpRequest::FamilyId({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpGetpolicyDumpRequest::FamilyName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => OpGetpolicyDumpRequest::Op({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpGetpolicyDumpRequest",
            r#type.and_then(|t| OpGetpolicyDumpRequest::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpGetpolicyDumpRequest<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetpolicyDumpRequest");
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
                OpGetpolicyDumpRequest::FamilyId(val) => fmt.field("FamilyId", &val),
                OpGetpolicyDumpRequest::FamilyName(val) => fmt.field("FamilyName", &val),
                OpGetpolicyDumpRequest::Op(val) => fmt.field("Op", &val),
            };
        }
        fmt.finish()
    }
}
impl<'a> Iterable<'a, OpGetpolicyDumpRequest<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpGetpolicyDumpRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetpolicyDumpRequest::attr_from_type(t)),
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
                OpGetpolicyDumpRequest::FamilyId(val) => {
                    if last_off == offset {
                        stack.push(("FamilyId", last_off));
                        break;
                    }
                }
                OpGetpolicyDumpRequest::FamilyName(val) => {
                    if last_off == offset {
                        stack.push(("FamilyName", last_off));
                        break;
                    }
                }
                OpGetpolicyDumpRequest::Op(val) => {
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
            stack.push(("OpGetpolicyDumpRequest", cur));
        }
        (stack, None)
    }
}
#[doc = "Get / dump genetlink policies"]
pub struct PushOpGetpolicyDumpReply<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetpolicyDumpReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetpolicyDumpReply<Prev> {
    pub fn new(mut prev: Prev) -> Self {
        Self::write_header(&mut prev);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev) {
        let mut header = PushBuiltinNfgenmsg::new();
        header.set_cmd(10u8);
        header.set_version(1u8);
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_family_id(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 1u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_policy(mut self) -> PushPolicyAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 8u16);
        PushPolicyAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn nested_op_policy(mut self) -> PushOpPolicyAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 9u16);
        PushOpPolicyAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushOpGetpolicyDumpReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get / dump genetlink policies"]
#[doc = "Original name: \"op-getpolicy-dump-reply\""]
#[derive(Clone)]
pub enum OpGetpolicyDumpReply<'a> {
    FamilyId(u16),
    Policy(Iterable<'a, PolicyAttrs<'a>>),
    OpPolicy(Iterable<'a, OpPolicyAttrs>),
}
impl<'a> Iterable<'a, OpGetpolicyDumpReply<'a>> {
    pub fn get_family_id(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetpolicyDumpReply::FamilyId(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetpolicyDumpReply", "FamilyId"))
    }
    pub fn get_policy(&self) -> Result<Iterable<'a, PolicyAttrs<'a>>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetpolicyDumpReply::Policy(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetpolicyDumpReply", "Policy"))
    }
    pub fn get_op_policy(&self) -> Result<Iterable<'a, OpPolicyAttrs>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetpolicyDumpReply::OpPolicy(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetpolicyDumpReply", "OpPolicy"))
    }
}
impl<'a> OpGetpolicyDumpReply<'a> {
    pub fn new(buf: &'a [u8]) -> Iterable<'a, OpGetpolicyDumpReply<'a>> {
        let mut header = PushBuiltinNfgenmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushBuiltinNfgenmsg::len()]);
        Iterable::with_loc(&buf[PushBuiltinNfgenmsg::len()..], buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        CtrlAttrs::attr_from_type(r#type)
    }
}
impl<'a> Iterator for Iterable<'a, OpGetpolicyDumpReply<'a>> {
    type Item = Result<OpGetpolicyDumpReply<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpGetpolicyDumpReply::FamilyId({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpGetpolicyDumpReply::Policy({
                    let res = Some(Iterable::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => OpGetpolicyDumpReply::OpPolicy({
                    let res = Some(Iterable::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                0 => break,
                n => break,
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpGetpolicyDumpReply",
            r#type.and_then(|t| OpGetpolicyDumpReply::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpGetpolicyDumpReply<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetpolicyDumpReply");
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
                OpGetpolicyDumpReply::FamilyId(val) => fmt.field("FamilyId", &val),
                OpGetpolicyDumpReply::Policy(val) => fmt.field("Policy", &val),
                OpGetpolicyDumpReply::OpPolicy(val) => fmt.field("OpPolicy", &val),
            };
        }
        fmt.finish()
    }
}
impl<'a> Iterable<'a, OpGetpolicyDumpReply<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpGetpolicyDumpReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetpolicyDumpReply::attr_from_type(t)),
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
                OpGetpolicyDumpReply::FamilyId(val) => {
                    if last_off == offset {
                        stack.push(("FamilyId", last_off));
                        break;
                    }
                }
                OpGetpolicyDumpReply::Policy(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetpolicyDumpReply::OpPolicy(val) => {
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
            stack.push(("OpGetpolicyDumpReply", cur));
        }
        (stack, missing)
    }
}
#[derive(Debug)]
pub struct RequestOpGetpolicyDumpRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpGetpolicyDumpRequest<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        PushOpGetpolicyDumpRequest::write_header(&mut request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode(&mut self) -> PushOpGetpolicyDumpRequest<&mut Vec<u8>> {
        PushOpGetpolicyDumpRequest::new_without_header(self.request.buf_mut())
    }
}
impl NetlinkRequest for RequestOpGetpolicyDumpRequest<'_> {
    type ReplyType<'buf> = Iterable<'buf, OpGetpolicyDumpReply<'buf>>;
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
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpGetpolicyDumpReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpGetpolicyDumpRequest::new(buf).lookup_attr(offset, missing_type)
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
    pub fn op_getfamily_dump_request(self) -> RequestOpGetfamilyDumpRequest<'buf> {
        RequestOpGetfamilyDumpRequest::new(self)
    }
    pub fn op_getfamily_do_request(self) -> RequestOpGetfamilyDoRequest<'buf> {
        RequestOpGetfamilyDoRequest::new(self)
    }
    pub fn op_getpolicy_dump_request(self) -> RequestOpGetpolicyDumpRequest<'buf> {
        RequestOpGetpolicyDumpRequest::new(self)
    }
}
