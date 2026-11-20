#![doc = "GPRS Tunnelling Protocol, user plane (GTP-U).\n\nThe gtp netdevice encapsulates and decapsulates user plane packets in\nGTP-U tunnels (GTPv0 and GTPv1-U, see 3GPP TS 29.060 and TS 29.281).\nThis family manages the PDP contexts that describe the tunnels and\ntriggers GTP-U echo requests. It is driven by user space control planes\nsuch as those built on libgtpnl.\n\nThe kernel registers the family with version 0, which the\ngenetlink-legacy schema cannot express.\n\nThe notifications multicast on the \\\"gtp\\\" group are not described: PDP\ncontext additions and deletions, and GTP-U echo responses, reuse the\nNEWPDP, DELPDP and ECHOREQ command values of the requests.\n"]
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
    traits::{self, NetlinkRequest, Protocol},
    utils::*,
};
pub const PROTONAME: &str = "gtp";
pub const PROTONAME_CSTR: &CStr = c"gtp";
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Version {
    V0 = 0,
    V1 = 1,
}
impl Version {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::V0,
            1 => Self::V1,
            _ => return None,
        })
    }
}
#[derive(Clone)]
pub enum Gtp<'a> {
    #[doc = "ifindex of the gtp netdevice the context is attached to.\n"]
    Link(u32),
    #[doc = "GTP version of the context, GTPv0 or GTPv1-U.\n\nAssociated type: [`Version`] (enum)"]
    Version(u32),
    #[doc = "Tunnel identifier, GTPv0 only.\n"]
    Tid(u64),
    #[doc = "IPv4 address of the remote GSN peer (GGSN or SGSN). Also known as\nGTPA_SGSN_ADDRESS, kept for legacy user space.\n"]
    PeerAddress(std::net::Ipv4Addr),
    #[doc = "IPv4 address of the mobile subscriber served by the context.\n"]
    MsAddress(std::net::Ipv4Addr),
    #[doc = "Flow label, GTPv0 only.\n"]
    Flow(u16),
    #[doc = "File descriptor of the network namespace of the gtp netdevice.\n"]
    NetNsFd(u32),
    #[doc = "Ingress Tunnel Endpoint Identifier, GTPv1-U only.\n"]
    ITei(u32),
    #[doc = "Egress Tunnel Endpoint Identifier, GTPv1-U only.\n"]
    OTei(u32),
    Pad(&'a [u8]),
    #[doc = "IPv6 address of the remote GSN peer (GGSN or SGSN).\n"]
    PeerAddr6(std::net::Ipv6Addr),
    #[doc = "IPv6 prefix of the mobile subscriber served by the context: a /64\nprefix, with the lower 64 bits set to zero.\n"]
    MsAddr6(std::net::Ipv6Addr),
    #[doc = "Address family (AF_INET or AF_INET6) of ms-address or ms-addr6, AF_INET\nwhen absent. The peer address uses the family of the tunnel socket.\n"]
    Family(u8),
}
impl<'a> IterableGtp<'a> {
    #[doc = "ifindex of the gtp netdevice the context is attached to.\n"]
    pub fn get_link(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Gtp::Link(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Gtp",
            "Link",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "GTP version of the context, GTPv0 or GTPv1-U.\n\nAssociated type: [`Version`] (enum)"]
    pub fn get_version(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Gtp::Version(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Gtp",
            "Version",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Tunnel identifier, GTPv0 only.\n"]
    pub fn get_tid(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Gtp::Tid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Gtp",
            "Tid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "IPv4 address of the remote GSN peer (GGSN or SGSN). Also known as\nGTPA_SGSN_ADDRESS, kept for legacy user space.\n"]
    pub fn get_peer_address(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Gtp::PeerAddress(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Gtp",
            "PeerAddress",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "IPv4 address of the mobile subscriber served by the context.\n"]
    pub fn get_ms_address(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Gtp::MsAddress(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Gtp",
            "MsAddress",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Flow label, GTPv0 only.\n"]
    pub fn get_flow(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Gtp::Flow(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Gtp",
            "Flow",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "File descriptor of the network namespace of the gtp netdevice.\n"]
    pub fn get_net_ns_fd(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Gtp::NetNsFd(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Gtp",
            "NetNsFd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Ingress Tunnel Endpoint Identifier, GTPv1-U only.\n"]
    pub fn get_i_tei(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Gtp::ITei(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Gtp",
            "ITei",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Egress Tunnel Endpoint Identifier, GTPv1-U only.\n"]
    pub fn get_o_tei(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Gtp::OTei(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Gtp",
            "OTei",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Gtp::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Gtp",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "IPv6 address of the remote GSN peer (GGSN or SGSN).\n"]
    pub fn get_peer_addr6(&self) -> Result<std::net::Ipv6Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Gtp::PeerAddr6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Gtp",
            "PeerAddr6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "IPv6 prefix of the mobile subscriber served by the context: a /64\nprefix, with the lower 64 bits set to zero.\n"]
    pub fn get_ms_addr6(&self) -> Result<std::net::Ipv6Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Gtp::MsAddr6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Gtp",
            "MsAddr6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Address family (AF_INET or AF_INET6) of ms-address or ms-addr6, AF_INET\nwhen absent. The peer address uses the family of the tunnel socket.\n"]
    pub fn get_family(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Gtp::Family(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Gtp",
            "Family",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Gtp<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableGtp<'a> {
        IterableGtp::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Link",
            2u16 => "Version",
            3u16 => "Tid",
            4u16 => "PeerAddress",
            5u16 => "MsAddress",
            6u16 => "Flow",
            7u16 => "NetNsFd",
            8u16 => "ITei",
            9u16 => "OTei",
            10u16 => "Pad",
            11u16 => "PeerAddr6",
            12u16 => "MsAddr6",
            13u16 => "Family",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableGtp<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableGtp<'a> {
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
impl<'a> Iterator for IterableGtp<'a> {
    type Item = Result<Gtp<'a>, ErrorContext>;
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
                1u16 => Gtp::Link({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Gtp::Version({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Gtp::Tid({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Gtp::PeerAddress({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Gtp::MsAddress({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Gtp::Flow({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => Gtp::NetNsFd({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => Gtp::ITei({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => Gtp::OTei({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => Gtp::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => Gtp::PeerAddr6({
                    let res = parse_ipv6(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => Gtp::MsAddr6({
                    let res = parse_ipv6(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => Gtp::Family({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Gtp",
            r#type.and_then(|t| Gtp::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableGtp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Gtp");
        let mut iter = IterableGtp::with_loc(&[], self.orig_loc);
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
                Gtp::Link(val) => fmt.field("Link", &val),
                Gtp::Version(val) => {
                    fmt.field("Version", &FormatEnum(val.into(), Version::from_value))
                }
                Gtp::Tid(val) => fmt.field("Tid", &val),
                Gtp::PeerAddress(val) => fmt.field("PeerAddress", &val),
                Gtp::MsAddress(val) => fmt.field("MsAddress", &val),
                Gtp::Flow(val) => fmt.field("Flow", &val),
                Gtp::NetNsFd(val) => fmt.field("NetNsFd", &val),
                Gtp::ITei(val) => fmt.field("ITei", &val),
                Gtp::OTei(val) => fmt.field("OTei", &val),
                Gtp::Pad(val) => fmt.field("Pad", &val),
                Gtp::PeerAddr6(val) => fmt.field("PeerAddr6", &val),
                Gtp::MsAddr6(val) => fmt.field("MsAddr6", &val),
                Gtp::Family(val) => fmt.field("Family", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableGtp<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Gtp", offset));
            return (stack, missing_type.and_then(|t| Gtp::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Gtp::Link(val) => {
                    if last_off == offset {
                        stack.push(("Link", last_off));
                        break;
                    }
                }
                Gtp::Version(val) => {
                    if last_off == offset {
                        stack.push(("Version", last_off));
                        break;
                    }
                }
                Gtp::Tid(val) => {
                    if last_off == offset {
                        stack.push(("Tid", last_off));
                        break;
                    }
                }
                Gtp::PeerAddress(val) => {
                    if last_off == offset {
                        stack.push(("PeerAddress", last_off));
                        break;
                    }
                }
                Gtp::MsAddress(val) => {
                    if last_off == offset {
                        stack.push(("MsAddress", last_off));
                        break;
                    }
                }
                Gtp::Flow(val) => {
                    if last_off == offset {
                        stack.push(("Flow", last_off));
                        break;
                    }
                }
                Gtp::NetNsFd(val) => {
                    if last_off == offset {
                        stack.push(("NetNsFd", last_off));
                        break;
                    }
                }
                Gtp::ITei(val) => {
                    if last_off == offset {
                        stack.push(("ITei", last_off));
                        break;
                    }
                }
                Gtp::OTei(val) => {
                    if last_off == offset {
                        stack.push(("OTei", last_off));
                        break;
                    }
                }
                Gtp::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                Gtp::PeerAddr6(val) => {
                    if last_off == offset {
                        stack.push(("PeerAddr6", last_off));
                        break;
                    }
                }
                Gtp::MsAddr6(val) => {
                    if last_off == offset {
                        stack.push(("MsAddr6", last_off));
                        break;
                    }
                }
                Gtp::Family(val) => {
                    if last_off == offset {
                        stack.push(("Family", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Gtp", cur));
        }
        (stack, None)
    }
}
pub struct PushGtp<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushGtp<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushGtp<Prev> {
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
    #[doc = "ifindex of the gtp netdevice the context is attached to.\n"]
    pub fn push_link(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "GTP version of the context, GTPv0 or GTPv1-U.\n\nAssociated type: [`Version`] (enum)"]
    pub fn push_version(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Tunnel identifier, GTPv0 only.\n"]
    pub fn push_tid(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 3u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "IPv4 address of the remote GSN peer (GGSN or SGSN). Also known as\nGTPA_SGSN_ADDRESS, kept for legacy user space.\n"]
    pub fn push_peer_address(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    #[doc = "IPv4 address of the mobile subscriber served by the context.\n"]
    pub fn push_ms_address(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    #[doc = "Flow label, GTPv0 only.\n"]
    pub fn push_flow(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 6u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "File descriptor of the network namespace of the gtp netdevice.\n"]
    pub fn push_net_ns_fd(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Ingress Tunnel Endpoint Identifier, GTPv1-U only.\n"]
    pub fn push_i_tei(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Egress Tunnel Endpoint Identifier, GTPv1-U only.\n"]
    pub fn push_o_tei(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 10u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "IPv6 address of the remote GSN peer (GGSN or SGSN).\n"]
    pub fn push_peer_addr6(mut self, value: std::net::Ipv6Addr) -> Self {
        push_header(self.as_vec_mut(), 11u16, 16 as u16);
        encode_ipv6(self.as_vec_mut(), value);
        self
    }
    #[doc = "IPv6 prefix of the mobile subscriber served by the context: a /64\nprefix, with the lower 64 bits set to zero.\n"]
    pub fn push_ms_addr6(mut self, value: std::net::Ipv6Addr) -> Self {
        push_header(self.as_vec_mut(), 12u16, 16 as u16);
        encode_ipv6(self.as_vec_mut(), value);
        self
    }
    #[doc = "Address family (AF_INET or AF_INET6) of ms-address or ms-addr6, AF_INET\nwhen absent. The peer address uses the family of the tunnel socket.\n"]
    pub fn push_family(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 13u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushGtp<Prev> {
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
    #[doc = "Flags: uns-admin-perm\n\n"]
    pub const GTP: &str = "gtp";
    #[doc = "Flags: uns-admin-perm\n\n"]
    pub const GTP_CSTR: &CStr = c"gtp";
}
#[doc = "Create or update a PDP context.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_link()](PushGtp::push_link)\n- [.push_version()](PushGtp::push_version)\n- [.push_tid()](PushGtp::push_tid)\n- [.push_peer_address()](PushGtp::push_peer_address)\n- [.push_ms_address()](PushGtp::push_ms_address)\n- [.push_flow()](PushGtp::push_flow)\n- [.push_net_ns_fd()](PushGtp::push_net_ns_fd)\n- [.push_i_tei()](PushGtp::push_i_tei)\n- [.push_o_tei()](PushGtp::push_o_tei)\n- [.push_peer_addr6()](PushGtp::push_peer_addr6)\n- [.push_ms_addr6()](PushGtp::push_ms_addr6)\n- [.push_family()](PushGtp::push_family)\n\n"]
#[derive(Debug)]
pub struct OpNewpdpDo<'r> {
    request: Request<'r>,
}
impl<'r> OpNewpdpDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushGtp<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushGtp::new(buf)
    }
    pub fn encode(&mut self) -> PushGtp<&mut Vec<u8>> {
        PushGtp::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushGtp<RequestBuf<'r>> {
        PushGtp::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableGtp<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableGtp::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpNewpdpDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("gtp".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableGtp<'buf>;
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
#[doc = "Delete a PDP context.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_link()](PushGtp::push_link)\n- [.push_version()](PushGtp::push_version)\n- [.push_tid()](PushGtp::push_tid)\n- [.push_ms_address()](PushGtp::push_ms_address)\n- [.push_net_ns_fd()](PushGtp::push_net_ns_fd)\n- [.push_i_tei()](PushGtp::push_i_tei)\n- [.push_ms_addr6()](PushGtp::push_ms_addr6)\n- [.push_family()](PushGtp::push_family)\n\n"]
#[derive(Debug)]
pub struct OpDelpdpDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDelpdpDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushGtp<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushGtp::new(buf)
    }
    pub fn encode(&mut self) -> PushGtp<&mut Vec<u8>> {
        PushGtp::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushGtp<RequestBuf<'r>> {
        PushGtp::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableGtp<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableGtp::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpDelpdpDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("gtp".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableGtp<'buf>;
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
#[doc = "Get or dump one or more PDP contexts.\n\nFlags: admin-perm\n\nReply attributes:\n- [.get_link()](IterableGtp::get_link)\n- [.get_version()](IterableGtp::get_version)\n- [.get_tid()](IterableGtp::get_tid)\n- [.get_peer_address()](IterableGtp::get_peer_address)\n- [.get_ms_address()](IterableGtp::get_ms_address)\n- [.get_flow()](IterableGtp::get_flow)\n- [.get_i_tei()](IterableGtp::get_i_tei)\n- [.get_o_tei()](IterableGtp::get_o_tei)\n- [.get_peer_addr6()](IterableGtp::get_peer_addr6)\n- [.get_ms_addr6()](IterableGtp::get_ms_addr6)\n- [.get_family()](IterableGtp::get_family)\n\n"]
#[derive(Debug)]
pub struct OpGetpdpDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetpdpDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushGtp<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushGtp::new(buf)
    }
    pub fn encode(&mut self) -> PushGtp<&mut Vec<u8>> {
        PushGtp::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushGtp<RequestBuf<'r>> {
        PushGtp::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableGtp<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableGtp::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpGetpdpDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("gtp".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableGtp<'buf>;
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
#[doc = "Get or dump one or more PDP contexts.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_link()](PushGtp::push_link)\n- [.push_version()](PushGtp::push_version)\n- [.push_tid()](PushGtp::push_tid)\n- [.push_ms_address()](PushGtp::push_ms_address)\n- [.push_net_ns_fd()](PushGtp::push_net_ns_fd)\n- [.push_i_tei()](PushGtp::push_i_tei)\n- [.push_ms_addr6()](PushGtp::push_ms_addr6)\n- [.push_family()](PushGtp::push_family)\n\nReply attributes:\n- [.get_link()](IterableGtp::get_link)\n- [.get_version()](IterableGtp::get_version)\n- [.get_tid()](IterableGtp::get_tid)\n- [.get_peer_address()](IterableGtp::get_peer_address)\n- [.get_ms_address()](IterableGtp::get_ms_address)\n- [.get_flow()](IterableGtp::get_flow)\n- [.get_i_tei()](IterableGtp::get_i_tei)\n- [.get_o_tei()](IterableGtp::get_o_tei)\n- [.get_peer_addr6()](IterableGtp::get_peer_addr6)\n- [.get_ms_addr6()](IterableGtp::get_ms_addr6)\n- [.get_family()](IterableGtp::get_family)\n\n"]
#[derive(Debug)]
pub struct OpGetpdpDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetpdpDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushGtp<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushGtp::new(buf)
    }
    pub fn encode(&mut self) -> PushGtp<&mut Vec<u8>> {
        PushGtp::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushGtp<RequestBuf<'r>> {
        PushGtp::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableGtp<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableGtp::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpGetpdpDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("gtp".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableGtp<'buf>;
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
#[doc = "Send a GTP-U echo request to a peer. ms-address is not a mobile\nsubscriber address here, but the local IPv4 address used as source of\nthe request.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_link()](PushGtp::push_link)\n- [.push_version()](PushGtp::push_version)\n- [.push_peer_address()](PushGtp::push_peer_address)\n- [.push_ms_address()](PushGtp::push_ms_address)\n- [.push_net_ns_fd()](PushGtp::push_net_ns_fd)\n\n"]
#[derive(Debug)]
pub struct OpEchoreqDo<'r> {
    request: Request<'r>,
}
impl<'r> OpEchoreqDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushGtp<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushGtp::new(buf)
    }
    pub fn encode(&mut self) -> PushGtp<&mut Vec<u8>> {
        PushGtp::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushGtp<RequestBuf<'r>> {
        PushGtp::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableGtp<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableGtp::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpEchoreqDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("gtp".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableGtp<'buf>;
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
    #[doc = "Create or update a PDP context.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_link()](PushGtp::push_link)\n- [.push_version()](PushGtp::push_version)\n- [.push_tid()](PushGtp::push_tid)\n- [.push_peer_address()](PushGtp::push_peer_address)\n- [.push_ms_address()](PushGtp::push_ms_address)\n- [.push_flow()](PushGtp::push_flow)\n- [.push_net_ns_fd()](PushGtp::push_net_ns_fd)\n- [.push_i_tei()](PushGtp::push_i_tei)\n- [.push_o_tei()](PushGtp::push_o_tei)\n- [.push_peer_addr6()](PushGtp::push_peer_addr6)\n- [.push_ms_addr6()](PushGtp::push_ms_addr6)\n- [.push_family()](PushGtp::push_family)\n\n"]
    pub fn op_newpdp_do(self) -> OpNewpdpDo<'buf> {
        let mut res = OpNewpdpDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-newpdp-do", OpNewpdpDo::lookup);
        res
    }
    #[doc = "Delete a PDP context.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_link()](PushGtp::push_link)\n- [.push_version()](PushGtp::push_version)\n- [.push_tid()](PushGtp::push_tid)\n- [.push_ms_address()](PushGtp::push_ms_address)\n- [.push_net_ns_fd()](PushGtp::push_net_ns_fd)\n- [.push_i_tei()](PushGtp::push_i_tei)\n- [.push_ms_addr6()](PushGtp::push_ms_addr6)\n- [.push_family()](PushGtp::push_family)\n\n"]
    pub fn op_delpdp_do(self) -> OpDelpdpDo<'buf> {
        let mut res = OpDelpdpDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-delpdp-do", OpDelpdpDo::lookup);
        res
    }
    #[doc = "Get or dump one or more PDP contexts.\n\nFlags: admin-perm\n\nReply attributes:\n- [.get_link()](IterableGtp::get_link)\n- [.get_version()](IterableGtp::get_version)\n- [.get_tid()](IterableGtp::get_tid)\n- [.get_peer_address()](IterableGtp::get_peer_address)\n- [.get_ms_address()](IterableGtp::get_ms_address)\n- [.get_flow()](IterableGtp::get_flow)\n- [.get_i_tei()](IterableGtp::get_i_tei)\n- [.get_o_tei()](IterableGtp::get_o_tei)\n- [.get_peer_addr6()](IterableGtp::get_peer_addr6)\n- [.get_ms_addr6()](IterableGtp::get_ms_addr6)\n- [.get_family()](IterableGtp::get_family)\n\n"]
    pub fn op_getpdp_dump(self) -> OpGetpdpDump<'buf> {
        let mut res = OpGetpdpDump::new(self);
        res.request
            .do_writeback(res.protocol(), "op-getpdp-dump", OpGetpdpDump::lookup);
        res
    }
    #[doc = "Get or dump one or more PDP contexts.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_link()](PushGtp::push_link)\n- [.push_version()](PushGtp::push_version)\n- [.push_tid()](PushGtp::push_tid)\n- [.push_ms_address()](PushGtp::push_ms_address)\n- [.push_net_ns_fd()](PushGtp::push_net_ns_fd)\n- [.push_i_tei()](PushGtp::push_i_tei)\n- [.push_ms_addr6()](PushGtp::push_ms_addr6)\n- [.push_family()](PushGtp::push_family)\n\nReply attributes:\n- [.get_link()](IterableGtp::get_link)\n- [.get_version()](IterableGtp::get_version)\n- [.get_tid()](IterableGtp::get_tid)\n- [.get_peer_address()](IterableGtp::get_peer_address)\n- [.get_ms_address()](IterableGtp::get_ms_address)\n- [.get_flow()](IterableGtp::get_flow)\n- [.get_i_tei()](IterableGtp::get_i_tei)\n- [.get_o_tei()](IterableGtp::get_o_tei)\n- [.get_peer_addr6()](IterableGtp::get_peer_addr6)\n- [.get_ms_addr6()](IterableGtp::get_ms_addr6)\n- [.get_family()](IterableGtp::get_family)\n\n"]
    pub fn op_getpdp_do(self) -> OpGetpdpDo<'buf> {
        let mut res = OpGetpdpDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-getpdp-do", OpGetpdpDo::lookup);
        res
    }
    #[doc = "Send a GTP-U echo request to a peer. ms-address is not a mobile\nsubscriber address here, but the local IPv4 address used as source of\nthe request.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_link()](PushGtp::push_link)\n- [.push_version()](PushGtp::push_version)\n- [.push_peer_address()](PushGtp::push_peer_address)\n- [.push_ms_address()](PushGtp::push_ms_address)\n- [.push_net_ns_fd()](PushGtp::push_net_ns_fd)\n\n"]
    pub fn op_echoreq_do(self) -> OpEchoreqDo<'buf> {
        let mut res = OpEchoreqDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-echoreq-do", OpEchoreqDo::lookup);
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableGtp::get_family;
        let _ = IterableGtp::get_flow;
        let _ = IterableGtp::get_i_tei;
        let _ = IterableGtp::get_link;
        let _ = IterableGtp::get_ms_addr6;
        let _ = IterableGtp::get_ms_address;
        let _ = IterableGtp::get_o_tei;
        let _ = IterableGtp::get_peer_addr6;
        let _ = IterableGtp::get_peer_address;
        let _ = IterableGtp::get_tid;
        let _ = IterableGtp::get_version;
        let _ = PushGtp::<&mut Vec<u8>>::push_family;
        let _ = PushGtp::<&mut Vec<u8>>::push_flow;
        let _ = PushGtp::<&mut Vec<u8>>::push_i_tei;
        let _ = PushGtp::<&mut Vec<u8>>::push_link;
        let _ = PushGtp::<&mut Vec<u8>>::push_ms_addr6;
        let _ = PushGtp::<&mut Vec<u8>>::push_ms_address;
        let _ = PushGtp::<&mut Vec<u8>>::push_net_ns_fd;
        let _ = PushGtp::<&mut Vec<u8>>::push_o_tei;
        let _ = PushGtp::<&mut Vec<u8>>::push_peer_addr6;
        let _ = PushGtp::<&mut Vec<u8>>::push_peer_address;
        let _ = PushGtp::<&mut Vec<u8>>::push_tid;
        let _ = PushGtp::<&mut Vec<u8>>::push_version;
    }
}
