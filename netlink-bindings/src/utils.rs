use std::fmt;

pub use crate::primitives::*;
pub use crate::traits::Pusher;
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
        println!();
        len += chunk.len();
    }
}

fn dump_hex_write(fmt: &mut impl fmt::Write, pad: &str, buf: &[u8]) -> fmt::Result {
    let mut len = 0;
    for chunk in buf.chunks(16) {
        write!(fmt, "{pad}")?;
        write!(fmt, "{len:04x?}: ")?;
        write!(fmt, "{chunk:02x?} ")?;
        for b in chunk {
            if b.is_ascii() && !b.is_ascii_control() {
                write!(fmt, "{}", char::from_u32(*b as u32).unwrap())?;
            } else {
                write!(fmt, ".")?;
            }
        }
        writeln!(fmt)?;
        len += chunk.len();
    }
    Ok(())
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

pub struct FormatBinStr<T: AsRef<[u8]>>(pub T);
impl<T: AsRef<[u8]>> Debug for FormatBinStr<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        // core::bstr is currently unstable
        let bstr = self.0.as_ref();
        if let Ok(cstr) = CStr::from_bytes_with_nul(bstr) {
            write!(fmt, "{cstr:?}")?;
        } else {
            for c in bstr.chunks(127) {
                let mut buf = [0u8; 128];
                buf[..c.len()].clone_from_slice(c);
                write!(fmt, "{:?}", CStr::from_bytes_until_nul(&buf).unwrap())?;
            }
        };
        Ok(())
    }
}

pub struct FormatIter<I: Iterator<Item = T> + Clone, T: Debug>(pub I);

impl<I: Iterator<Item = T> + Clone, T: Debug> Debug for FormatIter<I, T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = fmt.debug_list();
        for item in self.0.clone() {
            f.entry(&item);
        }
        f.finish()
    }
}

pub struct FormatHex<T: AsRef<[u8]>>(pub T);

impl<T: AsRef<[u8]>> Debug for FormatHex<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "\"")?;
        for i in self.0.as_ref() {
            write!(fmt, "{i:02x}")?
        }
        write!(fmt, "\"")?;
        Ok(())
    }
}

#[derive(Clone, Copy, Default)]
pub struct FormatUnrecognized<'a>(pub &'a [u8]);

impl fmt::Debug for FormatUnrecognized<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_struct("UnrecognozedAttribute");
        if let Some((header, attr)) = chop_header(self.0, &mut 0) {
            f.field("header", &header);
            f.field("data", &FormatHexdump(attr));
        } else {
            f.field("data", &FormatHexdump(self.0));
        }
        f.finish()
    }
}

#[derive(Clone, Copy, Default)]
pub struct FormatHexdump<'a>(pub &'a [u8]);
impl fmt::Debug for FormatHexdump<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hexdump(")?;
        if f.alternate() {
            writeln!(f)?;
            dump_hex_write(f, "    ", self.0)?;
        } else {
            self.0.fmt(f)?
        }
        write!(f, ")")?;
        Ok(())
    }
}

pub struct FormatMac<T: AsRef<[u8]>>(pub T);

impl<T: AsRef<[u8]>> Debug for FormatMac<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for i in self.0.as_ref() {
            if !first {
                write!(fmt, ":")?;
            }
            first = false;
            write!(fmt, "{i:02x}")?;
        }
        Ok(())
    }
}

pub struct FormatEnum<T: Debug>(pub u64, pub fn(u64) -> Option<T>);

impl<T: Debug> Debug for FormatEnum<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{} ", self.0)?;

        if let Some(var) = (self.1)(self.0) {
            write!(fmt, "[{var:?}]")?;
        } else {
            write!(fmt, "(unknown variant)")?;
        }

        Ok(())
    }
}

pub struct FormatFlags<T: Debug>(pub u64, pub fn(u64) -> Option<T>);

