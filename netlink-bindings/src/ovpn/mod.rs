#![doc = "Netlink protocol to control OpenVPN network devices\n"]
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
pub const PROTONAME: &str = "ovpn";
pub const PROTONAME_CSTR: &CStr = c"ovpn";
pub const NONCE_TAIL_SIZE: u64 = 8u64;
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum CipherAlg {
    None = 0,
    AesGcm = 1,
    Chacha20Poly1305 = 2,
}
impl CipherAlg {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::None,
            1 => Self::AesGcm,
            2 => Self::Chacha20Poly1305,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum DelPeerReason {
    Teardown = 0,
    Userspace = 1,
    Expired = 2,
    TransportError = 3,
    TransportDisconnect = 4,
}
impl DelPeerReason {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Teardown,
            1 => Self::Userspace,
            2 => Self::Expired,
            3 => Self::TransportError,
            4 => Self::TransportDisconnect,
            _ => return None,
        })
    }
}
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum KeySlot {
    Primary = 0,
    Secondary = 1,
}
impl KeySlot {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Primary,
            1 => Self::Secondary,
            _ => return None,
        })
    }
}
#[derive(Clone)]
pub enum Peer<'a> {
    #[doc = "The unique ID of the peer in the device context. To be used to identify\npeers during operations for a specific device. Also used to match\npackets received from this peer.\n"]
    Id(u32),
    #[doc = "The remote IPv4 address of the peer\n"]
    RemoteIpv4(std::net::Ipv4Addr),
    #[doc = "The remote IPv6 address of the peer\n"]
    RemoteIpv6(&'a [u8]),
    #[doc = "The scope id of the remote IPv6 address of the peer (RFC2553)\n"]
    RemoteIpv6ScopeId(u32),
    #[doc = "The remote port of the peer\n"]
    RemotePort(u16),
    #[doc = "The socket to be used to communicate with the peer\n"]
    Socket(u32),
    #[doc = "The ID of the netns the socket assigned to this peer lives in\n"]
    SocketNetnsid(i32),
    #[doc = "The IPv4 address assigned to the peer by the server\n"]
    VpnIpv4(std::net::Ipv4Addr),
    #[doc = "The IPv6 address assigned to the peer by the server\n"]
    VpnIpv6(&'a [u8]),
    #[doc = "The local IPv4 to be used to send packets to the peer (UDP only)\n"]
    LocalIpv4(std::net::Ipv4Addr),
    #[doc = "The local IPv6 to be used to send packets to the peer (UDP only)\n"]
    LocalIpv6(&'a [u8]),
    #[doc = "The local port to be used to send packets to the peer (UDP only)\n"]
    LocalPort(u16),
    #[doc = "The number of seconds after which a keep alive message is sent to the\npeer\n"]
    KeepaliveInterval(u32),
    #[doc = "The number of seconds from the last activity after which the peer is\nassumed dead\n"]
    KeepaliveTimeout(u32),
    #[doc = "The reason why a peer was deleted\n\nAssociated type: [`DelPeerReason`] (enum)"]
    DelReason(u32),
    #[doc = "Number of bytes received over the tunnel\n"]
    VpnRxBytes(u32),
    #[doc = "Number of bytes transmitted over the tunnel\n"]
    VpnTxBytes(u32),
    #[doc = "Number of packets received over the tunnel\n"]
    VpnRxPackets(u32),
    #[doc = "Number of packets transmitted over the tunnel\n"]
    VpnTxPackets(u32),
    #[doc = "Number of bytes received at the transport level\n"]
    LinkRxBytes(u32),
    #[doc = "Number of bytes transmitted at the transport level\n"]
    LinkTxBytes(u32),
    #[doc = "Number of packets received at the transport level\n"]
    LinkRxPackets(u32),
    #[doc = "Number of packets transmitted at the transport level\n"]
    LinkTxPackets(u32),
    #[doc = "The ID value used when transmitting packets to this peer. This way\noutgoing packets can have a different ID than incoming ones. Useful in\nmultipeer-to-multipeer connections, where each peer will advertise the\ntx-id to be used on the link.\n"]
    TxId(u32),
}
impl<'a> IterablePeer<'a> {
    #[doc = "The unique ID of the peer in the device context. To be used to identify\npeers during operations for a specific device. Also used to match\npackets received from this peer.\n"]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Peer::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Peer",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The remote IPv4 address of the peer\n"]
    pub fn get_remote_ipv4(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Peer::RemoteIpv4(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Peer",
            "RemoteIpv4",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The remote IPv6 address of the peer\n"]
    pub fn get_remote_ipv6(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Peer::RemoteIpv6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Peer",
            "RemoteIpv6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The scope id of the remote IPv6 address of the peer (RFC2553)\n"]
    pub fn get_remote_ipv6_scope_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Peer::RemoteIpv6ScopeId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Peer",
            "RemoteIpv6ScopeId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The remote port of the peer\n"]
    pub fn get_remote_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Peer::RemotePort(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Peer",
            "RemotePort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The socket to be used to communicate with the peer\n"]
    pub fn get_socket(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Peer::Socket(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Peer",
            "Socket",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The ID of the netns the socket assigned to this peer lives in\n"]
    pub fn get_socket_netnsid(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Peer::SocketNetnsid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Peer",
            "SocketNetnsid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The IPv4 address assigned to the peer by the server\n"]
    pub fn get_vpn_ipv4(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Peer::VpnIpv4(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Peer",
            "VpnIpv4",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The IPv6 address assigned to the peer by the server\n"]
    pub fn get_vpn_ipv6(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Peer::VpnIpv6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Peer",
            "VpnIpv6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The local IPv4 to be used to send packets to the peer (UDP only)\n"]
    pub fn get_local_ipv4(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Peer::LocalIpv4(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Peer",
            "LocalIpv4",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The local IPv6 to be used to send packets to the peer (UDP only)\n"]
    pub fn get_local_ipv6(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Peer::LocalIpv6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Peer",
            "LocalIpv6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The local port to be used to send packets to the peer (UDP only)\n"]
    pub fn get_local_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Peer::LocalPort(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Peer",
            "LocalPort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The number of seconds after which a keep alive message is sent to the\npeer\n"]
    pub fn get_keepalive_interval(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Peer::KeepaliveInterval(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Peer",
            "KeepaliveInterval",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The number of seconds from the last activity after which the peer is\nassumed dead\n"]
    pub fn get_keepalive_timeout(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Peer::KeepaliveTimeout(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Peer",
            "KeepaliveTimeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The reason why a peer was deleted\n\nAssociated type: [`DelPeerReason`] (enum)"]
    pub fn get_del_reason(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Peer::DelReason(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Peer",
            "DelReason",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of bytes received over the tunnel\n"]
    pub fn get_vpn_rx_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Peer::VpnRxBytes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Peer",
            "VpnRxBytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of bytes transmitted over the tunnel\n"]
    pub fn get_vpn_tx_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Peer::VpnTxBytes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Peer",
            "VpnTxBytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of packets received over the tunnel\n"]
    pub fn get_vpn_rx_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Peer::VpnRxPackets(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Peer",
            "VpnRxPackets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of packets transmitted over the tunnel\n"]
    pub fn get_vpn_tx_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Peer::VpnTxPackets(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Peer",
            "VpnTxPackets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of bytes received at the transport level\n"]
    pub fn get_link_rx_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Peer::LinkRxBytes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Peer",
            "LinkRxBytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of bytes transmitted at the transport level\n"]
    pub fn get_link_tx_bytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Peer::LinkTxBytes(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Peer",
            "LinkTxBytes",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of packets received at the transport level\n"]
    pub fn get_link_rx_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Peer::LinkRxPackets(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Peer",
            "LinkRxPackets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Number of packets transmitted at the transport level\n"]
    pub fn get_link_tx_packets(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Peer::LinkTxPackets(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Peer",
            "LinkTxPackets",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The ID value used when transmitting packets to this peer. This way\noutgoing packets can have a different ID than incoming ones. Useful in\nmultipeer-to-multipeer connections, where each peer will advertise the\ntx-id to be used on the link.\n"]
    pub fn get_tx_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Peer::TxId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Peer",
            "TxId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Peer<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePeer<'a> {
        IterablePeer::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Id",
            2u16 => "RemoteIpv4",
            3u16 => "RemoteIpv6",
            4u16 => "RemoteIpv6ScopeId",
            5u16 => "RemotePort",
            6u16 => "Socket",
            7u16 => "SocketNetnsid",
            8u16 => "VpnIpv4",
            9u16 => "VpnIpv6",
            10u16 => "LocalIpv4",
            11u16 => "LocalIpv6",
            12u16 => "LocalPort",
            13u16 => "KeepaliveInterval",
            14u16 => "KeepaliveTimeout",
            15u16 => "DelReason",
            16u16 => "VpnRxBytes",
            17u16 => "VpnTxBytes",
            18u16 => "VpnRxPackets",
            19u16 => "VpnTxPackets",
            20u16 => "LinkRxBytes",
            21u16 => "LinkTxBytes",
            22u16 => "LinkRxPackets",
            23u16 => "LinkTxPackets",
            24u16 => "TxId",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePeer<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePeer<'a> {
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
impl<'a> Iterator for IterablePeer<'a> {
    type Item = Result<Peer<'a>, ErrorContext>;
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
                1u16 => Peer::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Peer::RemoteIpv4({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Peer::RemoteIpv6({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Peer::RemoteIpv6ScopeId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Peer::RemotePort({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Peer::Socket({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => Peer::SocketNetnsid({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => Peer::VpnIpv4({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => Peer::VpnIpv6({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => Peer::LocalIpv4({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => Peer::LocalIpv6({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => Peer::LocalPort({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => Peer::KeepaliveInterval({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => Peer::KeepaliveTimeout({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => Peer::DelReason({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => Peer::VpnRxBytes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => Peer::VpnTxBytes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => Peer::VpnRxPackets({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => Peer::VpnTxPackets({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                20u16 => Peer::LinkRxBytes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                21u16 => Peer::LinkTxBytes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                22u16 => Peer::LinkRxPackets({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                23u16 => Peer::LinkTxPackets({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => Peer::TxId({
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
            "Peer",
            r#type.and_then(|t| Peer::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterablePeer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Peer");
        let mut iter = IterablePeer::with_loc(&[], self.orig_loc);
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
                Peer::Id(val) => fmt.field("Id", &val),
                Peer::RemoteIpv4(val) => fmt.field("RemoteIpv4", &val),
                Peer::RemoteIpv6(val) => fmt.field("RemoteIpv6", &val),
                Peer::RemoteIpv6ScopeId(val) => fmt.field("RemoteIpv6ScopeId", &val),
                Peer::RemotePort(val) => fmt.field("RemotePort", &val),
                Peer::Socket(val) => fmt.field("Socket", &val),
                Peer::SocketNetnsid(val) => fmt.field("SocketNetnsid", &val),
                Peer::VpnIpv4(val) => fmt.field("VpnIpv4", &val),
                Peer::VpnIpv6(val) => fmt.field("VpnIpv6", &val),
                Peer::LocalIpv4(val) => fmt.field("LocalIpv4", &val),
                Peer::LocalIpv6(val) => fmt.field("LocalIpv6", &val),
                Peer::LocalPort(val) => fmt.field("LocalPort", &val),
                Peer::KeepaliveInterval(val) => fmt.field("KeepaliveInterval", &val),
                Peer::KeepaliveTimeout(val) => fmt.field("KeepaliveTimeout", &val),
                Peer::DelReason(val) => fmt.field(
                    "DelReason",
                    &FormatEnum(val.into(), DelPeerReason::from_value),
                ),
                Peer::VpnRxBytes(val) => fmt.field("VpnRxBytes", &val),
                Peer::VpnTxBytes(val) => fmt.field("VpnTxBytes", &val),
                Peer::VpnRxPackets(val) => fmt.field("VpnRxPackets", &val),
                Peer::VpnTxPackets(val) => fmt.field("VpnTxPackets", &val),
                Peer::LinkRxBytes(val) => fmt.field("LinkRxBytes", &val),
                Peer::LinkTxBytes(val) => fmt.field("LinkTxBytes", &val),
                Peer::LinkRxPackets(val) => fmt.field("LinkRxPackets", &val),
                Peer::LinkTxPackets(val) => fmt.field("LinkTxPackets", &val),
                Peer::TxId(val) => fmt.field("TxId", &val),
            };
        }
        fmt.finish()
    }
}
impl IterablePeer<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Peer", offset));
            return (stack, missing_type.and_then(|t| Peer::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Peer::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                Peer::RemoteIpv4(val) => {
                    if last_off == offset {
                        stack.push(("RemoteIpv4", last_off));
                        break;
                    }
                }
                Peer::RemoteIpv6(val) => {
                    if last_off == offset {
                        stack.push(("RemoteIpv6", last_off));
                        break;
                    }
                }
                Peer::RemoteIpv6ScopeId(val) => {
                    if last_off == offset {
                        stack.push(("RemoteIpv6ScopeId", last_off));
                        break;
                    }
                }
                Peer::RemotePort(val) => {
                    if last_off == offset {
                        stack.push(("RemotePort", last_off));
                        break;
                    }
                }
                Peer::Socket(val) => {
                    if last_off == offset {
                        stack.push(("Socket", last_off));
                        break;
                    }
                }
                Peer::SocketNetnsid(val) => {
                    if last_off == offset {
                        stack.push(("SocketNetnsid", last_off));
                        break;
                    }
                }
                Peer::VpnIpv4(val) => {
                    if last_off == offset {
                        stack.push(("VpnIpv4", last_off));
                        break;
                    }
                }
                Peer::VpnIpv6(val) => {
                    if last_off == offset {
                        stack.push(("VpnIpv6", last_off));
                        break;
                    }
                }
                Peer::LocalIpv4(val) => {
                    if last_off == offset {
                        stack.push(("LocalIpv4", last_off));
                        break;
                    }
                }
                Peer::LocalIpv6(val) => {
                    if last_off == offset {
                        stack.push(("LocalIpv6", last_off));
                        break;
                    }
                }
                Peer::LocalPort(val) => {
                    if last_off == offset {
                        stack.push(("LocalPort", last_off));
                        break;
                    }
                }
                Peer::KeepaliveInterval(val) => {
                    if last_off == offset {
                        stack.push(("KeepaliveInterval", last_off));
                        break;
                    }
                }
                Peer::KeepaliveTimeout(val) => {
                    if last_off == offset {
                        stack.push(("KeepaliveTimeout", last_off));
                        break;
                    }
                }
                Peer::DelReason(val) => {
                    if last_off == offset {
                        stack.push(("DelReason", last_off));
                        break;
                    }
                }
                Peer::VpnRxBytes(val) => {
                    if last_off == offset {
                        stack.push(("VpnRxBytes", last_off));
                        break;
                    }
                }
                Peer::VpnTxBytes(val) => {
                    if last_off == offset {
                        stack.push(("VpnTxBytes", last_off));
                        break;
                    }
                }
                Peer::VpnRxPackets(val) => {
                    if last_off == offset {
                        stack.push(("VpnRxPackets", last_off));
                        break;
                    }
                }
                Peer::VpnTxPackets(val) => {
                    if last_off == offset {
                        stack.push(("VpnTxPackets", last_off));
                        break;
                    }
                }
                Peer::LinkRxBytes(val) => {
                    if last_off == offset {
                        stack.push(("LinkRxBytes", last_off));
                        break;
                    }
                }
                Peer::LinkTxBytes(val) => {
                    if last_off == offset {
                        stack.push(("LinkTxBytes", last_off));
                        break;
                    }
                }
                Peer::LinkRxPackets(val) => {
                    if last_off == offset {
                        stack.push(("LinkRxPackets", last_off));
                        break;
                    }
                }
                Peer::LinkTxPackets(val) => {
                    if last_off == offset {
                        stack.push(("LinkTxPackets", last_off));
                        break;
                    }
                }
                Peer::TxId(val) => {
                    if last_off == offset {
                        stack.push(("TxId", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Peer", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum PeerNewInput<'a> {
    #[doc = "The unique ID of the peer in the device context. To be used to identify\npeers during operations for a specific device. Also used to match\npackets received from this peer.\n"]
    Id(u32),
    #[doc = "The remote IPv4 address of the peer\n"]
    RemoteIpv4(std::net::Ipv4Addr),
    #[doc = "The remote IPv6 address of the peer\n"]
    RemoteIpv6(&'a [u8]),
    #[doc = "The scope id of the remote IPv6 address of the peer (RFC2553)\n"]
    RemoteIpv6ScopeId(u32),
    #[doc = "The remote port of the peer\n"]
    RemotePort(u16),
    #[doc = "The socket to be used to communicate with the peer\n"]
    Socket(u32),
    #[doc = "The IPv4 address assigned to the peer by the server\n"]
    VpnIpv4(std::net::Ipv4Addr),
    #[doc = "The IPv6 address assigned to the peer by the server\n"]
    VpnIpv6(&'a [u8]),
    #[doc = "The local IPv4 to be used to send packets to the peer (UDP only)\n"]
    LocalIpv4(std::net::Ipv4Addr),
    #[doc = "The local IPv6 to be used to send packets to the peer (UDP only)\n"]
    LocalIpv6(&'a [u8]),
    #[doc = "The number of seconds after which a keep alive message is sent to the\npeer\n"]
    KeepaliveInterval(u32),
    #[doc = "The number of seconds from the last activity after which the peer is\nassumed dead\n"]
    KeepaliveTimeout(u32),
    #[doc = "The ID value used when transmitting packets to this peer. This way\noutgoing packets can have a different ID than incoming ones. Useful in\nmultipeer-to-multipeer connections, where each peer will advertise the\ntx-id to be used on the link.\n"]
    TxId(u32),
}
impl<'a> IterablePeerNewInput<'a> {
    #[doc = "The unique ID of the peer in the device context. To be used to identify\npeers during operations for a specific device. Also used to match\npackets received from this peer.\n"]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerNewInput::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerNewInput",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The remote IPv4 address of the peer\n"]
    pub fn get_remote_ipv4(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerNewInput::RemoteIpv4(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerNewInput",
            "RemoteIpv4",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The remote IPv6 address of the peer\n"]
    pub fn get_remote_ipv6(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerNewInput::RemoteIpv6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerNewInput",
            "RemoteIpv6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The scope id of the remote IPv6 address of the peer (RFC2553)\n"]
    pub fn get_remote_ipv6_scope_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerNewInput::RemoteIpv6ScopeId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerNewInput",
            "RemoteIpv6ScopeId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The remote port of the peer\n"]
    pub fn get_remote_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerNewInput::RemotePort(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerNewInput",
            "RemotePort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The socket to be used to communicate with the peer\n"]
    pub fn get_socket(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerNewInput::Socket(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerNewInput",
            "Socket",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The IPv4 address assigned to the peer by the server\n"]
    pub fn get_vpn_ipv4(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerNewInput::VpnIpv4(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerNewInput",
            "VpnIpv4",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The IPv6 address assigned to the peer by the server\n"]
    pub fn get_vpn_ipv6(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerNewInput::VpnIpv6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerNewInput",
            "VpnIpv6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The local IPv4 to be used to send packets to the peer (UDP only)\n"]
    pub fn get_local_ipv4(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerNewInput::LocalIpv4(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerNewInput",
            "LocalIpv4",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The local IPv6 to be used to send packets to the peer (UDP only)\n"]
    pub fn get_local_ipv6(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerNewInput::LocalIpv6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerNewInput",
            "LocalIpv6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The number of seconds after which a keep alive message is sent to the\npeer\n"]
    pub fn get_keepalive_interval(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerNewInput::KeepaliveInterval(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerNewInput",
            "KeepaliveInterval",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The number of seconds from the last activity after which the peer is\nassumed dead\n"]
    pub fn get_keepalive_timeout(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerNewInput::KeepaliveTimeout(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerNewInput",
            "KeepaliveTimeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The ID value used when transmitting packets to this peer. This way\noutgoing packets can have a different ID than incoming ones. Useful in\nmultipeer-to-multipeer connections, where each peer will advertise the\ntx-id to be used on the link.\n"]
    pub fn get_tx_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerNewInput::TxId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerNewInput",
            "TxId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl PeerNewInput<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePeerNewInput<'a> {
        IterablePeerNewInput::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Peer::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePeerNewInput<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePeerNewInput<'a> {
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
impl<'a> Iterator for IterablePeerNewInput<'a> {
    type Item = Result<PeerNewInput<'a>, ErrorContext>;
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
                1u16 => PeerNewInput::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => PeerNewInput::RemoteIpv4({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => PeerNewInput::RemoteIpv6({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => PeerNewInput::RemoteIpv6ScopeId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => PeerNewInput::RemotePort({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => PeerNewInput::Socket({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => PeerNewInput::VpnIpv4({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => PeerNewInput::VpnIpv6({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => PeerNewInput::LocalIpv4({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => PeerNewInput::LocalIpv6({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => PeerNewInput::KeepaliveInterval({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => PeerNewInput::KeepaliveTimeout({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => PeerNewInput::TxId({
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
            "PeerNewInput",
            r#type.and_then(|t| PeerNewInput::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterablePeerNewInput<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("PeerNewInput");
        let mut iter = IterablePeerNewInput::with_loc(&[], self.orig_loc);
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
                PeerNewInput::Id(val) => fmt.field("Id", &val),
                PeerNewInput::RemoteIpv4(val) => fmt.field("RemoteIpv4", &val),
                PeerNewInput::RemoteIpv6(val) => fmt.field("RemoteIpv6", &val),
                PeerNewInput::RemoteIpv6ScopeId(val) => fmt.field("RemoteIpv6ScopeId", &val),
                PeerNewInput::RemotePort(val) => fmt.field("RemotePort", &val),
                PeerNewInput::Socket(val) => fmt.field("Socket", &val),
                PeerNewInput::VpnIpv4(val) => fmt.field("VpnIpv4", &val),
                PeerNewInput::VpnIpv6(val) => fmt.field("VpnIpv6", &val),
                PeerNewInput::LocalIpv4(val) => fmt.field("LocalIpv4", &val),
                PeerNewInput::LocalIpv6(val) => fmt.field("LocalIpv6", &val),
                PeerNewInput::KeepaliveInterval(val) => fmt.field("KeepaliveInterval", &val),
                PeerNewInput::KeepaliveTimeout(val) => fmt.field("KeepaliveTimeout", &val),
                PeerNewInput::TxId(val) => fmt.field("TxId", &val),
            };
        }
        fmt.finish()
    }
}
impl IterablePeerNewInput<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("PeerNewInput", offset));
            return (
                stack,
                missing_type.and_then(|t| PeerNewInput::attr_from_type(t)),
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
                PeerNewInput::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                PeerNewInput::RemoteIpv4(val) => {
                    if last_off == offset {
                        stack.push(("RemoteIpv4", last_off));
                        break;
                    }
                }
                PeerNewInput::RemoteIpv6(val) => {
                    if last_off == offset {
                        stack.push(("RemoteIpv6", last_off));
                        break;
                    }
                }
                PeerNewInput::RemoteIpv6ScopeId(val) => {
                    if last_off == offset {
                        stack.push(("RemoteIpv6ScopeId", last_off));
                        break;
                    }
                }
                PeerNewInput::RemotePort(val) => {
                    if last_off == offset {
                        stack.push(("RemotePort", last_off));
                        break;
                    }
                }
                PeerNewInput::Socket(val) => {
                    if last_off == offset {
                        stack.push(("Socket", last_off));
                        break;
                    }
                }
                PeerNewInput::VpnIpv4(val) => {
                    if last_off == offset {
                        stack.push(("VpnIpv4", last_off));
                        break;
                    }
                }
                PeerNewInput::VpnIpv6(val) => {
                    if last_off == offset {
                        stack.push(("VpnIpv6", last_off));
                        break;
                    }
                }
                PeerNewInput::LocalIpv4(val) => {
                    if last_off == offset {
                        stack.push(("LocalIpv4", last_off));
                        break;
                    }
                }
                PeerNewInput::LocalIpv6(val) => {
                    if last_off == offset {
                        stack.push(("LocalIpv6", last_off));
                        break;
                    }
                }
                PeerNewInput::KeepaliveInterval(val) => {
                    if last_off == offset {
                        stack.push(("KeepaliveInterval", last_off));
                        break;
                    }
                }
                PeerNewInput::KeepaliveTimeout(val) => {
                    if last_off == offset {
                        stack.push(("KeepaliveTimeout", last_off));
                        break;
                    }
                }
                PeerNewInput::TxId(val) => {
                    if last_off == offset {
                        stack.push(("TxId", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("PeerNewInput", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum PeerSetInput<'a> {
    #[doc = "The unique ID of the peer in the device context. To be used to identify\npeers during operations for a specific device. Also used to match\npackets received from this peer.\n"]
    Id(u32),
    #[doc = "The remote IPv4 address of the peer\n"]
    RemoteIpv4(std::net::Ipv4Addr),
    #[doc = "The remote IPv6 address of the peer\n"]
    RemoteIpv6(&'a [u8]),
    #[doc = "The scope id of the remote IPv6 address of the peer (RFC2553)\n"]
    RemoteIpv6ScopeId(u32),
    #[doc = "The remote port of the peer\n"]
    RemotePort(u16),
    #[doc = "The IPv4 address assigned to the peer by the server\n"]
    VpnIpv4(std::net::Ipv4Addr),
    #[doc = "The IPv6 address assigned to the peer by the server\n"]
    VpnIpv6(&'a [u8]),
    #[doc = "The local IPv4 to be used to send packets to the peer (UDP only)\n"]
    LocalIpv4(std::net::Ipv4Addr),
    #[doc = "The local IPv6 to be used to send packets to the peer (UDP only)\n"]
    LocalIpv6(&'a [u8]),
    #[doc = "The number of seconds after which a keep alive message is sent to the\npeer\n"]
    KeepaliveInterval(u32),
    #[doc = "The number of seconds from the last activity after which the peer is\nassumed dead\n"]
    KeepaliveTimeout(u32),
    #[doc = "The ID value used when transmitting packets to this peer. This way\noutgoing packets can have a different ID than incoming ones. Useful in\nmultipeer-to-multipeer connections, where each peer will advertise the\ntx-id to be used on the link.\n"]
    TxId(u32),
}
impl<'a> IterablePeerSetInput<'a> {
    #[doc = "The unique ID of the peer in the device context. To be used to identify\npeers during operations for a specific device. Also used to match\npackets received from this peer.\n"]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerSetInput::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerSetInput",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The remote IPv4 address of the peer\n"]
    pub fn get_remote_ipv4(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerSetInput::RemoteIpv4(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerSetInput",
            "RemoteIpv4",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The remote IPv6 address of the peer\n"]
    pub fn get_remote_ipv6(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerSetInput::RemoteIpv6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerSetInput",
            "RemoteIpv6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The scope id of the remote IPv6 address of the peer (RFC2553)\n"]
    pub fn get_remote_ipv6_scope_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerSetInput::RemoteIpv6ScopeId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerSetInput",
            "RemoteIpv6ScopeId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The remote port of the peer\n"]
    pub fn get_remote_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerSetInput::RemotePort(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerSetInput",
            "RemotePort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The IPv4 address assigned to the peer by the server\n"]
    pub fn get_vpn_ipv4(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerSetInput::VpnIpv4(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerSetInput",
            "VpnIpv4",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The IPv6 address assigned to the peer by the server\n"]
    pub fn get_vpn_ipv6(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerSetInput::VpnIpv6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerSetInput",
            "VpnIpv6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The local IPv4 to be used to send packets to the peer (UDP only)\n"]
    pub fn get_local_ipv4(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerSetInput::LocalIpv4(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerSetInput",
            "LocalIpv4",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The local IPv6 to be used to send packets to the peer (UDP only)\n"]
    pub fn get_local_ipv6(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerSetInput::LocalIpv6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerSetInput",
            "LocalIpv6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The number of seconds after which a keep alive message is sent to the\npeer\n"]
    pub fn get_keepalive_interval(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerSetInput::KeepaliveInterval(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerSetInput",
            "KeepaliveInterval",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The number of seconds from the last activity after which the peer is\nassumed dead\n"]
    pub fn get_keepalive_timeout(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerSetInput::KeepaliveTimeout(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerSetInput",
            "KeepaliveTimeout",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The ID value used when transmitting packets to this peer. This way\noutgoing packets can have a different ID than incoming ones. Useful in\nmultipeer-to-multipeer connections, where each peer will advertise the\ntx-id to be used on the link.\n"]
    pub fn get_tx_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerSetInput::TxId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerSetInput",
            "TxId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl PeerSetInput<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePeerSetInput<'a> {
        IterablePeerSetInput::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Peer::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePeerSetInput<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePeerSetInput<'a> {
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
impl<'a> Iterator for IterablePeerSetInput<'a> {
    type Item = Result<PeerSetInput<'a>, ErrorContext>;
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
                1u16 => PeerSetInput::Id({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => PeerSetInput::RemoteIpv4({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => PeerSetInput::RemoteIpv6({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => PeerSetInput::RemoteIpv6ScopeId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => PeerSetInput::RemotePort({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => PeerSetInput::VpnIpv4({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => PeerSetInput::VpnIpv6({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => PeerSetInput::LocalIpv4({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => PeerSetInput::LocalIpv6({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => PeerSetInput::KeepaliveInterval({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => PeerSetInput::KeepaliveTimeout({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                24u16 => PeerSetInput::TxId({
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
            "PeerSetInput",
            r#type.and_then(|t| PeerSetInput::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterablePeerSetInput<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("PeerSetInput");
        let mut iter = IterablePeerSetInput::with_loc(&[], self.orig_loc);
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
                PeerSetInput::Id(val) => fmt.field("Id", &val),
                PeerSetInput::RemoteIpv4(val) => fmt.field("RemoteIpv4", &val),
                PeerSetInput::RemoteIpv6(val) => fmt.field("RemoteIpv6", &val),
                PeerSetInput::RemoteIpv6ScopeId(val) => fmt.field("RemoteIpv6ScopeId", &val),
                PeerSetInput::RemotePort(val) => fmt.field("RemotePort", &val),
                PeerSetInput::VpnIpv4(val) => fmt.field("VpnIpv4", &val),
                PeerSetInput::VpnIpv6(val) => fmt.field("VpnIpv6", &val),
                PeerSetInput::LocalIpv4(val) => fmt.field("LocalIpv4", &val),
                PeerSetInput::LocalIpv6(val) => fmt.field("LocalIpv6", &val),
                PeerSetInput::KeepaliveInterval(val) => fmt.field("KeepaliveInterval", &val),
                PeerSetInput::KeepaliveTimeout(val) => fmt.field("KeepaliveTimeout", &val),
                PeerSetInput::TxId(val) => fmt.field("TxId", &val),
            };
        }
        fmt.finish()
    }
}
impl IterablePeerSetInput<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("PeerSetInput", offset));
            return (
                stack,
                missing_type.and_then(|t| PeerSetInput::attr_from_type(t)),
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
                PeerSetInput::Id(val) => {
                    if last_off == offset {
                        stack.push(("Id", last_off));
                        break;
                    }
                }
                PeerSetInput::RemoteIpv4(val) => {
                    if last_off == offset {
                        stack.push(("RemoteIpv4", last_off));
                        break;
                    }
                }
                PeerSetInput::RemoteIpv6(val) => {
                    if last_off == offset {
                        stack.push(("RemoteIpv6", last_off));
                        break;
                    }
                }
                PeerSetInput::RemoteIpv6ScopeId(val) => {
                    if last_off == offset {
                        stack.push(("RemoteIpv6ScopeId", last_off));
                        break;
                    }
                }
                PeerSetInput::RemotePort(val) => {
                    if last_off == offset {
                        stack.push(("RemotePort", last_off));
                        break;
                    }
                }
                PeerSetInput::VpnIpv4(val) => {
                    if last_off == offset {
                        stack.push(("VpnIpv4", last_off));
                        break;
                    }
                }
                PeerSetInput::VpnIpv6(val) => {
                    if last_off == offset {
                        stack.push(("VpnIpv6", last_off));
                        break;
                    }
                }
                PeerSetInput::LocalIpv4(val) => {
                    if last_off == offset {
                        stack.push(("LocalIpv4", last_off));
                        break;
                    }
                }
                PeerSetInput::LocalIpv6(val) => {
                    if last_off == offset {
                        stack.push(("LocalIpv6", last_off));
                        break;
                    }
                }
                PeerSetInput::KeepaliveInterval(val) => {
                    if last_off == offset {
                        stack.push(("KeepaliveInterval", last_off));
                        break;
                    }
                }
                PeerSetInput::KeepaliveTimeout(val) => {
                    if last_off == offset {
                        stack.push(("KeepaliveTimeout", last_off));
                        break;
                    }
                }
                PeerSetInput::TxId(val) => {
                    if last_off == offset {
                        stack.push(("TxId", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("PeerSetInput", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum PeerDelInput {
    #[doc = "The unique ID of the peer in the device context. To be used to identify\npeers during operations for a specific device. Also used to match\npackets received from this peer.\n"]
    Id(u32),
}
impl<'a> IterablePeerDelInput<'a> {
    #[doc = "The unique ID of the peer in the device context. To be used to identify\npeers during operations for a specific device. Also used to match\npackets received from this peer.\n"]
    pub fn get_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(PeerDelInput::Id(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PeerDelInput",
            "Id",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl PeerDelInput {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePeerDelInput<'a> {
        IterablePeerDelInput::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Peer::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePeerDelInput<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePeerDelInput<'a> {
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
impl<'a> Iterator for IterablePeerDelInput<'a> {
    type Item = Result<PeerDelInput, ErrorContext>;
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
                1u16 => PeerDelInput::Id({
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
            "PeerDelInput",
            r#type.and_then(|t| PeerDelInput::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterablePeerDelInput<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("PeerDelInput");
        let mut iter = IterablePeerDelInput::with_loc(&[], self.orig_loc);
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
                PeerDelInput::Id(val) => fmt.field("Id", &val),
            };
        }
        fmt.finish()
    }
}
impl IterablePeerDelInput<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("PeerDelInput", offset));
            return (
                stack,
                missing_type.and_then(|t| PeerDelInput::attr_from_type(t)),
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
                PeerDelInput::Id(val) => {
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
            stack.push(("PeerDelInput", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum Keyconf<'a> {
    #[doc = "The unique ID of the peer in the device context. To be used to identify\npeers during key operations\n"]
    PeerId(u32),
    #[doc = "The slot where the key should be stored\n\nAssociated type: [`KeySlot`] (enum)"]
    Slot(u32),
    #[doc = "The unique ID of the key in the peer context. Used to fetch the correct\nkey upon decryption\n"]
    KeyId(u32),
    #[doc = "The cipher to be used when communicating with the peer\n\nAssociated type: [`CipherAlg`] (enum)"]
    CipherAlg(u32),
    #[doc = "Key material for encrypt direction\n"]
    EncryptDir(IterableKeydir<'a>),
    #[doc = "Key material for decrypt direction\n"]
    DecryptDir(IterableKeydir<'a>),
}
impl<'a> IterableKeyconf<'a> {
    #[doc = "The unique ID of the peer in the device context. To be used to identify\npeers during key operations\n"]
    pub fn get_peer_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Keyconf::PeerId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Keyconf",
            "PeerId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The slot where the key should be stored\n\nAssociated type: [`KeySlot`] (enum)"]
    pub fn get_slot(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Keyconf::Slot(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Keyconf",
            "Slot",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The unique ID of the key in the peer context. Used to fetch the correct\nkey upon decryption\n"]
    pub fn get_key_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Keyconf::KeyId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Keyconf",
            "KeyId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The cipher to be used when communicating with the peer\n\nAssociated type: [`CipherAlg`] (enum)"]
    pub fn get_cipher_alg(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Keyconf::CipherAlg(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Keyconf",
            "CipherAlg",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Key material for encrypt direction\n"]
    pub fn get_encrypt_dir(&self) -> Result<IterableKeydir<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Keyconf::EncryptDir(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Keyconf",
            "EncryptDir",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Key material for decrypt direction\n"]
    pub fn get_decrypt_dir(&self) -> Result<IterableKeydir<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Keyconf::DecryptDir(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Keyconf",
            "DecryptDir",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Keyconf<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableKeyconf<'a> {
        IterableKeyconf::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "PeerId",
            2u16 => "Slot",
            3u16 => "KeyId",
            4u16 => "CipherAlg",
            5u16 => "EncryptDir",
            6u16 => "DecryptDir",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableKeyconf<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableKeyconf<'a> {
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
impl<'a> Iterator for IterableKeyconf<'a> {
    type Item = Result<Keyconf<'a>, ErrorContext>;
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
                1u16 => Keyconf::PeerId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Keyconf::Slot({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Keyconf::KeyId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Keyconf::CipherAlg({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Keyconf::EncryptDir({
                    let res = Some(IterableKeydir::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Keyconf::DecryptDir({
                    let res = Some(IterableKeydir::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Keyconf",
            r#type.and_then(|t| Keyconf::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableKeyconf<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Keyconf");
        let mut iter = IterableKeyconf::with_loc(&[], self.orig_loc);
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
                Keyconf::PeerId(val) => fmt.field("PeerId", &val),
                Keyconf::Slot(val) => {
                    fmt.field("Slot", &FormatEnum(val.into(), KeySlot::from_value))
                }
                Keyconf::KeyId(val) => fmt.field("KeyId", &val),
                Keyconf::CipherAlg(val) => {
                    fmt.field("CipherAlg", &FormatEnum(val.into(), CipherAlg::from_value))
                }
                Keyconf::EncryptDir(val) => fmt.field("EncryptDir", &val),
                Keyconf::DecryptDir(val) => fmt.field("DecryptDir", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableKeyconf<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Keyconf", offset));
            return (stack, missing_type.and_then(|t| Keyconf::attr_from_type(t)));
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
                Keyconf::PeerId(val) => {
                    if last_off == offset {
                        stack.push(("PeerId", last_off));
                        break;
                    }
                }
                Keyconf::Slot(val) => {
                    if last_off == offset {
                        stack.push(("Slot", last_off));
                        break;
                    }
                }
                Keyconf::KeyId(val) => {
                    if last_off == offset {
                        stack.push(("KeyId", last_off));
                        break;
                    }
                }
                Keyconf::CipherAlg(val) => {
                    if last_off == offset {
                        stack.push(("CipherAlg", last_off));
                        break;
                    }
                }
                Keyconf::EncryptDir(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Keyconf::DecryptDir(val) => {
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
            stack.push(("Keyconf", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum Keydir<'a> {
    #[doc = "The actual key to be used by the cipher\n"]
    CipherKey(&'a [u8]),
    #[doc = "Random nonce to be concatenated to the packet ID, in order to obtain the\nactual cipher IV\n"]
    NonceTail(&'a [u8]),
}
impl<'a> IterableKeydir<'a> {
    #[doc = "The actual key to be used by the cipher\n"]
    pub fn get_cipher_key(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Keydir::CipherKey(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Keydir",
            "CipherKey",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Random nonce to be concatenated to the packet ID, in order to obtain the\nactual cipher IV\n"]
    pub fn get_nonce_tail(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Keydir::NonceTail(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Keydir",
            "NonceTail",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Keydir<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableKeydir<'a> {
        IterableKeydir::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "CipherKey",
            2u16 => "NonceTail",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableKeydir<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableKeydir<'a> {
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
impl<'a> Iterator for IterableKeydir<'a> {
    type Item = Result<Keydir<'a>, ErrorContext>;
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
                1u16 => Keydir::CipherKey({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Keydir::NonceTail({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Keydir",
            r#type.and_then(|t| Keydir::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableKeydir<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Keydir");
        let mut iter = IterableKeydir::with_loc(&[], self.orig_loc);
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
                Keydir::CipherKey(val) => fmt.field("CipherKey", &FormatHexdump(val)),
                Keydir::NonceTail(val) => fmt.field("NonceTail", &FormatHexdump(val)),
            };
        }
        fmt.finish()
    }
}
impl IterableKeydir<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Keydir", offset));
            return (stack, missing_type.and_then(|t| Keydir::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Keydir::CipherKey(val) => {
                    if last_off == offset {
                        stack.push(("CipherKey", last_off));
                        break;
                    }
                }
                Keydir::NonceTail(val) => {
                    if last_off == offset {
                        stack.push(("NonceTail", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Keydir", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum KeyconfGet {
    #[doc = "The unique ID of the peer in the device context. To be used to identify\npeers during key operations\n"]
    PeerId(u32),
    #[doc = "The slot where the key should be stored\n\nAssociated type: [`KeySlot`] (enum)"]
    Slot(u32),
    #[doc = "The unique ID of the key in the peer context. Used to fetch the correct\nkey upon decryption\n"]
    KeyId(u32),
    #[doc = "The cipher to be used when communicating with the peer\n\nAssociated type: [`CipherAlg`] (enum)"]
    CipherAlg(u32),
}
impl<'a> IterableKeyconfGet<'a> {
    #[doc = "The unique ID of the peer in the device context. To be used to identify\npeers during key operations\n"]
    pub fn get_peer_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyconfGet::PeerId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyconfGet",
            "PeerId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The slot where the key should be stored\n\nAssociated type: [`KeySlot`] (enum)"]
    pub fn get_slot(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyconfGet::Slot(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyconfGet",
            "Slot",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The unique ID of the key in the peer context. Used to fetch the correct\nkey upon decryption\n"]
    pub fn get_key_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyconfGet::KeyId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyconfGet",
            "KeyId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The cipher to be used when communicating with the peer\n\nAssociated type: [`CipherAlg`] (enum)"]
    pub fn get_cipher_alg(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyconfGet::CipherAlg(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyconfGet",
            "CipherAlg",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl KeyconfGet {
    pub fn new<'a>(buf: &'a [u8]) -> IterableKeyconfGet<'a> {
        IterableKeyconfGet::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Keyconf::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableKeyconfGet<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableKeyconfGet<'a> {
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
impl<'a> Iterator for IterableKeyconfGet<'a> {
    type Item = Result<KeyconfGet, ErrorContext>;
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
                1u16 => KeyconfGet::PeerId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => KeyconfGet::Slot({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => KeyconfGet::KeyId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => KeyconfGet::CipherAlg({
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
            "KeyconfGet",
            r#type.and_then(|t| KeyconfGet::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableKeyconfGet<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("KeyconfGet");
        let mut iter = IterableKeyconfGet::with_loc(&[], self.orig_loc);
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
                KeyconfGet::PeerId(val) => fmt.field("PeerId", &val),
                KeyconfGet::Slot(val) => {
                    fmt.field("Slot", &FormatEnum(val.into(), KeySlot::from_value))
                }
                KeyconfGet::KeyId(val) => fmt.field("KeyId", &val),
                KeyconfGet::CipherAlg(val) => {
                    fmt.field("CipherAlg", &FormatEnum(val.into(), CipherAlg::from_value))
                }
            };
        }
        fmt.finish()
    }
}
impl IterableKeyconfGet<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("KeyconfGet", offset));
            return (
                stack,
                missing_type.and_then(|t| KeyconfGet::attr_from_type(t)),
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
                KeyconfGet::PeerId(val) => {
                    if last_off == offset {
                        stack.push(("PeerId", last_off));
                        break;
                    }
                }
                KeyconfGet::Slot(val) => {
                    if last_off == offset {
                        stack.push(("Slot", last_off));
                        break;
                    }
                }
                KeyconfGet::KeyId(val) => {
                    if last_off == offset {
                        stack.push(("KeyId", last_off));
                        break;
                    }
                }
                KeyconfGet::CipherAlg(val) => {
                    if last_off == offset {
                        stack.push(("CipherAlg", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("KeyconfGet", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum KeyconfSwapInput {
    #[doc = "The unique ID of the peer in the device context. To be used to identify\npeers during key operations\n"]
    PeerId(u32),
}
impl<'a> IterableKeyconfSwapInput<'a> {
    #[doc = "The unique ID of the peer in the device context. To be used to identify\npeers during key operations\n"]
    pub fn get_peer_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyconfSwapInput::PeerId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyconfSwapInput",
            "PeerId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl KeyconfSwapInput {
    pub fn new<'a>(buf: &'a [u8]) -> IterableKeyconfSwapInput<'a> {
        IterableKeyconfSwapInput::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Keyconf::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableKeyconfSwapInput<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableKeyconfSwapInput<'a> {
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
impl<'a> Iterator for IterableKeyconfSwapInput<'a> {
    type Item = Result<KeyconfSwapInput, ErrorContext>;
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
                1u16 => KeyconfSwapInput::PeerId({
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
            "KeyconfSwapInput",
            r#type.and_then(|t| KeyconfSwapInput::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableKeyconfSwapInput<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("KeyconfSwapInput");
        let mut iter = IterableKeyconfSwapInput::with_loc(&[], self.orig_loc);
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
                KeyconfSwapInput::PeerId(val) => fmt.field("PeerId", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableKeyconfSwapInput<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("KeyconfSwapInput", offset));
            return (
                stack,
                missing_type.and_then(|t| KeyconfSwapInput::attr_from_type(t)),
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
                KeyconfSwapInput::PeerId(val) => {
                    if last_off == offset {
                        stack.push(("PeerId", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("KeyconfSwapInput", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum KeyconfDelInput {
    #[doc = "The unique ID of the peer in the device context. To be used to identify\npeers during key operations\n"]
    PeerId(u32),
    #[doc = "The slot where the key should be stored\n\nAssociated type: [`KeySlot`] (enum)"]
    Slot(u32),
}
impl<'a> IterableKeyconfDelInput<'a> {
    #[doc = "The unique ID of the peer in the device context. To be used to identify\npeers during key operations\n"]
    pub fn get_peer_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyconfDelInput::PeerId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyconfDelInput",
            "PeerId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The slot where the key should be stored\n\nAssociated type: [`KeySlot`] (enum)"]
    pub fn get_slot(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(KeyconfDelInput::Slot(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "KeyconfDelInput",
            "Slot",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl KeyconfDelInput {
    pub fn new<'a>(buf: &'a [u8]) -> IterableKeyconfDelInput<'a> {
        IterableKeyconfDelInput::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Keyconf::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableKeyconfDelInput<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableKeyconfDelInput<'a> {
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
impl<'a> Iterator for IterableKeyconfDelInput<'a> {
    type Item = Result<KeyconfDelInput, ErrorContext>;
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
                1u16 => KeyconfDelInput::PeerId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => KeyconfDelInput::Slot({
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
            "KeyconfDelInput",
            r#type.and_then(|t| KeyconfDelInput::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableKeyconfDelInput<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("KeyconfDelInput");
        let mut iter = IterableKeyconfDelInput::with_loc(&[], self.orig_loc);
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
                KeyconfDelInput::PeerId(val) => fmt.field("PeerId", &val),
                KeyconfDelInput::Slot(val) => {
                    fmt.field("Slot", &FormatEnum(val.into(), KeySlot::from_value))
                }
            };
        }
        fmt.finish()
    }
}
impl IterableKeyconfDelInput<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("KeyconfDelInput", offset));
            return (
                stack,
                missing_type.and_then(|t| KeyconfDelInput::attr_from_type(t)),
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
                KeyconfDelInput::PeerId(val) => {
                    if last_off == offset {
                        stack.push(("PeerId", last_off));
                        break;
                    }
                }
                KeyconfDelInput::Slot(val) => {
                    if last_off == offset {
                        stack.push(("Slot", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("KeyconfDelInput", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum Ovpn<'a> {
    #[doc = "Index of the ovpn interface to operate on\n"]
    Ifindex(u32),
    #[doc = "The peer object containing the attributed of interest for the specific\noperation\n"]
    Peer(IterablePeer<'a>),
    #[doc = "Peer specific cipher configuration\n"]
    Keyconf(IterableKeyconf<'a>),
}
impl<'a> IterableOvpn<'a> {
    #[doc = "Index of the ovpn interface to operate on\n"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Ovpn::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Ovpn",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The peer object containing the attributed of interest for the specific\noperation\n"]
    pub fn get_peer(&self) -> Result<IterablePeer<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Ovpn::Peer(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Ovpn",
            "Peer",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Peer specific cipher configuration\n"]
    pub fn get_keyconf(&self) -> Result<IterableKeyconf<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Ovpn::Keyconf(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Ovpn",
            "Keyconf",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Ovpn<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOvpn<'a> {
        IterableOvpn::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Ifindex",
            2u16 => "Peer",
            3u16 => "Keyconf",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOvpn<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOvpn<'a> {
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
impl<'a> Iterator for IterableOvpn<'a> {
    type Item = Result<Ovpn<'a>, ErrorContext>;
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
                1u16 => Ovpn::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Ovpn::Peer({
                    let res = Some(IterablePeer::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Ovpn::Keyconf({
                    let res = Some(IterableKeyconf::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Ovpn",
            r#type.and_then(|t| Ovpn::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOvpn<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Ovpn");
        let mut iter = IterableOvpn::with_loc(&[], self.orig_loc);
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
                Ovpn::Ifindex(val) => fmt.field("Ifindex", &val),
                Ovpn::Peer(val) => fmt.field("Peer", &val),
                Ovpn::Keyconf(val) => fmt.field("Keyconf", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOvpn<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Ovpn", offset));
            return (stack, missing_type.and_then(|t| Ovpn::attr_from_type(t)));
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
                Ovpn::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                Ovpn::Peer(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                Ovpn::Keyconf(val) => {
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
            stack.push(("Ovpn", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum OvpnPeerNewInput<'a> {
    #[doc = "Index of the ovpn interface to operate on\n"]
    Ifindex(u32),
    #[doc = "The peer object containing the attributed of interest for the specific\noperation\n"]
    Peer(IterablePeerNewInput<'a>),
}
impl<'a> IterableOvpnPeerNewInput<'a> {
    #[doc = "Index of the ovpn interface to operate on\n"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(OvpnPeerNewInput::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OvpnPeerNewInput",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The peer object containing the attributed of interest for the specific\noperation\n"]
    pub fn get_peer(&self) -> Result<IterablePeerNewInput<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(OvpnPeerNewInput::Peer(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OvpnPeerNewInput",
            "Peer",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OvpnPeerNewInput<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOvpnPeerNewInput<'a> {
        IterableOvpnPeerNewInput::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Ovpn::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOvpnPeerNewInput<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOvpnPeerNewInput<'a> {
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
impl<'a> Iterator for IterableOvpnPeerNewInput<'a> {
    type Item = Result<OvpnPeerNewInput<'a>, ErrorContext>;
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
                1u16 => OvpnPeerNewInput::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OvpnPeerNewInput::Peer({
                    let res = Some(IterablePeerNewInput::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OvpnPeerNewInput",
            r#type.and_then(|t| OvpnPeerNewInput::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOvpnPeerNewInput<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OvpnPeerNewInput");
        let mut iter = IterableOvpnPeerNewInput::with_loc(&[], self.orig_loc);
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
                OvpnPeerNewInput::Ifindex(val) => fmt.field("Ifindex", &val),
                OvpnPeerNewInput::Peer(val) => fmt.field("Peer", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOvpnPeerNewInput<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("OvpnPeerNewInput", offset));
            return (
                stack,
                missing_type.and_then(|t| OvpnPeerNewInput::attr_from_type(t)),
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
                OvpnPeerNewInput::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OvpnPeerNewInput::Peer(val) => {
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
            stack.push(("OvpnPeerNewInput", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum OvpnPeerSetInput<'a> {
    #[doc = "Index of the ovpn interface to operate on\n"]
    Ifindex(u32),
    #[doc = "The peer object containing the attributed of interest for the specific\noperation\n"]
    Peer(IterablePeerSetInput<'a>),
}
impl<'a> IterableOvpnPeerSetInput<'a> {
    #[doc = "Index of the ovpn interface to operate on\n"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(OvpnPeerSetInput::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OvpnPeerSetInput",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The peer object containing the attributed of interest for the specific\noperation\n"]
    pub fn get_peer(&self) -> Result<IterablePeerSetInput<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(OvpnPeerSetInput::Peer(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OvpnPeerSetInput",
            "Peer",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OvpnPeerSetInput<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOvpnPeerSetInput<'a> {
        IterableOvpnPeerSetInput::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Ovpn::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOvpnPeerSetInput<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOvpnPeerSetInput<'a> {
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
impl<'a> Iterator for IterableOvpnPeerSetInput<'a> {
    type Item = Result<OvpnPeerSetInput<'a>, ErrorContext>;
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
                1u16 => OvpnPeerSetInput::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OvpnPeerSetInput::Peer({
                    let res = Some(IterablePeerSetInput::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OvpnPeerSetInput",
            r#type.and_then(|t| OvpnPeerSetInput::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOvpnPeerSetInput<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OvpnPeerSetInput");
        let mut iter = IterableOvpnPeerSetInput::with_loc(&[], self.orig_loc);
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
                OvpnPeerSetInput::Ifindex(val) => fmt.field("Ifindex", &val),
                OvpnPeerSetInput::Peer(val) => fmt.field("Peer", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOvpnPeerSetInput<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("OvpnPeerSetInput", offset));
            return (
                stack,
                missing_type.and_then(|t| OvpnPeerSetInput::attr_from_type(t)),
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
                OvpnPeerSetInput::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OvpnPeerSetInput::Peer(val) => {
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
            stack.push(("OvpnPeerSetInput", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum OvpnPeerDelInput<'a> {
    #[doc = "Index of the ovpn interface to operate on\n"]
    Ifindex(u32),
    #[doc = "The peer object containing the attributed of interest for the specific\noperation\n"]
    Peer(IterablePeerDelInput<'a>),
}
impl<'a> IterableOvpnPeerDelInput<'a> {
    #[doc = "Index of the ovpn interface to operate on\n"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(OvpnPeerDelInput::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OvpnPeerDelInput",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The peer object containing the attributed of interest for the specific\noperation\n"]
    pub fn get_peer(&self) -> Result<IterablePeerDelInput<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(OvpnPeerDelInput::Peer(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OvpnPeerDelInput",
            "Peer",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OvpnPeerDelInput<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOvpnPeerDelInput<'a> {
        IterableOvpnPeerDelInput::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Ovpn::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOvpnPeerDelInput<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOvpnPeerDelInput<'a> {
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
impl<'a> Iterator for IterableOvpnPeerDelInput<'a> {
    type Item = Result<OvpnPeerDelInput<'a>, ErrorContext>;
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
                1u16 => OvpnPeerDelInput::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OvpnPeerDelInput::Peer({
                    let res = Some(IterablePeerDelInput::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OvpnPeerDelInput",
            r#type.and_then(|t| OvpnPeerDelInput::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOvpnPeerDelInput<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OvpnPeerDelInput");
        let mut iter = IterableOvpnPeerDelInput::with_loc(&[], self.orig_loc);
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
                OvpnPeerDelInput::Ifindex(val) => fmt.field("Ifindex", &val),
                OvpnPeerDelInput::Peer(val) => fmt.field("Peer", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOvpnPeerDelInput<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("OvpnPeerDelInput", offset));
            return (
                stack,
                missing_type.and_then(|t| OvpnPeerDelInput::attr_from_type(t)),
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
                OvpnPeerDelInput::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OvpnPeerDelInput::Peer(val) => {
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
            stack.push(("OvpnPeerDelInput", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum OvpnKeyconfGet<'a> {
    #[doc = "Index of the ovpn interface to operate on\n"]
    Ifindex(u32),
    #[doc = "Peer specific cipher configuration\n"]
    Keyconf(IterableKeyconfGet<'a>),
}
impl<'a> IterableOvpnKeyconfGet<'a> {
    #[doc = "Index of the ovpn interface to operate on\n"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(OvpnKeyconfGet::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OvpnKeyconfGet",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Peer specific cipher configuration\n"]
    pub fn get_keyconf(&self) -> Result<IterableKeyconfGet<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(OvpnKeyconfGet::Keyconf(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OvpnKeyconfGet",
            "Keyconf",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OvpnKeyconfGet<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOvpnKeyconfGet<'a> {
        IterableOvpnKeyconfGet::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Ovpn::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOvpnKeyconfGet<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOvpnKeyconfGet<'a> {
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
impl<'a> Iterator for IterableOvpnKeyconfGet<'a> {
    type Item = Result<OvpnKeyconfGet<'a>, ErrorContext>;
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
                1u16 => OvpnKeyconfGet::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OvpnKeyconfGet::Keyconf({
                    let res = Some(IterableKeyconfGet::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OvpnKeyconfGet",
            r#type.and_then(|t| OvpnKeyconfGet::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOvpnKeyconfGet<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OvpnKeyconfGet");
        let mut iter = IterableOvpnKeyconfGet::with_loc(&[], self.orig_loc);
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
                OvpnKeyconfGet::Ifindex(val) => fmt.field("Ifindex", &val),
                OvpnKeyconfGet::Keyconf(val) => fmt.field("Keyconf", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOvpnKeyconfGet<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("OvpnKeyconfGet", offset));
            return (
                stack,
                missing_type.and_then(|t| OvpnKeyconfGet::attr_from_type(t)),
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
                OvpnKeyconfGet::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OvpnKeyconfGet::Keyconf(val) => {
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
            stack.push(("OvpnKeyconfGet", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum OvpnKeyconfSwapInput<'a> {
    #[doc = "Index of the ovpn interface to operate on\n"]
    Ifindex(u32),
    #[doc = "Peer specific cipher configuration\n"]
    Keyconf(IterableKeyconfSwapInput<'a>),
}
impl<'a> IterableOvpnKeyconfSwapInput<'a> {
    #[doc = "Index of the ovpn interface to operate on\n"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(OvpnKeyconfSwapInput::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OvpnKeyconfSwapInput",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Peer specific cipher configuration\n"]
    pub fn get_keyconf(&self) -> Result<IterableKeyconfSwapInput<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(OvpnKeyconfSwapInput::Keyconf(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OvpnKeyconfSwapInput",
            "Keyconf",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OvpnKeyconfSwapInput<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOvpnKeyconfSwapInput<'a> {
        IterableOvpnKeyconfSwapInput::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Ovpn::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOvpnKeyconfSwapInput<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOvpnKeyconfSwapInput<'a> {
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
impl<'a> Iterator for IterableOvpnKeyconfSwapInput<'a> {
    type Item = Result<OvpnKeyconfSwapInput<'a>, ErrorContext>;
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
                1u16 => OvpnKeyconfSwapInput::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OvpnKeyconfSwapInput::Keyconf({
                    let res = Some(IterableKeyconfSwapInput::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OvpnKeyconfSwapInput",
            r#type.and_then(|t| OvpnKeyconfSwapInput::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOvpnKeyconfSwapInput<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OvpnKeyconfSwapInput");
        let mut iter = IterableOvpnKeyconfSwapInput::with_loc(&[], self.orig_loc);
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
                OvpnKeyconfSwapInput::Ifindex(val) => fmt.field("Ifindex", &val),
                OvpnKeyconfSwapInput::Keyconf(val) => fmt.field("Keyconf", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOvpnKeyconfSwapInput<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("OvpnKeyconfSwapInput", offset));
            return (
                stack,
                missing_type.and_then(|t| OvpnKeyconfSwapInput::attr_from_type(t)),
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
                OvpnKeyconfSwapInput::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OvpnKeyconfSwapInput::Keyconf(val) => {
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
            stack.push(("OvpnKeyconfSwapInput", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum OvpnKeyconfDelInput<'a> {
    #[doc = "Index of the ovpn interface to operate on\n"]
    Ifindex(u32),
    #[doc = "Peer specific cipher configuration\n"]
    Keyconf(IterableKeyconfDelInput<'a>),
}
impl<'a> IterableOvpnKeyconfDelInput<'a> {
    #[doc = "Index of the ovpn interface to operate on\n"]
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(OvpnKeyconfDelInput::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OvpnKeyconfDelInput",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Peer specific cipher configuration\n"]
    pub fn get_keyconf(&self) -> Result<IterableKeyconfDelInput<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(OvpnKeyconfDelInput::Keyconf(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "OvpnKeyconfDelInput",
            "Keyconf",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl OvpnKeyconfDelInput<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableOvpnKeyconfDelInput<'a> {
        IterableOvpnKeyconfDelInput::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        Ovpn::attr_from_type(r#type)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableOvpnKeyconfDelInput<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableOvpnKeyconfDelInput<'a> {
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
impl<'a> Iterator for IterableOvpnKeyconfDelInput<'a> {
    type Item = Result<OvpnKeyconfDelInput<'a>, ErrorContext>;
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
                1u16 => OvpnKeyconfDelInput::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OvpnKeyconfDelInput::Keyconf({
                    let res = Some(IterableKeyconfDelInput::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "OvpnKeyconfDelInput",
            r#type.and_then(|t| OvpnKeyconfDelInput::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableOvpnKeyconfDelInput<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OvpnKeyconfDelInput");
        let mut iter = IterableOvpnKeyconfDelInput::with_loc(&[], self.orig_loc);
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
                OvpnKeyconfDelInput::Ifindex(val) => fmt.field("Ifindex", &val),
                OvpnKeyconfDelInput::Keyconf(val) => fmt.field("Keyconf", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableOvpnKeyconfDelInput<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("OvpnKeyconfDelInput", offset));
            return (
                stack,
                missing_type.and_then(|t| OvpnKeyconfDelInput::attr_from_type(t)),
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
                OvpnKeyconfDelInput::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OvpnKeyconfDelInput::Keyconf(val) => {
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
            stack.push(("OvpnKeyconfDelInput", cur));
        }
        (stack, missing)
    }
}
pub struct PushPeer<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushPeer<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushPeer<Prev> {
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
    #[doc = "The unique ID of the peer in the device context. To be used to identify\npeers during operations for a specific device. Also used to match\npackets received from this peer.\n"]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The remote IPv4 address of the peer\n"]
    pub fn push_remote_ipv4(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    #[doc = "The remote IPv6 address of the peer\n"]
    pub fn push_remote_ipv6(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "The scope id of the remote IPv6 address of the peer (RFC2553)\n"]
    pub fn push_remote_ipv6_scope_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The remote port of the peer\n"]
    pub fn push_remote_port(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 5u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "The socket to be used to communicate with the peer\n"]
    pub fn push_socket(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The ID of the netns the socket assigned to this peer lives in\n"]
    pub fn push_socket_netnsid(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The IPv4 address assigned to the peer by the server\n"]
    pub fn push_vpn_ipv4(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    #[doc = "The IPv6 address assigned to the peer by the server\n"]
    pub fn push_vpn_ipv6(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 9u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "The local IPv4 to be used to send packets to the peer (UDP only)\n"]
    pub fn push_local_ipv4(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    #[doc = "The local IPv6 to be used to send packets to the peer (UDP only)\n"]
    pub fn push_local_ipv6(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 11u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "The local port to be used to send packets to the peer (UDP only)\n"]
    pub fn push_local_port(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 12u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "The number of seconds after which a keep alive message is sent to the\npeer\n"]
    pub fn push_keepalive_interval(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 13u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The number of seconds from the last activity after which the peer is\nassumed dead\n"]
    pub fn push_keepalive_timeout(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 14u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The reason why a peer was deleted\n\nAssociated type: [`DelPeerReason`] (enum)"]
    pub fn push_del_reason(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 15u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of bytes received over the tunnel\n"]
    pub fn push_vpn_rx_bytes(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 16u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of bytes transmitted over the tunnel\n"]
    pub fn push_vpn_tx_bytes(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 17u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets received over the tunnel\n"]
    pub fn push_vpn_rx_packets(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 18u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets transmitted over the tunnel\n"]
    pub fn push_vpn_tx_packets(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 19u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of bytes received at the transport level\n"]
    pub fn push_link_rx_bytes(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 20u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of bytes transmitted at the transport level\n"]
    pub fn push_link_tx_bytes(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 21u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets received at the transport level\n"]
    pub fn push_link_rx_packets(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 22u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Number of packets transmitted at the transport level\n"]
    pub fn push_link_tx_packets(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 23u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The ID value used when transmitting packets to this peer. This way\noutgoing packets can have a different ID than incoming ones. Useful in\nmultipeer-to-multipeer connections, where each peer will advertise the\ntx-id to be used on the link.\n"]
    pub fn push_tx_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 24u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushPeer<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPeerNewInput<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushPeerNewInput<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushPeerNewInput<Prev> {
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
    #[doc = "The unique ID of the peer in the device context. To be used to identify\npeers during operations for a specific device. Also used to match\npackets received from this peer.\n"]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The remote IPv4 address of the peer\n"]
    pub fn push_remote_ipv4(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    #[doc = "The remote IPv6 address of the peer\n"]
    pub fn push_remote_ipv6(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "The scope id of the remote IPv6 address of the peer (RFC2553)\n"]
    pub fn push_remote_ipv6_scope_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The remote port of the peer\n"]
    pub fn push_remote_port(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 5u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "The socket to be used to communicate with the peer\n"]
    pub fn push_socket(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The IPv4 address assigned to the peer by the server\n"]
    pub fn push_vpn_ipv4(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    #[doc = "The IPv6 address assigned to the peer by the server\n"]
    pub fn push_vpn_ipv6(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 9u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "The local IPv4 to be used to send packets to the peer (UDP only)\n"]
    pub fn push_local_ipv4(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    #[doc = "The local IPv6 to be used to send packets to the peer (UDP only)\n"]
    pub fn push_local_ipv6(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 11u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "The number of seconds after which a keep alive message is sent to the\npeer\n"]
    pub fn push_keepalive_interval(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 13u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The number of seconds from the last activity after which the peer is\nassumed dead\n"]
    pub fn push_keepalive_timeout(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 14u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The ID value used when transmitting packets to this peer. This way\noutgoing packets can have a different ID than incoming ones. Useful in\nmultipeer-to-multipeer connections, where each peer will advertise the\ntx-id to be used on the link.\n"]
    pub fn push_tx_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 24u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushPeerNewInput<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPeerSetInput<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushPeerSetInput<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushPeerSetInput<Prev> {
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
    #[doc = "The unique ID of the peer in the device context. To be used to identify\npeers during operations for a specific device. Also used to match\npackets received from this peer.\n"]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The remote IPv4 address of the peer\n"]
    pub fn push_remote_ipv4(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    #[doc = "The remote IPv6 address of the peer\n"]
    pub fn push_remote_ipv6(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "The scope id of the remote IPv6 address of the peer (RFC2553)\n"]
    pub fn push_remote_ipv6_scope_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The remote port of the peer\n"]
    pub fn push_remote_port(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 5u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "The IPv4 address assigned to the peer by the server\n"]
    pub fn push_vpn_ipv4(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    #[doc = "The IPv6 address assigned to the peer by the server\n"]
    pub fn push_vpn_ipv6(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 9u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "The local IPv4 to be used to send packets to the peer (UDP only)\n"]
    pub fn push_local_ipv4(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    #[doc = "The local IPv6 to be used to send packets to the peer (UDP only)\n"]
    pub fn push_local_ipv6(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 11u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "The number of seconds after which a keep alive message is sent to the\npeer\n"]
    pub fn push_keepalive_interval(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 13u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The number of seconds from the last activity after which the peer is\nassumed dead\n"]
    pub fn push_keepalive_timeout(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 14u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The ID value used when transmitting packets to this peer. This way\noutgoing packets can have a different ID than incoming ones. Useful in\nmultipeer-to-multipeer connections, where each peer will advertise the\ntx-id to be used on the link.\n"]
    pub fn push_tx_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 24u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushPeerSetInput<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPeerDelInput<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushPeerDelInput<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushPeerDelInput<Prev> {
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
    #[doc = "The unique ID of the peer in the device context. To be used to identify\npeers during operations for a specific device. Also used to match\npackets received from this peer.\n"]
    pub fn push_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushPeerDelInput<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushKeyconf<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushKeyconf<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushKeyconf<Prev> {
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
    #[doc = "The unique ID of the peer in the device context. To be used to identify\npeers during key operations\n"]
    pub fn push_peer_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The slot where the key should be stored\n\nAssociated type: [`KeySlot`] (enum)"]
    pub fn push_slot(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The unique ID of the key in the peer context. Used to fetch the correct\nkey upon decryption\n"]
    pub fn push_key_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The cipher to be used when communicating with the peer\n\nAssociated type: [`CipherAlg`] (enum)"]
    pub fn push_cipher_alg(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Key material for encrypt direction\n"]
    pub fn nested_encrypt_dir(mut self) -> PushKeydir<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 5u16);
        PushKeydir {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Key material for decrypt direction\n"]
    pub fn nested_decrypt_dir(mut self) -> PushKeydir<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 6u16);
        PushKeydir {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushKeyconf<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushKeydir<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushKeydir<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushKeydir<Prev> {
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
    #[doc = "The actual key to be used by the cipher\n"]
    pub fn push_cipher_key(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "Random nonce to be concatenated to the packet ID, in order to obtain the\nactual cipher IV\n"]
    pub fn push_nonce_tail(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushKeydir<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushKeyconfGet<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushKeyconfGet<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushKeyconfGet<Prev> {
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
    #[doc = "The unique ID of the peer in the device context. To be used to identify\npeers during key operations\n"]
    pub fn push_peer_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The slot where the key should be stored\n\nAssociated type: [`KeySlot`] (enum)"]
    pub fn push_slot(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The unique ID of the key in the peer context. Used to fetch the correct\nkey upon decryption\n"]
    pub fn push_key_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The cipher to be used when communicating with the peer\n\nAssociated type: [`CipherAlg`] (enum)"]
    pub fn push_cipher_alg(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushKeyconfGet<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushKeyconfSwapInput<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushKeyconfSwapInput<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushKeyconfSwapInput<Prev> {
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
    #[doc = "The unique ID of the peer in the device context. To be used to identify\npeers during key operations\n"]
    pub fn push_peer_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushKeyconfSwapInput<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushKeyconfDelInput<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushKeyconfDelInput<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushKeyconfDelInput<Prev> {
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
    #[doc = "The unique ID of the peer in the device context. To be used to identify\npeers during key operations\n"]
    pub fn push_peer_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The slot where the key should be stored\n\nAssociated type: [`KeySlot`] (enum)"]
    pub fn push_slot(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushKeyconfDelInput<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushOvpn<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushOvpn<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushOvpn<Prev> {
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
    #[doc = "Index of the ovpn interface to operate on\n"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The peer object containing the attributed of interest for the specific\noperation\n"]
    pub fn nested_peer(mut self) -> PushPeer<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushPeer {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    #[doc = "Peer specific cipher configuration\n"]
    pub fn nested_keyconf(mut self) -> PushKeyconf<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 3u16);
        PushKeyconf {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushOvpn<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushOvpnPeerNewInput<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushOvpnPeerNewInput<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushOvpnPeerNewInput<Prev> {
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
    #[doc = "Index of the ovpn interface to operate on\n"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The peer object containing the attributed of interest for the specific\noperation\n"]
    pub fn nested_peer(mut self) -> PushPeerNewInput<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushPeerNewInput {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushOvpnPeerNewInput<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushOvpnPeerSetInput<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushOvpnPeerSetInput<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushOvpnPeerSetInput<Prev> {
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
    #[doc = "Index of the ovpn interface to operate on\n"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The peer object containing the attributed of interest for the specific\noperation\n"]
    pub fn nested_peer(mut self) -> PushPeerSetInput<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushPeerSetInput {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushOvpnPeerSetInput<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushOvpnPeerDelInput<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushOvpnPeerDelInput<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushOvpnPeerDelInput<Prev> {
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
    #[doc = "Index of the ovpn interface to operate on\n"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The peer object containing the attributed of interest for the specific\noperation\n"]
    pub fn nested_peer(mut self) -> PushPeerDelInput<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 2u16);
        PushPeerDelInput {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushOvpnPeerDelInput<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushOvpnKeyconfGet<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushOvpnKeyconfGet<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushOvpnKeyconfGet<Prev> {
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
    #[doc = "Index of the ovpn interface to operate on\n"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Peer specific cipher configuration\n"]
    pub fn nested_keyconf(mut self) -> PushKeyconfGet<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 3u16);
        PushKeyconfGet {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushOvpnKeyconfGet<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushOvpnKeyconfSwapInput<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushOvpnKeyconfSwapInput<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushOvpnKeyconfSwapInput<Prev> {
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
    #[doc = "Index of the ovpn interface to operate on\n"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Peer specific cipher configuration\n"]
    pub fn nested_keyconf(mut self) -> PushKeyconfSwapInput<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 3u16);
        PushKeyconfSwapInput {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushOvpnKeyconfSwapInput<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushOvpnKeyconfDelInput<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushOvpnKeyconfDelInput<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushOvpnKeyconfDelInput<Prev> {
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
    #[doc = "Index of the ovpn interface to operate on\n"]
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Peer specific cipher configuration\n"]
    pub fn nested_keyconf(mut self) -> PushKeyconfDelInput<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 3u16);
        PushKeyconfDelInput {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Pusher> Drop for PushOvpnKeyconfDelInput<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Notify attributes:\n- [`.get_peer()`](IterableOvpn::get_peer)\n"]
#[derive(Debug)]
pub struct OpPeerDelNotif;
impl OpPeerDelNotif {
    pub const CMD: u8 = 5u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterableOvpn<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableOvpn::with_loc(attrs, buf.as_ptr() as usize)
    }
}
#[doc = "Notify attributes:\n- [`.get_keyconf()`](IterableOvpnKeyconfGet::get_keyconf)\n"]
#[derive(Debug)]
pub struct OpKeySwapNotif;
impl OpKeySwapNotif {
    pub const CMD: u8 = 9u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterableOvpnKeyconfGet<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableOvpnKeyconfGet::with_loc(attrs, buf.as_ptr() as usize)
    }
}
#[doc = "Notify attributes:\n- [`.get_peer()`](IterableOvpn::get_peer)\n"]
#[derive(Debug)]
pub struct OpPeerFloatNotif;
impl OpPeerFloatNotif {
    pub const CMD: u8 = 11u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterableOvpn<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableOvpn::with_loc(attrs, buf.as_ptr() as usize)
    }
}
pub struct NotifGroup;
impl NotifGroup {
    #[doc = "Notifications:\n- [`OpPeerDelNotif`]\n- [`OpKeySwapNotif`]\n- [`OpPeerFloatNotif`]\n"]
    pub const PEERS: &str = "peers";
    #[doc = "Notifications:\n- [`OpPeerDelNotif`]\n- [`OpKeySwapNotif`]\n- [`OpPeerFloatNotif`]\n"]
    pub const PEERS_CSTR: &CStr = c"peers";
}
#[doc = "Add a remote peer\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushOvpnPeerNewInput::push_ifindex)\n- [.nested_peer()](PushOvpnPeerNewInput::nested_peer)\n\n"]
#[derive(Debug)]
pub struct OpPeerNewDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPeerNewDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushOvpnPeerNewInput<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushOvpnPeerNewInput::new(buf)
    }
    pub fn encode(&mut self) -> PushOvpnPeerNewInput<&mut Vec<u8>> {
        PushOvpnPeerNewInput::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOvpnPeerNewInput<RequestBuf<'r>> {
        PushOvpnPeerNewInput::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableOvpnPeerNewInput<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableOvpnPeerNewInput::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpPeerNewDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("ovpn".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableOvpnPeerNewInput<'buf>;
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
#[doc = "modify a remote peer\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushOvpnPeerSetInput::push_ifindex)\n- [.nested_peer()](PushOvpnPeerSetInput::nested_peer)\n\n"]
#[derive(Debug)]
pub struct OpPeerSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPeerSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushOvpnPeerSetInput<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushOvpnPeerSetInput::new(buf)
    }
    pub fn encode(&mut self) -> PushOvpnPeerSetInput<&mut Vec<u8>> {
        PushOvpnPeerSetInput::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOvpnPeerSetInput<RequestBuf<'r>> {
        PushOvpnPeerSetInput::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableOvpnPeerSetInput<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableOvpnPeerSetInput::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpPeerSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("ovpn".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableOvpnPeerSetInput<'buf>;
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
#[doc = "Retrieve data about existing remote peers (or a specific one)\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushOvpn::push_ifindex)\n\nReply attributes:\n- [.get_peer()](IterableOvpn::get_peer)\n\n"]
#[derive(Debug)]
pub struct OpPeerGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpPeerGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushOvpn<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushOvpn::new(buf)
    }
    pub fn encode(&mut self) -> PushOvpn<&mut Vec<u8>> {
        PushOvpn::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOvpn<RequestBuf<'r>> {
        PushOvpn::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableOvpn<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableOvpn::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpPeerGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("ovpn".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableOvpn<'buf>;
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
#[doc = "Retrieve data about existing remote peers (or a specific one)\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushOvpn::push_ifindex)\n- [.nested_peer()](PushOvpn::nested_peer)\n\nReply attributes:\n- [.get_peer()](IterableOvpn::get_peer)\n\n"]
#[derive(Debug)]
pub struct OpPeerGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPeerGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushOvpn<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushOvpn::new(buf)
    }
    pub fn encode(&mut self) -> PushOvpn<&mut Vec<u8>> {
        PushOvpn::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOvpn<RequestBuf<'r>> {
        PushOvpn::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableOvpn<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableOvpn::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpPeerGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("ovpn".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableOvpn<'buf>;
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
#[doc = "Delete existing remote peer\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushOvpnPeerDelInput::push_ifindex)\n- [.nested_peer()](PushOvpnPeerDelInput::nested_peer)\n\n"]
#[derive(Debug)]
pub struct OpPeerDelDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPeerDelDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushOvpnPeerDelInput<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushOvpnPeerDelInput::new(buf)
    }
    pub fn encode(&mut self) -> PushOvpnPeerDelInput<&mut Vec<u8>> {
        PushOvpnPeerDelInput::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOvpnPeerDelInput<RequestBuf<'r>> {
        PushOvpnPeerDelInput::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableOvpnPeerDelInput<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableOvpnPeerDelInput::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpPeerDelDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("ovpn".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableOvpnPeerDelInput<'buf>;
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
#[doc = "Add a cipher key for a specific peer\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushOvpn::push_ifindex)\n- [.nested_keyconf()](PushOvpn::nested_keyconf)\n\n"]
#[derive(Debug)]
pub struct OpKeyNewDo<'r> {
    request: Request<'r>,
}
impl<'r> OpKeyNewDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushOvpn<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushOvpn::new(buf)
    }
    pub fn encode(&mut self) -> PushOvpn<&mut Vec<u8>> {
        PushOvpn::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOvpn<RequestBuf<'r>> {
        PushOvpn::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableOvpn<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableOvpn::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 6u8;
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
impl NetlinkRequest for OpKeyNewDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("ovpn".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableOvpn<'buf>;
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
#[doc = "Retrieve non-sensitive data about peer key and cipher\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushOvpnKeyconfGet::push_ifindex)\n- [.nested_keyconf()](PushOvpnKeyconfGet::nested_keyconf)\n\nReply attributes:\n- [.get_keyconf()](IterableOvpnKeyconfGet::get_keyconf)\n\n"]
#[derive(Debug)]
pub struct OpKeyGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpKeyGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushOvpnKeyconfGet<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushOvpnKeyconfGet::new(buf)
    }
    pub fn encode(&mut self) -> PushOvpnKeyconfGet<&mut Vec<u8>> {
        PushOvpnKeyconfGet::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOvpnKeyconfGet<RequestBuf<'r>> {
        PushOvpnKeyconfGet::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableOvpnKeyconfGet<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableOvpnKeyconfGet::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 7u8;
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
impl NetlinkRequest for OpKeyGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("ovpn".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableOvpnKeyconfGet<'buf>;
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
#[doc = "Swap primary and secondary session keys for a specific peer\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushOvpnKeyconfSwapInput::push_ifindex)\n- [.nested_keyconf()](PushOvpnKeyconfSwapInput::nested_keyconf)\n\n"]
#[derive(Debug)]
pub struct OpKeySwapDo<'r> {
    request: Request<'r>,
}
impl<'r> OpKeySwapDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
    ) -> PushOvpnKeyconfSwapInput<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushOvpnKeyconfSwapInput::new(buf)
    }
    pub fn encode(&mut self) -> PushOvpnKeyconfSwapInput<&mut Vec<u8>> {
        PushOvpnKeyconfSwapInput::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOvpnKeyconfSwapInput<RequestBuf<'r>> {
        PushOvpnKeyconfSwapInput::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableOvpnKeyconfSwapInput<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableOvpnKeyconfSwapInput::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 8u8;
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
impl NetlinkRequest for OpKeySwapDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("ovpn".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableOvpnKeyconfSwapInput<'buf>;
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
#[doc = "Delete cipher key for a specific peer\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushOvpnKeyconfDelInput::push_ifindex)\n- [.nested_keyconf()](PushOvpnKeyconfDelInput::nested_keyconf)\n\n"]
#[derive(Debug)]
pub struct OpKeyDelDo<'r> {
    request: Request<'r>,
}
impl<'r> OpKeyDelDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
    ) -> PushOvpnKeyconfDelInput<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushOvpnKeyconfDelInput::new(buf)
    }
    pub fn encode(&mut self) -> PushOvpnKeyconfDelInput<&mut Vec<u8>> {
        PushOvpnKeyconfDelInput::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushOvpnKeyconfDelInput<RequestBuf<'r>> {
        PushOvpnKeyconfDelInput::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableOvpnKeyconfDelInput<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableOvpnKeyconfDelInput::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpKeyDelDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("ovpn".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableOvpnKeyconfDelInput<'buf>;
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
    #[doc = "Add a remote peer\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushOvpnPeerNewInput::push_ifindex)\n- [.nested_peer()](PushOvpnPeerNewInput::nested_peer)\n\n"]
    pub fn op_peer_new_do(self) -> OpPeerNewDo<'buf> {
        let mut res = OpPeerNewDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-peer-new-do", OpPeerNewDo::lookup);
        res
    }
    #[doc = "modify a remote peer\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushOvpnPeerSetInput::push_ifindex)\n- [.nested_peer()](PushOvpnPeerSetInput::nested_peer)\n\n"]
    pub fn op_peer_set_do(self) -> OpPeerSetDo<'buf> {
        let mut res = OpPeerSetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-peer-set-do", OpPeerSetDo::lookup);
        res
    }
    #[doc = "Retrieve data about existing remote peers (or a specific one)\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushOvpn::push_ifindex)\n\nReply attributes:\n- [.get_peer()](IterableOvpn::get_peer)\n\n"]
    pub fn op_peer_get_dump(self) -> OpPeerGetDump<'buf> {
        let mut res = OpPeerGetDump::new(self);
        res.request
            .do_writeback(res.protocol(), "op-peer-get-dump", OpPeerGetDump::lookup);
        res
    }
    #[doc = "Retrieve data about existing remote peers (or a specific one)\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushOvpn::push_ifindex)\n- [.nested_peer()](PushOvpn::nested_peer)\n\nReply attributes:\n- [.get_peer()](IterableOvpn::get_peer)\n\n"]
    pub fn op_peer_get_do(self) -> OpPeerGetDo<'buf> {
        let mut res = OpPeerGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-peer-get-do", OpPeerGetDo::lookup);
        res
    }
    #[doc = "Delete existing remote peer\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushOvpnPeerDelInput::push_ifindex)\n- [.nested_peer()](PushOvpnPeerDelInput::nested_peer)\n\n"]
    pub fn op_peer_del_do(self) -> OpPeerDelDo<'buf> {
        let mut res = OpPeerDelDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-peer-del-do", OpPeerDelDo::lookup);
        res
    }
    #[doc = "Add a cipher key for a specific peer\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushOvpn::push_ifindex)\n- [.nested_keyconf()](PushOvpn::nested_keyconf)\n\n"]
    pub fn op_key_new_do(self) -> OpKeyNewDo<'buf> {
        let mut res = OpKeyNewDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-key-new-do", OpKeyNewDo::lookup);
        res
    }
    #[doc = "Retrieve non-sensitive data about peer key and cipher\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushOvpnKeyconfGet::push_ifindex)\n- [.nested_keyconf()](PushOvpnKeyconfGet::nested_keyconf)\n\nReply attributes:\n- [.get_keyconf()](IterableOvpnKeyconfGet::get_keyconf)\n\n"]
    pub fn op_key_get_do(self) -> OpKeyGetDo<'buf> {
        let mut res = OpKeyGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-key-get-do", OpKeyGetDo::lookup);
        res
    }
    #[doc = "Swap primary and secondary session keys for a specific peer\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushOvpnKeyconfSwapInput::push_ifindex)\n- [.nested_keyconf()](PushOvpnKeyconfSwapInput::nested_keyconf)\n\n"]
    pub fn op_key_swap_do(self) -> OpKeySwapDo<'buf> {
        let mut res = OpKeySwapDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-key-swap-do", OpKeySwapDo::lookup);
        res
    }
    #[doc = "Delete cipher key for a specific peer\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_ifindex()](PushOvpnKeyconfDelInput::push_ifindex)\n- [.nested_keyconf()](PushOvpnKeyconfDelInput::nested_keyconf)\n\n"]
    pub fn op_key_del_do(self) -> OpKeyDelDo<'buf> {
        let mut res = OpKeyDelDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-key-del-do", OpKeyDelDo::lookup);
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableOvpn::get_peer;
        let _ = IterableOvpnKeyconfGet::get_keyconf;
        let _ = OpKeySwapNotif;
        let _ = OpPeerDelNotif;
        let _ = OpPeerFloatNotif;
        let _ = PushOvpn::<&mut Vec<u8>>::nested_keyconf;
        let _ = PushOvpn::<&mut Vec<u8>>::nested_peer;
        let _ = PushOvpn::<&mut Vec<u8>>::push_ifindex;
        let _ = PushOvpnKeyconfDelInput::<&mut Vec<u8>>::nested_keyconf;
        let _ = PushOvpnKeyconfDelInput::<&mut Vec<u8>>::push_ifindex;
        let _ = PushOvpnKeyconfGet::<&mut Vec<u8>>::nested_keyconf;
        let _ = PushOvpnKeyconfGet::<&mut Vec<u8>>::push_ifindex;
        let _ = PushOvpnKeyconfSwapInput::<&mut Vec<u8>>::nested_keyconf;
        let _ = PushOvpnKeyconfSwapInput::<&mut Vec<u8>>::push_ifindex;
        let _ = PushOvpnPeerDelInput::<&mut Vec<u8>>::nested_peer;
        let _ = PushOvpnPeerDelInput::<&mut Vec<u8>>::push_ifindex;
        let _ = PushOvpnPeerNewInput::<&mut Vec<u8>>::nested_peer;
        let _ = PushOvpnPeerNewInput::<&mut Vec<u8>>::push_ifindex;
        let _ = PushOvpnPeerSetInput::<&mut Vec<u8>>::nested_peer;
        let _ = PushOvpnPeerSetInput::<&mut Vec<u8>>::push_ifindex;
    }
}
