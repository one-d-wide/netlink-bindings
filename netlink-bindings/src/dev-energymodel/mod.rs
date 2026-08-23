#![doc = "Energy model netlink interface to notify its changes.\n"]
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
pub const PROTONAME: &str = "dev-energymodel";
pub const PROTONAME_CSTR: &CStr = c"dev-energymodel";
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum PerfStateFlags {
    #[doc = "The performance state is inefficient. There is in this perf-domain,\nanother performance state with a higher frequency but a lower or equal\npower cost.\n"]
    PerfStateInefficient = 1 << 0,
}
impl PerfStateFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::PerfStateInefficient,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum PerfDomainFlags {
    #[doc = "The power values are in micro-Watts or some other scale.\n"]
    PerfDomainMicrowatts = 1 << 0,
    #[doc = "Skip inefficient states when estimating energy consumption.\n"]
    PerfDomainSkipInefficiencies = 1 << 1,
    #[doc = "The power values are artificial and might be created by platform missing\nreal power information.\n"]
    PerfDomainArtificial = 1 << 2,
}
impl PerfDomainFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::PerfDomainMicrowatts,
            n if n == 1 << 1 => Self::PerfDomainSkipInefficiencies,
            n if n == 1 << 2 => Self::PerfDomainArtificial,
            _ => return None,
        })
    }
}
#[derive(Clone)]
pub enum PerfDomain<'a> {
    Pad(&'a [u8]),
    #[doc = "A unique ID number for each performance domain.\n"]
    PerfDomainId(u32),
    #[doc = "Bitmask of performance domain flags.\n\nAssociated type: [`PerfDomainFlags`] (enum)"]
    Flags(u64),
    #[doc = "CPUs that belong to this performance domain.\n\nAttribute may repeat multiple times (treat it as array)"]
    Cpus(u64),
}
impl<'a> IterablePerfDomain<'a> {
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PerfDomain::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PerfDomain",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "A unique ID number for each performance domain.\n"]
    pub fn get_perf_domain_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PerfDomain::PerfDomainId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PerfDomain",
            "PerfDomainId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Bitmask of performance domain flags.\n\nAssociated type: [`PerfDomainFlags`] (enum)"]
    pub fn get_flags(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PerfDomain::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PerfDomain",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "CPUs that belong to this performance domain.\n\nAttribute may repeat multiple times (treat it as array)"]
    pub fn get_cpus(&self) -> MultiAttrIterable<Self, PerfDomain<'a>, u64> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let PerfDomain::Cpus(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl PerfDomain<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePerfDomain<'a> {
        IterablePerfDomain::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Pad",
            2u16 => "PerfDomainId",
            3u16 => "Flags",
            4u16 => "Cpus",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePerfDomain<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePerfDomain<'a> {
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
impl<'a> Iterator for IterablePerfDomain<'a> {
    type Item = Result<PerfDomain<'a>, ErrorContext>;
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
                1u16 => PerfDomain::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => PerfDomain::PerfDomainId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => PerfDomain::Flags({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => PerfDomain::Cpus({
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
            "PerfDomain",
            r#type.and_then(|t| PerfDomain::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterablePerfDomain<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("PerfDomain");
        let mut iter = IterablePerfDomain::with_loc(&[], self.orig_loc);
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
                PerfDomain::Pad(val) => fmt.field("Pad", &val),
                PerfDomain::PerfDomainId(val) => fmt.field("PerfDomainId", &val),
                PerfDomain::Flags(val) => fmt.field(
                    "Flags",
                    &FormatFlags(val.into(), PerfDomainFlags::from_value),
                ),
                PerfDomain::Cpus(val) => fmt.field("Cpus", &val),
            };
        }
        fmt.finish()
    }
}
impl IterablePerfDomain<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("PerfDomain", offset));
            return (
                stack,
                missing_type.and_then(|t| PerfDomain::attr_from_type(t)),
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
                PerfDomain::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                PerfDomain::PerfDomainId(val) => {
                    if last_off == offset {
                        stack.push(("PerfDomainId", last_off));
                        break;
                    }
                }
                PerfDomain::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                PerfDomain::Cpus(val) => {
                    if last_off == offset {
                        stack.push(("Cpus", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("PerfDomain", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum PerfTable<'a> {
    #[doc = "A unique ID number for each performance domain.\n"]
    PerfDomainId(u32),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    PerfState(IterablePerfState<'a>),
}
impl<'a> IterablePerfTable<'a> {
    #[doc = "A unique ID number for each performance domain.\n"]
    pub fn get_perf_domain_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PerfTable::PerfDomainId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PerfTable",
            "PerfDomainId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_perf_state(&self) -> MultiAttrIterable<Self, PerfTable<'a>, IterablePerfState<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let PerfTable::PerfState(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl PerfTable<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePerfTable<'a> {
        IterablePerfTable::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "PerfDomainId",
            2u16 => "PerfState",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePerfTable<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePerfTable<'a> {
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
impl<'a> Iterator for IterablePerfTable<'a> {
    type Item = Result<PerfTable<'a>, ErrorContext>;
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
                1u16 => PerfTable::PerfDomainId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => PerfTable::PerfState({
                    let res = Some(IterablePerfState::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "PerfTable",
            r#type.and_then(|t| PerfTable::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterablePerfTable<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("PerfTable");
        let mut iter = IterablePerfTable::with_loc(&[], self.orig_loc);
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
                PerfTable::PerfDomainId(val) => fmt.field("PerfDomainId", &val),
                PerfTable::PerfState(val) => fmt.field("PerfState", &val),
            };
        }
        fmt.finish()
    }
}
impl IterablePerfTable<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("PerfTable", offset));
            return (
                stack,
                missing_type.and_then(|t| PerfTable::attr_from_type(t)),
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
                PerfTable::PerfDomainId(val) => {
                    if last_off == offset {
                        stack.push(("PerfDomainId", last_off));
                        break;
                    }
                }
                PerfTable::PerfState(val) => {
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
            stack.push(("PerfTable", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum PerfState<'a> {
    Pad(&'a [u8]),
    #[doc = "CPU performance (capacity) at a given frequency.\n"]
    Performance(u64),
    #[doc = "The frequency in KHz, for consistency with CPUFreq.\n"]
    Frequency(u64),
    #[doc = "The power consumed at this level (by 1 CPU or by a registered device).\nIt can be a total power: static and dynamic.\n"]
    Power(u64),
    #[doc = "The cost coefficient associated with this level, used during energy\ncalculation. Equal to: power \\* max_frequency / frequency.\n"]
    Cost(u64),
    #[doc = "Bitmask of performance state flags.\n\nAssociated type: [`PerfStateFlags`] (enum)"]
    Flags(u64),
}
impl<'a> IterablePerfState<'a> {
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PerfState::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PerfState",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "CPU performance (capacity) at a given frequency.\n"]
    pub fn get_performance(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PerfState::Performance(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PerfState",
            "Performance",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The frequency in KHz, for consistency with CPUFreq.\n"]
    pub fn get_frequency(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PerfState::Frequency(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PerfState",
            "Frequency",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The power consumed at this level (by 1 CPU or by a registered device).\nIt can be a total power: static and dynamic.\n"]
    pub fn get_power(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PerfState::Power(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PerfState",
            "Power",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The cost coefficient associated with this level, used during energy\ncalculation. Equal to: power \\* max_frequency / frequency.\n"]
    pub fn get_cost(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PerfState::Cost(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PerfState",
            "Cost",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Bitmask of performance state flags.\n\nAssociated type: [`PerfStateFlags`] (enum)"]
    pub fn get_flags(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PerfState::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PerfState",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl PerfState<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePerfState<'a> {
        IterablePerfState::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Pad",
            2u16 => "Performance",
            3u16 => "Frequency",
            4u16 => "Power",
            5u16 => "Cost",
            6u16 => "Flags",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePerfState<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePerfState<'a> {
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
impl<'a> Iterator for IterablePerfState<'a> {
    type Item = Result<PerfState<'a>, ErrorContext>;
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
                1u16 => PerfState::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => PerfState::Performance({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => PerfState::Frequency({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => PerfState::Power({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => PerfState::Cost({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => PerfState::Flags({
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
            "PerfState",
            r#type.and_then(|t| PerfState::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterablePerfState<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("PerfState");
        let mut iter = IterablePerfState::with_loc(&[], self.orig_loc);
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
                PerfState::Pad(val) => fmt.field("Pad", &val),
                PerfState::Performance(val) => fmt.field("Performance", &val),
                PerfState::Frequency(val) => fmt.field("Frequency", &val),
                PerfState::Power(val) => fmt.field("Power", &val),
                PerfState::Cost(val) => fmt.field("Cost", &val),
                PerfState::Flags(val) => fmt.field(
                    "Flags",
                    &FormatFlags(val.into(), PerfStateFlags::from_value),
                ),
            };
        }
        fmt.finish()
    }
}
impl IterablePerfState<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("PerfState", offset));
            return (
                stack,
                missing_type.and_then(|t| PerfState::attr_from_type(t)),
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
                PerfState::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                PerfState::Performance(val) => {
                    if last_off == offset {
                        stack.push(("Performance", last_off));
                        break;
                    }
                }
                PerfState::Frequency(val) => {
                    if last_off == offset {
                        stack.push(("Frequency", last_off));
                        break;
                    }
                }
                PerfState::Power(val) => {
                    if last_off == offset {
                        stack.push(("Power", last_off));
                        break;
                    }
                }
                PerfState::Cost(val) => {
                    if last_off == offset {
                        stack.push(("Cost", last_off));
                        break;
                    }
                }
                PerfState::Flags(val) => {
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
            stack.push(("PerfState", cur));
        }
        (stack, None)
    }
}
pub struct PushPerfDomain<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushPerfDomain<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushPerfDomain<Prev> {
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
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "A unique ID number for each performance domain.\n"]
    pub fn push_perf_domain_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Bitmask of performance domain flags.\n\nAssociated type: [`PerfDomainFlags`] (enum)"]
    pub fn push_flags(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 3u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "CPUs that belong to this performance domain.\n\nAttribute may repeat multiple times (treat it as array)"]
    pub fn push_cpus(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 4u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushPerfDomain<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPerfTable<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushPerfTable<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushPerfTable<Prev> {
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
    #[doc = "A unique ID number for each performance domain.\n"]
    pub fn push_perf_domain_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_perf_state(mut self) -> PushPerfState<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushPerfState {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushPerfTable<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPerfState<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushPerfState<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushPerfState<Prev> {
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
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "CPU performance (capacity) at a given frequency.\n"]
    pub fn push_performance(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 2u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The frequency in KHz, for consistency with CPUFreq.\n"]
    pub fn push_frequency(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 3u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The power consumed at this level (by 1 CPU or by a registered device).\nIt can be a total power: static and dynamic.\n"]
    pub fn push_power(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 4u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The cost coefficient associated with this level, used during energy\ncalculation. Equal to: power \\* max_frequency / frequency.\n"]
    pub fn push_cost(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 5u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Bitmask of performance state flags.\n\nAssociated type: [`PerfStateFlags`] (enum)"]
    pub fn push_flags(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 6u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushPerfState<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Notify attributes:\n- [`.get_perf_domain_id()`](IterablePerfTable::get_perf_domain_id)\n- [`.get_perf_state()`](IterablePerfTable::get_perf_state)\n"]
#[derive(Debug)]
pub struct OpPerfDomainCreatedNotif;
impl OpPerfDomainCreatedNotif {
    pub const CMD: u8 = 3u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterablePerfTable<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterablePerfTable::with_loc(attrs, buf.as_ptr() as usize)
    }
}
#[doc = "Notify attributes:\n- [`.get_perf_domain_id()`](IterablePerfTable::get_perf_domain_id)\n- [`.get_perf_state()`](IterablePerfTable::get_perf_state)\n"]
#[derive(Debug)]
pub struct OpPerfDomainUpdatedNotif;
impl OpPerfDomainUpdatedNotif {
    pub const CMD: u8 = 4u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterablePerfTable<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterablePerfTable::with_loc(attrs, buf.as_ptr() as usize)
    }
}
#[doc = "Notify attributes:\n- [`.get_perf_domain_id()`](IterablePerfTable::get_perf_domain_id)\n"]
#[derive(Debug)]
pub struct OpPerfDomainDeletedNotif;
impl OpPerfDomainDeletedNotif {
    pub const CMD: u8 = 5u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterablePerfTable<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterablePerfTable::with_loc(attrs, buf.as_ptr() as usize)
    }
}
pub struct NotifGroup;
impl NotifGroup {
    #[doc = "Notifications:\n- [`OpPerfDomainCreatedNotif`]\n- [`OpPerfDomainUpdatedNotif`]\n- [`OpPerfDomainDeletedNotif`]\n"]
    pub const EVENT: &str = "event";
    #[doc = "Notifications:\n- [`OpPerfDomainCreatedNotif`]\n- [`OpPerfDomainUpdatedNotif`]\n- [`OpPerfDomainDeletedNotif`]\n"]
    pub const EVENT_CSTR: &CStr = c"event";
}
#[doc = "Get the list of information for all performance domains.\n\nReply attributes:\n- [.get_perf_domain_id()](IterablePerfDomain::get_perf_domain_id)\n- [.get_flags()](IterablePerfDomain::get_flags)\n- [.get_cpus()](IterablePerfDomain::get_cpus)\n\n"]
#[derive(Debug)]
pub struct OpGetPerfDomainsDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetPerfDomainsDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushPerfDomain<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushPerfDomain::new(buf)
    }
    pub fn encode(&mut self) -> PushPerfDomain<&mut Vec<u8>> {
        PushPerfDomain::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushPerfDomain<RequestBuf<'r>> {
        PushPerfDomain::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterablePerfDomain<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterablePerfDomain::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpGetPerfDomainsDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("dev-energymodel".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterablePerfDomain<'buf>;
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
#[doc = "Get the list of information for all performance domains.\n\nRequest attributes:\n- [.push_perf_domain_id()](PushPerfDomain::push_perf_domain_id)\n\nReply attributes:\n- [.get_perf_domain_id()](IterablePerfDomain::get_perf_domain_id)\n- [.get_flags()](IterablePerfDomain::get_flags)\n- [.get_cpus()](IterablePerfDomain::get_cpus)\n\n"]
#[derive(Debug)]
pub struct OpGetPerfDomainsDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetPerfDomainsDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushPerfDomain<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushPerfDomain::new(buf)
    }
    pub fn encode(&mut self) -> PushPerfDomain<&mut Vec<u8>> {
        PushPerfDomain::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushPerfDomain<RequestBuf<'r>> {
        PushPerfDomain::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterablePerfDomain<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterablePerfDomain::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpGetPerfDomainsDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("dev-energymodel".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterablePerfDomain<'buf>;
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
#[doc = "Get the energy model table of a performance domain.\n\nRequest attributes:\n- [.push_perf_domain_id()](PushPerfTable::push_perf_domain_id)\n\nReply attributes:\n- [.get_perf_domain_id()](IterablePerfTable::get_perf_domain_id)\n- [.get_perf_state()](IterablePerfTable::get_perf_state)\n\n"]
#[derive(Debug)]
pub struct OpGetPerfTableDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetPerfTableDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushPerfTable<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushPerfTable::new(buf)
    }
    pub fn encode(&mut self) -> PushPerfTable<&mut Vec<u8>> {
        PushPerfTable::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushPerfTable<RequestBuf<'r>> {
        PushPerfTable::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterablePerfTable<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterablePerfTable::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpGetPerfTableDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("dev-energymodel".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterablePerfTable<'buf>;
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
    #[doc = "Get the list of information for all performance domains.\n\nReply attributes:\n- [.get_perf_domain_id()](IterablePerfDomain::get_perf_domain_id)\n- [.get_flags()](IterablePerfDomain::get_flags)\n- [.get_cpus()](IterablePerfDomain::get_cpus)\n\n"]
    pub fn op_get_perf_domains_dump(self) -> OpGetPerfDomainsDump<'buf> {
        let mut res = OpGetPerfDomainsDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-get-perf-domains-dump",
            OpGetPerfDomainsDump::lookup,
        );
        res
    }
    #[doc = "Get the list of information for all performance domains.\n\nRequest attributes:\n- [.push_perf_domain_id()](PushPerfDomain::push_perf_domain_id)\n\nReply attributes:\n- [.get_perf_domain_id()](IterablePerfDomain::get_perf_domain_id)\n- [.get_flags()](IterablePerfDomain::get_flags)\n- [.get_cpus()](IterablePerfDomain::get_cpus)\n\n"]
    pub fn op_get_perf_domains_do(self) -> OpGetPerfDomainsDo<'buf> {
        let mut res = OpGetPerfDomainsDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-get-perf-domains-do",
            OpGetPerfDomainsDo::lookup,
        );
        res
    }
    #[doc = "Get the energy model table of a performance domain.\n\nRequest attributes:\n- [.push_perf_domain_id()](PushPerfTable::push_perf_domain_id)\n\nReply attributes:\n- [.get_perf_domain_id()](IterablePerfTable::get_perf_domain_id)\n- [.get_perf_state()](IterablePerfTable::get_perf_state)\n\n"]
    pub fn op_get_perf_table_do(self) -> OpGetPerfTableDo<'buf> {
        let mut res = OpGetPerfTableDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-get-perf-table-do",
            OpGetPerfTableDo::lookup,
        );
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterablePerfDomain::get_cpus;
        let _ = IterablePerfDomain::get_flags;
        let _ = IterablePerfDomain::get_perf_domain_id;
        let _ = IterablePerfTable::get_perf_domain_id;
        let _ = IterablePerfTable::get_perf_state;
        let _ = OpPerfDomainCreatedNotif;
        let _ = OpPerfDomainDeletedNotif;
        let _ = OpPerfDomainUpdatedNotif;
        let _ = PushPerfDomain::<&mut Vec<u8>>::push_perf_domain_id;
        let _ = PushPerfTable::<&mut Vec<u8>>::push_perf_domain_id;
    }
}