impl<T: Debug> Debug for FormatFlags<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{} ", self.0)?;

        if self.0 == 0 {
            write!(fmt, "(empty)")?;
            return Ok(());
        }

        let mut seen_variant = false;
        for i in 0..u64::BITS {
            let bit = self.0 & (1 << i);
            if bit == 0 {
                continue;
            }

            if !seen_variant {
                seen_variant = true;
                write!(fmt, "[")?;
            } else {
                write!(fmt, ",")?;
            }

            if let Some(var) = (self.1)(bit) {
                write!(fmt, "{var:?}")?;
            } else {
                write!(fmt, "(unknown bit {i})")?;
            }
        }

        if seen_variant {
            write!(fmt, "]")?;
        }

        Ok(())
    }
}

pub struct DisplayAsDebug<T>(T);

impl<T: fmt::Display> fmt::Debug for DisplayAsDebug<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct FlattenErrorContext<T: fmt::Debug>(pub Result<T, ErrorContext>);

impl<T: Debug> fmt::Debug for FlattenErrorContext<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

pub struct MapFormatArray<I, T, M, D>(pub T, pub M)
where
    T: Clone + Iterator<Item = Result<I, ErrorContext>>,
    M: Clone + FnMut(I) -> D,
    D: fmt::Debug;

impl<I, T, M, D> fmt::Debug for MapFormatArray<I, T, M, D>
where
    T: Clone + Iterator<Item = Result<I, ErrorContext>>,
    M: Clone + FnMut(I) -> D,
    D: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_list();
        for item in self.0.clone() {
            f.entry(&FlattenErrorContext(item.map(self.1.clone())));
        }
        f.finish()
    }
}

pub const NLA_F_NESTED: u16 = 1 << 15;
pub const NLA_F_NET_BYTEORDER: u16 = 1 << 14;

pub const fn nla_type(r#type: u16) -> u16 {
    r#type & (!(NLA_F_NESTED | NLA_F_NET_BYTEORDER))
}

pub const NLA_ALIGNTO: usize = 4;

