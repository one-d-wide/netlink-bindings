#![doc = "DRM RAS (Reliability, Availability, Serviceability) over Generic\nNetlink. Provides a standardized mechanism for DRM drivers to register\n\\\"nodes\\\" representing hardware/software components capable of reporting\nerror counters. Userspace tools can query the list of nodes or\nindividual error counters via the Generic Netlink interface.\n"]
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
pub const PROTONAME: &str = "drm-ras";
pub const PROTONAME_CSTR: &CStr = c"drm-ras";
#[doc = "Type of the node. Currently, only error-counter nodes are supported,\nwhich expose reliability counters for a hardware/software component.\n"]
#[doc = "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Debug, Clone, Copy)]
pub enum NodeType {
    ErrorCounter = 1,
}
impl NodeType {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            1 => Self::ErrorCounter,
            _ => return None,
        })
    }
}
#[derive(Clone)]
pub enum NodeAttrs<'a> {
    #[doc = "Unique identifier for the node. Assigned dynamically by the DRM RAS core\nupon registration.\n"]
    NodeId(u32),
    #[doc = "Device name chosen by the driver at registration. Can be a PCI BDF,\nUUID, or module name if unique.\n"]
    DeviceName(&'a CStr),
    #[doc = "Node name chosen by the driver at registration. Can be an IP block name,\nor any name that identifies the RAS node inside the device.\n"]
    NodeName(&'a CStr),
    #[doc = "Type of this node, identifying its function.\n\nAssociated type: [`NodeType`] (enum)"]
    NodeType(u32),
}
impl<'a> IterableNodeAttrs<'a> {
    #[doc = "Unique identifier for the node. Assigned dynamically by the DRM RAS core\nupon registration.\n"]
    pub fn get_node_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NodeAttrs::NodeId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NodeAttrs",
            "NodeId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Device name chosen by the driver at registration. Can be a PCI BDF,\nUUID, or module name if unique.\n"]
    pub fn get_device_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NodeAttrs::DeviceName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NodeAttrs",
            "DeviceName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Node name chosen by the driver at registration. Can be an IP block name,\nor any name that identifies the RAS node inside the device.\n"]
    pub fn get_node_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NodeAttrs::NodeName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NodeAttrs",
            "NodeName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Type of this node, identifying its function.\n\nAssociated type: [`NodeType`] (enum)"]
    pub fn get_node_type(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(NodeAttrs::NodeType(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "NodeAttrs",
            "NodeType",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl NodeAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableNodeAttrs<'a> {
        IterableNodeAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "NodeId",
            2u16 => "DeviceName",
            3u16 => "NodeName",
            4u16 => "NodeType",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableNodeAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableNodeAttrs<'a> {
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
impl<'a> Iterator for IterableNodeAttrs<'a> {
    type Item = Result<NodeAttrs<'a>, ErrorContext>;
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
                1u16 => NodeAttrs::NodeId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => NodeAttrs::DeviceName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => NodeAttrs::NodeName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => NodeAttrs::NodeType({
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
            "NodeAttrs",
            r#type.and_then(|t| NodeAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableNodeAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("NodeAttrs");
        let mut iter = IterableNodeAttrs::with_loc(&[], self.orig_loc);
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
                NodeAttrs::NodeId(val) => fmt.field("NodeId", &val),
                NodeAttrs::DeviceName(val) => fmt.field("DeviceName", &val),
                NodeAttrs::NodeName(val) => fmt.field("NodeName", &val),
                NodeAttrs::NodeType(val) => {
                    fmt.field("NodeType", &FormatEnum(val.into(), NodeType::from_value))
                }
            };
        }
        fmt.finish()
    }
}
impl IterableNodeAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("NodeAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| NodeAttrs::attr_from_type(t)),
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
                NodeAttrs::NodeId(val) => {
                    if last_off == offset {
                        stack.push(("NodeId", last_off));
                        break;
                    }
                }
                NodeAttrs::DeviceName(val) => {
                    if last_off == offset {
                        stack.push(("DeviceName", last_off));
                        break;
                    }
                }
                NodeAttrs::NodeName(val) => {
                    if last_off == offset {
                        stack.push(("NodeName", last_off));
                        break;
                    }
                }
                NodeAttrs::NodeType(val) => {
                    if last_off == offset {
                        stack.push(("NodeType", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("NodeAttrs", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ErrorCounterAttrs<'a> {
    #[doc = "Node ID targeted by this error counter operation.\n"]
    NodeId(u32),
    #[doc = "Unique identifier for a specific error counter within an node.\n"]
    ErrorId(u32),
    #[doc = "Name of the error.\n"]
    ErrorName(&'a CStr),
    #[doc = "Current value of the requested error counter.\n"]
    ErrorValue(u32),
}
impl<'a> IterableErrorCounterAttrs<'a> {
    #[doc = "Node ID targeted by this error counter operation.\n"]
    pub fn get_node_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ErrorCounterAttrs::NodeId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ErrorCounterAttrs",
            "NodeId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Unique identifier for a specific error counter within an node.\n"]
    pub fn get_error_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ErrorCounterAttrs::ErrorId(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ErrorCounterAttrs",
            "ErrorId",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Name of the error.\n"]
    pub fn get_error_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ErrorCounterAttrs::ErrorName(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ErrorCounterAttrs",
            "ErrorName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Current value of the requested error counter.\n"]
    pub fn get_error_value(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(ErrorCounterAttrs::ErrorValue(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "ErrorCounterAttrs",
            "ErrorValue",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl ErrorCounterAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableErrorCounterAttrs<'a> {
        IterableErrorCounterAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "NodeId",
            2u16 => "ErrorId",
            3u16 => "ErrorName",
            4u16 => "ErrorValue",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableErrorCounterAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableErrorCounterAttrs<'a> {
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
impl<'a> Iterator for IterableErrorCounterAttrs<'a> {
    type Item = Result<ErrorCounterAttrs<'a>, ErrorContext>;
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
                1u16 => ErrorCounterAttrs::NodeId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => ErrorCounterAttrs::ErrorId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => ErrorCounterAttrs::ErrorName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => ErrorCounterAttrs::ErrorValue({
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
            "ErrorCounterAttrs",
            r#type.and_then(|t| ErrorCounterAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableErrorCounterAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ErrorCounterAttrs");
        let mut iter = IterableErrorCounterAttrs::with_loc(&[], self.orig_loc);
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
                ErrorCounterAttrs::NodeId(val) => fmt.field("NodeId", &val),
                ErrorCounterAttrs::ErrorId(val) => fmt.field("ErrorId", &val),
                ErrorCounterAttrs::ErrorName(val) => fmt.field("ErrorName", &val),
                ErrorCounterAttrs::ErrorValue(val) => fmt.field("ErrorValue", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableErrorCounterAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ErrorCounterAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| ErrorCounterAttrs::attr_from_type(t)),
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
                ErrorCounterAttrs::NodeId(val) => {
                    if last_off == offset {
                        stack.push(("NodeId", last_off));
                        break;
                    }
                }
                ErrorCounterAttrs::ErrorId(val) => {
                    if last_off == offset {
                        stack.push(("ErrorId", last_off));
                        break;
                    }
                }
                ErrorCounterAttrs::ErrorName(val) => {
                    if last_off == offset {
                        stack.push(("ErrorName", last_off));
                        break;
                    }
                }
                ErrorCounterAttrs::ErrorValue(val) => {
                    if last_off == offset {
                        stack.push(("ErrorValue", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ErrorCounterAttrs", cur));
        }
        (stack, None)
    }
}
pub struct PushNodeAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushNodeAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushNodeAttrs<Prev> {
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
    #[doc = "Unique identifier for the node. Assigned dynamically by the DRM RAS core\nupon registration.\n"]
    pub fn push_node_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Device name chosen by the driver at registration. Can be a PCI BDF,\nUUID, or module name if unique.\n"]
    pub fn push_device_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "Device name chosen by the driver at registration. Can be a PCI BDF,\nUUID, or module name if unique.\n"]
    pub fn push_device_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Node name chosen by the driver at registration. Can be an IP block name,\nor any name that identifies the RAS node inside the device.\n"]
    pub fn push_node_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            3u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "Node name chosen by the driver at registration. Can be an IP block name,\nor any name that identifies the RAS node inside the device.\n"]
    pub fn push_node_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Type of this node, identifying its function.\n\nAssociated type: [`NodeType`] (enum)"]
    pub fn push_node_type(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushNodeAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushErrorCounterAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushErrorCounterAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushErrorCounterAttrs<Prev> {
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
    #[doc = "Node ID targeted by this error counter operation.\n"]
    pub fn push_node_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Unique identifier for a specific error counter within an node.\n"]
    pub fn push_error_id(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Name of the error.\n"]
    pub fn push_error_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            3u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "Name of the error.\n"]
    pub fn push_error_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "Current value of the requested error counter.\n"]
    pub fn push_error_value(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushErrorCounterAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Retrieve the full list of currently registered DRM RAS nodes. Each node\nincludes its dynamically assigned ID, name, and type. **Important:**\nUser space must call this operation first to obtain the node IDs. These\nIDs are required for all subsequent operations on nodes, such as\nquerying error counters.\n\nFlags: admin-perm\n\nReply attributes:\n- [.get_node_id()](IterableNodeAttrs::get_node_id)\n- [.get_device_name()](IterableNodeAttrs::get_device_name)\n- [.get_node_name()](IterableNodeAttrs::get_node_name)\n- [.get_node_type()](IterableNodeAttrs::get_node_type)\n\n"]
#[derive(Debug)]
pub struct OpListNodesDump<'r> {
    request: Request<'r>,
}
impl<'r> OpListNodesDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushNodeAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushNodeAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushNodeAttrs<&mut Vec<u8>> {
        PushNodeAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushNodeAttrs<RequestBuf<'r>> {
        PushNodeAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableNodeAttrs<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableNodeAttrs::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpListNodesDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drm-ras".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableNodeAttrs<'buf>;
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
#[doc = "Retrieve error counter for a given node. The response includes the id,\nthe name, and even the current value of each counter.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_node_id()](PushErrorCounterAttrs::push_node_id)\n\nReply attributes:\n- [.get_error_id()](IterableErrorCounterAttrs::get_error_id)\n- [.get_error_name()](IterableErrorCounterAttrs::get_error_name)\n- [.get_error_value()](IterableErrorCounterAttrs::get_error_value)\n\n"]
#[derive(Debug)]
pub struct OpGetErrorCounterDump<'r> {
    request: Request<'r>,
}
impl<'r> OpGetErrorCounterDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
    ) -> PushErrorCounterAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushErrorCounterAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushErrorCounterAttrs<&mut Vec<u8>> {
        PushErrorCounterAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushErrorCounterAttrs<RequestBuf<'r>> {
        PushErrorCounterAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableErrorCounterAttrs<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableErrorCounterAttrs::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpGetErrorCounterDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drm-ras".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableErrorCounterAttrs<'buf>;
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
#[doc = "Retrieve error counter for a given node. The response includes the id,\nthe name, and even the current value of each counter.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_node_id()](PushErrorCounterAttrs::push_node_id)\n- [.push_error_id()](PushErrorCounterAttrs::push_error_id)\n\nReply attributes:\n- [.get_error_id()](IterableErrorCounterAttrs::get_error_id)\n- [.get_error_name()](IterableErrorCounterAttrs::get_error_name)\n- [.get_error_value()](IterableErrorCounterAttrs::get_error_value)\n\n"]
#[derive(Debug)]
pub struct OpGetErrorCounterDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetErrorCounterDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
    ) -> PushErrorCounterAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushErrorCounterAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushErrorCounterAttrs<&mut Vec<u8>> {
        PushErrorCounterAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushErrorCounterAttrs<RequestBuf<'r>> {
        PushErrorCounterAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableErrorCounterAttrs<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableErrorCounterAttrs::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpGetErrorCounterDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drm-ras".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableErrorCounterAttrs<'buf>;
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
#[doc = "Clear error counter for a given node. The request includes the error-id\nand node-id of the counter to be cleared.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_node_id()](PushErrorCounterAttrs::push_node_id)\n- [.push_error_id()](PushErrorCounterAttrs::push_error_id)\n\n"]
#[derive(Debug)]
pub struct OpClearErrorCounterDo<'r> {
    request: Request<'r>,
}
impl<'r> OpClearErrorCounterDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
    ) -> PushErrorCounterAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushErrorCounterAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushErrorCounterAttrs<&mut Vec<u8>> {
        PushErrorCounterAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushErrorCounterAttrs<RequestBuf<'r>> {
        PushErrorCounterAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableErrorCounterAttrs<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableErrorCounterAttrs::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpClearErrorCounterDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("drm-ras".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableErrorCounterAttrs<'buf>;
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
    #[doc = "Retrieve the full list of currently registered DRM RAS nodes. Each node\nincludes its dynamically assigned ID, name, and type. **Important:**\nUser space must call this operation first to obtain the node IDs. These\nIDs are required for all subsequent operations on nodes, such as\nquerying error counters.\n\nFlags: admin-perm\n\nReply attributes:\n- [.get_node_id()](IterableNodeAttrs::get_node_id)\n- [.get_device_name()](IterableNodeAttrs::get_device_name)\n- [.get_node_name()](IterableNodeAttrs::get_node_name)\n- [.get_node_type()](IterableNodeAttrs::get_node_type)\n\n"]
    pub fn op_list_nodes_dump(self) -> OpListNodesDump<'buf> {
        let mut res = OpListNodesDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-list-nodes-dump",
            OpListNodesDump::lookup,
        );
        res
    }
    #[doc = "Retrieve error counter for a given node. The response includes the id,\nthe name, and even the current value of each counter.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_node_id()](PushErrorCounterAttrs::push_node_id)\n\nReply attributes:\n- [.get_error_id()](IterableErrorCounterAttrs::get_error_id)\n- [.get_error_name()](IterableErrorCounterAttrs::get_error_name)\n- [.get_error_value()](IterableErrorCounterAttrs::get_error_value)\n\n"]
    pub fn op_get_error_counter_dump(self) -> OpGetErrorCounterDump<'buf> {
        let mut res = OpGetErrorCounterDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-get-error-counter-dump",
            OpGetErrorCounterDump::lookup,
        );
        res
    }
    #[doc = "Retrieve error counter for a given node. The response includes the id,\nthe name, and even the current value of each counter.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_node_id()](PushErrorCounterAttrs::push_node_id)\n- [.push_error_id()](PushErrorCounterAttrs::push_error_id)\n\nReply attributes:\n- [.get_error_id()](IterableErrorCounterAttrs::get_error_id)\n- [.get_error_name()](IterableErrorCounterAttrs::get_error_name)\n- [.get_error_value()](IterableErrorCounterAttrs::get_error_value)\n\n"]
    pub fn op_get_error_counter_do(self) -> OpGetErrorCounterDo<'buf> {
        let mut res = OpGetErrorCounterDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-get-error-counter-do",
            OpGetErrorCounterDo::lookup,
        );
        res
    }
    #[doc = "Clear error counter for a given node. The request includes the error-id\nand node-id of the counter to be cleared.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_node_id()](PushErrorCounterAttrs::push_node_id)\n- [.push_error_id()](PushErrorCounterAttrs::push_error_id)\n\n"]
    pub fn op_clear_error_counter_do(self) -> OpClearErrorCounterDo<'buf> {
        let mut res = OpClearErrorCounterDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-clear-error-counter-do",
            OpClearErrorCounterDo::lookup,
        );
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableErrorCounterAttrs::get_error_id;
        let _ = IterableErrorCounterAttrs::get_error_name;
        let _ = IterableErrorCounterAttrs::get_error_value;
        let _ = IterableNodeAttrs::get_device_name;
        let _ = IterableNodeAttrs::get_node_id;
        let _ = IterableNodeAttrs::get_node_name;
        let _ = IterableNodeAttrs::get_node_type;
        let _ = PushErrorCounterAttrs::<&mut Vec<u8>>::push_error_id;
        let _ = PushErrorCounterAttrs::<&mut Vec<u8>>::push_node_id;
    }
}
