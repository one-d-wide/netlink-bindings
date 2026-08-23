#![doc = "Binder interface over generic netlink\n"]
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
pub const PROTONAME: &str = "binder";
pub const PROTONAME_CSTR: &CStr = c"binder";
#[derive(Clone)]
pub enum Report<'a> {
    #[doc = "The enum binder_driver_return_protocol returned to the sender.\n"]
    Error(u32),
    #[doc = "The binder context where the transaction occurred.\n"]
    Context(&'a CStr),
    #[doc = "The PID of the sender process.\n"]
    FromPid(u32),
    #[doc = "The TID of the sender thread.\n"]
    FromTid(u32),
    #[doc = "The PID of the recipient process. This attribute may not be present if\nthe target could not be determined.\n"]
    ToPid(u32),
    #[doc = "The TID of the recipient thread. This attribute may not be present if\nthe target could not be determined.\n"]
    ToTid(u32),
    #[doc = "When present, indicates the failed transaction is a reply.\n"]
    IsReply(()),
    #[doc = "The bitmask of enum transaction_flags from the transaction.\n"]
    Flags(u32),
    #[doc = "The application-defined code from the transaction.\n"]
    Code(u32),
    #[doc = "The transaction payload size in bytes.\n"]
    DataSize(u32),
}
impl<'a> IterableReport<'a> {
    #[doc = "The enum binder_driver_return_protocol returned to the sender.\n"]
    pub fn get_error(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Report::Error(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Report",
            "Error",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The binder context where the transaction occurred.\n"]
    pub fn get_context(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Report::Context(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Report",
            "Context",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The PID of the sender process.\n"]
    pub fn get_from_pid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Report::FromPid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Report",
            "FromPid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The TID of the sender thread.\n"]
    pub fn get_from_tid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Report::FromTid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Report",
            "FromTid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The PID of the recipient process. This attribute may not be present if\nthe target could not be determined.\n"]
    pub fn get_to_pid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Report::ToPid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Report",
            "ToPid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The TID of the recipient thread. This attribute may not be present if\nthe target could not be determined.\n"]
    pub fn get_to_tid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Report::ToTid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Report",
            "ToTid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "When present, indicates the failed transaction is a reply.\n"]
    pub fn get_is_reply(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Report::IsReply(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Report",
            "IsReply",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The bitmask of enum transaction_flags from the transaction.\n"]
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Report::Flags(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Report",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The application-defined code from the transaction.\n"]
    pub fn get_code(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Report::Code(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Report",
            "Code",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "The transaction payload size in bytes.\n"]
    pub fn get_data_size(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Report::DataSize(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Report",
            "DataSize",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Report<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableReport<'a> {
        IterableReport::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Error",
            2u16 => "Context",
            3u16 => "FromPid",
            4u16 => "FromTid",
            5u16 => "ToPid",
            6u16 => "ToTid",
            7u16 => "IsReply",
            8u16 => "Flags",
            9u16 => "Code",
            10u16 => "DataSize",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableReport<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableReport<'a> {
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
impl<'a> Iterator for IterableReport<'a> {
    type Item = Result<Report<'a>, ErrorContext>;
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
                1u16 => Report::Error({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Report::Context({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Report::FromPid({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Report::FromTid({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Report::ToPid({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Report::ToTid({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => Report::IsReply(()),
                8u16 => Report::Flags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => Report::Code({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => Report::DataSize({
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
            "Report",
            r#type.and_then(|t| Report::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableReport<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Report");
        let mut iter = IterableReport::with_loc(&[], self.orig_loc);
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
                Report::Error(val) => fmt.field("Error", &val),
                Report::Context(val) => fmt.field("Context", &val),
                Report::FromPid(val) => fmt.field("FromPid", &val),
                Report::FromTid(val) => fmt.field("FromTid", &val),
                Report::ToPid(val) => fmt.field("ToPid", &val),
                Report::ToTid(val) => fmt.field("ToTid", &val),
                Report::IsReply(val) => fmt.field("IsReply", &val),
                Report::Flags(val) => fmt.field("Flags", &val),
                Report::Code(val) => fmt.field("Code", &val),
                Report::DataSize(val) => fmt.field("DataSize", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableReport<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Report", offset));
            return (stack, missing_type.and_then(|t| Report::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Report::Error(val) => {
                    if last_off == offset {
                        stack.push(("Error", last_off));
                        break;
                    }
                }
                Report::Context(val) => {
                    if last_off == offset {
                        stack.push(("Context", last_off));
                        break;
                    }
                }
                Report::FromPid(val) => {
                    if last_off == offset {
                        stack.push(("FromPid", last_off));
                        break;
                    }
                }
                Report::FromTid(val) => {
                    if last_off == offset {
                        stack.push(("FromTid", last_off));
                        break;
                    }
                }
                Report::ToPid(val) => {
                    if last_off == offset {
                        stack.push(("ToPid", last_off));
                        break;
                    }
                }
                Report::ToTid(val) => {
                    if last_off == offset {
                        stack.push(("ToTid", last_off));
                        break;
                    }
                }
                Report::IsReply(val) => {
                    if last_off == offset {
                        stack.push(("IsReply", last_off));
                        break;
                    }
                }
                Report::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                Report::Code(val) => {
                    if last_off == offset {
                        stack.push(("Code", last_off));
                        break;
                    }
                }
                Report::DataSize(val) => {
                    if last_off == offset {
                        stack.push(("DataSize", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Report", cur));
        }
        (stack, None)
    }
}
pub struct PushReport<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushReport<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushReport<Prev> {
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
    #[doc = "The enum binder_driver_return_protocol returned to the sender.\n"]
    pub fn push_error(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The binder context where the transaction occurred.\n"]
    pub fn push_context(mut self, value: &CStr) -> Self {
        push_header(
            self.as_vec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_vec_mut().extend(value.to_bytes_with_nul());
        self
    }
    #[doc = "The binder context where the transaction occurred.\n"]
    pub fn push_context_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_vec_mut().extend(value);
        self.as_vec_mut().push(0);
        self
    }
    #[doc = "The PID of the sender process.\n"]
    pub fn push_from_pid(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The TID of the sender thread.\n"]
    pub fn push_from_tid(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The PID of the recipient process. This attribute may not be present if\nthe target could not be determined.\n"]
    pub fn push_to_pid(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The TID of the recipient thread. This attribute may not be present if\nthe target could not be determined.\n"]
    pub fn push_to_tid(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "When present, indicates the failed transaction is a reply.\n"]
    pub fn push_is_reply(mut self, value: ()) -> Self {
        push_header(self.as_vec_mut(), 7u16, 0 as u16);
        self
    }
    #[doc = "The bitmask of enum transaction_flags from the transaction.\n"]
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 8u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The application-defined code from the transaction.\n"]
    pub fn push_code(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 9u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "The transaction payload size in bytes.\n"]
    pub fn push_data_size(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 10u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushReport<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Notify attributes:\n- [`.get_error()`](IterableReport::get_error)\n- [`.get_context()`](IterableReport::get_context)\n- [`.get_from_pid()`](IterableReport::get_from_pid)\n- [`.get_from_tid()`](IterableReport::get_from_tid)\n- [`.get_to_pid()`](IterableReport::get_to_pid)\n- [`.get_to_tid()`](IterableReport::get_to_tid)\n- [`.get_is_reply()`](IterableReport::get_is_reply)\n- [`.get_flags()`](IterableReport::get_flags)\n- [`.get_code()`](IterableReport::get_code)\n- [`.get_data_size()`](IterableReport::get_data_size)\n"]
#[derive(Debug)]
pub struct OpReportNotif;
impl OpReportNotif {
    pub const CMD: u8 = 1u8;
    pub fn decode_notif<'a>(buf: &'a [u8]) -> IterableReport<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableReport::with_loc(attrs, buf.as_ptr() as usize)
    }
}
pub struct NotifGroup;
impl NotifGroup {
    #[doc = "Notifications:\n- [`OpReportNotif`]\n"]
    pub const REPORT: &str = "report";
    #[doc = "Notifications:\n- [`OpReportNotif`]\n"]
    pub const REPORT_CSTR: &CStr = c"report";
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableReport::get_code;
        let _ = IterableReport::get_context;
        let _ = IterableReport::get_data_size;
        let _ = IterableReport::get_error;
        let _ = IterableReport::get_flags;
        let _ = IterableReport::get_from_pid;
        let _ = IterableReport::get_from_tid;
        let _ = IterableReport::get_is_reply;
        let _ = IterableReport::get_to_pid;
        let _ = IterableReport::get_to_tid;
        let _ = OpReportNotif;
    }
}