pub const fn align_up(len: usize, alignment: usize) -> usize {
    ((len) + alignment - 1) & !(alignment - 1)
}

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
pub fn write_header(buf: &mut Vec<u8>, r#type: u16) -> usize {
    push_header_type(buf, r#type, 0, false)
}

/// Returns header offset
/// The kernel doesn't really check byteorder bit nor set it correctly
pub fn push_header_type(buf: &mut Vec<u8>, mut r#type: u16, len: u16, is_nested: bool) -> usize {
    align(buf);

    let header_offset = buf.len();

    if is_nested {
        r#type |= NLA_F_NESTED;
    }

    // TODO: alignment for 8 byte types?
    buf.extend((len + 4).to_ne_bytes());
    buf.extend(r#type.to_ne_bytes());

    header_offset
}

pub fn push_pad(buf: &mut Vec<u8>, pad_type: u16, alignment: usize) {
    assert!(alignment.is_power_of_two());
    if alignment <= 4 {
        return;
    }

    align(buf);

    let needed = align_up(buf.len(), alignment) - buf.len();

    if needed == 0 {
        return;
    }

    push_header(buf, pad_type, (needed - 4) as u16);
    buf.extend(std::iter::repeat_n(0, needed - 4));
}

pub fn finalize_nested_header(buf: &mut Vec<u8>, offset: usize) {
    align(buf);

    let len = (buf.len() - offset) as u16;
    buf[offset..(offset + 2)].copy_from_slice(&len.to_ne_bytes());
}

/// A convenience wrapper on top of [`chop_header()`]
#[derive(Debug, Clone, Copy, Default)]
pub struct IterableChunks<'buf> {
    /// A buffer given to [`Self::new()`]
    pub buf: &'buf [u8],
    /// Offset of the next attribute in [`Self::buf`]
    pub pos: usize,
}

impl<'buf> IterableChunks<'buf> {
    pub fn new(buf: &'buf [u8]) -> Self {
        Self { buf, pos: 0 }
    }
    pub fn is_empty(&self) -> bool {
        self.buf.len() == self.pos
    }
}

impl<'buf> Iterator for IterableChunks<'buf> {
    type Item = (Header, &'buf [u8]);
    fn next(&mut self) -> Option<Self::Item> {
        chop_header(self.buf, &mut self.pos)
    }
}

/// Similar to [`IterableChunks`], but includes attribute header
#[derive(Debug, Clone, Copy, Default)]
pub struct IterateAttrs<'buf> {
    pub buf: &'buf [u8],
    pub pos: usize,
}

impl<'buf> IterateAttrs<'buf> {
    pub fn new(buf: &'buf [u8]) -> Self {
        Self { buf, pos: 0 }
    }
    pub fn is_empty(&self) -> bool {
        self.buf.len() == self.pos
    }
}

impl<'buf> Iterator for IterateAttrs<'buf> {
    type Item = &'buf [u8];
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos + 2 > self.buf.len() {
            self.pos = self.buf.len();
            return None;
        }

        let len = parse_u16(&self.buf[self.pos..self.pos + 2]).unwrap();
        if len < 4 {
            self.pos = self.buf.len();
            return None;
        }

        let new_pos = nla_align_up(self.pos + len as usize).min(self.buf.len());
        let attr = &self.buf[self.pos..new_pos];
        self.pos = new_pos;
        Some(attr)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Header {
    pub r#type: u16,
    pub is_nested: bool,
}

pub fn find_attr(buf: &[u8], r#type: u16) -> Option<&[u8]> {
    let mut pos = 0;
    while let Some((header, attr)) = chop_header(buf, &mut pos) {
        if header.r#type == r#type {
            return Some(attr);
        }
    }
    None
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

// TODO: drop in v0.4
#[deprecated = "Use netlink_bindings::traits::Pusher instead"]
pub use crate::traits::Pusher as Rec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorReason {
    /// Only used in `.get_<attr>()` methods
    AttrMissing,
    /// Value of the attribute can't be parsed
    ParsingError,
    /// Found attribute of type not mentioned in the specification
    UnknownAttr,
}

#[derive(Clone, PartialEq, Eq)]
pub struct ErrorContext {
    pub attrs: &'static str,
    pub attr: Option<&'static str>,
    pub offset: usize,
    pub reason: ErrorReason,
}

impl std::error::Error for ErrorContext {}

impl From<ErrorContext> for std::io::Error {
    fn from(value: ErrorContext) -> Self {
        Self::other(value)
    }
}

impl fmt::Debug for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ErrorContext")
            .field("message", &DisplayAsDebug(&self))
            .field("reason", &self.reason)
            .field("attrs", &self.attrs)
            .field("attr", &self.attr)
            .field("offset", &self.offset)
            .finish()
    }
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let attrs = self.attrs;
        if matches!(self.reason, ErrorReason::AttrMissing) {
            let attr = self.attr.unwrap();
            write!(f, "Missing attribute {attr:?} in {attrs:?}")?;
            return Ok(());
        } else {
            write!(f, "Error parsing ")?;
            if let Some(attr) = self.attr {
                write!(f, "attribute {attr:?} of {attrs:?}")?;
            } else {
                write!(f, "header of {attrs:?}")?;
                if matches!(self.reason, ErrorReason::UnknownAttr) {
                    write!(f, " (unknown attribute)")?;
                }
            }
        }
        write!(f, " at offset {} (0x{:02x})", self.offset, self.offset)?;
        Ok(())
    }
}

impl ErrorContext {
    #[cold]
    pub(crate) fn new(
        attrs: &'static str,
        attr: Option<&'static str>,
        orig_loc: usize,
        loc: usize,
    ) -> ErrorContext {
        let ctx = ErrorContext {
            attrs,
            attr,
            offset: Self::calc_offset(orig_loc, loc),
            reason: if attr.is_some() {
                ErrorReason::ParsingError
            } else {
                ErrorReason::UnknownAttr
            },
        };

        if cfg!(test) {
            panic!("{ctx}")
        } else {
            ctx
        }
    }

