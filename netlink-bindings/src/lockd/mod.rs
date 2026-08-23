#![doc = "lockd configuration over generic netlink\n"]
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
pub const PROTONAME: &str = "lockd";
pub const PROTONAME_CSTR: &CStr = c"lockd";
#[derive(Clone)]
pub enum Server {
    Gracetime(u32),
    TcpPort(u16),
    UdpPort(u16),
}
impl<'a> IterableServer<'a> {
    pub fn get_gracetime(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Server::Gracetime(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Server",
            "Gracetime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_tcp_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Server::TcpPort(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Server",
            "TcpPort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_udp_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Server::UdpPort(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Server",
            "UdpPort",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Server {
    pub fn new<'a>(buf: &'a [u8]) -> IterableServer<'a> {
        IterableServer::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Gracetime",
            2u16 => "TcpPort",
            3u16 => "UdpPort",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableServer<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableServer<'a> {
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
impl<'a> Iterator for IterableServer<'a> {
    type Item = Result<Server, ErrorContext>;
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
                1u16 => Server::Gracetime({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Server::TcpPort({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Server::UdpPort({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Server",
            r#type.and_then(|t| Server::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableServer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Server");
        let mut iter = IterableServer::with_loc(&[], self.orig_loc);
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
                Server::Gracetime(val) => fmt.field("Gracetime", &val),
                Server::TcpPort(val) => fmt.field("TcpPort", &val),
                Server::UdpPort(val) => fmt.field("UdpPort", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableServer<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Server", offset));
            return (stack, missing_type.and_then(|t| Server::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Server::Gracetime(val) => {
                    if last_off == offset {
                        stack.push(("Gracetime", last_off));
                        break;
                    }
                }
                Server::TcpPort(val) => {
                    if last_off == offset {
                        stack.push(("TcpPort", last_off));
                        break;
                    }
                }
                Server::UdpPort(val) => {
                    if last_off == offset {
                        stack.push(("UdpPort", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Server", cur));
        }
        (stack, None)
    }
}
pub struct PushServer<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushServer<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushServer<Prev> {
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
    pub fn push_gracetime(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_tcp_port(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 2u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_udp_port(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 3u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushServer<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "set the lockd server parameters\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_gracetime()](PushServer::push_gracetime)\n- [.push_tcp_port()](PushServer::push_tcp_port)\n- [.push_udp_port()](PushServer::push_udp_port)\n\n"]
#[derive(Debug)]
pub struct OpServerSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpServerSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushServer<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushServer::new(buf)
    }
    pub fn encode(&mut self) -> PushServer<&mut Vec<u8>> {
        PushServer::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushServer<RequestBuf<'r>> {
        PushServer::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableServer<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableServer::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpServerSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("lockd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableServer<'buf>;
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
#[doc = "get the lockd server parameters\n\nReply attributes:\n- [.get_gracetime()](IterableServer::get_gracetime)\n- [.get_tcp_port()](IterableServer::get_tcp_port)\n- [.get_udp_port()](IterableServer::get_udp_port)\n\n"]
#[derive(Debug)]
pub struct OpServerGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpServerGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushServer<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushServer::new(buf)
    }
    pub fn encode(&mut self) -> PushServer<&mut Vec<u8>> {
        PushServer::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushServer<RequestBuf<'r>> {
        PushServer::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableServer<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableServer::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpServerGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("lockd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableServer<'buf>;
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
    #[doc = "set the lockd server parameters\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_gracetime()](PushServer::push_gracetime)\n- [.push_tcp_port()](PushServer::push_tcp_port)\n- [.push_udp_port()](PushServer::push_udp_port)\n\n"]
    pub fn op_server_set_do(self) -> OpServerSetDo<'buf> {
        let mut res = OpServerSetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-server-set-do", OpServerSetDo::lookup);
        res
    }
    #[doc = "get the lockd server parameters\n\nReply attributes:\n- [.get_gracetime()](IterableServer::get_gracetime)\n- [.get_tcp_port()](IterableServer::get_tcp_port)\n- [.get_udp_port()](IterableServer::get_udp_port)\n\n"]
    pub fn op_server_get_do(self) -> OpServerGetDo<'buf> {
        let mut res = OpServerGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-server-get-do", OpServerGetDo::lookup);
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableServer::get_gracetime;
        let _ = IterableServer::get_tcp_port;
        let _ = IterableServer::get_udp_port;
        let _ = PushServer::<&mut Vec<u8>>::push_gracetime;
        let _ = PushServer::<&mut Vec<u8>>::push_tcp_port;
        let _ = PushServer::<&mut Vec<u8>>::push_udp_port;
    }
}
