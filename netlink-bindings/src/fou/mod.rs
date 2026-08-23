#![doc = "Foo-over-UDP.\n"]
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
pub const PROTONAME: &str = "fou";
pub const PROTONAME_CSTR: &CStr = c"fou";
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum EncapType {
    Unspec = 0,
    Direct = 1,
    Gue = 2,
}
impl EncapType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            0 => Self::Unspec,
            1 => Self::Direct,
            2 => Self::Gue,
            _ => return None,
        })
    }
}
#[derive(Clone)]
pub enum Fou<'a> {
    Port(u16),
    Af(u8),
    Ipproto(u8),
    Type(u8),
    RemcsumNopartial(()),
    LocalV4(u32),
    LocalV6(&'a [u8]),
    PeerV4(u32),
    PeerV6(&'a [u8]),
    PeerPort(u16),
    Ifindex(i32),
}
impl<'a> IterableFou<'a> {
    pub fn get_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Fou::Port(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Fou",
            "Port",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_af(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Fou::Af(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Fou",
            "Af",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ipproto(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Fou::Ipproto(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Fou",
            "Ipproto",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_type(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Fou::Type(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Fou",
            "Type",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_remcsum_nopartial(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Fou::RemcsumNopartial(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Fou",
            "RemcsumNopartial",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_local_v4(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Fou::LocalV4(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Fou",
            "LocalV4",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_local_v6(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Fou::LocalV6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Fou",
            "LocalV6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_peer_v4(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Fou::PeerV4(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Fou",
            "PeerV4",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_peer_v6(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Fou::PeerV6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Fou",
            "PeerV6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_peer_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Fou::PeerPort(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Fou",
            "PeerPort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_ifindex(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Fou::Ifindex(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Fou",
            "Ifindex",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Fou<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableFou<'a> {
        IterableFou::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "Unspec",
            1u16 => "Port",
            2u16 => "Af",
            3u16 => "Ipproto",
            4u16 => "Type",
            5u16 => "RemcsumNopartial",
            6u16 => "LocalV4",
            7u16 => "LocalV6",
            8u16 => "PeerV4",
            9u16 => "PeerV6",
            10u16 => "PeerPort",
            11u16 => "Ifindex",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableFou<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableFou<'a> {
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
impl<'a> Iterator for IterableFou<'a> {
    type Item = Result<Fou<'a>, ErrorContext>;
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
                1u16 => Fou::Port({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Fou::Af({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Fou::Ipproto({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Fou::Type({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Fou::RemcsumNopartial(()),
                6u16 => Fou::LocalV4({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => Fou::LocalV6({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => Fou::PeerV4({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => Fou::PeerV6({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => Fou::PeerPort({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => Fou::Ifindex({
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
            "Fou",
            r#type.and_then(|t| Fou::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableFou<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Fou");
        let mut iter = IterableFou::with_loc(&[], self.orig_loc);
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
                Fou::Port(val) => fmt.field("Port", &val),
                Fou::Af(val) => fmt.field("Af", &val),
                Fou::Ipproto(val) => fmt.field("Ipproto", &val),
                Fou::Type(val) => fmt.field("Type", &val),
                Fou::RemcsumNopartial(val) => fmt.field("RemcsumNopartial", &val),
                Fou::LocalV4(val) => fmt.field("LocalV4", &val),
                Fou::LocalV6(val) => fmt.field("LocalV6", &FormatHexdump(val)),
                Fou::PeerV4(val) => fmt.field("PeerV4", &val),
                Fou::PeerV6(val) => fmt.field("PeerV6", &FormatHexdump(val)),
                Fou::PeerPort(val) => fmt.field("PeerPort", &val),
                Fou::Ifindex(val) => fmt.field("Ifindex", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableFou<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Fou", offset));
            return (stack, missing_type.and_then(|t| Fou::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Fou::Port(val) => {
                    if last_off == offset {
                        stack.push(("Port", last_off));
                        break;
                    }
                }
                Fou::Af(val) => {
                    if last_off == offset {
                        stack.push(("Af", last_off));
                        break;
                    }
                }
                Fou::Ipproto(val) => {
                    if last_off == offset {
                        stack.push(("Ipproto", last_off));
                        break;
                    }
                }
                Fou::Type(val) => {
                    if last_off == offset {
                        stack.push(("Type", last_off));
                        break;
                    }
                }
                Fou::RemcsumNopartial(val) => {
                    if last_off == offset {
                        stack.push(("RemcsumNopartial", last_off));
                        break;
                    }
                }
                Fou::LocalV4(val) => {
                    if last_off == offset {
                        stack.push(("LocalV4", last_off));
                        break;
                    }
                }
                Fou::LocalV6(val) => {
                    if last_off == offset {
                        stack.push(("LocalV6", last_off));
                        break;
                    }
                }
                Fou::PeerV4(val) => {
                    if last_off == offset {
                        stack.push(("PeerV4", last_off));
                        break;
                    }
                }
                Fou::PeerV6(val) => {
                    if last_off == offset {
                        stack.push(("PeerV6", last_off));
                        break;
                    }
                }
                Fou::PeerPort(val) => {
                    if last_off == offset {
                        stack.push(("PeerPort", last_off));
                        break;
                    }
                }
                Fou::Ifindex(val) => {
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
            stack.push(("Fou", cur));
        }
        (stack, None)
    }
}
pub struct PushFou<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushFou<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushFou<Prev> {
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
    pub fn push_port(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 1u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_af(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 2u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ipproto(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 3u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_type(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 4u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_remcsum_nopartial(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 5u16, 0 as u16);
        self
    }
    pub fn push_local_v4(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_local_v6(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 7u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_peer_v4(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_peer_v6(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 9u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_peer_port(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 10u16, 2 as u16);
        self.as_vec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_ifindex(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 11u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushFou<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Add port.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_port()](PushFou::push_port)\n- [.push_ipproto()](PushFou::push_ipproto)\n- [.push_type()](PushFou::push_type)\n- [.push_remcsum_nopartial()](PushFou::push_remcsum_nopartial)\n- [.push_local_v4()](PushFou::push_local_v4)\n- [.push_local_v6()](PushFou::push_local_v6)\n- [.push_peer_v4()](PushFou::push_peer_v4)\n- [.push_peer_v6()](PushFou::push_peer_v6)\n- [.push_peer_port()](PushFou::push_peer_port)\n- [.push_ifindex()](PushFou::push_ifindex)\n\n"]
#[derive(Debug)]
pub struct OpAddDo<'r> {
    request: Request<'r>,
}
impl<'r> OpAddDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushFou<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushFou::new(buf)
    }
    pub fn encode(&mut self) -> PushFou<&mut Vec<u8>> {
        PushFou::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushFou<RequestBuf<'r>> {
        PushFou::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableFou<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableFou::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpAddDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("fou".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableFou<'buf>;
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
#[doc = "Delete port.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_port()](PushFou::push_port)\n- [.push_af()](PushFou::push_af)\n- [.push_local_v4()](PushFou::push_local_v4)\n- [.push_local_v6()](PushFou::push_local_v6)\n- [.push_peer_v4()](PushFou::push_peer_v4)\n- [.push_peer_v6()](PushFou::push_peer_v6)\n- [.push_peer_port()](PushFou::push_peer_port)\n- [.push_ifindex()](PushFou::push_ifindex)\n\n"]
#[derive(Debug)]
pub struct OpDelDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDelDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushFou<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushFou::new(buf)
    }
    pub fn encode(&mut self) -> PushFou<&mut Vec<u8>> {
        PushFou::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushFou<RequestBuf<'r>> {
        PushFou::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableFou<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableFou::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpDelDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("fou".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableFou<'buf>;
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
#[doc = "Get tunnel info.\n\nReply attributes:\n- [.get_port()](IterableFou::get_port)\n- [.get_ipproto()](IterableFou::get_ipproto)\n- [.get_type()](IterableFou::get_type)\n- [.get_remcsum_nopartial()](IterableFou::get_remcsum_nopartial)\n- [.get_local_v4()](IterableFou::get_local_v4)\n- [.get_local_v6()](IterableFou::get_local_v6)\n- [.get_peer_v4()](IterableFou::get_peer_v4)\n- [.get_peer_v6()](IterableFou::get_peer_v6)\n- [.get_peer_port()](IterableFou::get_peer_port)\n- [.get_ifindex()](IterableFou::get_ifindex)\n\n"]
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
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushFou<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushFou::new(buf)
    }
    pub fn encode(&mut self) -> PushFou<&mut Vec<u8>> {
        PushFou::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushFou<RequestBuf<'r>> {
        PushFou::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableFou<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableFou::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("fou".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableFou<'buf>;
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
#[doc = "Get tunnel info.\n\nRequest attributes:\n- [.push_port()](PushFou::push_port)\n- [.push_af()](PushFou::push_af)\n- [.push_local_v4()](PushFou::push_local_v4)\n- [.push_local_v6()](PushFou::push_local_v6)\n- [.push_peer_v4()](PushFou::push_peer_v4)\n- [.push_peer_v6()](PushFou::push_peer_v6)\n- [.push_peer_port()](PushFou::push_peer_port)\n- [.push_ifindex()](PushFou::push_ifindex)\n\nReply attributes:\n- [.get_port()](IterableFou::get_port)\n- [.get_ipproto()](IterableFou::get_ipproto)\n- [.get_type()](IterableFou::get_type)\n- [.get_remcsum_nopartial()](IterableFou::get_remcsum_nopartial)\n- [.get_local_v4()](IterableFou::get_local_v4)\n- [.get_local_v6()](IterableFou::get_local_v6)\n- [.get_peer_v4()](IterableFou::get_peer_v4)\n- [.get_peer_v6()](IterableFou::get_peer_v6)\n- [.get_peer_port()](IterableFou::get_peer_port)\n- [.get_ifindex()](IterableFou::get_ifindex)\n\n"]
#[derive(Debug)]
pub struct OpGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushFou<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushFou::new(buf)
    }
    pub fn encode(&mut self) -> PushFou<&mut Vec<u8>> {
        PushFou::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushFou<RequestBuf<'r>> {
        PushFou::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableFou<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableFou::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("fou".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableFou<'buf>;
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
    #[doc = "Add port.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_port()](PushFou::push_port)\n- [.push_ipproto()](PushFou::push_ipproto)\n- [.push_type()](PushFou::push_type)\n- [.push_remcsum_nopartial()](PushFou::push_remcsum_nopartial)\n- [.push_local_v4()](PushFou::push_local_v4)\n- [.push_local_v6()](PushFou::push_local_v6)\n- [.push_peer_v4()](PushFou::push_peer_v4)\n- [.push_peer_v6()](PushFou::push_peer_v6)\n- [.push_peer_port()](PushFou::push_peer_port)\n- [.push_ifindex()](PushFou::push_ifindex)\n\n"]
    pub fn op_add_do(self) -> OpAddDo<'buf> {
        let mut res = OpAddDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-add-do", OpAddDo::lookup);
        res
    }
    #[doc = "Delete port.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_port()](PushFou::push_port)\n- [.push_af()](PushFou::push_af)\n- [.push_local_v4()](PushFou::push_local_v4)\n- [.push_local_v6()](PushFou::push_local_v6)\n- [.push_peer_v4()](PushFou::push_peer_v4)\n- [.push_peer_v6()](PushFou::push_peer_v6)\n- [.push_peer_port()](PushFou::push_peer_port)\n- [.push_ifindex()](PushFou::push_ifindex)\n\n"]
    pub fn op_del_do(self) -> OpDelDo<'buf> {
        let mut res = OpDelDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-del-do", OpDelDo::lookup);
        res
    }
    #[doc = "Get tunnel info.\n\nReply attributes:\n- [.get_port()](IterableFou::get_port)\n- [.get_ipproto()](IterableFou::get_ipproto)\n- [.get_type()](IterableFou::get_type)\n- [.get_remcsum_nopartial()](IterableFou::get_remcsum_nopartial)\n- [.get_local_v4()](IterableFou::get_local_v4)\n- [.get_local_v6()](IterableFou::get_local_v6)\n- [.get_peer_v4()](IterableFou::get_peer_v4)\n- [.get_peer_v6()](IterableFou::get_peer_v6)\n- [.get_peer_port()](IterableFou::get_peer_port)\n- [.get_ifindex()](IterableFou::get_ifindex)\n\n"]
    pub fn op_get_dump(self) -> OpGetDump<'buf> {
        let mut res = OpGetDump::new(self);
        res.request
            .do_writeback(res.protocol(), "op-get-dump", OpGetDump::lookup);
        res
    }
    #[doc = "Get tunnel info.\n\nRequest attributes:\n- [.push_port()](PushFou::push_port)\n- [.push_af()](PushFou::push_af)\n- [.push_local_v4()](PushFou::push_local_v4)\n- [.push_local_v6()](PushFou::push_local_v6)\n- [.push_peer_v4()](PushFou::push_peer_v4)\n- [.push_peer_v6()](PushFou::push_peer_v6)\n- [.push_peer_port()](PushFou::push_peer_port)\n- [.push_ifindex()](PushFou::push_ifindex)\n\nReply attributes:\n- [.get_port()](IterableFou::get_port)\n- [.get_ipproto()](IterableFou::get_ipproto)\n- [.get_type()](IterableFou::get_type)\n- [.get_remcsum_nopartial()](IterableFou::get_remcsum_nopartial)\n- [.get_local_v4()](IterableFou::get_local_v4)\n- [.get_local_v6()](IterableFou::get_local_v6)\n- [.get_peer_v4()](IterableFou::get_peer_v4)\n- [.get_peer_v6()](IterableFou::get_peer_v6)\n- [.get_peer_port()](IterableFou::get_peer_port)\n- [.get_ifindex()](IterableFou::get_ifindex)\n\n"]
    pub fn op_get_do(self) -> OpGetDo<'buf> {
        let mut res = OpGetDo::new(self);
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
        let _ = IterableFou::get_ifindex;
        let _ = IterableFou::get_ipproto;
        let _ = IterableFou::get_local_v4;
        let _ = IterableFou::get_local_v6;
        let _ = IterableFou::get_peer_port;
        let _ = IterableFou::get_peer_v4;
        let _ = IterableFou::get_peer_v6;
        let _ = IterableFou::get_port;
        let _ = IterableFou::get_remcsum_nopartial;
        let _ = IterableFou::get_type;
        let _ = PushFou::<&mut Vec<u8>>::push_af;
        let _ = PushFou::<&mut Vec<u8>>::push_ifindex;
        let _ = PushFou::<&mut Vec<u8>>::push_ipproto;
        let _ = PushFou::<&mut Vec<u8>>::push_local_v4;
        let _ = PushFou::<&mut Vec<u8>>::push_local_v6;
        let _ = PushFou::<&mut Vec<u8>>::push_peer_port;
        let _ = PushFou::<&mut Vec<u8>>::push_peer_v4;
        let _ = PushFou::<&mut Vec<u8>>::push_peer_v6;
        let _ = PushFou::<&mut Vec<u8>>::push_port;
        let _ = PushFou::<&mut Vec<u8>>::push_remcsum_nopartial;
        let _ = PushFou::<&mut Vec<u8>>::push_type;
    }
}