    #[cold]
    pub(crate) fn new_missing(
        attrs: &'static str,
        attr: &'static str,
        orig_loc: usize,
        loc: usize,
    ) -> ErrorContext {
        let ctx = ErrorContext {
            attrs,
            attr: Some(attr),
            offset: Self::calc_offset(orig_loc, loc),
            reason: ErrorReason::AttrMissing,
        };

        if cfg!(test) {
            panic!("{ctx}")
        } else {
            ctx
        }
    }

    pub(crate) fn calc_offset(orig_loc: usize, loc: usize) -> usize {
        if orig_loc <= loc && loc - orig_loc <= u16::MAX as usize {
            loc - orig_loc
        } else {
            0
        }
    }
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy, Default)]
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

#[derive(Debug)]
pub enum RequestBuf<'a> {
    Ref(&'a mut Vec<u8>),
    Own(Vec<u8>),
}

impl RequestBuf<'_> {
    pub fn buf(&self) -> &Vec<u8> {
        match self {
            RequestBuf::Ref(buf) => buf,
            RequestBuf::Own(buf) => buf,
        }
    }

    pub fn buf_mut(&mut self) -> &mut Vec<u8> {
        match self {
            RequestBuf::Ref(buf) => buf,
            RequestBuf::Own(buf) => buf,
        }
    }
}

impl Pusher for RequestBuf<'_> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.buf_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.buf()
    }
}

pub struct PushWriter<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}

impl<Prev: Pusher> std::ops::Deref for PushWriter<Prev> {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        self.as_vec()
    }
}

impl<Prev: Pusher> std::ops::DerefMut for PushWriter<Prev> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_vec_mut()
    }
}

impl<Prev: Pusher> std::io::Write for PushWriter<Prev> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.as_vec_mut().write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.as_vec_mut().flush()
    }
}

impl<Prev: Pusher> Pusher for PushWriter<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}

impl<Prev: Pusher> PushWriter<Prev> {
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
}

impl<Prev: Pusher> Drop for PushWriter<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}

pub fn get_bits(buf: &[u8], byte_off: usize, bit_off: usize, bits: usize) -> u32 {
    assert!(bit_off < 8);
    assert!(bits <= 32);

    let max_bytes = (bit_off + bits).div_ceil(8);
    let first = byte_off;
    let last = byte_off + max_bytes;

    let mut reg_buf = [0u8; 8];
    reg_buf[..max_bytes].copy_from_slice(&buf[first..last]);
    let reg = u64::from_le_bytes(reg_buf) << (64 - bit_off - bits) >> (64 - bits);
    reg.to_le() as u32
}

pub fn set_bits(buf: &mut [u8], byte_off: usize, bit_off: usize, bits: usize, val: u32) {
    assert!(bit_off < 8);
    assert!(bits <= 32);

    let max_bytes = (bit_off + bits).div_ceil(8);
    let first = byte_off;
    let last = byte_off + max_bytes;

    let mask = !(((1 << bits) - 1) << bit_off);
    let reg = (val as u64) << bit_off;
    let mut reg_buf = [0u8; 8];
    reg_buf[..max_bytes].copy_from_slice(&buf[first..last]);
    let reg = (u64::from_le_bytes(reg_buf) & mask) | reg;
    let reg_buf = reg.to_le_bytes();
    buf[first..last].copy_from_slice(&reg_buf[..max_bytes])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bits_basic() {
        let buf = &[0b1010_0110u8, 0b0000_1100];

        assert_eq!(get_bits(buf, 0, 0, 4), 0b0110);
        assert_eq!(get_bits(buf, 0, 4, 4), 0b1010);
        assert_eq!(get_bits(buf, 0, 4, 8), 0b1100_1010);
    }

    #[test]
    fn test_set_bits_basic() {
        let buf = &mut [0u8; 2];

        set_bits(buf, 0, 0, 4, 0b0110);
        assert_eq!(buf[0], 0b0110);

        set_bits(buf, 0, 4, 4, 0b1111);
        assert_eq!(buf[0], 0b1111_0110);

        set_bits(buf, 0, 4, 8, 0b1100_1010);
        assert_eq!(buf, &[0b1010_0110, 0b0000_1100]);
    }
}
