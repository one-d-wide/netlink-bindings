#![doc = "OVS datapath configuration over generic netlink.\n"]
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
pub const PROTONAME: &str = "ovs_datapath";
pub const PROTONAME_CSTR: &CStr = c"ovs_datapath";
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum UserFeatures {
    #[doc = "Allow last Netlink attribute to be unaligned\n"]
    Unaligned = 1 << 0,
    #[doc = "Allow datapath to associate multiple Netlink PIDs to each vport\n"]
    VportPids = 1 << 1,
    #[doc = "Allow tc offload recirc sharing\n"]
    TcRecircSharing = 1 << 2,
    #[doc = "Allow per-cpu dispatch of upcalls\n"]
    DispatchUpcallPerCpu = 1 << 3,
}
impl UserFeatures {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::Unaligned,
            n if n == 1 << 1 => Self::VportPids,
            n if n == 1 << 2 => Self::TcRecircSharing,
            n if n == 1 << 3 => Self::DispatchUpcallPerCpu,
            _ => return None,
        })
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct OvsHeader {
    pub dp_ifindex: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for OvsHeader {
    fn default() -> Self {
        Self::new()
    }
}
impl OvsHeader {
    #[doc = "Create zero-initialized struct"]
    pub fn new() -> Self {
        Self::new_from_array([0u8; Self::len()])
    }
    #[doc = "Copy from contents from slice"]
    pub fn new_from_slice(other: &[u8]) -> Option<Self> {
        if other.len() != Self::len() {
            return None;
        }
        let mut buf = [0u8; Self::len()];
        buf.clone_from_slice(other);
        Some(Self::new_from_array(buf))
    }
    #[doc = "Copy from contents from another slice, padding with zeros or truncating when needed"]
    pub fn new_from_zeroed(other: &[u8]) -> Self {
        let mut buf = [0u8; Self::len()];
        let len = buf.len().min(other.len());
        buf[..len].clone_from_slice(&other[..len]);
        Self::new_from_array(buf)
    }
    pub fn new_from_array(buf: [u8; 4usize]) -> Self {
        unsafe { std::mem::transmute(buf) }
    }
    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            let ptr: *const u8 = std::mem::transmute(self as *const Self);
            std::slice::from_raw_parts(ptr, Self::len())
        }
    }
    pub fn from_slice(buf: &[u8]) -> &Self {
        assert!(buf.len() >= Self::len());
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf.as_ptr()) }
    }
    pub fn from_slice_mut(buf: &mut [u8]) -> &mut Self {
        assert!(buf.len() >= Self::len());
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf.as_ptr()) }
    }
    pub fn as_array(&self) -> &[u8; 4usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 4usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 4usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<OvsHeader>() == 4usize);
        const _: () = assert!(std::mem::align_of::<OvsHeader>() == 4usize);
        4usize
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(packed(4))]
#[repr(C)]
pub struct OvsDpStats {
    pub n_hit: u64,
    pub n_missed: u64,
    pub n_lost: u64,
    pub n_flows: u64,
}
#[doc = "Create zero-initialized struct"]
impl Default for OvsDpStats {
    fn default() -> Self {
        Self::new()
    }
}
impl OvsDpStats {
    #[doc = "Create zero-initialized struct"]
    pub fn new() -> Self {
        Self::new_from_array([0u8; Self::len()])
    }
    #[doc = "Copy from contents from slice"]
    pub fn new_from_slice(other: &[u8]) -> Option<Self> {
        if other.len() != Self::len() {
            return None;
        }
        let mut buf = [0u8; Self::len()];
        buf.clone_from_slice(other);
        Some(Self::new_from_array(buf))
    }
    #[doc = "Copy from contents from another slice, padding with zeros or truncating when needed"]
    pub fn new_from_zeroed(other: &[u8]) -> Self {
        let mut buf = [0u8; Self::len()];
        let len = buf.len().min(other.len());
        buf[..len].clone_from_slice(&other[..len]);
        Self::new_from_array(buf)
    }
    pub fn new_from_array(buf: [u8; 32usize]) -> Self {
        unsafe { std::mem::transmute(buf) }
    }
    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            let ptr: *const u8 = std::mem::transmute(self as *const Self);
            std::slice::from_raw_parts(ptr, Self::len())
        }
    }
    pub fn from_slice(buf: &[u8]) -> &Self {
        assert!(buf.len() >= Self::len());
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf.as_ptr()) }
    }
    pub fn from_slice_mut(buf: &mut [u8]) -> &mut Self {
        assert!(buf.len() >= Self::len());
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf.as_ptr()) }
    }
    pub fn as_array(&self) -> &[u8; 32usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 32usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 32usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<OvsDpStats>() == 32usize);
        const _: () = assert!(std::mem::align_of::<OvsDpStats>() == 4usize);
        32usize
    }
}
impl std::fmt::Debug for OvsDpStats {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("OvsDpStats")
            .field("n_hit", &{ self.n_hit })
            .field("n_missed", &{ self.n_missed })
            .field("n_lost", &{ self.n_lost })
            .field("n_flows", &{ self.n_flows })
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(packed(4))]
#[repr(C)]
pub struct OvsDpMegaflowStats {
    pub n_mask_hit: u64,
    pub n_masks: u32,
    pub padding: u32,
    pub n_cache_hit: u64,
    pub pad1: u64,
}
#[doc = "Create zero-initialized struct"]
impl Default for OvsDpMegaflowStats {
    fn default() -> Self {
        Self::new()
    }
}
impl OvsDpMegaflowStats {
    #[doc = "Create zero-initialized struct"]
    pub fn new() -> Self {
        Self::new_from_array([0u8; Self::len()])
    }
    #[doc = "Copy from contents from slice"]
    pub fn new_from_slice(other: &[u8]) -> Option<Self> {
        if other.len() != Self::len() {
            return None;
        }
        let mut buf = [0u8; Self::len()];
        buf.clone_from_slice(other);
        Some(Self::new_from_array(buf))
    }
    #[doc = "Copy from contents from another slice, padding with zeros or truncating when needed"]
    pub fn new_from_zeroed(other: &[u8]) -> Self {
        let mut buf = [0u8; Self::len()];
        let len = buf.len().min(other.len());
        buf[..len].clone_from_slice(&other[..len]);
        Self::new_from_array(buf)
    }
    pub fn new_from_array(buf: [u8; 32usize]) -> Self {
        unsafe { std::mem::transmute(buf) }
    }
    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            let ptr: *const u8 = std::mem::transmute(self as *const Self);
            std::slice::from_raw_parts(ptr, Self::len())
        }
    }
    pub fn from_slice(buf: &[u8]) -> &Self {
        assert!(buf.len() >= Self::len());
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf.as_ptr()) }
    }
    pub fn from_slice_mut(buf: &mut [u8]) -> &mut Self {
        assert!(buf.len() >= Self::len());
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf.as_ptr()) }
    }
    pub fn as_array(&self) -> &[u8; 32usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 32usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 32usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<OvsDpMegaflowStats>() == 32usize);
        const _: () = assert!(std::mem::align_of::<OvsDpMegaflowStats>() == 4usize);
        32usize
    }
}
impl std::fmt::Debug for OvsDpMegaflowStats {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("OvsDpMegaflowStats")
            .field("n_mask_hit", &{ self.n_mask_hit })
            .field("n_masks", &self.n_masks)
            .field("padding", &self.padding)
            .field("n_cache_hit", &{ self.n_cache_hit })
            .field("pad1", &{ self.pad1 })
            .finish()
    }
}
#[derive(Clone)]
pub enum Datapath<'a> {
    Name(&'a CStr),
    #[doc = "upcall pid\n"]
    UpcallPid(u32),
    Stats(OvsDpStats),
    MegaflowStats(OvsDpMegaflowStats),
    #[doc = "Associated type: [`UserFeatures`] (1 bit per enumeration)"]
    UserFeatures(u32),
    MasksCacheSize(u32),
    PerCpuPids(&'a [u8]),
    Ifindex(u32),
}
impl<'a> IterableDatapath<'a> {
    pub fn get_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Datapath::Name(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Datapath",
            "Name",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "upcall pid\n"]
    pub fn get_upcall_pid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Datapath::UpcallPid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Datapath",
            "UpcallPid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_stats(&self) -> Result<OvsDpStats, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Datapath::Stats(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Datapath",
            "Stats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_megaflow_stats(&self) -> Result<OvsDpMegaflowStats, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Datapath::MegaflowStats(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Datapath",
            "MegaflowStats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`UserFeatures`] (1 bit per enumeration)"]
    pub fn get_user_features(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Datapath::UserFeatures(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Datapath",
            "UserFeatures",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_masks_cache_size(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Datapath::MasksCacheSize(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Datapath",
            "MasksCacheSize",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_per_cpu_pids(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Datapath::PerCpuPids(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Datapath",
            "PerCpuPids",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Datapath::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Datapath",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Datapath<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDatapath<'a> {
        IterableDatapath::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Name",
            2u16 => "UpcallPid",
            3u16 => "Stats",
            4u16 => "MegaflowStats",
            5u16 => "UserFeatures",
            6u16 => "Pad",
            7u16 => "MasksCacheSize",
            8u16 => "PerCpuPids",
            9u16 => "Ifindex",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDatapath<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDatapath<'a> {
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
impl<'a> Iterator for IterableDatapath<'a> {
    type Item = Result<Datapath<'a>, ErrorContext>;
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
                1u16 => Datapath::Name({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Datapath::UpcallPid({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Datapath::Stats({
                    let res = Some(OvsDpStats::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Datapath::MegaflowStats({
                    let res = Some(OvsDpMegaflowStats::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Datapath::UserFeatures({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => Datapath::MasksCacheSize({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => Datapath::PerCpuPids({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => Datapath::Ifindex({
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
            "Datapath",
            r#type.and_then(|t| Datapath::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDatapath<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Datapath");
        let mut iter = IterableDatapath::with_loc(&[], self.orig_loc);
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
                Datapath::Name(val) => fmt.field("Name", &val),
                Datapath::UpcallPid(val) => fmt.field("UpcallPid", &val),
                Datapath::Stats(val) => fmt.field("Stats", &val),
                Datapath::MegaflowStats(val) => fmt.field("MegaflowStats", &val),
                Datapath::UserFeatures(val) => fmt.field(
                    "UserFeatures",
                    &FormatFlags(val.into(), UserFeatures::from_value),
                ),
                Datapath::MasksCacheSize(val) => fmt.field("MasksCacheSize", &val),
                Datapath::PerCpuPids(val) => fmt.field("PerCpuPids", &FormatHexdump(val)),
                Datapath::Ifindex(val) => fmt.field("Ifindex", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDatapath<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Datapath", offset));
            return (
                stack,
                missing_type.and_then(|t| Datapath::attr_from_type(t)),
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
                Datapath::Name(val) => {
                    if last_off == offset {
                        stack.push(("Name", last_off));
                        break;
                    }
                }
                Datapath::UpcallPid(val) => {
                    if last_off == offset {
                        stack.push(("UpcallPid", last_off));
                        break;
                    }
                }
                Datapath::Stats(val) => {
                    if last_off == offset {
                        stack.push(("Stats", last_off));
                        break;
                    }
                }
                Datapath::MegaflowStats(val) => {
                    if last_off == offset {
                        stack.push(("MegaflowStats", last_off));
                        break;
                    }
                }
                Datapath::UserFeatures(val) => {
                    if last_off == offset {
                        stack.push(("UserFeatures", last_off));
                        break;
                    }
                }
                Datapath::MasksCacheSize(val) => {
                    if last_off == offset {
                        stack.push(("MasksCacheSize", last_off));
                        break;
                    }
                }
                Datapath::PerCpuPids(val) => {
                    if last_off == offset {
                        stack.push(("PerCpuPids", last_off));
                        break;
                    }
                }
                Datapath::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Datapath", cur));
        }
        (stack, None)
    }
}
pub struct PushDatapath<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDatapath<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDatapath<Prev> {
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
    #[doc = "upcall pid\n"]
    pub fn push_upcall_pid(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_stats(mut self, value: OvsDpStats) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_megaflow_stats(mut self, value: OvsDpMegaflowStats) -> Self {
        push_header(self.as_vec_mut(), 4u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    #[doc = "Associated type: [`UserFeatures`] (1 bit per enumeration)"]
    pub fn push_user_features(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_masks_cache_size(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_per_cpu_pids(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 8u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushDatapath<Prev> {
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
    pub const OVS_DATAPATH: &str = "ovs_datapath";
    pub const OVS_DATAPATH_CSTR: &CStr = c"ovs_datapath";
}
#[doc = "Get / dump OVS data path configuration and state\n\nRequest attributes:\n- [.push_name()](PushDatapath::push_name)\n\nReply attributes:\n- [.get_name()](IterableDatapath::get_name)\n- [.get_upcall_pid()](IterableDatapath::get_upcall_pid)\n- [.get_stats()](IterableDatapath::get_stats)\n- [.get_megaflow_stats()](IterableDatapath::get_megaflow_stats)\n- [.get_user_features()](IterableDatapath::get_user_features)\n- [.get_masks_cache_size()](IterableDatapath::get_masks_cache_size)\n- [.get_per_cpu_pids()](IterableDatapath::get_per_cpu_pids)\n\n"]
#[derive(Debug)]
pub struct OpGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetDump<'r> {
    pub fn new(mut request: Request<'r>, header: &OvsHeader) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &OvsHeader,
    ) -> PushDatapath<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushDatapath::new(buf)
    }
    pub fn encode(&mut self) -> PushDatapath<&mut Vec<u8>> {
        PushDatapath::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDatapath<RequestBuf<'r>> {
        PushDatapath::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (OvsHeader, IterableDatapath<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(OvsHeader::len()));
        (
            OvsHeader::new_from_slice(header).unwrap_or_default(),
            IterableDatapath::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &OvsHeader) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &OvsHeader {
        let pos = self.request.pos;
        OvsHeader::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut OvsHeader {
        let pos = self.request.pos;
        OvsHeader::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("ovs_datapath".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (OvsHeader, IterableDatapath<'buf>);
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        Self::decode_request(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        Self::decode_request(buf)
            .1
            .lookup_attr(offset, missing_type)
    }
}
#[doc = "Get / dump OVS data path configuration and state\n\nRequest attributes:\n- [.push_name()](PushDatapath::push_name)\n\nReply attributes:\n- [.get_name()](IterableDatapath::get_name)\n- [.get_upcall_pid()](IterableDatapath::get_upcall_pid)\n- [.get_stats()](IterableDatapath::get_stats)\n- [.get_megaflow_stats()](IterableDatapath::get_megaflow_stats)\n- [.get_user_features()](IterableDatapath::get_user_features)\n- [.get_masks_cache_size()](IterableDatapath::get_masks_cache_size)\n- [.get_per_cpu_pids()](IterableDatapath::get_per_cpu_pids)\n\n"]
#[derive(Debug)]
pub struct OpGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetDo<'r> {
    pub fn new(mut request: Request<'r>, header: &OvsHeader) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &OvsHeader,
    ) -> PushDatapath<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushDatapath::new(buf)
    }
    pub fn encode(&mut self) -> PushDatapath<&mut Vec<u8>> {
        PushDatapath::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDatapath<RequestBuf<'r>> {
        PushDatapath::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (OvsHeader, IterableDatapath<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(OvsHeader::len()));
        (
            OvsHeader::new_from_slice(header).unwrap_or_default(),
            IterableDatapath::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &OvsHeader) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &OvsHeader {
        let pos = self.request.pos;
        OvsHeader::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut OvsHeader {
        let pos = self.request.pos;
        OvsHeader::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("ovs_datapath".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (OvsHeader, IterableDatapath<'buf>);
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        Self::decode_request(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        Self::decode_request(buf)
            .1
            .lookup_attr(offset, missing_type)
    }
}
#[doc = "Create new OVS data path\n\nRequest attributes:\n- [.push_name()](PushDatapath::push_name)\n- [.push_upcall_pid()](PushDatapath::push_upcall_pid)\n- [.push_user_features()](PushDatapath::push_user_features)\n\n"]
#[derive(Debug)]
pub struct OpNewDo<'r> {
    request: Request<'r>,
}
impl<'r> OpNewDo<'r> {
    pub fn new(mut request: Request<'r>, header: &OvsHeader) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &OvsHeader,
    ) -> PushDatapath<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushDatapath::new(buf)
    }
    pub fn encode(&mut self) -> PushDatapath<&mut Vec<u8>> {
        PushDatapath::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDatapath<RequestBuf<'r>> {
        PushDatapath::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (OvsHeader, IterableDatapath<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(OvsHeader::len()));
        (
            OvsHeader::new_from_slice(header).unwrap_or_default(),
            IterableDatapath::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &OvsHeader) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &OvsHeader {
        let pos = self.request.pos;
        OvsHeader::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut OvsHeader {
        let pos = self.request.pos;
        OvsHeader::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpNewDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("ovs_datapath".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (OvsHeader, IterableDatapath<'buf>);
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        Self::decode_request(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        Self::decode_request(buf)
            .1
            .lookup_attr(offset, missing_type)
    }
}
#[doc = "Delete existing OVS data path\n\nRequest attributes:\n- [.push_name()](PushDatapath::push_name)\n\n"]
#[derive(Debug)]
pub struct OpDelDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDelDo<'r> {
    pub fn new(mut request: Request<'r>, header: &OvsHeader) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &OvsHeader,
    ) -> PushDatapath<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushDatapath::new(buf)
    }
    pub fn encode(&mut self) -> PushDatapath<&mut Vec<u8>> {
        PushDatapath::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDatapath<RequestBuf<'r>> {
        PushDatapath::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (OvsHeader, IterableDatapath<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(OvsHeader::len()));
        (
            OvsHeader::new_from_slice(header).unwrap_or_default(),
            IterableDatapath::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &OvsHeader) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &OvsHeader {
        let pos = self.request.pos;
        OvsHeader::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut OvsHeader {
        let pos = self.request.pos;
        OvsHeader::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpDelDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("ovs_datapath".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (OvsHeader, IterableDatapath<'buf>);
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        Self::decode_request(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        Self::decode_request(buf)
            .1
            .lookup_attr(offset, missing_type)
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
    #[doc = "Get / dump OVS data path configuration and state\n\nRequest attributes:\n- [.push_name()](PushDatapath::push_name)\n\nReply attributes:\n- [.get_name()](IterableDatapath::get_name)\n- [.get_upcall_pid()](IterableDatapath::get_upcall_pid)\n- [.get_stats()](IterableDatapath::get_stats)\n- [.get_megaflow_stats()](IterableDatapath::get_megaflow_stats)\n- [.get_user_features()](IterableDatapath::get_user_features)\n- [.get_masks_cache_size()](IterableDatapath::get_masks_cache_size)\n- [.get_per_cpu_pids()](IterableDatapath::get_per_cpu_pids)\n\n"]
    pub fn op_get_dump(self, header: &OvsHeader) -> OpGetDump<'buf> {
        let mut res = OpGetDump::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-get-dump", OpGetDump::lookup);
        res
    }
    #[doc = "Get / dump OVS data path configuration and state\n\nRequest attributes:\n- [.push_name()](PushDatapath::push_name)\n\nReply attributes:\n- [.get_name()](IterableDatapath::get_name)\n- [.get_upcall_pid()](IterableDatapath::get_upcall_pid)\n- [.get_stats()](IterableDatapath::get_stats)\n- [.get_megaflow_stats()](IterableDatapath::get_megaflow_stats)\n- [.get_user_features()](IterableDatapath::get_user_features)\n- [.get_masks_cache_size()](IterableDatapath::get_masks_cache_size)\n- [.get_per_cpu_pids()](IterableDatapath::get_per_cpu_pids)\n\n"]
    pub fn op_get_do(self, header: &OvsHeader) -> OpGetDo<'buf> {
        let mut res = OpGetDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-get-do", OpGetDo::lookup);
        res
    }
    #[doc = "Create new OVS data path\n\nRequest attributes:\n- [.push_name()](PushDatapath::push_name)\n- [.push_upcall_pid()](PushDatapath::push_upcall_pid)\n- [.push_user_features()](PushDatapath::push_user_features)\n\n"]
    pub fn op_new_do(self, header: &OvsHeader) -> OpNewDo<'buf> {
        let mut res = OpNewDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-new-do", OpNewDo::lookup);
        res
    }
    #[doc = "Delete existing OVS data path\n\nRequest attributes:\n- [.push_name()](PushDatapath::push_name)\n\n"]
    pub fn op_del_do(self, header: &OvsHeader) -> OpDelDo<'buf> {
        let mut res = OpDelDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-del-do", OpDelDo::lookup);
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableDatapath::get_masks_cache_size;
        let _ = IterableDatapath::get_megaflow_stats;
        let _ = IterableDatapath::get_name;
        let _ = IterableDatapath::get_per_cpu_pids;
        let _ = IterableDatapath::get_stats;
        let _ = IterableDatapath::get_upcall_pid;
        let _ = IterableDatapath::get_user_features;
        let _ = PushDatapath::<&mut Vec<u8>>::push_name;
        let _ = PushDatapath::<&mut Vec<u8>>::push_upcall_pid;
        let _ = PushDatapath::<&mut Vec<u8>>::push_user_features;
    }
}
