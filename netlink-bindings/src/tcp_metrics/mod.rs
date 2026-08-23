#![doc = "Management interface for TCP metrics.\n"]
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
pub const PROTONAME: &str = "tcp_metrics";
pub const PROTONAME_CSTR: &CStr = c"tcp_metrics";
pub const TCP_FASTOPEN_COOKIE_MAX: u64 = 16u64;
#[derive(Clone)]
pub enum TcpMetrics<'a> {
    AddrIpv4(std::net::Ipv4Addr),
    AddrIpv6(std::net::Ipv6Addr),
    Age(u64),
    #[doc = "unused\n"]
    TwTsval(u32),
    #[doc = "unused\n"]
    TwTsStamp(i32),
    Vals(IterableMetrics<'a>),
    FopenMss(u16),
    FopenSynDrops(u16),
    FopenSynDropTs(u64),
    FopenCookie(&'a [u8]),
    SaddrIpv4(std::net::Ipv4Addr),
    SaddrIpv6(std::net::Ipv6Addr),
    Pad(&'a [u8]),
}
impl<'a> IterableTcpMetrics<'a> {
    pub fn get_addr_ipv4(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TcpMetrics::AddrIpv4(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TcpMetrics",
            "AddrIpv4",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_addr_ipv6(&self) -> Result<std::net::Ipv6Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TcpMetrics::AddrIpv6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TcpMetrics",
            "AddrIpv6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_age(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TcpMetrics::Age(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TcpMetrics",
            "Age",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "unused\n"]
    pub fn get_tw_tsval(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TcpMetrics::TwTsval(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TcpMetrics",
            "TwTsval",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "unused\n"]
    pub fn get_tw_ts_stamp(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TcpMetrics::TwTsStamp(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TcpMetrics",
            "TwTsStamp",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_vals(&self) -> Result<IterableMetrics<'a>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TcpMetrics::Vals(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TcpMetrics",
            "Vals",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fopen_mss(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TcpMetrics::FopenMss(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TcpMetrics",
            "FopenMss",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fopen_syn_drops(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TcpMetrics::FopenSynDrops(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TcpMetrics",
            "FopenSynDrops",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fopen_syn_drop_ts(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TcpMetrics::FopenSynDropTs(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TcpMetrics",
            "FopenSynDropTs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_fopen_cookie(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TcpMetrics::FopenCookie(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TcpMetrics",
            "FopenCookie",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_saddr_ipv4(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TcpMetrics::SaddrIpv4(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TcpMetrics",
            "SaddrIpv4",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_saddr_ipv6(&self) -> Result<std::net::Ipv6Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TcpMetrics::SaddrIpv6(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TcpMetrics",
            "SaddrIpv6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(TcpMetrics::Pad(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "TcpMetrics",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl TcpMetrics<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableTcpMetrics<'a> {
        IterableTcpMetrics::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "AddrIpv4",
            2u16 => "AddrIpv6",
            3u16 => "Age",
            4u16 => "TwTsval",
            5u16 => "TwTsStamp",
            6u16 => "Vals",
            7u16 => "FopenMss",
            8u16 => "FopenSynDrops",
            9u16 => "FopenSynDropTs",
            10u16 => "FopenCookie",
            11u16 => "SaddrIpv4",
            12u16 => "SaddrIpv6",
            13u16 => "Pad",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableTcpMetrics<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableTcpMetrics<'a> {
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
impl<'a> Iterator for IterableTcpMetrics<'a> {
    type Item = Result<TcpMetrics<'a>, ErrorContext>;
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
                1u16 => TcpMetrics::AddrIpv4({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => TcpMetrics::AddrIpv6({
                    let res = parse_be_u128(next).map(Ipv6Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => TcpMetrics::Age({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => TcpMetrics::TwTsval({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => TcpMetrics::TwTsStamp({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => TcpMetrics::Vals({
                    let res = Some(IterableMetrics::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => TcpMetrics::FopenMss({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => TcpMetrics::FopenSynDrops({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => TcpMetrics::FopenSynDropTs({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => TcpMetrics::FopenCookie({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => TcpMetrics::SaddrIpv4({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => TcpMetrics::SaddrIpv6({
                    let res = parse_be_u128(next).map(Ipv6Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => TcpMetrics::Pad({
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
            "TcpMetrics",
            r#type.and_then(|t| TcpMetrics::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableTcpMetrics<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("TcpMetrics");
        let mut iter = IterableTcpMetrics::with_loc(&[], self.orig_loc);
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
                TcpMetrics::AddrIpv4(val) => fmt.field("AddrIpv4", &val),
                TcpMetrics::AddrIpv6(val) => fmt.field("AddrIpv6", &val),
                TcpMetrics::Age(val) => fmt.field("Age", &val),
                TcpMetrics::TwTsval(val) => fmt.field("TwTsval", &val),
                TcpMetrics::TwTsStamp(val) => fmt.field("TwTsStamp", &val),
                TcpMetrics::Vals(val) => fmt.field("Vals", &val),
                TcpMetrics::FopenMss(val) => fmt.field("FopenMss", &val),
                TcpMetrics::FopenSynDrops(val) => fmt.field("FopenSynDrops", &val),
                TcpMetrics::FopenSynDropTs(val) => fmt.field("FopenSynDropTs", &val),
                TcpMetrics::FopenCookie(val) => fmt.field("FopenCookie", &FormatHexdump(val)),
                TcpMetrics::SaddrIpv4(val) => fmt.field("SaddrIpv4", &val),
                TcpMetrics::SaddrIpv6(val) => fmt.field("SaddrIpv6", &val),
                TcpMetrics::Pad(val) => fmt.field("Pad", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableTcpMetrics<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("TcpMetrics", offset));
            return (
                stack,
                missing_type.and_then(|t| TcpMetrics::attr_from_type(t)),
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
                TcpMetrics::AddrIpv4(val) => {
                    if last_off == offset {
                        stack.push(("AddrIpv4", last_off));
                        break;
                    }
                }
                TcpMetrics::AddrIpv6(val) => {
                    if last_off == offset {
                        stack.push(("AddrIpv6", last_off));
                        break;
                    }
                }
                TcpMetrics::Age(val) => {
                    if last_off == offset {
                        stack.push(("Age", last_off));
                        break;
                    }
                }
                TcpMetrics::TwTsval(val) => {
                    if last_off == offset {
                        stack.push(("TwTsval", last_off));
                        break;
                    }
                }
                TcpMetrics::TwTsStamp(val) => {
                    if last_off == offset {
                        stack.push(("TwTsStamp", last_off));
                        break;
                    }
                }
                TcpMetrics::Vals(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                TcpMetrics::FopenMss(val) => {
                    if last_off == offset {
                        stack.push(("FopenMss", last_off));
                        break;
                    }
                }
                TcpMetrics::FopenSynDrops(val) => {
                    if last_off == offset {
                        stack.push(("FopenSynDrops", last_off));
                        break;
                    }
                }
                TcpMetrics::FopenSynDropTs(val) => {
                    if last_off == offset {
                        stack.push(("FopenSynDropTs", last_off));
                        break;
                    }
                }
                TcpMetrics::FopenCookie(val) => {
                    if last_off == offset {
                        stack.push(("FopenCookie", last_off));
                        break;
                    }
                }
                TcpMetrics::SaddrIpv4(val) => {
                    if last_off == offset {
                        stack.push(("SaddrIpv4", last_off));
                        break;
                    }
                }
                TcpMetrics::SaddrIpv6(val) => {
                    if last_off == offset {
                        stack.push(("SaddrIpv6", last_off));
                        break;
                    }
                }
                TcpMetrics::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("TcpMetrics", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum Metrics {
    #[doc = "Round Trip Time (RTT), in msecs with 3 bits fractional (left-shift by 3\nto get the msec value).\n"]
    Rtt(u32),
    #[doc = "Round Trip Time VARiance (RTT), in msecs with 2 bits fractional\n(left-shift by 2 to get the msec value).\n"]
    Rttvar(u32),
    #[doc = "Slow Start THRESHold.\n"]
    Ssthresh(u32),
    #[doc = "Congestion Window.\n"]
    Cwnd(u32),
    #[doc = "Reodering metric.\n"]
    Reodering(u32),
    #[doc = "Round Trip Time (RTT), in usecs, with 3 bits fractional (left-shift by 3\nto get the msec value).\n"]
    RttUs(u32),
    #[doc = "Round Trip Time (RTT), in usecs, with 2 bits fractional (left-shift by 3\nto get the msec value).\n"]
    RttvarUs(u32),
}
impl<'a> IterableMetrics<'a> {
    #[doc = "Round Trip Time (RTT), in msecs with 3 bits fractional (left-shift by 3\nto get the msec value).\n"]
    pub fn get_rtt(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Metrics::Rtt(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Metrics",
            "Rtt",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Round Trip Time VARiance (RTT), in msecs with 2 bits fractional\n(left-shift by 2 to get the msec value).\n"]
    pub fn get_rttvar(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Metrics::Rttvar(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Metrics",
            "Rttvar",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Slow Start THRESHold.\n"]
    pub fn get_ssthresh(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Metrics::Ssthresh(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Metrics",
            "Ssthresh",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Congestion Window.\n"]
    pub fn get_cwnd(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Metrics::Cwnd(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Metrics",
            "Cwnd",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Reodering metric.\n"]
    pub fn get_reodering(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Metrics::Reodering(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Metrics",
            "Reodering",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Round Trip Time (RTT), in usecs, with 3 bits fractional (left-shift by 3\nto get the msec value).\n"]
    pub fn get_rtt_us(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Metrics::RttUs(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Metrics",
            "RttUs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Round Trip Time (RTT), in usecs, with 2 bits fractional (left-shift by 3\nto get the msec value).\n"]
    pub fn get_rttvar_us(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Ok(Metrics::RttvarUs(val)) = attr {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Metrics",
            "RttvarUs",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Metrics {
    pub fn new<'a>(buf: &'a [u8]) -> IterableMetrics<'a> {
        IterableMetrics::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Rtt",
            2u16 => "Rttvar",
            3u16 => "Ssthresh",
            4u16 => "Cwnd",
            5u16 => "Reodering",
            6u16 => "RttUs",
            7u16 => "RttvarUs",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableMetrics<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableMetrics<'a> {
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
impl<'a> Iterator for IterableMetrics<'a> {
    type Item = Result<Metrics, ErrorContext>;
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
                1u16 => Metrics::Rtt({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Metrics::Rttvar({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Metrics::Ssthresh({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Metrics::Cwnd({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Metrics::Reodering({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => Metrics::RttUs({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => Metrics::RttvarUs({
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
            "Metrics",
            r#type.and_then(|t| Metrics::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableMetrics<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Metrics");
        let mut iter = IterableMetrics::with_loc(&[], self.orig_loc);
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
                Metrics::Rtt(val) => fmt.field("Rtt", &val),
                Metrics::Rttvar(val) => fmt.field("Rttvar", &val),
                Metrics::Ssthresh(val) => fmt.field("Ssthresh", &val),
                Metrics::Cwnd(val) => fmt.field("Cwnd", &val),
                Metrics::Reodering(val) => fmt.field("Reodering", &val),
                Metrics::RttUs(val) => fmt.field("RttUs", &val),
                Metrics::RttvarUs(val) => fmt.field("RttvarUs", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableMetrics<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Metrics", offset));
            return (stack, missing_type.and_then(|t| Metrics::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Metrics::Rtt(val) => {
                    if last_off == offset {
                        stack.push(("Rtt", last_off));
                        break;
                    }
                }
                Metrics::Rttvar(val) => {
                    if last_off == offset {
                        stack.push(("Rttvar", last_off));
                        break;
                    }
                }
                Metrics::Ssthresh(val) => {
                    if last_off == offset {
                        stack.push(("Ssthresh", last_off));
                        break;
                    }
                }
                Metrics::Cwnd(val) => {
                    if last_off == offset {
                        stack.push(("Cwnd", last_off));
                        break;
                    }
                }
                Metrics::Reodering(val) => {
                    if last_off == offset {
                        stack.push(("Reodering", last_off));
                        break;
                    }
                }
                Metrics::RttUs(val) => {
                    if last_off == offset {
                        stack.push(("RttUs", last_off));
                        break;
                    }
                }
                Metrics::RttvarUs(val) => {
                    if last_off == offset {
                        stack.push(("RttvarUs", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Metrics", cur));
        }
        (stack, None)
    }
}
pub struct PushTcpMetrics<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushTcpMetrics<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushTcpMetrics<Prev> {
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
    pub fn push_addr_ipv4(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    pub fn push_addr_ipv6(mut self, value: std::net::Ipv6Addr) -> Self {
        push_header(self.as_vec_mut(), 2u16, 16 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    pub fn push_age(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 3u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "unused\n"]
    pub fn push_tw_tsval(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "unused\n"]
    pub fn push_tw_ts_stamp(mut self, value: i32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_vals(mut self) -> PushMetrics<Self> {
        let header_offset = push_nested_header(self.as_vec_mut(), 6u16);
        PushMetrics {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_fopen_mss(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 7u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fopen_syn_drops(mut self, value: u16) -> Self {
        push_header(self.as_vec_mut(), 8u16, 2 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fopen_syn_drop_ts(mut self, value: u64) -> Self {
        push_header(self.as_vec_mut(), 9u16, 8 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fopen_cookie(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 10u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
    pub fn push_saddr_ipv4(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_vec_mut(), 11u16, 4 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    pub fn push_saddr_ipv6(mut self, value: std::net::Ipv6Addr) -> Self {
        push_header(self.as_vec_mut(), 12u16, 16 as u16);
        self.as_vec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_vec_mut(), 13u16, value.len() as u16);
        self.as_vec_mut().extend(value);
        self
    }
}
impl<Prev: Pusher> Drop for PushTcpMetrics<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushMetrics<Prev: Pusher> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Pusher> Pusher for PushMetrics<Prev> {
    fn as_vec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_vec_mut()
    }
    fn as_vec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_vec()
    }
}
impl<Prev: Pusher> PushMetrics<Prev> {
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
    #[doc = "Round Trip Time (RTT), in msecs with 3 bits fractional (left-shift by 3\nto get the msec value).\n"]
    pub fn push_rtt(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 1u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Round Trip Time VARiance (RTT), in msecs with 2 bits fractional\n(left-shift by 2 to get the msec value).\n"]
    pub fn push_rttvar(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 2u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Slow Start THRESHold.\n"]
    pub fn push_ssthresh(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 3u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Congestion Window.\n"]
    pub fn push_cwnd(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 4u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Reodering metric.\n"]
    pub fn push_reodering(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 5u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Round Trip Time (RTT), in usecs, with 3 bits fractional (left-shift by 3\nto get the msec value).\n"]
    pub fn push_rtt_us(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 6u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
    #[doc = "Round Trip Time (RTT), in usecs, with 2 bits fractional (left-shift by 3\nto get the msec value).\n"]
    pub fn push_rttvar_us(mut self, value: u32) -> Self {
        push_header(self.as_vec_mut(), 7u16, 4 as u16);
        self.as_vec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Pusher> Drop for PushMetrics<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Retrieve metrics.\n\nReply attributes:\n- [.get_addr_ipv4()](IterableTcpMetrics::get_addr_ipv4)\n- [.get_addr_ipv6()](IterableTcpMetrics::get_addr_ipv6)\n- [.get_age()](IterableTcpMetrics::get_age)\n- [.get_vals()](IterableTcpMetrics::get_vals)\n- [.get_fopen_mss()](IterableTcpMetrics::get_fopen_mss)\n- [.get_fopen_syn_drops()](IterableTcpMetrics::get_fopen_syn_drops)\n- [.get_fopen_syn_drop_ts()](IterableTcpMetrics::get_fopen_syn_drop_ts)\n- [.get_fopen_cookie()](IterableTcpMetrics::get_fopen_cookie)\n- [.get_saddr_ipv4()](IterableTcpMetrics::get_saddr_ipv4)\n- [.get_saddr_ipv6()](IterableTcpMetrics::get_saddr_ipv6)\n\n"]
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
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushTcpMetrics<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushTcpMetrics::new(buf)
    }
    pub fn encode(&mut self) -> PushTcpMetrics<&mut Vec<u8>> {
        PushTcpMetrics::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushTcpMetrics<RequestBuf<'r>> {
        PushTcpMetrics::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableTcpMetrics<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableTcpMetrics::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("tcp_metrics".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableTcpMetrics<'buf>;
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
#[doc = "Retrieve metrics.\n\nRequest attributes:\n- [.push_addr_ipv4()](PushTcpMetrics::push_addr_ipv4)\n- [.push_addr_ipv6()](PushTcpMetrics::push_addr_ipv6)\n- [.push_saddr_ipv4()](PushTcpMetrics::push_saddr_ipv4)\n- [.push_saddr_ipv6()](PushTcpMetrics::push_saddr_ipv6)\n\nReply attributes:\n- [.get_addr_ipv4()](IterableTcpMetrics::get_addr_ipv4)\n- [.get_addr_ipv6()](IterableTcpMetrics::get_addr_ipv6)\n- [.get_age()](IterableTcpMetrics::get_age)\n- [.get_vals()](IterableTcpMetrics::get_vals)\n- [.get_fopen_mss()](IterableTcpMetrics::get_fopen_mss)\n- [.get_fopen_syn_drops()](IterableTcpMetrics::get_fopen_syn_drops)\n- [.get_fopen_syn_drop_ts()](IterableTcpMetrics::get_fopen_syn_drop_ts)\n- [.get_fopen_cookie()](IterableTcpMetrics::get_fopen_cookie)\n- [.get_saddr_ipv4()](IterableTcpMetrics::get_saddr_ipv4)\n- [.get_saddr_ipv6()](IterableTcpMetrics::get_saddr_ipv6)\n\n"]
#[derive(Debug)]
pub struct OpGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushTcpMetrics<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushTcpMetrics::new(buf)
    }
    pub fn encode(&mut self) -> PushTcpMetrics<&mut Vec<u8>> {
        PushTcpMetrics::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushTcpMetrics<RequestBuf<'r>> {
        PushTcpMetrics::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableTcpMetrics<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableTcpMetrics::with_loc(attrs, buf.as_ptr() as usize)
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
impl NetlinkRequest for OpGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("tcp_metrics".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableTcpMetrics<'buf>;
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
#[doc = "Delete metrics.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_addr_ipv4()](PushTcpMetrics::push_addr_ipv4)\n- [.push_addr_ipv6()](PushTcpMetrics::push_addr_ipv6)\n- [.push_saddr_ipv4()](PushTcpMetrics::push_saddr_ipv4)\n- [.push_saddr_ipv6()](PushTcpMetrics::push_saddr_ipv6)\n\n"]
#[derive(Debug)]
pub struct OpDelDo<'r> {
    request: Request<'r>,
}
impl<'r> OpDelDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushTcpMetrics<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushTcpMetrics::new(buf)
    }
    pub fn encode(&mut self) -> PushTcpMetrics<&mut Vec<u8>> {
        PushTcpMetrics::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushTcpMetrics<RequestBuf<'r>> {
        PushTcpMetrics::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableTcpMetrics<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableTcpMetrics::with_loc(attrs, buf.as_ptr() as usize)
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
        Protocol::Generic("tcp_metrics".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableTcpMetrics<'buf>;
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
    #[doc = "Retrieve metrics.\n\nReply attributes:\n- [.get_addr_ipv4()](IterableTcpMetrics::get_addr_ipv4)\n- [.get_addr_ipv6()](IterableTcpMetrics::get_addr_ipv6)\n- [.get_age()](IterableTcpMetrics::get_age)\n- [.get_vals()](IterableTcpMetrics::get_vals)\n- [.get_fopen_mss()](IterableTcpMetrics::get_fopen_mss)\n- [.get_fopen_syn_drops()](IterableTcpMetrics::get_fopen_syn_drops)\n- [.get_fopen_syn_drop_ts()](IterableTcpMetrics::get_fopen_syn_drop_ts)\n- [.get_fopen_cookie()](IterableTcpMetrics::get_fopen_cookie)\n- [.get_saddr_ipv4()](IterableTcpMetrics::get_saddr_ipv4)\n- [.get_saddr_ipv6()](IterableTcpMetrics::get_saddr_ipv6)\n\n"]
    pub fn op_get_dump(self) -> OpGetDump<'buf> {
        let mut res = OpGetDump::new(self);
        res.request
            .do_writeback(res.protocol(), "op-get-dump", OpGetDump::lookup);
        res
    }
    #[doc = "Retrieve metrics.\n\nRequest attributes:\n- [.push_addr_ipv4()](PushTcpMetrics::push_addr_ipv4)\n- [.push_addr_ipv6()](PushTcpMetrics::push_addr_ipv6)\n- [.push_saddr_ipv4()](PushTcpMetrics::push_saddr_ipv4)\n- [.push_saddr_ipv6()](PushTcpMetrics::push_saddr_ipv6)\n\nReply attributes:\n- [.get_addr_ipv4()](IterableTcpMetrics::get_addr_ipv4)\n- [.get_addr_ipv6()](IterableTcpMetrics::get_addr_ipv6)\n- [.get_age()](IterableTcpMetrics::get_age)\n- [.get_vals()](IterableTcpMetrics::get_vals)\n- [.get_fopen_mss()](IterableTcpMetrics::get_fopen_mss)\n- [.get_fopen_syn_drops()](IterableTcpMetrics::get_fopen_syn_drops)\n- [.get_fopen_syn_drop_ts()](IterableTcpMetrics::get_fopen_syn_drop_ts)\n- [.get_fopen_cookie()](IterableTcpMetrics::get_fopen_cookie)\n- [.get_saddr_ipv4()](IterableTcpMetrics::get_saddr_ipv4)\n- [.get_saddr_ipv6()](IterableTcpMetrics::get_saddr_ipv6)\n\n"]
    pub fn op_get_do(self) -> OpGetDo<'buf> {
        let mut res = OpGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-get-do", OpGetDo::lookup);
        res
    }
    #[doc = "Delete metrics.\n\nFlags: admin-perm\n\nRequest attributes:\n- [.push_addr_ipv4()](PushTcpMetrics::push_addr_ipv4)\n- [.push_addr_ipv6()](PushTcpMetrics::push_addr_ipv6)\n- [.push_saddr_ipv4()](PushTcpMetrics::push_saddr_ipv4)\n- [.push_saddr_ipv6()](PushTcpMetrics::push_saddr_ipv6)\n\n"]
    pub fn op_del_do(self) -> OpDelDo<'buf> {
        let mut res = OpDelDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-del-do", OpDelDo::lookup);
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterableTcpMetrics::get_addr_ipv4;
        let _ = IterableTcpMetrics::get_addr_ipv6;
        let _ = IterableTcpMetrics::get_age;
        let _ = IterableTcpMetrics::get_fopen_cookie;
        let _ = IterableTcpMetrics::get_fopen_mss;
        let _ = IterableTcpMetrics::get_fopen_syn_drop_ts;
        let _ = IterableTcpMetrics::get_fopen_syn_drops;
        let _ = IterableTcpMetrics::get_saddr_ipv4;
        let _ = IterableTcpMetrics::get_saddr_ipv6;
        let _ = IterableTcpMetrics::get_vals;
        let _ = PushTcpMetrics::<&mut Vec<u8>>::push_addr_ipv4;
        let _ = PushTcpMetrics::<&mut Vec<u8>>::push_addr_ipv6;
        let _ = PushTcpMetrics::<&mut Vec<u8>>::push_saddr_ipv4;
        let _ = PushTcpMetrics::<&mut Vec<u8>>::push_saddr_ipv6;
    }
}
