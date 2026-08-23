#![doc = "UNIX socket diagnostics\n"]
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
pub const PROTONAME: &str = "unix-diag";
pub const PROTONAME_CSTR: &CStr = c"unix-diag";
pub const PROTONUM: u16 = 4u16;
#[doc = "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Debug, Clone, Copy)]
pub enum ShowFlags {
    #[doc = "show name (not path)\n"]
    Name = 1 << 0,
    #[doc = "show VFS inode info\n"]
    Vfs = 1 << 1,
    #[doc = "show peer socket info\n"]
    Peer = 1 << 2,
    #[doc = "show pending connections\n"]
    Icons = 1 << 3,
    #[doc = "show skb receive queue len\n"]
    Rqlen = 1 << 4,
    #[doc = "show memory info of a socket\n"]
    Meminfo = 1 << 5,
    #[doc = "show socket\\'s UID\n"]
    Uid = 1 << 6,
}
impl ShowFlags {
    pub fn from_value(value: u64) -> Option<Self> {
        Some(match value {
            n if n == 1 << 0 => Self::Name,
            n if n == 1 << 1 => Self::Vfs,
            n if n == 1 << 2 => Self::Peer,
            n if n == 1 << 3 => Self::Icons,
            n if n == 1 << 4 => Self::Rqlen,
            n if n == 1 << 5 => Self::Meminfo,
            n if n == 1 << 6 => Self::Uid,
            _ => return None,
        })
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Req {
    pub sdiag_family: u8,
    pub sdiag_protocol: u8,
    pub pad: u16,
    #[doc = "States to dump\n"]
    pub udiag_states: u32,
    pub udiag_ino: u32,
    #[doc = "Show flags\n"]
    pub udiag_show: u32,
    pub udiag_cookie: [u8; 8usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for Req {
    fn default() -> Self {
        Self::new()
    }
}
impl Req {
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
    pub fn new_from_array(buf: [u8; 24usize]) -> Self {
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
    pub fn as_array(&self) -> &[u8; 24usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 24usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 24usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<Req>() == 24usize);
        const _: () = assert!(std::mem::align_of::<Req>() == 4usize);
        24usize
    }
}
impl std::fmt::Debug for Req {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Req")
            .field("sdiag_family", &self.sdiag_family)
            .field("sdiag_protocol", &self.sdiag_protocol)
            .field("pad", &self.pad)
            .field("udiag_states", &self.udiag_states)
            .field("udiag_ino", &self.udiag_ino)
            .field("udiag_show", &self.udiag_show)
            .field("udiag_cookie", &FormatHex(self.udiag_cookie))
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Msg {
    pub udiag_family: u8,
    pub udiag_type: u8,
    pub udiag_state: u8,
    pub pad: u8,
    pub udiag_ino: u32,
    pub udiag_cookie: [u8; 8usize],
}
#[doc = "Create zero-initialized struct"]
impl Default for Msg {
    fn default() -> Self {
        Self::new()
    }
}
impl Msg {
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
        const _: () = assert!(std::mem::size_of::<Msg>() == 16usize);
        const _: () = assert!(std::mem::align_of::<Msg>() == 4usize);
        16usize
    }
}
impl std::fmt::Debug for Msg {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Msg")
            .field("udiag_family", &self.udiag_family)
            .field("udiag_type", &self.udiag_type)
            .field("udiag_state", &self.udiag_state)
            .field("pad", &self.pad)
            .field("udiag_ino", &self.udiag_ino)
            .field("udiag_cookie", &FormatHex(self.udiag_cookie))
            .finish()
    }
}
#[doc = "VFS inode info\n"]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Vfs {
    pub udiag_vfs_ino: u32,
    pub udiag_vfs_dev: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for Vfs {
    fn default() -> Self {
        Self::new()
    }
}
impl Vfs {
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
    pub fn new_from_array(buf: [u8; 8usize]) -> Self {
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
    pub fn as_array(&self) -> &[u8; 8usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 8usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 8usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<Vfs>() == 8usize);
        const _: () = assert!(std::mem::align_of::<Vfs>() == 4usize);
        8usize
    }
}
#[doc = "Receive queue length info\n"]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Rqlen {
    pub udiag_rqueue: u32,
    pub udiag_wqueue: u32,
}
#[doc = "Create zero-initialized struct"]
impl Default for Rqlen {
    fn default() -> Self {
        Self::new()
    }
}
impl Rqlen {
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
    pub fn new_from_array(buf: [u8; 8usize]) -> Self {
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
    pub fn as_array(&self) -> &[u8; 8usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn from_array(buf: &[u8; 8usize]) -> &Self {
        assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
        unsafe { std::mem::transmute(buf) }
    }
    pub fn into_array(self) -> [u8; 8usize] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn len() -> usize {
        const _: () = assert!(std::mem::size_of::<Rqlen>() == 8usize);
        const _: () = assert!(std::mem::align_of::<Rqlen>() == 4usize);
        8usize
    }
}
#[derive(Clone)]
pub enum UnixDiagAttrs<'a> {
    #[doc = "Unix socket sun_path. May or may not contain \\'0\\'.\n"]
    Name(&'a [u8]),
    Vfs(Vfs),
    Peer(u32),
    Icons(&'a [u8]),
    Rqlen(Rqlen),
    Meminfo(&'a [u8]),
    Shutdown(u8),
    Uid(u32),
}
impl<'a> IterableUnixDiagAttrs<'a> {
    #[doc = "Unix socket sun_path. May or may not contain \\'0\\'.\n"]
    pub fn get_name(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(UnixDiagAttrs::Name(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "UnixDiagAttrs",
            "Name",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vfs(&self) -> Result<Vfs, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(UnixDiagAttrs::Vfs(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "UnixDiagAttrs",
            "Vfs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_peer(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(UnixDiagAttrs::Peer(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "UnixDiagAttrs",
            "Peer",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_icons(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(UnixDiagAttrs::Icons(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "UnixDiagAttrs",
            "Icons",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_rqlen(&self) -> Result<Rqlen, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(UnixDiagAttrs::Rqlen(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "UnixDiagAttrs",
            "Rqlen",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_meminfo(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(UnixDiagAttrs::Meminfo(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "UnixDiagAttrs",
            "Meminfo",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_shutdown(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(UnixDiagAttrs::Shutdown(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "UnixDiagAttrs",
            "Shutdown",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_uid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(UnixDiagAttrs::Uid(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "UnixDiagAttrs",
            "Uid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl UnixDiagAttrs<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableUnixDiagAttrs<'a> {
        IterableUnixDiagAttrs::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "Name",
            1u16 => "Vfs",
            2u16 => "Peer",
            3u16 => "Icons",
            4u16 => "Rqlen",
            5u16 => "Meminfo",
            6u16 => "Shutdown",
            7u16 => "Uid",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableUnixDiagAttrs<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableUnixDiagAttrs<'a> {
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
impl<'a> Iterator for IterableUnixDiagAttrs<'a> {
    type Item = Result<UnixDiagAttrs<'a>, ErrorContext>;
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
                0u16 => UnixDiagAttrs::Name({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                1u16 => UnixDiagAttrs::Vfs({
                    let res = Some(Vfs::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => UnixDiagAttrs::Peer({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => UnixDiagAttrs::Icons({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => UnixDiagAttrs::Rqlen({
                    let res = Some(Rqlen::new_from_zeroed(next));
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => UnixDiagAttrs::Meminfo({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => UnixDiagAttrs::Shutdown({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => UnixDiagAttrs::Uid({
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
            "UnixDiagAttrs",
            r#type.and_then(|t| UnixDiagAttrs::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableUnixDiagAttrs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("UnixDiagAttrs");
        let mut iter = IterableUnixDiagAttrs::with_loc(&[], self.orig_loc);
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
                UnixDiagAttrs::Name(val) => fmt.field("Name", &FormatBinStr(val)),
                UnixDiagAttrs::Vfs(val) => fmt.field("Vfs", &val),
                UnixDiagAttrs::Peer(val) => fmt.field("Peer", &val),
                UnixDiagAttrs::Icons(val) => fmt.field("Icons", &FormatHexdump(val)),
                UnixDiagAttrs::Rqlen(val) => fmt.field("Rqlen", &val),
                UnixDiagAttrs::Meminfo(val) => fmt.field("Meminfo", &FormatHexdump(val)),
                UnixDiagAttrs::Shutdown(val) => fmt.field("Shutdown", &val),
                UnixDiagAttrs::Uid(val) => fmt.field("Uid", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableUnixDiagAttrs<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("UnixDiagAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| UnixDiagAttrs::attr_from_type(t)),
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
                UnixDiagAttrs::Name(val) => {
                    if last_off == offset {
                        stack.push(("Name", last_off));
                        break;
                    }
                }
                UnixDiagAttrs::Vfs(val) => {
                    if last_off == offset {
                        stack.push(("Vfs", last_off));
                        break;
                    }
                }
                UnixDiagAttrs::Peer(val) => {
                    if last_off == offset {
                        stack.push(("Peer", last_off));
                        break;
                    }
                }
                UnixDiagAttrs::Icons(val) => {
                    if last_off == offset {
                        stack.push(("Icons", last_off));
                        break;
                    }
                }
                UnixDiagAttrs::Rqlen(val) => {
                    if last_off == offset {
                        stack.push(("Rqlen", last_off));
                        break;
                    }
                }
                UnixDiagAttrs::Meminfo(val) => {
                    if last_off == offset {
                        stack.push(("Meminfo", last_off));
                        break;
                    }
                }
                UnixDiagAttrs::Shutdown(val) => {
                    if last_off == offset {
                        stack.push(("Shutdown", last_off));
                        break;
                    }
                }
                UnixDiagAttrs::Uid(val) => {
                    if last_off == offset {
                        stack.push(("Uid", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("UnixDiagAttrs", cur));
        }
        (stack, None)
    }
}
pub struct PushUnixDiagAttrs<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushUnixDiagAttrs<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushUnixDiagAttrs<Prev> {
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
    #[doc = "Unix socket sun_path. May or may not contain \\'0\\'.\n"]
    pub fn push_name(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 0u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_vfs(mut self, value: Vfs) -> Self {
        push_header(self.as_vec_mut(), 1u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_peer(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_icons(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 3u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_rqlen(mut self, value: Rqlen) -> Self {
        push_header(self.as_vec_mut(), 4u16, value.as_slice().len() as u16);
        self.as_vec_mut().extend(value.as_slice());
        self
    }
    pub fn push_meminfo(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 5u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_shutdown(mut self, value: u8) -> Self {
        push_header(self.as_vec_mut(), 6u16, 1 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_uid(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushUnixDiagAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = ""]
#[derive(Debug)]
pub struct OpUnixDiagDump<'r> {
    request: Request<'r>,
}
impl<'r> OpUnixDiagDump<'r> {
    pub fn new(mut request: Request<'r>, header: &Req) -> Self {
        Self::write_header(request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(
        buf: &'buf mut Vec<u8>,
        header: &Req,
    ) -> PushUnixDiagAttrs<&'buf mut Vec<u8>> {
        Self::write_header(buf, header);
        PushUnixDiagAttrs::new(buf)
    }
    pub fn encode(&mut self) -> PushUnixDiagAttrs<&mut Vec<u8>> {
        PushUnixDiagAttrs::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushUnixDiagAttrs<RequestBuf<'r>> {
        PushUnixDiagAttrs::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> (Req, IterableUnixDiagAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Req::len()));
        (
            Req::new_from_slice(header).unwrap_or_default(),
            IterableUnixDiagAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn decode_reply<'a>(buf: &'a [u8]) -> (Msg, IterableUnixDiagAttrs<'a>) {
        let (header, attrs) = buf.split_at(buf.len().min(Msg::len()));
        (
            Msg::new_from_slice(header).unwrap_or_default(),
            IterableUnixDiagAttrs::with_loc(attrs, buf.as_ptr() as usize),
        )
    }
    fn write_header<Prev: Pusher>(prev: &mut Prev, header: &Req) {
        prev.as_vec_mut().extend(header.as_slice());
    }
    pub fn header(&self) -> &Req {
        let pos = self.request.pos;
        Req::from_slice(&self.request.buf()[pos..])
    }
    pub fn header_mut(&mut self) -> &mut Req {
        let pos = self.request.pos;
        Req::from_slice_mut(&mut self.request.buf_mut()[pos..])
    }
}
impl NetlinkRequest for OpUnixDiagDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 4u16,
            request_type: 20u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = (Msg, IterableUnixDiagAttrs<'buf>);
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        Self::decode_reply(buf)
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
    #[doc = ""]
    pub fn op_unix_diag_dump(self, header: &Req) -> OpUnixDiagDump<'buf> {
        let mut res = OpUnixDiagDump::new(self, header);
        res.request
            .do_writeback(res.protocol(), "op-unix-diag-dump", OpUnixDiagDump::lookup);
        res
    }
}
