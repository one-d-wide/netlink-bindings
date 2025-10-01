#![doc = "Netlink protocol to control WireGuard network devices.\n\nThe below enums and macros are for interfacing with WireGuard, using generic\nnetlink, with family WG_GENL_NAME and version WG_GENL_VERSION. It defines two\ncommands: get and set. Note that while they share many common attributes,\nthese two commands actually accept a slightly different set of inputs and\noutputs. These differences are noted under the individual attributes.\n"]
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
use crate::builtin::{PushBuiltinBitfield32, PushBuiltinNfgenmsg};
use crate::consts;
use crate::utils::*;
use crate::{NetlinkRequest, Protocol};
pub const PROTONAME: &CStr = c"wireguard";
pub const KEY_LEN: u64 = 32u64;
#[doc = "Original name: \"wgdevice-flags\" (flags) - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Clone)]
pub enum WgdeviceFlags {
    ReplacePeers = 1 << 0,
}
#[doc = "Original name: \"wgpeer-flags\" (flags) - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Clone)]
pub enum WgpeerFlags {
    RemoveMe = 1 << 0,
    ReplaceAllowedips = 1 << 1,
    UpdateOnly = 1 << 2,
}
#[doc = "Original name: \"wgallowedip-flags\" (flags) - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Clone)]
pub enum WgallowedipFlags {
    RemoveMe = 1 << 0,
}
#[doc = "Original name: \"wgdevice\""]
#[derive(Clone)]
pub enum Wgdevice<'a> {
    Ifindex(u32),
    Ifname(&'a CStr),
    #[doc = "Set to all zeros to remove."]
    PrivateKey(&'a [u8]),
    PublicKey(&'a [u8]),
    #[doc = "0 or WGDEVICE_F_REPLACE_PEERS if all current peers\nshould be removed prior to adding the list below.\n\nAssociated type: \"WgdeviceFlags\" (enum)"]
    Flags(u32),
    #[doc = "Set as 0 to choose randomly."]
    ListenPort(u16),
    #[doc = "Set as 0 to disable."]
    Fwmark(u32),
    Peers(Iterable<'a, Iterable<'a, Wgpeer<'a>>>),
}
impl<'a> Iterable<'a, Wgdevice<'a>> {
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Wgdevice::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("Wgdevice", "Ifindex"))
    }
    pub fn get_ifname(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Wgdevice::Ifname(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("Wgdevice", "Ifname"))
    }
    #[doc = "Set to all zeros to remove."]
    pub fn get_private_key(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Wgdevice::PrivateKey(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("Wgdevice", "PrivateKey"))
    }
    pub fn get_public_key(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Wgdevice::PublicKey(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("Wgdevice", "PublicKey"))
    }
    #[doc = "0 or WGDEVICE_F_REPLACE_PEERS if all current peers\nshould be removed prior to adding the list below.\n\nAssociated type: \"WgdeviceFlags\" (enum)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Wgdevice::Flags(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("Wgdevice", "Flags"))
    }
    #[doc = "Set as 0 to choose randomly."]
    pub fn get_listen_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Wgdevice::ListenPort(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("Wgdevice", "ListenPort"))
    }
    #[doc = "Set as 0 to disable."]
    pub fn get_fwmark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Wgdevice::Fwmark(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("Wgdevice", "Fwmark"))
    }
    pub fn get_peers(
        &self,
    ) -> Result<
        ArrayIterable<Iterable<'a, Iterable<'a, Wgpeer<'a>>>, Iterable<'a, Wgpeer<'a>>>,
        ErrorContext,
    > {
        for attr in self.clone() {
            if let Wgdevice::Peers(val) = attr? {
                return Ok(ArrayIterable::new(val));
            }
        }
        Err(self.error_missing("Wgdevice", "Peers"))
    }
}
impl<'a> Wgpeer<'a> {
    pub fn new_array(buf: &'a [u8]) -> Iterable<'a, Iterable<'a, Wgpeer<'a>>> {
        Iterable::new(buf)
    }
}
impl<'a> Iterator for Iterable<'a, Iterable<'a, Wgpeer<'a>>> {
    type Item = Result<Iterable<'a, Wgpeer<'a>>, ErrorContext>;
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
            "Wgpeer",
            None,
            self.buf.as_ptr().wrapping_add(self.pos),
        )))
    }
}
impl<'a> Wgdevice<'a> {
    pub fn new(buf: &'a [u8]) -> Iterable<'a, Wgdevice<'a>> {
        Iterable::new(buf)
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
impl<'a> Iterator for Iterable<'a, Wgdevice<'a>> {
    type Item = Result<Wgdevice<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
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
                    let res = Some(Iterable::with_loc(next, self.orig_loc));
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
            "Wgdevice",
            r#type.and_then(|t| Wgdevice::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, Iterable<'a, Wgpeer<'a>>> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_list()
            .entries(self.clone().map(FlattenErrorContext))
            .finish()
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, Wgdevice<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Wgdevice");
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
                Wgdevice::Ifindex(val) => fmt.field("Ifindex", &val),
                Wgdevice::Ifname(val) => fmt.field("Ifname", &val),
                Wgdevice::PrivateKey(val) => fmt.field("PrivateKey", &FormatHex(val)),
                Wgdevice::PublicKey(val) => fmt.field("PublicKey", &FormatHex(val)),
                Wgdevice::Flags(val) => fmt.field("Flags", &val),
                Wgdevice::ListenPort(val) => fmt.field("ListenPort", &val),
                Wgdevice::Fwmark(val) => fmt.field("Fwmark", &val),
                Wgdevice::Peers(val) => fmt.field("Peers", &val),
            };
        }
        fmt.finish()
    }
}
impl<'a> Iterable<'a, Wgdevice<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset {
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
#[doc = "Original name: \"wgpeer\""]
#[derive(Clone)]
pub enum Wgpeer<'a> {
    PublicKey(&'a [u8]),
    #[doc = "Set as all zeros to remove."]
    PresharedKey(&'a [u8]),
    #[doc = "0 and/or WGPEER_F_REMOVE_ME if the specified peer should not\nexist at the end of the operation, rather than added/updated\nand/or WGPEER_F_REPLACE_ALLOWEDIPS if all current allowed IPs\nof this peer should be removed prior to adding the list below\nand/or WGPEER_F_UPDATE_ONLY if the peer should only be set if\nit already exists.\n\nAssociated type: \"WgpeerFlags\" (enum)"]
    Flags(u32),
    #[doc = "struct sockaddr_in or struct sockaddr_in6"]
    Endpoint(std::net::SocketAddr),
    #[doc = "Set as 0 to disable."]
    PersistentKeepaliveInterval(u16),
    LastHandshakeTime(PushKernelTimespec),
    RxBytes(u64),
    TxBytes(u64),
    Allowedips(Iterable<'a, Iterable<'a, Wgallowedip>>),
    #[doc = "should not be set or used at all by most users of this API,\nas the most recent protocol will be used when this is unset.\nOtherwise, must be set to 1.\n"]
    ProtocolVersion(u32),
}
impl<'a> Iterable<'a, Wgpeer<'a>> {
    pub fn get_public_key(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Wgpeer::PublicKey(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("Wgpeer", "PublicKey"))
    }
    #[doc = "Set as all zeros to remove."]
    pub fn get_preshared_key(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Wgpeer::PresharedKey(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("Wgpeer", "PresharedKey"))
    }
    #[doc = "0 and/or WGPEER_F_REMOVE_ME if the specified peer should not\nexist at the end of the operation, rather than added/updated\nand/or WGPEER_F_REPLACE_ALLOWEDIPS if all current allowed IPs\nof this peer should be removed prior to adding the list below\nand/or WGPEER_F_UPDATE_ONLY if the peer should only be set if\nit already exists.\n\nAssociated type: \"WgpeerFlags\" (enum)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Wgpeer::Flags(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("Wgpeer", "Flags"))
    }
    #[doc = "struct sockaddr_in or struct sockaddr_in6"]
    pub fn get_endpoint(&self) -> Result<std::net::SocketAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Wgpeer::Endpoint(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("Wgpeer", "Endpoint"))
    }
    #[doc = "Set as 0 to disable."]
    pub fn get_persistent_keepalive_interval(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Wgpeer::PersistentKeepaliveInterval(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("Wgpeer", "PersistentKeepaliveInterval"))
    }
    pub fn get_last_handshake_time(&self) -> Result<PushKernelTimespec, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Wgpeer::LastHandshakeTime(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("Wgpeer", "LastHandshakeTime"))
    }
    pub fn get_rx_bytes(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Wgpeer::RxBytes(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("Wgpeer", "RxBytes"))
    }
    pub fn get_tx_bytes(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Wgpeer::TxBytes(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("Wgpeer", "TxBytes"))
    }
    pub fn get_allowedips(
        &self,
    ) -> Result<
        ArrayIterable<Iterable<'a, Iterable<'a, Wgallowedip>>, Iterable<'a, Wgallowedip>>,
        ErrorContext,
    > {
        for attr in self.clone() {
            if let Wgpeer::Allowedips(val) = attr? {
                return Ok(ArrayIterable::new(val));
            }
        }
        Err(self.error_missing("Wgpeer", "Allowedips"))
    }
    #[doc = "should not be set or used at all by most users of this API,\nas the most recent protocol will be used when this is unset.\nOtherwise, must be set to 1.\n"]
    pub fn get_protocol_version(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Wgpeer::ProtocolVersion(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("Wgpeer", "ProtocolVersion"))
    }
}
impl<'a> Wgallowedip {
    pub fn new_array(buf: &'a [u8]) -> Iterable<'a, Iterable<'a, Wgallowedip>> {
        Iterable::new(buf)
    }
}
impl<'a> Iterator for Iterable<'a, Iterable<'a, Wgallowedip>> {
    type Item = Result<Iterable<'a, Wgallowedip>, ErrorContext>;
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
            "Wgallowedip",
            None,
            self.buf.as_ptr().wrapping_add(self.pos),
        )))
    }
}
impl<'a> Wgpeer<'a> {
    pub fn new(buf: &'a [u8]) -> Iterable<'a, Wgpeer<'a>> {
        Iterable::new(buf)
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
impl<'a> Iterator for Iterable<'a, Wgpeer<'a>> {
    type Item = Result<Wgpeer<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
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
                    let res = PushKernelTimespec::new_from_slice(next);
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
                    let res = Some(Iterable::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => Wgpeer::ProtocolVersion({
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
            "Wgpeer",
            r#type.and_then(|t| Wgpeer::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, Iterable<'a, Wgallowedip>> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_list()
            .entries(self.clone().map(FlattenErrorContext))
            .finish()
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, Wgpeer<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Wgpeer");
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
                Wgpeer::PublicKey(val) => fmt.field("PublicKey", &FormatHex(val)),
                Wgpeer::PresharedKey(val) => fmt.field("PresharedKey", &FormatHex(val)),
                Wgpeer::Flags(val) => fmt.field("Flags", &val),
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
impl<'a> Iterable<'a, Wgpeer<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset {
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
#[doc = "Original name: \"wgallowedip\""]
#[derive(Clone)]
pub enum Wgallowedip {
    Family(u16),
    #[doc = "struct in_addr or struct in6_add"]
    Ipaddr(std::net::IpAddr),
    CidrMask(u8),
    #[doc = "WGALLOWEDIP_F_REMOVE_ME if the specified IP should be removed;\notherwise, this IP will be added if it is not already present.\n\nAssociated type: \"WgallowedipFlags\" (enum)"]
    Flags(u32),
}
impl<'a> Iterable<'a, Wgallowedip> {
    pub fn get_family(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Wgallowedip::Family(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("Wgallowedip", "Family"))
    }
    #[doc = "struct in_addr or struct in6_add"]
    pub fn get_ipaddr(&self) -> Result<std::net::IpAddr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Wgallowedip::Ipaddr(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("Wgallowedip", "Ipaddr"))
    }
    pub fn get_cidr_mask(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Wgallowedip::CidrMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("Wgallowedip", "CidrMask"))
    }
    #[doc = "WGALLOWEDIP_F_REMOVE_ME if the specified IP should be removed;\notherwise, this IP will be added if it is not already present.\n\nAssociated type: \"WgallowedipFlags\" (enum)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Wgallowedip::Flags(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("Wgallowedip", "Flags"))
    }
}
impl Wgallowedip {
    pub fn new(buf: &'_ [u8]) -> Iterable<'_, Wgallowedip> {
        Iterable::new(buf)
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
impl Iterator for Iterable<'_, Wgallowedip> {
    type Item = Result<Wgallowedip, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
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
            "Wgallowedip",
            r#type.and_then(|t| Wgallowedip::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl std::fmt::Debug for Iterable<'_, Wgallowedip> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Wgallowedip");
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
                Wgallowedip::Family(val) => fmt.field("Family", &val),
                Wgallowedip::Ipaddr(val) => fmt.field("Ipaddr", &val),
                Wgallowedip::CidrMask(val) => fmt.field("CidrMask", &val),
                Wgallowedip::Flags(val) => fmt.field("Flags", &val),
            };
        }
        fmt.finish()
    }
}
impl Iterable<'_, Wgallowedip> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset {
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
pub struct PushWgdevice<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushWgdevice<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
pub struct PushArrayWgpeer<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
    pub(crate) counter: u16,
}
impl<Prev: Rec> Rec for PushArrayWgpeer<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushArrayWgpeer<Prev> {
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
    pub fn entry_nested(mut self) -> PushWgpeer<Self> {
        let index = self.counter;
        self.counter += 1;
        let header_offset = push_nested_header(self.as_rec_mut(), index);
        PushWgpeer {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushArrayWgpeer<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
impl<Prev: Rec> PushWgdevice<Prev> {
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
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_ifname_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    #[doc = "Set to all zeros to remove."]
    pub fn push_private_key(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 3u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_public_key(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 4u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    #[doc = "0 or WGDEVICE_F_REPLACE_PEERS if all current peers\nshould be removed prior to adding the list below.\n\nAssociated type: \"WgdeviceFlags\" (enum)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 5u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Set as 0 to choose randomly."]
    pub fn push_listen_port(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 6u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Set as 0 to disable."]
    pub fn push_fwmark(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 7u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn array_peers(mut self) -> PushArrayWgpeer<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 8u16);
        PushArrayWgpeer {
            prev: Some(self),
            header_offset: Some(header_offset),
            counter: 0,
        }
    }
}
impl<Prev: Rec> Drop for PushWgdevice<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushWgpeer<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushWgpeer<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
pub struct PushArrayWgallowedip<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
    pub(crate) counter: u16,
}
impl<Prev: Rec> Rec for PushArrayWgallowedip<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushArrayWgallowedip<Prev> {
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
    pub fn entry_nested(mut self) -> PushWgallowedip<Self> {
        let index = self.counter;
        self.counter += 1;
        let header_offset = push_nested_header(self.as_rec_mut(), index);
        PushWgallowedip {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushArrayWgallowedip<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
impl<Prev: Rec> PushWgpeer<Prev> {
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
    pub fn push_public_key(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 1u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    #[doc = "Set as all zeros to remove."]
    pub fn push_preshared_key(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    #[doc = "0 and/or WGPEER_F_REMOVE_ME if the specified peer should not\nexist at the end of the operation, rather than added/updated\nand/or WGPEER_F_REPLACE_ALLOWEDIPS if all current allowed IPs\nof this peer should be removed prior to adding the list below\nand/or WGPEER_F_UPDATE_ONLY if the peer should only be set if\nit already exists.\n\nAssociated type: \"WgpeerFlags\" (enum)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "struct sockaddr_in or struct sockaddr_in6"]
    pub fn push_endpoint(mut self, value: std::net::SocketAddr) -> Self {
        push_header(self.as_rec_mut(), 4u16, {
            match &value {
                SocketAddr::V4(_) => 16,
                SocketAddr::V6(_) => 28,
            }
        } as u16);
        encode_sockaddr(self.as_rec_mut(), value);
        self
    }
    #[doc = "Set as 0 to disable."]
    pub fn push_persistent_keepalive_interval(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 5u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_last_handshake_time(mut self, value: PushKernelTimespec) -> Self {
        push_header(self.as_rec_mut(), 6u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_rx_bytes(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 7u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tx_bytes(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 8u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn array_allowedips(mut self) -> PushArrayWgallowedip<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 9u16);
        PushArrayWgallowedip {
            prev: Some(self),
            header_offset: Some(header_offset),
            counter: 0,
        }
    }
    #[doc = "should not be set or used at all by most users of this API,\nas the most recent protocol will be used when this is unset.\nOtherwise, must be set to 1.\n"]
    pub fn push_protocol_version(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 10u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushWgpeer<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushWgallowedip<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushWgallowedip<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushWgallowedip<Prev> {
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
    pub fn push_family(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 1u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "struct in_addr or struct in6_add"]
    pub fn push_ipaddr(mut self, value: std::net::IpAddr) -> Self {
        push_header(self.as_rec_mut(), 2u16, {
            match &value {
                IpAddr::V4(_) => 4,
                IpAddr::V6(_) => 16,
            }
        } as u16);
        encode_ip(self.as_rec_mut(), value);
        self
    }
    pub fn push_cidr_mask(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 3u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "WGALLOWEDIP_F_REMOVE_ME if the specified IP should be removed;\notherwise, this IP will be added if it is not already present.\n\nAssociated type: \"WgallowedipFlags\" (enum)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushWgallowedip<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Original name: \"--kernel-timespec\""]
#[derive(Clone)]
pub struct PushKernelTimespec {
    buf: [u8; 16usize],
}
impl PushKernelTimespec {
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
    #[doc = "Number of seconds, since UNIX epoch."]
    pub fn sec(&self) -> u64 {
        parse_u64(&self.buf[0usize..8usize]).unwrap()
    }
    #[doc = "Number of seconds, since UNIX epoch."]
    pub fn set_sec(&mut self, value: u64) {
        self.buf[0usize..8usize].copy_from_slice(&value.to_ne_bytes())
    }
    #[doc = "Number of nanoseconds, after the second began."]
    pub fn nsec(&self) -> u64 {
        parse_u64(&self.buf[8usize..16usize]).unwrap()
    }
    #[doc = "Number of nanoseconds, after the second began."]
    pub fn set_nsec(&mut self, value: u64) {
        self.buf[8usize..16usize].copy_from_slice(&value.to_ne_bytes())
    }
}
impl std::fmt::Debug for PushKernelTimespec {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("KernelTimespec")
            .field("sec", &self.sec())
            .field("nsec", &self.nsec())
            .finish()
    }
}
#[doc = "Retrieve WireGuard device.\n\nThe command should be called with one but not both of:\n* WGDEVICE_A_IFINDEX\n* WGDEVICE_A_IFNAME\n\nThe kernel will then return several messages (NLM_F_MULTI).\nIt is possible that all of the allowed IPs of a single peer will not\nfit within a single netlink message. In that case, the same peer will\nbe written in the following message, except it will only contain\nWGPEER_A_PUBLIC_KEY and WGPEER_A_ALLOWEDIPS. This may occur several\ntimes in a row for the same peer. It is then up to the receiver to\ncoalesce adjacent peers. Likewise, it is possible that all peers will\nnot fit within a single message. So, subsequent peers will be sent\nin following messages, except those will only contain\nWGDEVICE_A_IFNAME and WGDEVICE_A_PEERS. It is then up to the receiver\nto coalesce these messages to form the complete list of peers.\n\nSince this is an NLA_F_DUMP command, the final message will always be\nNLMSG_DONE, even if an error occurs. However, this NLMSG_DONE message\ncontains an integer error code. It is either zero or a negative error\ncode corresponding to the errno.\n"]
pub struct PushOpGetDeviceDumpRequest<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetDeviceDumpRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetDeviceDumpRequest<Prev> {
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
        header.set_cmd(0u8);
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
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_ifname_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    #[doc = "Set to all zeros to remove."]
    pub fn push_private_key(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 3u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_public_key(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 4u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    #[doc = "0 or WGDEVICE_F_REPLACE_PEERS if all current peers\nshould be removed prior to adding the list below.\n\nAssociated type: \"WgdeviceFlags\" (enum)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 5u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Set as 0 to choose randomly."]
    pub fn push_listen_port(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 6u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Set as 0 to disable."]
    pub fn push_fwmark(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 7u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn array_peers(mut self) -> PushArrayWgpeer<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 8u16);
        PushArrayWgpeer {
            prev: Some(self),
            header_offset: Some(header_offset),
            counter: 0,
        }
    }
}
impl<Prev: Rec> Drop for PushOpGetDeviceDumpRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Retrieve WireGuard device.\n\nThe command should be called with one but not both of:\n* WGDEVICE_A_IFINDEX\n* WGDEVICE_A_IFNAME\n\nThe kernel will then return several messages (NLM_F_MULTI).\nIt is possible that all of the allowed IPs of a single peer will not\nfit within a single netlink message. In that case, the same peer will\nbe written in the following message, except it will only contain\nWGPEER_A_PUBLIC_KEY and WGPEER_A_ALLOWEDIPS. This may occur several\ntimes in a row for the same peer. It is then up to the receiver to\ncoalesce adjacent peers. Likewise, it is possible that all peers will\nnot fit within a single message. So, subsequent peers will be sent\nin following messages, except those will only contain\nWGDEVICE_A_IFNAME and WGDEVICE_A_PEERS. It is then up to the receiver\nto coalesce these messages to form the complete list of peers.\n\nSince this is an NLA_F_DUMP command, the final message will always be\nNLMSG_DONE, even if an error occurs. However, this NLMSG_DONE message\ncontains an integer error code. It is either zero or a negative error\ncode corresponding to the errno.\n"]
#[doc = "Original name: \"op-get-device-dump-request\""]
#[derive(Clone)]
pub enum OpGetDeviceDumpRequest<'a> {
    Ifindex(u32),
    Ifname(&'a CStr),
    #[doc = "Set to all zeros to remove."]
    PrivateKey(&'a [u8]),
    PublicKey(&'a [u8]),
    #[doc = "0 or WGDEVICE_F_REPLACE_PEERS if all current peers\nshould be removed prior to adding the list below.\n\nAssociated type: \"WgdeviceFlags\" (enum)"]
    Flags(u32),
    #[doc = "Set as 0 to choose randomly."]
    ListenPort(u16),
    #[doc = "Set as 0 to disable."]
    Fwmark(u32),
    Peers(Iterable<'a, Iterable<'a, Wgpeer<'a>>>),
}
impl<'a> Iterable<'a, OpGetDeviceDumpRequest<'a>> {
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDeviceDumpRequest::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetDeviceDumpRequest", "Ifindex"))
    }
    pub fn get_ifname(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDeviceDumpRequest::Ifname(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetDeviceDumpRequest", "Ifname"))
    }
    #[doc = "Set to all zeros to remove."]
    pub fn get_private_key(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDeviceDumpRequest::PrivateKey(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetDeviceDumpRequest", "PrivateKey"))
    }
    pub fn get_public_key(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDeviceDumpRequest::PublicKey(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetDeviceDumpRequest", "PublicKey"))
    }
    #[doc = "0 or WGDEVICE_F_REPLACE_PEERS if all current peers\nshould be removed prior to adding the list below.\n\nAssociated type: \"WgdeviceFlags\" (enum)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDeviceDumpRequest::Flags(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetDeviceDumpRequest", "Flags"))
    }
    #[doc = "Set as 0 to choose randomly."]
    pub fn get_listen_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDeviceDumpRequest::ListenPort(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetDeviceDumpRequest", "ListenPort"))
    }
    #[doc = "Set as 0 to disable."]
    pub fn get_fwmark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDeviceDumpRequest::Fwmark(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetDeviceDumpRequest", "Fwmark"))
    }
    pub fn get_peers(
        &self,
    ) -> Result<
        ArrayIterable<Iterable<'a, Iterable<'a, Wgpeer<'a>>>, Iterable<'a, Wgpeer<'a>>>,
        ErrorContext,
    > {
        for attr in self.clone() {
            if let OpGetDeviceDumpRequest::Peers(val) = attr? {
                return Ok(ArrayIterable::new(val));
            }
        }
        Err(self.error_missing("OpGetDeviceDumpRequest", "Peers"))
    }
}
impl<'a> OpGetDeviceDumpRequest<'a> {
    pub fn new(buf: &'a [u8]) -> Iterable<'a, OpGetDeviceDumpRequest<'a>> {
        let mut header = PushBuiltinNfgenmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushBuiltinNfgenmsg::len()]);
        Iterable::with_loc(&buf[PushBuiltinNfgenmsg::len()..], buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Wgdevice::attr_from_type(r#type)
    }
}
impl<'a> Iterator for Iterable<'a, OpGetDeviceDumpRequest<'a>> {
    type Item = Result<OpGetDeviceDumpRequest<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpGetDeviceDumpRequest::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpGetDeviceDumpRequest::Ifname({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpGetDeviceDumpRequest::PrivateKey({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpGetDeviceDumpRequest::PublicKey({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpGetDeviceDumpRequest::Flags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpGetDeviceDumpRequest::ListenPort({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => OpGetDeviceDumpRequest::Fwmark({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpGetDeviceDumpRequest::Peers({
                    let res = Some(Iterable::with_loc(next, self.orig_loc));
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
            "OpGetDeviceDumpRequest",
            r#type.and_then(|t| OpGetDeviceDumpRequest::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpGetDeviceDumpRequest<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetDeviceDumpRequest");
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
                OpGetDeviceDumpRequest::Ifindex(val) => fmt.field("Ifindex", &val),
                OpGetDeviceDumpRequest::Ifname(val) => fmt.field("Ifname", &val),
                OpGetDeviceDumpRequest::PrivateKey(val) => fmt.field("PrivateKey", &FormatHex(val)),
                OpGetDeviceDumpRequest::PublicKey(val) => fmt.field("PublicKey", &FormatHex(val)),
                OpGetDeviceDumpRequest::Flags(val) => fmt.field("Flags", &val),
                OpGetDeviceDumpRequest::ListenPort(val) => fmt.field("ListenPort", &val),
                OpGetDeviceDumpRequest::Fwmark(val) => fmt.field("Fwmark", &val),
                OpGetDeviceDumpRequest::Peers(val) => fmt.field("Peers", &val),
            };
        }
        fmt.finish()
    }
}
impl<'a> Iterable<'a, OpGetDeviceDumpRequest<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpGetDeviceDumpRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetDeviceDumpRequest::attr_from_type(t)),
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
                OpGetDeviceDumpRequest::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OpGetDeviceDumpRequest::Ifname(val) => {
                    if last_off == offset {
                        stack.push(("Ifname", last_off));
                        break;
                    }
                }
                OpGetDeviceDumpRequest::PrivateKey(val) => {
                    if last_off == offset {
                        stack.push(("PrivateKey", last_off));
                        break;
                    }
                }
                OpGetDeviceDumpRequest::PublicKey(val) => {
                    if last_off == offset {
                        stack.push(("PublicKey", last_off));
                        break;
                    }
                }
                OpGetDeviceDumpRequest::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                OpGetDeviceDumpRequest::ListenPort(val) => {
                    if last_off == offset {
                        stack.push(("ListenPort", last_off));
                        break;
                    }
                }
                OpGetDeviceDumpRequest::Fwmark(val) => {
                    if last_off == offset {
                        stack.push(("Fwmark", last_off));
                        break;
                    }
                }
                OpGetDeviceDumpRequest::Peers(val) => {
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
            stack.push(("OpGetDeviceDumpRequest", cur));
        }
        (stack, missing)
    }
}
#[doc = "Retrieve WireGuard device.\n\nThe command should be called with one but not both of:\n* WGDEVICE_A_IFINDEX\n* WGDEVICE_A_IFNAME\n\nThe kernel will then return several messages (NLM_F_MULTI).\nIt is possible that all of the allowed IPs of a single peer will not\nfit within a single netlink message. In that case, the same peer will\nbe written in the following message, except it will only contain\nWGPEER_A_PUBLIC_KEY and WGPEER_A_ALLOWEDIPS. This may occur several\ntimes in a row for the same peer. It is then up to the receiver to\ncoalesce adjacent peers. Likewise, it is possible that all peers will\nnot fit within a single message. So, subsequent peers will be sent\nin following messages, except those will only contain\nWGDEVICE_A_IFNAME and WGDEVICE_A_PEERS. It is then up to the receiver\nto coalesce these messages to form the complete list of peers.\n\nSince this is an NLA_F_DUMP command, the final message will always be\nNLMSG_DONE, even if an error occurs. However, this NLMSG_DONE message\ncontains an integer error code. It is either zero or a negative error\ncode corresponding to the errno.\n"]
pub struct PushOpGetDeviceDumpReply<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetDeviceDumpReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetDeviceDumpReply<Prev> {
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
        header.set_cmd(0u8);
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
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_ifname_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    #[doc = "Set to all zeros to remove."]
    pub fn push_private_key(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 3u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_public_key(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 4u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    #[doc = "0 or WGDEVICE_F_REPLACE_PEERS if all current peers\nshould be removed prior to adding the list below.\n\nAssociated type: \"WgdeviceFlags\" (enum)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 5u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Set as 0 to choose randomly."]
    pub fn push_listen_port(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 6u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Set as 0 to disable."]
    pub fn push_fwmark(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 7u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn array_peers(mut self) -> PushArrayWgpeer<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 8u16);
        PushArrayWgpeer {
            prev: Some(self),
            header_offset: Some(header_offset),
            counter: 0,
        }
    }
}
impl<Prev: Rec> Drop for PushOpGetDeviceDumpReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Retrieve WireGuard device.\n\nThe command should be called with one but not both of:\n* WGDEVICE_A_IFINDEX\n* WGDEVICE_A_IFNAME\n\nThe kernel will then return several messages (NLM_F_MULTI).\nIt is possible that all of the allowed IPs of a single peer will not\nfit within a single netlink message. In that case, the same peer will\nbe written in the following message, except it will only contain\nWGPEER_A_PUBLIC_KEY and WGPEER_A_ALLOWEDIPS. This may occur several\ntimes in a row for the same peer. It is then up to the receiver to\ncoalesce adjacent peers. Likewise, it is possible that all peers will\nnot fit within a single message. So, subsequent peers will be sent\nin following messages, except those will only contain\nWGDEVICE_A_IFNAME and WGDEVICE_A_PEERS. It is then up to the receiver\nto coalesce these messages to form the complete list of peers.\n\nSince this is an NLA_F_DUMP command, the final message will always be\nNLMSG_DONE, even if an error occurs. However, this NLMSG_DONE message\ncontains an integer error code. It is either zero or a negative error\ncode corresponding to the errno.\n"]
#[doc = "Original name: \"op-get-device-dump-reply\""]
#[derive(Clone)]
pub enum OpGetDeviceDumpReply<'a> {
    Ifindex(u32),
    Ifname(&'a CStr),
    #[doc = "Set to all zeros to remove."]
    PrivateKey(&'a [u8]),
    PublicKey(&'a [u8]),
    #[doc = "0 or WGDEVICE_F_REPLACE_PEERS if all current peers\nshould be removed prior to adding the list below.\n\nAssociated type: \"WgdeviceFlags\" (enum)"]
    Flags(u32),
    #[doc = "Set as 0 to choose randomly."]
    ListenPort(u16),
    #[doc = "Set as 0 to disable."]
    Fwmark(u32),
    Peers(Iterable<'a, Iterable<'a, Wgpeer<'a>>>),
}
impl<'a> Iterable<'a, OpGetDeviceDumpReply<'a>> {
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDeviceDumpReply::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetDeviceDumpReply", "Ifindex"))
    }
    pub fn get_ifname(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDeviceDumpReply::Ifname(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetDeviceDumpReply", "Ifname"))
    }
    #[doc = "Set to all zeros to remove."]
    pub fn get_private_key(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDeviceDumpReply::PrivateKey(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetDeviceDumpReply", "PrivateKey"))
    }
    pub fn get_public_key(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDeviceDumpReply::PublicKey(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetDeviceDumpReply", "PublicKey"))
    }
    #[doc = "0 or WGDEVICE_F_REPLACE_PEERS if all current peers\nshould be removed prior to adding the list below.\n\nAssociated type: \"WgdeviceFlags\" (enum)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDeviceDumpReply::Flags(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetDeviceDumpReply", "Flags"))
    }
    #[doc = "Set as 0 to choose randomly."]
    pub fn get_listen_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDeviceDumpReply::ListenPort(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetDeviceDumpReply", "ListenPort"))
    }
    #[doc = "Set as 0 to disable."]
    pub fn get_fwmark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetDeviceDumpReply::Fwmark(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetDeviceDumpReply", "Fwmark"))
    }
    pub fn get_peers(
        &self,
    ) -> Result<
        ArrayIterable<Iterable<'a, Iterable<'a, Wgpeer<'a>>>, Iterable<'a, Wgpeer<'a>>>,
        ErrorContext,
    > {
        for attr in self.clone() {
            if let OpGetDeviceDumpReply::Peers(val) = attr? {
                return Ok(ArrayIterable::new(val));
            }
        }
        Err(self.error_missing("OpGetDeviceDumpReply", "Peers"))
    }
}
impl<'a> OpGetDeviceDumpReply<'a> {
    pub fn new(buf: &'a [u8]) -> Iterable<'a, OpGetDeviceDumpReply<'a>> {
        let mut header = PushBuiltinNfgenmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushBuiltinNfgenmsg::len()]);
        Iterable::with_loc(&buf[PushBuiltinNfgenmsg::len()..], buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Wgdevice::attr_from_type(r#type)
    }
}
impl<'a> Iterator for Iterable<'a, OpGetDeviceDumpReply<'a>> {
    type Item = Result<OpGetDeviceDumpReply<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpGetDeviceDumpReply::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpGetDeviceDumpReply::Ifname({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpGetDeviceDumpReply::PrivateKey({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpGetDeviceDumpReply::PublicKey({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpGetDeviceDumpReply::Flags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpGetDeviceDumpReply::ListenPort({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => OpGetDeviceDumpReply::Fwmark({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpGetDeviceDumpReply::Peers({
                    let res = Some(Iterable::with_loc(next, self.orig_loc));
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
            "OpGetDeviceDumpReply",
            r#type.and_then(|t| OpGetDeviceDumpReply::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpGetDeviceDumpReply<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetDeviceDumpReply");
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
                OpGetDeviceDumpReply::Ifindex(val) => fmt.field("Ifindex", &val),
                OpGetDeviceDumpReply::Ifname(val) => fmt.field("Ifname", &val),
                OpGetDeviceDumpReply::PrivateKey(val) => fmt.field("PrivateKey", &FormatHex(val)),
                OpGetDeviceDumpReply::PublicKey(val) => fmt.field("PublicKey", &FormatHex(val)),
                OpGetDeviceDumpReply::Flags(val) => fmt.field("Flags", &val),
                OpGetDeviceDumpReply::ListenPort(val) => fmt.field("ListenPort", &val),
                OpGetDeviceDumpReply::Fwmark(val) => fmt.field("Fwmark", &val),
                OpGetDeviceDumpReply::Peers(val) => fmt.field("Peers", &val),
            };
        }
        fmt.finish()
    }
}
impl<'a> Iterable<'a, OpGetDeviceDumpReply<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpGetDeviceDumpReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetDeviceDumpReply::attr_from_type(t)),
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
                OpGetDeviceDumpReply::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OpGetDeviceDumpReply::Ifname(val) => {
                    if last_off == offset {
                        stack.push(("Ifname", last_off));
                        break;
                    }
                }
                OpGetDeviceDumpReply::PrivateKey(val) => {
                    if last_off == offset {
                        stack.push(("PrivateKey", last_off));
                        break;
                    }
                }
                OpGetDeviceDumpReply::PublicKey(val) => {
                    if last_off == offset {
                        stack.push(("PublicKey", last_off));
                        break;
                    }
                }
                OpGetDeviceDumpReply::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                OpGetDeviceDumpReply::ListenPort(val) => {
                    if last_off == offset {
                        stack.push(("ListenPort", last_off));
                        break;
                    }
                }
                OpGetDeviceDumpReply::Fwmark(val) => {
                    if last_off == offset {
                        stack.push(("Fwmark", last_off));
                        break;
                    }
                }
                OpGetDeviceDumpReply::Peers(val) => {
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
            stack.push(("OpGetDeviceDumpReply", cur));
        }
        (stack, missing)
    }
}
#[derive(Debug)]
pub struct RequestOpGetDeviceDumpRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpGetDeviceDumpRequest<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        PushOpGetDeviceDumpRequest::write_header(&mut request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode(&mut self) -> PushOpGetDeviceDumpRequest<&mut Vec<u8>> {
        PushOpGetDeviceDumpRequest::new_without_header(self.request.buf_mut())
    }
}
impl NetlinkRequest for RequestOpGetDeviceDumpRequest<'_> {
    type ReplyType<'buf> = Iterable<'buf, OpGetDeviceDumpReply<'buf>>;
    fn protocol(&self) -> Protocol {
        Protocol::Generic("wireguard".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpGetDeviceDumpReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpGetDeviceDumpRequest::new(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "Set WireGuard device.\n\nThis command should be called with a wgdevice set, containing one but\nnot both of WGDEVICE_A_IFINDEX and WGDEVICE_A_IFNAME.\n\nIt is possible that the amount of configuration data exceeds that of\nthe maximum message length accepted by the kernel. In that case,\nseveral messages should be sent one after another, with each\nsuccessive one filling in information not contained in the prior.\nNote that if WGDEVICE_F_REPLACE_PEERS is specified in the first\nmessage, it probably should not be specified in fragments that come\nafter, so that the list of peers is only cleared the first time but\nappended after.\nLikewise for peers, if WGPEER_F_REPLACE_ALLOWEDIPS is specified in\nthe first message of a peer, it likely should not be specified in\nsubsequent fragments.\n\nIf an error occurs, NLMSG_ERROR will reply containing an errno.\n"]
pub struct PushOpSetDeviceDoRequest<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpSetDeviceDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpSetDeviceDoRequest<Prev> {
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
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ifname(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_ifname_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    #[doc = "Set to all zeros to remove."]
    pub fn push_private_key(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 3u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_public_key(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 4u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    #[doc = "0 or WGDEVICE_F_REPLACE_PEERS if all current peers\nshould be removed prior to adding the list below.\n\nAssociated type: \"WgdeviceFlags\" (enum)"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 5u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Set as 0 to choose randomly."]
    pub fn push_listen_port(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 6u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Set as 0 to disable."]
    pub fn push_fwmark(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 7u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn array_peers(mut self) -> PushArrayWgpeer<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 8u16);
        PushArrayWgpeer {
            prev: Some(self),
            header_offset: Some(header_offset),
            counter: 0,
        }
    }
}
impl<Prev: Rec> Drop for PushOpSetDeviceDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Set WireGuard device.\n\nThis command should be called with a wgdevice set, containing one but\nnot both of WGDEVICE_A_IFINDEX and WGDEVICE_A_IFNAME.\n\nIt is possible that the amount of configuration data exceeds that of\nthe maximum message length accepted by the kernel. In that case,\nseveral messages should be sent one after another, with each\nsuccessive one filling in information not contained in the prior.\nNote that if WGDEVICE_F_REPLACE_PEERS is specified in the first\nmessage, it probably should not be specified in fragments that come\nafter, so that the list of peers is only cleared the first time but\nappended after.\nLikewise for peers, if WGPEER_F_REPLACE_ALLOWEDIPS is specified in\nthe first message of a peer, it likely should not be specified in\nsubsequent fragments.\n\nIf an error occurs, NLMSG_ERROR will reply containing an errno.\n"]
#[doc = "Original name: \"op-set-device-do-request\""]
#[derive(Clone)]
pub enum OpSetDeviceDoRequest<'a> {
    Ifindex(u32),
    Ifname(&'a CStr),
    #[doc = "Set to all zeros to remove."]
    PrivateKey(&'a [u8]),
    PublicKey(&'a [u8]),
    #[doc = "0 or WGDEVICE_F_REPLACE_PEERS if all current peers\nshould be removed prior to adding the list below.\n\nAssociated type: \"WgdeviceFlags\" (enum)"]
    Flags(u32),
    #[doc = "Set as 0 to choose randomly."]
    ListenPort(u16),
    #[doc = "Set as 0 to disable."]
    Fwmark(u32),
    Peers(Iterable<'a, Iterable<'a, Wgpeer<'a>>>),
}
impl<'a> Iterable<'a, OpSetDeviceDoRequest<'a>> {
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpSetDeviceDoRequest::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpSetDeviceDoRequest", "Ifindex"))
    }
    pub fn get_ifname(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpSetDeviceDoRequest::Ifname(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpSetDeviceDoRequest", "Ifname"))
    }
    #[doc = "Set to all zeros to remove."]
    pub fn get_private_key(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpSetDeviceDoRequest::PrivateKey(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpSetDeviceDoRequest", "PrivateKey"))
    }
    pub fn get_public_key(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpSetDeviceDoRequest::PublicKey(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpSetDeviceDoRequest", "PublicKey"))
    }
    #[doc = "0 or WGDEVICE_F_REPLACE_PEERS if all current peers\nshould be removed prior to adding the list below.\n\nAssociated type: \"WgdeviceFlags\" (enum)"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpSetDeviceDoRequest::Flags(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpSetDeviceDoRequest", "Flags"))
    }
    #[doc = "Set as 0 to choose randomly."]
    pub fn get_listen_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpSetDeviceDoRequest::ListenPort(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpSetDeviceDoRequest", "ListenPort"))
    }
    #[doc = "Set as 0 to disable."]
    pub fn get_fwmark(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpSetDeviceDoRequest::Fwmark(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpSetDeviceDoRequest", "Fwmark"))
    }
    pub fn get_peers(
        &self,
    ) -> Result<
        ArrayIterable<Iterable<'a, Iterable<'a, Wgpeer<'a>>>, Iterable<'a, Wgpeer<'a>>>,
        ErrorContext,
    > {
        for attr in self.clone() {
            if let OpSetDeviceDoRequest::Peers(val) = attr? {
                return Ok(ArrayIterable::new(val));
            }
        }
        Err(self.error_missing("OpSetDeviceDoRequest", "Peers"))
    }
}
impl<'a> OpSetDeviceDoRequest<'a> {
    pub fn new(buf: &'a [u8]) -> Iterable<'a, OpSetDeviceDoRequest<'a>> {
        let mut header = PushBuiltinNfgenmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushBuiltinNfgenmsg::len()]);
        Iterable::with_loc(&buf[PushBuiltinNfgenmsg::len()..], buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Wgdevice::attr_from_type(r#type)
    }
}
impl<'a> Iterator for Iterable<'a, OpSetDeviceDoRequest<'a>> {
    type Item = Result<OpSetDeviceDoRequest<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpSetDeviceDoRequest::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpSetDeviceDoRequest::Ifname({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpSetDeviceDoRequest::PrivateKey({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpSetDeviceDoRequest::PublicKey({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpSetDeviceDoRequest::Flags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpSetDeviceDoRequest::ListenPort({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => OpSetDeviceDoRequest::Fwmark({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpSetDeviceDoRequest::Peers({
                    let res = Some(Iterable::with_loc(next, self.orig_loc));
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
            "OpSetDeviceDoRequest",
            r#type.and_then(|t| OpSetDeviceDoRequest::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpSetDeviceDoRequest<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpSetDeviceDoRequest");
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
                OpSetDeviceDoRequest::Ifindex(val) => fmt.field("Ifindex", &val),
                OpSetDeviceDoRequest::Ifname(val) => fmt.field("Ifname", &val),
                OpSetDeviceDoRequest::PrivateKey(val) => fmt.field("PrivateKey", &FormatHex(val)),
                OpSetDeviceDoRequest::PublicKey(val) => fmt.field("PublicKey", &FormatHex(val)),
                OpSetDeviceDoRequest::Flags(val) => fmt.field("Flags", &val),
                OpSetDeviceDoRequest::ListenPort(val) => fmt.field("ListenPort", &val),
                OpSetDeviceDoRequest::Fwmark(val) => fmt.field("Fwmark", &val),
                OpSetDeviceDoRequest::Peers(val) => fmt.field("Peers", &val),
            };
        }
        fmt.finish()
    }
}
impl<'a> Iterable<'a, OpSetDeviceDoRequest<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpSetDeviceDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpSetDeviceDoRequest::attr_from_type(t)),
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
                OpSetDeviceDoRequest::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OpSetDeviceDoRequest::Ifname(val) => {
                    if last_off == offset {
                        stack.push(("Ifname", last_off));
                        break;
                    }
                }
                OpSetDeviceDoRequest::PrivateKey(val) => {
                    if last_off == offset {
                        stack.push(("PrivateKey", last_off));
                        break;
                    }
                }
                OpSetDeviceDoRequest::PublicKey(val) => {
                    if last_off == offset {
                        stack.push(("PublicKey", last_off));
                        break;
                    }
                }
                OpSetDeviceDoRequest::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                OpSetDeviceDoRequest::ListenPort(val) => {
                    if last_off == offset {
                        stack.push(("ListenPort", last_off));
                        break;
                    }
                }
                OpSetDeviceDoRequest::Fwmark(val) => {
                    if last_off == offset {
                        stack.push(("Fwmark", last_off));
                        break;
                    }
                }
                OpSetDeviceDoRequest::Peers(val) => {
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
            stack.push(("OpSetDeviceDoRequest", cur));
        }
        (stack, missing)
    }
}
#[doc = "Set WireGuard device.\n\nThis command should be called with a wgdevice set, containing one but\nnot both of WGDEVICE_A_IFINDEX and WGDEVICE_A_IFNAME.\n\nIt is possible that the amount of configuration data exceeds that of\nthe maximum message length accepted by the kernel. In that case,\nseveral messages should be sent one after another, with each\nsuccessive one filling in information not contained in the prior.\nNote that if WGDEVICE_F_REPLACE_PEERS is specified in the first\nmessage, it probably should not be specified in fragments that come\nafter, so that the list of peers is only cleared the first time but\nappended after.\nLikewise for peers, if WGPEER_F_REPLACE_ALLOWEDIPS is specified in\nthe first message of a peer, it likely should not be specified in\nsubsequent fragments.\n\nIf an error occurs, NLMSG_ERROR will reply containing an errno.\n"]
pub struct PushOpSetDeviceDoReply<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpSetDeviceDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpSetDeviceDoReply<Prev> {
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
impl<Prev: Rec> Drop for PushOpSetDeviceDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Set WireGuard device.\n\nThis command should be called with a wgdevice set, containing one but\nnot both of WGDEVICE_A_IFINDEX and WGDEVICE_A_IFNAME.\n\nIt is possible that the amount of configuration data exceeds that of\nthe maximum message length accepted by the kernel. In that case,\nseveral messages should be sent one after another, with each\nsuccessive one filling in information not contained in the prior.\nNote that if WGDEVICE_F_REPLACE_PEERS is specified in the first\nmessage, it probably should not be specified in fragments that come\nafter, so that the list of peers is only cleared the first time but\nappended after.\nLikewise for peers, if WGPEER_F_REPLACE_ALLOWEDIPS is specified in\nthe first message of a peer, it likely should not be specified in\nsubsequent fragments.\n\nIf an error occurs, NLMSG_ERROR will reply containing an errno.\n"]
#[doc = "Original name: \"op-set-device-do-reply\""]
#[derive(Clone)]
pub enum OpSetDeviceDoReply {}
impl<'a> Iterable<'a, OpSetDeviceDoReply> {}
impl OpSetDeviceDoReply {
    pub fn new(buf: &'_ [u8]) -> Iterable<'_, OpSetDeviceDoReply> {
        let mut header = PushBuiltinNfgenmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushBuiltinNfgenmsg::len()]);
        Iterable::with_loc(&buf[PushBuiltinNfgenmsg::len()..], buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Wgdevice::attr_from_type(r#type)
    }
}
impl Iterator for Iterable<'_, OpSetDeviceDoReply> {
    type Item = Result<OpSetDeviceDoReply, ErrorContext>;
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
            "OpSetDeviceDoReply",
            r#type.and_then(|t| OpSetDeviceDoReply::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl std::fmt::Debug for Iterable<'_, OpSetDeviceDoReply> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpSetDeviceDoReply");
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
impl Iterable<'_, OpSetDeviceDoReply> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset + PushBuiltinNfgenmsg::len() {
            stack.push(("OpSetDeviceDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpSetDeviceDoReply::attr_from_type(t)),
            );
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpSetDeviceDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpSetDeviceDoRequest<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        PushOpSetDeviceDoRequest::write_header(&mut request.buf_mut());
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpSetDeviceDoRequest<&mut Vec<u8>> {
        PushOpSetDeviceDoRequest::new_without_header(self.request.buf_mut())
    }
}
impl NetlinkRequest for RequestOpSetDeviceDoRequest<'_> {
    type ReplyType<'buf> = Iterable<'buf, OpSetDeviceDoReply>;
    fn protocol(&self) -> Protocol {
        Protocol::Generic("wireguard".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpSetDeviceDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpSetDeviceDoRequest::new(buf).lookup_attr(offset, missing_type)
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
    pub fn op_get_device_dump_request(self) -> RequestOpGetDeviceDumpRequest<'buf> {
        RequestOpGetDeviceDumpRequest::new(self)
    }
    pub fn op_set_device_do_request(self) -> RequestOpSetDeviceDoRequest<'buf> {
        RequestOpSetDeviceDoRequest::new(self)
    }
}
