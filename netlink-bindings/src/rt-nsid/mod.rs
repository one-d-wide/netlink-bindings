#![doc = "Network namespace id management over rtnetlink. Ids are relative to the\nnetwork namespace of the socket making the request, and are the values\nreported by e.g. IFLA_LINK_NETNSID.\n"]
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
pub const PROTONAME: &str = "rt-nsid";
pub const PROTONAME_CSTR: &CStr = c"rt-nsid";
pub const PROTONUM: u16 = 0u16;
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Rtgenmsg {
    pub family: u8,
    pub _pad: [u8; 3usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for Rtgenmsg {
    fn default() -> Self {
        Self::new()
    }
}
impl Rtgenmsg {
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
        const _: () = assert!(std::mem::size_of::<Rtgenmsg>() == 4usize);
        const _: () = assert!(std::mem::align_of::<Rtgenmsg>() == 1usize);
        4usize
    }
}
impl std::fmt::Debug for Rtgenmsg {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Rtgenmsg")
            .field("family", &self.family)
            .finish()
    }
}
#[derive(Clone)]
pub enum NsidAttrs {
    #[doc = "Id of the namespace, -1 if none is assigned\n"]
    Nsid(i32),
    #[doc = "Identify the target namespace by process id\n"]
    Pid(u32),
    #[doc = "Identify the target namespace by file descriptor\n"]
    Fd(u32),
    #[doc = "Query / assign from the point of view of this namespace\n"]
    TargetNsid(i32),
    CurrentNsid(i32),
}
impl<'a> IterableNsidAttrs<'a> {
    #[doc = "Id of the namespace, -1 if none is assigned\n"]
    pub fn get_nsid(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NsidAttrs::Nsid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NsidAttrs",
            "Nsid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Identify the target namespace by process id\n"]
    pub fn get_pid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NsidAttrs::Pid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NsidAttrs",
            "Pid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Identify the target namespace by file descriptor\n"]
    pub fn get_fd(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NsidAttrs::Fd(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NsidAttrs",
            "Fd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Query / assign from the point of view of this namespace\n"]
    pub fn get_target_nsid(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NsidAttrs::TargetNsid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NsidAttrs",
            "TargetNsid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_current_nsid(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NsidAttrs::CurrentNsid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NsidAttrs",
            "CurrentNsid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl NsidAttrs {
    pub fn new<'a>(buf: &'a [u8]) -> IterableNsidAttrs<'a> {
        IterableNsidAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Nsid",
            2u16 => "Pid",
            3u16 => "Fd",
            4u16 => "TargetNsid",
            5u16 => "CurrentNsid",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableNsidAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableNsidAttrs<'a> {
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
impl<'a> Iterator for IterableNsidAttrs<'a> {
    type Item = Result<NsidAttrs, ErrorContext>;
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
                1u16 => NsidAttrs::Nsid({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => NsidAttrs::Pid({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => NsidAttrs::Fd({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => NsidAttrs::TargetNsid({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => NsidAttrs::CurrentNsid({
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
            "NsidAttrs",
            r#type.and_then(|t| NsidAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableNsidAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("NsidAttrs");
        let mut iter = IterableNsidAttrs::with_loc(&[], self.orig_loc);
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
                NsidAttrs::Nsid(val) => fmt.field("Nsid", &val),
                NsidAttrs::Pid(val) => fmt.field("Pid", &val),
                NsidAttrs::Fd(val) => fmt.field("Fd", &val),
                NsidAttrs::TargetNsid(val) => fmt.field("TargetNsid", &val),
                NsidAttrs::CurrentNsid(val) => fmt.field("CurrentNsid", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableNsidAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("NsidAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| NsidAttrs::attr_from_type(t)),
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
                NsidAttrs::Nsid(val) => {
                    if last_off == offset {
                        stack.push(("Nsid", last_off));
                        break;
                    }
                }
                NsidAttrs::Pid(val) => {
                    if last_off == offset {
                        stack.push(("Pid", last_off));
                        break;
                    }
                }
                NsidAttrs::Fd(val) => {
                    if last_off == offset {
                        stack.push(("Fd", last_off));
                        break;
                    }
                }
                NsidAttrs::TargetNsid(val) => {
                    if last_off == offset {
                        stack.push(("TargetNsid", last_off));
                        break;
                    }
                }
                NsidAttrs::CurrentNsid(val) => {
                    if last_off == offset {
                        stack.push(("CurrentNsid", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("NsidAttrs", cur));
        }
        (stack, None)
    }
}
pub struct PushNsidAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushNsidAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushNsidAttrs<Prev> {
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
    #[doc = "Id of the namespace, -1 if none is assigned\n"]
    pub fn push_nsid(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Identify the target namespace by process id\n"]
    pub fn push_pid(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Identify the target namespace by file descriptor\n"]
    pub fn push_fd(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Query / assign from the point of view of this namespace\n"]
    pub fn push_target_nsid(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_current_nsid(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushNsidAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Assign an id to a network namespace\n\nRequest attributes:\n- [.push_nsid()](PushNsidAttrs::push_nsid)\n- [.push_pid()](PushNsidAttrs::push_pid)\n- [.push_fd()](PushNsidAttrs::push_fd)\n- [.push_target_nsid()](PushNsidAttrs::push_target_nsid)\n\n"]
#[derive(Debug)]
pub struct OpNewnsidDo<'r> {
    request: Request<'r>,
}
impl<'r> OpNewnsidDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Rtgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Rtgenmsg,
    ) -> PushNsidAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushNsidAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushNsidAttrs<&mut Vec<u8>> {
        PushNsidAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushNsidAttrs<RequestBuf<'r>> {
        PushNsidAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Rtgenmsg, IterableNsidAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Rtgenmsg::len()));
        (
            Rtgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableNsidAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Rtgenmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Rtgenmsg {
        let pos = self.request.pos;
        Rtgenmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Rtgenmsg {
        let pos = self.request.pos;
        Rtgenmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpNewnsidDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 88u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Rtgenmsg, IterableNsidAttrs<'buf>);
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
#[doc = "Get / dump network namespace ids\n\nRequest attributes:\n- [.push_target_nsid()](PushNsidAttrs::push_target_nsid)\n\nReply attributes:\n- [.get_nsid()](IterableNsidAttrs::get_nsid)\n- [.get_current_nsid()](IterableNsidAttrs::get_current_nsid)\n\n"]
#[derive(Debug)]
pub struct OpGetnsidDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetnsidDump<'r> {
    pub fn new(mut request: Request<'r>, header: &Rtgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Rtgenmsg,
    ) -> PushNsidAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushNsidAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushNsidAttrs<&mut Vec<u8>> {
        PushNsidAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushNsidAttrs<RequestBuf<'r>> {
        PushNsidAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Rtgenmsg, IterableNsidAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Rtgenmsg::len()));
        (
            Rtgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableNsidAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Rtgenmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Rtgenmsg {
        let pos = self.request.pos;
        Rtgenmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Rtgenmsg {
        let pos = self.request.pos;
        Rtgenmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetnsidDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 90u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Rtgenmsg, IterableNsidAttrs<'buf>);
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
#[doc = "Get / dump network namespace ids\n\nRequest attributes:\n- [.push_nsid()](PushNsidAttrs::push_nsid)\n- [.push_pid()](PushNsidAttrs::push_pid)\n- [.push_fd()](PushNsidAttrs::push_fd)\n- [.push_target_nsid()](PushNsidAttrs::push_target_nsid)\n\nReply attributes:\n- [.get_nsid()](IterableNsidAttrs::get_nsid)\n- [.get_current_nsid()](IterableNsidAttrs::get_current_nsid)\n\n"]
#[derive(Debug)]
pub struct OpGetnsidDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetnsidDo<'r> {
    pub fn new(mut request: Request<'r>, header: &Rtgenmsg) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Rtgenmsg,
    ) -> PushNsidAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushNsidAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushNsidAttrs<&mut Vec<u8>> {
        PushNsidAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushNsidAttrs<RequestBuf<'r>> {
        PushNsidAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Rtgenmsg, IterableNsidAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Rtgenmsg::len()));
        (
            Rtgenmsg::new_from_slice(header).unwrap_or_default(),
            IterableNsidAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Rtgenmsg) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Rtgenmsg {
        let pos = self.request.pos;
        Rtgenmsg::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Rtgenmsg {
        let pos = self.request.pos;
        Rtgenmsg::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpGetnsidDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 90u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Rtgenmsg, IterableNsidAttrs<'buf>);
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
#[derive(Debug)]
pub struct ChainedFinal<'a> {
    inner: Chained<'a>,
}
#[derive(Debug)]
pub struct Chained<'a> {
    buf: RequestBuf<'a>,
    first_seq: u32,
    lookups: Vec<(&'static str, LookupFn)>,
    last_header_offset: usize,
    last_kind: Option<RequestInfo>,
}
impl<'a> ChainedFinal<'a> {
    pub fn into_chained(self) -> Chained<'a> {
        self.inner
    }
    pub fn buf(&self) -> &Vec<u8> {
        self.inner.buf()
    }
    pub fn buf_mut(&mut self) -> &mut Vec<u8> {
        self.inner.buf_mut()
    }
    fn get_index(&self, seq: u32) -> Option<u32> {
        let min = self.inner.first_seq;
        let max = min.wrapping_add(self.inner.lookups.len() as u32);
        return if min <= max {
            (min..max).contains(&seq).then(|| seq - min)
        } else if min <= seq {
            Some(seq - min)
        } else if seq < max {
            Some(u32::MAX - min + seq)
        } else {
            None
        };
    }
}
impl crate::traits::NetlinkChained for ChainedFinal<'_> {
    fn protonum(&self) -> u16 {
        PROTONUM
    }
    fn payload(&self) -> &[u8] {
        self.buf()
    }
    fn chain_len(&self) -> usize {
        self.inner.lookups.len()
    }
    fn get_index(&self, seq: u32) -> Option<usize> {
        self.get_index(seq).map(|n| n as usize)
    }
    fn name(&self, index: usize) -> &'static str {
        self.inner.lookups[index].0
    }
    fn lookup(&self, index: usize) -> LookupFn {
        self.inner.lookups[index].1
    }
}
impl Chained<'static> {
    pub fn new(first_seq: u32) -> Self {
        Self::new_from_buf(Vec::new(), first_seq)
    }
    pub fn new_from_buf(mut buf: Vec<u8>, first_seq: u32) -> Self {
        buf.clear();
        Self {
            buf: RequestBuf::Own(buf),
            first_seq,
            lookups: Vec::new(),
            last_header_offset: 0,
            last_kind: None,
        }
    }
    pub fn into_buf(self) -> Vec<u8> {
        match self.buf {
            RequestBuf::Own(buf) => buf,
            _ => unreachable!(),
        }
    }
}
impl<'a> Chained<'a> {
    pub fn new_with_buf(buf: &'a mut Vec<u8>, first_seq: u32) -> Self {
        buf.clear();
        Self {
            buf: RequestBuf::Ref(buf),
            first_seq,
            lookups: Vec::new(),
            last_header_offset: 0,
            last_kind: None,
        }
    }
    pub fn finalize(mut self) -> ChainedFinal<'a> {
        self.update_header();
        ChainedFinal { inner: self }
    }
    pub fn request(&mut self) -> Request<'_> {
        self.update_header();
        self.last_header_offset = self.buf().len();
        self.buf_mut().extend_from_slice(Nlmsghdr::new().as_slice());
        let mut request = Request::new_extend(self.buf.buf_mut());
        self.last_kind = None;
        request.writeback = Some(&mut self.last_kind);
        request
    }
    pub fn buf(&self) -> &Vec<u8> {
        self.buf.buf()
    }
    pub fn buf_mut(&mut self) -> &mut Vec<u8> {
        self.buf.buf_mut()
    }
    fn update_header(&mut self) {
        let Some(RequestInfo {
            protocol,
            flags,
            name,
            lookup,
        }) = self.last_kind
        else {
            if !self.buf().is_empty() {
                assert_eq!(self.last_header_offset + Nlmsghdr::len(), self.buf().len());
                self.buf.buf_mut().truncate(self.last_header_offset);
            }
            return;
        };
        let header_offset = self.last_header_offset;
        let request_type = match protocol {
            Protocol::Raw { request_type, .. } => request_type,
            Protocol::Generic(_) => unreachable!(),
        };
        let index = self.lookups.len();
        let seq = self.first_seq.wrapping_add(index as u32);
        self.lookups.push((name, lookup));
        let buf = self.buf_mut();
        align(buf);
        let header = Nlmsghdr {
            len: (buf.len() - header_offset) as u32,
            r#type: request_type,
            flags: flags | consts::NLM_F_REQUEST as u16 | consts::NLM_F_ACK as u16,
            seq,
            pid: 0,
        };
        buf[header_offset..(header_offset + 16)].clone_from_slice(header.as_slice());
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
    #[doc = "Assign an id to a network namespace\n\nRequest attributes:\n- [.push_nsid()](PushNsidAttrs::push_nsid)\n- [.push_pid()](PushNsidAttrs::push_pid)\n- [.push_fd()](PushNsidAttrs::push_fd)\n- [.push_target_nsid()](PushNsidAttrs::push_target_nsid)\n\n"]
    pub fn op_newnsid_do(self, header: &Rtgenmsg) -> OpNewnsidDo<'buf> {
        let mut res = OpNewnsidDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-newnsid-do", OpNewnsidDo::lookup);
        res
    }
    #[doc = "Get / dump network namespace ids\n\nRequest attributes:\n- [.push_target_nsid()](PushNsidAttrs::push_target_nsid)\n\nReply attributes:\n- [.get_nsid()](IterableNsidAttrs::get_nsid)\n- [.get_current_nsid()](IterableNsidAttrs::get_current_nsid)\n\n"]
    pub fn op_getnsid_dump(self, header: &Rtgenmsg) -> OpGetnsidDump<'buf> {
        let mut res = OpGetnsidDump::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getnsid-dump", OpGetnsidDump::lookup);
        res
    }
    #[doc = "Get / dump network namespace ids\n\nRequest attributes:\n- [.push_nsid()](PushNsidAttrs::push_nsid)\n- [.push_pid()](PushNsidAttrs::push_pid)\n- [.push_fd()](PushNsidAttrs::push_fd)\n- [.push_target_nsid()](PushNsidAttrs::push_target_nsid)\n\nReply attributes:\n- [.get_nsid()](IterableNsidAttrs::get_nsid)\n- [.get_current_nsid()](IterableNsidAttrs::get_current_nsid)\n\n"]
    pub fn op_getnsid_do(self, header: &Rtgenmsg) -> OpGetnsidDo<'buf> {
        let mut res = OpGetnsidDo::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-getnsid-do", OpGetnsidDo::lookup);
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableNsidAttrs::get_current_nsid;
        let _ = IterableNsidAttrs::get_nsid;
        let _ = PushNsidAttrs::<&mut Vec<u8>>::push_fd;
        let _ = PushNsidAttrs::<&mut Vec<u8>>::push_nsid;
        let _ = PushNsidAttrs::<&mut Vec<u8>>::push_pid;
        let _ = PushNsidAttrs::<&mut Vec<u8>>::push_target_nsid;
    }
}
