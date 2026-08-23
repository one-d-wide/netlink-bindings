#![doc = "OVS vport configuration over generic netlink.\n"]
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
pub const PROTONAME: &str = "ovs_vport";
pub const PROTONAME_CSTR: &CStr = c"ovs_vport";
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum VportType {
    Unspec = 0,
    Netdev = 1,
    Internal = 2,
    Gre = 3,
    Vxlan = 4,
    Geneve = 5,
}
impl VportType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Unspec,
            1 => Self::Netdev,
            2 => Self::Internal,
            3 => Self::Gre,
            4 => Self::Vxlan,
            5 => Self::Geneve,
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
pub struct OvsVportStats {
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
    pub rx_dropped: u64,
    pub tx_dropped: u64,
}
#[doc = "Create zero-initialized struct"]
impl Default for OvsVportStats {
    fn default() -> Self {
        Self::new()
    }
}
impl OvsVportStats {
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
    pub fn new_from_array(buf: [u8; 64usize]) -> Self {
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
    pub fn as_array(&self) -> &[u8; 64usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 64usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 64usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<OvsVportStats>() == 64usize);
        const _: () = assert!(std::mem::align_of::<OvsVportStats>() == 4usize);
        64usize
    }
}
impl std::fmt::Debug for OvsVportStats {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("OvsVportStats")
            .field("rx_packets", &{ self.rx_packets })
            .field("tx_packets", &{ self.tx_packets })
            .field("rx_bytes", &{ self.rx_bytes })
            .field("tx_bytes", &{ self.tx_bytes })
            .field("rx_errors", &{ self.rx_errors })
            .field("tx_errors", &{ self.tx_errors })
            .field("rx_dropped", &{ self.rx_dropped })
            .field("tx_dropped", &{ self.tx_dropped })
            .finish()
    }
}
#[derive(Clone)]
pub enum VportOptions {
    DstPort(u32),
    Extension(u32),
}
impl<'a> IterableVportOptions<'a> {
    pub fn get_dst_port(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VportOptions::DstPort(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VportOptions",
            "DstPort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_extension(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(VportOptions::Extension(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "VportOptions",
            "Extension",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl VportOptions {
    pub fn new<'a>(buf: &'a [u8]) -> IterableVportOptions<'a> {
        IterableVportOptions::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "DstPort",
            2u16 => "Extension",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableVportOptions<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableVportOptions<'a> {
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
impl<'a> Iterator for IterableVportOptions<'a> {
    type Item = Result<VportOptions, ErrorContext>;
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
                1u16 => VportOptions::DstPort({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => VportOptions::Extension({
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
            "VportOptions",
            r#type.and_then(|t| VportOptions::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableVportOptions<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("VportOptions");
        let mut iter = IterableVportOptions::with_loc(&[], self.orig_loc);
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
                VportOptions::DstPort(val) => fmt.field("DstPort", &val),
                VportOptions::Extension(val) => fmt.field("Extension", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableVportOptions<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("VportOptions", offset));
            return (
                stack,
                missing_type.and_then(|t| VportOptions::attr_from_type(t)),
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
                VportOptions::DstPort(val) => {
                    if last_off == offset {
                        stack.push(("DstPort", last_off));
                        break;
                    }
                }
                VportOptions::Extension(val) => {
                    if last_off == offset {
                        stack.push(("Extension", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("VportOptions", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum UpcallStats {
    Success(u64),
    Fail(u64),
}
impl<'a> IterableUpcallStats<'a> {
    pub fn get_success(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(UpcallStats::Success(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "UpcallStats",
            "Success",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fail(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(UpcallStats::Fail(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "UpcallStats",
            "Fail",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl UpcallStats {
    pub fn new<'a>(buf: &'a [u8]) -> IterableUpcallStats<'a> {
        IterableUpcallStats::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "Success",
            1u16 => "Fail",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableUpcallStats<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableUpcallStats<'a> {
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
impl<'a> Iterator for IterableUpcallStats<'a> {
    type Item = Result<UpcallStats, ErrorContext>;
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
                0u16 => UpcallStats::Success({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                1u16 => UpcallStats::Fail({
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
            "UpcallStats",
            r#type.and_then(|t| UpcallStats::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableUpcallStats<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("UpcallStats");
        let mut iter = IterableUpcallStats::with_loc(&[], self.orig_loc);
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
                UpcallStats::Success(val) => fmt.field("Success", &val),
                UpcallStats::Fail(val) => fmt.field("Fail", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableUpcallStats<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("UpcallStats", offset));
            return (
                stack,
                missing_type.and_then(|t| UpcallStats::attr_from_type(t)),
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
                UpcallStats::Success(val) => {
                    if last_off == offset {
                        stack.push(("Success", last_off));
                        break;
                    }
                }
                UpcallStats::Fail(val) => {
                    if last_off == offset {
                        stack.push(("Fail", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("UpcallStats", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum Vport<'a> {
    PortNo(u32),
    #[doc = "Associated type: [`VportType`] (enum)"]
    Type(u32),
    Name(&'a CStr),
    Options(IterableVportOptions<'a>),
    UpcallPid(&'a [u8]),
    Stats(OvsVportStats),
    Ifindex(u32),
    Netnsid(u32),
    UpcallStats(IterableUpcallStats<'a>),
}
impl<'a> IterableVport<'a> {
    pub fn get_port_no(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Vport::PortNo(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Vport",
            "PortNo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Associated type: [`VportType`] (enum)"]
    pub fn get_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Vport::Type(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Vport",
            "Type",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Vport::Name(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Vport",
            "Name",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_options(&self) -> Result<IterableVportOptions<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Vport::Options(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Vport",
            "Options",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_upcall_pid(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Vport::UpcallPid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Vport",
            "UpcallPid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_stats(&self) -> Result<OvsVportStats, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Vport::Stats(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Vport",
            "Stats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Vport::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Vport",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_netnsid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Vport::Netnsid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Vport",
            "Netnsid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_upcall_stats(&self) -> Result<IterableUpcallStats<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Vport::UpcallStats(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Vport",
            "UpcallStats",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Vport<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableVport<'a> {
        IterableVport::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "Unspec",
            1u16 => "PortNo",
            2u16 => "Type",
            3u16 => "Name",
            4u16 => "Options",
            5u16 => "UpcallPid",
            6u16 => "Stats",
            7u16 => "Pad",
            8u16 => "Ifindex",
            9u16 => "Netnsid",
            10u16 => "UpcallStats",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableVport<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableVport<'a> {
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
impl<'a> Iterator for IterableVport<'a> {
    type Item = Result<Vport<'a>, ErrorContext>;
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
                1u16 => Vport::PortNo({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Vport::Type({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Vport::Name({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Vport::Options({
                    let res = Some(IterableVportOptions::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Vport::UpcallPid({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Vport::Stats({
                    let res = Some(OvsVportStats::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => Vport::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => Vport::Netnsid({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => Vport::UpcallStats({
                    let res = Some(IterableUpcallStats::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Vport",
            r#type.and_then(|t| Vport::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableVport<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Vport");
        let mut iter = IterableVport::with_loc(&[], self.orig_loc);
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
                Vport::PortNo(val) => fmt.field("PortNo", &val),
                Vport::Type(val) => {
                    fmt.field("Type", &FormatEnum(val.into(), VportType::from_value))
                }
                Vport::Name(val) => fmt.field("Name", &val),
                Vport::Options(val) => fmt.field("Options", &val),
                Vport::UpcallPid(val) => fmt.field("UpcallPid", &FormatHexdump(val)),
                Vport::Stats(val) => fmt.field("Stats", &val),
                Vport::Ifindex(val) => fmt.field("Ifindex", &val),
                Vport::Netnsid(val) => fmt.field("Netnsid", &val),
                Vport::UpcallStats(val) => fmt.field("UpcallStats", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableVport<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Vport", offset));
            return (stack, missing_type.and_then(|t| Vport::attr_from_type(t)));
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
                Vport::PortNo(val) => {
                    if last_off == offset {
                        stack.push(("PortNo", last_off));
                        break;
                    }
                }
                Vport::Type(val) => {
                    if last_off == offset {
                        stack.push(("Type", last_off));
                        break;
                    }
                }
                Vport::Name(val) => {
                    if last_off == offset {
                        stack.push(("Name", last_off));
                        break;
                    }
                }
                Vport::Options(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Vport::UpcallPid(val) => {
                    if last_off == offset {
                        stack.push(("UpcallPid", last_off));
                        break;
                    }
                }
                Vport::Stats(val) => {
                    if last_off == offset {
                        stack.push(("Stats", last_off));
                        break;
                    }
                }
                Vport::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                Vport::Netnsid(val) => {
                    if last_off == offset {
                        stack.push(("Netnsid", last_off));
                        break;
                    }
                }
                Vport::UpcallStats(val) => {
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
            stack.push(("Vport", cur));
        }
        (stack, missing)
    }
}
pub struct PushVportOptions<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushVportOptions<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushVportOptions<Prev> {
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
    pub fn push_dst_port(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_extension(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushVportOptions<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushUpcallStats<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushUpcallStats<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushUpcallStats<Prev> {
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
    pub fn push_success(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 0u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fail(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 1u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushUpcallStats<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushVport<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushVport<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushVport<Prev> {
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
    pub fn push_port_no(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Associated type: [`VportType`] (enum)"]
    pub fn push_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            3u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    pub fn nested_options(mut self) -> PushVportOptions<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 4u16);
        PushVportOptions {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_upcall_pid(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 5u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_stats(mut self, value: OvsVportStats) -> Self {
        push_header(self.as_vec_mut(), 6u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_netnsid(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_upcall_stats(mut self) -> PushUpcallStats<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 10u16);
        PushUpcallStats {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushVport<Prev> {
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
    pub const OVS_VPORT: &str = "ovs_vport";
    pub const OVS_VPORT_CSTR: &CStr = c"ovs_vport";
}
#[doc = "Create a new OVS vport\n\nRequest attributes:\n- [.push_type()](PushVport::push_type)\n- [.push_name()](PushVport::push_name)\n- [.nested_options()](PushVport::nested_options)\n- [.push_upcall_pid()](PushVport::push_upcall_pid)\n- [.push_ifindex()](PushVport::push_ifindex)\n\n"]
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
    ) -> PushVport<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushVport::new(buf)
    }
    pub fn encode(&mut self) -> PushVport<&mut Vec<u8>> {
        PushVport::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushVport<RequestBuf<'r>> {
        PushVport::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (OvsHeader, IterableVport<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(OvsHeader::len()));
        (
            OvsHeader::new_from_slice(header).unwrap_or_default(),
            IterableVport::with_loc(attrs, buf.as_ptr() as usize),
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
        Protocol::Generic("ovs_vport".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (OvsHeader, IterableVport<'buf>);
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
#[doc = "Delete existing OVS vport from a data path\n\nRequest attributes:\n- [.push_port_no()](PushVport::push_port_no)\n- [.push_type()](PushVport::push_type)\n- [.push_name()](PushVport::push_name)\n\n"]
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
    ) -> PushVport<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushVport::new(buf)
    }
    pub fn encode(&mut self) -> PushVport<&mut Vec<u8>> {
        PushVport::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushVport<RequestBuf<'r>> {
        PushVport::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (OvsHeader, IterableVport<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(OvsHeader::len()));
        (
            OvsHeader::new_from_slice(header).unwrap_or_default(),
            IterableVport::with_loc(attrs, buf.as_ptr() as usize),
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
        Protocol::Generic("ovs_vport".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (OvsHeader, IterableVport<'buf>);
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
#[doc = "Get / dump OVS vport configuration and state\n\nRequest attributes:\n- [.push_name()](PushVport::push_name)\n\nReply attributes:\n- [.get_port_no()](IterableVport::get_port_no)\n- [.get_type()](IterableVport::get_type)\n- [.get_name()](IterableVport::get_name)\n- [.get_upcall_pid()](IterableVport::get_upcall_pid)\n- [.get_stats()](IterableVport::get_stats)\n- [.get_ifindex()](IterableVport::get_ifindex)\n- [.get_netnsid()](IterableVport::get_netnsid)\n- [.get_upcall_stats()](IterableVport::get_upcall_stats)\n\n"]
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
    ) -> PushVport<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushVport::new(buf)
    }
    pub fn encode(&mut self) -> PushVport<&mut Vec<u8>> {
        PushVport::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushVport<RequestBuf<'r>> {
        PushVport::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (OvsHeader, IterableVport<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(OvsHeader::len()));
        (
            OvsHeader::new_from_slice(header).unwrap_or_default(),
            IterableVport::with_loc(attrs, buf.as_ptr() as usize),
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
        Protocol::Generic("ovs_vport".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (OvsHeader, IterableVport<'buf>);
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
#[doc = "Get / dump OVS vport configuration and state\n\nRequest attributes:\n- [.push_name()](PushVport::push_name)\n\nReply attributes:\n- [.get_port_no()](IterableVport::get_port_no)\n- [.get_type()](IterableVport::get_type)\n- [.get_name()](IterableVport::get_name)\n- [.get_upcall_pid()](IterableVport::get_upcall_pid)\n- [.get_stats()](IterableVport::get_stats)\n- [.get_ifindex()](IterableVport::get_ifindex)\n- [.get_netnsid()](IterableVport::get_netnsid)\n- [.get_upcall_stats()](IterableVport::get_upcall_stats)\n\n"]
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
    ) -> PushVport<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushVport::new(buf)
    }
    pub fn encode(&mut self) -> PushVport<&mut Vec<u8>> {
        PushVport::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushVport<RequestBuf<'r>> {
        PushVport::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (OvsHeader, IterableVport<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(OvsHeader::len()));
        (
            OvsHeader::new_from_slice(header).unwrap_or_default(),
            IterableVport::with_loc(attrs, buf.as_ptr() as usize),
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
        Protocol::Generic("ovs_vport".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (OvsHeader, IterableVport<'buf>);
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
    #[doc = "Create a new OVS vport\n\nRequest attributes:\n- [.push_type()](PushVport::push_type)\n- [.push_name()](PushVport::push_name)\n- [.nested_options()](PushVport::nested_options)\n- [.push_upcall_pid()](PushVport::push_upcall_pid)\n- [.push_ifindex()](PushVport::push_ifindex)\n\n"]
    pub fn op_new_do(self, header: &OvsHeader) -> OpNewDo<'buf> {
        let mut res = OpNewDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-new-do", OpNewDo::lookup);
        res
    }
    #[doc = "Delete existing OVS vport from a data path\n\nRequest attributes:\n- [.push_port_no()](PushVport::push_port_no)\n- [.push_type()](PushVport::push_type)\n- [.push_name()](PushVport::push_name)\n\n"]
    pub fn op_del_do(self, header: &OvsHeader) -> OpDelDo<'buf> {
        let mut res = OpDelDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-del-do", OpDelDo::lookup);
        res
    }
    #[doc = "Get / dump OVS vport configuration and state\n\nRequest attributes:\n- [.push_name()](PushVport::push_name)\n\nReply attributes:\n- [.get_port_no()](IterableVport::get_port_no)\n- [.get_type()](IterableVport::get_type)\n- [.get_name()](IterableVport::get_name)\n- [.get_upcall_pid()](IterableVport::get_upcall_pid)\n- [.get_stats()](IterableVport::get_stats)\n- [.get_ifindex()](IterableVport::get_ifindex)\n- [.get_netnsid()](IterableVport::get_netnsid)\n- [.get_upcall_stats()](IterableVport::get_upcall_stats)\n\n"]
    pub fn op_get_dump(self, header: &OvsHeader) -> OpGetDump<'buf> {
        let mut res = OpGetDump::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-get-dump", OpGetDump::lookup);
        res
    }
    #[doc = "Get / dump OVS vport configuration and state\n\nRequest attributes:\n- [.push_name()](PushVport::push_name)\n\nReply attributes:\n- [.get_port_no()](IterableVport::get_port_no)\n- [.get_type()](IterableVport::get_type)\n- [.get_name()](IterableVport::get_name)\n- [.get_upcall_pid()](IterableVport::get_upcall_pid)\n- [.get_stats()](IterableVport::get_stats)\n- [.get_ifindex()](IterableVport::get_ifindex)\n- [.get_netnsid()](IterableVport::get_netnsid)\n- [.get_upcall_stats()](IterableVport::get_upcall_stats)\n\n"]
    pub fn op_get_do(self, header: &OvsHeader) -> OpGetDo<'buf> {
        let mut res = OpGetDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-get-do", OpGetDo::lookup);
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableVport::get_ifindex;
        let _ = IterableVport::get_name;
        let _ = IterableVport::get_netnsid;
        let _ = IterableVport::get_port_no;
        let _ = IterableVport::get_stats;
        let _ = IterableVport::get_type;
        let _ = IterableVport::get_upcall_pid;
        let _ = IterableVport::get_upcall_stats;
        let _ = PushVport::<&mut Vec<u8>>::nested_options;
        let _ = PushVport::<&mut Vec<u8>>::push_ifindex;
        let _ = PushVport::<&mut Vec<u8>>::push_name;
        let _ = PushVport::<&mut Vec<u8>>::push_port_no;
        let _ = PushVport::<&mut Vec<u8>>::push_type;
        let _ = PushVport::<&mut Vec<u8>>::push_upcall_pid;
    }
}
