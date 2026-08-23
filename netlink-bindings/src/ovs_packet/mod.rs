#![doc = "OVS packet execution over generic netlink.\n\nOnly OVS_PACKET_CMD_EXECUTE is exposed as a genl operation.\nOVS_PACKET_CMD_MISS and OVS_PACKET_CMD_ACTION are kernel-to-userspace\nupcalls sent via genlmsg_unicast() to the vport\\'s upcall_pid and have\nno associated genl_ops or multicast group.\n\nSeveral attributes in the attribute set (userdata, egress-tun-key, len)\nexist for the upcall path and are not used by the EXECUTE operation. For\nEXECUTE, packet, key, and actions are mandatory (kernel returns -EINVAL\nwithout them).\n"]
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
pub const PROTONAME: &str = "ovs_packet";
pub const PROTONAME_CSTR: &CStr = c"ovs_packet";
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
#[derive(Clone)]
pub enum Packet<'a> {
    #[doc = "Packet data, from the start of the Ethernet header.\n"]
    Packet(&'a [u8]),
    #[doc = "Nested [OVS_KEY_ATTR]()\\* attributes, extracted flow key. Defined as\nbinary because the key attribute-set belongs to the ovs_flow family\nspec; cross-spec references are not supported.\n"]
    Key(&'a [u8]),
    #[doc = "Nested [OVS_ACTION_ATTR]()\\* attributes. Defined as binary for the same\nreason as key.\n"]
    Actions(&'a [u8]),
    #[doc = "Opaque userspace cookie from OVS_USERSPACE_ATTR_USERDATA.\n"]
    Userdata(&'a [u8]),
    #[doc = "Nested [OVS_TUNNEL_KEY_ATTR]()\\* for output tunnel metadata.\n"]
    EgressTunKey(&'a [u8]),
    #[doc = "Packet operation is a feature probe, error logging suppressed.\n"]
    Probe(()),
    #[doc = "Maximum received IP fragment size.\n"]
    Mru(u16),
    #[doc = "Packet size before truncation.\n"]
    Len(u32),
    #[doc = "Packet hash, low 32 bits are skb hash, upper bits are flags.\n"]
    Hash(u64),
    #[doc = "Netlink PID to use for upcalls during EXECUTE processing.\n"]
    UpcallPid(u32),
}
impl<'a> IterablePacket<'a> {
    #[doc = "Packet data, from the start of the Ethernet header.\n"]
    pub fn get_packet(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Packet::Packet(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Packet",
            "Packet",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Nested [OVS_KEY_ATTR]()\\* attributes, extracted flow key. Defined as\nbinary because the key attribute-set belongs to the ovs_flow family\nspec; cross-spec references are not supported.\n"]
    pub fn get_key(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Packet::Key(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Packet",
            "Key",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Nested [OVS_ACTION_ATTR]()\\* attributes. Defined as binary for the same\nreason as key.\n"]
    pub fn get_actions(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Packet::Actions(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Packet",
            "Actions",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Opaque userspace cookie from OVS_USERSPACE_ATTR_USERDATA.\n"]
    pub fn get_userdata(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Packet::Userdata(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Packet",
            "Userdata",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Nested [OVS_TUNNEL_KEY_ATTR]()\\* for output tunnel metadata.\n"]
    pub fn get_egress_tun_key(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Packet::EgressTunKey(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Packet",
            "EgressTunKey",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Packet operation is a feature probe, error logging suppressed.\n"]
    pub fn get_probe(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Packet::Probe(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Packet",
            "Probe",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Maximum received IP fragment size.\n"]
    pub fn get_mru(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Packet::Mru(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Packet",
            "Mru",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Packet size before truncation.\n"]
    pub fn get_len(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Packet::Len(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Packet",
            "Len",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Packet hash, low 32 bits are skb hash, upper bits are flags.\n"]
    pub fn get_hash(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Packet::Hash(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Packet",
            "Hash",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Netlink PID to use for upcalls during EXECUTE processing.\n"]
    pub fn get_upcall_pid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Packet::UpcallPid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Packet",
            "UpcallPid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Packet<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePacket<'a> {
        IterablePacket::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Packet",
            2u16 => "Key",
            3u16 => "Actions",
            4u16 => "Userdata",
            5u16 => "EgressTunKey",
            6u16 => "Unused1",
            7u16 => "Unused2",
            8u16 => "Probe",
            9u16 => "Mru",
            10u16 => "Len",
            11u16 => "Hash",
            12u16 => "UpcallPid",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePacket<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePacket<'a> {
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
impl<'a> Iterator for IterablePacket<'a> {
    type Item = Result<Packet<'a>, ErrorContext>;
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
                1u16 => Packet::Packet({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Packet::Key({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Packet::Actions({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Packet::Userdata({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Packet::EgressTunKey({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => Packet::Probe(()),
                9u16 => Packet::Mru({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => Packet::Len({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => Packet::Hash({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => Packet::UpcallPid({
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
            "Packet",
            r#type.and_then(|t| Packet::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterablePacket<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Packet");
        let mut iter = IterablePacket::with_loc(&[], self.orig_loc);
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
                Packet::Packet(val) => fmt.field("Packet", &FormatHexdump(val)),
                Packet::Key(val) => fmt.field("Key", &FormatHexdump(val)),
                Packet::Actions(val) => fmt.field("Actions", &FormatHexdump(val)),
                Packet::Userdata(val) => fmt.field("Userdata", &FormatHexdump(val)),
                Packet::EgressTunKey(val) => fmt.field("EgressTunKey", &FormatHexdump(val)),
                Packet::Probe(val) => fmt.field("Probe", &val),
                Packet::Mru(val) => fmt.field("Mru", &val),
                Packet::Len(val) => fmt.field("Len", &val),
                Packet::Hash(val) => fmt.field("Hash", &val),
                Packet::UpcallPid(val) => fmt.field("UpcallPid", &val),
            };
        }
        fmt.finish()
    }
}
impl IterablePacket<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Packet", offset));
            return (stack, missing_type.and_then(|t| Packet::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Packet::Packet(val) => {
                    if last_off == offset {
                        stack.push(("Packet", last_off));
                        break;
                    }
                }
                Packet::Key(val) => {
                    if last_off == offset {
                        stack.push(("Key", last_off));
                        break;
                    }
                }
                Packet::Actions(val) => {
                    if last_off == offset {
                        stack.push(("Actions", last_off));
                        break;
                    }
                }
                Packet::Userdata(val) => {
                    if last_off == offset {
                        stack.push(("Userdata", last_off));
                        break;
                    }
                }
                Packet::EgressTunKey(val) => {
                    if last_off == offset {
                        stack.push(("EgressTunKey", last_off));
                        break;
                    }
                }
                Packet::Probe(val) => {
                    if last_off == offset {
                        stack.push(("Probe", last_off));
                        break;
                    }
                }
                Packet::Mru(val) => {
                    if last_off == offset {
                        stack.push(("Mru", last_off));
                        break;
                    }
                }
                Packet::Len(val) => {
                    if last_off == offset {
                        stack.push(("Len", last_off));
                        break;
                    }
                }
                Packet::Hash(val) => {
                    if last_off == offset {
                        stack.push(("Hash", last_off));
                        break;
                    }
                }
                Packet::UpcallPid(val) => {
                    if last_off == offset {
                        stack.push(("UpcallPid", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Packet", cur));
        }
        (stack, None)
    }
}
pub struct PushPacket<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushPacket<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushPacket<Prev> {
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
    #[doc = "Packet data, from the start of the Ethernet header.\n"]
    pub fn push_packet(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "Nested [OVS_KEY_ATTR]()\\* attributes, extracted flow key. Defined as\nbinary because the key attribute-set belongs to the ovs_flow family\nspec; cross-spec references are not supported.\n"]
    pub fn push_key(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "Nested [OVS_ACTION_ATTR]()\\* attributes. Defined as binary for the same\nreason as key.\n"]
    pub fn push_actions(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "Opaque userspace cookie from OVS_USERSPACE_ATTR_USERDATA.\n"]
    pub fn push_userdata(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 4u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "Nested [OVS_TUNNEL_KEY_ATTR]()\\* for output tunnel metadata.\n"]
    pub fn push_egress_tun_key(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 5u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    #[doc = "Packet operation is a feature probe, error logging suppressed.\n"]
    pub fn push_probe(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 8u16, 0 as u16);
        self
    }
    #[doc = "Maximum received IP fragment size.\n"]
    pub fn push_mru(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 9u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Packet size before truncation.\n"]
    pub fn push_len(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Packet hash, low 32 bits are skb hash, upper bits are flags.\n"]
    pub fn push_hash(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 11u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Netlink PID to use for upcalls during EXECUTE processing.\n"]
    pub fn push_upcall_pid(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 12u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushPacket<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Apply actions to a packet.\n\nRequest attributes:\n- [.push_packet()](PushPacket::push_packet)\n- [.push_key()](PushPacket::push_key)\n- [.push_actions()](PushPacket::push_actions)\n- [.push_probe()](PushPacket::push_probe)\n- [.push_mru()](PushPacket::push_mru)\n- [.push_hash()](PushPacket::push_hash)\n- [.push_upcall_pid()](PushPacket::push_upcall_pid)\n\n"]
#[derive(Debug)]
pub struct OpExecuteDo<'r> {
    request: Request<'r>,
}
impl<'r> OpExecuteDo<'r> {
    pub fn new(mut request: Request<'r>, header: &OvsHeader) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &OvsHeader,
    ) -> PushPacket<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushPacket::new(buf)
    }
    pub fn encode(&mut self) -> PushPacket<&mut Vec<u8>> {
        PushPacket::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushPacket<RequestBuf<'r>> {
        PushPacket::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (OvsHeader, IterablePacket<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(OvsHeader::len()));
        (
            OvsHeader::new_from_slice(header).unwrap_or_default(),
            IterablePacket::with_loc(attrs, buf.as_ptr() as usize),
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
impl NetlinkRequest for OpExecuteDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("ovs_packet".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (OvsHeader, IterablePacket<'buf>);
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
    #[doc = "Apply actions to a packet.\n\nRequest attributes:\n- [.push_packet()](PushPacket::push_packet)\n- [.push_key()](PushPacket::push_key)\n- [.push_actions()](PushPacket::push_actions)\n- [.push_probe()](PushPacket::push_probe)\n- [.push_mru()](PushPacket::push_mru)\n- [.push_hash()](PushPacket::push_hash)\n- [.push_upcall_pid()](PushPacket::push_upcall_pid)\n\n"]
    pub fn op_execute_do(self, header: &OvsHeader) -> OpExecuteDo<'buf> {
        let mut res = OpExecuteDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-execute-do", OpExecuteDo::lookup);
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = PushPacket::<&mut Vec<u8>>::push_actions;
        let _ = PushPacket::<&mut Vec<u8>>::push_hash;
        let _ = PushPacket::<&mut Vec<u8>>::push_key;
        let _ = PushPacket::<&mut Vec<u8>>::push_mru;
        let _ = PushPacket::<&mut Vec<u8>>::push_packet;
        let _ = PushPacket::<&mut Vec<u8>>::push_probe;
        let _ = PushPacket::<&mut Vec<u8>>::push_upcall_pid;
    }
}
