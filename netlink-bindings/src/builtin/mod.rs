#![doc = "Auxilary types not porovided by any particular family\n"]
#![allow(clippy::all)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]
#![allow(unreachable_code)]
#![allow(unreachable_patterns)]
#[cfg(test)]
mod tests;
use crate::consts;
use crate::utils::*;
use crate::{NetlinkRequest, Protocol};
pub const PROTONAME: &CStr = c"builtin";
#[doc = "Generic family header"]
#[doc = "Wrapper for bitfield32 type"]
#[doc = "Header of a Netlink message"]
#[doc = "Original name: \"nlmsgerr-attrs\""]
#[derive(Clone)]
pub enum NlmsgerrAttrs<'a> {
    #[doc = "error message string (string)"]
    Msg(&'a CStr),
    #[doc = "offset of the invalid attribute in the original message, counting from the beginning of the header (u32)"]
    Offset(u32),
    #[doc = "arbitrary subsystem specific cookie to be used - in the success case - to identify a created object or operation or similar (binary)"]
    Cookie(&'a [u8]),
    #[doc = "policy for a rejected attribute"]
    Policy(Iterable<'a, PolicyTypeAttrs<'a>>),
    #[doc = "type of a missing required attribute, NLMSGERR_ATTR_MISS_NEST will not be present if the attribute was missing at the message level"]
    MissingType(u16),
    #[doc = "offset of the nest where attribute was missing"]
    MissingNest(u32),
}
impl<'a> Iterable<'a, NlmsgerrAttrs<'a>> {
    #[doc = "error message string (string)"]
    pub fn get_msg(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NlmsgerrAttrs::Msg(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NlmsgerrAttrs", "Msg"))
    }
    #[doc = "offset of the invalid attribute in the original message, counting from the beginning of the header (u32)"]
    pub fn get_offset(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NlmsgerrAttrs::Offset(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NlmsgerrAttrs", "Offset"))
    }
    #[doc = "arbitrary subsystem specific cookie to be used - in the success case - to identify a created object or operation or similar (binary)"]
    pub fn get_cookie(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NlmsgerrAttrs::Cookie(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NlmsgerrAttrs", "Cookie"))
    }
    #[doc = "policy for a rejected attribute"]
    pub fn get_policy(&self) -> Result<Iterable<'a, PolicyTypeAttrs<'a>>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NlmsgerrAttrs::Policy(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NlmsgerrAttrs", "Policy"))
    }
    #[doc = "type of a missing required attribute, NLMSGERR_ATTR_MISS_NEST will not be present if the attribute was missing at the message level"]
    pub fn get_missing_type(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NlmsgerrAttrs::MissingType(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NlmsgerrAttrs", "MissingType"))
    }
    #[doc = "offset of the nest where attribute was missing"]
    pub fn get_missing_nest(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NlmsgerrAttrs::MissingNest(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NlmsgerrAttrs", "MissingNest"))
    }
}
impl<'a> NlmsgerrAttrs<'a> {
    pub fn new(buf: &'a [u8]) -> Iterable<'a, NlmsgerrAttrs<'a>> {
        Iterable::new(buf)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "Unused",
            1u16 => "Msg",
            2u16 => "Offset",
            3u16 => "Cookie",
            4u16 => "Policy",
            5u16 => "MissingType",
            6u16 => "MissingNest",
            _ => return None,
        };
        Some(res)
    }
}
impl<'a> Iterator for Iterable<'a, NlmsgerrAttrs<'a>> {
    type Item = Result<NlmsgerrAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => NlmsgerrAttrs::Msg({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => NlmsgerrAttrs::Offset({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => NlmsgerrAttrs::Cookie({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => NlmsgerrAttrs::Policy({
                    let res = Some(Iterable::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => NlmsgerrAttrs::MissingType({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => NlmsgerrAttrs::MissingNest({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(test) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "NlmsgerrAttrs",
            r#type.and_then(|t| NlmsgerrAttrs::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, NlmsgerrAttrs<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("NlmsgerrAttrs");
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
                NlmsgerrAttrs::Msg(val) => fmt.field("Msg", &val),
                NlmsgerrAttrs::Offset(val) => fmt.field("Offset", &val),
                NlmsgerrAttrs::Cookie(val) => fmt.field("Cookie", &val),
                NlmsgerrAttrs::Policy(val) => fmt.field("Policy", &val),
                NlmsgerrAttrs::MissingType(val) => fmt.field("MissingType", &val),
                NlmsgerrAttrs::MissingNest(val) => fmt.field("MissingNest", &val),
            };
        }
        fmt.finish()
    }
}
impl<'a> Iterable<'a, NlmsgerrAttrs<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("NlmsgerrAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| NlmsgerrAttrs::attr_from_type(t)),
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
                NlmsgerrAttrs::Msg(val) => {
                    if last_off == offset {
                        stack.push(("Msg", last_off));
                        break;
                    }
                }
                NlmsgerrAttrs::Offset(val) => {
                    if last_off == offset {
                        stack.push(("Offset", last_off));
                        break;
                    }
                }
                NlmsgerrAttrs::Cookie(val) => {
                    if last_off == offset {
                        stack.push(("Cookie", last_off));
                        break;
                    }
                }
                NlmsgerrAttrs::Policy(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                NlmsgerrAttrs::MissingType(val) => {
                    if last_off == offset {
                        stack.push(("MissingType", last_off));
                        break;
                    }
                }
                NlmsgerrAttrs::MissingNest(val) => {
                    if last_off == offset {
                        stack.push(("MissingNest", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("NlmsgerrAttrs", cur));
        }
        (stack, missing)
    }
}
#[doc = "Original name: \"policy-type-attrs\""]
#[derive(Clone)]
pub enum PolicyTypeAttrs<'a> {
    #[doc = "type of the attribute, enum netlink_attribute_type (U32)"]
    Type(u32),
    #[doc = "minimum value for signed integers (S64)"]
    MinValueSigned(i64),
    #[doc = "maximum value for signed integers (S64)"]
    MaxValueSigned(i64),
    #[doc = "minimum value for unsigned integers (U64)"]
    MinValueU(u64),
    #[doc = "maximum value for unsigned integers (U64)"]
    MaxValueU(u64),
    #[doc = "minimum length for binary attributes, no minimum if not given (U32)"]
    MinLength(u32),
    #[doc = "maximum length for binary attributes, no maximum if not given (U32)"]
    MaxLength(u32),
    #[doc = "sub policy for nested and nested array types (U32)"]
    PolicyIdx(u32),
    #[doc = "maximum sub policy attribute for nested and nested array types, this can in theory be < the size of the policy pointed to by the index, if limited inside the nesting (U32)"]
    PolicyMaxtype(u32),
    #[doc = "valid mask for the bitfield32 type (U32)"]
    Bitfield32Mask(u32),
    #[doc = "pad attribute for 64-bit alignment"]
    Pad(&'a [u8]),
    #[doc = "mask of valid bits for unsigned integers (U64)"]
    Mask(u64),
}
impl<'a> Iterable<'a, PolicyTypeAttrs<'a>> {
    #[doc = "type of the attribute, enum netlink_attribute_type (U32)"]
    pub fn get_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PolicyTypeAttrs::Type(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("PolicyTypeAttrs", "Type"))
    }
    #[doc = "minimum value for signed integers (S64)"]
    pub fn get_min_value_signed(&self) -> Result<i64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PolicyTypeAttrs::MinValueSigned(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("PolicyTypeAttrs", "MinValueSigned"))
    }
    #[doc = "maximum value for signed integers (S64)"]
    pub fn get_max_value_signed(&self) -> Result<i64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PolicyTypeAttrs::MaxValueSigned(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("PolicyTypeAttrs", "MaxValueSigned"))
    }
    #[doc = "minimum value for unsigned integers (U64)"]
    pub fn get_min_value_u(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PolicyTypeAttrs::MinValueU(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("PolicyTypeAttrs", "MinValueU"))
    }
    #[doc = "maximum value for unsigned integers (U64)"]
    pub fn get_max_value_u(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PolicyTypeAttrs::MaxValueU(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("PolicyTypeAttrs", "MaxValueU"))
    }
    #[doc = "minimum length for binary attributes, no minimum if not given (U32)"]
    pub fn get_min_length(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PolicyTypeAttrs::MinLength(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("PolicyTypeAttrs", "MinLength"))
    }
    #[doc = "maximum length for binary attributes, no maximum if not given (U32)"]
    pub fn get_max_length(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PolicyTypeAttrs::MaxLength(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("PolicyTypeAttrs", "MaxLength"))
    }
    #[doc = "sub policy for nested and nested array types (U32)"]
    pub fn get_policy_idx(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PolicyTypeAttrs::PolicyIdx(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("PolicyTypeAttrs", "PolicyIdx"))
    }
    #[doc = "maximum sub policy attribute for nested and nested array types, this can in theory be < the size of the policy pointed to by the index, if limited inside the nesting (U32)"]
    pub fn get_policy_maxtype(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PolicyTypeAttrs::PolicyMaxtype(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("PolicyTypeAttrs", "PolicyMaxtype"))
    }
    #[doc = "valid mask for the bitfield32 type (U32)"]
    pub fn get_bitfield32_mask(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PolicyTypeAttrs::Bitfield32Mask(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("PolicyTypeAttrs", "Bitfield32Mask"))
    }
    #[doc = "pad attribute for 64-bit alignment"]
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PolicyTypeAttrs::Pad(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("PolicyTypeAttrs", "Pad"))
    }
    #[doc = "mask of valid bits for unsigned integers (U64)"]
    pub fn get_mask(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PolicyTypeAttrs::Mask(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("PolicyTypeAttrs", "Mask"))
    }
}
impl<'a> PolicyTypeAttrs<'a> {
    pub fn new(buf: &'a [u8]) -> Iterable<'a, PolicyTypeAttrs<'a>> {
        Iterable::new(buf)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "Unspec",
            1u16 => "Type",
            2u16 => "MinValueSigned",
            3u16 => "MaxValueSigned",
            4u16 => "MinValueU",
            5u16 => "MaxValueU",
            6u16 => "MinLength",
            7u16 => "MaxLength",
            8u16 => "PolicyIdx",
            9u16 => "PolicyMaxtype",
            10u16 => "Bitfield32Mask",
            11u16 => "Pad",
            12u16 => "Mask",
            _ => return None,
        };
        Some(res)
    }
}
impl<'a> Iterator for Iterable<'a, PolicyTypeAttrs<'a>> {
    type Item = Result<PolicyTypeAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => PolicyTypeAttrs::Type({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => PolicyTypeAttrs::MinValueSigned({
                    let res = parse_i64(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => PolicyTypeAttrs::MaxValueSigned({
                    let res = parse_i64(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => PolicyTypeAttrs::MinValueU({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => PolicyTypeAttrs::MaxValueU({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => PolicyTypeAttrs::MinLength({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => PolicyTypeAttrs::MaxLength({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => PolicyTypeAttrs::PolicyIdx({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => PolicyTypeAttrs::PolicyMaxtype({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => PolicyTypeAttrs::Bitfield32Mask({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => PolicyTypeAttrs::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => PolicyTypeAttrs::Mask({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(test) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "PolicyTypeAttrs",
            r#type.and_then(|t| PolicyTypeAttrs::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, PolicyTypeAttrs<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("PolicyTypeAttrs");
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
                PolicyTypeAttrs::Type(val) => fmt.field("Type", &val),
                PolicyTypeAttrs::MinValueSigned(val) => fmt.field("MinValueSigned", &val),
                PolicyTypeAttrs::MaxValueSigned(val) => fmt.field("MaxValueSigned", &val),
                PolicyTypeAttrs::MinValueU(val) => fmt.field("MinValueU", &val),
                PolicyTypeAttrs::MaxValueU(val) => fmt.field("MaxValueU", &val),
                PolicyTypeAttrs::MinLength(val) => fmt.field("MinLength", &val),
                PolicyTypeAttrs::MaxLength(val) => fmt.field("MaxLength", &val),
                PolicyTypeAttrs::PolicyIdx(val) => fmt.field("PolicyIdx", &val),
                PolicyTypeAttrs::PolicyMaxtype(val) => fmt.field("PolicyMaxtype", &val),
                PolicyTypeAttrs::Bitfield32Mask(val) => fmt.field("Bitfield32Mask", &val),
                PolicyTypeAttrs::Pad(val) => fmt.field("Pad", &val),
                PolicyTypeAttrs::Mask(val) => fmt.field("Mask", &val),
            };
        }
        fmt.finish()
    }
}
impl<'a> Iterable<'a, PolicyTypeAttrs<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("PolicyTypeAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| PolicyTypeAttrs::attr_from_type(t)),
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
                PolicyTypeAttrs::Type(val) => {
                    if last_off == offset {
                        stack.push(("Type", last_off));
                        break;
                    }
                }
                PolicyTypeAttrs::MinValueSigned(val) => {
                    if last_off == offset {
                        stack.push(("MinValueSigned", last_off));
                        break;
                    }
                }
                PolicyTypeAttrs::MaxValueSigned(val) => {
                    if last_off == offset {
                        stack.push(("MaxValueSigned", last_off));
                        break;
                    }
                }
                PolicyTypeAttrs::MinValueU(val) => {
                    if last_off == offset {
                        stack.push(("MinValueU", last_off));
                        break;
                    }
                }
                PolicyTypeAttrs::MaxValueU(val) => {
                    if last_off == offset {
                        stack.push(("MaxValueU", last_off));
                        break;
                    }
                }
                PolicyTypeAttrs::MinLength(val) => {
                    if last_off == offset {
                        stack.push(("MinLength", last_off));
                        break;
                    }
                }
                PolicyTypeAttrs::MaxLength(val) => {
                    if last_off == offset {
                        stack.push(("MaxLength", last_off));
                        break;
                    }
                }
                PolicyTypeAttrs::PolicyIdx(val) => {
                    if last_off == offset {
                        stack.push(("PolicyIdx", last_off));
                        break;
                    }
                }
                PolicyTypeAttrs::PolicyMaxtype(val) => {
                    if last_off == offset {
                        stack.push(("PolicyMaxtype", last_off));
                        break;
                    }
                }
                PolicyTypeAttrs::Bitfield32Mask(val) => {
                    if last_off == offset {
                        stack.push(("Bitfield32Mask", last_off));
                        break;
                    }
                }
                PolicyTypeAttrs::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                PolicyTypeAttrs::Mask(val) => {
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
            stack.push(("PolicyTypeAttrs", cur));
        }
        (stack, None)
    }
}
pub struct PushNlmsgerrAttrs<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushNlmsgerrAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushNlmsgerrAttrs<Prev> {
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
    #[doc = "error message string (string)"]
    pub fn push_msg(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "error message string (string)"]
    pub fn push_msg_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    #[doc = "offset of the invalid attribute in the original message, counting from the beginning of the header (u32)"]
    pub fn push_offset(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "arbitrary subsystem specific cookie to be used - in the success case - to identify a created object or operation or similar (binary)"]
    pub fn push_cookie(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 3u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    #[doc = "policy for a rejected attribute"]
    pub fn nested_policy(mut self) -> PushPolicyTypeAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 4u16);
        PushPolicyTypeAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "type of a missing required attribute, NLMSGERR_ATTR_MISS_NEST will not be present if the attribute was missing at the message level"]
    pub fn push_missing_type(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 5u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "offset of the nest where attribute was missing"]
    pub fn push_missing_nest(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 6u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushNlmsgerrAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPolicyTypeAttrs<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushPolicyTypeAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushPolicyTypeAttrs<Prev> {
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
    #[doc = "type of the attribute, enum netlink_attribute_type (U32)"]
    pub fn push_type(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "minimum value for signed integers (S64)"]
    pub fn push_min_value_signed(mut self, value: i64) -> Self {
        push_header(self.as_rec_mut(), 2u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "maximum value for signed integers (S64)"]
    pub fn push_max_value_signed(mut self, value: i64) -> Self {
        push_header(self.as_rec_mut(), 3u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "minimum value for unsigned integers (U64)"]
    pub fn push_min_value_u(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 4u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "maximum value for unsigned integers (U64)"]
    pub fn push_max_value_u(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 5u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "minimum length for binary attributes, no minimum if not given (U32)"]
    pub fn push_min_length(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 6u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "maximum length for binary attributes, no maximum if not given (U32)"]
    pub fn push_max_length(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 7u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "sub policy for nested and nested array types (U32)"]
    pub fn push_policy_idx(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "maximum sub policy attribute for nested and nested array types, this can in theory be < the size of the policy pointed to by the index, if limited inside the nesting (U32)"]
    pub fn push_policy_maxtype(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 9u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "valid mask for the bitfield32 type (U32)"]
    pub fn push_bitfield32_mask(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 10u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "pad attribute for 64-bit alignment"]
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 11u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    #[doc = "mask of valid bits for unsigned integers (U64)"]
    pub fn push_mask(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 12u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushPolicyTypeAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Original name: \"builtin-nfgenmsg\""]
#[derive(Clone)]
pub struct PushBuiltinNfgenmsg {
    buf: [u8; 4usize],
}
impl PushBuiltinNfgenmsg {
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
        4usize
    }
    pub fn cmd(&self) -> u8 {
        parse_u8(&self.buf[0usize..1usize]).unwrap()
    }
    pub fn set_cmd(&mut self, value: u8) {
        self.buf[0usize..1usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn version(&self) -> u8 {
        parse_u8(&self.buf[1usize..2usize]).unwrap()
    }
    pub fn set_version(&mut self, value: u8) {
        self.buf[1usize..2usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn reserved(&self) -> u16 {
        parse_u16(&self.buf[2usize..4usize]).unwrap()
    }
    pub fn set_reserved(&mut self, value: u16) {
        self.buf[2usize..4usize].copy_from_slice(&value.to_ne_bytes())
    }
}
impl std::fmt::Debug for PushBuiltinNfgenmsg {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("BuiltinNfgenmsg")
            .field("cmd", &self.cmd())
            .field("version", &self.version())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[doc = "Original name: \"builtin-bitfield32\""]
#[derive(Clone)]
pub struct PushBuiltinBitfield32 {
    buf: [u8; 8usize],
}
impl PushBuiltinBitfield32 {
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
    pub fn value(&self) -> u32 {
        parse_u32(&self.buf[0usize..4usize]).unwrap()
    }
    pub fn set_value(&mut self, value: u32) {
        self.buf[0usize..4usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn selector(&self) -> u32 {
        parse_u32(&self.buf[4usize..8usize]).unwrap()
    }
    pub fn set_selector(&mut self, value: u32) {
        self.buf[4usize..8usize].copy_from_slice(&value.to_ne_bytes())
    }
}
impl std::fmt::Debug for PushBuiltinBitfield32 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("BuiltinBitfield32")
            .field("value", &self.value())
            .field("selector", &self.selector())
            .finish()
    }
}
#[doc = "Original name: \"nlmsghdr\""]
#[derive(Clone)]
pub struct PushNlmsghdr {
    buf: [u8; 16usize],
}
impl PushNlmsghdr {
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
    pub fn get_len(&self) -> u32 {
        parse_u32(&self.buf[0usize..4usize]).unwrap()
    }
    pub fn set_len(&mut self, value: u32) {
        self.buf[0usize..4usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn get_type(&self) -> u16 {
        parse_u16(&self.buf[4usize..6usize]).unwrap()
    }
    pub fn set_type(&mut self, value: u16) {
        self.buf[4usize..6usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn flags(&self) -> u16 {
        parse_u16(&self.buf[6usize..8usize]).unwrap()
    }
    pub fn set_flags(&mut self, value: u16) {
        self.buf[6usize..8usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn seq(&self) -> u32 {
        parse_u32(&self.buf[8usize..12usize]).unwrap()
    }
    pub fn set_seq(&mut self, value: u32) {
        self.buf[8usize..12usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn pid(&self) -> u32 {
        parse_u32(&self.buf[12usize..16usize]).unwrap()
    }
    pub fn set_pid(&mut self, value: u32) {
        self.buf[12usize..16usize].copy_from_slice(&value.to_ne_bytes())
    }
}
impl std::fmt::Debug for PushNlmsghdr {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Nlmsghdr")
            .field("len", &self.get_len())
            .field("type", &self.get_type())
            .field("flags", &self.flags())
            .field("seq", &self.seq())
            .field("pid", &self.pid())
            .finish()
    }
}
