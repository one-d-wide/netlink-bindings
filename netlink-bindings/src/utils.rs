use std::marker::PhantomData;

pub use crate::primitives::*;
pub use std::{ffi::CStr, fmt::Debug, iter::Iterator};

pub fn dump_hex(buf: &[u8]) {
    let mut len = 0;
    for chunk in buf.chunks(16) {
        print!("{len:04x?}: ");
        print!("{chunk:02x?} ");
        for b in chunk {
            if b.is_ascii() && !b.is_ascii_control() {
                print!("{}", char::from_u32(*b as u32).unwrap());
            } else {
                print!(".");
            }
        }
        println!(".");
        len += chunk.len();
    }
}

pub fn dump_assert_eq(left: &[u8], right: &[u8]) {
    if left.len() != right.len() {
        dump_hex(left);
        dump_hex(right);
        panic!("Length mismatched");
    }
    if let Some(pos) = left.iter().zip(right.iter()).position(|(l, r)| *l != *r) {
        println!();
        println!("Left:");
        dump_hex(left);
        println!();
        println!("Right:");
        dump_hex(right);
        panic!("Differ at byte {pos} (0x{pos:x?})");
    }
}

pub struct FormatHex<'a>(pub &'a [u8]);

impl<'a> Debug for FormatHex<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "\"")?;
        for i in self.0 {
            write!(fmt, "{i:02x}")?
        }
        write!(fmt, "\"")?;
        Ok(())
    }
}

/// Generic family header. Used in genetlink protocol if spec doesn't specify it's own
#[derive(Clone)]
pub struct PushBuiltinNfgenmsg {
    buf: [u8; 4usize],
}

impl PushBuiltinNfgenmsg {
    #[doc = "Create zero-initialized struct"]
    pub fn new() -> Self {
        Self {
            buf: [0u8; Self::len()],
        }
    }
    #[doc = "Copy from contents from other slice. Panics if length is mismatched"]
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
        4usize
    }
    pub fn cmd(&self) -> u8 {
        parse_u8(&self.buf[0usize..1usize]).unwrap()
    }
    pub fn set_cmd(&mut self, cmd: u8) {
        self.buf[0usize..1usize].copy_from_slice(&cmd.to_ne_bytes())
    }
    pub fn set_version(&mut self, version: u8) {
        self.buf[1usize..2usize].copy_from_slice(&version.to_ne_bytes())
    }
    pub fn version(&self) -> u8 {
        parse_u8(&self.buf[1usize..2usize]).unwrap()
    }
    pub fn reserved(&self) -> u16 {
        parse_be_u16(&self.buf[2usize..4usize]).unwrap()
    }
    pub fn set_reserved(&mut self, reserved: u16) {
        self.buf[2usize..4usize].copy_from_slice(&reserved.to_be_bytes())
    }
}

impl Debug for PushBuiltinNfgenmsg {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("PushBuiltinNfgenmsg")
            .field("cmd", &self.cmd())
            .field("version", &self.version())
            .field("reserved", &self.reserved())
            .finish()
    }
}

#[derive(Clone)]
pub struct PushBuiltinBitfield32 {
    buf: [u8; 8usize],
}

impl PushBuiltinBitfield32 {
    #[doc = "Create zero-initialized struct"]
    pub fn new() -> Self {
        Self {
            buf: [0u8; Self::len()],
        }
    }
    #[doc = "Copy from contents from other slice. Panics if length is mismatched"]
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
        8usize
    }
    pub fn value(&self) -> u32 {
        parse_u32(&self.buf[0usize..4usize]).unwrap()
    }
    pub fn set_value(&mut self, value: u32) {
        self.buf[0usize..4usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn selector(&self) -> u32 {
        parse_u32(&self.buf[4usize..8usize]).unwrap()
    }
    pub fn set_selector(&mut self, selector: u32) {
        self.buf[4usize..8usize].copy_from_slice(&selector.to_ne_bytes())
    }
}

impl Debug for PushBuiltinBitfield32 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("PushBuiltinBitfield32")
            .field("value", &self.value())
            .field("selector", &self.selector())
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub attrs: &'static str,
    pub attr: Option<&'static str>,
    pub offset: usize,
}

impl std::error::Error for ErrorContext {}

impl std::fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error parsing ")?;
        if let Some(attr) = self.attr {
            write!(f, "attribute {:?} of {:?}", attr, self.attrs)?;
        } else {
            write!(f, "header of {:?}", self.attrs)?;
        }
        write!(f, " at offset {}", self.offset)?;
        Ok(())
    }
}

pub struct FlattenErrorContext<T: Debug>(pub Result<T, ErrorContext>);

impl<T: Debug> Debug for FlattenErrorContext<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Ok(ok) => ok.fmt(f),
            Err(err) => {
                f.write_str("Err(")?;
                err.fmt(f)?;
                f.write_str(")")
            }
        }
    }
}

pub const NLA_F_NESTED: u16 = 1 << 15;
pub const NLA_F_NET_BYTEORDER: u16 = 1 << 14;

pub const fn nla_type(r#type: u16) -> u16 {
    r#type & (!(NLA_F_NESTED | NLA_F_NET_BYTEORDER))
}

pub const NLA_ALIGNTO: usize = 4;

