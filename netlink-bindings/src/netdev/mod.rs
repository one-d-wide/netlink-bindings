#![doc = "netdev configuration over generic netlink."]
#![allow(clippy::all)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]
#![allow(unreachable_code)]
#![allow(unreachable_patterns)]
use crate::builtin::{PushBuiltinBitfield32, PushBuiltinNfgenmsg, PushDummy, PushNlmsghdr};
use crate::{
    consts,
    traits::{NetlinkRequest, Protocol},
    utils::*,
};
pub const PROTONAME: &CStr = c"netdev";
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum XdpAct {
    #[doc = "XDP features set supported by all drivers (XDP_ABORTED, XDP_DROP, XDP_PASS, XDP_TX)"]
    Basic = 1 << 0,
    #[doc = "The netdev supports XDP_REDIRECT"]
    Redirect = 1 << 1,
    #[doc = "This feature informs if netdev implements ndo_xdp_xmit callback."]
    NdoXmit = 1 << 2,
    #[doc = "This feature informs if netdev supports AF_XDP in zero copy mode."]
    XskZerocopy = 1 << 3,
    #[doc = "This feature informs if netdev supports XDP hw offloading."]
    HwOffload = 1 << 4,
    #[doc = "This feature informs if netdev implements non-linear XDP buffer support in the driver napi callback."]
    RxSg = 1 << 5,
    #[doc = "This feature informs if netdev implements non-linear XDP buffer support in ndo_xdp_xmit callback."]
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
    #[doc = "HW timestamping egress packets is supported by the driver."]
    TxTimestamp = 1 << 0,
    #[doc = "L3 checksum HW offload is supported by the driver."]
    TxChecksum = 1 << 1,
    #[doc = "Launch time HW offload is supported by the driver."]
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
    #[doc = "netdev ifindex"]
    Ifindex(u32),
    Pad(&'a [u8]),
    #[doc = "Bitmask of enabled xdp-features.\nAssociated type: \"XdpAct\" (enum)"]
    XdpFeatures(u64),
    #[doc = "max fragment count supported by ZC driver"]
    XdpZcMaxSegs(u32),
    #[doc = "Bitmask of supported XDP receive metadata features. See Documentation/networking/xdp-rx-metadata.rst for more details.\nAssociated type: \"XdpRxMetadata\" (enum)"]
    XdpRxMetadataFeatures(u64),
    #[doc = "Bitmask of enabled AF_XDP features.\nAssociated type: \"XskFlags\" (enum)"]
    XskFeatures(u64),
}
impl<'a> IterableDev<'a> {
    #[doc = "netdev ifindex"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Dev::Ifindex(val) = attr? {
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
            if let Dev::Pad(val) = attr? {
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
    #[doc = "Bitmask of enabled xdp-features.\nAssociated type: \"XdpAct\" (enum)"]
    pub fn get_xdp_features(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Dev::XdpFeatures(val) = attr? {
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
    #[doc = "max fragment count supported by ZC driver"]
    pub fn get_xdp_zc_max_segs(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Dev::XdpZcMaxSegs(val) = attr? {
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
    #[doc = "Bitmask of supported XDP receive metadata features. See Documentation/networking/xdp-rx-metadata.rst for more details.\nAssociated type: \"XdpRxMetadata\" (enum)"]
    pub fn get_xdp_rx_metadata_features(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Dev::XdpRxMetadataFeatures(val) = attr? {
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
    #[doc = "Bitmask of enabled AF_XDP features.\nAssociated type: \"XskFlags\" (enum)"]
    pub fn get_xsk_features(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Dev::XskFeatures(val) = attr? {
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
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
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
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
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
        if cur == offset {
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
pub enum IoUringProviderInfo {}
impl<'a> IterableIoUringProviderInfo<'a> {}
impl IoUringProviderInfo {
    pub fn new<'a>(buf: &'a [u8]) -> IterableIoUringProviderInfo<'a> {
        IterableIoUringProviderInfo::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        None
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
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
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
impl IterableIoUringProviderInfo<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("IoUringProviderInfo", offset));
            return (
                stack,
                missing_type.and_then(|t| IoUringProviderInfo::attr_from_type(t)),
            );
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum PagePool<'a> {
    #[doc = "Unique ID of a Page Pool instance."]
    Id(u32),
    #[doc = "ifindex of the netdev to which the pool belongs.\nMay be reported as 0 if the page pool was allocated for a netdev\nwhich got destroyed already (page pools may outlast their netdevs\nbecause they wait for all memory to be returned).\n"]
    Ifindex(u32),
    #[doc = "Id of NAPI using this Page Pool instance."]
    NapiId(u32),
    #[doc = "Number of outstanding references to this page pool (allocated\nbut yet to be freed pages). Allocated pages may be held in\nsocket receive queues, driver receive ring, page pool recycling\nring, the page pool cache, etc.\n"]
    Inflight(u32),
    #[doc = "Amount of memory held by inflight pages.\n"]
    InflightMem(u32),
    #[doc = "Seconds in CLOCK_BOOTTIME of when Page Pool was detached by\nthe driver. Once detached Page Pool can no longer be used to\nallocate memory.\nPage Pools wait for all the memory allocated from them to be freed\nbefore truly disappearing. \"Detached\" Page Pools cannot be\n\"re-attached\", they are just waiting to disappear.\nAttribute is absent if Page Pool has not been detached, and\ncan still be used to allocate new memory.\n"]
    DetachTime(u32),
    #[doc = "ID of the dmabuf this page-pool is attached to."]
    Dmabuf(u32),
    #[doc = "io-uring memory provider information."]
    IoUring(IterableIoUringProviderInfo<'a>),
}
impl<'a> IterablePagePool<'a> {
    #[doc = "Unique ID of a Page Pool instance."]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PagePool::Id(val) = attr? {
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
    #[doc = "ifindex of the netdev to which the pool belongs.\nMay be reported as 0 if the page pool was allocated for a netdev\nwhich got destroyed already (page pools may outlast their netdevs\nbecause they wait for all memory to be returned).\n"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PagePool::Ifindex(val) = attr? {
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
    #[doc = "Id of NAPI using this Page Pool instance."]
    pub fn get_napi_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PagePool::NapiId(val) = attr? {
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
    #[doc = "Number of outstanding references to this page pool (allocated\nbut yet to be freed pages). Allocated pages may be held in\nsocket receive queues, driver receive ring, page pool recycling\nring, the page pool cache, etc.\n"]
    pub fn get_inflight(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PagePool::Inflight(val) = attr? {
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
            if let PagePool::InflightMem(val) = attr? {
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
    #[doc = "Seconds in CLOCK_BOOTTIME of when Page Pool was detached by\nthe driver. Once detached Page Pool can no longer be used to\nallocate memory.\nPage Pools wait for all the memory allocated from them to be freed\nbefore truly disappearing. \"Detached\" Page Pools cannot be\n\"re-attached\", they are just waiting to disappear.\nAttribute is absent if Page Pool has not been detached, and\ncan still be used to allocate new memory.\n"]
    pub fn get_detach_time(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PagePool::DetachTime(val) = attr? {
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
    #[doc = "ID of the dmabuf this page-pool is attached to."]
    pub fn get_dmabuf(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PagePool::Dmabuf(val) = attr? {
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
    #[doc = "io-uring memory provider information."]
    pub fn get_io_uring(&self) -> Result<IterableIoUringProviderInfo<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PagePool::IoUring(val) = attr? {
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
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
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
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
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
        if cur == offset {
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
    #[doc = "Unique ID of a Page Pool instance."]
    Id(u32),
    #[doc = "ifindex of the netdev to which the pool belongs.\nMay be reported as 0 if the page pool was allocated for a netdev\nwhich got destroyed already (page pools may outlast their netdevs\nbecause they wait for all memory to be returned).\n"]
    Ifindex(u32),
}
impl<'a> IterablePagePoolInfo<'a> {
    #[doc = "Unique ID of a Page Pool instance."]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PagePoolInfo::Id(val) = attr? {
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
    #[doc = "ifindex of the netdev to which the pool belongs.\nMay be reported as 0 if the page pool was allocated for a netdev\nwhich got destroyed already (page pools may outlast their netdevs\nbecause they wait for all memory to be returned).\n"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PagePoolInfo::Ifindex(val) = attr? {
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
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
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
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
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
        if cur == offset {
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
    #[doc = "Page pool identifying information."]
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
    #[doc = "Page pool identifying information."]
    pub fn get_info(&self) -> Result<IterablePagePoolInfo<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PagePoolStats::Info(val) = attr? {
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
            if let PagePoolStats::AllocFast(val) = attr? {
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
            if let PagePoolStats::AllocSlow(val) = attr? {
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
            if let PagePoolStats::AllocSlowHighOrder(val) = attr? {
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
            if let PagePoolStats::AllocEmpty(val) = attr? {
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
            if let PagePoolStats::AllocRefill(val) = attr? {
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
            if let PagePoolStats::AllocWaive(val) = attr? {
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
            if let PagePoolStats::RecycleCached(val) = attr? {
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
            if let PagePoolStats::RecycleCacheFull(val) = attr? {
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
            if let PagePoolStats::RecycleRing(val) = attr? {
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
            if let PagePoolStats::RecycleRingFull(val) = attr? {
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
            if let PagePoolStats::RecycleReleasedRefcnt(val) = attr? {
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
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
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
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
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
        if cur == offset {
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
    #[doc = "ifindex of the netdevice to which NAPI instance belongs."]
    Ifindex(u32),
    #[doc = "ID of the NAPI instance."]
    Id(u32),
    #[doc = "The associated interrupt vector number for the napi"]
    Irq(u32),
    #[doc = "PID of the napi thread, if NAPI is configured to operate in threaded mode. If NAPI is not in threaded mode (i.e. uses normal softirq context), the attribute will be absent."]
    Pid(u32),
    #[doc = "The number of consecutive empty polls before IRQ deferral ends and hardware IRQs are re-enabled."]
    DeferHardIrqs(u32),
    #[doc = "The timeout, in nanoseconds, of when to trigger the NAPI watchdog timer which schedules NAPI processing. Additionally, a non-zero value will also prevent GRO from flushing recent super-frames at the end of a NAPI cycle. This may add receive latency in exchange for reducing the number of frames processed by the network stack."]
    GroFlushTimeout(u32),
    #[doc = "The timeout, in nanoseconds, of how long to suspend irq processing, if event polling finds events"]
    IrqSuspendTimeout(u32),
    #[doc = "Whether the NAPI is configured to operate in threaded polling mode. If this is set to enabled then the NAPI context operates in threaded polling mode. If this is set to busy-poll, then the threaded polling mode also busy polls.\nAssociated type: \"NapiThreaded\" (enum)"]
    Threaded(u32),
}
impl<'a> IterableNapi<'a> {
    #[doc = "ifindex of the netdevice to which NAPI instance belongs."]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Napi::Ifindex(val) = attr? {
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
    #[doc = "ID of the NAPI instance."]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Napi::Id(val) = attr? {
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
    #[doc = "The associated interrupt vector number for the napi"]
    pub fn get_irq(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Napi::Irq(val) = attr? {
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
    #[doc = "PID of the napi thread, if NAPI is configured to operate in threaded mode. If NAPI is not in threaded mode (i.e. uses normal softirq context), the attribute will be absent."]
    pub fn get_pid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Napi::Pid(val) = attr? {
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
    #[doc = "The number of consecutive empty polls before IRQ deferral ends and hardware IRQs are re-enabled."]
    pub fn get_defer_hard_irqs(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Napi::DeferHardIrqs(val) = attr? {
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
    #[doc = "The timeout, in nanoseconds, of when to trigger the NAPI watchdog timer which schedules NAPI processing. Additionally, a non-zero value will also prevent GRO from flushing recent super-frames at the end of a NAPI cycle. This may add receive latency in exchange for reducing the number of frames processed by the network stack."]
    pub fn get_gro_flush_timeout(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Napi::GroFlushTimeout(val) = attr? {
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
    #[doc = "The timeout, in nanoseconds, of how long to suspend irq processing, if event polling finds events"]
    pub fn get_irq_suspend_timeout(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Napi::IrqSuspendTimeout(val) = attr? {
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
    #[doc = "Whether the NAPI is configured to operate in threaded polling mode. If this is set to enabled then the NAPI context operates in threaded polling mode. If this is set to busy-poll, then the threaded polling mode also busy polls.\nAssociated type: \"NapiThreaded\" (enum)"]
    pub fn get_threaded(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Napi::Threaded(val) = attr? {
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
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
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
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
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
        if cur == offset {
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
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
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
impl IterableXskInfo<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("XskInfo", offset));
            return (stack, missing_type.and_then(|t| XskInfo::attr_from_type(t)));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum Queue<'a> {
    #[doc = "Queue index; most queue types are indexed like a C array, with indexes starting at 0 and ending at queue count - 1. Queue indexes are scoped to an interface and queue type."]
    Id(u32),
    #[doc = "ifindex of the netdevice to which the queue belongs."]
    Ifindex(u32),
    #[doc = "Queue type as rx, tx. Each queue type defines a separate ID space. XDP TX queues allocated in the kernel are not linked to NAPIs and thus not listed. AF_XDP queues will have more information set in the xsk attribute.\nAssociated type: \"QueueType\" (enum)"]
    Type(u32),
    #[doc = "ID of the NAPI instance which services this queue."]
    NapiId(u32),
    #[doc = "ID of the dmabuf attached to this queue, if any."]
    Dmabuf(u32),
    #[doc = "io_uring memory provider information."]
    IoUring(IterableIoUringProviderInfo<'a>),
    #[doc = "XSK information for this queue, if any."]
    Xsk(IterableXskInfo<'a>),
}
impl<'a> IterableQueue<'a> {
    #[doc = "Queue index; most queue types are indexed like a C array, with indexes starting at 0 and ending at queue count - 1. Queue indexes are scoped to an interface and queue type."]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Queue::Id(val) = attr? {
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
    #[doc = "ifindex of the netdevice to which the queue belongs."]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Queue::Ifindex(val) = attr? {
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
    #[doc = "Queue type as rx, tx. Each queue type defines a separate ID space. XDP TX queues allocated in the kernel are not linked to NAPIs and thus not listed. AF_XDP queues will have more information set in the xsk attribute.\nAssociated type: \"QueueType\" (enum)"]
    pub fn get_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Queue::Type(val) = attr? {
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
    #[doc = "ID of the NAPI instance which services this queue."]
    pub fn get_napi_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Queue::NapiId(val) = attr? {
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
    #[doc = "ID of the dmabuf attached to this queue, if any."]
    pub fn get_dmabuf(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Queue::Dmabuf(val) = attr? {
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
    #[doc = "io_uring memory provider information."]
    pub fn get_io_uring(&self) -> Result<IterableIoUringProviderInfo<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Queue::IoUring(val) = attr? {
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
    #[doc = "XSK information for this queue, if any."]
    pub fn get_xsk(&self) -> Result<IterableXskInfo<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Queue::Xsk(val) = attr? {
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
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
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
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
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
                Queue::Id(val) => fmt.field("Id", &val),
                Queue::Ifindex(val) => fmt.field("Ifindex", &val),
                Queue::Type(val) => {
                    fmt.field("Type", &FormatEnum(val.into(), QueueType::from_value))
                }
                Queue::NapiId(val) => fmt.field("NapiId", &val),
                Queue::Dmabuf(val) => fmt.field("Dmabuf", &val),
                Queue::IoUring(val) => fmt.field("IoUring", &val),
                Queue::Xsk(val) => fmt.field("Xsk", &val),
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
        if cur == offset {
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
    #[doc = "ifindex of the netdevice to which stats belong."]
    Ifindex(u32),
    #[doc = "Queue type as rx, tx, for queue-id.\nAssociated type: \"QueueType\" (enum)"]
    QueueType(u32),
    #[doc = "Queue ID, if stats are scoped to a single queue instance."]
    QueueId(u32),
    #[doc = "What object type should be used to iterate over the stats.\n\nAssociated type: \"QstatsScope\" (enum)"]
    Scope(u32),
    #[doc = "Number of wire packets successfully received and passed to the stack.\nFor drivers supporting XDP, XDP is considered the first layer\nof the stack, so packets consumed by XDP are still counted here.\n"]
    RxPackets(u32),
    #[doc = "Successfully received bytes, see `rx-packets`."]
    RxBytes(u32),
    #[doc = "Number of wire packets successfully sent. Packet is considered to be\nsuccessfully sent once it is in device memory (usually this means\nthe device has issued a DMA completion for the packet).\n"]
    TxPackets(u32),
    #[doc = "Successfully sent bytes, see `tx-packets`."]
    TxBytes(u32),
    #[doc = "Number of times skb or buffer allocation failed on the Rx datapath.\nAllocation failure may, or may not result in a packet drop, depending\non driver implementation and whether system recovers quickly.\n"]
    RxAllocFail(u32),
    #[doc = "Number of all packets which entered the device, but never left it,\nincluding but not limited to: packets dropped due to lack of buffer\nspace, processing errors, explicit or implicit policies and packet\nfilters.\n"]
    RxHwDrops(u32),
    #[doc = "Number of packets dropped due to transient lack of resources, such as\nbuffer space, host descriptors etc.\n"]
    RxHwDropOverruns(u32),
    #[doc = "Number of packets that were marked as CHECKSUM_COMPLETE."]
    RxCsumComplete(u32),
    #[doc = "Number of packets that were marked as CHECKSUM_UNNECESSARY."]
    RxCsumUnnecessary(u32),
    #[doc = "Number of packets that were not checksummed by device."]
    RxCsumNone(u32),
    #[doc = "Number of packets with bad checksum. The packets are not discarded,\nbut still delivered to the stack.\n"]
    RxCsumBad(u32),
    #[doc = "Number of packets that were coalesced from smaller packets by the\ndevice. Counts only packets coalesced with the HW-GRO netdevice\nfeature, LRO-coalesced packets are not counted.\n"]
    RxHwGroPackets(u32),
    #[doc = "See `rx-hw-gro-packets`."]
    RxHwGroBytes(u32),
    #[doc = "Number of packets that were coalesced to bigger packetss with the\nHW-GRO netdevice feature. LRO-coalesced packets are not counted.\n"]
    RxHwGroWirePackets(u32),
    #[doc = "See `rx-hw-gro-wire-packets`."]
    RxHwGroWireBytes(u32),
    #[doc = "Number of the packets dropped by the device due to the received\npackets bitrate exceeding the device rate limit.\n"]
    RxHwDropRatelimits(u32),
    #[doc = "Number of packets that arrived at the device but never left it,\nencompassing packets dropped for reasons such as processing errors, as\nwell as those affected by explicitly defined policies and packet\nfiltering criteria.\n"]
    TxHwDrops(u32),
    #[doc = "Number of packets dropped because they were invalid or malformed."]
    TxHwDropErrors(u32),
    #[doc = "Number of packets that did not require the device to calculate the\nchecksum.\n"]
    TxCsumNone(u32),
    #[doc = "Number of packets that required the device to calculate the checksum.\nThis counter includes the number of GSO wire packets for which device\ncalculated the L4 checksum.\n"]
    TxNeedsCsum(u32),
    #[doc = "Number of packets that necessitated segmentation into smaller packets\nby the device.\n"]
    TxHwGsoPackets(u32),
    #[doc = "See `tx-hw-gso-packets`."]
    TxHwGsoBytes(u32),
    #[doc = "Number of wire-sized packets generated by processing\n`tx-hw-gso-packets`\n"]
    TxHwGsoWirePackets(u32),
    #[doc = "See `tx-hw-gso-wire-packets`."]
    TxHwGsoWireBytes(u32),
    #[doc = "Number of the packets dropped by the device due to the transmit\npackets bitrate exceeding the device rate limit.\n"]
    TxHwDropRatelimits(u32),
    #[doc = "Number of times driver paused accepting new tx packets\nfrom the stack to this queue, because the queue was full.\nNote that if BQL is supported and enabled on the device\nthe networking stack will avoid queuing a lot of data at once.\n"]
    TxStop(u32),
    #[doc = "Number of times driver re-started accepting send\nrequests to this queue from the stack.\n"]
    TxWake(u32),
}
impl<'a> IterableQstats<'a> {
    #[doc = "ifindex of the netdevice to which stats belong."]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::Ifindex(val) = attr? {
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
    #[doc = "Queue type as rx, tx, for queue-id.\nAssociated type: \"QueueType\" (enum)"]
    pub fn get_queue_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::QueueType(val) = attr? {
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
    #[doc = "Queue ID, if stats are scoped to a single queue instance."]
    pub fn get_queue_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::QueueId(val) = attr? {
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
    #[doc = "What object type should be used to iterate over the stats.\n\nAssociated type: \"QstatsScope\" (enum)"]
    pub fn get_scope(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::Scope(val) = attr? {
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
    #[doc = "Number of wire packets successfully received and passed to the stack.\nFor drivers supporting XDP, XDP is considered the first layer\nof the stack, so packets consumed by XDP are still counted here.\n"]
    pub fn get_rx_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::RxPackets(val) = attr? {
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
    #[doc = "Successfully received bytes, see `rx-packets`."]
    pub fn get_rx_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::RxBytes(val) = attr? {
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
    #[doc = "Number of wire packets successfully sent. Packet is considered to be\nsuccessfully sent once it is in device memory (usually this means\nthe device has issued a DMA completion for the packet).\n"]
    pub fn get_tx_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::TxPackets(val) = attr? {
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
    #[doc = "Successfully sent bytes, see `tx-packets`."]
    pub fn get_tx_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::TxBytes(val) = attr? {
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
    #[doc = "Number of times skb or buffer allocation failed on the Rx datapath.\nAllocation failure may, or may not result in a packet drop, depending\non driver implementation and whether system recovers quickly.\n"]
    pub fn get_rx_alloc_fail(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::RxAllocFail(val) = attr? {
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
            if let Qstats::RxHwDrops(val) = attr? {
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
            if let Qstats::RxHwDropOverruns(val) = attr? {
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
    #[doc = "Number of packets that were marked as CHECKSUM_COMPLETE."]
    pub fn get_rx_csum_complete(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::RxCsumComplete(val) = attr? {
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
    #[doc = "Number of packets that were marked as CHECKSUM_UNNECESSARY."]
    pub fn get_rx_csum_unnecessary(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::RxCsumUnnecessary(val) = attr? {
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
    #[doc = "Number of packets that were not checksummed by device."]
    pub fn get_rx_csum_none(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::RxCsumNone(val) = attr? {
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
    #[doc = "Number of packets with bad checksum. The packets are not discarded,\nbut still delivered to the stack.\n"]
    pub fn get_rx_csum_bad(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::RxCsumBad(val) = attr? {
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
    #[doc = "Number of packets that were coalesced from smaller packets by the\ndevice. Counts only packets coalesced with the HW-GRO netdevice\nfeature, LRO-coalesced packets are not counted.\n"]
    pub fn get_rx_hw_gro_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::RxHwGroPackets(val) = attr? {
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
    #[doc = "See `rx-hw-gro-packets`."]
    pub fn get_rx_hw_gro_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::RxHwGroBytes(val) = attr? {
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
    #[doc = "Number of packets that were coalesced to bigger packetss with the\nHW-GRO netdevice feature. LRO-coalesced packets are not counted.\n"]
    pub fn get_rx_hw_gro_wire_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::RxHwGroWirePackets(val) = attr? {
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
    #[doc = "See `rx-hw-gro-wire-packets`."]
    pub fn get_rx_hw_gro_wire_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::RxHwGroWireBytes(val) = attr? {
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
    #[doc = "Number of the packets dropped by the device due to the received\npackets bitrate exceeding the device rate limit.\n"]
    pub fn get_rx_hw_drop_ratelimits(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::RxHwDropRatelimits(val) = attr? {
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
            if let Qstats::TxHwDrops(val) = attr? {
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
    #[doc = "Number of packets dropped because they were invalid or malformed."]
    pub fn get_tx_hw_drop_errors(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::TxHwDropErrors(val) = attr? {
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
            if let Qstats::TxCsumNone(val) = attr? {
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
            if let Qstats::TxNeedsCsum(val) = attr? {
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
    #[doc = "Number of packets that necessitated segmentation into smaller packets\nby the device.\n"]
    pub fn get_tx_hw_gso_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::TxHwGsoPackets(val) = attr? {
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
    #[doc = "See `tx-hw-gso-packets`."]
    pub fn get_tx_hw_gso_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::TxHwGsoBytes(val) = attr? {
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
    #[doc = "Number of wire-sized packets generated by processing\n`tx-hw-gso-packets`\n"]
    pub fn get_tx_hw_gso_wire_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::TxHwGsoWirePackets(val) = attr? {
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
    #[doc = "See `tx-hw-gso-wire-packets`."]
    pub fn get_tx_hw_gso_wire_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::TxHwGsoWireBytes(val) = attr? {
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
    #[doc = "Number of the packets dropped by the device due to the transmit\npackets bitrate exceeding the device rate limit.\n"]
    pub fn get_tx_hw_drop_ratelimits(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::TxHwDropRatelimits(val) = attr? {
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
    #[doc = "Number of times driver paused accepting new tx packets\nfrom the stack to this queue, because the queue was full.\nNote that if BQL is supported and enabled on the device\nthe networking stack will avoid queuing a lot of data at once.\n"]
    pub fn get_tx_stop(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::TxStop(val) = attr? {
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
    #[doc = "Number of times driver re-started accepting send\nrequests to this queue from the stack.\n"]
    pub fn get_tx_wake(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Qstats::TxWake(val) = attr? {
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
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
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
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
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
        if cur == offset {
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
    #[doc = "Queue index; most queue types are indexed like a C array, with indexes starting at 0 and ending at queue count - 1. Queue indexes are scoped to an interface and queue type."]
    Id(u32),
    #[doc = "Queue type as rx, tx. Each queue type defines a separate ID space. XDP TX queues allocated in the kernel are not linked to NAPIs and thus not listed. AF_XDP queues will have more information set in the xsk attribute.\nAssociated type: \"QueueType\" (enum)"]
    Type(u32),
}
impl<'a> IterableQueueId<'a> {
    #[doc = "Queue index; most queue types are indexed like a C array, with indexes starting at 0 and ending at queue count - 1. Queue indexes are scoped to an interface and queue type."]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let QueueId::Id(val) = attr? {
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
    #[doc = "Queue type as rx, tx. Each queue type defines a separate ID space. XDP TX queues allocated in the kernel are not linked to NAPIs and thus not listed. AF_XDP queues will have more information set in the xsk attribute.\nAssociated type: \"QueueType\" (enum)"]
    pub fn get_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let QueueId::Type(val) = attr? {
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
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
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
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
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
        if cur == offset {
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
pub enum Dmabuf<'a> {
    #[doc = "netdev ifindex to bind the dmabuf to."]
    Ifindex(u32),
    #[doc = "receive queues to bind the dmabuf to.\nAttribute may repeat multiple times (treat it as array)"]
    Queues(IterableQueueId<'a>),
    #[doc = "dmabuf file descriptor to bind."]
    Fd(u32),
    #[doc = "id of the dmabuf binding"]
    Id(u32),
}
impl<'a> IterableDmabuf<'a> {
    #[doc = "netdev ifindex to bind the dmabuf to."]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Dmabuf::Ifindex(val) = attr? {
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
    #[doc = "receive queues to bind the dmabuf to.\nAttribute may repeat multiple times (treat it as array)"]
    pub fn get_queues(&self) -> MultiAttrIterable<Self, Dmabuf<'a>, IterableQueueId<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Dmabuf::Queues(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    #[doc = "dmabuf file descriptor to bind."]
    pub fn get_fd(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Dmabuf::Fd(val) = attr? {
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
    #[doc = "id of the dmabuf binding"]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Dmabuf::Id(val) = attr? {
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
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
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
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
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
        if cur == offset {
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
pub struct PushDev<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushDev<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushDev<Prev> {
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
    #[doc = "netdev ifindex"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    #[doc = "Bitmask of enabled xdp-features.\nAssociated type: \"XdpAct\" (enum)"]
    pub fn push_xdp_features(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 3u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "max fragment count supported by ZC driver"]
    pub fn push_xdp_zc_max_segs(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Bitmask of supported XDP receive metadata features. See Documentation/networking/xdp-rx-metadata.rst for more details.\nAssociated type: \"XdpRxMetadata\" (enum)"]
    pub fn push_xdp_rx_metadata_features(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 5u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Bitmask of enabled AF_XDP features.\nAssociated type: \"XskFlags\" (enum)"]
    pub fn push_xsk_features(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 6u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushDev<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushIoUringProviderInfo<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushIoUringProviderInfo<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushIoUringProviderInfo<Prev> {
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
}
impl<Prev: Rec> Drop for PushIoUringProviderInfo<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPagePool<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushPagePool<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushPagePool<Prev> {
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
    #[doc = "Unique ID of a Page Pool instance."]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ifindex of the netdev to which the pool belongs.\nMay be reported as 0 if the page pool was allocated for a netdev\nwhich got destroyed already (page pools may outlast their netdevs\nbecause they wait for all memory to be returned).\n"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Id of NAPI using this Page Pool instance."]
    pub fn push_napi_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of outstanding references to this page pool (allocated\nbut yet to be freed pages). Allocated pages may be held in\nsocket receive queues, driver receive ring, page pool recycling\nring, the page pool cache, etc.\n"]
    pub fn push_inflight(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Amount of memory held by inflight pages.\n"]
    pub fn push_inflight_mem(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 5u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Seconds in CLOCK_BOOTTIME of when Page Pool was detached by\nthe driver. Once detached Page Pool can no longer be used to\nallocate memory.\nPage Pools wait for all the memory allocated from them to be freed\nbefore truly disappearing. \"Detached\" Page Pools cannot be\n\"re-attached\", they are just waiting to disappear.\nAttribute is absent if Page Pool has not been detached, and\ncan still be used to allocate new memory.\n"]
    pub fn push_detach_time(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 6u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ID of the dmabuf this page-pool is attached to."]
    pub fn push_dmabuf(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 7u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "io-uring memory provider information."]
    pub fn nested_io_uring(mut self) -> PushIoUringProviderInfo<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 8u16);
        PushIoUringProviderInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushPagePool<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPagePoolInfo<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushPagePoolInfo<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushPagePoolInfo<Prev> {
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
    #[doc = "Unique ID of a Page Pool instance."]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ifindex of the netdev to which the pool belongs.\nMay be reported as 0 if the page pool was allocated for a netdev\nwhich got destroyed already (page pools may outlast their netdevs\nbecause they wait for all memory to be returned).\n"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushPagePoolInfo<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPagePoolStats<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushPagePoolStats<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushPagePoolStats<Prev> {
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
    #[doc = "Page pool identifying information."]
    pub fn nested_info(mut self) -> PushPagePoolInfo<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 1u16);
        PushPagePoolInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_alloc_fast(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_alloc_slow(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 9u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_alloc_slow_high_order(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 10u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_alloc_empty(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 11u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_alloc_refill(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 12u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_alloc_waive(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 13u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_recycle_cached(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 14u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_recycle_cache_full(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 15u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_recycle_ring(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 16u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_recycle_ring_full(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 17u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_recycle_released_refcnt(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 18u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushPagePoolStats<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushNapi<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushNapi<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushNapi<Prev> {
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
    #[doc = "ifindex of the netdevice to which NAPI instance belongs."]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ID of the NAPI instance."]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The associated interrupt vector number for the napi"]
    pub fn push_irq(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "PID of the napi thread, if NAPI is configured to operate in threaded mode. If NAPI is not in threaded mode (i.e. uses normal softirq context), the attribute will be absent."]
    pub fn push_pid(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The number of consecutive empty polls before IRQ deferral ends and hardware IRQs are re-enabled."]
    pub fn push_defer_hard_irqs(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 5u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The timeout, in nanoseconds, of when to trigger the NAPI watchdog timer which schedules NAPI processing. Additionally, a non-zero value will also prevent GRO from flushing recent super-frames at the end of a NAPI cycle. This may add receive latency in exchange for reducing the number of frames processed by the network stack."]
    pub fn push_gro_flush_timeout(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 6u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The timeout, in nanoseconds, of how long to suspend irq processing, if event polling finds events"]
    pub fn push_irq_suspend_timeout(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 7u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether the NAPI is configured to operate in threaded polling mode. If this is set to enabled then the NAPI context operates in threaded polling mode. If this is set to busy-poll, then the threaded polling mode also busy polls.\nAssociated type: \"NapiThreaded\" (enum)"]
    pub fn push_threaded(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushNapi<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushXskInfo<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushXskInfo<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushXskInfo<Prev> {
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
}
impl<Prev: Rec> Drop for PushXskInfo<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushQueue<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushQueue<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushQueue<Prev> {
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
    #[doc = "Queue index; most queue types are indexed like a C array, with indexes starting at 0 and ending at queue count - 1. Queue indexes are scoped to an interface and queue type."]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ifindex of the netdevice to which the queue belongs."]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Queue type as rx, tx. Each queue type defines a separate ID space. XDP TX queues allocated in the kernel are not linked to NAPIs and thus not listed. AF_XDP queues will have more information set in the xsk attribute.\nAssociated type: \"QueueType\" (enum)"]
    pub fn push_type(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ID of the NAPI instance which services this queue."]
    pub fn push_napi_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ID of the dmabuf attached to this queue, if any."]
    pub fn push_dmabuf(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 5u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "io_uring memory provider information."]
    pub fn nested_io_uring(mut self) -> PushIoUringProviderInfo<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 6u16);
        PushIoUringProviderInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "XSK information for this queue, if any."]
    pub fn nested_xsk(mut self) -> PushXskInfo<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 7u16);
        PushXskInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushQueue<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushQstats<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushQstats<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushQstats<Prev> {
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
    #[doc = "ifindex of the netdevice to which stats belong."]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Queue type as rx, tx, for queue-id.\nAssociated type: \"QueueType\" (enum)"]
    pub fn push_queue_type(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Queue ID, if stats are scoped to a single queue instance."]
    pub fn push_queue_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "What object type should be used to iterate over the stats.\n\nAssociated type: \"QstatsScope\" (enum)"]
    pub fn push_scope(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of wire packets successfully received and passed to the stack.\nFor drivers supporting XDP, XDP is considered the first layer\nof the stack, so packets consumed by XDP are still counted here.\n"]
    pub fn push_rx_packets(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Successfully received bytes, see `rx-packets`."]
    pub fn push_rx_bytes(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 9u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of wire packets successfully sent. Packet is considered to be\nsuccessfully sent once it is in device memory (usually this means\nthe device has issued a DMA completion for the packet).\n"]
    pub fn push_tx_packets(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 10u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Successfully sent bytes, see `tx-packets`."]
    pub fn push_tx_bytes(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 11u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of times skb or buffer allocation failed on the Rx datapath.\nAllocation failure may, or may not result in a packet drop, depending\non driver implementation and whether system recovers quickly.\n"]
    pub fn push_rx_alloc_fail(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 12u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of all packets which entered the device, but never left it,\nincluding but not limited to: packets dropped due to lack of buffer\nspace, processing errors, explicit or implicit policies and packet\nfilters.\n"]
    pub fn push_rx_hw_drops(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 13u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets dropped due to transient lack of resources, such as\nbuffer space, host descriptors etc.\n"]
    pub fn push_rx_hw_drop_overruns(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 14u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that were marked as CHECKSUM_COMPLETE."]
    pub fn push_rx_csum_complete(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 15u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that were marked as CHECKSUM_UNNECESSARY."]
    pub fn push_rx_csum_unnecessary(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 16u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that were not checksummed by device."]
    pub fn push_rx_csum_none(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 17u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets with bad checksum. The packets are not discarded,\nbut still delivered to the stack.\n"]
    pub fn push_rx_csum_bad(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 18u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that were coalesced from smaller packets by the\ndevice. Counts only packets coalesced with the HW-GRO netdevice\nfeature, LRO-coalesced packets are not counted.\n"]
    pub fn push_rx_hw_gro_packets(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 19u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "See `rx-hw-gro-packets`."]
    pub fn push_rx_hw_gro_bytes(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 20u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that were coalesced to bigger packetss with the\nHW-GRO netdevice feature. LRO-coalesced packets are not counted.\n"]
    pub fn push_rx_hw_gro_wire_packets(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 21u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "See `rx-hw-gro-wire-packets`."]
    pub fn push_rx_hw_gro_wire_bytes(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 22u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of the packets dropped by the device due to the received\npackets bitrate exceeding the device rate limit.\n"]
    pub fn push_rx_hw_drop_ratelimits(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 23u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that arrived at the device but never left it,\nencompassing packets dropped for reasons such as processing errors, as\nwell as those affected by explicitly defined policies and packet\nfiltering criteria.\n"]
    pub fn push_tx_hw_drops(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 24u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets dropped because they were invalid or malformed."]
    pub fn push_tx_hw_drop_errors(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 25u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that did not require the device to calculate the\nchecksum.\n"]
    pub fn push_tx_csum_none(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 26u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that required the device to calculate the checksum.\nThis counter includes the number of GSO wire packets for which device\ncalculated the L4 checksum.\n"]
    pub fn push_tx_needs_csum(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 27u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that necessitated segmentation into smaller packets\nby the device.\n"]
    pub fn push_tx_hw_gso_packets(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 28u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "See `tx-hw-gso-packets`."]
    pub fn push_tx_hw_gso_bytes(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 29u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of wire-sized packets generated by processing\n`tx-hw-gso-packets`\n"]
    pub fn push_tx_hw_gso_wire_packets(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 30u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "See `tx-hw-gso-wire-packets`."]
    pub fn push_tx_hw_gso_wire_bytes(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 31u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of the packets dropped by the device due to the transmit\npackets bitrate exceeding the device rate limit.\n"]
    pub fn push_tx_hw_drop_ratelimits(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 32u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of times driver paused accepting new tx packets\nfrom the stack to this queue, because the queue was full.\nNote that if BQL is supported and enabled on the device\nthe networking stack will avoid queuing a lot of data at once.\n"]
    pub fn push_tx_stop(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 33u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of times driver re-started accepting send\nrequests to this queue from the stack.\n"]
    pub fn push_tx_wake(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 34u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushQstats<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushQueueId<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushQueueId<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushQueueId<Prev> {
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
    #[doc = "Queue index; most queue types are indexed like a C array, with indexes starting at 0 and ending at queue count - 1. Queue indexes are scoped to an interface and queue type."]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Queue type as rx, tx. Each queue type defines a separate ID space. XDP TX queues allocated in the kernel are not linked to NAPIs and thus not listed. AF_XDP queues will have more information set in the xsk attribute.\nAssociated type: \"QueueType\" (enum)"]
    pub fn push_type(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushQueueId<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushDmabuf<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushDmabuf<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushDmabuf<Prev> {
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
    #[doc = "netdev ifindex to bind the dmabuf to."]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "receive queues to bind the dmabuf to.\nAttribute may repeat multiple times (treat it as array)"]
    pub fn nested_queues(mut self) -> PushQueueId<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 2u16);
        PushQueueId {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "dmabuf file descriptor to bind."]
    pub fn push_fd(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "id of the dmabuf binding"]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushDmabuf<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get / dump information about a netdev."]
pub struct PushOpDevGetDumpRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpDevGetDumpRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpDevGetDumpRequest<Prev> {
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
}
impl<Prev: Rec> Drop for PushOpDevGetDumpRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get / dump information about a netdev."]
#[derive(Clone)]
pub enum OpDevGetDumpRequest {}
impl<'a> IterableOpDevGetDumpRequest<'a> {}
impl OpDevGetDumpRequest {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpDevGetDumpRequest<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpDevGetDumpRequest::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Dev::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpDevGetDumpRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpDevGetDumpRequest<'a> {
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
impl<'a> Iterator for IterableOpDevGetDumpRequest<'a> {
    type Item = Result<OpDevGetDumpRequest, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpDevGetDumpRequest",
            r#type.and_then(|t| OpDevGetDumpRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpDevGetDumpRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpDevGetDumpRequest");
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
impl IterableOpDevGetDumpRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpDevGetDumpRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpDevGetDumpRequest::attr_from_type(t)),
            );
        }
        (stack, None)
    }
}
#[doc = "Get / dump information about a netdev."]
pub struct PushOpDevGetDumpReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpDevGetDumpReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpDevGetDumpReply<Prev> {
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
    #[doc = "netdev ifindex"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Bitmask of enabled xdp-features.\nAssociated type: \"XdpAct\" (enum)"]
    pub fn push_xdp_features(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 3u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "max fragment count supported by ZC driver"]
    pub fn push_xdp_zc_max_segs(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Bitmask of supported XDP receive metadata features. See Documentation/networking/xdp-rx-metadata.rst for more details.\nAssociated type: \"XdpRxMetadata\" (enum)"]
    pub fn push_xdp_rx_metadata_features(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 5u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Bitmask of enabled AF_XDP features.\nAssociated type: \"XskFlags\" (enum)"]
    pub fn push_xsk_features(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 6u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpDevGetDumpReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get / dump information about a netdev."]
#[derive(Clone)]
pub enum OpDevGetDumpReply {
    #[doc = "netdev ifindex"]
    Ifindex(u32),
    #[doc = "Bitmask of enabled xdp-features.\nAssociated type: \"XdpAct\" (enum)"]
    XdpFeatures(u64),
    #[doc = "max fragment count supported by ZC driver"]
    XdpZcMaxSegs(u32),
    #[doc = "Bitmask of supported XDP receive metadata features. See Documentation/networking/xdp-rx-metadata.rst for more details.\nAssociated type: \"XdpRxMetadata\" (enum)"]
    XdpRxMetadataFeatures(u64),
    #[doc = "Bitmask of enabled AF_XDP features.\nAssociated type: \"XskFlags\" (enum)"]
    XskFeatures(u64),
}
impl<'a> IterableOpDevGetDumpReply<'a> {
    #[doc = "netdev ifindex"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDevGetDumpReply::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDevGetDumpReply",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Bitmask of enabled xdp-features.\nAssociated type: \"XdpAct\" (enum)"]
    pub fn get_xdp_features(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDevGetDumpReply::XdpFeatures(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDevGetDumpReply",
            "XdpFeatures",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "max fragment count supported by ZC driver"]
    pub fn get_xdp_zc_max_segs(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDevGetDumpReply::XdpZcMaxSegs(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDevGetDumpReply",
            "XdpZcMaxSegs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Bitmask of supported XDP receive metadata features. See Documentation/networking/xdp-rx-metadata.rst for more details.\nAssociated type: \"XdpRxMetadata\" (enum)"]
    pub fn get_xdp_rx_metadata_features(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDevGetDumpReply::XdpRxMetadataFeatures(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDevGetDumpReply",
            "XdpRxMetadataFeatures",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Bitmask of enabled AF_XDP features.\nAssociated type: \"XskFlags\" (enum)"]
    pub fn get_xsk_features(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDevGetDumpReply::XskFeatures(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDevGetDumpReply",
            "XskFeatures",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpDevGetDumpReply {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpDevGetDumpReply<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpDevGetDumpReply::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Dev::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpDevGetDumpReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpDevGetDumpReply<'a> {
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
impl<'a> Iterator for IterableOpDevGetDumpReply<'a> {
    type Item = Result<OpDevGetDumpReply, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpDevGetDumpReply::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpDevGetDumpReply::XdpFeatures({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpDevGetDumpReply::XdpZcMaxSegs({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpDevGetDumpReply::XdpRxMetadataFeatures({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpDevGetDumpReply::XskFeatures({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpDevGetDumpReply",
            r#type.and_then(|t| OpDevGetDumpReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpDevGetDumpReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpDevGetDumpReply");
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
                OpDevGetDumpReply::Ifindex(val) => fmt.field("Ifindex", &val),
                OpDevGetDumpReply::XdpFeatures(val) => {
                    fmt.field("XdpFeatures", &FormatFlags(val.into(), XdpAct::from_value))
                }
                OpDevGetDumpReply::XdpZcMaxSegs(val) => fmt.field("XdpZcMaxSegs", &val),
                OpDevGetDumpReply::XdpRxMetadataFeatures(val) => fmt.field(
                    "XdpRxMetadataFeatures",
                    &FormatFlags(val.into(), XdpRxMetadata::from_value),
                ),
                OpDevGetDumpReply::XskFeatures(val) => fmt.field(
                    "XskFeatures",
                    &FormatFlags(val.into(), XskFlags::from_value),
                ),
            };
        }
        fmt.finish()
    }
}
impl IterableOpDevGetDumpReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpDevGetDumpReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpDevGetDumpReply::attr_from_type(t)),
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
                OpDevGetDumpReply::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OpDevGetDumpReply::XdpFeatures(val) => {
                    if last_off == offset {
                        stack.push(("XdpFeatures", last_off));
                        break;
                    }
                }
                OpDevGetDumpReply::XdpZcMaxSegs(val) => {
                    if last_off == offset {
                        stack.push(("XdpZcMaxSegs", last_off));
                        break;
                    }
                }
                OpDevGetDumpReply::XdpRxMetadataFeatures(val) => {
                    if last_off == offset {
                        stack.push(("XdpRxMetadataFeatures", last_off));
                        break;
                    }
                }
                OpDevGetDumpReply::XskFeatures(val) => {
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
            stack.push(("OpDevGetDumpReply", cur));
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpDevGetDumpRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpDevGetDumpRequest<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        PushOpDevGetDumpRequest::write_header(&mut request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode(&mut self) -> PushOpDevGetDumpRequest<&mut Vec<u8>> {
        PushOpDevGetDumpRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpDevGetDumpRequest<RequestBuf<'r>> {
        PushOpDevGetDumpRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpDevGetDumpRequest<'_> {
    type ReplyType<'buf> = IterableOpDevGetDumpReply<'buf>;
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpDevGetDumpReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpDevGetDumpRequest::new(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "Get / dump information about a netdev."]
pub struct PushOpDevGetDoRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpDevGetDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpDevGetDoRequest<Prev> {
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
    #[doc = "netdev ifindex"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpDevGetDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get / dump information about a netdev."]
#[derive(Clone)]
pub enum OpDevGetDoRequest {
    #[doc = "netdev ifindex"]
    Ifindex(u32),
}
impl<'a> IterableOpDevGetDoRequest<'a> {
    #[doc = "netdev ifindex"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDevGetDoRequest::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDevGetDoRequest",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpDevGetDoRequest {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpDevGetDoRequest<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpDevGetDoRequest::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Dev::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpDevGetDoRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpDevGetDoRequest<'a> {
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
impl<'a> Iterator for IterableOpDevGetDoRequest<'a> {
    type Item = Result<OpDevGetDoRequest, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpDevGetDoRequest::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpDevGetDoRequest",
            r#type.and_then(|t| OpDevGetDoRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpDevGetDoRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpDevGetDoRequest");
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
                OpDevGetDoRequest::Ifindex(val) => fmt.field("Ifindex", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpDevGetDoRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpDevGetDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpDevGetDoRequest::attr_from_type(t)),
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
                OpDevGetDoRequest::Ifindex(val) => {
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
            stack.push(("OpDevGetDoRequest", cur));
        }
        (stack, None)
    }
}
#[doc = "Get / dump information about a netdev."]
pub struct PushOpDevGetDoReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpDevGetDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpDevGetDoReply<Prev> {
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
    #[doc = "netdev ifindex"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Bitmask of enabled xdp-features.\nAssociated type: \"XdpAct\" (enum)"]
    pub fn push_xdp_features(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 3u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "max fragment count supported by ZC driver"]
    pub fn push_xdp_zc_max_segs(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Bitmask of supported XDP receive metadata features. See Documentation/networking/xdp-rx-metadata.rst for more details.\nAssociated type: \"XdpRxMetadata\" (enum)"]
    pub fn push_xdp_rx_metadata_features(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 5u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Bitmask of enabled AF_XDP features.\nAssociated type: \"XskFlags\" (enum)"]
    pub fn push_xsk_features(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 6u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpDevGetDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get / dump information about a netdev."]
#[derive(Clone)]
pub enum OpDevGetDoReply {
    #[doc = "netdev ifindex"]
    Ifindex(u32),
    #[doc = "Bitmask of enabled xdp-features.\nAssociated type: \"XdpAct\" (enum)"]
    XdpFeatures(u64),
    #[doc = "max fragment count supported by ZC driver"]
    XdpZcMaxSegs(u32),
    #[doc = "Bitmask of supported XDP receive metadata features. See Documentation/networking/xdp-rx-metadata.rst for more details.\nAssociated type: \"XdpRxMetadata\" (enum)"]
    XdpRxMetadataFeatures(u64),
    #[doc = "Bitmask of enabled AF_XDP features.\nAssociated type: \"XskFlags\" (enum)"]
    XskFeatures(u64),
}
impl<'a> IterableOpDevGetDoReply<'a> {
    #[doc = "netdev ifindex"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDevGetDoReply::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDevGetDoReply",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Bitmask of enabled xdp-features.\nAssociated type: \"XdpAct\" (enum)"]
    pub fn get_xdp_features(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDevGetDoReply::XdpFeatures(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDevGetDoReply",
            "XdpFeatures",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "max fragment count supported by ZC driver"]
    pub fn get_xdp_zc_max_segs(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDevGetDoReply::XdpZcMaxSegs(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDevGetDoReply",
            "XdpZcMaxSegs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Bitmask of supported XDP receive metadata features. See Documentation/networking/xdp-rx-metadata.rst for more details.\nAssociated type: \"XdpRxMetadata\" (enum)"]
    pub fn get_xdp_rx_metadata_features(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDevGetDoReply::XdpRxMetadataFeatures(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDevGetDoReply",
            "XdpRxMetadataFeatures",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Bitmask of enabled AF_XDP features.\nAssociated type: \"XskFlags\" (enum)"]
    pub fn get_xsk_features(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDevGetDoReply::XskFeatures(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpDevGetDoReply",
            "XskFeatures",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpDevGetDoReply {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpDevGetDoReply<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpDevGetDoReply::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Dev::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpDevGetDoReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpDevGetDoReply<'a> {
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
impl<'a> Iterator for IterableOpDevGetDoReply<'a> {
    type Item = Result<OpDevGetDoReply, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpDevGetDoReply::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpDevGetDoReply::XdpFeatures({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpDevGetDoReply::XdpZcMaxSegs({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpDevGetDoReply::XdpRxMetadataFeatures({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpDevGetDoReply::XskFeatures({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpDevGetDoReply",
            r#type.and_then(|t| OpDevGetDoReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpDevGetDoReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpDevGetDoReply");
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
                OpDevGetDoReply::Ifindex(val) => fmt.field("Ifindex", &val),
                OpDevGetDoReply::XdpFeatures(val) => {
                    fmt.field("XdpFeatures", &FormatFlags(val.into(), XdpAct::from_value))
                }
                OpDevGetDoReply::XdpZcMaxSegs(val) => fmt.field("XdpZcMaxSegs", &val),
                OpDevGetDoReply::XdpRxMetadataFeatures(val) => fmt.field(
                    "XdpRxMetadataFeatures",
                    &FormatFlags(val.into(), XdpRxMetadata::from_value),
                ),
                OpDevGetDoReply::XskFeatures(val) => fmt.field(
                    "XskFeatures",
                    &FormatFlags(val.into(), XskFlags::from_value),
                ),
            };
        }
        fmt.finish()
    }
}
impl IterableOpDevGetDoReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpDevGetDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpDevGetDoReply::attr_from_type(t)),
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
                OpDevGetDoReply::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OpDevGetDoReply::XdpFeatures(val) => {
                    if last_off == offset {
                        stack.push(("XdpFeatures", last_off));
                        break;
                    }
                }
                OpDevGetDoReply::XdpZcMaxSegs(val) => {
                    if last_off == offset {
                        stack.push(("XdpZcMaxSegs", last_off));
                        break;
                    }
                }
                OpDevGetDoReply::XdpRxMetadataFeatures(val) => {
                    if last_off == offset {
                        stack.push(("XdpRxMetadataFeatures", last_off));
                        break;
                    }
                }
                OpDevGetDoReply::XskFeatures(val) => {
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
            stack.push(("OpDevGetDoReply", cur));
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpDevGetDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpDevGetDoRequest<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        PushOpDevGetDoRequest::write_header(&mut request.buf_mut());
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpDevGetDoRequest<&mut Vec<u8>> {
        PushOpDevGetDoRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpDevGetDoRequest<RequestBuf<'r>> {
        PushOpDevGetDoRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpDevGetDoRequest<'_> {
    type ReplyType<'buf> = IterableOpDevGetDoReply<'buf>;
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpDevGetDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpDevGetDoRequest::new(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "Get / dump information about Page Pools.\n(Only Page Pools associated with a net_device can be listed.)\n"]
pub struct PushOpPagePoolGetDumpRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpPagePoolGetDumpRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpPagePoolGetDumpRequest<Prev> {
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
        header.set_cmd(5u8);
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
impl<Prev: Rec> Drop for PushOpPagePoolGetDumpRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get / dump information about Page Pools.\n(Only Page Pools associated with a net_device can be listed.)\n"]
#[derive(Clone)]
pub enum OpPagePoolGetDumpRequest {}
impl<'a> IterableOpPagePoolGetDumpRequest<'a> {}
impl OpPagePoolGetDumpRequest {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpPagePoolGetDumpRequest<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpPagePoolGetDumpRequest::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        PagePool::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpPagePoolGetDumpRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpPagePoolGetDumpRequest<'a> {
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
impl<'a> Iterator for IterableOpPagePoolGetDumpRequest<'a> {
    type Item = Result<OpPagePoolGetDumpRequest, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpPagePoolGetDumpRequest",
            r#type.and_then(|t| OpPagePoolGetDumpRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpPagePoolGetDumpRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpPagePoolGetDumpRequest");
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
impl IterableOpPagePoolGetDumpRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpPagePoolGetDumpRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpPagePoolGetDumpRequest::attr_from_type(t)),
            );
        }
        (stack, None)
    }
}
#[doc = "Get / dump information about Page Pools.\n(Only Page Pools associated with a net_device can be listed.)\n"]
pub struct PushOpPagePoolGetDumpReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpPagePoolGetDumpReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpPagePoolGetDumpReply<Prev> {
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
        header.set_cmd(5u8);
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
    #[doc = "Unique ID of a Page Pool instance."]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ifindex of the netdev to which the pool belongs.\nMay be reported as 0 if the page pool was allocated for a netdev\nwhich got destroyed already (page pools may outlast their netdevs\nbecause they wait for all memory to be returned).\n"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Id of NAPI using this Page Pool instance."]
    pub fn push_napi_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of outstanding references to this page pool (allocated\nbut yet to be freed pages). Allocated pages may be held in\nsocket receive queues, driver receive ring, page pool recycling\nring, the page pool cache, etc.\n"]
    pub fn push_inflight(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Amount of memory held by inflight pages.\n"]
    pub fn push_inflight_mem(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 5u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Seconds in CLOCK_BOOTTIME of when Page Pool was detached by\nthe driver. Once detached Page Pool can no longer be used to\nallocate memory.\nPage Pools wait for all the memory allocated from them to be freed\nbefore truly disappearing. \"Detached\" Page Pools cannot be\n\"re-attached\", they are just waiting to disappear.\nAttribute is absent if Page Pool has not been detached, and\ncan still be used to allocate new memory.\n"]
    pub fn push_detach_time(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 6u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ID of the dmabuf this page-pool is attached to."]
    pub fn push_dmabuf(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 7u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "io-uring memory provider information."]
    pub fn nested_io_uring(mut self) -> PushIoUringProviderInfo<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 8u16);
        PushIoUringProviderInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushOpPagePoolGetDumpReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get / dump information about Page Pools.\n(Only Page Pools associated with a net_device can be listed.)\n"]
#[derive(Clone)]
pub enum OpPagePoolGetDumpReply<'a> {
    #[doc = "Unique ID of a Page Pool instance."]
    Id(u32),
    #[doc = "ifindex of the netdev to which the pool belongs.\nMay be reported as 0 if the page pool was allocated for a netdev\nwhich got destroyed already (page pools may outlast their netdevs\nbecause they wait for all memory to be returned).\n"]
    Ifindex(u32),
    #[doc = "Id of NAPI using this Page Pool instance."]
    NapiId(u32),
    #[doc = "Number of outstanding references to this page pool (allocated\nbut yet to be freed pages). Allocated pages may be held in\nsocket receive queues, driver receive ring, page pool recycling\nring, the page pool cache, etc.\n"]
    Inflight(u32),
    #[doc = "Amount of memory held by inflight pages.\n"]
    InflightMem(u32),
    #[doc = "Seconds in CLOCK_BOOTTIME of when Page Pool was detached by\nthe driver. Once detached Page Pool can no longer be used to\nallocate memory.\nPage Pools wait for all the memory allocated from them to be freed\nbefore truly disappearing. \"Detached\" Page Pools cannot be\n\"re-attached\", they are just waiting to disappear.\nAttribute is absent if Page Pool has not been detached, and\ncan still be used to allocate new memory.\n"]
    DetachTime(u32),
    #[doc = "ID of the dmabuf this page-pool is attached to."]
    Dmabuf(u32),
    #[doc = "io-uring memory provider information."]
    IoUring(IterableIoUringProviderInfo<'a>),
}
impl<'a> IterableOpPagePoolGetDumpReply<'a> {
    #[doc = "Unique ID of a Page Pool instance."]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolGetDumpReply::Id(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolGetDumpReply",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ifindex of the netdev to which the pool belongs.\nMay be reported as 0 if the page pool was allocated for a netdev\nwhich got destroyed already (page pools may outlast their netdevs\nbecause they wait for all memory to be returned).\n"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolGetDumpReply::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolGetDumpReply",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Id of NAPI using this Page Pool instance."]
    pub fn get_napi_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolGetDumpReply::NapiId(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolGetDumpReply",
            "NapiId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of outstanding references to this page pool (allocated\nbut yet to be freed pages). Allocated pages may be held in\nsocket receive queues, driver receive ring, page pool recycling\nring, the page pool cache, etc.\n"]
    pub fn get_inflight(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolGetDumpReply::Inflight(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolGetDumpReply",
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
            if let OpPagePoolGetDumpReply::InflightMem(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolGetDumpReply",
            "InflightMem",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Seconds in CLOCK_BOOTTIME of when Page Pool was detached by\nthe driver. Once detached Page Pool can no longer be used to\nallocate memory.\nPage Pools wait for all the memory allocated from them to be freed\nbefore truly disappearing. \"Detached\" Page Pools cannot be\n\"re-attached\", they are just waiting to disappear.\nAttribute is absent if Page Pool has not been detached, and\ncan still be used to allocate new memory.\n"]
    pub fn get_detach_time(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolGetDumpReply::DetachTime(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolGetDumpReply",
            "DetachTime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ID of the dmabuf this page-pool is attached to."]
    pub fn get_dmabuf(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolGetDumpReply::Dmabuf(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolGetDumpReply",
            "Dmabuf",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "io-uring memory provider information."]
    pub fn get_io_uring(&self) -> Result<IterableIoUringProviderInfo<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolGetDumpReply::IoUring(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolGetDumpReply",
            "IoUring",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpPagePoolGetDumpReply<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpPagePoolGetDumpReply<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpPagePoolGetDumpReply::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        PagePool::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpPagePoolGetDumpReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpPagePoolGetDumpReply<'a> {
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
impl<'a> Iterator for IterableOpPagePoolGetDumpReply<'a> {
    type Item = Result<OpPagePoolGetDumpReply<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpPagePoolGetDumpReply::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpPagePoolGetDumpReply::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpPagePoolGetDumpReply::NapiId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpPagePoolGetDumpReply::Inflight({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpPagePoolGetDumpReply::InflightMem({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpPagePoolGetDumpReply::DetachTime({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => OpPagePoolGetDumpReply::Dmabuf({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpPagePoolGetDumpReply::IoUring({
                    let res = Some(IterableIoUringProviderInfo::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpPagePoolGetDumpReply",
            r#type.and_then(|t| OpPagePoolGetDumpReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpPagePoolGetDumpReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpPagePoolGetDumpReply");
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
                OpPagePoolGetDumpReply::Id(val) => fmt.field("Id", &val),
                OpPagePoolGetDumpReply::Ifindex(val) => fmt.field("Ifindex", &val),
                OpPagePoolGetDumpReply::NapiId(val) => fmt.field("NapiId", &val),
                OpPagePoolGetDumpReply::Inflight(val) => fmt.field("Inflight", &val),
                OpPagePoolGetDumpReply::InflightMem(val) => fmt.field("InflightMem", &val),
                OpPagePoolGetDumpReply::DetachTime(val) => fmt.field("DetachTime", &val),
                OpPagePoolGetDumpReply::Dmabuf(val) => fmt.field("Dmabuf", &val),
                OpPagePoolGetDumpReply::IoUring(val) => fmt.field("IoUring", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpPagePoolGetDumpReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpPagePoolGetDumpReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpPagePoolGetDumpReply::attr_from_type(t)),
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
                OpPagePoolGetDumpReply::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                OpPagePoolGetDumpReply::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OpPagePoolGetDumpReply::NapiId(val) => {
                    if last_off == offset {
                        stack.push(("NapiId", last_off));
                        break;
                    }
                }
                OpPagePoolGetDumpReply::Inflight(val) => {
                    if last_off == offset {
                        stack.push(("Inflight", last_off));
                        break;
                    }
                }
                OpPagePoolGetDumpReply::InflightMem(val) => {
                    if last_off == offset {
                        stack.push(("InflightMem", last_off));
                        break;
                    }
                }
                OpPagePoolGetDumpReply::DetachTime(val) => {
                    if last_off == offset {
                        stack.push(("DetachTime", last_off));
                        break;
                    }
                }
                OpPagePoolGetDumpReply::Dmabuf(val) => {
                    if last_off == offset {
                        stack.push(("Dmabuf", last_off));
                        break;
                    }
                }
                OpPagePoolGetDumpReply::IoUring(val) => {
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
            stack.push(("OpPagePoolGetDumpReply", cur));
        }
        (stack, missing)
    }
}
#[derive(Debug)]
pub struct RequestOpPagePoolGetDumpRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpPagePoolGetDumpRequest<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        PushOpPagePoolGetDumpRequest::write_header(&mut request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode(&mut self) -> PushOpPagePoolGetDumpRequest<&mut Vec<u8>> {
        PushOpPagePoolGetDumpRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpPagePoolGetDumpRequest<RequestBuf<'r>> {
        PushOpPagePoolGetDumpRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpPagePoolGetDumpRequest<'_> {
    type ReplyType<'buf> = IterableOpPagePoolGetDumpReply<'buf>;
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpPagePoolGetDumpReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpPagePoolGetDumpRequest::new(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "Get / dump information about Page Pools.\n(Only Page Pools associated with a net_device can be listed.)\n"]
pub struct PushOpPagePoolGetDoRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpPagePoolGetDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpPagePoolGetDoRequest<Prev> {
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
        header.set_cmd(5u8);
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
    #[doc = "Unique ID of a Page Pool instance."]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpPagePoolGetDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get / dump information about Page Pools.\n(Only Page Pools associated with a net_device can be listed.)\n"]
#[derive(Clone)]
pub enum OpPagePoolGetDoRequest {
    #[doc = "Unique ID of a Page Pool instance."]
    Id(u32),
}
impl<'a> IterableOpPagePoolGetDoRequest<'a> {
    #[doc = "Unique ID of a Page Pool instance."]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolGetDoRequest::Id(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolGetDoRequest",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpPagePoolGetDoRequest {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpPagePoolGetDoRequest<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpPagePoolGetDoRequest::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        PagePool::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpPagePoolGetDoRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpPagePoolGetDoRequest<'a> {
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
impl<'a> Iterator for IterableOpPagePoolGetDoRequest<'a> {
    type Item = Result<OpPagePoolGetDoRequest, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpPagePoolGetDoRequest::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpPagePoolGetDoRequest",
            r#type.and_then(|t| OpPagePoolGetDoRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpPagePoolGetDoRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpPagePoolGetDoRequest");
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
                OpPagePoolGetDoRequest::Id(val) => fmt.field("Id", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpPagePoolGetDoRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpPagePoolGetDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpPagePoolGetDoRequest::attr_from_type(t)),
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
                OpPagePoolGetDoRequest::Id(val) => {
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
            stack.push(("OpPagePoolGetDoRequest", cur));
        }
        (stack, None)
    }
}
#[doc = "Get / dump information about Page Pools.\n(Only Page Pools associated with a net_device can be listed.)\n"]
pub struct PushOpPagePoolGetDoReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpPagePoolGetDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpPagePoolGetDoReply<Prev> {
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
        header.set_cmd(5u8);
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
    #[doc = "Unique ID of a Page Pool instance."]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ifindex of the netdev to which the pool belongs.\nMay be reported as 0 if the page pool was allocated for a netdev\nwhich got destroyed already (page pools may outlast their netdevs\nbecause they wait for all memory to be returned).\n"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Id of NAPI using this Page Pool instance."]
    pub fn push_napi_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of outstanding references to this page pool (allocated\nbut yet to be freed pages). Allocated pages may be held in\nsocket receive queues, driver receive ring, page pool recycling\nring, the page pool cache, etc.\n"]
    pub fn push_inflight(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Amount of memory held by inflight pages.\n"]
    pub fn push_inflight_mem(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 5u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Seconds in CLOCK_BOOTTIME of when Page Pool was detached by\nthe driver. Once detached Page Pool can no longer be used to\nallocate memory.\nPage Pools wait for all the memory allocated from them to be freed\nbefore truly disappearing. \"Detached\" Page Pools cannot be\n\"re-attached\", they are just waiting to disappear.\nAttribute is absent if Page Pool has not been detached, and\ncan still be used to allocate new memory.\n"]
    pub fn push_detach_time(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 6u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ID of the dmabuf this page-pool is attached to."]
    pub fn push_dmabuf(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 7u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "io-uring memory provider information."]
    pub fn nested_io_uring(mut self) -> PushIoUringProviderInfo<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 8u16);
        PushIoUringProviderInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushOpPagePoolGetDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get / dump information about Page Pools.\n(Only Page Pools associated with a net_device can be listed.)\n"]
#[derive(Clone)]
pub enum OpPagePoolGetDoReply<'a> {
    #[doc = "Unique ID of a Page Pool instance."]
    Id(u32),
    #[doc = "ifindex of the netdev to which the pool belongs.\nMay be reported as 0 if the page pool was allocated for a netdev\nwhich got destroyed already (page pools may outlast their netdevs\nbecause they wait for all memory to be returned).\n"]
    Ifindex(u32),
    #[doc = "Id of NAPI using this Page Pool instance."]
    NapiId(u32),
    #[doc = "Number of outstanding references to this page pool (allocated\nbut yet to be freed pages). Allocated pages may be held in\nsocket receive queues, driver receive ring, page pool recycling\nring, the page pool cache, etc.\n"]
    Inflight(u32),
    #[doc = "Amount of memory held by inflight pages.\n"]
    InflightMem(u32),
    #[doc = "Seconds in CLOCK_BOOTTIME of when Page Pool was detached by\nthe driver. Once detached Page Pool can no longer be used to\nallocate memory.\nPage Pools wait for all the memory allocated from them to be freed\nbefore truly disappearing. \"Detached\" Page Pools cannot be\n\"re-attached\", they are just waiting to disappear.\nAttribute is absent if Page Pool has not been detached, and\ncan still be used to allocate new memory.\n"]
    DetachTime(u32),
    #[doc = "ID of the dmabuf this page-pool is attached to."]
    Dmabuf(u32),
    #[doc = "io-uring memory provider information."]
    IoUring(IterableIoUringProviderInfo<'a>),
}
impl<'a> IterableOpPagePoolGetDoReply<'a> {
    #[doc = "Unique ID of a Page Pool instance."]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolGetDoReply::Id(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolGetDoReply",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ifindex of the netdev to which the pool belongs.\nMay be reported as 0 if the page pool was allocated for a netdev\nwhich got destroyed already (page pools may outlast their netdevs\nbecause they wait for all memory to be returned).\n"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolGetDoReply::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolGetDoReply",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Id of NAPI using this Page Pool instance."]
    pub fn get_napi_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolGetDoReply::NapiId(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolGetDoReply",
            "NapiId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of outstanding references to this page pool (allocated\nbut yet to be freed pages). Allocated pages may be held in\nsocket receive queues, driver receive ring, page pool recycling\nring, the page pool cache, etc.\n"]
    pub fn get_inflight(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolGetDoReply::Inflight(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolGetDoReply",
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
            if let OpPagePoolGetDoReply::InflightMem(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolGetDoReply",
            "InflightMem",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Seconds in CLOCK_BOOTTIME of when Page Pool was detached by\nthe driver. Once detached Page Pool can no longer be used to\nallocate memory.\nPage Pools wait for all the memory allocated from them to be freed\nbefore truly disappearing. \"Detached\" Page Pools cannot be\n\"re-attached\", they are just waiting to disappear.\nAttribute is absent if Page Pool has not been detached, and\ncan still be used to allocate new memory.\n"]
    pub fn get_detach_time(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolGetDoReply::DetachTime(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolGetDoReply",
            "DetachTime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ID of the dmabuf this page-pool is attached to."]
    pub fn get_dmabuf(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolGetDoReply::Dmabuf(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolGetDoReply",
            "Dmabuf",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "io-uring memory provider information."]
    pub fn get_io_uring(&self) -> Result<IterableIoUringProviderInfo<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolGetDoReply::IoUring(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolGetDoReply",
            "IoUring",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpPagePoolGetDoReply<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpPagePoolGetDoReply<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpPagePoolGetDoReply::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        PagePool::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpPagePoolGetDoReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpPagePoolGetDoReply<'a> {
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
impl<'a> Iterator for IterableOpPagePoolGetDoReply<'a> {
    type Item = Result<OpPagePoolGetDoReply<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpPagePoolGetDoReply::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpPagePoolGetDoReply::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpPagePoolGetDoReply::NapiId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpPagePoolGetDoReply::Inflight({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpPagePoolGetDoReply::InflightMem({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpPagePoolGetDoReply::DetachTime({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => OpPagePoolGetDoReply::Dmabuf({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpPagePoolGetDoReply::IoUring({
                    let res = Some(IterableIoUringProviderInfo::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpPagePoolGetDoReply",
            r#type.and_then(|t| OpPagePoolGetDoReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpPagePoolGetDoReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpPagePoolGetDoReply");
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
                OpPagePoolGetDoReply::Id(val) => fmt.field("Id", &val),
                OpPagePoolGetDoReply::Ifindex(val) => fmt.field("Ifindex", &val),
                OpPagePoolGetDoReply::NapiId(val) => fmt.field("NapiId", &val),
                OpPagePoolGetDoReply::Inflight(val) => fmt.field("Inflight", &val),
                OpPagePoolGetDoReply::InflightMem(val) => fmt.field("InflightMem", &val),
                OpPagePoolGetDoReply::DetachTime(val) => fmt.field("DetachTime", &val),
                OpPagePoolGetDoReply::Dmabuf(val) => fmt.field("Dmabuf", &val),
                OpPagePoolGetDoReply::IoUring(val) => fmt.field("IoUring", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpPagePoolGetDoReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpPagePoolGetDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpPagePoolGetDoReply::attr_from_type(t)),
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
                OpPagePoolGetDoReply::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                OpPagePoolGetDoReply::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OpPagePoolGetDoReply::NapiId(val) => {
                    if last_off == offset {
                        stack.push(("NapiId", last_off));
                        break;
                    }
                }
                OpPagePoolGetDoReply::Inflight(val) => {
                    if last_off == offset {
                        stack.push(("Inflight", last_off));
                        break;
                    }
                }
                OpPagePoolGetDoReply::InflightMem(val) => {
                    if last_off == offset {
                        stack.push(("InflightMem", last_off));
                        break;
                    }
                }
                OpPagePoolGetDoReply::DetachTime(val) => {
                    if last_off == offset {
                        stack.push(("DetachTime", last_off));
                        break;
                    }
                }
                OpPagePoolGetDoReply::Dmabuf(val) => {
                    if last_off == offset {
                        stack.push(("Dmabuf", last_off));
                        break;
                    }
                }
                OpPagePoolGetDoReply::IoUring(val) => {
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
            stack.push(("OpPagePoolGetDoReply", cur));
        }
        (stack, missing)
    }
}
#[derive(Debug)]
pub struct RequestOpPagePoolGetDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpPagePoolGetDoRequest<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        PushOpPagePoolGetDoRequest::write_header(&mut request.buf_mut());
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpPagePoolGetDoRequest<&mut Vec<u8>> {
        PushOpPagePoolGetDoRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpPagePoolGetDoRequest<RequestBuf<'r>> {
        PushOpPagePoolGetDoRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpPagePoolGetDoRequest<'_> {
    type ReplyType<'buf> = IterableOpPagePoolGetDoReply<'buf>;
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpPagePoolGetDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpPagePoolGetDoRequest::new(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "Get page pool statistics."]
pub struct PushOpPagePoolStatsGetDumpRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpPagePoolStatsGetDumpRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpPagePoolStatsGetDumpRequest<Prev> {
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
        header.set_cmd(9u8);
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
impl<Prev: Rec> Drop for PushOpPagePoolStatsGetDumpRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get page pool statistics."]
#[derive(Clone)]
pub enum OpPagePoolStatsGetDumpRequest {}
impl<'a> IterableOpPagePoolStatsGetDumpRequest<'a> {}
impl OpPagePoolStatsGetDumpRequest {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpPagePoolStatsGetDumpRequest<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpPagePoolStatsGetDumpRequest::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        PagePoolStats::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpPagePoolStatsGetDumpRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpPagePoolStatsGetDumpRequest<'a> {
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
impl<'a> Iterator for IterableOpPagePoolStatsGetDumpRequest<'a> {
    type Item = Result<OpPagePoolStatsGetDumpRequest, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpPagePoolStatsGetDumpRequest",
            r#type.and_then(|t| OpPagePoolStatsGetDumpRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpPagePoolStatsGetDumpRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpPagePoolStatsGetDumpRequest");
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
impl IterableOpPagePoolStatsGetDumpRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpPagePoolStatsGetDumpRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpPagePoolStatsGetDumpRequest::attr_from_type(t)),
            );
        }
        (stack, None)
    }
}
#[doc = "Get page pool statistics."]
pub struct PushOpPagePoolStatsGetDumpReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpPagePoolStatsGetDumpReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpPagePoolStatsGetDumpReply<Prev> {
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
        header.set_cmd(9u8);
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
    #[doc = "Page pool identifying information."]
    pub fn nested_info(mut self) -> PushPagePoolInfo<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 1u16);
        PushPagePoolInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_alloc_fast(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_alloc_slow(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 9u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_alloc_slow_high_order(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 10u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_alloc_empty(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 11u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_alloc_refill(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 12u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_alloc_waive(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 13u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_recycle_cached(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 14u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_recycle_cache_full(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 15u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_recycle_ring(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 16u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_recycle_ring_full(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 17u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_recycle_released_refcnt(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 18u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpPagePoolStatsGetDumpReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get page pool statistics."]
#[derive(Clone)]
pub enum OpPagePoolStatsGetDumpReply<'a> {
    #[doc = "Page pool identifying information."]
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
impl<'a> IterableOpPagePoolStatsGetDumpReply<'a> {
    #[doc = "Page pool identifying information."]
    pub fn get_info(&self) -> Result<IterablePagePoolInfo<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolStatsGetDumpReply::Info(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolStatsGetDumpReply",
            "Info",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_alloc_fast(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolStatsGetDumpReply::AllocFast(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolStatsGetDumpReply",
            "AllocFast",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_alloc_slow(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolStatsGetDumpReply::AllocSlow(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolStatsGetDumpReply",
            "AllocSlow",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_alloc_slow_high_order(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolStatsGetDumpReply::AllocSlowHighOrder(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolStatsGetDumpReply",
            "AllocSlowHighOrder",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_alloc_empty(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolStatsGetDumpReply::AllocEmpty(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolStatsGetDumpReply",
            "AllocEmpty",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_alloc_refill(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolStatsGetDumpReply::AllocRefill(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolStatsGetDumpReply",
            "AllocRefill",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_alloc_waive(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolStatsGetDumpReply::AllocWaive(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolStatsGetDumpReply",
            "AllocWaive",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_recycle_cached(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolStatsGetDumpReply::RecycleCached(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolStatsGetDumpReply",
            "RecycleCached",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_recycle_cache_full(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolStatsGetDumpReply::RecycleCacheFull(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolStatsGetDumpReply",
            "RecycleCacheFull",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_recycle_ring(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolStatsGetDumpReply::RecycleRing(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolStatsGetDumpReply",
            "RecycleRing",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_recycle_ring_full(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolStatsGetDumpReply::RecycleRingFull(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolStatsGetDumpReply",
            "RecycleRingFull",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_recycle_released_refcnt(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolStatsGetDumpReply::RecycleReleasedRefcnt(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolStatsGetDumpReply",
            "RecycleReleasedRefcnt",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpPagePoolStatsGetDumpReply<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpPagePoolStatsGetDumpReply<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpPagePoolStatsGetDumpReply::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        PagePoolStats::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpPagePoolStatsGetDumpReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpPagePoolStatsGetDumpReply<'a> {
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
impl<'a> Iterator for IterableOpPagePoolStatsGetDumpReply<'a> {
    type Item = Result<OpPagePoolStatsGetDumpReply<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpPagePoolStatsGetDumpReply::Info({
                    let res = Some(IterablePagePoolInfo::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpPagePoolStatsGetDumpReply::AllocFast({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => OpPagePoolStatsGetDumpReply::AllocSlow({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => OpPagePoolStatsGetDumpReply::AllocSlowHighOrder({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => OpPagePoolStatsGetDumpReply::AllocEmpty({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => OpPagePoolStatsGetDumpReply::AllocRefill({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => OpPagePoolStatsGetDumpReply::AllocWaive({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => OpPagePoolStatsGetDumpReply::RecycleCached({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => OpPagePoolStatsGetDumpReply::RecycleCacheFull({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => OpPagePoolStatsGetDumpReply::RecycleRing({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => OpPagePoolStatsGetDumpReply::RecycleRingFull({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => OpPagePoolStatsGetDumpReply::RecycleReleasedRefcnt({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpPagePoolStatsGetDumpReply",
            r#type.and_then(|t| OpPagePoolStatsGetDumpReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpPagePoolStatsGetDumpReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpPagePoolStatsGetDumpReply");
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
                OpPagePoolStatsGetDumpReply::Info(val) => fmt.field("Info", &val),
                OpPagePoolStatsGetDumpReply::AllocFast(val) => fmt.field("AllocFast", &val),
                OpPagePoolStatsGetDumpReply::AllocSlow(val) => fmt.field("AllocSlow", &val),
                OpPagePoolStatsGetDumpReply::AllocSlowHighOrder(val) => {
                    fmt.field("AllocSlowHighOrder", &val)
                }
                OpPagePoolStatsGetDumpReply::AllocEmpty(val) => fmt.field("AllocEmpty", &val),
                OpPagePoolStatsGetDumpReply::AllocRefill(val) => fmt.field("AllocRefill", &val),
                OpPagePoolStatsGetDumpReply::AllocWaive(val) => fmt.field("AllocWaive", &val),
                OpPagePoolStatsGetDumpReply::RecycleCached(val) => fmt.field("RecycleCached", &val),
                OpPagePoolStatsGetDumpReply::RecycleCacheFull(val) => {
                    fmt.field("RecycleCacheFull", &val)
                }
                OpPagePoolStatsGetDumpReply::RecycleRing(val) => fmt.field("RecycleRing", &val),
                OpPagePoolStatsGetDumpReply::RecycleRingFull(val) => {
                    fmt.field("RecycleRingFull", &val)
                }
                OpPagePoolStatsGetDumpReply::RecycleReleasedRefcnt(val) => {
                    fmt.field("RecycleReleasedRefcnt", &val)
                }
            };
        }
        fmt.finish()
    }
}
impl IterableOpPagePoolStatsGetDumpReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpPagePoolStatsGetDumpReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpPagePoolStatsGetDumpReply::attr_from_type(t)),
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
                OpPagePoolStatsGetDumpReply::Info(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpPagePoolStatsGetDumpReply::AllocFast(val) => {
                    if last_off == offset {
                        stack.push(("AllocFast", last_off));
                        break;
                    }
                }
                OpPagePoolStatsGetDumpReply::AllocSlow(val) => {
                    if last_off == offset {
                        stack.push(("AllocSlow", last_off));
                        break;
                    }
                }
                OpPagePoolStatsGetDumpReply::AllocSlowHighOrder(val) => {
                    if last_off == offset {
                        stack.push(("AllocSlowHighOrder", last_off));
                        break;
                    }
                }
                OpPagePoolStatsGetDumpReply::AllocEmpty(val) => {
                    if last_off == offset {
                        stack.push(("AllocEmpty", last_off));
                        break;
                    }
                }
                OpPagePoolStatsGetDumpReply::AllocRefill(val) => {
                    if last_off == offset {
                        stack.push(("AllocRefill", last_off));
                        break;
                    }
                }
                OpPagePoolStatsGetDumpReply::AllocWaive(val) => {
                    if last_off == offset {
                        stack.push(("AllocWaive", last_off));
                        break;
                    }
                }
                OpPagePoolStatsGetDumpReply::RecycleCached(val) => {
                    if last_off == offset {
                        stack.push(("RecycleCached", last_off));
                        break;
                    }
                }
                OpPagePoolStatsGetDumpReply::RecycleCacheFull(val) => {
                    if last_off == offset {
                        stack.push(("RecycleCacheFull", last_off));
                        break;
                    }
                }
                OpPagePoolStatsGetDumpReply::RecycleRing(val) => {
                    if last_off == offset {
                        stack.push(("RecycleRing", last_off));
                        break;
                    }
                }
                OpPagePoolStatsGetDumpReply::RecycleRingFull(val) => {
                    if last_off == offset {
                        stack.push(("RecycleRingFull", last_off));
                        break;
                    }
                }
                OpPagePoolStatsGetDumpReply::RecycleReleasedRefcnt(val) => {
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
            stack.push(("OpPagePoolStatsGetDumpReply", cur));
        }
        (stack, missing)
    }
}
#[derive(Debug)]
pub struct RequestOpPagePoolStatsGetDumpRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpPagePoolStatsGetDumpRequest<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        PushOpPagePoolStatsGetDumpRequest::write_header(&mut request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode(&mut self) -> PushOpPagePoolStatsGetDumpRequest<&mut Vec<u8>> {
        PushOpPagePoolStatsGetDumpRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpPagePoolStatsGetDumpRequest<RequestBuf<'r>> {
        PushOpPagePoolStatsGetDumpRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpPagePoolStatsGetDumpRequest<'_> {
    type ReplyType<'buf> = IterableOpPagePoolStatsGetDumpReply<'buf>;
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpPagePoolStatsGetDumpReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpPagePoolStatsGetDumpRequest::new(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "Get page pool statistics."]
pub struct PushOpPagePoolStatsGetDoRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpPagePoolStatsGetDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpPagePoolStatsGetDoRequest<Prev> {
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
        header.set_cmd(9u8);
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
    #[doc = "Page pool identifying information."]
    pub fn nested_info(mut self) -> PushPagePoolInfo<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 1u16);
        PushPagePoolInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushOpPagePoolStatsGetDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get page pool statistics."]
#[derive(Clone)]
pub enum OpPagePoolStatsGetDoRequest<'a> {
    #[doc = "Page pool identifying information."]
    Info(IterablePagePoolInfo<'a>),
}
impl<'a> IterableOpPagePoolStatsGetDoRequest<'a> {
    #[doc = "Page pool identifying information."]
    pub fn get_info(&self) -> Result<IterablePagePoolInfo<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolStatsGetDoRequest::Info(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolStatsGetDoRequest",
            "Info",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpPagePoolStatsGetDoRequest<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpPagePoolStatsGetDoRequest<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpPagePoolStatsGetDoRequest::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        PagePoolStats::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpPagePoolStatsGetDoRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpPagePoolStatsGetDoRequest<'a> {
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
impl<'a> Iterator for IterableOpPagePoolStatsGetDoRequest<'a> {
    type Item = Result<OpPagePoolStatsGetDoRequest<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpPagePoolStatsGetDoRequest::Info({
                    let res = Some(IterablePagePoolInfo::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpPagePoolStatsGetDoRequest",
            r#type.and_then(|t| OpPagePoolStatsGetDoRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpPagePoolStatsGetDoRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpPagePoolStatsGetDoRequest");
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
                OpPagePoolStatsGetDoRequest::Info(val) => fmt.field("Info", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpPagePoolStatsGetDoRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpPagePoolStatsGetDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpPagePoolStatsGetDoRequest::attr_from_type(t)),
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
                OpPagePoolStatsGetDoRequest::Info(val) => {
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
            stack.push(("OpPagePoolStatsGetDoRequest", cur));
        }
        (stack, missing)
    }
}
#[doc = "Get page pool statistics."]
pub struct PushOpPagePoolStatsGetDoReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpPagePoolStatsGetDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpPagePoolStatsGetDoReply<Prev> {
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
        header.set_cmd(9u8);
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
    #[doc = "Page pool identifying information."]
    pub fn nested_info(mut self) -> PushPagePoolInfo<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 1u16);
        PushPagePoolInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_alloc_fast(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_alloc_slow(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 9u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_alloc_slow_high_order(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 10u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_alloc_empty(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 11u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_alloc_refill(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 12u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_alloc_waive(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 13u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_recycle_cached(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 14u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_recycle_cache_full(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 15u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_recycle_ring(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 16u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_recycle_ring_full(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 17u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_recycle_released_refcnt(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 18u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpPagePoolStatsGetDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get page pool statistics."]
#[derive(Clone)]
pub enum OpPagePoolStatsGetDoReply<'a> {
    #[doc = "Page pool identifying information."]
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
impl<'a> IterableOpPagePoolStatsGetDoReply<'a> {
    #[doc = "Page pool identifying information."]
    pub fn get_info(&self) -> Result<IterablePagePoolInfo<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolStatsGetDoReply::Info(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolStatsGetDoReply",
            "Info",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_alloc_fast(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolStatsGetDoReply::AllocFast(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolStatsGetDoReply",
            "AllocFast",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_alloc_slow(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolStatsGetDoReply::AllocSlow(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolStatsGetDoReply",
            "AllocSlow",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_alloc_slow_high_order(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolStatsGetDoReply::AllocSlowHighOrder(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolStatsGetDoReply",
            "AllocSlowHighOrder",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_alloc_empty(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolStatsGetDoReply::AllocEmpty(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolStatsGetDoReply",
            "AllocEmpty",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_alloc_refill(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolStatsGetDoReply::AllocRefill(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolStatsGetDoReply",
            "AllocRefill",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_alloc_waive(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolStatsGetDoReply::AllocWaive(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolStatsGetDoReply",
            "AllocWaive",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_recycle_cached(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolStatsGetDoReply::RecycleCached(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolStatsGetDoReply",
            "RecycleCached",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_recycle_cache_full(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolStatsGetDoReply::RecycleCacheFull(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolStatsGetDoReply",
            "RecycleCacheFull",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_recycle_ring(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolStatsGetDoReply::RecycleRing(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolStatsGetDoReply",
            "RecycleRing",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_recycle_ring_full(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolStatsGetDoReply::RecycleRingFull(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolStatsGetDoReply",
            "RecycleRingFull",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_recycle_released_refcnt(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpPagePoolStatsGetDoReply::RecycleReleasedRefcnt(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpPagePoolStatsGetDoReply",
            "RecycleReleasedRefcnt",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpPagePoolStatsGetDoReply<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpPagePoolStatsGetDoReply<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpPagePoolStatsGetDoReply::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        PagePoolStats::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpPagePoolStatsGetDoReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpPagePoolStatsGetDoReply<'a> {
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
impl<'a> Iterator for IterableOpPagePoolStatsGetDoReply<'a> {
    type Item = Result<OpPagePoolStatsGetDoReply<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpPagePoolStatsGetDoReply::Info({
                    let res = Some(IterablePagePoolInfo::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpPagePoolStatsGetDoReply::AllocFast({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => OpPagePoolStatsGetDoReply::AllocSlow({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => OpPagePoolStatsGetDoReply::AllocSlowHighOrder({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => OpPagePoolStatsGetDoReply::AllocEmpty({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => OpPagePoolStatsGetDoReply::AllocRefill({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => OpPagePoolStatsGetDoReply::AllocWaive({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => OpPagePoolStatsGetDoReply::RecycleCached({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => OpPagePoolStatsGetDoReply::RecycleCacheFull({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => OpPagePoolStatsGetDoReply::RecycleRing({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => OpPagePoolStatsGetDoReply::RecycleRingFull({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => OpPagePoolStatsGetDoReply::RecycleReleasedRefcnt({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpPagePoolStatsGetDoReply",
            r#type.and_then(|t| OpPagePoolStatsGetDoReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpPagePoolStatsGetDoReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpPagePoolStatsGetDoReply");
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
                OpPagePoolStatsGetDoReply::Info(val) => fmt.field("Info", &val),
                OpPagePoolStatsGetDoReply::AllocFast(val) => fmt.field("AllocFast", &val),
                OpPagePoolStatsGetDoReply::AllocSlow(val) => fmt.field("AllocSlow", &val),
                OpPagePoolStatsGetDoReply::AllocSlowHighOrder(val) => {
                    fmt.field("AllocSlowHighOrder", &val)
                }
                OpPagePoolStatsGetDoReply::AllocEmpty(val) => fmt.field("AllocEmpty", &val),
                OpPagePoolStatsGetDoReply::AllocRefill(val) => fmt.field("AllocRefill", &val),
                OpPagePoolStatsGetDoReply::AllocWaive(val) => fmt.field("AllocWaive", &val),
                OpPagePoolStatsGetDoReply::RecycleCached(val) => fmt.field("RecycleCached", &val),
                OpPagePoolStatsGetDoReply::RecycleCacheFull(val) => {
                    fmt.field("RecycleCacheFull", &val)
                }
                OpPagePoolStatsGetDoReply::RecycleRing(val) => fmt.field("RecycleRing", &val),
                OpPagePoolStatsGetDoReply::RecycleRingFull(val) => {
                    fmt.field("RecycleRingFull", &val)
                }
                OpPagePoolStatsGetDoReply::RecycleReleasedRefcnt(val) => {
                    fmt.field("RecycleReleasedRefcnt", &val)
                }
            };
        }
        fmt.finish()
    }
}
impl IterableOpPagePoolStatsGetDoReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpPagePoolStatsGetDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpPagePoolStatsGetDoReply::attr_from_type(t)),
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
                OpPagePoolStatsGetDoReply::Info(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpPagePoolStatsGetDoReply::AllocFast(val) => {
                    if last_off == offset {
                        stack.push(("AllocFast", last_off));
                        break;
                    }
                }
                OpPagePoolStatsGetDoReply::AllocSlow(val) => {
                    if last_off == offset {
                        stack.push(("AllocSlow", last_off));
                        break;
                    }
                }
                OpPagePoolStatsGetDoReply::AllocSlowHighOrder(val) => {
                    if last_off == offset {
                        stack.push(("AllocSlowHighOrder", last_off));
                        break;
                    }
                }
                OpPagePoolStatsGetDoReply::AllocEmpty(val) => {
                    if last_off == offset {
                        stack.push(("AllocEmpty", last_off));
                        break;
                    }
                }
                OpPagePoolStatsGetDoReply::AllocRefill(val) => {
                    if last_off == offset {
                        stack.push(("AllocRefill", last_off));
                        break;
                    }
                }
                OpPagePoolStatsGetDoReply::AllocWaive(val) => {
                    if last_off == offset {
                        stack.push(("AllocWaive", last_off));
                        break;
                    }
                }
                OpPagePoolStatsGetDoReply::RecycleCached(val) => {
                    if last_off == offset {
                        stack.push(("RecycleCached", last_off));
                        break;
                    }
                }
                OpPagePoolStatsGetDoReply::RecycleCacheFull(val) => {
                    if last_off == offset {
                        stack.push(("RecycleCacheFull", last_off));
                        break;
                    }
                }
                OpPagePoolStatsGetDoReply::RecycleRing(val) => {
                    if last_off == offset {
                        stack.push(("RecycleRing", last_off));
                        break;
                    }
                }
                OpPagePoolStatsGetDoReply::RecycleRingFull(val) => {
                    if last_off == offset {
                        stack.push(("RecycleRingFull", last_off));
                        break;
                    }
                }
                OpPagePoolStatsGetDoReply::RecycleReleasedRefcnt(val) => {
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
            stack.push(("OpPagePoolStatsGetDoReply", cur));
        }
        (stack, missing)
    }
}
#[derive(Debug)]
pub struct RequestOpPagePoolStatsGetDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpPagePoolStatsGetDoRequest<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        PushOpPagePoolStatsGetDoRequest::write_header(&mut request.buf_mut());
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpPagePoolStatsGetDoRequest<&mut Vec<u8>> {
        PushOpPagePoolStatsGetDoRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpPagePoolStatsGetDoRequest<RequestBuf<'r>> {
        PushOpPagePoolStatsGetDoRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpPagePoolStatsGetDoRequest<'_> {
    type ReplyType<'buf> = IterableOpPagePoolStatsGetDoReply<'buf>;
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpPagePoolStatsGetDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpPagePoolStatsGetDoRequest::new(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "Get queue information from the kernel. Only configured queues will be reported (as opposed to all available hardware queues)."]
pub struct PushOpQueueGetDumpRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpQueueGetDumpRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpQueueGetDumpRequest<Prev> {
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
    #[doc = "ifindex of the netdevice to which the queue belongs."]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpQueueGetDumpRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get queue information from the kernel. Only configured queues will be reported (as opposed to all available hardware queues)."]
#[derive(Clone)]
pub enum OpQueueGetDumpRequest {
    #[doc = "ifindex of the netdevice to which the queue belongs."]
    Ifindex(u32),
}
impl<'a> IterableOpQueueGetDumpRequest<'a> {
    #[doc = "ifindex of the netdevice to which the queue belongs."]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQueueGetDumpRequest::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQueueGetDumpRequest",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpQueueGetDumpRequest {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpQueueGetDumpRequest<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpQueueGetDumpRequest::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Queue::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpQueueGetDumpRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpQueueGetDumpRequest<'a> {
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
impl<'a> Iterator for IterableOpQueueGetDumpRequest<'a> {
    type Item = Result<OpQueueGetDumpRequest, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                2u16 => OpQueueGetDumpRequest::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpQueueGetDumpRequest",
            r#type.and_then(|t| OpQueueGetDumpRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpQueueGetDumpRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpQueueGetDumpRequest");
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
                OpQueueGetDumpRequest::Ifindex(val) => fmt.field("Ifindex", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpQueueGetDumpRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpQueueGetDumpRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpQueueGetDumpRequest::attr_from_type(t)),
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
                OpQueueGetDumpRequest::Ifindex(val) => {
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
            stack.push(("OpQueueGetDumpRequest", cur));
        }
        (stack, None)
    }
}
#[doc = "Get queue information from the kernel. Only configured queues will be reported (as opposed to all available hardware queues)."]
pub struct PushOpQueueGetDumpReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpQueueGetDumpReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpQueueGetDumpReply<Prev> {
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
    #[doc = "Queue index; most queue types are indexed like a C array, with indexes starting at 0 and ending at queue count - 1. Queue indexes are scoped to an interface and queue type."]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ifindex of the netdevice to which the queue belongs."]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Queue type as rx, tx. Each queue type defines a separate ID space. XDP TX queues allocated in the kernel are not linked to NAPIs and thus not listed. AF_XDP queues will have more information set in the xsk attribute.\nAssociated type: \"QueueType\" (enum)"]
    pub fn push_type(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ID of the NAPI instance which services this queue."]
    pub fn push_napi_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ID of the dmabuf attached to this queue, if any."]
    pub fn push_dmabuf(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 5u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "io_uring memory provider information."]
    pub fn nested_io_uring(mut self) -> PushIoUringProviderInfo<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 6u16);
        PushIoUringProviderInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "XSK information for this queue, if any."]
    pub fn nested_xsk(mut self) -> PushXskInfo<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 7u16);
        PushXskInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushOpQueueGetDumpReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get queue information from the kernel. Only configured queues will be reported (as opposed to all available hardware queues)."]
#[derive(Clone)]
pub enum OpQueueGetDumpReply<'a> {
    #[doc = "Queue index; most queue types are indexed like a C array, with indexes starting at 0 and ending at queue count - 1. Queue indexes are scoped to an interface and queue type."]
    Id(u32),
    #[doc = "ifindex of the netdevice to which the queue belongs."]
    Ifindex(u32),
    #[doc = "Queue type as rx, tx. Each queue type defines a separate ID space. XDP TX queues allocated in the kernel are not linked to NAPIs and thus not listed. AF_XDP queues will have more information set in the xsk attribute.\nAssociated type: \"QueueType\" (enum)"]
    Type(u32),
    #[doc = "ID of the NAPI instance which services this queue."]
    NapiId(u32),
    #[doc = "ID of the dmabuf attached to this queue, if any."]
    Dmabuf(u32),
    #[doc = "io_uring memory provider information."]
    IoUring(IterableIoUringProviderInfo<'a>),
    #[doc = "XSK information for this queue, if any."]
    Xsk(IterableXskInfo<'a>),
}
impl<'a> IterableOpQueueGetDumpReply<'a> {
    #[doc = "Queue index; most queue types are indexed like a C array, with indexes starting at 0 and ending at queue count - 1. Queue indexes are scoped to an interface and queue type."]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQueueGetDumpReply::Id(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQueueGetDumpReply",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ifindex of the netdevice to which the queue belongs."]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQueueGetDumpReply::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQueueGetDumpReply",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Queue type as rx, tx. Each queue type defines a separate ID space. XDP TX queues allocated in the kernel are not linked to NAPIs and thus not listed. AF_XDP queues will have more information set in the xsk attribute.\nAssociated type: \"QueueType\" (enum)"]
    pub fn get_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQueueGetDumpReply::Type(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQueueGetDumpReply",
            "Type",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ID of the NAPI instance which services this queue."]
    pub fn get_napi_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQueueGetDumpReply::NapiId(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQueueGetDumpReply",
            "NapiId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ID of the dmabuf attached to this queue, if any."]
    pub fn get_dmabuf(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQueueGetDumpReply::Dmabuf(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQueueGetDumpReply",
            "Dmabuf",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "io_uring memory provider information."]
    pub fn get_io_uring(&self) -> Result<IterableIoUringProviderInfo<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQueueGetDumpReply::IoUring(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQueueGetDumpReply",
            "IoUring",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "XSK information for this queue, if any."]
    pub fn get_xsk(&self) -> Result<IterableXskInfo<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQueueGetDumpReply::Xsk(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQueueGetDumpReply",
            "Xsk",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpQueueGetDumpReply<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpQueueGetDumpReply<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpQueueGetDumpReply::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Queue::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpQueueGetDumpReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpQueueGetDumpReply<'a> {
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
impl<'a> Iterator for IterableOpQueueGetDumpReply<'a> {
    type Item = Result<OpQueueGetDumpReply<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpQueueGetDumpReply::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpQueueGetDumpReply::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpQueueGetDumpReply::Type({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpQueueGetDumpReply::NapiId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpQueueGetDumpReply::Dmabuf({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpQueueGetDumpReply::IoUring({
                    let res = Some(IterableIoUringProviderInfo::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => OpQueueGetDumpReply::Xsk({
                    let res = Some(IterableXskInfo::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpQueueGetDumpReply",
            r#type.and_then(|t| OpQueueGetDumpReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpQueueGetDumpReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpQueueGetDumpReply");
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
                OpQueueGetDumpReply::Id(val) => fmt.field("Id", &val),
                OpQueueGetDumpReply::Ifindex(val) => fmt.field("Ifindex", &val),
                OpQueueGetDumpReply::Type(val) => {
                    fmt.field("Type", &FormatEnum(val.into(), QueueType::from_value))
                }
                OpQueueGetDumpReply::NapiId(val) => fmt.field("NapiId", &val),
                OpQueueGetDumpReply::Dmabuf(val) => fmt.field("Dmabuf", &val),
                OpQueueGetDumpReply::IoUring(val) => fmt.field("IoUring", &val),
                OpQueueGetDumpReply::Xsk(val) => fmt.field("Xsk", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpQueueGetDumpReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpQueueGetDumpReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpQueueGetDumpReply::attr_from_type(t)),
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
                OpQueueGetDumpReply::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                OpQueueGetDumpReply::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OpQueueGetDumpReply::Type(val) => {
                    if last_off == offset {
                        stack.push(("Type", last_off));
                        break;
                    }
                }
                OpQueueGetDumpReply::NapiId(val) => {
                    if last_off == offset {
                        stack.push(("NapiId", last_off));
                        break;
                    }
                }
                OpQueueGetDumpReply::Dmabuf(val) => {
                    if last_off == offset {
                        stack.push(("Dmabuf", last_off));
                        break;
                    }
                }
                OpQueueGetDumpReply::IoUring(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpQueueGetDumpReply::Xsk(val) => {
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
            stack.push(("OpQueueGetDumpReply", cur));
        }
        (stack, missing)
    }
}
#[derive(Debug)]
pub struct RequestOpQueueGetDumpRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpQueueGetDumpRequest<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        PushOpQueueGetDumpRequest::write_header(&mut request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode(&mut self) -> PushOpQueueGetDumpRequest<&mut Vec<u8>> {
        PushOpQueueGetDumpRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpQueueGetDumpRequest<RequestBuf<'r>> {
        PushOpQueueGetDumpRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpQueueGetDumpRequest<'_> {
    type ReplyType<'buf> = IterableOpQueueGetDumpReply<'buf>;
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpQueueGetDumpReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpQueueGetDumpRequest::new(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "Get queue information from the kernel. Only configured queues will be reported (as opposed to all available hardware queues)."]
pub struct PushOpQueueGetDoRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpQueueGetDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpQueueGetDoRequest<Prev> {
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
    #[doc = "Queue index; most queue types are indexed like a C array, with indexes starting at 0 and ending at queue count - 1. Queue indexes are scoped to an interface and queue type."]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ifindex of the netdevice to which the queue belongs."]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Queue type as rx, tx. Each queue type defines a separate ID space. XDP TX queues allocated in the kernel are not linked to NAPIs and thus not listed. AF_XDP queues will have more information set in the xsk attribute.\nAssociated type: \"QueueType\" (enum)"]
    pub fn push_type(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpQueueGetDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get queue information from the kernel. Only configured queues will be reported (as opposed to all available hardware queues)."]
#[derive(Clone)]
pub enum OpQueueGetDoRequest {
    #[doc = "Queue index; most queue types are indexed like a C array, with indexes starting at 0 and ending at queue count - 1. Queue indexes are scoped to an interface and queue type."]
    Id(u32),
    #[doc = "ifindex of the netdevice to which the queue belongs."]
    Ifindex(u32),
    #[doc = "Queue type as rx, tx. Each queue type defines a separate ID space. XDP TX queues allocated in the kernel are not linked to NAPIs and thus not listed. AF_XDP queues will have more information set in the xsk attribute.\nAssociated type: \"QueueType\" (enum)"]
    Type(u32),
}
impl<'a> IterableOpQueueGetDoRequest<'a> {
    #[doc = "Queue index; most queue types are indexed like a C array, with indexes starting at 0 and ending at queue count - 1. Queue indexes are scoped to an interface and queue type."]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQueueGetDoRequest::Id(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQueueGetDoRequest",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ifindex of the netdevice to which the queue belongs."]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQueueGetDoRequest::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQueueGetDoRequest",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Queue type as rx, tx. Each queue type defines a separate ID space. XDP TX queues allocated in the kernel are not linked to NAPIs and thus not listed. AF_XDP queues will have more information set in the xsk attribute.\nAssociated type: \"QueueType\" (enum)"]
    pub fn get_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQueueGetDoRequest::Type(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQueueGetDoRequest",
            "Type",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpQueueGetDoRequest {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpQueueGetDoRequest<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpQueueGetDoRequest::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Queue::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpQueueGetDoRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpQueueGetDoRequest<'a> {
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
impl<'a> Iterator for IterableOpQueueGetDoRequest<'a> {
    type Item = Result<OpQueueGetDoRequest, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpQueueGetDoRequest::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpQueueGetDoRequest::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpQueueGetDoRequest::Type({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpQueueGetDoRequest",
            r#type.and_then(|t| OpQueueGetDoRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpQueueGetDoRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpQueueGetDoRequest");
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
                OpQueueGetDoRequest::Id(val) => fmt.field("Id", &val),
                OpQueueGetDoRequest::Ifindex(val) => fmt.field("Ifindex", &val),
                OpQueueGetDoRequest::Type(val) => {
                    fmt.field("Type", &FormatEnum(val.into(), QueueType::from_value))
                }
            };
        }
        fmt.finish()
    }
}
impl IterableOpQueueGetDoRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpQueueGetDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpQueueGetDoRequest::attr_from_type(t)),
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
                OpQueueGetDoRequest::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                OpQueueGetDoRequest::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OpQueueGetDoRequest::Type(val) => {
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
            stack.push(("OpQueueGetDoRequest", cur));
        }
        (stack, None)
    }
}
#[doc = "Get queue information from the kernel. Only configured queues will be reported (as opposed to all available hardware queues)."]
pub struct PushOpQueueGetDoReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpQueueGetDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpQueueGetDoReply<Prev> {
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
    #[doc = "Queue index; most queue types are indexed like a C array, with indexes starting at 0 and ending at queue count - 1. Queue indexes are scoped to an interface and queue type."]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ifindex of the netdevice to which the queue belongs."]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Queue type as rx, tx. Each queue type defines a separate ID space. XDP TX queues allocated in the kernel are not linked to NAPIs and thus not listed. AF_XDP queues will have more information set in the xsk attribute.\nAssociated type: \"QueueType\" (enum)"]
    pub fn push_type(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ID of the NAPI instance which services this queue."]
    pub fn push_napi_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ID of the dmabuf attached to this queue, if any."]
    pub fn push_dmabuf(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 5u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "io_uring memory provider information."]
    pub fn nested_io_uring(mut self) -> PushIoUringProviderInfo<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 6u16);
        PushIoUringProviderInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "XSK information for this queue, if any."]
    pub fn nested_xsk(mut self) -> PushXskInfo<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 7u16);
        PushXskInfo {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushOpQueueGetDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get queue information from the kernel. Only configured queues will be reported (as opposed to all available hardware queues)."]
#[derive(Clone)]
pub enum OpQueueGetDoReply<'a> {
    #[doc = "Queue index; most queue types are indexed like a C array, with indexes starting at 0 and ending at queue count - 1. Queue indexes are scoped to an interface and queue type."]
    Id(u32),
    #[doc = "ifindex of the netdevice to which the queue belongs."]
    Ifindex(u32),
    #[doc = "Queue type as rx, tx. Each queue type defines a separate ID space. XDP TX queues allocated in the kernel are not linked to NAPIs and thus not listed. AF_XDP queues will have more information set in the xsk attribute.\nAssociated type: \"QueueType\" (enum)"]
    Type(u32),
    #[doc = "ID of the NAPI instance which services this queue."]
    NapiId(u32),
    #[doc = "ID of the dmabuf attached to this queue, if any."]
    Dmabuf(u32),
    #[doc = "io_uring memory provider information."]
    IoUring(IterableIoUringProviderInfo<'a>),
    #[doc = "XSK information for this queue, if any."]
    Xsk(IterableXskInfo<'a>),
}
impl<'a> IterableOpQueueGetDoReply<'a> {
    #[doc = "Queue index; most queue types are indexed like a C array, with indexes starting at 0 and ending at queue count - 1. Queue indexes are scoped to an interface and queue type."]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQueueGetDoReply::Id(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQueueGetDoReply",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ifindex of the netdevice to which the queue belongs."]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQueueGetDoReply::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQueueGetDoReply",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Queue type as rx, tx. Each queue type defines a separate ID space. XDP TX queues allocated in the kernel are not linked to NAPIs and thus not listed. AF_XDP queues will have more information set in the xsk attribute.\nAssociated type: \"QueueType\" (enum)"]
    pub fn get_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQueueGetDoReply::Type(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQueueGetDoReply",
            "Type",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ID of the NAPI instance which services this queue."]
    pub fn get_napi_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQueueGetDoReply::NapiId(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQueueGetDoReply",
            "NapiId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ID of the dmabuf attached to this queue, if any."]
    pub fn get_dmabuf(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQueueGetDoReply::Dmabuf(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQueueGetDoReply",
            "Dmabuf",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "io_uring memory provider information."]
    pub fn get_io_uring(&self) -> Result<IterableIoUringProviderInfo<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQueueGetDoReply::IoUring(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQueueGetDoReply",
            "IoUring",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "XSK information for this queue, if any."]
    pub fn get_xsk(&self) -> Result<IterableXskInfo<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQueueGetDoReply::Xsk(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQueueGetDoReply",
            "Xsk",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpQueueGetDoReply<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpQueueGetDoReply<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpQueueGetDoReply::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Queue::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpQueueGetDoReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpQueueGetDoReply<'a> {
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
impl<'a> Iterator for IterableOpQueueGetDoReply<'a> {
    type Item = Result<OpQueueGetDoReply<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpQueueGetDoReply::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpQueueGetDoReply::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpQueueGetDoReply::Type({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpQueueGetDoReply::NapiId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpQueueGetDoReply::Dmabuf({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpQueueGetDoReply::IoUring({
                    let res = Some(IterableIoUringProviderInfo::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => OpQueueGetDoReply::Xsk({
                    let res = Some(IterableXskInfo::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpQueueGetDoReply",
            r#type.and_then(|t| OpQueueGetDoReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpQueueGetDoReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpQueueGetDoReply");
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
                OpQueueGetDoReply::Id(val) => fmt.field("Id", &val),
                OpQueueGetDoReply::Ifindex(val) => fmt.field("Ifindex", &val),
                OpQueueGetDoReply::Type(val) => {
                    fmt.field("Type", &FormatEnum(val.into(), QueueType::from_value))
                }
                OpQueueGetDoReply::NapiId(val) => fmt.field("NapiId", &val),
                OpQueueGetDoReply::Dmabuf(val) => fmt.field("Dmabuf", &val),
                OpQueueGetDoReply::IoUring(val) => fmt.field("IoUring", &val),
                OpQueueGetDoReply::Xsk(val) => fmt.field("Xsk", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpQueueGetDoReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpQueueGetDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpQueueGetDoReply::attr_from_type(t)),
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
                OpQueueGetDoReply::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                OpQueueGetDoReply::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OpQueueGetDoReply::Type(val) => {
                    if last_off == offset {
                        stack.push(("Type", last_off));
                        break;
                    }
                }
                OpQueueGetDoReply::NapiId(val) => {
                    if last_off == offset {
                        stack.push(("NapiId", last_off));
                        break;
                    }
                }
                OpQueueGetDoReply::Dmabuf(val) => {
                    if last_off == offset {
                        stack.push(("Dmabuf", last_off));
                        break;
                    }
                }
                OpQueueGetDoReply::IoUring(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpQueueGetDoReply::Xsk(val) => {
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
            stack.push(("OpQueueGetDoReply", cur));
        }
        (stack, missing)
    }
}
#[derive(Debug)]
pub struct RequestOpQueueGetDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpQueueGetDoRequest<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        PushOpQueueGetDoRequest::write_header(&mut request.buf_mut());
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpQueueGetDoRequest<&mut Vec<u8>> {
        PushOpQueueGetDoRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpQueueGetDoRequest<RequestBuf<'r>> {
        PushOpQueueGetDoRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpQueueGetDoRequest<'_> {
    type ReplyType<'buf> = IterableOpQueueGetDoReply<'buf>;
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpQueueGetDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpQueueGetDoRequest::new(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "Get information about NAPI instances configured on the system."]
pub struct PushOpNapiGetDumpRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpNapiGetDumpRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpNapiGetDumpRequest<Prev> {
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
        header.set_cmd(11u8);
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
    #[doc = "ifindex of the netdevice to which NAPI instance belongs."]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpNapiGetDumpRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get information about NAPI instances configured on the system."]
#[derive(Clone)]
pub enum OpNapiGetDumpRequest {
    #[doc = "ifindex of the netdevice to which NAPI instance belongs."]
    Ifindex(u32),
}
impl<'a> IterableOpNapiGetDumpRequest<'a> {
    #[doc = "ifindex of the netdevice to which NAPI instance belongs."]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNapiGetDumpRequest::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNapiGetDumpRequest",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpNapiGetDumpRequest {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpNapiGetDumpRequest<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpNapiGetDumpRequest::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Napi::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpNapiGetDumpRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpNapiGetDumpRequest<'a> {
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
impl<'a> Iterator for IterableOpNapiGetDumpRequest<'a> {
    type Item = Result<OpNapiGetDumpRequest, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpNapiGetDumpRequest::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpNapiGetDumpRequest",
            r#type.and_then(|t| OpNapiGetDumpRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpNapiGetDumpRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpNapiGetDumpRequest");
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
                OpNapiGetDumpRequest::Ifindex(val) => fmt.field("Ifindex", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpNapiGetDumpRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpNapiGetDumpRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpNapiGetDumpRequest::attr_from_type(t)),
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
                OpNapiGetDumpRequest::Ifindex(val) => {
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
            stack.push(("OpNapiGetDumpRequest", cur));
        }
        (stack, None)
    }
}
#[doc = "Get information about NAPI instances configured on the system."]
pub struct PushOpNapiGetDumpReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpNapiGetDumpReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpNapiGetDumpReply<Prev> {
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
        header.set_cmd(11u8);
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
    #[doc = "ifindex of the netdevice to which NAPI instance belongs."]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ID of the NAPI instance."]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The associated interrupt vector number for the napi"]
    pub fn push_irq(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "PID of the napi thread, if NAPI is configured to operate in threaded mode. If NAPI is not in threaded mode (i.e. uses normal softirq context), the attribute will be absent."]
    pub fn push_pid(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The number of consecutive empty polls before IRQ deferral ends and hardware IRQs are re-enabled."]
    pub fn push_defer_hard_irqs(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 5u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The timeout, in nanoseconds, of when to trigger the NAPI watchdog timer which schedules NAPI processing. Additionally, a non-zero value will also prevent GRO from flushing recent super-frames at the end of a NAPI cycle. This may add receive latency in exchange for reducing the number of frames processed by the network stack."]
    pub fn push_gro_flush_timeout(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 6u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The timeout, in nanoseconds, of how long to suspend irq processing, if event polling finds events"]
    pub fn push_irq_suspend_timeout(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 7u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether the NAPI is configured to operate in threaded polling mode. If this is set to enabled then the NAPI context operates in threaded polling mode. If this is set to busy-poll, then the threaded polling mode also busy polls.\nAssociated type: \"NapiThreaded\" (enum)"]
    pub fn push_threaded(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpNapiGetDumpReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get information about NAPI instances configured on the system."]
#[derive(Clone)]
pub enum OpNapiGetDumpReply {
    #[doc = "ifindex of the netdevice to which NAPI instance belongs."]
    Ifindex(u32),
    #[doc = "ID of the NAPI instance."]
    Id(u32),
    #[doc = "The associated interrupt vector number for the napi"]
    Irq(u32),
    #[doc = "PID of the napi thread, if NAPI is configured to operate in threaded mode. If NAPI is not in threaded mode (i.e. uses normal softirq context), the attribute will be absent."]
    Pid(u32),
    #[doc = "The number of consecutive empty polls before IRQ deferral ends and hardware IRQs are re-enabled."]
    DeferHardIrqs(u32),
    #[doc = "The timeout, in nanoseconds, of when to trigger the NAPI watchdog timer which schedules NAPI processing. Additionally, a non-zero value will also prevent GRO from flushing recent super-frames at the end of a NAPI cycle. This may add receive latency in exchange for reducing the number of frames processed by the network stack."]
    GroFlushTimeout(u32),
    #[doc = "The timeout, in nanoseconds, of how long to suspend irq processing, if event polling finds events"]
    IrqSuspendTimeout(u32),
    #[doc = "Whether the NAPI is configured to operate in threaded polling mode. If this is set to enabled then the NAPI context operates in threaded polling mode. If this is set to busy-poll, then the threaded polling mode also busy polls.\nAssociated type: \"NapiThreaded\" (enum)"]
    Threaded(u32),
}
impl<'a> IterableOpNapiGetDumpReply<'a> {
    #[doc = "ifindex of the netdevice to which NAPI instance belongs."]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNapiGetDumpReply::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNapiGetDumpReply",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ID of the NAPI instance."]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNapiGetDumpReply::Id(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNapiGetDumpReply",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The associated interrupt vector number for the napi"]
    pub fn get_irq(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNapiGetDumpReply::Irq(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNapiGetDumpReply",
            "Irq",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "PID of the napi thread, if NAPI is configured to operate in threaded mode. If NAPI is not in threaded mode (i.e. uses normal softirq context), the attribute will be absent."]
    pub fn get_pid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNapiGetDumpReply::Pid(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNapiGetDumpReply",
            "Pid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The number of consecutive empty polls before IRQ deferral ends and hardware IRQs are re-enabled."]
    pub fn get_defer_hard_irqs(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNapiGetDumpReply::DeferHardIrqs(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNapiGetDumpReply",
            "DeferHardIrqs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The timeout, in nanoseconds, of when to trigger the NAPI watchdog timer which schedules NAPI processing. Additionally, a non-zero value will also prevent GRO from flushing recent super-frames at the end of a NAPI cycle. This may add receive latency in exchange for reducing the number of frames processed by the network stack."]
    pub fn get_gro_flush_timeout(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNapiGetDumpReply::GroFlushTimeout(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNapiGetDumpReply",
            "GroFlushTimeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The timeout, in nanoseconds, of how long to suspend irq processing, if event polling finds events"]
    pub fn get_irq_suspend_timeout(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNapiGetDumpReply::IrqSuspendTimeout(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNapiGetDumpReply",
            "IrqSuspendTimeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether the NAPI is configured to operate in threaded polling mode. If this is set to enabled then the NAPI context operates in threaded polling mode. If this is set to busy-poll, then the threaded polling mode also busy polls.\nAssociated type: \"NapiThreaded\" (enum)"]
    pub fn get_threaded(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNapiGetDumpReply::Threaded(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNapiGetDumpReply",
            "Threaded",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpNapiGetDumpReply {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpNapiGetDumpReply<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpNapiGetDumpReply::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Napi::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpNapiGetDumpReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpNapiGetDumpReply<'a> {
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
impl<'a> Iterator for IterableOpNapiGetDumpReply<'a> {
    type Item = Result<OpNapiGetDumpReply, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpNapiGetDumpReply::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpNapiGetDumpReply::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpNapiGetDumpReply::Irq({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpNapiGetDumpReply::Pid({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpNapiGetDumpReply::DeferHardIrqs({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpNapiGetDumpReply::GroFlushTimeout({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => OpNapiGetDumpReply::IrqSuspendTimeout({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpNapiGetDumpReply::Threaded({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpNapiGetDumpReply",
            r#type.and_then(|t| OpNapiGetDumpReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpNapiGetDumpReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpNapiGetDumpReply");
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
                OpNapiGetDumpReply::Ifindex(val) => fmt.field("Ifindex", &val),
                OpNapiGetDumpReply::Id(val) => fmt.field("Id", &val),
                OpNapiGetDumpReply::Irq(val) => fmt.field("Irq", &val),
                OpNapiGetDumpReply::Pid(val) => fmt.field("Pid", &val),
                OpNapiGetDumpReply::DeferHardIrqs(val) => fmt.field("DeferHardIrqs", &val),
                OpNapiGetDumpReply::GroFlushTimeout(val) => fmt.field("GroFlushTimeout", &val),
                OpNapiGetDumpReply::IrqSuspendTimeout(val) => fmt.field("IrqSuspendTimeout", &val),
                OpNapiGetDumpReply::Threaded(val) => fmt.field(
                    "Threaded",
                    &FormatEnum(val.into(), NapiThreaded::from_value),
                ),
            };
        }
        fmt.finish()
    }
}
impl IterableOpNapiGetDumpReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpNapiGetDumpReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpNapiGetDumpReply::attr_from_type(t)),
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
                OpNapiGetDumpReply::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OpNapiGetDumpReply::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                OpNapiGetDumpReply::Irq(val) => {
                    if last_off == offset {
                        stack.push(("Irq", last_off));
                        break;
                    }
                }
                OpNapiGetDumpReply::Pid(val) => {
                    if last_off == offset {
                        stack.push(("Pid", last_off));
                        break;
                    }
                }
                OpNapiGetDumpReply::DeferHardIrqs(val) => {
                    if last_off == offset {
                        stack.push(("DeferHardIrqs", last_off));
                        break;
                    }
                }
                OpNapiGetDumpReply::GroFlushTimeout(val) => {
                    if last_off == offset {
                        stack.push(("GroFlushTimeout", last_off));
                        break;
                    }
                }
                OpNapiGetDumpReply::IrqSuspendTimeout(val) => {
                    if last_off == offset {
                        stack.push(("IrqSuspendTimeout", last_off));
                        break;
                    }
                }
                OpNapiGetDumpReply::Threaded(val) => {
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
            stack.push(("OpNapiGetDumpReply", cur));
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpNapiGetDumpRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpNapiGetDumpRequest<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        PushOpNapiGetDumpRequest::write_header(&mut request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode(&mut self) -> PushOpNapiGetDumpRequest<&mut Vec<u8>> {
        PushOpNapiGetDumpRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpNapiGetDumpRequest<RequestBuf<'r>> {
        PushOpNapiGetDumpRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpNapiGetDumpRequest<'_> {
    type ReplyType<'buf> = IterableOpNapiGetDumpReply<'buf>;
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpNapiGetDumpReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpNapiGetDumpRequest::new(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "Get information about NAPI instances configured on the system."]
pub struct PushOpNapiGetDoRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpNapiGetDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpNapiGetDoRequest<Prev> {
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
        header.set_cmd(11u8);
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
    #[doc = "ID of the NAPI instance."]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpNapiGetDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get information about NAPI instances configured on the system."]
#[derive(Clone)]
pub enum OpNapiGetDoRequest {
    #[doc = "ID of the NAPI instance."]
    Id(u32),
}
impl<'a> IterableOpNapiGetDoRequest<'a> {
    #[doc = "ID of the NAPI instance."]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNapiGetDoRequest::Id(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNapiGetDoRequest",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpNapiGetDoRequest {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpNapiGetDoRequest<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpNapiGetDoRequest::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Napi::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpNapiGetDoRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpNapiGetDoRequest<'a> {
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
impl<'a> Iterator for IterableOpNapiGetDoRequest<'a> {
    type Item = Result<OpNapiGetDoRequest, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                2u16 => OpNapiGetDoRequest::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpNapiGetDoRequest",
            r#type.and_then(|t| OpNapiGetDoRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpNapiGetDoRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpNapiGetDoRequest");
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
                OpNapiGetDoRequest::Id(val) => fmt.field("Id", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpNapiGetDoRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpNapiGetDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpNapiGetDoRequest::attr_from_type(t)),
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
                OpNapiGetDoRequest::Id(val) => {
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
            stack.push(("OpNapiGetDoRequest", cur));
        }
        (stack, None)
    }
}
#[doc = "Get information about NAPI instances configured on the system."]
pub struct PushOpNapiGetDoReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpNapiGetDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpNapiGetDoReply<Prev> {
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
        header.set_cmd(11u8);
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
    #[doc = "ifindex of the netdevice to which NAPI instance belongs."]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "ID of the NAPI instance."]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The associated interrupt vector number for the napi"]
    pub fn push_irq(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "PID of the napi thread, if NAPI is configured to operate in threaded mode. If NAPI is not in threaded mode (i.e. uses normal softirq context), the attribute will be absent."]
    pub fn push_pid(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The number of consecutive empty polls before IRQ deferral ends and hardware IRQs are re-enabled."]
    pub fn push_defer_hard_irqs(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 5u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The timeout, in nanoseconds, of when to trigger the NAPI watchdog timer which schedules NAPI processing. Additionally, a non-zero value will also prevent GRO from flushing recent super-frames at the end of a NAPI cycle. This may add receive latency in exchange for reducing the number of frames processed by the network stack."]
    pub fn push_gro_flush_timeout(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 6u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The timeout, in nanoseconds, of how long to suspend irq processing, if event polling finds events"]
    pub fn push_irq_suspend_timeout(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 7u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether the NAPI is configured to operate in threaded polling mode. If this is set to enabled then the NAPI context operates in threaded polling mode. If this is set to busy-poll, then the threaded polling mode also busy polls.\nAssociated type: \"NapiThreaded\" (enum)"]
    pub fn push_threaded(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpNapiGetDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get information about NAPI instances configured on the system."]
#[derive(Clone)]
pub enum OpNapiGetDoReply {
    #[doc = "ifindex of the netdevice to which NAPI instance belongs."]
    Ifindex(u32),
    #[doc = "ID of the NAPI instance."]
    Id(u32),
    #[doc = "The associated interrupt vector number for the napi"]
    Irq(u32),
    #[doc = "PID of the napi thread, if NAPI is configured to operate in threaded mode. If NAPI is not in threaded mode (i.e. uses normal softirq context), the attribute will be absent."]
    Pid(u32),
    #[doc = "The number of consecutive empty polls before IRQ deferral ends and hardware IRQs are re-enabled."]
    DeferHardIrqs(u32),
    #[doc = "The timeout, in nanoseconds, of when to trigger the NAPI watchdog timer which schedules NAPI processing. Additionally, a non-zero value will also prevent GRO from flushing recent super-frames at the end of a NAPI cycle. This may add receive latency in exchange for reducing the number of frames processed by the network stack."]
    GroFlushTimeout(u32),
    #[doc = "The timeout, in nanoseconds, of how long to suspend irq processing, if event polling finds events"]
    IrqSuspendTimeout(u32),
    #[doc = "Whether the NAPI is configured to operate in threaded polling mode. If this is set to enabled then the NAPI context operates in threaded polling mode. If this is set to busy-poll, then the threaded polling mode also busy polls.\nAssociated type: \"NapiThreaded\" (enum)"]
    Threaded(u32),
}
impl<'a> IterableOpNapiGetDoReply<'a> {
    #[doc = "ifindex of the netdevice to which NAPI instance belongs."]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNapiGetDoReply::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNapiGetDoReply",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "ID of the NAPI instance."]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNapiGetDoReply::Id(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNapiGetDoReply",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The associated interrupt vector number for the napi"]
    pub fn get_irq(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNapiGetDoReply::Irq(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNapiGetDoReply",
            "Irq",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "PID of the napi thread, if NAPI is configured to operate in threaded mode. If NAPI is not in threaded mode (i.e. uses normal softirq context), the attribute will be absent."]
    pub fn get_pid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNapiGetDoReply::Pid(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNapiGetDoReply",
            "Pid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The number of consecutive empty polls before IRQ deferral ends and hardware IRQs are re-enabled."]
    pub fn get_defer_hard_irqs(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNapiGetDoReply::DeferHardIrqs(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNapiGetDoReply",
            "DeferHardIrqs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The timeout, in nanoseconds, of when to trigger the NAPI watchdog timer which schedules NAPI processing. Additionally, a non-zero value will also prevent GRO from flushing recent super-frames at the end of a NAPI cycle. This may add receive latency in exchange for reducing the number of frames processed by the network stack."]
    pub fn get_gro_flush_timeout(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNapiGetDoReply::GroFlushTimeout(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNapiGetDoReply",
            "GroFlushTimeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The timeout, in nanoseconds, of how long to suspend irq processing, if event polling finds events"]
    pub fn get_irq_suspend_timeout(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNapiGetDoReply::IrqSuspendTimeout(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNapiGetDoReply",
            "IrqSuspendTimeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether the NAPI is configured to operate in threaded polling mode. If this is set to enabled then the NAPI context operates in threaded polling mode. If this is set to busy-poll, then the threaded polling mode also busy polls.\nAssociated type: \"NapiThreaded\" (enum)"]
    pub fn get_threaded(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNapiGetDoReply::Threaded(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNapiGetDoReply",
            "Threaded",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpNapiGetDoReply {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpNapiGetDoReply<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpNapiGetDoReply::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Napi::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpNapiGetDoReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpNapiGetDoReply<'a> {
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
impl<'a> Iterator for IterableOpNapiGetDoReply<'a> {
    type Item = Result<OpNapiGetDoReply, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpNapiGetDoReply::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpNapiGetDoReply::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpNapiGetDoReply::Irq({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpNapiGetDoReply::Pid({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpNapiGetDoReply::DeferHardIrqs({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpNapiGetDoReply::GroFlushTimeout({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => OpNapiGetDoReply::IrqSuspendTimeout({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpNapiGetDoReply::Threaded({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpNapiGetDoReply",
            r#type.and_then(|t| OpNapiGetDoReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpNapiGetDoReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpNapiGetDoReply");
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
                OpNapiGetDoReply::Ifindex(val) => fmt.field("Ifindex", &val),
                OpNapiGetDoReply::Id(val) => fmt.field("Id", &val),
                OpNapiGetDoReply::Irq(val) => fmt.field("Irq", &val),
                OpNapiGetDoReply::Pid(val) => fmt.field("Pid", &val),
                OpNapiGetDoReply::DeferHardIrqs(val) => fmt.field("DeferHardIrqs", &val),
                OpNapiGetDoReply::GroFlushTimeout(val) => fmt.field("GroFlushTimeout", &val),
                OpNapiGetDoReply::IrqSuspendTimeout(val) => fmt.field("IrqSuspendTimeout", &val),
                OpNapiGetDoReply::Threaded(val) => fmt.field(
                    "Threaded",
                    &FormatEnum(val.into(), NapiThreaded::from_value),
                ),
            };
        }
        fmt.finish()
    }
}
impl IterableOpNapiGetDoReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpNapiGetDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpNapiGetDoReply::attr_from_type(t)),
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
                OpNapiGetDoReply::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OpNapiGetDoReply::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                OpNapiGetDoReply::Irq(val) => {
                    if last_off == offset {
                        stack.push(("Irq", last_off));
                        break;
                    }
                }
                OpNapiGetDoReply::Pid(val) => {
                    if last_off == offset {
                        stack.push(("Pid", last_off));
                        break;
                    }
                }
                OpNapiGetDoReply::DeferHardIrqs(val) => {
                    if last_off == offset {
                        stack.push(("DeferHardIrqs", last_off));
                        break;
                    }
                }
                OpNapiGetDoReply::GroFlushTimeout(val) => {
                    if last_off == offset {
                        stack.push(("GroFlushTimeout", last_off));
                        break;
                    }
                }
                OpNapiGetDoReply::IrqSuspendTimeout(val) => {
                    if last_off == offset {
                        stack.push(("IrqSuspendTimeout", last_off));
                        break;
                    }
                }
                OpNapiGetDoReply::Threaded(val) => {
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
            stack.push(("OpNapiGetDoReply", cur));
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpNapiGetDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpNapiGetDoRequest<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        PushOpNapiGetDoRequest::write_header(&mut request.buf_mut());
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpNapiGetDoRequest<&mut Vec<u8>> {
        PushOpNapiGetDoRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpNapiGetDoRequest<RequestBuf<'r>> {
        PushOpNapiGetDoRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpNapiGetDoRequest<'_> {
    type ReplyType<'buf> = IterableOpNapiGetDoReply<'buf>;
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpNapiGetDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpNapiGetDoRequest::new(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "Get / dump fine grained statistics. Which statistics are reported\ndepends on the device and the driver, and whether the driver stores\nsoftware counters per-queue.\n"]
pub struct PushOpQstatsGetDumpRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpQstatsGetDumpRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpQstatsGetDumpRequest<Prev> {
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
        header.set_cmd(12u8);
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
    #[doc = "ifindex of the netdevice to which stats belong."]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "What object type should be used to iterate over the stats.\n\nAssociated type: \"QstatsScope\" (enum)"]
    pub fn push_scope(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpQstatsGetDumpRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get / dump fine grained statistics. Which statistics are reported\ndepends on the device and the driver, and whether the driver stores\nsoftware counters per-queue.\n"]
#[derive(Clone)]
pub enum OpQstatsGetDumpRequest {
    #[doc = "ifindex of the netdevice to which stats belong."]
    Ifindex(u32),
    #[doc = "What object type should be used to iterate over the stats.\n\nAssociated type: \"QstatsScope\" (enum)"]
    Scope(u32),
}
impl<'a> IterableOpQstatsGetDumpRequest<'a> {
    #[doc = "ifindex of the netdevice to which stats belong."]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpRequest::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpRequest",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "What object type should be used to iterate over the stats.\n\nAssociated type: \"QstatsScope\" (enum)"]
    pub fn get_scope(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpRequest::Scope(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpRequest",
            "Scope",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpQstatsGetDumpRequest {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpQstatsGetDumpRequest<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpQstatsGetDumpRequest::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Qstats::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpQstatsGetDumpRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpQstatsGetDumpRequest<'a> {
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
impl<'a> Iterator for IterableOpQstatsGetDumpRequest<'a> {
    type Item = Result<OpQstatsGetDumpRequest, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpQstatsGetDumpRequest::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpQstatsGetDumpRequest::Scope({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpQstatsGetDumpRequest",
            r#type.and_then(|t| OpQstatsGetDumpRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpQstatsGetDumpRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpQstatsGetDumpRequest");
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
                OpQstatsGetDumpRequest::Ifindex(val) => fmt.field("Ifindex", &val),
                OpQstatsGetDumpRequest::Scope(val) => {
                    fmt.field("Scope", &FormatFlags(val.into(), QstatsScope::from_value))
                }
            };
        }
        fmt.finish()
    }
}
impl IterableOpQstatsGetDumpRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpQstatsGetDumpRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpQstatsGetDumpRequest::attr_from_type(t)),
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
                OpQstatsGetDumpRequest::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpRequest::Scope(val) => {
                    if last_off == offset {
                        stack.push(("Scope", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpQstatsGetDumpRequest", cur));
        }
        (stack, None)
    }
}
#[doc = "Get / dump fine grained statistics. Which statistics are reported\ndepends on the device and the driver, and whether the driver stores\nsoftware counters per-queue.\n"]
pub struct PushOpQstatsGetDumpReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpQstatsGetDumpReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpQstatsGetDumpReply<Prev> {
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
        header.set_cmd(12u8);
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
    #[doc = "ifindex of the netdevice to which stats belong."]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Queue type as rx, tx, for queue-id.\nAssociated type: \"QueueType\" (enum)"]
    pub fn push_queue_type(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Queue ID, if stats are scoped to a single queue instance."]
    pub fn push_queue_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of wire packets successfully received and passed to the stack.\nFor drivers supporting XDP, XDP is considered the first layer\nof the stack, so packets consumed by XDP are still counted here.\n"]
    pub fn push_rx_packets(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Successfully received bytes, see `rx-packets`."]
    pub fn push_rx_bytes(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 9u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of wire packets successfully sent. Packet is considered to be\nsuccessfully sent once it is in device memory (usually this means\nthe device has issued a DMA completion for the packet).\n"]
    pub fn push_tx_packets(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 10u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Successfully sent bytes, see `tx-packets`."]
    pub fn push_tx_bytes(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 11u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of times skb or buffer allocation failed on the Rx datapath.\nAllocation failure may, or may not result in a packet drop, depending\non driver implementation and whether system recovers quickly.\n"]
    pub fn push_rx_alloc_fail(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 12u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of all packets which entered the device, but never left it,\nincluding but not limited to: packets dropped due to lack of buffer\nspace, processing errors, explicit or implicit policies and packet\nfilters.\n"]
    pub fn push_rx_hw_drops(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 13u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets dropped due to transient lack of resources, such as\nbuffer space, host descriptors etc.\n"]
    pub fn push_rx_hw_drop_overruns(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 14u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that were marked as CHECKSUM_COMPLETE."]
    pub fn push_rx_csum_complete(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 15u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that were marked as CHECKSUM_UNNECESSARY."]
    pub fn push_rx_csum_unnecessary(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 16u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that were not checksummed by device."]
    pub fn push_rx_csum_none(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 17u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets with bad checksum. The packets are not discarded,\nbut still delivered to the stack.\n"]
    pub fn push_rx_csum_bad(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 18u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that were coalesced from smaller packets by the\ndevice. Counts only packets coalesced with the HW-GRO netdevice\nfeature, LRO-coalesced packets are not counted.\n"]
    pub fn push_rx_hw_gro_packets(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 19u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "See `rx-hw-gro-packets`."]
    pub fn push_rx_hw_gro_bytes(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 20u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that were coalesced to bigger packetss with the\nHW-GRO netdevice feature. LRO-coalesced packets are not counted.\n"]
    pub fn push_rx_hw_gro_wire_packets(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 21u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "See `rx-hw-gro-wire-packets`."]
    pub fn push_rx_hw_gro_wire_bytes(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 22u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of the packets dropped by the device due to the received\npackets bitrate exceeding the device rate limit.\n"]
    pub fn push_rx_hw_drop_ratelimits(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 23u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that arrived at the device but never left it,\nencompassing packets dropped for reasons such as processing errors, as\nwell as those affected by explicitly defined policies and packet\nfiltering criteria.\n"]
    pub fn push_tx_hw_drops(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 24u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets dropped because they were invalid or malformed."]
    pub fn push_tx_hw_drop_errors(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 25u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that did not require the device to calculate the\nchecksum.\n"]
    pub fn push_tx_csum_none(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 26u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that required the device to calculate the checksum.\nThis counter includes the number of GSO wire packets for which device\ncalculated the L4 checksum.\n"]
    pub fn push_tx_needs_csum(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 27u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets that necessitated segmentation into smaller packets\nby the device.\n"]
    pub fn push_tx_hw_gso_packets(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 28u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "See `tx-hw-gso-packets`."]
    pub fn push_tx_hw_gso_bytes(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 29u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of wire-sized packets generated by processing\n`tx-hw-gso-packets`\n"]
    pub fn push_tx_hw_gso_wire_packets(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 30u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "See `tx-hw-gso-wire-packets`."]
    pub fn push_tx_hw_gso_wire_bytes(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 31u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of the packets dropped by the device due to the transmit\npackets bitrate exceeding the device rate limit.\n"]
    pub fn push_tx_hw_drop_ratelimits(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 32u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of times driver paused accepting new tx packets\nfrom the stack to this queue, because the queue was full.\nNote that if BQL is supported and enabled on the device\nthe networking stack will avoid queuing a lot of data at once.\n"]
    pub fn push_tx_stop(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 33u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of times driver re-started accepting send\nrequests to this queue from the stack.\n"]
    pub fn push_tx_wake(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 34u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpQstatsGetDumpReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get / dump fine grained statistics. Which statistics are reported\ndepends on the device and the driver, and whether the driver stores\nsoftware counters per-queue.\n"]
#[derive(Clone)]
pub enum OpQstatsGetDumpReply {
    #[doc = "ifindex of the netdevice to which stats belong."]
    Ifindex(u32),
    #[doc = "Queue type as rx, tx, for queue-id.\nAssociated type: \"QueueType\" (enum)"]
    QueueType(u32),
    #[doc = "Queue ID, if stats are scoped to a single queue instance."]
    QueueId(u32),
    #[doc = "Number of wire packets successfully received and passed to the stack.\nFor drivers supporting XDP, XDP is considered the first layer\nof the stack, so packets consumed by XDP are still counted here.\n"]
    RxPackets(u32),
    #[doc = "Successfully received bytes, see `rx-packets`."]
    RxBytes(u32),
    #[doc = "Number of wire packets successfully sent. Packet is considered to be\nsuccessfully sent once it is in device memory (usually this means\nthe device has issued a DMA completion for the packet).\n"]
    TxPackets(u32),
    #[doc = "Successfully sent bytes, see `tx-packets`."]
    TxBytes(u32),
    #[doc = "Number of times skb or buffer allocation failed on the Rx datapath.\nAllocation failure may, or may not result in a packet drop, depending\non driver implementation and whether system recovers quickly.\n"]
    RxAllocFail(u32),
    #[doc = "Number of all packets which entered the device, but never left it,\nincluding but not limited to: packets dropped due to lack of buffer\nspace, processing errors, explicit or implicit policies and packet\nfilters.\n"]
    RxHwDrops(u32),
    #[doc = "Number of packets dropped due to transient lack of resources, such as\nbuffer space, host descriptors etc.\n"]
    RxHwDropOverruns(u32),
    #[doc = "Number of packets that were marked as CHECKSUM_COMPLETE."]
    RxCsumComplete(u32),
    #[doc = "Number of packets that were marked as CHECKSUM_UNNECESSARY."]
    RxCsumUnnecessary(u32),
    #[doc = "Number of packets that were not checksummed by device."]
    RxCsumNone(u32),
    #[doc = "Number of packets with bad checksum. The packets are not discarded,\nbut still delivered to the stack.\n"]
    RxCsumBad(u32),
    #[doc = "Number of packets that were coalesced from smaller packets by the\ndevice. Counts only packets coalesced with the HW-GRO netdevice\nfeature, LRO-coalesced packets are not counted.\n"]
    RxHwGroPackets(u32),
    #[doc = "See `rx-hw-gro-packets`."]
    RxHwGroBytes(u32),
    #[doc = "Number of packets that were coalesced to bigger packetss with the\nHW-GRO netdevice feature. LRO-coalesced packets are not counted.\n"]
    RxHwGroWirePackets(u32),
    #[doc = "See `rx-hw-gro-wire-packets`."]
    RxHwGroWireBytes(u32),
    #[doc = "Number of the packets dropped by the device due to the received\npackets bitrate exceeding the device rate limit.\n"]
    RxHwDropRatelimits(u32),
    #[doc = "Number of packets that arrived at the device but never left it,\nencompassing packets dropped for reasons such as processing errors, as\nwell as those affected by explicitly defined policies and packet\nfiltering criteria.\n"]
    TxHwDrops(u32),
    #[doc = "Number of packets dropped because they were invalid or malformed."]
    TxHwDropErrors(u32),
    #[doc = "Number of packets that did not require the device to calculate the\nchecksum.\n"]
    TxCsumNone(u32),
    #[doc = "Number of packets that required the device to calculate the checksum.\nThis counter includes the number of GSO wire packets for which device\ncalculated the L4 checksum.\n"]
    TxNeedsCsum(u32),
    #[doc = "Number of packets that necessitated segmentation into smaller packets\nby the device.\n"]
    TxHwGsoPackets(u32),
    #[doc = "See `tx-hw-gso-packets`."]
    TxHwGsoBytes(u32),
    #[doc = "Number of wire-sized packets generated by processing\n`tx-hw-gso-packets`\n"]
    TxHwGsoWirePackets(u32),
    #[doc = "See `tx-hw-gso-wire-packets`."]
    TxHwGsoWireBytes(u32),
    #[doc = "Number of the packets dropped by the device due to the transmit\npackets bitrate exceeding the device rate limit.\n"]
    TxHwDropRatelimits(u32),
    #[doc = "Number of times driver paused accepting new tx packets\nfrom the stack to this queue, because the queue was full.\nNote that if BQL is supported and enabled on the device\nthe networking stack will avoid queuing a lot of data at once.\n"]
    TxStop(u32),
    #[doc = "Number of times driver re-started accepting send\nrequests to this queue from the stack.\n"]
    TxWake(u32),
}
impl<'a> IterableOpQstatsGetDumpReply<'a> {
    #[doc = "ifindex of the netdevice to which stats belong."]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpReply::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Queue type as rx, tx, for queue-id.\nAssociated type: \"QueueType\" (enum)"]
    pub fn get_queue_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpReply::QueueType(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
            "QueueType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Queue ID, if stats are scoped to a single queue instance."]
    pub fn get_queue_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpReply::QueueId(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
            "QueueId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of wire packets successfully received and passed to the stack.\nFor drivers supporting XDP, XDP is considered the first layer\nof the stack, so packets consumed by XDP are still counted here.\n"]
    pub fn get_rx_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpReply::RxPackets(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
            "RxPackets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Successfully received bytes, see `rx-packets`."]
    pub fn get_rx_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpReply::RxBytes(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
            "RxBytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of wire packets successfully sent. Packet is considered to be\nsuccessfully sent once it is in device memory (usually this means\nthe device has issued a DMA completion for the packet).\n"]
    pub fn get_tx_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpReply::TxPackets(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
            "TxPackets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Successfully sent bytes, see `tx-packets`."]
    pub fn get_tx_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpReply::TxBytes(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
            "TxBytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of times skb or buffer allocation failed on the Rx datapath.\nAllocation failure may, or may not result in a packet drop, depending\non driver implementation and whether system recovers quickly.\n"]
    pub fn get_rx_alloc_fail(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpReply::RxAllocFail(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
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
            if let OpQstatsGetDumpReply::RxHwDrops(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
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
            if let OpQstatsGetDumpReply::RxHwDropOverruns(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
            "RxHwDropOverruns",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of packets that were marked as CHECKSUM_COMPLETE."]
    pub fn get_rx_csum_complete(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpReply::RxCsumComplete(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
            "RxCsumComplete",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of packets that were marked as CHECKSUM_UNNECESSARY."]
    pub fn get_rx_csum_unnecessary(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpReply::RxCsumUnnecessary(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
            "RxCsumUnnecessary",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of packets that were not checksummed by device."]
    pub fn get_rx_csum_none(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpReply::RxCsumNone(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
            "RxCsumNone",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of packets with bad checksum. The packets are not discarded,\nbut still delivered to the stack.\n"]
    pub fn get_rx_csum_bad(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpReply::RxCsumBad(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
            "RxCsumBad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of packets that were coalesced from smaller packets by the\ndevice. Counts only packets coalesced with the HW-GRO netdevice\nfeature, LRO-coalesced packets are not counted.\n"]
    pub fn get_rx_hw_gro_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpReply::RxHwGroPackets(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
            "RxHwGroPackets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "See `rx-hw-gro-packets`."]
    pub fn get_rx_hw_gro_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpReply::RxHwGroBytes(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
            "RxHwGroBytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of packets that were coalesced to bigger packetss with the\nHW-GRO netdevice feature. LRO-coalesced packets are not counted.\n"]
    pub fn get_rx_hw_gro_wire_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpReply::RxHwGroWirePackets(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
            "RxHwGroWirePackets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "See `rx-hw-gro-wire-packets`."]
    pub fn get_rx_hw_gro_wire_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpReply::RxHwGroWireBytes(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
            "RxHwGroWireBytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of the packets dropped by the device due to the received\npackets bitrate exceeding the device rate limit.\n"]
    pub fn get_rx_hw_drop_ratelimits(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpReply::RxHwDropRatelimits(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
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
            if let OpQstatsGetDumpReply::TxHwDrops(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
            "TxHwDrops",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of packets dropped because they were invalid or malformed."]
    pub fn get_tx_hw_drop_errors(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpReply::TxHwDropErrors(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
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
            if let OpQstatsGetDumpReply::TxCsumNone(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
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
            if let OpQstatsGetDumpReply::TxNeedsCsum(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
            "TxNeedsCsum",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of packets that necessitated segmentation into smaller packets\nby the device.\n"]
    pub fn get_tx_hw_gso_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpReply::TxHwGsoPackets(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
            "TxHwGsoPackets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "See `tx-hw-gso-packets`."]
    pub fn get_tx_hw_gso_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpReply::TxHwGsoBytes(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
            "TxHwGsoBytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of wire-sized packets generated by processing\n`tx-hw-gso-packets`\n"]
    pub fn get_tx_hw_gso_wire_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpReply::TxHwGsoWirePackets(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
            "TxHwGsoWirePackets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "See `tx-hw-gso-wire-packets`."]
    pub fn get_tx_hw_gso_wire_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpReply::TxHwGsoWireBytes(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
            "TxHwGsoWireBytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of the packets dropped by the device due to the transmit\npackets bitrate exceeding the device rate limit.\n"]
    pub fn get_tx_hw_drop_ratelimits(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpReply::TxHwDropRatelimits(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
            "TxHwDropRatelimits",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of times driver paused accepting new tx packets\nfrom the stack to this queue, because the queue was full.\nNote that if BQL is supported and enabled on the device\nthe networking stack will avoid queuing a lot of data at once.\n"]
    pub fn get_tx_stop(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpReply::TxStop(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
            "TxStop",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of times driver re-started accepting send\nrequests to this queue from the stack.\n"]
    pub fn get_tx_wake(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpQstatsGetDumpReply::TxWake(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpQstatsGetDumpReply",
            "TxWake",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpQstatsGetDumpReply {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpQstatsGetDumpReply<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpQstatsGetDumpReply::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Qstats::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpQstatsGetDumpReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpQstatsGetDumpReply<'a> {
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
impl<'a> Iterator for IterableOpQstatsGetDumpReply<'a> {
    type Item = Result<OpQstatsGetDumpReply, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpQstatsGetDumpReply::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpQstatsGetDumpReply::QueueType({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpQstatsGetDumpReply::QueueId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpQstatsGetDumpReply::RxPackets({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => OpQstatsGetDumpReply::RxBytes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => OpQstatsGetDumpReply::TxPackets({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => OpQstatsGetDumpReply::TxBytes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => OpQstatsGetDumpReply::RxAllocFail({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => OpQstatsGetDumpReply::RxHwDrops({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => OpQstatsGetDumpReply::RxHwDropOverruns({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => OpQstatsGetDumpReply::RxCsumComplete({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => OpQstatsGetDumpReply::RxCsumUnnecessary({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => OpQstatsGetDumpReply::RxCsumNone({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => OpQstatsGetDumpReply::RxCsumBad({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => OpQstatsGetDumpReply::RxHwGroPackets({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => OpQstatsGetDumpReply::RxHwGroBytes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => OpQstatsGetDumpReply::RxHwGroWirePackets({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => OpQstatsGetDumpReply::RxHwGroWireBytes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => OpQstatsGetDumpReply::RxHwDropRatelimits({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => OpQstatsGetDumpReply::TxHwDrops({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                25u16 => OpQstatsGetDumpReply::TxHwDropErrors({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                26u16 => OpQstatsGetDumpReply::TxCsumNone({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                27u16 => OpQstatsGetDumpReply::TxNeedsCsum({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                28u16 => OpQstatsGetDumpReply::TxHwGsoPackets({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                29u16 => OpQstatsGetDumpReply::TxHwGsoBytes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                30u16 => OpQstatsGetDumpReply::TxHwGsoWirePackets({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                31u16 => OpQstatsGetDumpReply::TxHwGsoWireBytes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                32u16 => OpQstatsGetDumpReply::TxHwDropRatelimits({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                33u16 => OpQstatsGetDumpReply::TxStop({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                34u16 => OpQstatsGetDumpReply::TxWake({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpQstatsGetDumpReply",
            r#type.and_then(|t| OpQstatsGetDumpReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpQstatsGetDumpReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpQstatsGetDumpReply");
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
                OpQstatsGetDumpReply::Ifindex(val) => fmt.field("Ifindex", &val),
                OpQstatsGetDumpReply::QueueType(val) => {
                    fmt.field("QueueType", &FormatEnum(val.into(), QueueType::from_value))
                }
                OpQstatsGetDumpReply::QueueId(val) => fmt.field("QueueId", &val),
                OpQstatsGetDumpReply::RxPackets(val) => fmt.field("RxPackets", &val),
                OpQstatsGetDumpReply::RxBytes(val) => fmt.field("RxBytes", &val),
                OpQstatsGetDumpReply::TxPackets(val) => fmt.field("TxPackets", &val),
                OpQstatsGetDumpReply::TxBytes(val) => fmt.field("TxBytes", &val),
                OpQstatsGetDumpReply::RxAllocFail(val) => fmt.field("RxAllocFail", &val),
                OpQstatsGetDumpReply::RxHwDrops(val) => fmt.field("RxHwDrops", &val),
                OpQstatsGetDumpReply::RxHwDropOverruns(val) => fmt.field("RxHwDropOverruns", &val),
                OpQstatsGetDumpReply::RxCsumComplete(val) => fmt.field("RxCsumComplete", &val),
                OpQstatsGetDumpReply::RxCsumUnnecessary(val) => {
                    fmt.field("RxCsumUnnecessary", &val)
                }
                OpQstatsGetDumpReply::RxCsumNone(val) => fmt.field("RxCsumNone", &val),
                OpQstatsGetDumpReply::RxCsumBad(val) => fmt.field("RxCsumBad", &val),
                OpQstatsGetDumpReply::RxHwGroPackets(val) => fmt.field("RxHwGroPackets", &val),
                OpQstatsGetDumpReply::RxHwGroBytes(val) => fmt.field("RxHwGroBytes", &val),
                OpQstatsGetDumpReply::RxHwGroWirePackets(val) => {
                    fmt.field("RxHwGroWirePackets", &val)
                }
                OpQstatsGetDumpReply::RxHwGroWireBytes(val) => fmt.field("RxHwGroWireBytes", &val),
                OpQstatsGetDumpReply::RxHwDropRatelimits(val) => {
                    fmt.field("RxHwDropRatelimits", &val)
                }
                OpQstatsGetDumpReply::TxHwDrops(val) => fmt.field("TxHwDrops", &val),
                OpQstatsGetDumpReply::TxHwDropErrors(val) => fmt.field("TxHwDropErrors", &val),
                OpQstatsGetDumpReply::TxCsumNone(val) => fmt.field("TxCsumNone", &val),
                OpQstatsGetDumpReply::TxNeedsCsum(val) => fmt.field("TxNeedsCsum", &val),
                OpQstatsGetDumpReply::TxHwGsoPackets(val) => fmt.field("TxHwGsoPackets", &val),
                OpQstatsGetDumpReply::TxHwGsoBytes(val) => fmt.field("TxHwGsoBytes", &val),
                OpQstatsGetDumpReply::TxHwGsoWirePackets(val) => {
                    fmt.field("TxHwGsoWirePackets", &val)
                }
                OpQstatsGetDumpReply::TxHwGsoWireBytes(val) => fmt.field("TxHwGsoWireBytes", &val),
                OpQstatsGetDumpReply::TxHwDropRatelimits(val) => {
                    fmt.field("TxHwDropRatelimits", &val)
                }
                OpQstatsGetDumpReply::TxStop(val) => fmt.field("TxStop", &val),
                OpQstatsGetDumpReply::TxWake(val) => fmt.field("TxWake", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpQstatsGetDumpReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpQstatsGetDumpReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpQstatsGetDumpReply::attr_from_type(t)),
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
                OpQstatsGetDumpReply::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::QueueType(val) => {
                    if last_off == offset {
                        stack.push(("QueueType", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::QueueId(val) => {
                    if last_off == offset {
                        stack.push(("QueueId", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::RxPackets(val) => {
                    if last_off == offset {
                        stack.push(("RxPackets", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::RxBytes(val) => {
                    if last_off == offset {
                        stack.push(("RxBytes", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::TxPackets(val) => {
                    if last_off == offset {
                        stack.push(("TxPackets", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::TxBytes(val) => {
                    if last_off == offset {
                        stack.push(("TxBytes", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::RxAllocFail(val) => {
                    if last_off == offset {
                        stack.push(("RxAllocFail", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::RxHwDrops(val) => {
                    if last_off == offset {
                        stack.push(("RxHwDrops", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::RxHwDropOverruns(val) => {
                    if last_off == offset {
                        stack.push(("RxHwDropOverruns", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::RxCsumComplete(val) => {
                    if last_off == offset {
                        stack.push(("RxCsumComplete", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::RxCsumUnnecessary(val) => {
                    if last_off == offset {
                        stack.push(("RxCsumUnnecessary", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::RxCsumNone(val) => {
                    if last_off == offset {
                        stack.push(("RxCsumNone", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::RxCsumBad(val) => {
                    if last_off == offset {
                        stack.push(("RxCsumBad", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::RxHwGroPackets(val) => {
                    if last_off == offset {
                        stack.push(("RxHwGroPackets", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::RxHwGroBytes(val) => {
                    if last_off == offset {
                        stack.push(("RxHwGroBytes", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::RxHwGroWirePackets(val) => {
                    if last_off == offset {
                        stack.push(("RxHwGroWirePackets", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::RxHwGroWireBytes(val) => {
                    if last_off == offset {
                        stack.push(("RxHwGroWireBytes", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::RxHwDropRatelimits(val) => {
                    if last_off == offset {
                        stack.push(("RxHwDropRatelimits", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::TxHwDrops(val) => {
                    if last_off == offset {
                        stack.push(("TxHwDrops", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::TxHwDropErrors(val) => {
                    if last_off == offset {
                        stack.push(("TxHwDropErrors", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::TxCsumNone(val) => {
                    if last_off == offset {
                        stack.push(("TxCsumNone", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::TxNeedsCsum(val) => {
                    if last_off == offset {
                        stack.push(("TxNeedsCsum", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::TxHwGsoPackets(val) => {
                    if last_off == offset {
                        stack.push(("TxHwGsoPackets", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::TxHwGsoBytes(val) => {
                    if last_off == offset {
                        stack.push(("TxHwGsoBytes", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::TxHwGsoWirePackets(val) => {
                    if last_off == offset {
                        stack.push(("TxHwGsoWirePackets", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::TxHwGsoWireBytes(val) => {
                    if last_off == offset {
                        stack.push(("TxHwGsoWireBytes", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::TxHwDropRatelimits(val) => {
                    if last_off == offset {
                        stack.push(("TxHwDropRatelimits", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::TxStop(val) => {
                    if last_off == offset {
                        stack.push(("TxStop", last_off));
                        break;
                    }
                }
                OpQstatsGetDumpReply::TxWake(val) => {
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
            stack.push(("OpQstatsGetDumpReply", cur));
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpQstatsGetDumpRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpQstatsGetDumpRequest<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        PushOpQstatsGetDumpRequest::write_header(&mut request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode(&mut self) -> PushOpQstatsGetDumpRequest<&mut Vec<u8>> {
        PushOpQstatsGetDumpRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpQstatsGetDumpRequest<RequestBuf<'r>> {
        PushOpQstatsGetDumpRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpQstatsGetDumpRequest<'_> {
    type ReplyType<'buf> = IterableOpQstatsGetDumpReply<'buf>;
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpQstatsGetDumpReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpQstatsGetDumpRequest::new(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "Bind dmabuf to netdev"]
pub struct PushOpBindRxDoRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpBindRxDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpBindRxDoRequest<Prev> {
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
        header.set_cmd(13u8);
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
    #[doc = "netdev ifindex to bind the dmabuf to."]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "receive queues to bind the dmabuf to.\nAttribute may repeat multiple times (treat it as array)"]
    pub fn nested_queues(mut self) -> PushQueueId<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 2u16);
        PushQueueId {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "dmabuf file descriptor to bind."]
    pub fn push_fd(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpBindRxDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Bind dmabuf to netdev"]
#[derive(Clone)]
pub enum OpBindRxDoRequest<'a> {
    #[doc = "netdev ifindex to bind the dmabuf to."]
    Ifindex(u32),
    #[doc = "receive queues to bind the dmabuf to.\nAttribute may repeat multiple times (treat it as array)"]
    Queues(IterableQueueId<'a>),
    #[doc = "dmabuf file descriptor to bind."]
    Fd(u32),
}
impl<'a> IterableOpBindRxDoRequest<'a> {
    #[doc = "netdev ifindex to bind the dmabuf to."]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpBindRxDoRequest::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpBindRxDoRequest",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "receive queues to bind the dmabuf to.\nAttribute may repeat multiple times (treat it as array)"]
    pub fn get_queues(
        &self,
    ) -> MultiAttrIterable<Self, OpBindRxDoRequest<'a>, IterableQueueId<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let OpBindRxDoRequest::Queues(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    #[doc = "dmabuf file descriptor to bind."]
    pub fn get_fd(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpBindRxDoRequest::Fd(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpBindRxDoRequest",
            "Fd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpBindRxDoRequest<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpBindRxDoRequest<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpBindRxDoRequest::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Dmabuf::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpBindRxDoRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpBindRxDoRequest<'a> {
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
impl<'a> Iterator for IterableOpBindRxDoRequest<'a> {
    type Item = Result<OpBindRxDoRequest<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpBindRxDoRequest::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpBindRxDoRequest::Queues({
                    let res = Some(IterableQueueId::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpBindRxDoRequest::Fd({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpBindRxDoRequest",
            r#type.and_then(|t| OpBindRxDoRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOpBindRxDoRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpBindRxDoRequest");
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
                OpBindRxDoRequest::Ifindex(val) => fmt.field("Ifindex", &val),
                OpBindRxDoRequest::Queues(val) => fmt.field("Queues", &val),
                OpBindRxDoRequest::Fd(val) => fmt.field("Fd", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpBindRxDoRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpBindRxDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpBindRxDoRequest::attr_from_type(t)),
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
                OpBindRxDoRequest::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OpBindRxDoRequest::Queues(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpBindRxDoRequest::Fd(val) => {
                    if last_off == offset {
                        stack.push(("Fd", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpBindRxDoRequest", cur));
        }
        (stack, missing)
    }
}
#[doc = "Bind dmabuf to netdev"]
pub struct PushOpBindRxDoReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpBindRxDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpBindRxDoReply<Prev> {
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
        header.set_cmd(13u8);
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
    #[doc = "id of the dmabuf binding"]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpBindRxDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Bind dmabuf to netdev"]
#[derive(Clone)]
pub enum OpBindRxDoReply {
    #[doc = "id of the dmabuf binding"]
    Id(u32),
}
impl<'a> IterableOpBindRxDoReply<'a> {
    #[doc = "id of the dmabuf binding"]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpBindRxDoReply::Id(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpBindRxDoReply",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpBindRxDoReply {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpBindRxDoReply<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpBindRxDoReply::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Dmabuf::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpBindRxDoReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpBindRxDoReply<'a> {
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
impl<'a> Iterator for IterableOpBindRxDoReply<'a> {
    type Item = Result<OpBindRxDoReply, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                4u16 => OpBindRxDoReply::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpBindRxDoReply",
            r#type.and_then(|t| OpBindRxDoReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpBindRxDoReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpBindRxDoReply");
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
                OpBindRxDoReply::Id(val) => fmt.field("Id", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpBindRxDoReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpBindRxDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpBindRxDoReply::attr_from_type(t)),
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
                OpBindRxDoReply::Id(val) => {
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
            stack.push(("OpBindRxDoReply", cur));
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpBindRxDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpBindRxDoRequest<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        PushOpBindRxDoRequest::write_header(&mut request.buf_mut());
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpBindRxDoRequest<&mut Vec<u8>> {
        PushOpBindRxDoRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpBindRxDoRequest<RequestBuf<'r>> {
        PushOpBindRxDoRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpBindRxDoRequest<'_> {
    type ReplyType<'buf> = IterableOpBindRxDoReply<'buf>;
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpBindRxDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpBindRxDoRequest::new(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "Set configurable NAPI instance settings."]
pub struct PushOpNapiSetDoRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpNapiSetDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpNapiSetDoRequest<Prev> {
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
        header.set_cmd(14u8);
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
    #[doc = "ID of the NAPI instance."]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The number of consecutive empty polls before IRQ deferral ends and hardware IRQs are re-enabled."]
    pub fn push_defer_hard_irqs(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 5u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The timeout, in nanoseconds, of when to trigger the NAPI watchdog timer which schedules NAPI processing. Additionally, a non-zero value will also prevent GRO from flushing recent super-frames at the end of a NAPI cycle. This may add receive latency in exchange for reducing the number of frames processed by the network stack."]
    pub fn push_gro_flush_timeout(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 6u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The timeout, in nanoseconds, of how long to suspend irq processing, if event polling finds events"]
    pub fn push_irq_suspend_timeout(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 7u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Whether the NAPI is configured to operate in threaded polling mode. If this is set to enabled then the NAPI context operates in threaded polling mode. If this is set to busy-poll, then the threaded polling mode also busy polls.\nAssociated type: \"NapiThreaded\" (enum)"]
    pub fn push_threaded(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpNapiSetDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Set configurable NAPI instance settings."]
#[derive(Clone)]
pub enum OpNapiSetDoRequest {
    #[doc = "ID of the NAPI instance."]
    Id(u32),
    #[doc = "The number of consecutive empty polls before IRQ deferral ends and hardware IRQs are re-enabled."]
    DeferHardIrqs(u32),
    #[doc = "The timeout, in nanoseconds, of when to trigger the NAPI watchdog timer which schedules NAPI processing. Additionally, a non-zero value will also prevent GRO from flushing recent super-frames at the end of a NAPI cycle. This may add receive latency in exchange for reducing the number of frames processed by the network stack."]
    GroFlushTimeout(u32),
    #[doc = "The timeout, in nanoseconds, of how long to suspend irq processing, if event polling finds events"]
    IrqSuspendTimeout(u32),
    #[doc = "Whether the NAPI is configured to operate in threaded polling mode. If this is set to enabled then the NAPI context operates in threaded polling mode. If this is set to busy-poll, then the threaded polling mode also busy polls.\nAssociated type: \"NapiThreaded\" (enum)"]
    Threaded(u32),
}
impl<'a> IterableOpNapiSetDoRequest<'a> {
    #[doc = "ID of the NAPI instance."]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNapiSetDoRequest::Id(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNapiSetDoRequest",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The number of consecutive empty polls before IRQ deferral ends and hardware IRQs are re-enabled."]
    pub fn get_defer_hard_irqs(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNapiSetDoRequest::DeferHardIrqs(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNapiSetDoRequest",
            "DeferHardIrqs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The timeout, in nanoseconds, of when to trigger the NAPI watchdog timer which schedules NAPI processing. Additionally, a non-zero value will also prevent GRO from flushing recent super-frames at the end of a NAPI cycle. This may add receive latency in exchange for reducing the number of frames processed by the network stack."]
    pub fn get_gro_flush_timeout(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNapiSetDoRequest::GroFlushTimeout(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNapiSetDoRequest",
            "GroFlushTimeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The timeout, in nanoseconds, of how long to suspend irq processing, if event polling finds events"]
    pub fn get_irq_suspend_timeout(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNapiSetDoRequest::IrqSuspendTimeout(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNapiSetDoRequest",
            "IrqSuspendTimeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Whether the NAPI is configured to operate in threaded polling mode. If this is set to enabled then the NAPI context operates in threaded polling mode. If this is set to busy-poll, then the threaded polling mode also busy polls.\nAssociated type: \"NapiThreaded\" (enum)"]
    pub fn get_threaded(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNapiSetDoRequest::Threaded(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpNapiSetDoRequest",
            "Threaded",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpNapiSetDoRequest {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpNapiSetDoRequest<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpNapiSetDoRequest::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Napi::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpNapiSetDoRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpNapiSetDoRequest<'a> {
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
impl<'a> Iterator for IterableOpNapiSetDoRequest<'a> {
    type Item = Result<OpNapiSetDoRequest, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                2u16 => OpNapiSetDoRequest::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpNapiSetDoRequest::DeferHardIrqs({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpNapiSetDoRequest::GroFlushTimeout({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => OpNapiSetDoRequest::IrqSuspendTimeout({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpNapiSetDoRequest::Threaded({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpNapiSetDoRequest",
            r#type.and_then(|t| OpNapiSetDoRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpNapiSetDoRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpNapiSetDoRequest");
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
                OpNapiSetDoRequest::Id(val) => fmt.field("Id", &val),
                OpNapiSetDoRequest::DeferHardIrqs(val) => fmt.field("DeferHardIrqs", &val),
                OpNapiSetDoRequest::GroFlushTimeout(val) => fmt.field("GroFlushTimeout", &val),
                OpNapiSetDoRequest::IrqSuspendTimeout(val) => fmt.field("IrqSuspendTimeout", &val),
                OpNapiSetDoRequest::Threaded(val) => fmt.field(
                    "Threaded",
                    &FormatEnum(val.into(), NapiThreaded::from_value),
                ),
            };
        }
        fmt.finish()
    }
}
impl IterableOpNapiSetDoRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpNapiSetDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpNapiSetDoRequest::attr_from_type(t)),
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
                OpNapiSetDoRequest::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                OpNapiSetDoRequest::DeferHardIrqs(val) => {
                    if last_off == offset {
                        stack.push(("DeferHardIrqs", last_off));
                        break;
                    }
                }
                OpNapiSetDoRequest::GroFlushTimeout(val) => {
                    if last_off == offset {
                        stack.push(("GroFlushTimeout", last_off));
                        break;
                    }
                }
                OpNapiSetDoRequest::IrqSuspendTimeout(val) => {
                    if last_off == offset {
                        stack.push(("IrqSuspendTimeout", last_off));
                        break;
                    }
                }
                OpNapiSetDoRequest::Threaded(val) => {
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
            stack.push(("OpNapiSetDoRequest", cur));
        }
        (stack, None)
    }
}
#[doc = "Set configurable NAPI instance settings."]
pub struct PushOpNapiSetDoReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpNapiSetDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpNapiSetDoReply<Prev> {
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
        header.set_cmd(14u8);
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
impl<Prev: Rec> Drop for PushOpNapiSetDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Set configurable NAPI instance settings."]
#[derive(Clone)]
pub enum OpNapiSetDoReply {}
impl<'a> IterableOpNapiSetDoReply<'a> {}
impl OpNapiSetDoReply {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpNapiSetDoReply<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpNapiSetDoReply::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Napi::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpNapiSetDoReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpNapiSetDoReply<'a> {
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
impl<'a> Iterator for IterableOpNapiSetDoReply<'a> {
    type Item = Result<OpNapiSetDoReply, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpNapiSetDoReply",
            r#type.and_then(|t| OpNapiSetDoReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpNapiSetDoReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpNapiSetDoReply");
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
impl IterableOpNapiSetDoReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpNapiSetDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpNapiSetDoReply::attr_from_type(t)),
            );
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpNapiSetDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpNapiSetDoRequest<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        PushOpNapiSetDoRequest::write_header(&mut request.buf_mut());
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpNapiSetDoRequest<&mut Vec<u8>> {
        PushOpNapiSetDoRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpNapiSetDoRequest<RequestBuf<'r>> {
        PushOpNapiSetDoRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpNapiSetDoRequest<'_> {
    type ReplyType<'buf> = IterableOpNapiSetDoReply<'buf>;
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpNapiSetDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpNapiSetDoRequest::new(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "Bind dmabuf to netdev for TX"]
pub struct PushOpBindTxDoRequest<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpBindTxDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpBindTxDoRequest<Prev> {
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
        header.set_cmd(15u8);
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
    #[doc = "netdev ifindex to bind the dmabuf to."]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "dmabuf file descriptor to bind."]
    pub fn push_fd(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpBindTxDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Bind dmabuf to netdev for TX"]
#[derive(Clone)]
pub enum OpBindTxDoRequest {
    #[doc = "netdev ifindex to bind the dmabuf to."]
    Ifindex(u32),
    #[doc = "dmabuf file descriptor to bind."]
    Fd(u32),
}
impl<'a> IterableOpBindTxDoRequest<'a> {
    #[doc = "netdev ifindex to bind the dmabuf to."]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpBindTxDoRequest::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpBindTxDoRequest",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "dmabuf file descriptor to bind."]
    pub fn get_fd(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpBindTxDoRequest::Fd(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpBindTxDoRequest",
            "Fd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpBindTxDoRequest {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpBindTxDoRequest<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpBindTxDoRequest::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Dmabuf::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpBindTxDoRequest<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpBindTxDoRequest<'a> {
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
impl<'a> Iterator for IterableOpBindTxDoRequest<'a> {
    type Item = Result<OpBindTxDoRequest, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpBindTxDoRequest::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpBindTxDoRequest::Fd({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpBindTxDoRequest",
            r#type.and_then(|t| OpBindTxDoRequest::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpBindTxDoRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpBindTxDoRequest");
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
                OpBindTxDoRequest::Ifindex(val) => fmt.field("Ifindex", &val),
                OpBindTxDoRequest::Fd(val) => fmt.field("Fd", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpBindTxDoRequest<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpBindTxDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpBindTxDoRequest::attr_from_type(t)),
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
                OpBindTxDoRequest::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OpBindTxDoRequest::Fd(val) => {
                    if last_off == offset {
                        stack.push(("Fd", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpBindTxDoRequest", cur));
        }
        (stack, None)
    }
}
#[doc = "Bind dmabuf to netdev for TX"]
pub struct PushOpBindTxDoReply<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpBindTxDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpBindTxDoReply<Prev> {
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
        header.set_cmd(15u8);
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
    #[doc = "id of the dmabuf binding"]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpBindTxDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Bind dmabuf to netdev for TX"]
#[derive(Clone)]
pub enum OpBindTxDoReply {
    #[doc = "id of the dmabuf binding"]
    Id(u32),
}
impl<'a> IterableOpBindTxDoReply<'a> {
    #[doc = "id of the dmabuf binding"]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpBindTxDoReply::Id(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OpBindTxDoReply",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OpBindTxDoReply {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOpBindTxDoReply<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(PushBuiltinNfgenmsg::len()));
        IterableOpBindTxDoReply::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Dmabuf::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOpBindTxDoReply<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOpBindTxDoReply<'a> {
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
impl<'a> Iterator for IterableOpBindTxDoReply<'a> {
    type Item = Result<OpBindTxDoReply, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                4u16 => OpBindTxDoReply::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(any(test, feature = "deny-unknown-attrs")) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OpBindTxDoReply",
            r#type.and_then(|t| OpBindTxDoReply::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableOpBindTxDoReply<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpBindTxDoReply");
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
                OpBindTxDoReply::Id(val) => fmt.field("Id", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOpBindTxDoReply<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpBindTxDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpBindTxDoReply::attr_from_type(t)),
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
                OpBindTxDoReply::Id(val) => {
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
            stack.push(("OpBindTxDoReply", cur));
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpBindTxDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpBindTxDoRequest<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        PushOpBindTxDoRequest::write_header(&mut request.buf_mut());
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpBindTxDoRequest<&mut Vec<u8>> {
        PushOpBindTxDoRequest::new_without_header(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOpBindTxDoRequest<RequestBuf<'r>> {
        PushOpBindTxDoRequest::new_without_header(self.request.buf)
    }
}
impl NetlinkRequest for RequestOpBindTxDoRequest<'_> {
    type ReplyType<'buf> = IterableOpBindTxDoReply<'buf>;
    fn protocol(&self) -> Protocol {
        Protocol::Generic("netdev".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpBindTxDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpBindTxDoRequest::new(buf).lookup_attr(offset, missing_type)
    }
}
use crate::traits::LookupFn;
use crate::utils::RequestBuf;
#[derive(Debug)]
pub struct Request<'buf> {
    buf: RequestBuf<'buf>,
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
    pub fn new_from_buf(buf: Vec<u8>) -> Self {
        Self {
            flags: 0,
            buf: RequestBuf::Own(buf),
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
        Self {
            flags: 0,
            buf: RequestBuf::Ref(buf),
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
    #[doc = "Set `NLM_F_DUMP` flag"]
    fn set_dump(mut self) -> Self {
        self.flags |= consts::NLM_F_DUMP as u16;
        self
    }
    pub fn op_dev_get_dump_request(self) -> RequestOpDevGetDumpRequest<'buf> {
        let mut res = RequestOpDevGetDumpRequest::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-dev-get-dump-request",
            RequestOpDevGetDumpRequest::lookup,
        );
        res
    }
    pub fn op_dev_get_do_request(self) -> RequestOpDevGetDoRequest<'buf> {
        let mut res = RequestOpDevGetDoRequest::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-dev-get-do-request",
            RequestOpDevGetDoRequest::lookup,
        );
        res
    }
    pub fn op_page_pool_get_dump_request(self) -> RequestOpPagePoolGetDumpRequest<'buf> {
        let mut res = RequestOpPagePoolGetDumpRequest::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-page-pool-get-dump-request",
            RequestOpPagePoolGetDumpRequest::lookup,
        );
        res
    }
    pub fn op_page_pool_get_do_request(self) -> RequestOpPagePoolGetDoRequest<'buf> {
        let mut res = RequestOpPagePoolGetDoRequest::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-page-pool-get-do-request",
            RequestOpPagePoolGetDoRequest::lookup,
        );
        res
    }
    pub fn op_page_pool_stats_get_dump_request(self) -> RequestOpPagePoolStatsGetDumpRequest<'buf> {
        let mut res = RequestOpPagePoolStatsGetDumpRequest::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-page-pool-stats-get-dump-request",
            RequestOpPagePoolStatsGetDumpRequest::lookup,
        );
        res
    }
    pub fn op_page_pool_stats_get_do_request(self) -> RequestOpPagePoolStatsGetDoRequest<'buf> {
        let mut res = RequestOpPagePoolStatsGetDoRequest::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-page-pool-stats-get-do-request",
            RequestOpPagePoolStatsGetDoRequest::lookup,
        );
        res
    }
    pub fn op_queue_get_dump_request(self) -> RequestOpQueueGetDumpRequest<'buf> {
        let mut res = RequestOpQueueGetDumpRequest::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-queue-get-dump-request",
            RequestOpQueueGetDumpRequest::lookup,
        );
        res
    }
    pub fn op_queue_get_do_request(self) -> RequestOpQueueGetDoRequest<'buf> {
        let mut res = RequestOpQueueGetDoRequest::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-queue-get-do-request",
            RequestOpQueueGetDoRequest::lookup,
        );
        res
    }
    pub fn op_napi_get_dump_request(self) -> RequestOpNapiGetDumpRequest<'buf> {
        let mut res = RequestOpNapiGetDumpRequest::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-napi-get-dump-request",
            RequestOpNapiGetDumpRequest::lookup,
        );
        res
    }
    pub fn op_napi_get_do_request(self) -> RequestOpNapiGetDoRequest<'buf> {
        let mut res = RequestOpNapiGetDoRequest::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-napi-get-do-request",
            RequestOpNapiGetDoRequest::lookup,
        );
        res
    }
    pub fn op_qstats_get_dump_request(self) -> RequestOpQstatsGetDumpRequest<'buf> {
        let mut res = RequestOpQstatsGetDumpRequest::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-qstats-get-dump-request",
            RequestOpQstatsGetDumpRequest::lookup,
        );
        res
    }
    pub fn op_bind_rx_do_request(self) -> RequestOpBindRxDoRequest<'buf> {
        let mut res = RequestOpBindRxDoRequest::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-bind-rx-do-request",
            RequestOpBindRxDoRequest::lookup,
        );
        res
    }
    pub fn op_napi_set_do_request(self) -> RequestOpNapiSetDoRequest<'buf> {
        let mut res = RequestOpNapiSetDoRequest::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-napi-set-do-request",
            RequestOpNapiSetDoRequest::lookup,
        );
        res
    }
    pub fn op_bind_tx_do_request(self) -> RequestOpBindTxDoRequest<'buf> {
        let mut res = RequestOpBindTxDoRequest::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-bind-tx-do-request",
            RequestOpBindTxDoRequest::lookup,
        );
        res
    }
}
