#![doc = "Networking HW rate limiting configuration.\n\nThis API allows configuring HW shapers available on the network devices\nat different levels (queues, network device) and allows arbitrary\nmanipulation of the scheduling tree of the involved shapers.\n\nEach \\@shaper is identified within the given device, by a \\@handle,\ncomprising both a \\@scope and an \\@id.\n\nDepending on the \\@scope value, the shapers are attached to specific HW\nobjects (queues, devices) or, for \\@node scope, represent a scheduling\ngroup, that can be placed in an arbitrary location of the scheduling\ntree.\n\nShapers can be created with two different operations: the \\@set\noperation, to create and update a single \\\"attached\\\" shaper, and the\n\\@group operation, to create and update a scheduling group. Only the\n\\@group operation can create \\@node scope shapers.\n\nExisting shapers can be deleted/reset via the \\@delete operation.\n\nThe user can query the running configuration via the \\@get operation.\n\nDifferent devices can provide different feature sets, e.g. with no\nsupport for complex scheduling hierarchy, or for some shaping\nparameters. The user can introspect the HW capabilities via the\n\\@cap-get operation.\n"]
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
pub const PROTONAME: &str = "net-shaper";
pub const PROTONAME_CSTR: &CStr = c"net-shaper";
#[doc = "Defines the shaper \\@id interpretation.\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum Scope {
    #[doc = "The scope is not specified.\n"]
    Unspec = 0,
    #[doc = "The main shaper for the given network device.\n"]
    Netdev = 1,
    #[doc = "The shaper is attached to the given device queue, the \\@id represents\nthe queue number.\n"]
    Queue = 2,
    #[doc = "The shaper allows grouping of queues or other node shapers; can be\nnested in either \\@netdev shapers or other \\@node shapers, allowing\nplacement in any location of the scheduling tree, except leaves and\nroot.\n"]
    Node = 3,
}
impl Scope {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Unspec,
            1 => Self::Netdev,
            2 => Self::Queue,
            3 => Self::Node,
            _ => return None,
        })
    }
}
#[doc = "Different metric supported by the shaper.\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum Metric {
    #[doc = "Shaper operates on a bits per second basis.\n"]
    Bps = 0,
    #[doc = "Shaper operates on a packets per second basis.\n"]
    Pps = 1,
}
impl Metric {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Bps,
            1 => Self::Pps,
            _ => return None,
        })
    }
}
#[derive(Clone)]
pub enum NetShaper<'a> {
    #[doc = "Unique identifier for the given shaper inside the owning device.\n"]
    Handle(IterableHandle<'a>),
    #[doc = "Metric used by the given shaper for bw-min, bw-max and burst.\n\nAssociated type: [`Metric`] (enum)"]
    Metric(u32),
    #[doc = "Guaranteed bandwidth for the given shaper.\n"]
    BwMin(u32),
    #[doc = "Maximum bandwidth for the given shaper or 0 when unlimited.\n"]
    BwMax(u32),
    #[doc = "Maximum burst-size for shaping. Should not be interpreted as a quantum.\n"]
    Burst(u32),
    #[doc = "Scheduling priority for the given shaper. The priority scheduling is\napplied to sibling shapers.\n"]
    Priority(u32),
    #[doc = "Relative weight for round robin scheduling of the given shaper. The\nscheduling is applied to all sibling shapers with the same priority.\n"]
    Weight(u32),
    #[doc = "Interface index owning the specified shaper.\n"]
    Ifindex(u32),
    #[doc = "Identifier for the parent of the affected shaper. Only needed for\n\\@group operation.\n"]
    Parent(IterableHandle<'a>),
    #[doc = "Describes a set of leaves shapers for a \\@group operation.\n\nAttribute may repeat multiple times (treat it as array)"]
    Leaves(IterableLeafInfo<'a>),
}
impl<'a> IterableNetShaper<'a> {
    #[doc = "Unique identifier for the given shaper inside the owning device.\n"]
    pub fn get_handle(&self) -> Result<IterableHandle<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetShaper::Handle(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetShaper",
            "Handle",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Metric used by the given shaper for bw-min, bw-max and burst.\n\nAssociated type: [`Metric`] (enum)"]
    pub fn get_metric(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetShaper::Metric(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetShaper",
            "Metric",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Guaranteed bandwidth for the given shaper.\n"]
    pub fn get_bw_min(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetShaper::BwMin(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetShaper",
            "BwMin",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Maximum bandwidth for the given shaper or 0 when unlimited.\n"]
    pub fn get_bw_max(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetShaper::BwMax(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetShaper",
            "BwMax",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Maximum burst-size for shaping. Should not be interpreted as a quantum.\n"]
    pub fn get_burst(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetShaper::Burst(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetShaper",
            "Burst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Scheduling priority for the given shaper. The priority scheduling is\napplied to sibling shapers.\n"]
    pub fn get_priority(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetShaper::Priority(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetShaper",
            "Priority",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Relative weight for round robin scheduling of the given shaper. The\nscheduling is applied to all sibling shapers with the same priority.\n"]
    pub fn get_weight(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetShaper::Weight(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetShaper",
            "Weight",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Interface index owning the specified shaper.\n"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetShaper::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetShaper",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Identifier for the parent of the affected shaper. Only needed for\n\\@group operation.\n"]
    pub fn get_parent(&self) -> Result<IterableHandle<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NetShaper::Parent(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NetShaper",
            "Parent",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Describes a set of leaves shapers for a \\@group operation.\n\nAttribute may repeat multiple times (treat it as array)"]
    pub fn get_leaves(&self) -> MultiAttrIterable<Self, NetShaper<'a>, IterableLeafInfo<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let NetShaper::Leaves(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl NetShaper<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableNetShaper<'a> {
        IterableNetShaper::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Handle",
            2u16 => "Metric",
            3u16 => "BwMin",
            4u16 => "BwMax",
            5u16 => "Burst",
            6u16 => "Priority",
            7u16 => "Weight",
            8u16 => "Ifindex",
            9u16 => "Parent",
            10u16 => "Leaves",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableNetShaper<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableNetShaper<'a> {
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
impl<'a> Iterator for IterableNetShaper<'a> {
    type Item = Result<NetShaper<'a>, ErrorContext>;
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
                1u16 => NetShaper::Handle({
                    let res = Some(IterableHandle::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => NetShaper::Metric({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => NetShaper::BwMin({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => NetShaper::BwMax({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => NetShaper::Burst({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => NetShaper::Priority({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => NetShaper::Weight({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => NetShaper::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => NetShaper::Parent({
                    let res = Some(IterableHandle::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => NetShaper::Leaves({
                    let res = Some(IterableLeafInfo::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "NetShaper",
            r#type.and_then(|t| NetShaper::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableNetShaper<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("NetShaper");
        let mut iter = IterableNetShaper::with_loc(&[], self.orig_loc);
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
                NetShaper::Handle(val) => fmt.field("Handle", &val),
                NetShaper::Metric(val) => {
                    fmt.field("Metric", &FormatEnum(val.into(), Metric::from_value))
                }
                NetShaper::BwMin(val) => fmt.field("BwMin", &val),
                NetShaper::BwMax(val) => fmt.field("BwMax", &val),
                NetShaper::Burst(val) => fmt.field("Burst", &val),
                NetShaper::Priority(val) => fmt.field("Priority", &val),
                NetShaper::Weight(val) => fmt.field("Weight", &val),
                NetShaper::Ifindex(val) => fmt.field("Ifindex", &val),
                NetShaper::Parent(val) => fmt.field("Parent", &val),
                NetShaper::Leaves(val) => fmt.field("Leaves", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableNetShaper<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("NetShaper", offset));
            return (
                stack,
                missing_type.and_then(|t| NetShaper::attr_from_type(t)),
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
                NetShaper::Handle(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                NetShaper::Metric(val) => {
                    if last_off == offset {
                        stack.push(("Metric", last_off));
                        break;
                    }
                }
                NetShaper::BwMin(val) => {
                    if last_off == offset {
                        stack.push(("BwMin", last_off));
                        break;
                    }
                }
                NetShaper::BwMax(val) => {
                    if last_off == offset {
                        stack.push(("BwMax", last_off));
                        break;
                    }
                }
                NetShaper::Burst(val) => {
                    if last_off == offset {
                        stack.push(("Burst", last_off));
                        break;
                    }
                }
                NetShaper::Priority(val) => {
                    if last_off == offset {
                        stack.push(("Priority", last_off));
                        break;
                    }
                }
                NetShaper::Weight(val) => {
                    if last_off == offset {
                        stack.push(("Weight", last_off));
                        break;
                    }
                }
                NetShaper::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                NetShaper::Parent(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                NetShaper::Leaves(val) => {
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
            stack.push(("NetShaper", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum Handle {
    #[doc = "Defines the shaper \\@id interpretation.\n\nAssociated type: [`Scope`] (enum)"]
    Scope(u32),
    #[doc = "Numeric identifier of a shaper. The id semantic depends on the scope.\nFor \\@queue scope it\\'s the queue id and for \\@node scope it\\'s the node\nidentifier.\n"]
    Id(u32),
}
impl<'a> IterableHandle<'a> {
    #[doc = "Defines the shaper \\@id interpretation.\n\nAssociated type: [`Scope`] (enum)"]
    pub fn get_scope(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Handle::Scope(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Handle",
            "Scope",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Numeric identifier of a shaper. The id semantic depends on the scope.\nFor \\@queue scope it\\'s the queue id and for \\@node scope it\\'s the node\nidentifier.\n"]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Handle::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Handle",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Handle {
    pub fn new<'a>(buf: &'a [u8]) -> IterableHandle<'a> {
        IterableHandle::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Scope",
            2u16 => "Id",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableHandle<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableHandle<'a> {
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
impl<'a> Iterator for IterableHandle<'a> {
    type Item = Result<Handle, ErrorContext>;
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
                1u16 => Handle::Scope({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Handle::Id({
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
            "Handle",
            r#type.and_then(|t| Handle::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableHandle<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Handle");
        let mut iter = IterableHandle::with_loc(&[], self.orig_loc);
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
                Handle::Scope(val) => {
                    fmt.field("Scope", &FormatEnum(val.into(), Scope::from_value))
                }
                Handle::Id(val) => fmt.field("Id", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableHandle<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Handle", offset));
            return (stack, missing_type.and_then(|t| Handle::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Handle::Scope(val) => {
                    if last_off == offset {
                        stack.push(("Scope", last_off));
                        break;
                    }
                }
                Handle::Id(val) => {
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
            stack.push(("Handle", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum LeafInfo<'a> {
    #[doc = "Unique identifier for the given shaper inside the owning device.\n"]
    Handle(IterableHandle<'a>),
    #[doc = "Scheduling priority for the given shaper. The priority scheduling is\napplied to sibling shapers.\n"]
    Priority(u32),
    #[doc = "Relative weight for round robin scheduling of the given shaper. The\nscheduling is applied to all sibling shapers with the same priority.\n"]
    Weight(u32),
}
impl<'a> IterableLeafInfo<'a> {
    #[doc = "Unique identifier for the given shaper inside the owning device.\n"]
    pub fn get_handle(&self) -> Result<IterableHandle<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LeafInfo::Handle(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LeafInfo",
            "Handle",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Scheduling priority for the given shaper. The priority scheduling is\napplied to sibling shapers.\n"]
    pub fn get_priority(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LeafInfo::Priority(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LeafInfo",
            "Priority",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Relative weight for round robin scheduling of the given shaper. The\nscheduling is applied to all sibling shapers with the same priority.\n"]
    pub fn get_weight(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(LeafInfo::Weight(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "LeafInfo",
            "Weight",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl LeafInfo<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLeafInfo<'a> {
        IterableLeafInfo::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        NetShaper::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLeafInfo<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLeafInfo<'a> {
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
impl<'a> Iterator for IterableLeafInfo<'a> {
    type Item = Result<LeafInfo<'a>, ErrorContext>;
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
                1u16 => LeafInfo::Handle({
                    let res = Some(IterableHandle::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => LeafInfo::Priority({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => LeafInfo::Weight({
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
            "LeafInfo",
            r#type.and_then(|t| LeafInfo::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableLeafInfo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("LeafInfo");
        let mut iter = IterableLeafInfo::with_loc(&[], self.orig_loc);
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
                LeafInfo::Handle(val) => fmt.field("Handle", &val),
                LeafInfo::Priority(val) => fmt.field("Priority", &val),
                LeafInfo::Weight(val) => fmt.field("Weight", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableLeafInfo<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("LeafInfo", offset));
            return (
                stack,
                missing_type.and_then(|t| LeafInfo::attr_from_type(t)),
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
                LeafInfo::Handle(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                LeafInfo::Priority(val) => {
                    if last_off == offset {
                        stack.push(("Priority", last_off));
                        break;
                    }
                }
                LeafInfo::Weight(val) => {
                    if last_off == offset {
                        stack.push(("Weight", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("LeafInfo", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum Caps {
    #[doc = "Interface index queried for shapers capabilities.\n"]
    Ifindex(u32),
    #[doc = "The scope to which the queried capabilities apply.\n\nAssociated type: [`Scope`] (enum)"]
    Scope(u32),
    #[doc = "The device accepts \\'bps\\' metric for bw-min, bw-max and burst.\n"]
    SupportMetricBps(()),
    #[doc = "The device accepts \\'pps\\' metric for bw-min, bw-max and burst.\n"]
    SupportMetricPps(()),
    #[doc = "The device supports nesting shaper belonging to this scope below\n\\'node\\' scoped shapers. Only \\'queue\\' and \\'node\\' scope can have flag\n\\'support-nesting\\'.\n"]
    SupportNesting(()),
    #[doc = "The device supports a minimum guaranteed B/W.\n"]
    SupportBwMin(()),
    #[doc = "The device supports maximum B/W shaping.\n"]
    SupportBwMax(()),
    #[doc = "The device supports a maximum burst size.\n"]
    SupportBurst(()),
    #[doc = "The device supports priority scheduling.\n"]
    SupportPriority(()),
    #[doc = "The device supports weighted round robin scheduling.\n"]
    SupportWeight(()),
}
impl<'a> IterableCaps<'a> {
    #[doc = "Interface index queried for shapers capabilities.\n"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Caps::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Caps",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The scope to which the queried capabilities apply.\n\nAssociated type: [`Scope`] (enum)"]
    pub fn get_scope(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Caps::Scope(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Caps",
            "Scope",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The device accepts \\'bps\\' metric for bw-min, bw-max and burst.\n"]
    pub fn get_support_metric_bps(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Caps::SupportMetricBps(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Caps",
            "SupportMetricBps",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The device accepts \\'pps\\' metric for bw-min, bw-max and burst.\n"]
    pub fn get_support_metric_pps(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Caps::SupportMetricPps(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Caps",
            "SupportMetricPps",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The device supports nesting shaper belonging to this scope below\n\\'node\\' scoped shapers. Only \\'queue\\' and \\'node\\' scope can have flag\n\\'support-nesting\\'.\n"]
    pub fn get_support_nesting(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Caps::SupportNesting(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Caps",
            "SupportNesting",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The device supports a minimum guaranteed B/W.\n"]
    pub fn get_support_bw_min(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Caps::SupportBwMin(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Caps",
            "SupportBwMin",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The device supports maximum B/W shaping.\n"]
    pub fn get_support_bw_max(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Caps::SupportBwMax(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Caps",
            "SupportBwMax",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The device supports a maximum burst size.\n"]
    pub fn get_support_burst(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Caps::SupportBurst(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Caps",
            "SupportBurst",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The device supports priority scheduling.\n"]
    pub fn get_support_priority(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Caps::SupportPriority(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Caps",
            "SupportPriority",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The device supports weighted round robin scheduling.\n"]
    pub fn get_support_weight(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Caps::SupportWeight(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Caps",
            "SupportWeight",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Caps {
    pub fn new<'a>(buf: &'a [u8]) -> IterableCaps<'a> {
        IterableCaps::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Ifindex",
            2u16 => "Scope",
            3u16 => "SupportMetricBps",
            4u16 => "SupportMetricPps",
            5u16 => "SupportNesting",
            6u16 => "SupportBwMin",
            7u16 => "SupportBwMax",
            8u16 => "SupportBurst",
            9u16 => "SupportPriority",
            10u16 => "SupportWeight",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableCaps<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableCaps<'a> {
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
impl<'a> Iterator for IterableCaps<'a> {
    type Item = Result<Caps, ErrorContext>;
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
                1u16 => Caps::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Caps::Scope({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Caps::SupportMetricBps(()),
                4u16 => Caps::SupportMetricPps(()),
                5u16 => Caps::SupportNesting(()),
                6u16 => Caps::SupportBwMin(()),
                7u16 => Caps::SupportBwMax(()),
                8u16 => Caps::SupportBurst(()),
                9u16 => Caps::SupportPriority(()),
                10u16 => Caps::SupportWeight(()),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Caps",
            r#type.and_then(|t| Caps::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableCaps<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Caps");
        let mut iter = IterableCaps::with_loc(&[], self.orig_loc);
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
                Caps::Ifindex(val) => fmt.field("Ifindex", &val),
                Caps::Scope(val) => fmt.field("Scope", &FormatEnum(val.into(), Scope::from_value)),
                Caps::SupportMetricBps(val) => fmt.field("SupportMetricBps", &val),
                Caps::SupportMetricPps(val) => fmt.field("SupportMetricPps", &val),
                Caps::SupportNesting(val) => fmt.field("SupportNesting", &val),
                Caps::SupportBwMin(val) => fmt.field("SupportBwMin", &val),
                Caps::SupportBwMax(val) => fmt.field("SupportBwMax", &val),
                Caps::SupportBurst(val) => fmt.field("SupportBurst", &val),
                Caps::SupportPriority(val) => fmt.field("SupportPriority", &val),
                Caps::SupportWeight(val) => fmt.field("SupportWeight", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableCaps<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Caps", offset));
            return (stack, missing_type.and_then(|t| Caps::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Caps::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                Caps::Scope(val) => {
                    if last_off == offset {
                        stack.push(("Scope", last_off));
                        break;
                    }
                }
                Caps::SupportMetricBps(val) => {
                    if last_off == offset {
                        stack.push(("SupportMetricBps", last_off));
                        break;
                    }
                }
                Caps::SupportMetricPps(val) => {
                    if last_off == offset {
                        stack.push(("SupportMetricPps", last_off));
                        break;
                    }
                }
                Caps::SupportNesting(val) => {
                    if last_off == offset {
                        stack.push(("SupportNesting", last_off));
                        break;
                    }
                }
                Caps::SupportBwMin(val) => {
                    if last_off == offset {
                        stack.push(("SupportBwMin", last_off));
                        break;
                    }
                }
                Caps::SupportBwMax(val) => {
                    if last_off == offset {
                        stack.push(("SupportBwMax", last_off));
                        break;
                    }
                }
                Caps::SupportBurst(val) => {
                    if last_off == offset {
                        stack.push(("SupportBurst", last_off));
                        break;
                    }
                }
                Caps::SupportPriority(val) => {
                    if last_off == offset {
                        stack.push(("SupportPriority", last_off));
                        break;
                    }
                }
                Caps::SupportWeight(val) => {
                    if last_off == offset {
                        stack.push(("SupportWeight", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Caps", cur));
        }
        (stack, None)
    }
}
pub struct PushNetShaper<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushNetShaper<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushNetShaper<Prev> {
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
    #[doc = "Unique identifier for the given shaper inside the owning device.\n"]
    pub fn nested_handle(mut self) -> PushHandle<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushHandle {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Metric used by the given shaper for bw-min, bw-max and burst.\n\nAssociated type: [`Metric`] (enum)"]
    pub fn push_metric(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Guaranteed bandwidth for the given shaper.\n"]
    pub fn push_bw_min(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Maximum bandwidth for the given shaper or 0 when unlimited.\n"]
    pub fn push_bw_max(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Maximum burst-size for shaping. Should not be interpreted as a quantum.\n"]
    pub fn push_burst(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Scheduling priority for the given shaper. The priority scheduling is\napplied to sibling shapers.\n"]
    pub fn push_priority(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Relative weight for round robin scheduling of the given shaper. The\nscheduling is applied to all sibling shapers with the same priority.\n"]
    pub fn push_weight(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Interface index owning the specified shaper.\n"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Identifier for the parent of the affected shaper. Only needed for\n\\@group operation.\n"]
    pub fn nested_parent(mut self) -> PushHandle<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 9u16);
        PushHandle {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Describes a set of leaves shapers for a \\@group operation.\n\nAttribute may repeat multiple times (treat it as array)"]
    pub fn nested_leaves(mut self) -> PushLeafInfo<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 10u16);
        PushLeafInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushNetShaper<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushHandle<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushHandle<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushHandle<Prev> {
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
    #[doc = "Defines the shaper \\@id interpretation.\n\nAssociated type: [`Scope`] (enum)"]
    pub fn push_scope(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Numeric identifier of a shaper. The id semantic depends on the scope.\nFor \\@queue scope it\\'s the queue id and for \\@node scope it\\'s the node\nidentifier.\n"]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushHandle<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLeafInfo<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLeafInfo<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLeafInfo<Prev> {
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
    #[doc = "Unique identifier for the given shaper inside the owning device.\n"]
    pub fn nested_handle(mut self) -> PushHandle<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushHandle {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Scheduling priority for the given shaper. The priority scheduling is\napplied to sibling shapers.\n"]
    pub fn push_priority(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Relative weight for round robin scheduling of the given shaper. The\nscheduling is applied to all sibling shapers with the same priority.\n"]
    pub fn push_weight(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushLeafInfo<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushCaps<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushCaps<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushCaps<Prev> {
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
    #[doc = "Interface index queried for shapers capabilities.\n"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The scope to which the queried capabilities apply.\n\nAssociated type: [`Scope`] (enum)"]
    pub fn push_scope(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The device accepts \\'bps\\' metric for bw-min, bw-max and burst.\n"]
    pub fn push_support_metric_bps(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 3u16, 0 as u16);
        self
    }
    #[doc = "The device accepts \\'pps\\' metric for bw-min, bw-max and burst.\n"]
    pub fn push_support_metric_pps(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 4u16, 0 as u16);
        self
    }
    #[doc = "The device supports nesting shaper belonging to this scope below\n\\'node\\' scoped shapers. Only \\'queue\\' and \\'node\\' scope can have flag\n\\'support-nesting\\'.\n"]
    pub fn push_support_nesting(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 5u16, 0 as u16);
        self
    }
    #[doc = "The device supports a minimum guaranteed B/W.\n"]
    pub fn push_support_bw_min(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 6u16, 0 as u16);
        self
    }
    #[doc = "The device supports maximum B/W shaping.\n"]
    pub fn push_support_bw_max(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 7u16, 0 as u16);
        self
    }
    #[doc = "The device supports a maximum burst size.\n"]
    pub fn push_support_burst(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 8u16, 0 as u16);
        self
    }
    #[doc = "The device supports priority scheduling.\n"]
    pub fn push_support_priority(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 9u16, 0 as u16);
        self
    }
    #[doc = "The device supports weighted round robin scheduling.\n"]
    pub fn push_support_weight(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 10u16, 0 as u16);
        self
    }
}
impl<Prev: Pusher> Drop for PushCaps<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get information about a shaper for a given device.\n\nRequest attributes:\n- [.push_ifindex()](PushNetShaper::push_ifindex)\n\nReply attributes:\n- [.get_handle()](IterableNetShaper::get_handle)\n- [.get_metric()](IterableNetShaper::get_metric)\n- [.get_bw_min()](IterableNetShaper::get_bw_min)\n- [.get_bw_max()](IterableNetShaper::get_bw_max)\n- [.get_burst()](IterableNetShaper::get_burst)\n- [.get_priority()](IterableNetShaper::get_priority)\n- [.get_weight()](IterableNetShaper::get_weight)\n- [.get_ifindex()](IterableNetShaper::get_ifindex)\n- [.get_parent()](IterableNetShaper::get_parent)\n\n"]
#[derive(Debug)]
pub struct OpGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushNetShaper<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushNetShaper::new(buf)
    }
    pub fn encode(&mut self) -> PushNetShaper<&mut Vec<u8>> {
        PushNetShaper::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushNetShaper<RequestBuf<'r>> {
        PushNetShaper::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableNetShaper<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableNetShaper::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("net-shaper".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableNetShaper<'buf>;
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
#[doc = "Get information about a shaper for a given device.\n\nRequest attributes:\n- [.nested_handle()](PushNetShaper::nested_handle)\n- [.push_ifindex()](PushNetShaper::push_ifindex)\n\nReply attributes:\n- [.get_handle()](IterableNetShaper::get_handle)\n- [.get_metric()](IterableNetShaper::get_metric)\n- [.get_bw_min()](IterableNetShaper::get_bw_min)\n- [.get_bw_max()](IterableNetShaper::get_bw_max)\n- [.get_burst()](IterableNetShaper::get_burst)\n- [.get_priority()](IterableNetShaper::get_priority)\n- [.get_weight()](IterableNetShaper::get_weight)\n- [.get_ifindex()](IterableNetShaper::get_ifindex)\n- [.get_parent()](IterableNetShaper::get_parent)\n\n"]
#[derive(Debug)]
pub struct OpGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushNetShaper<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushNetShaper::new(buf)
    }
    pub fn encode(&mut self) -> PushNetShaper<&mut Vec<u8>> {
        PushNetShaper::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushNetShaper<RequestBuf<'r>> {
        PushNetShaper::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableNetShaper<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableNetShaper::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("net-shaper".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableNetShaper<'buf>;
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
#[doc = "Create or update the specified shaper. The set operation can\\'t be used\nto create a \\@node scope shaper, use the \\@group operation instead.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_handle()](PushNetShaper::nested_handle)\n- [.push_metric()](PushNetShaper::push_metric)\n- [.push_bw_min()](PushNetShaper::push_bw_min)\n- [.push_bw_max()](PushNetShaper::push_bw_max)\n- [.push_burst()](PushNetShaper::push_burst)\n- [.push_priority()](PushNetShaper::push_priority)\n- [.push_weight()](PushNetShaper::push_weight)\n- [.push_ifindex()](PushNetShaper::push_ifindex)\n\n"]
#[derive(Debug)]
pub struct OpSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushNetShaper<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushNetShaper::new(buf)
    }
    pub fn encode(&mut self) -> PushNetShaper<&mut Vec<u8>> {
        PushNetShaper::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushNetShaper<RequestBuf<'r>> {
        PushNetShaper::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableNetShaper<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableNetShaper::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("net-shaper".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableNetShaper<'buf>;
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
#[doc = "Clear (remove) the specified shaper. When deleting a \\@node shaper,\nreattach all the node\\'s leaves to the deleted node\\'s parent. If, after\nthe removal, the parent shaper has no more leaves and the parent shaper\nscope is \\@node, the parent node is deleted, recursively. When deleting\na \\@queue shaper or a \\@netdev shaper, the shaper disappears from the\nhierarchy, but the queue/device can still send traffic: it has an\nimplicit node with infinite bandwidth. The queue\\'s implicit node feeds\nan implicit RR node at the root of the hierarchy.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_handle()](PushNetShaper::nested_handle)\n- [.push_ifindex()](PushNetShaper::push_ifindex)\n\n"]
#[derive(Debug)]
pub struct OpDeleteDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDeleteDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushNetShaper<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushNetShaper::new(buf)
    }
    pub fn encode(&mut self) -> PushNetShaper<&mut Vec<u8>> {
        PushNetShaper::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushNetShaper<RequestBuf<'r>> {
        PushNetShaper::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableNetShaper<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableNetShaper::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpDeleteDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("net-shaper".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableNetShaper<'buf>;
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
#[doc = "Create or update a scheduling group, attaching the specified \\@leaves\nshapers under the specified node identified by \\@handle. The \\@leaves\nshapers scope must be \\@queue and the node shaper scope must be either\n\\@node or \\@netdev. When the node shaper has \\@node scope, if the\n\\@handle \\@id is not specified, a new shaper of such scope is created,\notherwise the specified node must already exist. When updating an\nexisting node shaper, the specified \\@leaves are added to the existing\nnode; such node will also retain any preexisting leave. The \\@parent\nhandle for a new node shaper defaults to the parent of all the leaves,\nprovided all the leaves share the same parent. Otherwise \\@parent handle\nmust be specified. The user can optionally provide shaping attributes\nfor the node shaper. The operation is atomic, on failure no change is\napplied to the device shaping configuration, otherwise the \\@node shaper\nfull identifier, comprising \\@binding and \\@handle, is provided as the\nreply.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_handle()](PushNetShaper::nested_handle)\n- [.push_metric()](PushNetShaper::push_metric)\n- [.push_bw_min()](PushNetShaper::push_bw_min)\n- [.push_bw_max()](PushNetShaper::push_bw_max)\n- [.push_burst()](PushNetShaper::push_burst)\n- [.push_priority()](PushNetShaper::push_priority)\n- [.push_weight()](PushNetShaper::push_weight)\n- [.push_ifindex()](PushNetShaper::push_ifindex)\n- [.nested_parent()](PushNetShaper::nested_parent)\n- [.nested_leaves()](PushNetShaper::nested_leaves)\n\nReply attributes:\n- [.get_handle()](IterableNetShaper::get_handle)\n- [.get_ifindex()](IterableNetShaper::get_ifindex)\n\n"]
#[derive(Debug)]
pub struct OpGroupDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGroupDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushNetShaper<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushNetShaper::new(buf)
    }
    pub fn encode(&mut self) -> PushNetShaper<&mut Vec<u8>> {
        PushNetShaper::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushNetShaper<RequestBuf<'r>> {
        PushNetShaper::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableNetShaper<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableNetShaper::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpGroupDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("net-shaper".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableNetShaper<'buf>;
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
#[doc = "Get the shaper capabilities supported by the given device for the\nspecified scope.\n\nRequest attributes:\n- [.push_ifindex()](PushCaps::push_ifindex)\n\nReply attributes:\n- [.get_ifindex()](IterableCaps::get_ifindex)\n- [.get_scope()](IterableCaps::get_scope)\n- [.get_support_metric_bps()](IterableCaps::get_support_metric_bps)\n- [.get_support_metric_pps()](IterableCaps::get_support_metric_pps)\n- [.get_support_nesting()](IterableCaps::get_support_nesting)\n- [.get_support_bw_min()](IterableCaps::get_support_bw_min)\n- [.get_support_bw_max()](IterableCaps::get_support_bw_max)\n- [.get_support_burst()](IterableCaps::get_support_burst)\n- [.get_support_priority()](IterableCaps::get_support_priority)\n- [.get_support_weight()](IterableCaps::get_support_weight)\n\n"]
#[derive(Debug)]
pub struct OpCapGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpCapGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushCaps<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushCaps::new(buf)
    }
    pub fn encode(&mut self) -> PushCaps<&mut Vec<u8>> {
        PushCaps::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushCaps<RequestBuf<'r>> {
        PushCaps::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableCaps<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableCaps::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpCapGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("net-shaper".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableCaps<'buf>;
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
#[doc = "Get the shaper capabilities supported by the given device for the\nspecified scope.\n\nRequest attributes:\n- [.push_ifindex()](PushCaps::push_ifindex)\n- [.push_scope()](PushCaps::push_scope)\n\nReply attributes:\n- [.get_ifindex()](IterableCaps::get_ifindex)\n- [.get_scope()](IterableCaps::get_scope)\n- [.get_support_metric_bps()](IterableCaps::get_support_metric_bps)\n- [.get_support_metric_pps()](IterableCaps::get_support_metric_pps)\n- [.get_support_nesting()](IterableCaps::get_support_nesting)\n- [.get_support_bw_min()](IterableCaps::get_support_bw_min)\n- [.get_support_bw_max()](IterableCaps::get_support_bw_max)\n- [.get_support_burst()](IterableCaps::get_support_burst)\n- [.get_support_priority()](IterableCaps::get_support_priority)\n- [.get_support_weight()](IterableCaps::get_support_weight)\n\n"]
#[derive(Debug)]
pub struct OpCapGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpCapGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushCaps<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushCaps::new(buf)
    }
    pub fn encode(&mut self) -> PushCaps<&mut Vec<u8>> {
        PushCaps::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushCaps<RequestBuf<'r>> {
        PushCaps::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableCaps<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableCaps::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpCapGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("net-shaper".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableCaps<'buf>;
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
    #[doc = "Get information about a shaper for a given device.\n\nRequest attributes:\n- [.push_ifindex()](PushNetShaper::push_ifindex)\n\nReply attributes:\n- [.get_handle()](IterableNetShaper::get_handle)\n- [.get_metric()](IterableNetShaper::get_metric)\n- [.get_bw_min()](IterableNetShaper::get_bw_min)\n- [.get_bw_max()](IterableNetShaper::get_bw_max)\n- [.get_burst()](IterableNetShaper::get_burst)\n- [.get_priority()](IterableNetShaper::get_priority)\n- [.get_weight()](IterableNetShaper::get_weight)\n- [.get_ifindex()](IterableNetShaper::get_ifindex)\n- [.get_parent()](IterableNetShaper::get_parent)\n\n"]
    pub fn op_get_dump(self) -> OpGetDump<'buf> {
        let mut res = OpGetDump::new(self);
        res.request
            .do_writeback(res.protocol(), "op-get-dump", OpGetDump::lookup);
        res
    }
    #[doc = "Get information about a shaper for a given device.\n\nRequest attributes:\n- [.nested_handle()](PushNetShaper::nested_handle)\n- [.push_ifindex()](PushNetShaper::push_ifindex)\n\nReply attributes:\n- [.get_handle()](IterableNetShaper::get_handle)\n- [.get_metric()](IterableNetShaper::get_metric)\n- [.get_bw_min()](IterableNetShaper::get_bw_min)\n- [.get_bw_max()](IterableNetShaper::get_bw_max)\n- [.get_burst()](IterableNetShaper::get_burst)\n- [.get_priority()](IterableNetShaper::get_priority)\n- [.get_weight()](IterableNetShaper::get_weight)\n- [.get_ifindex()](IterableNetShaper::get_ifindex)\n- [.get_parent()](IterableNetShaper::get_parent)\n\n"]
    pub fn op_get_do(self) -> OpGetDo<'buf> {
        let mut res = OpGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-get-do", OpGetDo::lookup);
        res
    }
    #[doc = "Create or update the specified shaper. The set operation can\\'t be used\nto create a \\@node scope shaper, use the \\@group operation instead.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_handle()](PushNetShaper::nested_handle)\n- [.push_metric()](PushNetShaper::push_metric)\n- [.push_bw_min()](PushNetShaper::push_bw_min)\n- [.push_bw_max()](PushNetShaper::push_bw_max)\n- [.push_burst()](PushNetShaper::push_burst)\n- [.push_priority()](PushNetShaper::push_priority)\n- [.push_weight()](PushNetShaper::push_weight)\n- [.push_ifindex()](PushNetShaper::push_ifindex)\n\n"]
    pub fn op_set_do(self) -> OpSetDo<'buf> {
        let mut res = OpSetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-set-do", OpSetDo::lookup);
        res
    }
    #[doc = "Clear (remove) the specified shaper. When deleting a \\@node shaper,\nreattach all the node\\'s leaves to the deleted node\\'s parent. If, after\nthe removal, the parent shaper has no more leaves and the parent shaper\nscope is \\@node, the parent node is deleted, recursively. When deleting\na \\@queue shaper or a \\@netdev shaper, the shaper disappears from the\nhierarchy, but the queue/device can still send traffic: it has an\nimplicit node with infinite bandwidth. The queue\\'s implicit node feeds\nan implicit RR node at the root of the hierarchy.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_handle()](PushNetShaper::nested_handle)\n- [.push_ifindex()](PushNetShaper::push_ifindex)\n\n"]
    pub fn op_delete_do(self) -> OpDeleteDo<'buf> {
        let mut res = OpDeleteDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-delete-do", OpDeleteDo::lookup);
        res
    }
    #[doc = "Create or update a scheduling group, attaching the specified \\@leaves\nshapers under the specified node identified by \\@handle. The \\@leaves\nshapers scope must be \\@queue and the node shaper scope must be either\n\\@node or \\@netdev. When the node shaper has \\@node scope, if the\n\\@handle \\@id is not specified, a new shaper of such scope is created,\notherwise the specified node must already exist. When updating an\nexisting node shaper, the specified \\@leaves are added to the existing\nnode; such node will also retain any preexisting leave. The \\@parent\nhandle for a new node shaper defaults to the parent of all the leaves,\nprovided all the leaves share the same parent. Otherwise \\@parent handle\nmust be specified. The user can optionally provide shaping attributes\nfor the node shaper. The operation is atomic, on failure no change is\napplied to the device shaping configuration, otherwise the \\@node shaper\nfull identifier, comprising \\@binding and \\@handle, is provided as the\nreply.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.nested_handle()](PushNetShaper::nested_handle)\n- [.push_metric()](PushNetShaper::push_metric)\n- [.push_bw_min()](PushNetShaper::push_bw_min)\n- [.push_bw_max()](PushNetShaper::push_bw_max)\n- [.push_burst()](PushNetShaper::push_burst)\n- [.push_priority()](PushNetShaper::push_priority)\n- [.push_weight()](PushNetShaper::push_weight)\n- [.push_ifindex()](PushNetShaper::push_ifindex)\n- [.nested_parent()](PushNetShaper::nested_parent)\n- [.nested_leaves()](PushNetShaper::nested_leaves)\n\nReply attributes:\n- [.get_handle()](IterableNetShaper::get_handle)\n- [.get_ifindex()](IterableNetShaper::get_ifindex)\n\n"]
    pub fn op_group_do(self) -> OpGroupDo<'buf> {
        let mut res = OpGroupDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-group-do", OpGroupDo::lookup);
        res
    }
    #[doc = "Get the shaper capabilities supported by the given device for the\nspecified scope.\n\nRequest attributes:\n- [.push_ifindex()](PushCaps::push_ifindex)\n\nReply attributes:\n- [.get_ifindex()](IterableCaps::get_ifindex)\n- [.get_scope()](IterableCaps::get_scope)\n- [.get_support_metric_bps()](IterableCaps::get_support_metric_bps)\n- [.get_support_metric_pps()](IterableCaps::get_support_metric_pps)\n- [.get_support_nesting()](IterableCaps::get_support_nesting)\n- [.get_support_bw_min()](IterableCaps::get_support_bw_min)\n- [.get_support_bw_max()](IterableCaps::get_support_bw_max)\n- [.get_support_burst()](IterableCaps::get_support_burst)\n- [.get_support_priority()](IterableCaps::get_support_priority)\n- [.get_support_weight()](IterableCaps::get_support_weight)\n\n"]
    pub fn op_cap_get_dump(self) -> OpCapGetDump<'buf> {
        let mut res = OpCapGetDump::new(self);
        res.request
            .do_writeback(res.protocol(), "op-cap-get-dump", OpCapGetDump::lookup);
        res
    }
    #[doc = "Get the shaper capabilities supported by the given device for the\nspecified scope.\n\nRequest attributes:\n- [.push_ifindex()](PushCaps::push_ifindex)\n- [.push_scope()](PushCaps::push_scope)\n\nReply attributes:\n- [.get_ifindex()](IterableCaps::get_ifindex)\n- [.get_scope()](IterableCaps::get_scope)\n- [.get_support_metric_bps()](IterableCaps::get_support_metric_bps)\n- [.get_support_metric_pps()](IterableCaps::get_support_metric_pps)\n- [.get_support_nesting()](IterableCaps::get_support_nesting)\n- [.get_support_bw_min()](IterableCaps::get_support_bw_min)\n- [.get_support_bw_max()](IterableCaps::get_support_bw_max)\n- [.get_support_burst()](IterableCaps::get_support_burst)\n- [.get_support_priority()](IterableCaps::get_support_priority)\n- [.get_support_weight()](IterableCaps::get_support_weight)\n\n"]
    pub fn op_cap_get_do(self) -> OpCapGetDo<'buf> {
        let mut res = OpCapGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-cap-get-do", OpCapGetDo::lookup);
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableCaps::get_ifindex;
        let _ = IterableCaps::get_scope;
        let _ = IterableCaps::get_support_burst;
        let _ = IterableCaps::get_support_bw_max;
        let _ = IterableCaps::get_support_bw_min;
        let _ = IterableCaps::get_support_metric_bps;
        let _ = IterableCaps::get_support_metric_pps;
        let _ = IterableCaps::get_support_nesting;
        let _ = IterableCaps::get_support_priority;
        let _ = IterableCaps::get_support_weight;
        let _ = IterableNetShaper::get_burst;
        let _ = IterableNetShaper::get_bw_max;
        let _ = IterableNetShaper::get_bw_min;
        let _ = IterableNetShaper::get_handle;
        let _ = IterableNetShaper::get_ifindex;
        let _ = IterableNetShaper::get_metric;
        let _ = IterableNetShaper::get_parent;
        let _ = IterableNetShaper::get_priority;
        let _ = IterableNetShaper::get_weight;
        let _ = PushCaps::<&mut Vec<u8>>::push_ifindex;
        let _ = PushCaps::<&mut Vec<u8>>::push_scope;
        let _ = PushNetShaper::<&mut Vec<u8>>::nested_handle;
        let _ = PushNetShaper::<&mut Vec<u8>>::nested_leaves;
        let _ = PushNetShaper::<&mut Vec<u8>>::nested_parent;
        let _ = PushNetShaper::<&mut Vec<u8>>::push_burst;
        let _ = PushNetShaper::<&mut Vec<u8>>::push_bw_max;
        let _ = PushNetShaper::<&mut Vec<u8>>::push_bw_min;
        let _ = PushNetShaper::<&mut Vec<u8>>::push_ifindex;
        let _ = PushNetShaper::<&mut Vec<u8>>::push_metric;
        let _ = PushNetShaper::<&mut Vec<u8>>::push_priority;
        let _ = PushNetShaper::<&mut Vec<u8>>::push_weight;
    }
}