pub const fn nla_align_up(len: usize) -> usize {
    ((len) + NLA_ALIGNTO - 1) & !(NLA_ALIGNTO - 1)
}

pub fn align(buf: &mut Vec<u8>) {
    let len = buf.len();
    buf.extend(std::iter::repeat_n(0u8, nla_align_up(len) - len));
}

/// Returns header offset
pub fn push_nested_header(buf: &mut Vec<u8>, r#type: u16) -> usize {
    push_header_type(buf, r#type, 0, true)
}

/// Returns header offset
pub fn push_header(buf: &mut Vec<u8>, r#type: u16, len: u16) -> usize {
    push_header_type(buf, r#type, len, false)
}

/// Returns header offset
/// Kernel doesn't really check nor set byteorder bit it correctly
fn push_header_type(buf: &mut Vec<u8>, mut r#type: u16, len: u16, is_nested: bool) -> usize {
    align(buf);

    let header_offset = buf.len();

    if is_nested {
        r#type |= NLA_F_NESTED;
    }

    // TODO: alignment for 8 byte types?
    buf.extend((len + 4).to_ne_bytes());
    buf.extend(r#type.to_ne_bytes());

    align(buf);

    header_offset
}

pub fn finalize_nested_header(buf: &mut Vec<u8>, offset: usize) {
    align(buf);

    let len = (buf.len() - offset) as u16;
    buf[offset..(offset + 2)].copy_from_slice(&len.to_ne_bytes());
}

#[derive(Debug, Clone, Copy)]
pub struct Header {
    pub r#type: u16,
    pub is_nested: bool,
}

pub fn chop_header<'a>(buf: &'a [u8], pos: &mut usize) -> Option<(Header, &'a [u8])> {
    let buf = &buf[*pos..];

    if buf.len() < 4 {
        return None;
    }

    let len = parse_u16(&buf[0..2]).unwrap();
    let r#type = parse_u16(&buf[2..4]).unwrap();

    let next_len = nla_align_up(len as usize);

    if len < 4 || buf.len() < len as usize {
        return None;
    }

    let next = &buf[4..len as usize];
    *pos += next_len.min(buf.len());

    Some((
        Header {
            r#type: nla_type(r#type),
            is_nested: r#type & NLA_F_NESTED != 0,
        },
        next,
    ))
}

pub trait Rec {
    fn as_rec_mut(&mut self) -> &mut Vec<u8>;
}

impl Rec for &mut Vec<u8> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self
    }
}

#[derive(Clone)]
pub struct Iterable<'a, AttrSet> {
    pub(crate) buf: &'a [u8],
    /// Current position of the iterable in the [`buf`]
    pub(crate) pos: usize,
    /// Pointer to beginning of first slice in the chain.
    /// Only used in calculating byte offset for error context
    pub(crate) orig_loc: *const u8,
    pub(crate) phantom: PhantomData<AttrSet>,
}

impl<'a, AttrSet> Iterable<'a, AttrSet> {
    pub(crate) fn new(buf: &'a [u8]) -> Self {
        Iterable::with_loc(buf, buf.as_ptr())
    }

    pub(crate) fn with_loc(buf: &'a [u8], orig_loc: *const u8) -> Self {
        Iterable {
            buf,
            pos: 0,
            orig_loc,
            phantom: PhantomData,
        }
    }

    #[cold]
    pub(crate) fn error_context(
        &mut self,
        attrs: &'static str,
        attr: Option<&'static str>,
        loc: *const u8,
    ) -> ErrorContext {
        self.buf = &[];

        let orig = self.orig_loc as usize;
        let loc = loc as usize;

        let offset = if orig <= loc && loc - orig <= u16::MAX as usize {
            loc - orig
        } else {
            0
        };

        let ctx = ErrorContext {
            attrs,
            attr,
            offset,
        };

        if cfg!(test) {
            panic!("{ctx}")
        } else {
            ctx
        }
    }

    pub fn get_buf(&self) -> &'a [u8] {
        self.buf
    }
}

#[derive(Clone)]
pub struct MultiAttrIterable<I, T, V>
where
    I: Iterator<Item = Result<T, ErrorContext>>,
{
    pub(crate) inner: I,
    pub(crate) f: fn(T) -> Option<V>,
}

impl<I, T, V> MultiAttrIterable<I, T, V>
where
    I: Iterator<Item = Result<T, ErrorContext>>,
{
    pub fn new(inner: I, f: fn(T) -> Option<V>) -> Self {
        Self { inner, f }
    }
}

impl<I, T, V> Iterator for MultiAttrIterable<I, T, V>
where
    I: Iterator<Item = Result<T, ErrorContext>>,
{
    type Item = V;
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(Ok(val)) => (self.f)(val),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct ArrayIterable<I, T>
where
    I: Iterator<Item = Result<T, ErrorContext>>,
{
    pub(crate) inner: I,
}

impl<I, T> ArrayIterable<I, T>
where
    I: Iterator<Item = Result<T, ErrorContext>>,
{
    pub fn new(inner: I) -> Self {
        Self { inner }
    }
}

impl<I, T> Iterator for ArrayIterable<I, T>
where
    I: Iterator<Item = Result<T, ErrorContext>>,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(Ok(val)) => Some(val),
            _ => None,
        }
    }
}
