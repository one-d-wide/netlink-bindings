#![doc = "**Netlink protocol to control WireGuard network devices.**\n\nThe below enums and macros are for interfacing with WireGuard, using\ngeneric netlink, with family `WG_GENL_NAME` and version\n`WG_GENL_VERSION`. It defines two commands: get and set. Note that while\nthey share many common attributes, these two commands actually accept a\nslightly different set of inputs and outputs. These differences are\nnoted under the individual attributes.\n"]
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
use crate::builtin::{BuiltinBitfield32, BuiltinNfgenmsg, Nlmsghdr, PushDummy};
use crate::{
    consts,
    traits::{NetlinkRequest, Protocol},
    utils::*,
};
pub const PROTONAME: &str = "wireguard";
pub const PROTONAME_CSTR: &CStr = c"wireguard";
pub const KEY_LEN: u64 = 32u64;
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum WgdeviceFlags {
    ReplacePeers = 1 << 0,
}
impl WgdeviceFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::ReplacePeers,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum WgpeerFlags {
    RemoveMe = 1 << 0,
    ReplaceAllowedips = 1 << 1,
    UpdateOnly = 1 << 2,
}
impl WgpeerFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::RemoveMe,
            n if n == 1 << 1 => Self::ReplaceAllowedips,
            n if n == 1 << 2 => Self::UpdateOnly,
            _ => return None,
        })
    }
}
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum WgallowedipFlags {
    RemoveMe = 1 << 0,
}
impl WgallowedipFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::RemoveMe,
            _ => return None,
        })
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(packed(4))]
#[repr(C)]
pub struct KernelTimespec {
    #[doc = "Number of seconds, since UNIX epoch.\n"]
    pub sec: u64,
    #[doc = "Number of nanoseconds, after the second began.\n"]
    pub nsec: u64,
}
#[doc = "Create zero-initialized struct"]
impl Default for KernelTimespec {
    fn default() -> Self {
        Self::new()
    }
}
impl KernelTimespec {
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
    pub fn new_from_array(buf: [u8; 16usize]) -> Self {
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
    pub fn as_array(&self) -> &[u8; 16usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 16usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 16usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<KernelTimespec>() == 16usize);
        const _: () = assert!(std::mem::align_of::<KernelTimespec>() == 4usize);
        16usize
    }
}
impl std::fmt::Debug for KernelTimespec {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("KernelTimespec")
            .field("sec", &{ self.sec })
            .field("nsec", &{ self.nsec })
            .finish()
    }
}
#[derive(Clone)]
pub enum Wgdevice<'a> {
    Ifindex(u32),
    Ifname(&'a CStr),
    #[doc = "Set to all zeros to remove.\n"]
    PrivateKey(&'a [u8]),
    PublicKey(&'a [u8]),
    #[doc = "`0` or `WGDEVICE_F_REPLACE_PEERS` if all current peers should be removed\nprior to adding the list below.\n\nAssociated type: [`WgdeviceFlags`] (enum)"]
    Flags(u32),
    #[doc = "Set as `0` to choose randomly.\n"]
    ListenPort(u16),
    #[doc = "Set as `0` to disable.\n"]
    Fwmark(u32),
    #[doc = "The index/type parameter is unused on `SET_DEVICE` operations and is\nzero on `GET_DEVICE` operations.\n"]
    Peers(IterableArrayWgpeer<'a>),
}
impl<'a> IterableWgdevice<'a> {
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Wgdevice::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Wgdevice",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ifname(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Wgdevice::Ifname(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Wgdevice",
            "Ifname",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Set to all zeros to remove.\n"]
    pub fn get_private_key(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Wgdevice::PrivateKey(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Wgdevice",
            "PrivateKey",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_public_key(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Wgdevice::PublicKey(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Wgdevice",
            "PublicKey",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "`0` or `WGDEVICE_F_REPLACE_PEERS` if all current peers should be removed\nprior to adding the list below.\n\nAssociated type: [`WgdeviceFlags`] (enum)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Wgdevice::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Wgdevice",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Set as `0` to choose randomly.\n"]
    pub fn get_listen_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Wgdevice::ListenPort(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Wgdevice",
            "ListenPort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Set as `0` to disable.\n"]
    pub fn get_fwmark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Wgdevice::Fwmark(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Wgdevice",
            "Fwmark",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The index/type parameter is unused on `SET_DEVICE` operations and is\nzero on `GET_DEVICE` operations.\n"]
    pub fn get_peers(
        &self,
    ) -> Result<ArrayIterable<IterableArrayWgpeer<'a>, IterableWgpeer<'a>>, ErrorContext> {
        for attr in self.clone() {
            if let Ok(Wgdevice::Peers(val)) = attr {
                return Ok(ArrayIterable::new(val));
            }
        }
        Err(ErrorContext::new_missing(
            "Wgdevice",
            "Peers",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl<'a> Wgpeer<'a> {
    pub fn new_array(buf: &[u8]) -> IterableArrayWgpeer<'_> {
        IterableArrayWgpeer::with_loc(buf, buf.as_ptr() as usize)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableArrayWgpeer<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableArrayWgpeer<'a> {
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
impl<'a> Iterator for IterableArrayWgpeer<'a> {
    type Item = Result<IterableWgpeer<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            {
                return Some(Ok(IterableWgpeer::with_loc(next, self.orig_loc)));
            }
        }
        let pos = self.pos;
        self.pos = self.buf.len();
        Some(Err(ErrorContext::new(
            "Wgpeer",
            None,
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl Wgdevice<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableWgdevice<'a> {
        IterableWgdevice::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "Unspec",
            1u16 => "Ifindex",
            2u16 => "Ifname",
            3u16 => "PrivateKey",
            4u16 => "PublicKey",
            5u16 => "Flags",
            6u16 => "ListenPort",
            7u16 => "Fwmark",
            8u16 => "Peers",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableWgdevice<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableWgdevice<'a> {
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
impl<'a> Iterator for IterableWgdevice<'a> {
    type Item = Result<Wgdevice<'a>, ErrorContext>;
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
                1u16 => Wgdevice::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Wgdevice::Ifname({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Wgdevice::PrivateKey({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Wgdevice::PublicKey({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Wgdevice::Flags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Wgdevice::ListenPort({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => Wgdevice::Fwmark({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => Wgdevice::Peers({
                    let res = Some(IterableArrayWgpeer::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Wgdevice",
            r#type.and_then(|t| Wgdevice::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableArrayWgpeer<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_list()
            .entries(self.clone().map(FlattenErrorContext))
            .finish()
    }
}
impl<'a> std::fmt::Debug for IterableWgdevice<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Wgdevice");
        let mut iter = IterableWgdevice::with_loc(&[], self.orig_loc);
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
                Wgdevice::Ifindex(val) => fmt.field("Ifindex", &val),
                Wgdevice::Ifname(val) => fmt.field("Ifname", &val),
                Wgdevice::PrivateKey(val) => fmt.field("PrivateKey", &FormatHex(val)),
                Wgdevice::PublicKey(val) => fmt.field("PublicKey", &FormatHex(val)),
                Wgdevice::Flags(val) => {
                    fmt.field("Flags", &FormatFlags(val.into(), WgdeviceFlags::from_value))
                }
                Wgdevice::ListenPort(val) => fmt.field("ListenPort", &val),
                Wgdevice::Fwmark(val) => fmt.field("Fwmark", &val),
                Wgdevice::Peers(val) => fmt.field("Peers", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableWgdevice<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Wgdevice", offset));
            return (
                stack,
                missing_type.and_then(|t| Wgdevice::attr_from_type(t)),
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
                Wgdevice::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                Wgdevice::Ifname(val) => {
                    if last_off == offset {
                        stack.push(("Ifname", last_off));
                        break;
                    }
                }
                Wgdevice::PrivateKey(val) => {
                    if last_off == offset {
                        stack.push(("PrivateKey", last_off));
                        break;
                    }
                }
                Wgdevice::PublicKey(val) => {
                    if last_off == offset {
                        stack.push(("PublicKey", last_off));
                        break;
                    }
                }
                Wgdevice::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                Wgdevice::ListenPort(val) => {
                    if last_off == offset {
                        stack.push(("ListenPort", last_off));
                        break;
                    }
                }
                Wgdevice::Fwmark(val) => {
                    if last_off == offset {
                        stack.push(("Fwmark", last_off));
                        break;
                    }
                }
                Wgdevice::Peers(val) => {
                    for entry in val {
                        let Ok(attr) = entry else { break };
                        (stack, missing) = attr.lookup_attr(offset, missing_type);
                        if !stack.is_empty() {
                            break;
                        }
                    }
                    if !stack.is_empty() {
                        stack.push(("Peers", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Wgdevice", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum Wgpeer<'a> {
    PublicKey(&'a [u8]),
    #[doc = "Set as all zeros to remove.\n"]
    PresharedKey(&'a [u8]),
    #[doc = "`0` and/or `WGPEER_F_REMOVE_ME` if the specified peer should not exist\nat the end of the operation, rather than added/updated and/or\n`WGPEER_F_REPLACE_ALLOWEDIPS` if all current allowed IPs of this peer\nshould be removed prior to adding the list below and/or\n`WGPEER_F_UPDATE_ONLY` if the peer should only be set if it already\nexists.\n\nAssociated type: [`WgpeerFlags`] (enum)"]
    Flags(u32),
    #[doc = "struct sockaddr_in or struct sockaddr_in6\n"]
    Endpoint(std::net::SocketAddr),
    #[doc = "Set as `0` to disable.\n"]
    PersistentKeepaliveInterval(u16),
    LastHandshakeTime(KernelTimespec),
    RxBytes(u64),
    TxBytes(u64),
    #[doc = "The index/type parameter is unused on `SET_DEVICE` operations and is\nzero on `GET_DEVICE` operations.\n"]
    Allowedips(IterableArrayWgallowedip<'a>),
    #[doc = "Should not be set or used at all by most users of this API, as the most\nrecent protocol will be used when this is unset. Otherwise, must be set\nto `1`.\n"]
    ProtocolVersion(u32),
}
impl<'a> IterableWgpeer<'a> {
    pub fn get_public_key(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Wgpeer::PublicKey(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Wgpeer",
            "PublicKey",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Set as all zeros to remove.\n"]
    pub fn get_preshared_key(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Wgpeer::PresharedKey(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Wgpeer",
            "PresharedKey",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "`0` and/or `WGPEER_F_REMOVE_ME` if the specified peer should not exist\nat the end of the operation, rather than added/updated and/or\n`WGPEER_F_REPLACE_ALLOWEDIPS` if all current allowed IPs of this peer\nshould be removed prior to adding the list below and/or\n`WGPEER_F_UPDATE_ONLY` if the peer should only be set if it already\nexists.\n\nAssociated type: [`WgpeerFlags`] (enum)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Wgpeer::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Wgpeer",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "struct sockaddr_in or struct sockaddr_in6\n"]
    pub fn get_endpoint(&self) -> Result<std::net::SocketAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Wgpeer::Endpoint(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Wgpeer",
            "Endpoint",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Set as `0` to disable.\n"]
    pub fn get_persistent_keepalive_interval(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Wgpeer::PersistentKeepaliveInterval(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Wgpeer",
            "PersistentKeepaliveInterval",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_last_handshake_time(&self) -> Result<KernelTimespec, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Wgpeer::LastHandshakeTime(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Wgpeer",
            "LastHandshakeTime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rx_bytes(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Wgpeer::RxBytes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Wgpeer",
            "RxBytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tx_bytes(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Wgpeer::TxBytes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Wgpeer",
            "TxBytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The index/type parameter is unused on `SET_DEVICE` operations and is\nzero on `GET_DEVICE` operations.\n"]
    pub fn get_allowedips(
        &self,
    ) -> Result<ArrayIterable<IterableArrayWgallowedip<'a>, IterableWgallowedip<'a>>, ErrorContext>
    {
        for attr in self.clone() {
            if let Ok(Wgpeer::Allowedips(val)) = attr {
                return Ok(ArrayIterable::new(val));
            }
        }
        Err(ErrorContext::new_missing(
            "Wgpeer",
            "Allowedips",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Should not be set or used at all by most users of this API, as the most\nrecent protocol will be used when this is unset. Otherwise, must be set\nto `1`.\n"]
    pub fn get_protocol_version(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Wgpeer::ProtocolVersion(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Wgpeer",
            "ProtocolVersion",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Wgallowedip {
    pub fn new_array(buf: &[u8]) -> IterableArrayWgallowedip<'_> {
        IterableArrayWgallowedip::with_loc(buf, buf.as_ptr() as usize)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableArrayWgallowedip<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableArrayWgallowedip<'a> {
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
impl<'a> Iterator for IterableArrayWgallowedip<'a> {
    type Item = Result<IterableWgallowedip<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            {
                return Some(Ok(IterableWgallowedip::with_loc(next, self.orig_loc)));
            }
        }
        let pos = self.pos;
        self.pos = self.buf.len();
        Some(Err(ErrorContext::new(
            "Wgallowedip",
            None,
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl Wgpeer<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableWgpeer<'a> {
        IterableWgpeer::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "Unspec",
            1u16 => "PublicKey",
            2u16 => "PresharedKey",
            3u16 => "Flags",
            4u16 => "Endpoint",
            5u16 => "PersistentKeepaliveInterval",
            6u16 => "LastHandshakeTime",
            7u16 => "RxBytes",
            8u16 => "TxBytes",
            9u16 => "Allowedips",
            10u16 => "ProtocolVersion",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableWgpeer<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableWgpeer<'a> {
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
impl<'a> Iterator for IterableWgpeer<'a> {
    type Item = Result<Wgpeer<'a>, ErrorContext>;
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
                1u16 => Wgpeer::PublicKey({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Wgpeer::PresharedKey({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Wgpeer::Flags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Wgpeer::Endpoint({
                    let res = parse_sockaddr(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Wgpeer::PersistentKeepaliveInterval({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Wgpeer::LastHandshakeTime({
                    let res = Some(KernelTimespec::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => Wgpeer::RxBytes({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => Wgpeer::TxBytes({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => Wgpeer::Allowedips({
                    let res = Some(IterableArrayWgallowedip::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => Wgpeer::ProtocolVersion({
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
            "Wgpeer",
            r#type.and_then(|t| Wgpeer::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableArrayWgallowedip<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_list()
            .entries(self.clone().map(FlattenErrorContext))
            .finish()
    }
}
impl<'a> std::fmt::Debug for IterableWgpeer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Wgpeer");
        let mut iter = IterableWgpeer::with_loc(&[], self.orig_loc);
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
                Wgpeer::PublicKey(val) => fmt.field("PublicKey", &FormatHex(val)),
                Wgpeer::PresharedKey(val) => fmt.field("PresharedKey", &FormatHex(val)),
                Wgpeer::Flags(val) => {
                    fmt.field("Flags", &FormatFlags(val.into(), WgpeerFlags::from_value))
                }
                Wgpeer::Endpoint(val) => fmt.field("Endpoint", &val),
                Wgpeer::PersistentKeepaliveInterval(val) => {
                    fmt.field("PersistentKeepaliveInterval", &val)
                }
                Wgpeer::LastHandshakeTime(val) => fmt.field("LastHandshakeTime", &val),
                Wgpeer::RxBytes(val) => fmt.field("RxBytes", &val),
                Wgpeer::TxBytes(val) => fmt.field("TxBytes", &val),
                Wgpeer::Allowedips(val) => fmt.field("Allowedips", &val),
                Wgpeer::ProtocolVersion(val) => fmt.field("ProtocolVersion", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableWgpeer<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Wgpeer", offset));
            return (stack, missing_type.and_then(|t| Wgpeer::attr_from_type(t)));
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
                Wgpeer::PublicKey(val) => {
                    if last_off == offset {
                        stack.push(("PublicKey", last_off));
                        break;
                    }
                }
                Wgpeer::PresharedKey(val) => {
                    if last_off == offset {
                        stack.push(("PresharedKey", last_off));
                        break;
                    }
                }
                Wgpeer::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                Wgpeer::Endpoint(val) => {
                    if last_off == offset {
                        stack.push(("Endpoint", last_off));
                        break;
                    }
                }
                Wgpeer::PersistentKeepaliveInterval(val) => {
                    if last_off == offset {
                        stack.push(("PersistentKeepaliveInterval", last_off));
                        break;
                    }
                }
                Wgpeer::LastHandshakeTime(val) => {
                    if last_off == offset {
                        stack.push(("LastHandshakeTime", last_off));
                        break;
                    }
                }
                Wgpeer::RxBytes(val) => {
                    if last_off == offset {
                        stack.push(("RxBytes", last_off));
                        break;
                    }
                }
                Wgpeer::TxBytes(val) => {
                    if last_off == offset {
                        stack.push(("TxBytes", last_off));
                        break;
                    }
                }
                Wgpeer::Allowedips(val) => {
                    for entry in val {
                        let Ok(attr) = entry else { break };
                        (stack, missing) = attr.lookup_attr(offset, missing_type);
                        if !stack.is_empty() {
                            break;
                        }
                    }
                    if !stack.is_empty() {
                        stack.push(("Allowedips", last_off));
                        break;
                    }
                }
                Wgpeer::ProtocolVersion(val) => {
                    if last_off == offset {
                        stack.push(("ProtocolVersion", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Wgpeer", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum Wgallowedip {
    #[doc = "IP family, either `AF_INET` or `AF_INET6`.\n"]
    Family(u16),
    #[doc = "Either `struct in_addr` or `struct in6_addr`.\n"]
    Ipaddr(std::net::IpAddr),
    CidrMask(u8),
    #[doc = "`WGALLOWEDIP_F_REMOVE_ME` if the specified IP should be removed;\notherwise, this IP will be added if it is not already present.\n\nAssociated type: [`WgallowedipFlags`] (enum)"]
    Flags(u32),
}
impl<'a> IterableWgallowedip<'a> {
    #[doc = "IP family, either `AF_INET` or `AF_INET6`.\n"]
    pub fn get_family(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Wgallowedip::Family(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Wgallowedip",
            "Family",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Either `struct in_addr` or `struct in6_addr`.\n"]
    pub fn get_ipaddr(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Wgallowedip::Ipaddr(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Wgallowedip",
            "Ipaddr",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_cidr_mask(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Wgallowedip::CidrMask(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Wgallowedip",
            "CidrMask",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "`WGALLOWEDIP_F_REMOVE_ME` if the specified IP should be removed;\notherwise, this IP will be added if it is not already present.\n\nAssociated type: [`WgallowedipFlags`] (enum)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Wgallowedip::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Wgallowedip",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Wgallowedip {
    pub fn new<'a>(buf: &'a [u8]) -> IterableWgallowedip<'a> {
        IterableWgallowedip::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "Unspec",
            1u16 => "Family",
            2u16 => "Ipaddr",
            3u16 => "CidrMask",
            4u16 => "Flags",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableWgallowedip<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableWgallowedip<'a> {
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
impl<'a> Iterator for IterableWgallowedip<'a> {
    type Item = Result<Wgallowedip, ErrorContext>;
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
                1u16 => Wgallowedip::Family({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Wgallowedip::Ipaddr({
                    let res = parse_ip(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Wgallowedip::CidrMask({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Wgallowedip::Flags({
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
            "Wgallowedip",
            r#type.and_then(|t| Wgallowedip::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableWgallowedip<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Wgallowedip");
        let mut iter = IterableWgallowedip::with_loc(&[], self.orig_loc);
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
                Wgallowedip::Family(val) => fmt.field("Family", &val),
                Wgallowedip::Ipaddr(val) => fmt.field("Ipaddr", &val),
                Wgallowedip::CidrMask(val) => fmt.field("CidrMask", &val),
                Wgallowedip::Flags(val) => fmt.field(
                    "Flags",
                    &FormatFlags(val.into(), WgallowedipFlags::from_value),
                ),
            };
        }
        fmt.finish()
    }
}
impl IterableWgallowedip<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Wgallowedip", offset));
            return (
                stack,
                missing_type.and_then(|t| Wgallowedip::attr_from_type(t)),
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
                Wgallowedip::Family(val) => {
                    if last_off == offset {
                        stack.push(("Family", last_off));
                        break;
                    }
                }
                Wgallowedip::Ipaddr(val) => {
                    if last_off == offset {
                        stack.push(("Ipaddr", last_off));
                        break;
                    }
                }
                Wgallowedip::CidrMask(val) => {
                    if last_off == offset {
                        stack.push(("CidrMask", last_off));
                        break;
                    }
                }
                Wgallowedip::Flags(val) => {
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
            stack.push(("Wgallowedip", cur));
        }
        (stack, None)
    }
}
pub struct PushWgdevice<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushWgdevice<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
pub struct PushArrayWgpeer<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
    pub(crate) counter: u16,
}
impl<Prev: Pusher> Pusher for PushArrayWgpeer<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushArrayWgpeer<Prev> {
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
    pub fn entry_nested(mut self) -> PushWgpeer<Self> {
        let index = self.counter;
        self.counter += 1;
        let header_offset = push_nested_header(self.as_vec_mut(), index);
        PushWgpeer {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushArrayWgpeer<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
impl<Prev: Pusher> PushWgdevice<Prev> {
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
    pub fn push_ifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_ifname_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Set to all zeros to remove.\n"]
    pub fn push_private_key(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_public_key(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 4u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "`0` or `WGDEVICE_F_REPLACE_PEERS` if all current peers should be removed\nprior to adding the list below.\n\nAssociated type: [`WgdeviceFlags`] (enum)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Set as `0` to choose randomly.\n"]
    pub fn push_listen_port(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 6u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Set as `0` to disable.\n"]
    pub fn push_fwmark(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The index/type parameter is unused on `SET_DEVICE` operations and is\nzero on `GET_DEVICE` operations.\n"]
    pub fn array_peers(mut self) -> PushArrayWgpeer<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 8u16);
        PushArrayWgpeer {
            prev: Some(self),
            header_offset: Some(header_offset),
            counter: 0,
        }
    }
}
impl<Prev: Pusher> Drop for PushWgdevice<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushWgpeer<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushWgpeer<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
pub struct PushArrayWgallowedip<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
    pub(crate) counter: u16,
}
impl<Prev: Pusher> Pusher for PushArrayWgallowedip<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushArrayWgallowedip<Prev> {
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
    pub fn entry_nested(mut self) -> PushWgallowedip<Self> {
        let index = self.counter;
        self.counter += 1;
        let header_offset = push_nested_header(self.as_vec_mut(), index);
        PushWgallowedip {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushArrayWgallowedip<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
impl<Prev: Pusher> PushWgpeer<Prev> {
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
    pub fn push_public_key(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "Set as all zeros to remove.\n"]
    pub fn push_preshared_key(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "`0` and/or `WGPEER_F_REMOVE_ME` if the specified peer should not exist\nat the end of the operation, rather than added/updated and/or\n`WGPEER_F_REPLACE_ALLOWEDIPS` if all current allowed IPs of this peer\nshould be removed prior to adding the list below and/or\n`WGPEER_F_UPDATE_ONLY` if the peer should only be set if it already\nexists.\n\nAssociated type: [`WgpeerFlags`] (enum)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "struct sockaddr_in or struct sockaddr_in6\n"]
    pub fn push_endpoint(mut self, value: std::net::SocketAddr) -> Self {
        push_header(self.as_vec_mut(), 4u16, {
            match &value {
                SocketAddr::V4(_) => 16,
                SocketAddr::V6(_) => 28,
            }
        } as u16);
        encode_sockaddr(self.as_vec_mut(), value);
        self
    }
    #[doc = "Set as `0` to disable.\n"]
    pub fn push_persistent_keepalive_interval(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 5u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_last_handshake_time(mut self, value: KernelTimespec) -> Self {
        push_header(self.as_vec_mut(), 6u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_rx_bytes(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 7u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tx_bytes(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 8u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The index/type parameter is unused on `SET_DEVICE` operations and is\nzero on `GET_DEVICE` operations.\n"]
    pub fn array_allowedips(mut self) -> PushArrayWgallowedip<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 9u16);
        PushArrayWgallowedip {
            prev: Some(self),
            header_offset: Some(header_offset),
            counter: 0,
        }
    }
    #[doc = "Should not be set or used at all by most users of this API, as the most\nrecent protocol will be used when this is unset. Otherwise, must be set\nto `1`.\n"]
    pub fn push_protocol_version(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushWgpeer<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushWgallowedip<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushWgallowedip<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushWgallowedip<Prev> {
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
    #[doc = "IP family, either `AF_INET` or `AF_INET6`.\n"]
    pub fn push_family(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 1u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Either `struct in_addr` or `struct in6_addr`.\n"]
    pub fn push_ipaddr(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_vec_mut(), 2u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_vec_mut(), value);
        self
    }
    pub fn push_cidr_mask(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 3u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "`WGALLOWEDIP_F_REMOVE_ME` if the specified IP should be removed;\notherwise, this IP will be added if it is not already present.\n\nAssociated type: [`WgallowedipFlags`] (enum)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushWgallowedip<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "# Retrieve WireGuard device\n\nThe command should be called with one but not both of:\n\n- `WGDEVICE_A_IFINDEX`\n- `WGDEVICE_A_IFNAME`\n\nThe kernel will then return several messages (`NLM_F_MULTI`). It is\npossible that all of the allowed IPs of a single peer will not fit\nwithin a single netlink message. In that case, the same peer will be\nwritten in the following message, except it will only contain\n`WGPEER_A_PUBLIC_KEY` and `WGPEER_A_ALLOWEDIPS`. This may occur several\ntimes in a row for the same peer. It is then up to the receiver to\ncoalesce adjacent peers. Likewise, it is possible that all peers will\nnot fit within a single message. So, subsequent peers will be sent in\nfollowing messages, except those will only contain `WGDEVICE_A_IFNAME`\nand `WGDEVICE_A_PEERS`. It is then up to the receiver to coalesce these\nmessages to form the complete list of peers.\n\nSince this is an `NLA_F_DUMP` command, the final message will always be\n`NLMSG_DONE`, even if an error occurs. However, this `NLMSG_DONE`\nmessage contains an integer error code. It is either zero or a negative\nerror code corresponding to the errno.\n\nFlags: uns-admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushWgdevice::push_ifindex)\n- [.push_ifname()](PushWgdevice::push_ifname)\n\nReply attributes:\n- [.get_ifindex()](IterableWgdevice::get_ifindex)\n- [.get_ifname()](IterableWgdevice::get_ifname)\n- [.get_private_key()](IterableWgdevice::get_private_key)\n- [.get_public_key()](IterableWgdevice::get_public_key)\n- [.get_flags()](IterableWgdevice::get_flags)\n- [.get_listen_port()](IterableWgdevice::get_listen_port)\n- [.get_fwmark()](IterableWgdevice::get_fwmark)\n- [.get_peers()](IterableWgdevice::get_peers)\n\n"]
#[derive(Debug)]
pub struct OpGetDeviceDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetDeviceDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushWgdevice<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushWgdevice::new(buf)
    }
    pub fn encode(&mut self) -> PushWgdevice<&mut Vec<u8>> {
        PushWgdevice::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushWgdevice<RequestBuf<'r>> {
        PushWgdevice::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableWgdevice<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableWgdevice::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpGetDeviceDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("wireguard".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableWgdevice<'buf>;
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
#[doc = "# Set WireGuard device\n\nThis command should be called with a wgdevice set, containing one but\nnot both of `WGDEVICE_A_IFINDEX` and `WGDEVICE_A_IFNAME`.\n\nIt is possible that the amount of configuration data exceeds that of the\nmaximum message length accepted by the kernel. In that case, several\nmessages should be sent one after another, with each successive one\nfilling in information not contained in the prior. Note that if\n`WGDEVICE_F_REPLACE_PEERS` is specified in the first message, it\nprobably should not be specified in fragments that come after, so that\nthe list of peers is only cleared the first time but appended after.\nLikewise for peers, if `WGPEER_F_REPLACE_ALLOWEDIPS` is specified in the\nfirst message of a peer, it likely should not be specified in subsequent\nfragments.\n\nIf an error occurs, `NLMSG_ERROR` will reply containing an errno.\n\nFlags: uns-admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushWgdevice::push_ifindex)\n- [.push_ifname()](PushWgdevice::push_ifname)\n- [.push_private_key()](PushWgdevice::push_private_key)\n- [.push_public_key()](PushWgdevice::push_public_key)\n- [.push_flags()](PushWgdevice::push_flags)\n- [.push_listen_port()](PushWgdevice::push_listen_port)\n- [.push_fwmark()](PushWgdevice::push_fwmark)\n- [.array_peers()](PushWgdevice::array_peers)\n\n"]
#[derive(Debug)]
pub struct OpSetDeviceDo<'r> {
    request: Request<'r>,
}
impl<'r> OpSetDeviceDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushWgdevice<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushWgdevice::new(buf)
    }
    pub fn encode(&mut self) -> PushWgdevice<&mut Vec<u8>> {
        PushWgdevice::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushWgdevice<RequestBuf<'r>> {
        PushWgdevice::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableWgdevice<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableWgdevice::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpSetDeviceDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("wireguard".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableWgdevice<'buf>;
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
    #[doc = "# Retrieve WireGuard device\n\nThe command should be called with one but not both of:\n\n- `WGDEVICE_A_IFINDEX`\n- `WGDEVICE_A_IFNAME`\n\nThe kernel will then return several messages (`NLM_F_MULTI`). It is\npossible that all of the allowed IPs of a single peer will not fit\nwithin a single netlink message. In that case, the same peer will be\nwritten in the following message, except it will only contain\n`WGPEER_A_PUBLIC_KEY` and `WGPEER_A_ALLOWEDIPS`. This may occur several\ntimes in a row for the same peer. It is then up to the receiver to\ncoalesce adjacent peers. Likewise, it is possible that all peers will\nnot fit within a single message. So, subsequent peers will be sent in\nfollowing messages, except those will only contain `WGDEVICE_A_IFNAME`\nand `WGDEVICE_A_PEERS`. It is then up to the receiver to coalesce these\nmessages to form the complete list of peers.\n\nSince this is an `NLA_F_DUMP` command, the final message will always be\n`NLMSG_DONE`, even if an error occurs. However, this `NLMSG_DONE`\nmessage contains an integer error code. It is either zero or a negative\nerror code corresponding to the errno.\n\nFlags: uns-admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushWgdevice::push_ifindex)\n- [.push_ifname()](PushWgdevice::push_ifname)\n\nReply attributes:\n- [.get_ifindex()](IterableWgdevice::get_ifindex)\n- [.get_ifname()](IterableWgdevice::get_ifname)\n- [.get_private_key()](IterableWgdevice::get_private_key)\n- [.get_public_key()](IterableWgdevice::get_public_key)\n- [.get_flags()](IterableWgdevice::get_flags)\n- [.get_listen_port()](IterableWgdevice::get_listen_port)\n- [.get_fwmark()](IterableWgdevice::get_fwmark)\n- [.get_peers()](IterableWgdevice::get_peers)\n\n"]
    pub fn op_get_device_dump(self) -> OpGetDeviceDump<'buf> {
        let mut res = OpGetDeviceDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-get-device-dump",
            OpGetDeviceDump::lookup,
        );
        res
    }
    #[doc = "# Set WireGuard device\n\nThis command should be called with a wgdevice set, containing one but\nnot both of `WGDEVICE_A_IFINDEX` and `WGDEVICE_A_IFNAME`.\n\nIt is possible that the amount of configuration data exceeds that of the\nmaximum message length accepted by the kernel. In that case, several\nmessages should be sent one after another, with each successive one\nfilling in information not contained in the prior. Note that if\n`WGDEVICE_F_REPLACE_PEERS` is specified in the first message, it\nprobably should not be specified in fragments that come after, so that\nthe list of peers is only cleared the first time but appended after.\nLikewise for peers, if `WGPEER_F_REPLACE_ALLOWEDIPS` is specified in the\nfirst message of a peer, it likely should not be specified in subsequent\nfragments.\n\nIf an error occurs, `NLMSG_ERROR` will reply containing an errno.\n\nFlags: uns-admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushWgdevice::push_ifindex)\n- [.push_ifname()](PushWgdevice::push_ifname)\n- [.push_private_key()](PushWgdevice::push_private_key)\n- [.push_public_key()](PushWgdevice::push_public_key)\n- [.push_flags()](PushWgdevice::push_flags)\n- [.push_listen_port()](PushWgdevice::push_listen_port)\n- [.push_fwmark()](PushWgdevice::push_fwmark)\n- [.array_peers()](PushWgdevice::array_peers)\n\n"]
    pub fn op_set_device_do(self) -> OpSetDeviceDo<'buf> {
        let mut res = OpSetDeviceDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-set-device-do", OpSetDeviceDo::lookup);
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableWgdevice::get_flags;
        let _ = IterableWgdevice::get_fwmark;
        let _ = IterableWgdevice::get_ifindex;
        let _ = IterableWgdevice::get_ifname;
        let _ = IterableWgdevice::get_listen_port;
        let _ = IterableWgdevice::get_peers;
        let _ = IterableWgdevice::get_private_key;
        let _ = IterableWgdevice::get_public_key;
        let _ = PushWgdevice::<&mut Vec<u8>>::array_peers;
        let _ = PushWgdevice::<&mut Vec<u8>>::push_flags;
        let _ = PushWgdevice::<&mut Vec<u8>>::push_fwmark;
        let _ = PushWgdevice::<&mut Vec<u8>>::push_ifindex;
        let _ = PushWgdevice::<&mut Vec<u8>>::push_ifname;
        let _ = PushWgdevice::<&mut Vec<u8>>::push_listen_port;
        let _ = PushWgdevice::<&mut Vec<u8>>::push_private_key;
        let _ = PushWgdevice::<&mut Vec<u8>>::push_public_key;
    }
}
