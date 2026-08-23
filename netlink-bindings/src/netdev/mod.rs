#![doc = "netdev configuration over generic netlink.\n"]
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
pub const PROTONAME: &str = "netdev";
pub const PROTONAME_CSTR: &CStr = c"netdev";
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum XdpAct {
    #[doc = "XDP features set supported by all drivers (XDP_ABORTED, XDP_DROP,\nXDP_PASS, XDP_TX)\n"]
    Basic = 1 << 0,
    #[doc = "The netdev supports XDP_REDIRECT\n"]
    Redirect = 1 << 1,
    #[doc = "This feature informs if netdev implements ndo_xdp_xmit callback.\n"]
    NdoXmit = 1 << 2,
    #[doc = "This feature informs if netdev supports AF_XDP in zero copy mode.\n"]
    XskZerocopy = 1 << 3,
    #[doc = "This feature informs if netdev supports XDP hw offloading.\n"]
    HwOffload = 1 << 4,
    #[doc = "This feature informs if netdev implements non-linear XDP buffer support\nin the driver napi callback.\n"]
    RxSg = 1 << 5,
    #[doc = "This feature informs if netdev implements non-linear XDP buffer support\nin ndo_xdp_xmit callback.\n"]
    NdoXmitSg = 1 << 6,
}
impl XdpAct {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::Basic,
            n if n == 1 << 1 => Self::Redirect,
            n if n == 1 << 2 => Self::NdoXmit,
            n if n == 1 << 3 => Self::XskZerocopy,
            n if n == 1 << 4 => Self::HwOffload,
            n if n == 1 << 5 => Self::RxSg,
            n if n == 1 << 6 => Self::NdoXmitSg,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum XdpRxMetadata {
    #[doc = "Device is capable of exposing receive HW timestamp via\nbpf_xdp_metadata_rx_timestamp().\n"]
    Timestamp = 1 << 0,
    #[doc = "Device is capable of exposing receive packet hash via\nbpf_xdp_metadata_rx_hash().\n"]
    Hash = 1 << 1,
    #[doc = "Device is capable of exposing receive packet VLAN tag via\nbpf_xdp_metadata_rx_vlan_tag().\n"]
    VlanTag = 1 << 2,
}
impl XdpRxMetadata {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::Timestamp,
            n if n == 1 << 1 => Self::Hash,
            n if n == 1 << 2 => Self::VlanTag,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum XskFlags {
    #[doc = "HW timestamping egress packets is supported by the driver.\n"]
    TxTimestamp = 1 << 0,
    #[doc = "L3 checksum HW offload is supported by the driver.\n"]
    TxChecksum = 1 << 1,
    #[doc = "Launch time HW offload is supported by the driver.\n"]
    TxLaunchTimeFifo = 1 << 2,
}
impl XskFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::TxTimestamp,
            n if n == 1 << 1 => Self::TxChecksum,
            n if n == 1 << 2 => Self::TxLaunchTimeFifo,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum QueueType {
    Rx = 0,
    Tx = 1,
}
impl QueueType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Rx,
            1 => Self::Tx,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum QstatsScope {
    Queue = 1 << 0,
}
impl QstatsScope {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::Queue,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum NapiThreaded {
    Disabled = 0,
    Enabled = 1,
    BusyPoll = 2,
}
impl NapiThreaded {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Disabled,
            1 => Self::Enabled,
            2 => Self::BusyPoll,
            _ => return None,
        })
    }
}
#[derive(Clone)]
pub enum Dev<'a> {
    #[doc = "netdev ifindex\n"]
    Ifindex(u32),
    Pad(&'a [u8]),
    #[doc = "Bitmask of enabled xdp-features.\n\nAssociated type: [`XdpAct`] (enum)"]
    XdpFeatures(u64),
    #[doc = "max fragment count supported by ZC driver\n"]
    XdpZcMaxSegs(u32),
    #[doc = "Bitmask of supported XDP receive metadata features. See\nDocumentation/networking/xdp-rx-metadata.rst for more details.\n\nAssociated type: [`XdpRxMetadata`] (enum)"]
    XdpRxMetadataFeatures(u64),
    #[doc = "Bitmask of enabled AF_XDP features.\n\nAssociated type: [`XskFlags`] (enum)"]
    XskFeatures(u64),
}
impl<'a> IterableDev<'a> {
    #[doc = "netdev ifindex\n"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dev::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dev",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dev::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dev",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Bitmask of enabled xdp-features.\n\nAssociated type: [`XdpAct`] (enum)"]
    pub fn get_xdp_features(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dev::XdpFeatures(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dev",
            "XdpFeatures",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "max fragment count supported by ZC driver\n"]
    pub fn get_xdp_zc_max_segs(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dev::XdpZcMaxSegs(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dev",
            "XdpZcMaxSegs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Bitmask of supported XDP receive metadata features. See\nDocumentation/networking/xdp-rx-metadata.rst for more details.\n\nAssociated type: [`XdpRxMetadata`] (enum)"]
    pub fn get_xdp_rx_metadata_features(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dev::XdpRxMetadataFeatures(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dev",
            "XdpRxMetadataFeatures",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Bitmask of enabled AF_XDP features.\n\nAssociated type: [`XskFlags`] (enum)"]
    pub fn get_xsk_features(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dev::XskFeatures(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dev",
            "XskFeatures",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Dev<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDev<'a> {
        IterableDev::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Ifindex",
            2u16 => "Pad",
            3u16 => "XdpFeatures",
            4u16 => "XdpZcMaxSegs",
            5u16 => "XdpRxMetadataFeatures",
            6u16 => "XskFeatures",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDev<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDev<'a> {
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
impl<'a> Iterator for IterableDev<'a> {
    type Item = Result<Dev<'a>, ErrorContext>;
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
                1u16 => Dev::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Dev::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Dev::XdpFeatures({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Dev::XdpZcMaxSegs({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Dev::XdpRxMetadataFeatures({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Dev::XskFeatures({
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
            "Dev",
            r#type.and_then(|t| Dev::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDev<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Dev");
        let mut iter = IterableDev::with_loc(&[], self.orig_loc);
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
                Dev::Ifindex(val) => fmt.field("Ifindex", &val),
                Dev::Pad(val) => fmt.field("Pad", &val),
                Dev::XdpFeatures(val) => {
                    fmt.field("XdpFeatures", &FormatFlags(val.into(), XdpAct::from_value))
                }
                Dev::XdpZcMaxSegs(val) => fmt.field("XdpZcMaxSegs", &val),
                Dev::XdpRxMetadataFeatures(val) => fmt.field(
                    "XdpRxMetadataFeatures",
                    &FormatFlags(val.into(), XdpRxMetadata::from_value),
                ),
                Dev::XskFeatures(val) => fmt.field(
                    "XskFeatures",
                    &FormatFlags(val.into(), XskFlags::from_value),
                ),
            };
        }
        fmt.finish()
    }
}
impl IterableDev<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Dev", offset));
            return (stack, missing_type.and_then(|t| Dev::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Dev::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                Dev::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                Dev::XdpFeatures(val) => {
                    if last_off == offset {
                        stack.push(("XdpFeatures", last_off));
                        break;
                    }
                }
                Dev::XdpZcMaxSegs(val) => {
                    if last_off == offset {
                        stack.push(("XdpZcMaxSegs", last_off));
                        break;
                    }
                }
                Dev::XdpRxMetadataFeatures(val) => {
                    if last_off == offset {
                        stack.push(("XdpRxMetadataFeatures", last_off));
                        break;
                    }
                }
                Dev::XskFeatures(val) => {
                    if last_off == offset {
                        stack.push(("XskFeatures", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Dev", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum IoUringProviderInfo {
    #[doc = "RX buffer length in bytes for this io_uring memory provider. Reflects\nthe rx_buf_len passed at io_uring zerocopy rx registration time.\n"]
    RxBufLen(u32),
}
impl<'a> IterableIoUringProviderInfo<'a> {
    #[doc = "RX buffer length in bytes for this io_uring memory provider. Reflects\nthe rx_buf_len passed at io_uring zerocopy rx registration time.\n"]
    pub fn get_rx_buf_len(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(IoUringProviderInfo::RxBufLen(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "IoUringProviderInfo",
            "RxBufLen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl IoUringProviderInfo {
    pub fn new<'a>(buf: &'a [u8]) -> IterableIoUringProviderInfo<'a> {
        IterableIoUringProviderInfo::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "RxBufLen",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableIoUringProviderInfo<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableIoUringProviderInfo<'a> {
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
impl<'a> Iterator for IterableIoUringProviderInfo<'a> {
    type Item = Result<IoUringProviderInfo, ErrorContext>;
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
                1u16 => IoUringProviderInfo::RxBufLen({
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
            "IoUringProviderInfo",
            r#type.and_then(|t| IoUringProviderInfo::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableIoUringProviderInfo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("IoUringProviderInfo");
        let mut iter = IterableIoUringProviderInfo::with_loc(&[], self.orig_loc);
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
                IoUringProviderInfo::RxBufLen(val) => fmt.field("RxBufLen", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableIoUringProviderInfo<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("IoUringProviderInfo", offset));
            return (
                stack,
                missing_type.and_then(|t| IoUringProviderInfo::attr_from_type(t)),
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
                IoUringProviderInfo::RxBufLen(val) => {
                    if last_off == offset {
                        stack.push(("RxBufLen", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("IoUringProviderInfo", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum PagePool<'a> {
    #[doc = "Unique ID of a Page Pool instance.\n"]
    Id(u32),
    #[doc = "ifindex of the netdev to which the pool belongs. May not be reported if\nthe page pool was allocated for a netdev which got destroyed already\n(page pools may outlast their netdevs because they wait for all memory\nto be returned).\n"]
    Ifindex(u32),
    #[doc = "Id of NAPI using this Page Pool instance.\n"]
    NapiId(u32),
    #[doc = "Number of outstanding references to this page pool (allocated but yet to\nbe freed pages). Allocated pages may be held in socket receive queues,\ndriver receive ring, page pool recycling ring, the page pool cache, etc.\n"]
    Inflight(u32),
    #[doc = "Amount of memory held by inflight pages.\n"]
    InflightMem(u32),
    #[doc = "Seconds in CLOCK_BOOTTIME of when Page Pool was detached by the driver.\nOnce detached Page Pool can no longer be used to allocate memory. Page\nPools wait for all the memory allocated from them to be freed before\ntruly disappearing. \\\"Detached\\\" Page Pools cannot be \\\"re-attached\\\",\nthey are just waiting to disappear. Attribute is absent if Page Pool has\nnot been detached, and can still be used to allocate new memory.\n"]
    DetachTime(u32),
    #[doc = "ID of the dmabuf this page-pool is attached to.\n"]
    Dmabuf(u32),
    #[doc = "io-uring memory provider information.\n"]
    IoUring(IterableIoUringProviderInfo<'a>),
}
impl<'a> IterablePagePool<'a> {
    #[doc = "Unique ID of a Page Pool instance.\n"]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PagePool::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PagePool",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ifindex of the netdev to which the pool belongs. May not be reported if\nthe page pool was allocated for a netdev which got destroyed already\n(page pools may outlast their netdevs because they wait for all memory\nto be returned).\n"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PagePool::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PagePool",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Id of NAPI using this Page Pool instance.\n"]
    pub fn get_napi_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PagePool::NapiId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PagePool",
            "NapiId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of outstanding references to this page pool (allocated but yet to\nbe freed pages). Allocated pages may be held in socket receive queues,\ndriver receive ring, page pool recycling ring, the page pool cache, etc.\n"]
    pub fn get_inflight(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PagePool::Inflight(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PagePool",
            "Inflight",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Amount of memory held by inflight pages.\n"]
    pub fn get_inflight_mem(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PagePool::InflightMem(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PagePool",
            "InflightMem",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Seconds in CLOCK_BOOTTIME of when Page Pool was detached by the driver.\nOnce detached Page Pool can no longer be used to allocate memory. Page\nPools wait for all the memory allocated from them to be freed before\ntruly disappearing. \\\"Detached\\\" Page Pools cannot be \\\"re-attached\\\",\nthey are just waiting to disappear. Attribute is absent if Page Pool has\nnot been detached, and can still be used to allocate new memory.\n"]
    pub fn get_detach_time(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PagePool::DetachTime(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PagePool",
            "DetachTime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ID of the dmabuf this page-pool is attached to.\n"]
    pub fn get_dmabuf(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PagePool::Dmabuf(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PagePool",
            "Dmabuf",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "io-uring memory provider information.\n"]
    pub fn get_io_uring(&self) -> Result<IterableIoUringProviderInfo<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PagePool::IoUring(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PagePool",
            "IoUring",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl PagePool<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePagePool<'a> {
        IterablePagePool::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Id",
            2u16 => "Ifindex",
            3u16 => "NapiId",
            4u16 => "Inflight",
            5u16 => "InflightMem",
            6u16 => "DetachTime",
            7u16 => "Dmabuf",
            8u16 => "IoUring",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePagePool<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePagePool<'a> {
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
impl<'a> Iterator for IterablePagePool<'a> {
    type Item = Result<PagePool<'a>, ErrorContext>;
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
                1u16 => PagePool::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => PagePool::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => PagePool::NapiId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => PagePool::Inflight({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => PagePool::InflightMem({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => PagePool::DetachTime({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => PagePool::Dmabuf({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => PagePool::IoUring({
                    let res = Some(IterableIoUringProviderInfo::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "PagePool",
            r#type.and_then(|t| PagePool::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterablePagePool<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("PagePool");
        let mut iter = IterablePagePool::with_loc(&[], self.orig_loc);
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
                PagePool::Id(val) => fmt.field("Id", &val),
                PagePool::Ifindex(val) => fmt.field("Ifindex", &val),
                PagePool::NapiId(val) => fmt.field("NapiId", &val),
                PagePool::Inflight(val) => fmt.field("Inflight", &val),
                PagePool::InflightMem(val) => fmt.field("InflightMem", &val),
                PagePool::DetachTime(val) => fmt.field("DetachTime", &val),
                PagePool::Dmabuf(val) => fmt.field("Dmabuf", &val),
                PagePool::IoUring(val) => fmt.field("IoUring", &val),
            };
        }
        fmt.finish()
    }
}
impl IterablePagePool<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("PagePool", offset));
            return (
                stack,
                missing_type.and_then(|t| PagePool::attr_from_type(t)),
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
                PagePool::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                PagePool::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                PagePool::NapiId(val) => {
                    if last_off == offset {
                        stack.push(("NapiId", last_off));
                        break;
                    }
                }
                PagePool::Inflight(val) => {
                    if last_off == offset {
                        stack.push(("Inflight", last_off));
                        break;
                    }
                }
                PagePool::InflightMem(val) => {
                    if last_off == offset {
                        stack.push(("InflightMem", last_off));
                        break;
                    }
                }
                PagePool::DetachTime(val) => {
                    if last_off == offset {
                        stack.push(("DetachTime", last_off));
                        break;
                    }
                }
                PagePool::Dmabuf(val) => {
                    if last_off == offset {
                        stack.push(("Dmabuf", last_off));
                        break;
                    }
                }
                PagePool::IoUring(val) => {
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
            stack.push(("PagePool", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum PagePoolInfo {
    #[doc = "Unique ID of a Page Pool instance.\n"]
    Id(u32),
    #[doc = "ifindex of the netdev to which the pool belongs. May not be reported if\nthe page pool was allocated for a netdev which got destroyed already\n(page pools may outlast their netdevs because they wait for all memory\nto be returned).\n"]
    Ifindex(u32),
}
impl<'a> IterablePagePoolInfo<'a> {
    #[doc = "Unique ID of a Page Pool instance.\n"]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PagePoolInfo::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PagePoolInfo",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ifindex of the netdev to which the pool belongs. May not be reported if\nthe page pool was allocated for a netdev which got destroyed already\n(page pools may outlast their netdevs because they wait for all memory\nto be returned).\n"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PagePoolInfo::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PagePoolInfo",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl PagePoolInfo {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePagePoolInfo<'a> {
        IterablePagePoolInfo::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        PagePool::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePagePoolInfo<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePagePoolInfo<'a> {
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
impl<'a> Iterator for IterablePagePoolInfo<'a> {
    type Item = Result<PagePoolInfo, ErrorContext>;
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
                1u16 => PagePoolInfo::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => PagePoolInfo::Ifindex({
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
            "PagePoolInfo",
            r#type.and_then(|t| PagePoolInfo::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterablePagePoolInfo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("PagePoolInfo");
        let mut iter = IterablePagePoolInfo::with_loc(&[], self.orig_loc);
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
                PagePoolInfo::Id(val) => fmt.field("Id", &val),
                PagePoolInfo::Ifindex(val) => fmt.field("Ifindex", &val),
            };
        }
        fmt.finish()
    }
}
impl IterablePagePoolInfo<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("PagePoolInfo", offset));
            return (
                stack,
                missing_type.and_then(|t| PagePoolInfo::attr_from_type(t)),
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
                PagePoolInfo::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                PagePoolInfo::Ifindex(val) => {
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
            stack.push(("PagePoolInfo", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum PagePoolStats<'a> {
    #[doc = "Page pool identifying information.\n"]
    Info(IterablePagePoolInfo<'a>),
    AllocFast(u32),
    AllocSlow(u32),
    AllocSlowHighOrder(u32),
    AllocEmpty(u32),
    AllocRefill(u32),
    AllocWaive(u32),
    RecycleCached(u32),
    RecycleCacheFull(u32),
    RecycleRing(u32),
    RecycleRingFull(u32),
    RecycleReleasedRefcnt(u32),
}
impl<'a> IterablePagePoolStats<'a> {
    #[doc = "Page pool identifying information.\n"]
    pub fn get_info(&self) -> Result<IterablePagePoolInfo<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PagePoolStats::Info(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PagePoolStats",
            "Info",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_alloc_fast(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PagePoolStats::AllocFast(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PagePoolStats",
            "AllocFast",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_alloc_slow(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PagePoolStats::AllocSlow(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PagePoolStats",
            "AllocSlow",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_alloc_slow_high_order(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PagePoolStats::AllocSlowHighOrder(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PagePoolStats",
            "AllocSlowHighOrder",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_alloc_empty(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PagePoolStats::AllocEmpty(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PagePoolStats",
            "AllocEmpty",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_alloc_refill(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PagePoolStats::AllocRefill(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PagePoolStats",
            "AllocRefill",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_alloc_waive(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PagePoolStats::AllocWaive(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PagePoolStats",
            "AllocWaive",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_recycle_cached(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PagePoolStats::RecycleCached(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PagePoolStats",
            "RecycleCached",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_recycle_cache_full(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PagePoolStats::RecycleCacheFull(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PagePoolStats",
            "RecycleCacheFull",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_recycle_ring(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PagePoolStats::RecycleRing(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PagePoolStats",
            "RecycleRing",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_recycle_ring_full(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PagePoolStats::RecycleRingFull(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PagePoolStats",
            "RecycleRingFull",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_recycle_released_refcnt(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PagePoolStats::RecycleReleasedRefcnt(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PagePoolStats",
            "RecycleReleasedRefcnt",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl PagePoolStats<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePagePoolStats<'a> {
        IterablePagePoolStats::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Info",
            8u16 => "AllocFast",
            9u16 => "AllocSlow",
            10u16 => "AllocSlowHighOrder",
            11u16 => "AllocEmpty",
            12u16 => "AllocRefill",
            13u16 => "AllocWaive",
            14u16 => "RecycleCached",
            15u16 => "RecycleCacheFull",
            16u16 => "RecycleRing",
            17u16 => "RecycleRingFull",
            18u16 => "RecycleReleasedRefcnt",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePagePoolStats<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePagePoolStats<'a> {
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
impl<'a> Iterator for IterablePagePoolStats<'a> {
    type Item = Result<PagePoolStats<'a>, ErrorContext>;
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
                1u16 => PagePoolStats::Info({
                    let res = Some(IterablePagePoolInfo::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => PagePoolStats::AllocFast({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => PagePoolStats::AllocSlow({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => PagePoolStats::AllocSlowHighOrder({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => PagePoolStats::AllocEmpty({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => PagePoolStats::AllocRefill({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => PagePoolStats::AllocWaive({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => PagePoolStats::RecycleCached({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => PagePoolStats::RecycleCacheFull({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => PagePoolStats::RecycleRing({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => PagePoolStats::RecycleRingFull({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => PagePoolStats::RecycleReleasedRefcnt({
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
            "PagePoolStats",
            r#type.and_then(|t| PagePoolStats::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterablePagePoolStats<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("PagePoolStats");
        let mut iter = IterablePagePoolStats::with_loc(&[], self.orig_loc);
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
                PagePoolStats::Info(val) => fmt.field("Info", &val),
                PagePoolStats::AllocFast(val) => fmt.field("AllocFast", &val),
                PagePoolStats::AllocSlow(val) => fmt.field("AllocSlow", &val),
                PagePoolStats::AllocSlowHighOrder(val) => fmt.field("AllocSlowHighOrder", &val),
                PagePoolStats::AllocEmpty(val) => fmt.field("AllocEmpty", &val),
                PagePoolStats::AllocRefill(val) => fmt.field("AllocRefill", &val),
                PagePoolStats::AllocWaive(val) => fmt.field("AllocWaive", &val),
                PagePoolStats::RecycleCached(val) => fmt.field("RecycleCached", &val),
                PagePoolStats::RecycleCacheFull(val) => fmt.field("RecycleCacheFull", &val),
                PagePoolStats::RecycleRing(val) => fmt.field("RecycleRing", &val),
                PagePoolStats::RecycleRingFull(val) => fmt.field("RecycleRingFull", &val),
                PagePoolStats::RecycleReleasedRefcnt(val) => {
                    fmt.field("RecycleReleasedRefcnt", &val)
                }
            };
        }
        fmt.finish()
    }
}
impl IterablePagePoolStats<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("PagePoolStats", offset));
            return (
                stack,
                missing_type.and_then(|t| PagePoolStats::attr_from_type(t)),
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
                PagePoolStats::Info(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                PagePoolStats::AllocFast(val) => {
                    if last_off == offset {
                        stack.push(("AllocFast", last_off));
                        break;
                    }
                }
                PagePoolStats::AllocSlow(val) => {
                    if last_off == offset {
                        stack.push(("AllocSlow", last_off));
                        break;
                    }
                }
                PagePoolStats::AllocSlowHighOrder(val) => {
                    if last_off == offset {
                        stack.push(("AllocSlowHighOrder", last_off));
                        break;
                    }
                }
                PagePoolStats::AllocEmpty(val) => {
                    if last_off == offset {
                        stack.push(("AllocEmpty", last_off));
                        break;
                    }
                }
                PagePoolStats::AllocRefill(val) => {
                    if last_off == offset {
                        stack.push(("AllocRefill", last_off));
                        break;
                    }
                }
                PagePoolStats::AllocWaive(val) => {
                    if last_off == offset {
                        stack.push(("AllocWaive", last_off));
                        break;
                    }
                }
                PagePoolStats::RecycleCached(val) => {
                    if last_off == offset {
                        stack.push(("RecycleCached", last_off));
                        break;
                    }
                }
                PagePoolStats::RecycleCacheFull(val) => {
                    if last_off == offset {
                        stack.push(("RecycleCacheFull", last_off));
                        break;
                    }
                }
                PagePoolStats::RecycleRing(val) => {
                    if last_off == offset {
                        stack.push(("RecycleRing", last_off));
                        break;
                    }
                }
                PagePoolStats::RecycleRingFull(val) => {
                    if last_off == offset {
                        stack.push(("RecycleRingFull", last_off));
                        break;
                    }
                }
                PagePoolStats::RecycleReleasedRefcnt(val) => {
                    if last_off == offset {
                        stack.push(("RecycleReleasedRefcnt", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("PagePoolStats", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum Napi {
    #[doc = "ifindex of the netdevice to which NAPI instance belongs.\n"]
    Ifindex(u32),
    #[doc = "ID of the NAPI instance.\n"]
    Id(u32),
    #[doc = "The associated interrupt vector number for the napi\n"]
    Irq(u32),
    #[doc = "PID of the napi thread, if NAPI is configured to operate in threaded\nmode. If NAPI is not in threaded mode (i.e. uses normal softirq\ncontext), the attribute will be absent.\n"]
    Pid(u32),
    #[doc = "The number of consecutive empty polls before IRQ deferral ends and\nhardware IRQs are re-enabled.\n"]
    DeferHardIrqs(u32),
    #[doc = "The timeout, in nanoseconds, of when to trigger the NAPI watchdog timer\nwhich schedules NAPI processing. Additionally, a non-zero value will\nalso prevent GRO from flushing recent super-frames at the end of a NAPI\ncycle. This may add receive latency in exchange for reducing the number\nof frames processed by the network stack.\n"]
    GroFlushTimeout(u32),
    #[doc = "The timeout, in nanoseconds, of how long to suspend irq processing, if\nevent polling finds events\n"]
    IrqSuspendTimeout(u32),
    #[doc = "Whether the NAPI is configured to operate in threaded polling mode. If\nthis is set to enabled then the NAPI context operates in threaded\npolling mode. If this is set to busy-poll, then the threaded polling\nmode also busy polls.\n\nAssociated type: [`NapiThreaded`] (enum)"]
    Threaded(u32),
}
impl<'a> IterableNapi<'a> {
    #[doc = "ifindex of the netdevice to which NAPI instance belongs.\n"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Napi::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Napi",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ID of the NAPI instance.\n"]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Napi::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Napi",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The associated interrupt vector number for the napi\n"]
    pub fn get_irq(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Napi::Irq(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Napi",
            "Irq",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "PID of the napi thread, if NAPI is configured to operate in threaded\nmode. If NAPI is not in threaded mode (i.e. uses normal softirq\ncontext), the attribute will be absent.\n"]
    pub fn get_pid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Napi::Pid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Napi",
            "Pid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The number of consecutive empty polls before IRQ deferral ends and\nhardware IRQs are re-enabled.\n"]
    pub fn get_defer_hard_irqs(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Napi::DeferHardIrqs(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Napi",
            "DeferHardIrqs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The timeout, in nanoseconds, of when to trigger the NAPI watchdog timer\nwhich schedules NAPI processing. Additionally, a non-zero value will\nalso prevent GRO from flushing recent super-frames at the end of a NAPI\ncycle. This may add receive latency in exchange for reducing the number\nof frames processed by the network stack.\n"]
    pub fn get_gro_flush_timeout(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Napi::GroFlushTimeout(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Napi",
            "GroFlushTimeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The timeout, in nanoseconds, of how long to suspend irq processing, if\nevent polling finds events\n"]
    pub fn get_irq_suspend_timeout(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Napi::IrqSuspendTimeout(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Napi",
            "IrqSuspendTimeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether the NAPI is configured to operate in threaded polling mode. If\nthis is set to enabled then the NAPI context operates in threaded\npolling mode. If this is set to busy-poll, then the threaded polling\nmode also busy polls.\n\nAssociated type: [`NapiThreaded`] (enum)"]
    pub fn get_threaded(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Napi::Threaded(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Napi",
            "Threaded",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Napi {
    pub fn new<'a>(buf: &'a [u8]) -> IterableNapi<'a> {
        IterableNapi::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Ifindex",
            2u16 => "Id",
            3u16 => "Irq",
            4u16 => "Pid",
            5u16 => "DeferHardIrqs",
            6u16 => "GroFlushTimeout",
            7u16 => "IrqSuspendTimeout",
            8u16 => "Threaded",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableNapi<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableNapi<'a> {
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
impl<'a> Iterator for IterableNapi<'a> {
    type Item = Result<Napi, ErrorContext>;
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
                1u16 => Napi::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Napi::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Napi::Irq({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Napi::Pid({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Napi::DeferHardIrqs({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Napi::GroFlushTimeout({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => Napi::IrqSuspendTimeout({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => Napi::Threaded({
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
            "Napi",
            r#type.and_then(|t| Napi::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableNapi<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Napi");
        let mut iter = IterableNapi::with_loc(&[], self.orig_loc);
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
                Napi::Ifindex(val) => fmt.field("Ifindex", &val),
                Napi::Id(val) => fmt.field("Id", &val),
                Napi::Irq(val) => fmt.field("Irq", &val),
                Napi::Pid(val) => fmt.field("Pid", &val),
                Napi::DeferHardIrqs(val) => fmt.field("DeferHardIrqs", &val),
                Napi::GroFlushTimeout(val) => fmt.field("GroFlushTimeout", &val),
                Napi::IrqSuspendTimeout(val) => fmt.field("IrqSuspendTimeout", &val),
                Napi::Threaded(val) => fmt.field(
                    "Threaded",
                    &FormatEnum(val.into(), NapiThreaded::from_value),
                ),
            };
        }
        fmt.finish()
    }
}
impl IterableNapi<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Napi", offset));
            return (stack, missing_type.and_then(|t| Napi::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Napi::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                Napi::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                Napi::Irq(val) => {
                    if last_off == offset {
                        stack.push(("Irq", last_off));
                        break;
                    }
                }
                Napi::Pid(val) => {
                    if last_off == offset {
                        stack.push(("Pid", last_off));
                        break;
                    }
                }
                Napi::DeferHardIrqs(val) => {
                    if last_off == offset {
                        stack.push(("DeferHardIrqs", last_off));
                        break;
                    }
                }
                Napi::GroFlushTimeout(val) => {
                    if last_off == offset {
                        stack.push(("GroFlushTimeout", last_off));
                        break;
                    }
                }
                Napi::IrqSuspendTimeout(val) => {
                    if last_off == offset {
                        stack.push(("IrqSuspendTimeout", last_off));
                        break;
                    }
                }
                Napi::Threaded(val) => {
                    if last_off == offset {
                        stack.push(("Threaded", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Napi", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum XskInfo {}
impl<'a> IterableXskInfo<'a> {}
impl XskInfo {
    pub fn new<'a>(buf: &'a [u8]) -> IterableXskInfo<'a> {
        IterableXskInfo::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        None
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableXskInfo<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableXskInfo<'a> {
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
impl<'a> Iterator for IterableXskInfo<'a> {
    type Item = Result<XskInfo, ErrorContext>;
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
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "XskInfo",
            r#type.and_then(|t| XskInfo::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableXskInfo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("XskInfo");
        let mut iter = IterableXskInfo::with_loc(&[], self.orig_loc);
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
            match attr {};
        }
        fmt.finish()
    }
}
impl IterableXskInfo<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("XskInfo", offset));
            return (stack, missing_type.and_then(|t| XskInfo::attr_from_type(t)));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum Queue<'a> {
    #[doc = "Queue index; most queue types are indexed like a C array, with indexes\nstarting at 0 and ending at queue count - 1. Queue indexes are scoped to\nan interface and queue type.\n"]
    Id(u32),
    #[doc = "ifindex of the netdevice to which the queue belongs.\n"]
    Ifindex(u32),
    #[doc = "Queue type as rx, tx. Each queue type defines a separate ID space. XDP\nTX queues allocated in the kernel are not linked to NAPIs and thus not\nlisted. AF_XDP queues will have more information set in the xsk\nattribute.\n\nAssociated type: [`QueueType`] (enum)"]
    Type(u32),
    #[doc = "ID of the NAPI instance which services this queue.\n"]
    NapiId(u32),
    #[doc = "ID of the dmabuf attached to this queue, if any.\n"]
    Dmabuf(u32),
    #[doc = "io_uring memory provider information.\n"]
    IoUring(IterableIoUringProviderInfo<'a>),
    #[doc = "XSK information for this queue, if any.\n"]
    Xsk(IterableXskInfo<'a>),
    #[doc = "A queue from a virtual device can have a lease which refers to another\nqueue from a physical device. This is useful for memory providers and\nAF_XDP operations which take an ifindex and queue id to allow\napplications to bind against virtual devices in containers.\n"]
    Lease(IterableLease<'a>),
}
impl<'a> IterableQueue<'a> {
    #[doc = "Queue index; most queue types are indexed like a C array, with indexes\nstarting at 0 and ending at queue count - 1. Queue indexes are scoped to\nan interface and queue type.\n"]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Queue::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Queue",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ifindex of the netdevice to which the queue belongs.\n"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Queue::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Queue",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Queue type as rx, tx. Each queue type defines a separate ID space. XDP\nTX queues allocated in the kernel are not linked to NAPIs and thus not\nlisted. AF_XDP queues will have more information set in the xsk\nattribute.\n\nAssociated type: [`QueueType`] (enum)"]
    pub fn get_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Queue::Type(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Queue",
            "Type",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ID of the NAPI instance which services this queue.\n"]
    pub fn get_napi_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Queue::NapiId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Queue",
            "NapiId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ID of the dmabuf attached to this queue, if any.\n"]
    pub fn get_dmabuf(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Queue::Dmabuf(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Queue",
            "Dmabuf",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "io_uring memory provider information.\n"]
    pub fn get_io_uring(&self) -> Result<IterableIoUringProviderInfo<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Queue::IoUring(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Queue",
            "IoUring",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "XSK information for this queue, if any.\n"]
    pub fn get_xsk(&self) -> Result<IterableXskInfo<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Queue::Xsk(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Queue",
            "Xsk",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "A queue from a virtual device can have a lease which refers to another\nqueue from a physical device. This is useful for memory providers and\nAF_XDP operations which take an ifindex and queue id to allow\napplications to bind against virtual devices in containers.\n"]
    pub fn get_lease(&self) -> Result<IterableLease<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Queue::Lease(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Queue",
            "Lease",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Queue<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableQueue<'a> {
        IterableQueue::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Id",
            2u16 => "Ifindex",
            3u16 => "Type",
            4u16 => "NapiId",
            5u16 => "Dmabuf",
            6u16 => "IoUring",
            7u16 => "Xsk",
            8u16 => "Lease",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableQueue<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableQueue<'a> {
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
impl<'a> Iterator for IterableQueue<'a> {
    type Item = Result<Queue<'a>, ErrorContext>;
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
                1u16 => Queue::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Queue::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Queue::Type({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Queue::NapiId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Queue::Dmabuf({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Queue::IoUring({
                    let res = Some(IterableIoUringProviderInfo::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => Queue::Xsk({
                    let res = Some(IterableXskInfo::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => Queue::Lease({
                    let res = Some(IterableLease::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Queue",
            r#type.and_then(|t| Queue::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableQueue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Queue");
        let mut iter = IterableQueue::with_loc(&[], self.orig_loc);
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
                Queue::Id(val) => fmt.field("Id", &val),
                Queue::Ifindex(val) => fmt.field("Ifindex", &val),
                Queue::Type(val) => {
                    fmt.field("Type", &FormatEnum(val.into(), QueueType::from_value))
                }
                Queue::NapiId(val) => fmt.field("NapiId", &val),
                Queue::Dmabuf(val) => fmt.field("Dmabuf", &val),
                Queue::IoUring(val) => fmt.field("IoUring", &val),
                Queue::Xsk(val) => fmt.field("Xsk", &val),
                Queue::Lease(val) => fmt.field("Lease", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableQueue<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Queue", offset));
            return (stack, missing_type.and_then(|t| Queue::attr_from_type(t)));
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
                Queue::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                Queue::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                Queue::Type(val) => {
                    if last_off == offset {
                        stack.push(("Type", last_off));
                        break;
                    }
                }
                Queue::NapiId(val) => {
                    if last_off == offset {
                        stack.push(("NapiId", last_off));
                        break;
                    }
                }
                Queue::Dmabuf(val) => {
                    if last_off == offset {
                        stack.push(("Dmabuf", last_off));
                        break;
                    }
                }
                Queue::IoUring(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Queue::Xsk(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Queue::Lease(val) => {
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
            stack.push(("Queue", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum Qstats {
    #[doc = "ifindex of the netdevice to which stats belong.\n"]
    Ifindex(u32),
    #[doc = "Queue type as rx, tx, for queue-id.\n\nAssociated type: [`QueueType`] (enum)"]
    QueueType(u32),
    #[doc = "Queue ID, if stats are scoped to a single queue instance.\n"]
    QueueId(u32),
    #[doc = "What object type should be used to iterate over the stats.\n\nAssociated type: [`QstatsScope`] (enum)"]
    Scope(u32),
    #[doc = "Number of wire packets successfully received and passed to the stack.\nFor drivers supporting XDP, XDP is considered the first layer of the\nstack, so packets consumed by XDP are still counted here.\n"]
    RxPackets(u32),
    #[doc = "Successfully received bytes, see [rx-packets]{.title-ref}.\n"]
    RxBytes(u32),
    #[doc = "Number of wire packets successfully sent. Packet is considered to be\nsuccessfully sent once it is in device memory (usually this means the\ndevice has issued a DMA completion for the packet).\n"]
    TxPackets(u32),
    #[doc = "Successfully sent bytes, see [tx-packets]{.title-ref}.\n"]
    TxBytes(u32),
    #[doc = "Number of times skb or buffer allocation failed on the Rx datapath.\nAllocation failure may, or may not result in a packet drop, depending on\ndriver implementation and whether system recovers quickly.\n"]
    RxAllocFail(u32),
    #[doc = "Number of all packets which entered the device, but never left it,\nincluding but not limited to: packets dropped due to lack of buffer\nspace, processing errors, explicit or implicit policies and packet\nfilters.\n"]
    RxHwDrops(u32),
    #[doc = "Number of packets dropped due to transient lack of resources, such as\nbuffer space, host descriptors etc.\n"]
    RxHwDropOverruns(u32),
    #[doc = "Number of packets that were marked as CHECKSUM_COMPLETE.\n"]
    RxCsumComplete(u32),
    #[doc = "Number of packets that were marked as CHECKSUM_UNNECESSARY.\n"]
    RxCsumUnnecessary(u32),
    #[doc = "Number of packets that were not checksummed by device.\n"]
    RxCsumNone(u32),
    #[doc = "Number of packets with bad checksum. The packets are not discarded, but\nstill delivered to the stack.\n"]
    RxCsumBad(u32),
    #[doc = "Number of packets that were coalesced from smaller packets by the\ndevice. Counts only packets coalesced with the HW-GRO netdevice feature,\nLRO-coalesced packets are not counted.\n"]
    RxHwGroPackets(u32),
    #[doc = "See [rx-hw-gro-packets]{.title-ref}.\n"]
    RxHwGroBytes(u32),
    #[doc = "Number of packets that were coalesced to bigger packetss with the HW-GRO\nnetdevice feature. LRO-coalesced packets are not counted.\n"]
    RxHwGroWirePackets(u32),
    #[doc = "See [rx-hw-gro-wire-packets]{.title-ref}.\n"]
    RxHwGroWireBytes(u32),
    #[doc = "Number of the packets dropped by the device due to the received packets\nbitrate exceeding the device rate limit.\n"]
    RxHwDropRatelimits(u32),
    #[doc = "Number of packets that arrived at the device but never left it,\nencompassing packets dropped for reasons such as processing errors, as\nwell as those affected by explicitly defined policies and packet\nfiltering criteria.\n"]
    TxHwDrops(u32),
    #[doc = "Number of packets dropped because they were invalid or malformed.\n"]
    TxHwDropErrors(u32),
    #[doc = "Number of packets that did not require the device to calculate the\nchecksum.\n"]
    TxCsumNone(u32),
    #[doc = "Number of packets that required the device to calculate the checksum.\nThis counter includes the number of GSO wire packets for which device\ncalculated the L4 checksum.\n"]
    TxNeedsCsum(u32),
    #[doc = "Number of packets that necessitated segmentation into smaller packets by\nthe device.\n"]
    TxHwGsoPackets(u32),
    #[doc = "See [tx-hw-gso-packets]{.title-ref}.\n"]
    TxHwGsoBytes(u32),
    #[doc = "Number of wire-sized packets generated by processing\n[tx-hw-gso-packets]{.title-ref}\n"]
    TxHwGsoWirePackets(u32),
    #[doc = "See [tx-hw-gso-wire-packets]{.title-ref}.\n"]
    TxHwGsoWireBytes(u32),
    #[doc = "Number of the packets dropped by the device due to the transmit packets\nbitrate exceeding the device rate limit.\n"]
    TxHwDropRatelimits(u32),
    #[doc = "Number of times driver paused accepting new tx packets from the stack to\nthis queue, because the queue was full. Note that if BQL is supported\nand enabled on the device the networking stack will avoid queuing a lot\nof data at once.\n"]
    TxStop(u32),
    #[doc = "Number of times driver re-started accepting send requests to this queue\nfrom the stack.\n"]
    TxWake(u32),
}
impl<'a> IterableQstats<'a> {
    #[doc = "ifindex of the netdevice to which stats belong.\n"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Queue type as rx, tx, for queue-id.\n\nAssociated type: [`QueueType`] (enum)"]
    pub fn get_queue_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::QueueType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "QueueType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Queue ID, if stats are scoped to a single queue instance.\n"]
    pub fn get_queue_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::QueueId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "QueueId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "What object type should be used to iterate over the stats.\n\nAssociated type: [`QstatsScope`] (enum)"]
    pub fn get_scope(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::Scope(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "Scope",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of wire packets successfully received and passed to the stack.\nFor drivers supporting XDP, XDP is considered the first layer of the\nstack, so packets consumed by XDP are still counted here.\n"]
    pub fn get_rx_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::RxPackets(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "RxPackets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Successfully received bytes, see [rx-packets]{.title-ref}.\n"]
    pub fn get_rx_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::RxBytes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "RxBytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of wire packets successfully sent. Packet is considered to be\nsuccessfully sent once it is in device memory (usually this means the\ndevice has issued a DMA completion for the packet).\n"]
    pub fn get_tx_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::TxPackets(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "TxPackets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Successfully sent bytes, see [tx-packets]{.title-ref}.\n"]
    pub fn get_tx_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::TxBytes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "TxBytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of times skb or buffer allocation failed on the Rx datapath.\nAllocation failure may, or may not result in a packet drop, depending on\ndriver implementation and whether system recovers quickly.\n"]
    pub fn get_rx_alloc_fail(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::RxAllocFail(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "RxAllocFail",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of all packets which entered the device, but never left it,\nincluding but not limited to: packets dropped due to lack of buffer\nspace, processing errors, explicit or implicit policies and packet\nfilters.\n"]
    pub fn get_rx_hw_drops(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::RxHwDrops(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "RxHwDrops",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of packets dropped due to transient lack of resources, such as\nbuffer space, host descriptors etc.\n"]
    pub fn get_rx_hw_drop_overruns(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::RxHwDropOverruns(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "RxHwDropOverruns",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of packets that were marked as CHECKSUM_COMPLETE.\n"]
    pub fn get_rx_csum_complete(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::RxCsumComplete(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "RxCsumComplete",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of packets that were marked as CHECKSUM_UNNECESSARY.\n"]
    pub fn get_rx_csum_unnecessary(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::RxCsumUnnecessary(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "RxCsumUnnecessary",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of packets that were not checksummed by device.\n"]
    pub fn get_rx_csum_none(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::RxCsumNone(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "RxCsumNone",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of packets with bad checksum. The packets are not discarded, but\nstill delivered to the stack.\n"]
    pub fn get_rx_csum_bad(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::RxCsumBad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "RxCsumBad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of packets that were coalesced from smaller packets by the\ndevice. Counts only packets coalesced with the HW-GRO netdevice feature,\nLRO-coalesced packets are not counted.\n"]
    pub fn get_rx_hw_gro_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::RxHwGroPackets(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "RxHwGroPackets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "See [rx-hw-gro-packets]{.title-ref}.\n"]
    pub fn get_rx_hw_gro_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::RxHwGroBytes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "RxHwGroBytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of packets that were coalesced to bigger packetss with the HW-GRO\nnetdevice feature. LRO-coalesced packets are not counted.\n"]
    pub fn get_rx_hw_gro_wire_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::RxHwGroWirePackets(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "RxHwGroWirePackets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "See [rx-hw-gro-wire-packets]{.title-ref}.\n"]
    pub fn get_rx_hw_gro_wire_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::RxHwGroWireBytes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "RxHwGroWireBytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of the packets dropped by the device due to the received packets\nbitrate exceeding the device rate limit.\n"]
    pub fn get_rx_hw_drop_ratelimits(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::RxHwDropRatelimits(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "RxHwDropRatelimits",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of packets that arrived at the device but never left it,\nencompassing packets dropped for reasons such as processing errors, as\nwell as those affected by explicitly defined policies and packet\nfiltering criteria.\n"]
    pub fn get_tx_hw_drops(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::TxHwDrops(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "TxHwDrops",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of packets dropped because they were invalid or malformed.\n"]
    pub fn get_tx_hw_drop_errors(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::TxHwDropErrors(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "TxHwDropErrors",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of packets that did not require the device to calculate the\nchecksum.\n"]
    pub fn get_tx_csum_none(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::TxCsumNone(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "TxCsumNone",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of packets that required the device to calculate the checksum.\nThis counter includes the number of GSO wire packets for which device\ncalculated the L4 checksum.\n"]
    pub fn get_tx_needs_csum(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::TxNeedsCsum(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "TxNeedsCsum",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of packets that necessitated segmentation into smaller packets by\nthe device.\n"]
    pub fn get_tx_hw_gso_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::TxHwGsoPackets(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "TxHwGsoPackets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "See [tx-hw-gso-packets]{.title-ref}.\n"]
    pub fn get_tx_hw_gso_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::TxHwGsoBytes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "TxHwGsoBytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of wire-sized packets generated by processing\n[tx-hw-gso-packets]{.title-ref}\n"]
    pub fn get_tx_hw_gso_wire_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::TxHwGsoWirePackets(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "TxHwGsoWirePackets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "See [tx-hw-gso-wire-packets]{.title-ref}.\n"]
    pub fn get_tx_hw_gso_wire_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::TxHwGsoWireBytes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "TxHwGsoWireBytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of the packets dropped by the device due to the transmit packets\nbitrate exceeding the device rate limit.\n"]
    pub fn get_tx_hw_drop_ratelimits(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::TxHwDropRatelimits(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "TxHwDropRatelimits",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of times driver paused accepting new tx packets from the stack to\nthis queue, because the queue was full. Note that if BQL is supported\nand enabled on the device the networking stack will avoid queuing a lot\nof data at once.\n"]
    pub fn get_tx_stop(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::TxStop(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "TxStop",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of times driver re-started accepting send requests to this queue\nfrom the stack.\n"]
    pub fn get_tx_wake(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Qstats::TxWake(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Qstats",
            "TxWake",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Qstats {
    pub fn new<'a>(buf: &'a [u8]) -> IterableQstats<'a> {
        IterableQstats::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Ifindex",
            2u16 => "QueueType",
            3u16 => "QueueId",
            4u16 => "Scope",
            8u16 => "RxPackets",
            9u16 => "RxBytes",
            10u16 => "TxPackets",
            11u16 => "TxBytes",
            12u16 => "RxAllocFail",
            13u16 => "RxHwDrops",
            14u16 => "RxHwDropOverruns",
            15u16 => "RxCsumComplete",
            16u16 => "RxCsumUnnecessary",
            17u16 => "RxCsumNone",
            18u16 => "RxCsumBad",
            19u16 => "RxHwGroPackets",
            20u16 => "RxHwGroBytes",
            21u16 => "RxHwGroWirePackets",
            22u16 => "RxHwGroWireBytes",
            23u16 => "RxHwDropRatelimits",
            24u16 => "TxHwDrops",
            25u16 => "TxHwDropErrors",
            26u16 => "TxCsumNone",
            27u16 => "TxNeedsCsum",
            28u16 => "TxHwGsoPackets",
            29u16 => "TxHwGsoBytes",
            30u16 => "TxHwGsoWirePackets",
            31u16 => "TxHwGsoWireBytes",
            32u16 => "TxHwDropRatelimits",
            33u16 => "TxStop",
            34u16 => "TxWake",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableQstats<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableQstats<'a> {
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
impl<'a> Iterator for IterableQstats<'a> {
    type Item = Result<Qstats, ErrorContext>;
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
                1u16 => Qstats::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Qstats::QueueType({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Qstats::QueueId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Qstats::Scope({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => Qstats::RxPackets({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => Qstats::RxBytes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => Qstats::TxPackets({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => Qstats::TxBytes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => Qstats::RxAllocFail({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => Qstats::RxHwDrops({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => Qstats::RxHwDropOverruns({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => Qstats::RxCsumComplete({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => Qstats::RxCsumUnnecessary({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => Qstats::RxCsumNone({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => Qstats::RxCsumBad({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => Qstats::RxHwGroPackets({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => Qstats::RxHwGroBytes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => Qstats::RxHwGroWirePackets({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => Qstats::RxHwGroWireBytes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => Qstats::RxHwDropRatelimits({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => Qstats::TxHwDrops({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                25u16 => Qstats::TxHwDropErrors({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                26u16 => Qstats::TxCsumNone({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                27u16 => Qstats::TxNeedsCsum({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                28u16 => Qstats::TxHwGsoPackets({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                29u16 => Qstats::TxHwGsoBytes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                30u16 => Qstats::TxHwGsoWirePackets({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                31u16 => Qstats::TxHwGsoWireBytes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                32u16 => Qstats::TxHwDropRatelimits({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                33u16 => Qstats::TxStop({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                34u16 => Qstats::TxWake({
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
            "Qstats",
            r#type.and_then(|t| Qstats::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableQstats<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Qstats");
        let mut iter = IterableQstats::with_loc(&[], self.orig_loc);
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
                Qstats::Ifindex(val) => fmt.field("Ifindex", &val),
                Qstats::QueueType(val) => {
                    fmt.field("QueueType", &FormatEnum(val.into(), QueueType::from_value))
                }
                Qstats::QueueId(val) => fmt.field("QueueId", &val),
                Qstats::Scope(val) => {
                    fmt.field("Scope", &FormatFlags(val.into(), QstatsScope::from_value))
                }
                Qstats::RxPackets(val) => fmt.field("RxPackets", &val),
                Qstats::RxBytes(val) => fmt.field("RxBytes", &val),
                Qstats::TxPackets(val) => fmt.field("TxPackets", &val),
                Qstats::TxBytes(val) => fmt.field("TxBytes", &val),
                Qstats::RxAllocFail(val) => fmt.field("RxAllocFail", &val),
                Qstats::RxHwDrops(val) => fmt.field("RxHwDrops", &val),
                Qstats::RxHwDropOverruns(val) => fmt.field("RxHwDropOverruns", &val),
                Qstats::RxCsumComplete(val) => fmt.field("RxCsumComplete", &val),
                Qstats::RxCsumUnnecessary(val) => fmt.field("RxCsumUnnecessary", &val),
                Qstats::RxCsumNone(val) => fmt.field("RxCsumNone", &val),
                Qstats::RxCsumBad(val) => fmt.field("RxCsumBad", &val),
                Qstats::RxHwGroPackets(val) => fmt.field("RxHwGroPackets", &val),
                Qstats::RxHwGroBytes(val) => fmt.field("RxHwGroBytes", &val),
                Qstats::RxHwGroWirePackets(val) => fmt.field("RxHwGroWirePackets", &val),
                Qstats::RxHwGroWireBytes(val) => fmt.field("RxHwGroWireBytes", &val),
                Qstats::RxHwDropRatelimits(val) => fmt.field("RxHwDropRatelimits", &val),
                Qstats::TxHwDrops(val) => fmt.field("TxHwDrops", &val),
                Qstats::TxHwDropErrors(val) => fmt.field("TxHwDropErrors", &val),
                Qstats::TxCsumNone(val) => fmt.field("TxCsumNone", &val),
                Qstats::TxNeedsCsum(val) => fmt.field("TxNeedsCsum", &val),
                Qstats::TxHwGsoPackets(val) => fmt.field("TxHwGsoPackets", &val),
                Qstats::TxHwGsoBytes(val) => fmt.field("TxHwGsoBytes", &val),
                Qstats::TxHwGsoWirePackets(val) => fmt.field("TxHwGsoWirePackets", &val),
                Qstats::TxHwGsoWireBytes(val) => fmt.field("TxHwGsoWireBytes", &val),
                Qstats::TxHwDropRatelimits(val) => fmt.field("TxHwDropRatelimits", &val),
                Qstats::TxStop(val) => fmt.field("TxStop", &val),
                Qstats::TxWake(val) => fmt.field("TxWake", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableQstats<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Qstats", offset));
            return (stack, missing_type.and_then(|t| Qstats::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Qstats::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                Qstats::QueueType(val) => {
                    if last_off == offset {
                        stack.push(("QueueType", last_off));
                        break;
                    }
                }
                Qstats::QueueId(val) => {
                    if last_off == offset {
                        stack.push(("QueueId", last_off));
                        break;
                    }
                }
                Qstats::Scope(val) => {
                    if last_off == offset {
                        stack.push(("Scope", last_off));
                        break;
                    }
                }
                Qstats::RxPackets(val) => {
                    if last_off == offset {
                        stack.push(("RxPackets", last_off));
                        break;
                    }
                }
                Qstats::RxBytes(val) => {
                    if last_off == offset {
                        stack.push(("RxBytes", last_off));
                        break;
                    }
                }
                Qstats::TxPackets(val) => {
                    if last_off == offset {
                        stack.push(("TxPackets", last_off));
                        break;
                    }
                }
                Qstats::TxBytes(val) => {
                    if last_off == offset {
                        stack.push(("TxBytes", last_off));
                        break;
                    }
                }
                Qstats::RxAllocFail(val) => {
                    if last_off == offset {
                        stack.push(("RxAllocFail", last_off));
                        break;
                    }
                }
                Qstats::RxHwDrops(val) => {
                    if last_off == offset {
                        stack.push(("RxHwDrops", last_off));
                        break;
                    }
                }
                Qstats::RxHwDropOverruns(val) => {
                    if last_off == offset {
                        stack.push(("RxHwDropOverruns", last_off));
                        break;
                    }
                }
                Qstats::RxCsumComplete(val) => {
                    if last_off == offset {
                        stack.push(("RxCsumComplete", last_off));
                        break;
                    }
                }
                Qstats::RxCsumUnnecessary(val) => {
                    if last_off == offset {
                        stack.push(("RxCsumUnnecessary", last_off));
                        break;
                    }
                }
                Qstats::RxCsumNone(val) => {
                    if last_off == offset {
                        stack.push(("RxCsumNone", last_off));
                        break;
                    }
                }
                Qstats::RxCsumBad(val) => {
                    if last_off == offset {
                        stack.push(("RxCsumBad", last_off));
                        break;
                    }
                }
                Qstats::RxHwGroPackets(val) => {
                    if last_off == offset {
                        stack.push(("RxHwGroPackets", last_off));
                        break;
                    }
                }
                Qstats::RxHwGroBytes(val) => {
                    if last_off == offset {
                        stack.push(("RxHwGroBytes", last_off));
                        break;
                    }
                }
                Qstats::RxHwGroWirePackets(val) => {
                    if last_off == offset {
                        stack.push(("RxHwGroWirePackets", last_off));
                        break;
                    }
                }
                Qstats::RxHwGroWireBytes(val) => {
                    if last_off == offset {
                        stack.push(("RxHwGroWireBytes", last_off));
                        break;
                    }
                }
                Qstats::RxHwDropRatelimits(val) => {
                    if last_off == offset {
                        stack.push(("RxHwDropRatelimits", last_off));
                        break;
                    }
                }
                Qstats::TxHwDrops(val) => {
                    if last_off == offset {
                        stack.push(("TxHwDrops", last_off));
                        break;
                    }
                }
                Qstats::TxHwDropErrors(val) => {
                    if last_off == offset {
                        stack.push(("TxHwDropErrors", last_off));
                        break;
                    }
                }
                Qstats::TxCsumNone(val) => {
                    if last_off == offset {
                        stack.push(("TxCsumNone", last_off));
                        break;
                    }
                }
                Qstats::TxNeedsCsum(val) => {
                    if last_off == offset {
                        stack.push(("TxNeedsCsum", last_off));
                        break;
                    }
                }
                Qstats::TxHwGsoPackets(val) => {
                    if last_off == offset {
                        stack.push(("TxHwGsoPackets", last_off));
                        break;
                    }
                }
                Qstats::TxHwGsoBytes(val) => {
                    if last_off == offset {
                        stack.push(("TxHwGsoBytes", last_off));
                        break;
                    }
                }
                Qstats::TxHwGsoWirePackets(val) => {
                    if last_off == offset {
                        stack.push(("TxHwGsoWirePackets", last_off));
                        break;
                    }
                }
                Qstats::TxHwGsoWireBytes(val) => {
                    if last_off == offset {
                        stack.push(("TxHwGsoWireBytes", last_off));
                        break;
                    }
                }
                Qstats::TxHwDropRatelimits(val) => {
                    if last_off == offset {
                        stack.push(("TxHwDropRatelimits", last_off));
                        break;
                    }
                }
                Qstats::TxStop(val) => {
                    if last_off == offset {
                        stack.push(("TxStop", last_off));
                        break;
                    }
                }
                Qstats::TxWake(val) => {
                    if last_off == offset {
                        stack.push(("TxWake", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Qstats", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum QueueId {
    #[doc = "Queue index; most queue types are indexed like a C array, with indexes\nstarting at 0 and ending at queue count - 1. Queue indexes are scoped to\nan interface and queue type.\n"]
    Id(u32),
    #[doc = "Queue type as rx, tx. Each queue type defines a separate ID space. XDP\nTX queues allocated in the kernel are not linked to NAPIs and thus not\nlisted. AF_XDP queues will have more information set in the xsk\nattribute.\n\nAssociated type: [`QueueType`] (enum)"]
    Type(u32),
}
impl<'a> IterableQueueId<'a> {
    #[doc = "Queue index; most queue types are indexed like a C array, with indexes\nstarting at 0 and ending at queue count - 1. Queue indexes are scoped to\nan interface and queue type.\n"]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(QueueId::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "QueueId",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Queue type as rx, tx. Each queue type defines a separate ID space. XDP\nTX queues allocated in the kernel are not linked to NAPIs and thus not\nlisted. AF_XDP queues will have more information set in the xsk\nattribute.\n\nAssociated type: [`QueueType`] (enum)"]
    pub fn get_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(QueueId::Type(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "QueueId",
            "Type",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl QueueId {
    pub fn new<'a>(buf: &'a [u8]) -> IterableQueueId<'a> {
        IterableQueueId::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Queue::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableQueueId<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableQueueId<'a> {
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
impl<'a> Iterator for IterableQueueId<'a> {
    type Item = Result<QueueId, ErrorContext>;
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
                1u16 => QueueId::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => QueueId::Type({
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
            "QueueId",
            r#type.and_then(|t| QueueId::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableQueueId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("QueueId");
        let mut iter = IterableQueueId::with_loc(&[], self.orig_loc);
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
                QueueId::Id(val) => fmt.field("Id", &val),
                QueueId::Type(val) => {
                    fmt.field("Type", &FormatEnum(val.into(), QueueType::from_value))
                }
            };
        }
        fmt.finish()
    }
}
impl IterableQueueId<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("QueueId", offset));
            return (stack, missing_type.and_then(|t| QueueId::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                QueueId::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                QueueId::Type(val) => {
                    if last_off == offset {
                        stack.push(("Type", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("QueueId", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum Lease<'a> {
    #[doc = "The netdev ifindex to lease the queue from.\n"]
    Ifindex(u32),
    #[doc = "The netdev queue to lease from.\n"]
    Queue(IterableQueueId<'a>),
    #[doc = "The network namespace id of the netdev.\n"]
    NetnsId(i32),
}
impl<'a> IterableLease<'a> {
    #[doc = "The netdev ifindex to lease the queue from.\n"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Lease::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Lease",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The netdev queue to lease from.\n"]
    pub fn get_queue(&self) -> Result<IterableQueueId<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Lease::Queue(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Lease",
            "Queue",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The network namespace id of the netdev.\n"]
    pub fn get_netns_id(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Lease::NetnsId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Lease",
            "NetnsId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Lease<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableLease<'a> {
        IterableLease::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Ifindex",
            2u16 => "Queue",
            3u16 => "NetnsId",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableLease<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableLease<'a> {
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
impl<'a> Iterator for IterableLease<'a> {
    type Item = Result<Lease<'a>, ErrorContext>;
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
                1u16 => Lease::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Lease::Queue({
                    let res = Some(IterableQueueId::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Lease::NetnsId({
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
            "Lease",
            r#type.and_then(|t| Lease::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableLease<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Lease");
        let mut iter = IterableLease::with_loc(&[], self.orig_loc);
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
                Lease::Ifindex(val) => fmt.field("Ifindex", &val),
                Lease::Queue(val) => fmt.field("Queue", &val),
                Lease::NetnsId(val) => fmt.field("NetnsId", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableLease<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Lease", offset));
            return (stack, missing_type.and_then(|t| Lease::attr_from_type(t)));
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
                Lease::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                Lease::Queue(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Lease::NetnsId(val) => {
                    if last_off == offset {
                        stack.push(("NetnsId", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Lease", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum Dmabuf<'a> {
    #[doc = "netdev ifindex to bind the dmabuf to.\n"]
    Ifindex(u32),
    #[doc = "receive queues to bind the dmabuf to.\n\nAttribute may repeat multiple times (treat it as array)"]
    Queues(IterableQueueId<'a>),
    #[doc = "dmabuf file descriptor to bind.\n"]
    Fd(u32),
    #[doc = "id of the dmabuf binding\n"]
    Id(u32),
}
impl<'a> IterableDmabuf<'a> {
    #[doc = "netdev ifindex to bind the dmabuf to.\n"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dmabuf::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dmabuf",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "receive queues to bind the dmabuf to.\n\nAttribute may repeat multiple times (treat it as array)"]
    pub fn get_queues(&self) -> MultiAttrIterable<Self, Dmabuf<'a>, IterableQueueId<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Dmabuf::Queues(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    #[doc = "dmabuf file descriptor to bind.\n"]
    pub fn get_fd(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dmabuf::Fd(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dmabuf",
            "Fd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "id of the dmabuf binding\n"]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Dmabuf::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Dmabuf",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Dmabuf<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableDmabuf<'a> {
        IterableDmabuf::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Ifindex",
            2u16 => "Queues",
            3u16 => "Fd",
            4u16 => "Id",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableDmabuf<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableDmabuf<'a> {
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
impl<'a> Iterator for IterableDmabuf<'a> {
    type Item = Result<Dmabuf<'a>, ErrorContext>;
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
                1u16 => Dmabuf::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Dmabuf::Queues({
                    let res = Some(IterableQueueId::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Dmabuf::Fd({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Dmabuf::Id({
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
            "Dmabuf",
            r#type.and_then(|t| Dmabuf::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableDmabuf<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Dmabuf");
        let mut iter = IterableDmabuf::with_loc(&[], self.orig_loc);
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
                Dmabuf::Ifindex(val) => fmt.field("Ifindex", &val),
                Dmabuf::Queues(val) => fmt.field("Queues", &val),
                Dmabuf::Fd(val) => fmt.field("Fd", &val),
                Dmabuf::Id(val) => fmt.field("Id", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableDmabuf<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Dmabuf", offset));
            return (stack, missing_type.and_then(|t| Dmabuf::attr_from_type(t)));
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
                Dmabuf::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                Dmabuf::Queues(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Dmabuf::Fd(val) => {
                    if last_off == offset {
                        stack.push(("Fd", last_off));
                        break;
                    }
                }
                Dmabuf::Id(val) => {
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
            stack.push(("Dmabuf", cur));
        }
        (stack, missing)
    }
}
pub struct PushDev<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDev<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDev<Prev> {
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
    #[doc = "netdev ifindex\n"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "Bitmask of enabled xdp-features.\n\nAssociated type: [`XdpAct`] (enum)"]
    pub fn push_xdp_features(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 3u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "max fragment count supported by ZC driver\n"]
    pub fn push_xdp_zc_max_segs(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Bitmask of supported XDP receive metadata features. See\nDocumentation/networking/xdp-rx-metadata.rst for more details.\n\nAssociated type: [`XdpRxMetadata`] (enum)"]
    pub fn push_xdp_rx_metadata_features(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 5u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Bitmask of enabled AF_XDP features.\n\nAssociated type: [`XskFlags`] (enum)"]
    pub fn push_xsk_features(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 6u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushDev<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushIoUringProviderInfo<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushIoUringProviderInfo<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushIoUringProviderInfo<Prev> {
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
    #[doc = "RX buffer length in bytes for this io_uring memory provider. Reflects\nthe rx_buf_len passed at io_uring zerocopy rx registration time.\n"]
    pub fn push_rx_buf_len(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushIoUringProviderInfo<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPagePool<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushPagePool<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushPagePool<Prev> {
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
    #[doc = "Unique ID of a Page Pool instance.\n"]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ifindex of the netdev to which the pool belongs. May not be reported if\nthe page pool was allocated for a netdev which got destroyed already\n(page pools may outlast their netdevs because they wait for all memory\nto be returned).\n"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Id of NAPI using this Page Pool instance.\n"]
    pub fn push_napi_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of outstanding references to this page pool (allocated but yet to\nbe freed pages). Allocated pages may be held in socket receive queues,\ndriver receive ring, page pool recycling ring, the page pool cache, etc.\n"]
    pub fn push_inflight(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Amount of memory held by inflight pages.\n"]
    pub fn push_inflight_mem(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Seconds in CLOCK_BOOTTIME of when Page Pool was detached by the driver.\nOnce detached Page Pool can no longer be used to allocate memory. Page\nPools wait for all the memory allocated from them to be freed before\ntruly disappearing. \\\"Detached\\\" Page Pools cannot be \\\"re-attached\\\",\nthey are just waiting to disappear. Attribute is absent if Page Pool has\nnot been detached, and can still be used to allocate new memory.\n"]
    pub fn push_detach_time(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ID of the dmabuf this page-pool is attached to.\n"]
    pub fn push_dmabuf(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "io-uring memory provider information.\n"]
    pub fn nested_io_uring(mut self) -> PushIoUringProviderInfo<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 8u16);
        PushIoUringProviderInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushPagePool<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPagePoolInfo<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushPagePoolInfo<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushPagePoolInfo<Prev> {
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
    #[doc = "Unique ID of a Page Pool instance.\n"]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ifindex of the netdev to which the pool belongs. May not be reported if\nthe page pool was allocated for a netdev which got destroyed already\n(page pools may outlast their netdevs because they wait for all memory\nto be returned).\n"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushPagePoolInfo<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPagePoolStats<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushPagePoolStats<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushPagePoolStats<Prev> {
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
    #[doc = "Page pool identifying information.\n"]
    pub fn nested_info(mut self) -> PushPagePoolInfo<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 1u16);
        PushPagePoolInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_alloc_fast(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_alloc_slow(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_alloc_slow_high_order(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_alloc_empty(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 11u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_alloc_refill(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 12u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_alloc_waive(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 13u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_recycle_cached(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 14u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_recycle_cache_full(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 15u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_recycle_ring(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 16u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_recycle_ring_full(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 17u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_recycle_released_refcnt(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 18u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushPagePoolStats<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushNapi<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushNapi<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushNapi<Prev> {
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
    #[doc = "ifindex of the netdevice to which NAPI instance belongs.\n"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ID of the NAPI instance.\n"]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The associated interrupt vector number for the napi\n"]
    pub fn push_irq(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "PID of the napi thread, if NAPI is configured to operate in threaded\nmode. If NAPI is not in threaded mode (i.e. uses normal softirq\ncontext), the attribute will be absent.\n"]
    pub fn push_pid(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The number of consecutive empty polls before IRQ deferral ends and\nhardware IRQs are re-enabled.\n"]
    pub fn push_defer_hard_irqs(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The timeout, in nanoseconds, of when to trigger the NAPI watchdog timer\nwhich schedules NAPI processing. Additionally, a non-zero value will\nalso prevent GRO from flushing recent super-frames at the end of a NAPI\ncycle. This may add receive latency in exchange for reducing the number\nof frames processed by the network stack.\n"]
    pub fn push_gro_flush_timeout(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The timeout, in nanoseconds, of how long to suspend irq processing, if\nevent polling finds events\n"]
    pub fn push_irq_suspend_timeout(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether the NAPI is configured to operate in threaded polling mode. If\nthis is set to enabled then the NAPI context operates in threaded\npolling mode. If this is set to busy-poll, then the threaded polling\nmode also busy polls.\n\nAssociated type: [`NapiThreaded`] (enum)"]
    pub fn push_threaded(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushNapi<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushXskInfo<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushXskInfo<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushXskInfo<Prev> {
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
}
impl<Prev: Pusher> Drop for PushXskInfo<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushQueue<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushQueue<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushQueue<Prev> {
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
    #[doc = "Queue index; most queue types are indexed like a C array, with indexes\nstarting at 0 and ending at queue count - 1. Queue indexes are scoped to\nan interface and queue type.\n"]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ifindex of the netdevice to which the queue belongs.\n"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Queue type as rx, tx. Each queue type defines a separate ID space. XDP\nTX queues allocated in the kernel are not linked to NAPIs and thus not\nlisted. AF_XDP queues will have more information set in the xsk\nattribute.\n\nAssociated type: [`QueueType`] (enum)"]
    pub fn push_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ID of the NAPI instance which services this queue.\n"]
    pub fn push_napi_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ID of the dmabuf attached to this queue, if any.\n"]
    pub fn push_dmabuf(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "io_uring memory provider information.\n"]
    pub fn nested_io_uring(mut self) -> PushIoUringProviderInfo<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 6u16);
        PushIoUringProviderInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "XSK information for this queue, if any.\n"]
    pub fn nested_xsk(mut self) -> PushXskInfo<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 7u16);
        PushXskInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "A queue from a virtual device can have a lease which refers to another\nqueue from a physical device. This is useful for memory providers and\nAF_XDP operations which take an ifindex and queue id to allow\napplications to bind against virtual devices in containers.\n"]
    pub fn nested_lease(mut self) -> PushLease<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 8u16);
        PushLease {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushQueue<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushQstats<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushQstats<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushQstats<Prev> {
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
    #[doc = "ifindex of the netdevice to which stats belong.\n"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Queue type as rx, tx, for queue-id.\n\nAssociated type: [`QueueType`] (enum)"]
    pub fn push_queue_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Queue ID, if stats are scoped to a single queue instance.\n"]
    pub fn push_queue_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "What object type should be used to iterate over the stats.\n\nAssociated type: [`QstatsScope`] (enum)"]
    pub fn push_scope(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of wire packets successfully received and passed to the stack.\nFor drivers supporting XDP, XDP is considered the first layer of the\nstack, so packets consumed by XDP are still counted here.\n"]
    pub fn push_rx_packets(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Successfully received bytes, see [rx-packets]{.title-ref}.\n"]
    pub fn push_rx_bytes(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of wire packets successfully sent. Packet is considered to be\nsuccessfully sent once it is in device memory (usually this means the\ndevice has issued a DMA completion for the packet).\n"]
    pub fn push_tx_packets(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Successfully sent bytes, see [tx-packets]{.title-ref}.\n"]
    pub fn push_tx_bytes(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 11u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of times skb or buffer allocation failed on the Rx datapath.\nAllocation failure may, or may not result in a packet drop, depending on\ndriver implementation and whether system recovers quickly.\n"]
    pub fn push_rx_alloc_fail(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 12u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of all packets which entered the device, but never left it,\nincluding but not limited to: packets dropped due to lack of buffer\nspace, processing errors, explicit or implicit policies and packet\nfilters.\n"]
    pub fn push_rx_hw_drops(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 13u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets dropped due to transient lack of resources, such as\nbuffer space, host descriptors etc.\n"]
    pub fn push_rx_hw_drop_overruns(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 14u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that were marked as CHECKSUM_COMPLETE.\n"]
    pub fn push_rx_csum_complete(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 15u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that were marked as CHECKSUM_UNNECESSARY.\n"]
    pub fn push_rx_csum_unnecessary(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 16u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that were not checksummed by device.\n"]
    pub fn push_rx_csum_none(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 17u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets with bad checksum. The packets are not discarded, but\nstill delivered to the stack.\n"]
    pub fn push_rx_csum_bad(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 18u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that were coalesced from smaller packets by the\ndevice. Counts only packets coalesced with the HW-GRO netdevice feature,\nLRO-coalesced packets are not counted.\n"]
    pub fn push_rx_hw_gro_packets(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 19u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "See [rx-hw-gro-packets]{.title-ref}.\n"]
    pub fn push_rx_hw_gro_bytes(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 20u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that were coalesced to bigger packetss with the HW-GRO\nnetdevice feature. LRO-coalesced packets are not counted.\n"]
    pub fn push_rx_hw_gro_wire_packets(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 21u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "See [rx-hw-gro-wire-packets]{.title-ref}.\n"]
    pub fn push_rx_hw_gro_wire_bytes(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 22u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of the packets dropped by the device due to the received packets\nbitrate exceeding the device rate limit.\n"]
    pub fn push_rx_hw_drop_ratelimits(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 23u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that arrived at the device but never left it,\nencompassing packets dropped for reasons such as processing errors, as\nwell as those affected by explicitly defined policies and packet\nfiltering criteria.\n"]
    pub fn push_tx_hw_drops(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 24u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets dropped because they were invalid or malformed.\n"]
    pub fn push_tx_hw_drop_errors(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 25u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that did not require the device to calculate the\nchecksum.\n"]
    pub fn push_tx_csum_none(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 26u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that required the device to calculate the checksum.\nThis counter includes the number of GSO wire packets for which device\ncalculated the L4 checksum.\n"]
    pub fn push_tx_needs_csum(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 27u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that necessitated segmentation into smaller packets by\nthe device.\n"]
    pub fn push_tx_hw_gso_packets(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 28u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "See [tx-hw-gso-packets]{.title-ref}.\n"]
    pub fn push_tx_hw_gso_bytes(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 29u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of wire-sized packets generated by processing\n[tx-hw-gso-packets]{.title-ref}\n"]
    pub fn push_tx_hw_gso_wire_packets(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 30u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "See [tx-hw-gso-wire-packets]{.title-ref}.\n"]
    pub fn push_tx_hw_gso_wire_bytes(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 31u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of the packets dropped by the device due to the transmit packets\nbitrate exceeding the device rate limit.\n"]
    pub fn push_tx_hw_drop_ratelimits(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 32u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of times driver paused accepting new tx packets from the stack to\nthis queue, because the queue was full. Note that if BQL is supported\nand enabled on the device the networking stack will avoid queuing a lot\nof data at once.\n"]
    pub fn push_tx_stop(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 33u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of times driver re-started accepting send requests to this queue\nfrom the stack.\n"]
    pub fn push_tx_wake(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 34u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushQstats<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushQueueId<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushQueueId<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushQueueId<Prev> {
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
    #[doc = "Queue index; most queue types are indexed like a C array, with indexes\nstarting at 0 and ending at queue count - 1. Queue indexes are scoped to\nan interface and queue type.\n"]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Queue type as rx, tx. Each queue type defines a separate ID space. XDP\nTX queues allocated in the kernel are not linked to NAPIs and thus not\nlisted. AF_XDP queues will have more information set in the xsk\nattribute.\n\nAssociated type: [`QueueType`] (enum)"]
    pub fn push_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushQueueId<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushLease<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushLease<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushLease<Prev> {
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
    #[doc = "The netdev ifindex to lease the queue from.\n"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The netdev queue to lease from.\n"]
    pub fn nested_queue(mut self) -> PushQueueId<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushQueueId {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "The network namespace id of the netdev.\n"]
    pub fn push_netns_id(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushLease<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDmabuf<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushDmabuf<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushDmabuf<Prev> {
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
    #[doc = "netdev ifindex to bind the dmabuf to.\n"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "receive queues to bind the dmabuf to.\n\nAttribute may repeat multiple times (treat it as array)"]
    pub fn nested_queues(mut self) -> PushQueueId<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushQueueId {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "dmabuf file descriptor to bind.\n"]
    pub fn push_fd(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "id of the dmabuf binding\n"]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushDmabuf<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Notify attributes:\n- [`.get_ifindex()`](IterableDev::get_ifindex)\n- [`.get_xdp_features()`](IterableDev::get_xdp_features)\n- [`.get_xdp_zc_max_segs()`](IterableDev::get_xdp_zc_max_segs)\n- [`.get_xdp_rx_metadata_features()`](IterableDev::get_xdp_rx_metadata_features)\n- [`.get_xsk_features()`](IterableDev::get_xsk_features)\n"]
#[derive(Debug)]
pub struct OpDevAddNotif;
impl OpDevAddNotif {
    pub const CMD: u8 = 2u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterableDev<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDev::with_loc(attrs, buf.as_ptr() as usize)
    }
}
#[doc = "Notify attributes:\n- [`.get_ifindex()`](IterableDev::get_ifindex)\n- [`.get_xdp_features()`](IterableDev::get_xdp_features)\n- [`.get_xdp_zc_max_segs()`](IterableDev::get_xdp_zc_max_segs)\n- [`.get_xdp_rx_metadata_features()`](IterableDev::get_xdp_rx_metadata_features)\n- [`.get_xsk_features()`](IterableDev::get_xsk_features)\n"]
#[derive(Debug)]
pub struct OpDevDelNotif;
impl OpDevDelNotif {
    pub const CMD: u8 = 3u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterableDev<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDev::with_loc(attrs, buf.as_ptr() as usize)
    }
}
#[doc = "Notify attributes:\n- [`.get_ifindex()`](IterableDev::get_ifindex)\n- [`.get_xdp_features()`](IterableDev::get_xdp_features)\n- [`.get_xdp_zc_max_segs()`](IterableDev::get_xdp_zc_max_segs)\n- [`.get_xdp_rx_metadata_features()`](IterableDev::get_xdp_rx_metadata_features)\n- [`.get_xsk_features()`](IterableDev::get_xsk_features)\n"]
#[derive(Debug)]
pub struct OpDevChangeNotif;
impl OpDevChangeNotif {
    pub const CMD: u8 = 4u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterableDev<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDev::with_loc(attrs, buf.as_ptr() as usize)
    }
}
#[doc = "Notify attributes:\n- [`.get_id()`](IterablePagePool::get_id)\n- [`.get_ifindex()`](IterablePagePool::get_ifindex)\n- [`.get_napi_id()`](IterablePagePool::get_napi_id)\n- [`.get_inflight()`](IterablePagePool::get_inflight)\n- [`.get_inflight_mem()`](IterablePagePool::get_inflight_mem)\n- [`.get_detach_time()`](IterablePagePool::get_detach_time)\n- [`.get_dmabuf()`](IterablePagePool::get_dmabuf)\n- [`.get_io_uring()`](IterablePagePool::get_io_uring)\n"]
#[derive(Debug)]
pub struct OpPagePoolAddNotif;
impl OpPagePoolAddNotif {
    pub const CMD: u8 = 6u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterablePagePool<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterablePagePool::with_loc(attrs, buf.as_ptr() as usize)
    }
}
#[doc = "Notify attributes:\n- [`.get_id()`](IterablePagePool::get_id)\n- [`.get_ifindex()`](IterablePagePool::get_ifindex)\n- [`.get_napi_id()`](IterablePagePool::get_napi_id)\n- [`.get_inflight()`](IterablePagePool::get_inflight)\n- [`.get_inflight_mem()`](IterablePagePool::get_inflight_mem)\n- [`.get_detach_time()`](IterablePagePool::get_detach_time)\n- [`.get_dmabuf()`](IterablePagePool::get_dmabuf)\n- [`.get_io_uring()`](IterablePagePool::get_io_uring)\n"]
#[derive(Debug)]
pub struct OpPagePoolDelNotif;
impl OpPagePoolDelNotif {
    pub const CMD: u8 = 7u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterablePagePool<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterablePagePool::with_loc(attrs, buf.as_ptr() as usize)
    }
}
#[doc = "Notify attributes:\n- [`.get_id()`](IterablePagePool::get_id)\n- [`.get_ifindex()`](IterablePagePool::get_ifindex)\n- [`.get_napi_id()`](IterablePagePool::get_napi_id)\n- [`.get_inflight()`](IterablePagePool::get_inflight)\n- [`.get_inflight_mem()`](IterablePagePool::get_inflight_mem)\n- [`.get_detach_time()`](IterablePagePool::get_detach_time)\n- [`.get_dmabuf()`](IterablePagePool::get_dmabuf)\n- [`.get_io_uring()`](IterablePagePool::get_io_uring)\n"]
#[derive(Debug)]
pub struct OpPagePoolChangeNotif;
impl OpPagePoolChangeNotif {
    pub const CMD: u8 = 8u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterablePagePool<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterablePagePool::with_loc(attrs, buf.as_ptr() as usize)
    }
}
pub struct NotifGroup;
impl NotifGroup {
    #[doc = "Notifications:\n- [`OpDevAddNotif`]\n- [`OpDevDelNotif`]\n- [`OpDevChangeNotif`]\n"]
    pub const MGMT: &str = "mgmt";
    #[doc = "Notifications:\n- [`OpDevAddNotif`]\n- [`OpDevDelNotif`]\n- [`OpDevChangeNotif`]\n"]
    pub const MGMT_CSTR: &CStr = c"mgmt";
    #[doc = "Notifications:\n- [`OpPagePoolAddNotif`]\n- [`OpPagePoolDelNotif`]\n- [`OpPagePoolChangeNotif`]\n"]
    pub const PAGE_POOL: &str = "page-pool";
    #[doc = "Notifications:\n- [`OpPagePoolAddNotif`]\n- [`OpPagePoolDelNotif`]\n- [`OpPagePoolChangeNotif`]\n"]
    pub const PAGE_POOL_CSTR: &CStr = c"page-pool";
}
#[doc = "Get / dump information about a netdev.\n\nReply attributes:\n- [.get_ifindex()](IterableDev::get_ifindex)\n- [.get_xdp_features()](IterableDev::get_xdp_features)\n- [.get_xdp_zc_max_segs()](IterableDev::get_xdp_zc_max_segs)\n- [.get_xdp_rx_metadata_features()](IterableDev::get_xdp_rx_metadata_features)\n- [.get_xsk_features()](IterableDev::get_xsk_features)\n\n"]
#[derive(Debug)]
pub struct OpDevGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpDevGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDev<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDev::new(buf)
    }
    pub fn encode(&mut self) -> PushDev<&mut Vec<u8>> {
        PushDev::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDev<RequestBuf<'r>> {
        PushDev::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDev<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDev::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpDevGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDev<'buf>;
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
#[doc = "Get / dump information about a netdev.\n\nRequest attributes:\n- [.push_ifindex()](PushDev::push_ifindex)\n\nReply attributes:\n- [.get_ifindex()](IterableDev::get_ifindex)\n- [.get_xdp_features()](IterableDev::get_xdp_features)\n- [.get_xdp_zc_max_segs()](IterableDev::get_xdp_zc_max_segs)\n- [.get_xdp_rx_metadata_features()](IterableDev::get_xdp_rx_metadata_features)\n- [.get_xsk_features()](IterableDev::get_xsk_features)\n\n"]
#[derive(Debug)]
pub struct OpDevGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDevGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDev<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDev::new(buf)
    }
    pub fn encode(&mut self) -> PushDev<&mut Vec<u8>> {
        PushDev::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDev<RequestBuf<'r>> {
        PushDev::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDev<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDev::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpDevGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDev<'buf>;
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
#[doc = "Get / dump information about Page Pools. Only Page Pools associated by\nthe driver with a net_device can be listed. ifindex will not be reported\nif the net_device no longer exists.\n\nRequest attributes:\n- [.push_ifindex()](PushPagePool::push_ifindex)\n\nReply attributes:\n- [.get_id()](IterablePagePool::get_id)\n- [.get_ifindex()](IterablePagePool::get_ifindex)\n- [.get_napi_id()](IterablePagePool::get_napi_id)\n- [.get_inflight()](IterablePagePool::get_inflight)\n- [.get_inflight_mem()](IterablePagePool::get_inflight_mem)\n- [.get_detach_time()](IterablePagePool::get_detach_time)\n- [.get_dmabuf()](IterablePagePool::get_dmabuf)\n- [.get_io_uring()](IterablePagePool::get_io_uring)\n\n"]
#[derive(Debug)]
pub struct OpPagePoolGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpPagePoolGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushPagePool<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushPagePool::new(buf)
    }
    pub fn encode(&mut self) -> PushPagePool<&mut Vec<u8>> {
        PushPagePool::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushPagePool<RequestBuf<'r>> {
        PushPagePool::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterablePagePool<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterablePagePool::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpPagePoolGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterablePagePool<'buf>;
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
#[doc = "Get / dump information about Page Pools. Only Page Pools associated by\nthe driver with a net_device can be listed. ifindex will not be reported\nif the net_device no longer exists.\n\nRequest attributes:\n- [.push_id()](PushPagePool::push_id)\n\nReply attributes:\n- [.get_id()](IterablePagePool::get_id)\n- [.get_ifindex()](IterablePagePool::get_ifindex)\n- [.get_napi_id()](IterablePagePool::get_napi_id)\n- [.get_inflight()](IterablePagePool::get_inflight)\n- [.get_inflight_mem()](IterablePagePool::get_inflight_mem)\n- [.get_detach_time()](IterablePagePool::get_detach_time)\n- [.get_dmabuf()](IterablePagePool::get_dmabuf)\n- [.get_io_uring()](IterablePagePool::get_io_uring)\n\n"]
#[derive(Debug)]
pub struct OpPagePoolGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPagePoolGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushPagePool<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushPagePool::new(buf)
    }
    pub fn encode(&mut self) -> PushPagePool<&mut Vec<u8>> {
        PushPagePool::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushPagePool<RequestBuf<'r>> {
        PushPagePool::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterablePagePool<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterablePagePool::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpPagePoolGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterablePagePool<'buf>;
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
#[doc = "Get page pool statistics.\n\nRequest attributes:\n- [.nested_info()](PushPagePoolStats::nested_info)\n\nReply attributes:\n- [.get_info()](IterablePagePoolStats::get_info)\n- [.get_alloc_fast()](IterablePagePoolStats::get_alloc_fast)\n- [.get_alloc_slow()](IterablePagePoolStats::get_alloc_slow)\n- [.get_alloc_slow_high_order()](IterablePagePoolStats::get_alloc_slow_high_order)\n- [.get_alloc_empty()](IterablePagePoolStats::get_alloc_empty)\n- [.get_alloc_refill()](IterablePagePoolStats::get_alloc_refill)\n- [.get_alloc_waive()](IterablePagePoolStats::get_alloc_waive)\n- [.get_recycle_cached()](IterablePagePoolStats::get_recycle_cached)\n- [.get_recycle_cache_full()](IterablePagePoolStats::get_recycle_cache_full)\n- [.get_recycle_ring()](IterablePagePoolStats::get_recycle_ring)\n- [.get_recycle_ring_full()](IterablePagePoolStats::get_recycle_ring_full)\n- [.get_recycle_released_refcnt()](IterablePagePoolStats::get_recycle_released_refcnt)\n\n"]
#[derive(Debug)]
pub struct OpPagePoolStatsGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpPagePoolStatsGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushPagePoolStats<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushPagePoolStats::new(buf)
    }
    pub fn encode(&mut self) -> PushPagePoolStats<&mut Vec<u8>> {
        PushPagePoolStats::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushPagePoolStats<RequestBuf<'r>> {
        PushPagePoolStats::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterablePagePoolStats<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterablePagePoolStats::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpPagePoolStatsGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterablePagePoolStats<'buf>;
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
#[doc = "Get page pool statistics.\n\nRequest attributes:\n- [.nested_info()](PushPagePoolStats::nested_info)\n\nReply attributes:\n- [.get_info()](IterablePagePoolStats::get_info)\n- [.get_alloc_fast()](IterablePagePoolStats::get_alloc_fast)\n- [.get_alloc_slow()](IterablePagePoolStats::get_alloc_slow)\n- [.get_alloc_slow_high_order()](IterablePagePoolStats::get_alloc_slow_high_order)\n- [.get_alloc_empty()](IterablePagePoolStats::get_alloc_empty)\n- [.get_alloc_refill()](IterablePagePoolStats::get_alloc_refill)\n- [.get_alloc_waive()](IterablePagePoolStats::get_alloc_waive)\n- [.get_recycle_cached()](IterablePagePoolStats::get_recycle_cached)\n- [.get_recycle_cache_full()](IterablePagePoolStats::get_recycle_cache_full)\n- [.get_recycle_ring()](IterablePagePoolStats::get_recycle_ring)\n- [.get_recycle_ring_full()](IterablePagePoolStats::get_recycle_ring_full)\n- [.get_recycle_released_refcnt()](IterablePagePoolStats::get_recycle_released_refcnt)\n\n"]
#[derive(Debug)]
pub struct OpPagePoolStatsGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPagePoolStatsGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushPagePoolStats<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushPagePoolStats::new(buf)
    }
    pub fn encode(&mut self) -> PushPagePoolStats<&mut Vec<u8>> {
        PushPagePoolStats::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushPagePoolStats<RequestBuf<'r>> {
        PushPagePoolStats::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterablePagePoolStats<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterablePagePoolStats::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpPagePoolStatsGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterablePagePoolStats<'buf>;
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
#[doc = "Get queue information from the kernel. Only configured queues will be\nreported (as opposed to all available hardware queues).\n\nRequest attributes:\n- [.push_ifindex()](PushQueue::push_ifindex)\n\nReply attributes:\n- [.get_id()](IterableQueue::get_id)\n- [.get_ifindex()](IterableQueue::get_ifindex)\n- [.get_type()](IterableQueue::get_type)\n- [.get_napi_id()](IterableQueue::get_napi_id)\n- [.get_dmabuf()](IterableQueue::get_dmabuf)\n- [.get_io_uring()](IterableQueue::get_io_uring)\n- [.get_xsk()](IterableQueue::get_xsk)\n- [.get_lease()](IterableQueue::get_lease)\n\n"]
#[derive(Debug)]
pub struct OpQueueGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpQueueGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushQueue<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushQueue::new(buf)
    }
    pub fn encode(&mut self) -> PushQueue<&mut Vec<u8>> {
        PushQueue::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushQueue<RequestBuf<'r>> {
        PushQueue::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableQueue<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableQueue::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpQueueGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableQueue<'buf>;
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
#[doc = "Get queue information from the kernel. Only configured queues will be\nreported (as opposed to all available hardware queues).\n\nRequest attributes:\n- [.push_id()](PushQueue::push_id)\n- [.push_ifindex()](PushQueue::push_ifindex)\n- [.push_type()](PushQueue::push_type)\n\nReply attributes:\n- [.get_id()](IterableQueue::get_id)\n- [.get_ifindex()](IterableQueue::get_ifindex)\n- [.get_type()](IterableQueue::get_type)\n- [.get_napi_id()](IterableQueue::get_napi_id)\n- [.get_dmabuf()](IterableQueue::get_dmabuf)\n- [.get_io_uring()](IterableQueue::get_io_uring)\n- [.get_xsk()](IterableQueue::get_xsk)\n- [.get_lease()](IterableQueue::get_lease)\n\n"]
#[derive(Debug)]
pub struct OpQueueGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpQueueGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushQueue<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushQueue::new(buf)
    }
    pub fn encode(&mut self) -> PushQueue<&mut Vec<u8>> {
        PushQueue::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushQueue<RequestBuf<'r>> {
        PushQueue::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableQueue<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableQueue::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpQueueGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableQueue<'buf>;
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
#[doc = "Get information about NAPI instances configured on the system.\n\nRequest attributes:\n- [.push_ifindex()](PushNapi::push_ifindex)\n\nReply attributes:\n- [.get_ifindex()](IterableNapi::get_ifindex)\n- [.get_id()](IterableNapi::get_id)\n- [.get_irq()](IterableNapi::get_irq)\n- [.get_pid()](IterableNapi::get_pid)\n- [.get_defer_hard_irqs()](IterableNapi::get_defer_hard_irqs)\n- [.get_gro_flush_timeout()](IterableNapi::get_gro_flush_timeout)\n- [.get_irq_suspend_timeout()](IterableNapi::get_irq_suspend_timeout)\n- [.get_threaded()](IterableNapi::get_threaded)\n\n"]
#[derive(Debug)]
pub struct OpNapiGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpNapiGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushNapi<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushNapi::new(buf)
    }
    pub fn encode(&mut self) -> PushNapi<&mut Vec<u8>> {
        PushNapi::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushNapi<RequestBuf<'r>> {
        PushNapi::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableNapi<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableNapi::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpNapiGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableNapi<'buf>;
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
#[doc = "Get information about NAPI instances configured on the system.\n\nRequest attributes:\n- [.push_id()](PushNapi::push_id)\n\nReply attributes:\n- [.get_ifindex()](IterableNapi::get_ifindex)\n- [.get_id()](IterableNapi::get_id)\n- [.get_irq()](IterableNapi::get_irq)\n- [.get_pid()](IterableNapi::get_pid)\n- [.get_defer_hard_irqs()](IterableNapi::get_defer_hard_irqs)\n- [.get_gro_flush_timeout()](IterableNapi::get_gro_flush_timeout)\n- [.get_irq_suspend_timeout()](IterableNapi::get_irq_suspend_timeout)\n- [.get_threaded()](IterableNapi::get_threaded)\n\n"]
#[derive(Debug)]
pub struct OpNapiGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpNapiGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushNapi<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushNapi::new(buf)
    }
    pub fn encode(&mut self) -> PushNapi<&mut Vec<u8>> {
        PushNapi::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushNapi<RequestBuf<'r>> {
        PushNapi::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableNapi<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableNapi::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpNapiGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableNapi<'buf>;
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
#[doc = "Get / dump fine grained statistics. Which statistics are reported\ndepends on the device and the driver, and whether the driver stores\nsoftware counters per-queue.\n\nRequest attributes:\n- [.push_ifindex()](PushQstats::push_ifindex)\n- [.push_scope()](PushQstats::push_scope)\n\nReply attributes:\n- [.get_ifindex()](IterableQstats::get_ifindex)\n- [.get_queue_type()](IterableQstats::get_queue_type)\n- [.get_queue_id()](IterableQstats::get_queue_id)\n- [.get_rx_packets()](IterableQstats::get_rx_packets)\n- [.get_rx_bytes()](IterableQstats::get_rx_bytes)\n- [.get_tx_packets()](IterableQstats::get_tx_packets)\n- [.get_tx_bytes()](IterableQstats::get_tx_bytes)\n- [.get_rx_alloc_fail()](IterableQstats::get_rx_alloc_fail)\n- [.get_rx_hw_drops()](IterableQstats::get_rx_hw_drops)\n- [.get_rx_hw_drop_overruns()](IterableQstats::get_rx_hw_drop_overruns)\n- [.get_rx_csum_complete()](IterableQstats::get_rx_csum_complete)\n- [.get_rx_csum_unnecessary()](IterableQstats::get_rx_csum_unnecessary)\n- [.get_rx_csum_none()](IterableQstats::get_rx_csum_none)\n- [.get_rx_csum_bad()](IterableQstats::get_rx_csum_bad)\n- [.get_rx_hw_gro_packets()](IterableQstats::get_rx_hw_gro_packets)\n- [.get_rx_hw_gro_bytes()](IterableQstats::get_rx_hw_gro_bytes)\n- [.get_rx_hw_gro_wire_packets()](IterableQstats::get_rx_hw_gro_wire_packets)\n- [.get_rx_hw_gro_wire_bytes()](IterableQstats::get_rx_hw_gro_wire_bytes)\n- [.get_rx_hw_drop_ratelimits()](IterableQstats::get_rx_hw_drop_ratelimits)\n- [.get_tx_hw_drops()](IterableQstats::get_tx_hw_drops)\n- [.get_tx_hw_drop_errors()](IterableQstats::get_tx_hw_drop_errors)\n- [.get_tx_csum_none()](IterableQstats::get_tx_csum_none)\n- [.get_tx_needs_csum()](IterableQstats::get_tx_needs_csum)\n- [.get_tx_hw_gso_packets()](IterableQstats::get_tx_hw_gso_packets)\n- [.get_tx_hw_gso_bytes()](IterableQstats::get_tx_hw_gso_bytes)\n- [.get_tx_hw_gso_wire_packets()](IterableQstats::get_tx_hw_gso_wire_packets)\n- [.get_tx_hw_gso_wire_bytes()](IterableQstats::get_tx_hw_gso_wire_bytes)\n- [.get_tx_hw_drop_ratelimits()](IterableQstats::get_tx_hw_drop_ratelimits)\n- [.get_tx_stop()](IterableQstats::get_tx_stop)\n- [.get_tx_wake()](IterableQstats::get_tx_wake)\n\n"]
#[derive(Debug)]
pub struct OpQstatsGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpQstatsGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushQstats<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushQstats::new(buf)
    }
    pub fn encode(&mut self) -> PushQstats<&mut Vec<u8>> {
        PushQstats::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushQstats<RequestBuf<'r>> {
        PushQstats::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableQstats<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableQstats::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 12u8;
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
impl NetlinkRequest for OpQstatsGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableQstats<'buf>;
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
#[doc = "Bind dmabuf to netdev\n\nFlags: uns-admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushDmabuf::push_ifindex)\n- [.nested_queues()](PushDmabuf::nested_queues)\n- [.push_fd()](PushDmabuf::push_fd)\n\nReply attributes:\n- [.get_id()](IterableDmabuf::get_id)\n\n"]
#[derive(Debug)]
pub struct OpBindRxDo<'r> {
    request: Request<'r>,
}
impl<'r> OpBindRxDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDmabuf<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDmabuf::new(buf)
    }
    pub fn encode(&mut self) -> PushDmabuf<&mut Vec<u8>> {
        PushDmabuf::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDmabuf<RequestBuf<'r>> {
        PushDmabuf::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDmabuf<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDmabuf::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 13u8;
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
impl NetlinkRequest for OpBindRxDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDmabuf<'buf>;
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
#[doc = "Set configurable NAPI instance settings.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_id()](PushNapi::push_id)\n- [.push_defer_hard_irqs()](PushNapi::push_defer_hard_irqs)\n- [.push_gro_flush_timeout()](PushNapi::push_gro_flush_timeout)\n- [.push_irq_suspend_timeout()](PushNapi::push_irq_suspend_timeout)\n- [.push_threaded()](PushNapi::push_threaded)\n\n"]
#[derive(Debug)]
pub struct OpNapiSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpNapiSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushNapi<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushNapi::new(buf)
    }
    pub fn encode(&mut self) -> PushNapi<&mut Vec<u8>> {
        PushNapi::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushNapi<RequestBuf<'r>> {
        PushNapi::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableNapi<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableNapi::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 14u8;
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
impl NetlinkRequest for OpNapiSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableNapi<'buf>;
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
#[doc = "Bind dmabuf to netdev for TX\n\nRequest attributes:\n- [.push_ifindex()](PushDmabuf::push_ifindex)\n- [.push_fd()](PushDmabuf::push_fd)\n\nReply attributes:\n- [.get_id()](IterableDmabuf::get_id)\n\n"]
#[derive(Debug)]
pub struct OpBindTxDo<'r> {
    request: Request<'r>,
}
impl<'r> OpBindTxDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushDmabuf<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushDmabuf::new(buf)
    }
    pub fn encode(&mut self) -> PushDmabuf<&mut Vec<u8>> {
        PushDmabuf::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushDmabuf<RequestBuf<'r>> {
        PushDmabuf::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableDmabuf<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableDmabuf::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 15u8;
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
impl NetlinkRequest for OpBindTxDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableDmabuf<'buf>;
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
#[doc = "Create a new queue for the given netdevice. Whether this operation is\nsupported depends on the device and the driver.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushQueue::push_ifindex)\n- [.push_type()](PushQueue::push_type)\n- [.nested_lease()](PushQueue::nested_lease)\n\nReply attributes:\n- [.get_id()](IterableQueue::get_id)\n\n"]
#[derive(Debug)]
pub struct OpQueueCreateDo<'r> {
    request: Request<'r>,
}
impl<'r> OpQueueCreateDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushQueue<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushQueue::new(buf)
    }
    pub fn encode(&mut self) -> PushQueue<&mut Vec<u8>> {
        PushQueue::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushQueue<RequestBuf<'r>> {
        PushQueue::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableQueue<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableQueue::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 16u8;
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
impl NetlinkRequest for OpQueueCreateDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableQueue<'buf>;
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
    #[doc = "Get / dump information about a netdev.\n\nReply attributes:\n- [.get_ifindex()](IterableDev::get_ifindex)\n- [.get_xdp_features()](IterableDev::get_xdp_features)\n- [.get_xdp_zc_max_segs()](IterableDev::get_xdp_zc_max_segs)\n- [.get_xdp_rx_metadata_features()](IterableDev::get_xdp_rx_metadata_features)\n- [.get_xsk_features()](IterableDev::get_xsk_features)\n\n"]
    pub fn op_dev_get_dump(self) -> OpDevGetDump<'buf> {
        let mut res = OpDevGetDump::new(self);
        res.request
            .do_writeback(res.protocol(), "op-dev-get-dump", OpDevGetDump::lookup);
        res
    }
    #[doc = "Get / dump information about a netdev.\n\nRequest attributes:\n- [.push_ifindex()](PushDev::push_ifindex)\n\nReply attributes:\n- [.get_ifindex()](IterableDev::get_ifindex)\n- [.get_xdp_features()](IterableDev::get_xdp_features)\n- [.get_xdp_zc_max_segs()](IterableDev::get_xdp_zc_max_segs)\n- [.get_xdp_rx_metadata_features()](IterableDev::get_xdp_rx_metadata_features)\n- [.get_xsk_features()](IterableDev::get_xsk_features)\n\n"]
    pub fn op_dev_get_do(self) -> OpDevGetDo<'buf> {
        let mut res = OpDevGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-dev-get-do", OpDevGetDo::lookup);
        res
    }
    #[doc = "Get / dump information about Page Pools. Only Page Pools associated by\nthe driver with a net_device can be listed. ifindex will not be reported\nif the net_device no longer exists.\n\nRequest attributes:\n- [.push_ifindex()](PushPagePool::push_ifindex)\n\nReply attributes:\n- [.get_id()](IterablePagePool::get_id)\n- [.get_ifindex()](IterablePagePool::get_ifindex)\n- [.get_napi_id()](IterablePagePool::get_napi_id)\n- [.get_inflight()](IterablePagePool::get_inflight)\n- [.get_inflight_mem()](IterablePagePool::get_inflight_mem)\n- [.get_detach_time()](IterablePagePool::get_detach_time)\n- [.get_dmabuf()](IterablePagePool::get_dmabuf)\n- [.get_io_uring()](IterablePagePool::get_io_uring)\n\n"]
    pub fn op_page_pool_get_dump(self) -> OpPagePoolGetDump<'buf> {
        let mut res = OpPagePoolGetDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-page-pool-get-dump",
            OpPagePoolGetDump::lookup,
        );
        res
    }
    #[doc = "Get / dump information about Page Pools. Only Page Pools associated by\nthe driver with a net_device can be listed. ifindex will not be reported\nif the net_device no longer exists.\n\nRequest attributes:\n- [.push_id()](PushPagePool::push_id)\n\nReply attributes:\n- [.get_id()](IterablePagePool::get_id)\n- [.get_ifindex()](IterablePagePool::get_ifindex)\n- [.get_napi_id()](IterablePagePool::get_napi_id)\n- [.get_inflight()](IterablePagePool::get_inflight)\n- [.get_inflight_mem()](IterablePagePool::get_inflight_mem)\n- [.get_detach_time()](IterablePagePool::get_detach_time)\n- [.get_dmabuf()](IterablePagePool::get_dmabuf)\n- [.get_io_uring()](IterablePagePool::get_io_uring)\n\n"]
    pub fn op_page_pool_get_do(self) -> OpPagePoolGetDo<'buf> {
        let mut res = OpPagePoolGetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-page-pool-get-do",
            OpPagePoolGetDo::lookup,
        );
        res
    }
    #[doc = "Get page pool statistics.\n\nRequest attributes:\n- [.nested_info()](PushPagePoolStats::nested_info)\n\nReply attributes:\n- [.get_info()](IterablePagePoolStats::get_info)\n- [.get_alloc_fast()](IterablePagePoolStats::get_alloc_fast)\n- [.get_alloc_slow()](IterablePagePoolStats::get_alloc_slow)\n- [.get_alloc_slow_high_order()](IterablePagePoolStats::get_alloc_slow_high_order)\n- [.get_alloc_empty()](IterablePagePoolStats::get_alloc_empty)\n- [.get_alloc_refill()](IterablePagePoolStats::get_alloc_refill)\n- [.get_alloc_waive()](IterablePagePoolStats::get_alloc_waive)\n- [.get_recycle_cached()](IterablePagePoolStats::get_recycle_cached)\n- [.get_recycle_cache_full()](IterablePagePoolStats::get_recycle_cache_full)\n- [.get_recycle_ring()](IterablePagePoolStats::get_recycle_ring)\n- [.get_recycle_ring_full()](IterablePagePoolStats::get_recycle_ring_full)\n- [.get_recycle_released_refcnt()](IterablePagePoolStats::get_recycle_released_refcnt)\n\n"]
    pub fn op_page_pool_stats_get_dump(self) -> OpPagePoolStatsGetDump<'buf> {
        let mut res = OpPagePoolStatsGetDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-page-pool-stats-get-dump",
            OpPagePoolStatsGetDump::lookup,
        );
        res
    }
    #[doc = "Get page pool statistics.\n\nRequest attributes:\n- [.nested_info()](PushPagePoolStats::nested_info)\n\nReply attributes:\n- [.get_info()](IterablePagePoolStats::get_info)\n- [.get_alloc_fast()](IterablePagePoolStats::get_alloc_fast)\n- [.get_alloc_slow()](IterablePagePoolStats::get_alloc_slow)\n- [.get_alloc_slow_high_order()](IterablePagePoolStats::get_alloc_slow_high_order)\n- [.get_alloc_empty()](IterablePagePoolStats::get_alloc_empty)\n- [.get_alloc_refill()](IterablePagePoolStats::get_alloc_refill)\n- [.get_alloc_waive()](IterablePagePoolStats::get_alloc_waive)\n- [.get_recycle_cached()](IterablePagePoolStats::get_recycle_cached)\n- [.get_recycle_cache_full()](IterablePagePoolStats::get_recycle_cache_full)\n- [.get_recycle_ring()](IterablePagePoolStats::get_recycle_ring)\n- [.get_recycle_ring_full()](IterablePagePoolStats::get_recycle_ring_full)\n- [.get_recycle_released_refcnt()](IterablePagePoolStats::get_recycle_released_refcnt)\n\n"]
    pub fn op_page_pool_stats_get_do(self) -> OpPagePoolStatsGetDo<'buf> {
        let mut res = OpPagePoolStatsGetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-page-pool-stats-get-do",
            OpPagePoolStatsGetDo::lookup,
        );
        res
    }
    #[doc = "Get queue information from the kernel. Only configured queues will be\nreported (as opposed to all available hardware queues).\n\nRequest attributes:\n- [.push_ifindex()](PushQueue::push_ifindex)\n\nReply attributes:\n- [.get_id()](IterableQueue::get_id)\n- [.get_ifindex()](IterableQueue::get_ifindex)\n- [.get_type()](IterableQueue::get_type)\n- [.get_napi_id()](IterableQueue::get_napi_id)\n- [.get_dmabuf()](IterableQueue::get_dmabuf)\n- [.get_io_uring()](IterableQueue::get_io_uring)\n- [.get_xsk()](IterableQueue::get_xsk)\n- [.get_lease()](IterableQueue::get_lease)\n\n"]
    pub fn op_queue_get_dump(self) -> OpQueueGetDump<'buf> {
        let mut res = OpQueueGetDump::new(self);
        res.request
            .do_writeback(res.protocol(), "op-queue-get-dump", OpQueueGetDump::lookup);
        res
    }
    #[doc = "Get queue information from the kernel. Only configured queues will be\nreported (as opposed to all available hardware queues).\n\nRequest attributes:\n- [.push_id()](PushQueue::push_id)\n- [.push_ifindex()](PushQueue::push_ifindex)\n- [.push_type()](PushQueue::push_type)\n\nReply attributes:\n- [.get_id()](IterableQueue::get_id)\n- [.get_ifindex()](IterableQueue::get_ifindex)\n- [.get_type()](IterableQueue::get_type)\n- [.get_napi_id()](IterableQueue::get_napi_id)\n- [.get_dmabuf()](IterableQueue::get_dmabuf)\n- [.get_io_uring()](IterableQueue::get_io_uring)\n- [.get_xsk()](IterableQueue::get_xsk)\n- [.get_lease()](IterableQueue::get_lease)\n\n"]
    pub fn op_queue_get_do(self) -> OpQueueGetDo<'buf> {
        let mut res = OpQueueGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-queue-get-do", OpQueueGetDo::lookup);
        res
    }
    #[doc = "Get information about NAPI instances configured on the system.\n\nRequest attributes:\n- [.push_ifindex()](PushNapi::push_ifindex)\n\nReply attributes:\n- [.get_ifindex()](IterableNapi::get_ifindex)\n- [.get_id()](IterableNapi::get_id)\n- [.get_irq()](IterableNapi::get_irq)\n- [.get_pid()](IterableNapi::get_pid)\n- [.get_defer_hard_irqs()](IterableNapi::get_defer_hard_irqs)\n- [.get_gro_flush_timeout()](IterableNapi::get_gro_flush_timeout)\n- [.get_irq_suspend_timeout()](IterableNapi::get_irq_suspend_timeout)\n- [.get_threaded()](IterableNapi::get_threaded)\n\n"]
    pub fn op_napi_get_dump(self) -> OpNapiGetDump<'buf> {
        let mut res = OpNapiGetDump::new(self);
        res.request
            .do_writeback(res.protocol(), "op-napi-get-dump", OpNapiGetDump::lookup);
        res
    }
    #[doc = "Get information about NAPI instances configured on the system.\n\nRequest attributes:\n- [.push_id()](PushNapi::push_id)\n\nReply attributes:\n- [.get_ifindex()](IterableNapi::get_ifindex)\n- [.get_id()](IterableNapi::get_id)\n- [.get_irq()](IterableNapi::get_irq)\n- [.get_pid()](IterableNapi::get_pid)\n- [.get_defer_hard_irqs()](IterableNapi::get_defer_hard_irqs)\n- [.get_gro_flush_timeout()](IterableNapi::get_gro_flush_timeout)\n- [.get_irq_suspend_timeout()](IterableNapi::get_irq_suspend_timeout)\n- [.get_threaded()](IterableNapi::get_threaded)\n\n"]
    pub fn op_napi_get_do(self) -> OpNapiGetDo<'buf> {
        let mut res = OpNapiGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-napi-get-do", OpNapiGetDo::lookup);
        res
    }
    #[doc = "Get / dump fine grained statistics. Which statistics are reported\ndepends on the device and the driver, and whether the driver stores\nsoftware counters per-queue.\n\nRequest attributes:\n- [.push_ifindex()](PushQstats::push_ifindex)\n- [.push_scope()](PushQstats::push_scope)\n\nReply attributes:\n- [.get_ifindex()](IterableQstats::get_ifindex)\n- [.get_queue_type()](IterableQstats::get_queue_type)\n- [.get_queue_id()](IterableQstats::get_queue_id)\n- [.get_rx_packets()](IterableQstats::get_rx_packets)\n- [.get_rx_bytes()](IterableQstats::get_rx_bytes)\n- [.get_tx_packets()](IterableQstats::get_tx_packets)\n- [.get_tx_bytes()](IterableQstats::get_tx_bytes)\n- [.get_rx_alloc_fail()](IterableQstats::get_rx_alloc_fail)\n- [.get_rx_hw_drops()](IterableQstats::get_rx_hw_drops)\n- [.get_rx_hw_drop_overruns()](IterableQstats::get_rx_hw_drop_overruns)\n- [.get_rx_csum_complete()](IterableQstats::get_rx_csum_complete)\n- [.get_rx_csum_unnecessary()](IterableQstats::get_rx_csum_unnecessary)\n- [.get_rx_csum_none()](IterableQstats::get_rx_csum_none)\n- [.get_rx_csum_bad()](IterableQstats::get_rx_csum_bad)\n- [.get_rx_hw_gro_packets()](IterableQstats::get_rx_hw_gro_packets)\n- [.get_rx_hw_gro_bytes()](IterableQstats::get_rx_hw_gro_bytes)\n- [.get_rx_hw_gro_wire_packets()](IterableQstats::get_rx_hw_gro_wire_packets)\n- [.get_rx_hw_gro_wire_bytes()](IterableQstats::get_rx_hw_gro_wire_bytes)\n- [.get_rx_hw_drop_ratelimits()](IterableQstats::get_rx_hw_drop_ratelimits)\n- [.get_tx_hw_drops()](IterableQstats::get_tx_hw_drops)\n- [.get_tx_hw_drop_errors()](IterableQstats::get_tx_hw_drop_errors)\n- [.get_tx_csum_none()](IterableQstats::get_tx_csum_none)\n- [.get_tx_needs_csum()](IterableQstats::get_tx_needs_csum)\n- [.get_tx_hw_gso_packets()](IterableQstats::get_tx_hw_gso_packets)\n- [.get_tx_hw_gso_bytes()](IterableQstats::get_tx_hw_gso_bytes)\n- [.get_tx_hw_gso_wire_packets()](IterableQstats::get_tx_hw_gso_wire_packets)\n- [.get_tx_hw_gso_wire_bytes()](IterableQstats::get_tx_hw_gso_wire_bytes)\n- [.get_tx_hw_drop_ratelimits()](IterableQstats::get_tx_hw_drop_ratelimits)\n- [.get_tx_stop()](IterableQstats::get_tx_stop)\n- [.get_tx_wake()](IterableQstats::get_tx_wake)\n\n"]
    pub fn op_qstats_get_dump(self) -> OpQstatsGetDump<'buf> {
        let mut res = OpQstatsGetDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-qstats-get-dump",
            OpQstatsGetDump::lookup,
        );
        res
    }
    #[doc = "Bind dmabuf to netdev\n\nFlags: uns-admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushDmabuf::push_ifindex)\n- [.nested_queues()](PushDmabuf::nested_queues)\n- [.push_fd()](PushDmabuf::push_fd)\n\nReply attributes:\n- [.get_id()](IterableDmabuf::get_id)\n\n"]
    pub fn op_bind_rx_do(self) -> OpBindRxDo<'buf> {
        let mut res = OpBindRxDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-bind-rx-do", OpBindRxDo::lookup);
        res
    }
    #[doc = "Set configurable NAPI instance settings.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_id()](PushNapi::push_id)\n- [.push_defer_hard_irqs()](PushNapi::push_defer_hard_irqs)\n- [.push_gro_flush_timeout()](PushNapi::push_gro_flush_timeout)\n- [.push_irq_suspend_timeout()](PushNapi::push_irq_suspend_timeout)\n- [.push_threaded()](PushNapi::push_threaded)\n\n"]
    pub fn op_napi_set_do(self) -> OpNapiSetDo<'buf> {
        let mut res = OpNapiSetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-napi-set-do", OpNapiSetDo::lookup);
        res
    }
    #[doc = "Bind dmabuf to netdev for TX\n\nRequest attributes:\n- [.push_ifindex()](PushDmabuf::push_ifindex)\n- [.push_fd()](PushDmabuf::push_fd)\n\nReply attributes:\n- [.get_id()](IterableDmabuf::get_id)\n\n"]
    pub fn op_bind_tx_do(self) -> OpBindTxDo<'buf> {
        let mut res = OpBindTxDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-bind-tx-do", OpBindTxDo::lookup);
        res
    }
    #[doc = "Create a new queue for the given netdevice. Whether this operation is\nsupported depends on the device and the driver.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushQueue::push_ifindex)\n- [.push_type()](PushQueue::push_type)\n- [.nested_lease()](PushQueue::nested_lease)\n\nReply attributes:\n- [.get_id()](IterableQueue::get_id)\n\n"]
    pub fn op_queue_create_do(self) -> OpQueueCreateDo<'buf> {
        let mut res = OpQueueCreateDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-queue-create-do",
            OpQueueCreateDo::lookup,
        );
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableDev::get_ifindex;
        let _ = IterableDev::get_xdp_features;
        let _ = IterableDev::get_xdp_rx_metadata_features;
        let _ = IterableDev::get_xdp_zc_max_segs;
        let _ = IterableDev::get_xsk_features;
        let _ = IterableDmabuf::get_id;
        let _ = IterableNapi::get_defer_hard_irqs;
        let _ = IterableNapi::get_gro_flush_timeout;
        let _ = IterableNapi::get_id;
        let _ = IterableNapi::get_ifindex;
        let _ = IterableNapi::get_irq;
        let _ = IterableNapi::get_irq_suspend_timeout;
        let _ = IterableNapi::get_pid;
        let _ = IterableNapi::get_threaded;
        let _ = IterablePagePool::get_detach_time;
        let _ = IterablePagePool::get_dmabuf;
        let _ = IterablePagePool::get_id;
        let _ = IterablePagePool::get_ifindex;
        let _ = IterablePagePool::get_inflight;
        let _ = IterablePagePool::get_inflight_mem;
        let _ = IterablePagePool::get_io_uring;
        let _ = IterablePagePool::get_napi_id;
        let _ = IterablePagePoolStats::get_alloc_empty;
        let _ = IterablePagePoolStats::get_alloc_fast;
        let _ = IterablePagePoolStats::get_alloc_refill;
        let _ = IterablePagePoolStats::get_alloc_slow;
        let _ = IterablePagePoolStats::get_alloc_slow_high_order;
        let _ = IterablePagePoolStats::get_alloc_waive;
        let _ = IterablePagePoolStats::get_info;
        let _ = IterablePagePoolStats::get_recycle_cache_full;
        let _ = IterablePagePoolStats::get_recycle_cached;
        let _ = IterablePagePoolStats::get_recycle_released_refcnt;
        let _ = IterablePagePoolStats::get_recycle_ring;
        let _ = IterablePagePoolStats::get_recycle_ring_full;
        let _ = IterableQstats::get_ifindex;
        let _ = IterableQstats::get_queue_id;
        let _ = IterableQstats::get_queue_type;
        let _ = IterableQstats::get_rx_alloc_fail;
        let _ = IterableQstats::get_rx_bytes;
        let _ = IterableQstats::get_rx_csum_bad;
        let _ = IterableQstats::get_rx_csum_complete;
        let _ = IterableQstats::get_rx_csum_none;
        let _ = IterableQstats::get_rx_csum_unnecessary;
        let _ = IterableQstats::get_rx_hw_drop_overruns;
        let _ = IterableQstats::get_rx_hw_drop_ratelimits;
        let _ = IterableQstats::get_rx_hw_drops;
        let _ = IterableQstats::get_rx_hw_gro_bytes;
        let _ = IterableQstats::get_rx_hw_gro_packets;
        let _ = IterableQstats::get_rx_hw_gro_wire_bytes;
        let _ = IterableQstats::get_rx_hw_gro_wire_packets;
        let _ = IterableQstats::get_rx_packets;
        let _ = IterableQstats::get_tx_bytes;
        let _ = IterableQstats::get_tx_csum_none;
        let _ = IterableQstats::get_tx_hw_drop_errors;
        let _ = IterableQstats::get_tx_hw_drop_ratelimits;
        let _ = IterableQstats::get_tx_hw_drops;
        let _ = IterableQstats::get_tx_hw_gso_bytes;
        let _ = IterableQstats::get_tx_hw_gso_packets;
        let _ = IterableQstats::get_tx_hw_gso_wire_bytes;
        let _ = IterableQstats::get_tx_hw_gso_wire_packets;
        let _ = IterableQstats::get_tx_needs_csum;
        let _ = IterableQstats::get_tx_packets;
        let _ = IterableQstats::get_tx_stop;
        let _ = IterableQstats::get_tx_wake;
        let _ = IterableQueue::get_dmabuf;
        let _ = IterableQueue::get_id;
        let _ = IterableQueue::get_ifindex;
        let _ = IterableQueue::get_io_uring;
        let _ = IterableQueue::get_lease;
        let _ = IterableQueue::get_napi_id;
        let _ = IterableQueue::get_type;
        let _ = IterableQueue::get_xsk;
        let _ = OpDevAddNotif;
        let _ = OpDevChangeNotif;
        let _ = OpDevDelNotif;
        let _ = OpPagePoolAddNotif;
        let _ = OpPagePoolChangeNotif;
        let _ = OpPagePoolDelNotif;
        let _ = PushDev::<&mut Vec<u8>>::push_ifindex;
        let _ = PushDmabuf::<&mut Vec<u8>>::nested_queues;
        let _ = PushDmabuf::<&mut Vec<u8>>::push_fd;
        let _ = PushDmabuf::<&mut Vec<u8>>::push_ifindex;
        let _ = PushNapi::<&mut Vec<u8>>::push_defer_hard_irqs;
        let _ = PushNapi::<&mut Vec<u8>>::push_gro_flush_timeout;
        let _ = PushNapi::<&mut Vec<u8>>::push_id;
        let _ = PushNapi::<&mut Vec<u8>>::push_ifindex;
        let _ = PushNapi::<&mut Vec<u8>>::push_irq_suspend_timeout;
        let _ = PushNapi::<&mut Vec<u8>>::push_threaded;
        let _ = PushPagePool::<&mut Vec<u8>>::push_id;
        let _ = PushPagePool::<&mut Vec<u8>>::push_ifindex;
        let _ = PushPagePoolStats::<&mut Vec<u8>>::nested_info;
        let _ = PushQstats::<&mut Vec<u8>>::push_ifindex;
        let _ = PushQstats::<&mut Vec<u8>>::push_scope;
        let _ = PushQueue::<&mut Vec<u8>>::nested_lease;
        let _ = PushQueue::<&mut Vec<u8>>::push_id;
        let _ = PushQueue::<&mut Vec<u8>>::push_ifindex;
        let _ = PushQueue::<&mut Vec<u8>>::push_type;
    }
}
