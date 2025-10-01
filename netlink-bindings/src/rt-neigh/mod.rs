#![doc = "IP neighbour management over rtnetlink."]
#![allow(clippy::all)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]
#![allow(unreachable_code)]
#![allow(unreachable_patterns)]
use crate::builtin::{PushBuiltinBitfield32, PushBuiltinNfgenmsg};
use crate::consts;
use crate::utils::*;
use crate::{NetlinkRequest, Protocol};
pub const PROTONAME: &CStr = c"rt-neigh";
pub const PROTONUM: u16 = 0u16;
#[doc = "Original name: \"nud-state\" (flags) - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Clone)]
pub enum NudState {
    Incomplete = 1 << 0,
    Reachable = 1 << 1,
    Stale = 1 << 2,
    Delay = 1 << 3,
    Probe = 1 << 4,
    Failed = 1 << 5,
    Noarp = 1 << 6,
    Permanent = 1 << 7,
}
#[doc = "Original name: \"ntf-flags\" (flags) - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Clone)]
pub enum NtfFlags {
    Use = 1 << 0,
    _Self = 1 << 1,
    Master = 1 << 2,
    Proxy = 1 << 3,
    ExtLearned = 1 << 4,
    Offloaded = 1 << 5,
    Sticky = 1 << 6,
    Router = 1 << 7,
}
#[doc = "Original name: \"ntf-ext-flags\" (flags) - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)"]
#[derive(Clone)]
pub enum NtfExtFlags {
    Managed = 1 << 0,
    Locked = 1 << 1,
    ExtValidated = 1 << 2,
}
#[doc = "Original name: \"rtm-type\" (enum) - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)"]
#[derive(Clone)]
pub enum RtmType {
    Unspec = 0,
    Unicast = 1,
    Local = 2,
    Broadcast = 3,
    Anycast = 4,
    Multicast = 5,
    Blackhole = 6,
    Unreachable = 7,
    Prohibit = 8,
    Throw = 9,
    Nat = 10,
    Xresolve = 11,
}
#[doc = "Original name: \"neighbour-attrs\""]
#[derive(Clone)]
pub enum NeighbourAttrs<'a> {
    Unspec(&'a [u8]),
    Dst(&'a [u8]),
    Lladdr(&'a [u8]),
    Cacheinfo(PushNdaCacheinfo),
    Probes(u32),
    Vlan(u16),
    Port(u16),
    Vni(u32),
    Ifindex(u32),
    Master(u32),
    LinkNetnsid(i32),
    SrcVni(u32),
    Protocol(u8),
    NhId(u32),
    FdbExtAttrs(&'a [u8]),
    #[doc = "Associated type: \"NtfExtFlags\" (enum)"]
    FlagsExt(u32),
    NdmStateMask(u16),
    NdmFlagsMask(u8),
}
impl<'a> Iterable<'a, NeighbourAttrs<'a>> {
    pub fn get_unspec(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NeighbourAttrs::Unspec(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NeighbourAttrs", "Unspec"))
    }
    pub fn get_dst(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NeighbourAttrs::Dst(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NeighbourAttrs", "Dst"))
    }
    pub fn get_lladdr(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NeighbourAttrs::Lladdr(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NeighbourAttrs", "Lladdr"))
    }
    pub fn get_cacheinfo(&self) -> Result<PushNdaCacheinfo, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NeighbourAttrs::Cacheinfo(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NeighbourAttrs", "Cacheinfo"))
    }
    pub fn get_probes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NeighbourAttrs::Probes(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NeighbourAttrs", "Probes"))
    }
    pub fn get_vlan(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NeighbourAttrs::Vlan(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NeighbourAttrs", "Vlan"))
    }
    pub fn get_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NeighbourAttrs::Port(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NeighbourAttrs", "Port"))
    }
    pub fn get_vni(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NeighbourAttrs::Vni(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NeighbourAttrs", "Vni"))
    }
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NeighbourAttrs::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NeighbourAttrs", "Ifindex"))
    }
    pub fn get_master(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NeighbourAttrs::Master(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NeighbourAttrs", "Master"))
    }
    pub fn get_link_netnsid(&self) -> Result<i32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NeighbourAttrs::LinkNetnsid(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NeighbourAttrs", "LinkNetnsid"))
    }
    pub fn get_src_vni(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NeighbourAttrs::SrcVni(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NeighbourAttrs", "SrcVni"))
    }
    pub fn get_protocol(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NeighbourAttrs::Protocol(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NeighbourAttrs", "Protocol"))
    }
    pub fn get_nh_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NeighbourAttrs::NhId(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NeighbourAttrs", "NhId"))
    }
    pub fn get_fdb_ext_attrs(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NeighbourAttrs::FdbExtAttrs(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NeighbourAttrs", "FdbExtAttrs"))
    }
    #[doc = "Associated type: \"NtfExtFlags\" (enum)"]
    pub fn get_flags_ext(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NeighbourAttrs::FlagsExt(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NeighbourAttrs", "FlagsExt"))
    }
    pub fn get_ndm_state_mask(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NeighbourAttrs::NdmStateMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NeighbourAttrs", "NdmStateMask"))
    }
    pub fn get_ndm_flags_mask(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NeighbourAttrs::NdmFlagsMask(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NeighbourAttrs", "NdmFlagsMask"))
    }
}
impl<'a> NeighbourAttrs<'a> {
    pub fn new(buf: &'a [u8]) -> Iterable<'a, NeighbourAttrs<'a>> {
        Iterable::new(buf)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            0u16 => "Unspec",
            1u16 => "Dst",
            2u16 => "Lladdr",
            3u16 => "Cacheinfo",
            4u16 => "Probes",
            5u16 => "Vlan",
            6u16 => "Port",
            7u16 => "Vni",
            8u16 => "Ifindex",
            9u16 => "Master",
            10u16 => "LinkNetnsid",
            11u16 => "SrcVni",
            12u16 => "Protocol",
            13u16 => "NhId",
            14u16 => "FdbExtAttrs",
            15u16 => "FlagsExt",
            16u16 => "NdmStateMask",
            17u16 => "NdmFlagsMask",
            _ => return None,
        };
        Some(res)
    }
}
impl<'a> Iterator for Iterable<'a, NeighbourAttrs<'a>> {
    type Item = Result<NeighbourAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                0u16 => NeighbourAttrs::Unspec({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                1u16 => NeighbourAttrs::Dst({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => NeighbourAttrs::Lladdr({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => NeighbourAttrs::Cacheinfo({
                    let res = PushNdaCacheinfo::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => NeighbourAttrs::Probes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => NeighbourAttrs::Vlan({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => NeighbourAttrs::Port({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => NeighbourAttrs::Vni({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => NeighbourAttrs::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => NeighbourAttrs::Master({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => NeighbourAttrs::LinkNetnsid({
                    let res = parse_i32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => NeighbourAttrs::SrcVni({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => NeighbourAttrs::Protocol({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => NeighbourAttrs::NhId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => NeighbourAttrs::FdbExtAttrs({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => NeighbourAttrs::FlagsExt({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => NeighbourAttrs::NdmStateMask({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => NeighbourAttrs::NdmFlagsMask({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(test) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "NeighbourAttrs",
            r#type.and_then(|t| NeighbourAttrs::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, NeighbourAttrs<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("NeighbourAttrs");
        for attr in self.clone() {
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
                NeighbourAttrs::Unspec(val) => fmt.field("Unspec", &val),
                NeighbourAttrs::Dst(val) => fmt.field("Dst", &val),
                NeighbourAttrs::Lladdr(val) => fmt.field("Lladdr", &val),
                NeighbourAttrs::Cacheinfo(val) => fmt.field("Cacheinfo", &val),
                NeighbourAttrs::Probes(val) => fmt.field("Probes", &val),
                NeighbourAttrs::Vlan(val) => fmt.field("Vlan", &val),
                NeighbourAttrs::Port(val) => fmt.field("Port", &val),
                NeighbourAttrs::Vni(val) => fmt.field("Vni", &val),
                NeighbourAttrs::Ifindex(val) => fmt.field("Ifindex", &val),
                NeighbourAttrs::Master(val) => fmt.field("Master", &val),
                NeighbourAttrs::LinkNetnsid(val) => fmt.field("LinkNetnsid", &val),
                NeighbourAttrs::SrcVni(val) => fmt.field("SrcVni", &val),
                NeighbourAttrs::Protocol(val) => fmt.field("Protocol", &val),
                NeighbourAttrs::NhId(val) => fmt.field("NhId", &val),
                NeighbourAttrs::FdbExtAttrs(val) => fmt.field("FdbExtAttrs", &val),
                NeighbourAttrs::FlagsExt(val) => fmt.field("FlagsExt", &val),
                NeighbourAttrs::NdmStateMask(val) => fmt.field("NdmStateMask", &val),
                NeighbourAttrs::NdmFlagsMask(val) => fmt.field("NdmFlagsMask", &val),
            };
        }
        fmt.finish()
    }
}
impl<'a> Iterable<'a, NeighbourAttrs<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("NeighbourAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| NeighbourAttrs::attr_from_type(t)),
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
                NeighbourAttrs::Unspec(val) => {
                    if last_off == offset {
                        stack.push(("Unspec", last_off));
                        break;
                    }
                }
                NeighbourAttrs::Dst(val) => {
                    if last_off == offset {
                        stack.push(("Dst", last_off));
                        break;
                    }
                }
                NeighbourAttrs::Lladdr(val) => {
                    if last_off == offset {
                        stack.push(("Lladdr", last_off));
                        break;
                    }
                }
                NeighbourAttrs::Cacheinfo(val) => {
                    if last_off == offset {
                        stack.push(("Cacheinfo", last_off));
                        break;
                    }
                }
                NeighbourAttrs::Probes(val) => {
                    if last_off == offset {
                        stack.push(("Probes", last_off));
                        break;
                    }
                }
                NeighbourAttrs::Vlan(val) => {
                    if last_off == offset {
                        stack.push(("Vlan", last_off));
                        break;
                    }
                }
                NeighbourAttrs::Port(val) => {
                    if last_off == offset {
                        stack.push(("Port", last_off));
                        break;
                    }
                }
                NeighbourAttrs::Vni(val) => {
                    if last_off == offset {
                        stack.push(("Vni", last_off));
                        break;
                    }
                }
                NeighbourAttrs::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                NeighbourAttrs::Master(val) => {
                    if last_off == offset {
                        stack.push(("Master", last_off));
                        break;
                    }
                }
                NeighbourAttrs::LinkNetnsid(val) => {
                    if last_off == offset {
                        stack.push(("LinkNetnsid", last_off));
                        break;
                    }
                }
                NeighbourAttrs::SrcVni(val) => {
                    if last_off == offset {
                        stack.push(("SrcVni", last_off));
                        break;
                    }
                }
                NeighbourAttrs::Protocol(val) => {
                    if last_off == offset {
                        stack.push(("Protocol", last_off));
                        break;
                    }
                }
                NeighbourAttrs::NhId(val) => {
                    if last_off == offset {
                        stack.push(("NhId", last_off));
                        break;
                    }
                }
                NeighbourAttrs::FdbExtAttrs(val) => {
                    if last_off == offset {
                        stack.push(("FdbExtAttrs", last_off));
                        break;
                    }
                }
                NeighbourAttrs::FlagsExt(val) => {
                    if last_off == offset {
                        stack.push(("FlagsExt", last_off));
                        break;
                    }
                }
                NeighbourAttrs::NdmStateMask(val) => {
                    if last_off == offset {
                        stack.push(("NdmStateMask", last_off));
                        break;
                    }
                }
                NeighbourAttrs::NdmFlagsMask(val) => {
                    if last_off == offset {
                        stack.push(("NdmFlagsMask", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("NeighbourAttrs", cur));
        }
        (stack, None)
    }
}
#[doc = "Original name: \"ndt-attrs\""]
#[derive(Clone)]
pub enum NdtAttrs<'a> {
    Name(&'a CStr),
    Thresh1(u32),
    Thresh2(u32),
    Thresh3(u32),
    Config(PushNdtConfig),
    Parms(Iterable<'a, NdtpaAttrs<'a>>),
    Stats(PushNdtStats),
    GcInterval(u64),
    Pad(&'a [u8]),
}
impl<'a> Iterable<'a, NdtAttrs<'a>> {
    pub fn get_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtAttrs::Name(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtAttrs", "Name"))
    }
    pub fn get_thresh1(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtAttrs::Thresh1(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtAttrs", "Thresh1"))
    }
    pub fn get_thresh2(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtAttrs::Thresh2(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtAttrs", "Thresh2"))
    }
    pub fn get_thresh3(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtAttrs::Thresh3(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtAttrs", "Thresh3"))
    }
    pub fn get_config(&self) -> Result<PushNdtConfig, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtAttrs::Config(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtAttrs", "Config"))
    }
    pub fn get_parms(&self) -> Result<Iterable<'a, NdtpaAttrs<'a>>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtAttrs::Parms(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtAttrs", "Parms"))
    }
    pub fn get_stats(&self) -> Result<PushNdtStats, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtAttrs::Stats(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtAttrs", "Stats"))
    }
    pub fn get_gc_interval(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtAttrs::GcInterval(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtAttrs", "GcInterval"))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtAttrs::Pad(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtAttrs", "Pad"))
    }
}
impl<'a> NdtAttrs<'a> {
    pub fn new(buf: &'a [u8]) -> Iterable<'a, NdtAttrs<'a>> {
        Iterable::new(buf)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Name",
            2u16 => "Thresh1",
            3u16 => "Thresh2",
            4u16 => "Thresh3",
            5u16 => "Config",
            6u16 => "Parms",
            7u16 => "Stats",
            8u16 => "GcInterval",
            9u16 => "Pad",
            _ => return None,
        };
        Some(res)
    }
}
impl<'a> Iterator for Iterable<'a, NdtAttrs<'a>> {
    type Item = Result<NdtAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => NdtAttrs::Name({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => NdtAttrs::Thresh1({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => NdtAttrs::Thresh2({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => NdtAttrs::Thresh3({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => NdtAttrs::Config({
                    let res = PushNdtConfig::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => NdtAttrs::Parms({
                    let res = Some(Iterable::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => NdtAttrs::Stats({
                    let res = PushNdtStats::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => NdtAttrs::GcInterval({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => NdtAttrs::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(test) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "NdtAttrs",
            r#type.and_then(|t| NdtAttrs::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, NdtAttrs<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("NdtAttrs");
        for attr in self.clone() {
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
                NdtAttrs::Name(val) => fmt.field("Name", &val),
                NdtAttrs::Thresh1(val) => fmt.field("Thresh1", &val),
                NdtAttrs::Thresh2(val) => fmt.field("Thresh2", &val),
                NdtAttrs::Thresh3(val) => fmt.field("Thresh3", &val),
                NdtAttrs::Config(val) => fmt.field("Config", &val),
                NdtAttrs::Parms(val) => fmt.field("Parms", &val),
                NdtAttrs::Stats(val) => fmt.field("Stats", &val),
                NdtAttrs::GcInterval(val) => fmt.field("GcInterval", &val),
                NdtAttrs::Pad(val) => fmt.field("Pad", &val),
            };
        }
        fmt.finish()
    }
}
impl<'a> Iterable<'a, NdtAttrs<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("NdtAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| NdtAttrs::attr_from_type(t)),
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
                NdtAttrs::Name(val) => {
                    if last_off == offset {
                        stack.push(("Name", last_off));
                        break;
                    }
                }
                NdtAttrs::Thresh1(val) => {
                    if last_off == offset {
                        stack.push(("Thresh1", last_off));
                        break;
                    }
                }
                NdtAttrs::Thresh2(val) => {
                    if last_off == offset {
                        stack.push(("Thresh2", last_off));
                        break;
                    }
                }
                NdtAttrs::Thresh3(val) => {
                    if last_off == offset {
                        stack.push(("Thresh3", last_off));
                        break;
                    }
                }
                NdtAttrs::Config(val) => {
                    if last_off == offset {
                        stack.push(("Config", last_off));
                        break;
                    }
                }
                NdtAttrs::Parms(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                NdtAttrs::Stats(val) => {
                    if last_off == offset {
                        stack.push(("Stats", last_off));
                        break;
                    }
                }
                NdtAttrs::GcInterval(val) => {
                    if last_off == offset {
                        stack.push(("GcInterval", last_off));
                        break;
                    }
                }
                NdtAttrs::Pad(val) => {
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
            stack.push(("NdtAttrs", cur));
        }
        (stack, missing)
    }
}
#[doc = "Original name: \"ndtpa-attrs\""]
#[derive(Clone)]
pub enum NdtpaAttrs<'a> {
    Ifindex(u32),
    Refcnt(u32),
    ReachableTime(u64),
    BaseReachableTime(u64),
    RetransTime(u64),
    GcStaletime(u64),
    DelayProbeTime(u64),
    QueueLen(u32),
    AppProbes(u32),
    UcastProbes(u32),
    McastProbes(u32),
    AnycastDelay(u64),
    ProxyDelay(u64),
    ProxyQlen(u32),
    Locktime(u64),
    QueueLenbytes(u32),
    McastReprobes(u32),
    Pad(&'a [u8]),
    IntervalProbeTimeMs(u64),
}
impl<'a> Iterable<'a, NdtpaAttrs<'a>> {
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtpaAttrs::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtpaAttrs", "Ifindex"))
    }
    pub fn get_refcnt(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtpaAttrs::Refcnt(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtpaAttrs", "Refcnt"))
    }
    pub fn get_reachable_time(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtpaAttrs::ReachableTime(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtpaAttrs", "ReachableTime"))
    }
    pub fn get_base_reachable_time(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtpaAttrs::BaseReachableTime(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtpaAttrs", "BaseReachableTime"))
    }
    pub fn get_retrans_time(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtpaAttrs::RetransTime(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtpaAttrs", "RetransTime"))
    }
    pub fn get_gc_staletime(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtpaAttrs::GcStaletime(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtpaAttrs", "GcStaletime"))
    }
    pub fn get_delay_probe_time(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtpaAttrs::DelayProbeTime(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtpaAttrs", "DelayProbeTime"))
    }
    pub fn get_queue_len(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtpaAttrs::QueueLen(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtpaAttrs", "QueueLen"))
    }
    pub fn get_app_probes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtpaAttrs::AppProbes(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtpaAttrs", "AppProbes"))
    }
    pub fn get_ucast_probes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtpaAttrs::UcastProbes(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtpaAttrs", "UcastProbes"))
    }
    pub fn get_mcast_probes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtpaAttrs::McastProbes(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtpaAttrs", "McastProbes"))
    }
    pub fn get_anycast_delay(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtpaAttrs::AnycastDelay(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtpaAttrs", "AnycastDelay"))
    }
    pub fn get_proxy_delay(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtpaAttrs::ProxyDelay(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtpaAttrs", "ProxyDelay"))
    }
    pub fn get_proxy_qlen(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtpaAttrs::ProxyQlen(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtpaAttrs", "ProxyQlen"))
    }
    pub fn get_locktime(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtpaAttrs::Locktime(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtpaAttrs", "Locktime"))
    }
    pub fn get_queue_lenbytes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtpaAttrs::QueueLenbytes(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtpaAttrs", "QueueLenbytes"))
    }
    pub fn get_mcast_reprobes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtpaAttrs::McastReprobes(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtpaAttrs", "McastReprobes"))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtpaAttrs::Pad(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtpaAttrs", "Pad"))
    }
    pub fn get_interval_probe_time_ms(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let NdtpaAttrs::IntervalProbeTimeMs(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("NdtpaAttrs", "IntervalProbeTimeMs"))
    }
}
impl<'a> NdtpaAttrs<'a> {
    pub fn new(buf: &'a [u8]) -> Iterable<'a, NdtpaAttrs<'a>> {
        Iterable::new(buf)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Ifindex",
            2u16 => "Refcnt",
            3u16 => "ReachableTime",
            4u16 => "BaseReachableTime",
            5u16 => "RetransTime",
            6u16 => "GcStaletime",
            7u16 => "DelayProbeTime",
            8u16 => "QueueLen",
            9u16 => "AppProbes",
            10u16 => "UcastProbes",
            11u16 => "McastProbes",
            12u16 => "AnycastDelay",
            13u16 => "ProxyDelay",
            14u16 => "ProxyQlen",
            15u16 => "Locktime",
            16u16 => "QueueLenbytes",
            17u16 => "McastReprobes",
            18u16 => "Pad",
            19u16 => "IntervalProbeTimeMs",
            _ => return None,
        };
        Some(res)
    }
}
impl<'a> Iterator for Iterable<'a, NdtpaAttrs<'a>> {
    type Item = Result<NdtpaAttrs<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => NdtpaAttrs::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => NdtpaAttrs::Refcnt({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => NdtpaAttrs::ReachableTime({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => NdtpaAttrs::BaseReachableTime({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => NdtpaAttrs::RetransTime({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => NdtpaAttrs::GcStaletime({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => NdtpaAttrs::DelayProbeTime({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => NdtpaAttrs::QueueLen({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => NdtpaAttrs::AppProbes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => NdtpaAttrs::UcastProbes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => NdtpaAttrs::McastProbes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => NdtpaAttrs::AnycastDelay({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => NdtpaAttrs::ProxyDelay({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => NdtpaAttrs::ProxyQlen({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => NdtpaAttrs::Locktime({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                16u16 => NdtpaAttrs::QueueLenbytes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                17u16 => NdtpaAttrs::McastReprobes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                18u16 => NdtpaAttrs::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                19u16 => NdtpaAttrs::IntervalProbeTimeMs({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(test) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "NdtpaAttrs",
            r#type.and_then(|t| NdtpaAttrs::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, NdtpaAttrs<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("NdtpaAttrs");
        for attr in self.clone() {
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
                NdtpaAttrs::Ifindex(val) => fmt.field("Ifindex", &val),
                NdtpaAttrs::Refcnt(val) => fmt.field("Refcnt", &val),
                NdtpaAttrs::ReachableTime(val) => fmt.field("ReachableTime", &val),
                NdtpaAttrs::BaseReachableTime(val) => fmt.field("BaseReachableTime", &val),
                NdtpaAttrs::RetransTime(val) => fmt.field("RetransTime", &val),
                NdtpaAttrs::GcStaletime(val) => fmt.field("GcStaletime", &val),
                NdtpaAttrs::DelayProbeTime(val) => fmt.field("DelayProbeTime", &val),
                NdtpaAttrs::QueueLen(val) => fmt.field("QueueLen", &val),
                NdtpaAttrs::AppProbes(val) => fmt.field("AppProbes", &val),
                NdtpaAttrs::UcastProbes(val) => fmt.field("UcastProbes", &val),
                NdtpaAttrs::McastProbes(val) => fmt.field("McastProbes", &val),
                NdtpaAttrs::AnycastDelay(val) => fmt.field("AnycastDelay", &val),
                NdtpaAttrs::ProxyDelay(val) => fmt.field("ProxyDelay", &val),
                NdtpaAttrs::ProxyQlen(val) => fmt.field("ProxyQlen", &val),
                NdtpaAttrs::Locktime(val) => fmt.field("Locktime", &val),
                NdtpaAttrs::QueueLenbytes(val) => fmt.field("QueueLenbytes", &val),
                NdtpaAttrs::McastReprobes(val) => fmt.field("McastReprobes", &val),
                NdtpaAttrs::Pad(val) => fmt.field("Pad", &val),
                NdtpaAttrs::IntervalProbeTimeMs(val) => fmt.field("IntervalProbeTimeMs", &val),
            };
        }
        fmt.finish()
    }
}
impl<'a> Iterable<'a, NdtpaAttrs<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset {
            stack.push(("NdtpaAttrs", offset));
            return (
                stack,
                missing_type.and_then(|t| NdtpaAttrs::attr_from_type(t)),
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
                NdtpaAttrs::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                NdtpaAttrs::Refcnt(val) => {
                    if last_off == offset {
                        stack.push(("Refcnt", last_off));
                        break;
                    }
                }
                NdtpaAttrs::ReachableTime(val) => {
                    if last_off == offset {
                        stack.push(("ReachableTime", last_off));
                        break;
                    }
                }
                NdtpaAttrs::BaseReachableTime(val) => {
                    if last_off == offset {
                        stack.push(("BaseReachableTime", last_off));
                        break;
                    }
                }
                NdtpaAttrs::RetransTime(val) => {
                    if last_off == offset {
                        stack.push(("RetransTime", last_off));
                        break;
                    }
                }
                NdtpaAttrs::GcStaletime(val) => {
                    if last_off == offset {
                        stack.push(("GcStaletime", last_off));
                        break;
                    }
                }
                NdtpaAttrs::DelayProbeTime(val) => {
                    if last_off == offset {
                        stack.push(("DelayProbeTime", last_off));
                        break;
                    }
                }
                NdtpaAttrs::QueueLen(val) => {
                    if last_off == offset {
                        stack.push(("QueueLen", last_off));
                        break;
                    }
                }
                NdtpaAttrs::AppProbes(val) => {
                    if last_off == offset {
                        stack.push(("AppProbes", last_off));
                        break;
                    }
                }
                NdtpaAttrs::UcastProbes(val) => {
                    if last_off == offset {
                        stack.push(("UcastProbes", last_off));
                        break;
                    }
                }
                NdtpaAttrs::McastProbes(val) => {
                    if last_off == offset {
                        stack.push(("McastProbes", last_off));
                        break;
                    }
                }
                NdtpaAttrs::AnycastDelay(val) => {
                    if last_off == offset {
                        stack.push(("AnycastDelay", last_off));
                        break;
                    }
                }
                NdtpaAttrs::ProxyDelay(val) => {
                    if last_off == offset {
                        stack.push(("ProxyDelay", last_off));
                        break;
                    }
                }
                NdtpaAttrs::ProxyQlen(val) => {
                    if last_off == offset {
                        stack.push(("ProxyQlen", last_off));
                        break;
                    }
                }
                NdtpaAttrs::Locktime(val) => {
                    if last_off == offset {
                        stack.push(("Locktime", last_off));
                        break;
                    }
                }
                NdtpaAttrs::QueueLenbytes(val) => {
                    if last_off == offset {
                        stack.push(("QueueLenbytes", last_off));
                        break;
                    }
                }
                NdtpaAttrs::McastReprobes(val) => {
                    if last_off == offset {
                        stack.push(("McastReprobes", last_off));
                        break;
                    }
                }
                NdtpaAttrs::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                NdtpaAttrs::IntervalProbeTimeMs(val) => {
                    if last_off == offset {
                        stack.push(("IntervalProbeTimeMs", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("NdtpaAttrs", cur));
        }
        (stack, None)
    }
}
pub struct PushNeighbourAttrs<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushNeighbourAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushNeighbourAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_unspec(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 0u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_dst(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 1u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_lladdr(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_cacheinfo(mut self, value: PushNdaCacheinfo) -> Self {
        push_header(self.as_rec_mut(), 3u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_probes(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_vlan(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 5u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_port(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 6u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_vni(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 7u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_master(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 9u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_link_netnsid(mut self, value: i32) -> Self {
        push_header(self.as_rec_mut(), 10u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_src_vni(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 11u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_protocol(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 12u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_nh_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 13u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fdb_ext_attrs(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 14u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    #[doc = "Associated type: \"NtfExtFlags\" (enum)"]
    pub fn push_flags_ext(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 15u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ndm_state_mask(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 16u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ndm_flags_mask(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 17u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushNeighbourAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushNdtAttrs<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushNdtAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushNdtAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_thresh1(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_thresh2(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_thresh3(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_config(mut self, value: PushNdtConfig) -> Self {
        push_header(self.as_rec_mut(), 5u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn nested_parms(mut self) -> PushNdtpaAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 6u16);
        PushNdtpaAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_stats(mut self, value: PushNdtStats) -> Self {
        push_header(self.as_rec_mut(), 7u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_gc_interval(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 8u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 9u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
}
impl<Prev: Rec> Drop for PushNdtAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushNdtpaAttrs<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushNdtpaAttrs<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushNdtpaAttrs<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_refcnt(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_reachable_time(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 3u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_base_reachable_time(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 4u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_retrans_time(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 5u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_gc_staletime(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 6u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_delay_probe_time(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 7u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_queue_len(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_app_probes(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 9u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ucast_probes(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 10u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mcast_probes(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 11u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_anycast_delay(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 12u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_proxy_delay(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 13u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_proxy_qlen(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 14u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_locktime(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 15u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_queue_lenbytes(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 16u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_mcast_reprobes(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 17u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 18u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_interval_probe_time_ms(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 19u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushNdtpaAttrs<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Original name: \"ndmsg\""]
#[derive(Clone)]
pub struct PushNdmsg {
    buf: [u8; 12usize],
}
impl PushNdmsg {
    #[doc = "Create zero-initialized struct"]
    pub fn new() -> Self {
        Self {
            buf: [0u8; Self::len()],
        }
    }
    #[doc = "Copy from contents from other slice"]
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
        12usize
    }
    pub fn ndm_family(&self) -> u8 {
        parse_u8(&self.buf[0usize..1usize]).unwrap()
    }
    pub fn set_ndm_family(&mut self, value: u8) {
        self.buf[0usize..1usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn ndm_ifindex(&self) -> i32 {
        parse_i32(&self.buf[4usize..8usize]).unwrap()
    }
    pub fn set_ndm_ifindex(&mut self, value: i32) {
        self.buf[4usize..8usize].copy_from_slice(&value.to_ne_bytes())
    }
    #[doc = "Associated type: \"NudState\" (enum)"]
    pub fn ndm_state(&self) -> u16 {
        parse_u16(&self.buf[8usize..10usize]).unwrap()
    }
    #[doc = "Associated type: \"NudState\" (enum)"]
    pub fn set_ndm_state(&mut self, value: u16) {
        self.buf[8usize..10usize].copy_from_slice(&value.to_ne_bytes())
    }
    #[doc = "Associated type: \"NtfFlags\" (enum)"]
    pub fn ndm_flags(&self) -> u8 {
        parse_u8(&self.buf[10usize..11usize]).unwrap()
    }
    #[doc = "Associated type: \"NtfFlags\" (enum)"]
    pub fn set_ndm_flags(&mut self, value: u8) {
        self.buf[10usize..11usize].copy_from_slice(&value.to_ne_bytes())
    }
    #[doc = "Associated type: \"RtmType\" (enum)"]
    pub fn ndm_type(&self) -> u8 {
        parse_u8(&self.buf[11usize..12usize]).unwrap()
    }
    #[doc = "Associated type: \"RtmType\" (enum)"]
    pub fn set_ndm_type(&mut self, value: u8) {
        self.buf[11usize..12usize].copy_from_slice(&value.to_ne_bytes())
    }
}
impl std::fmt::Debug for PushNdmsg {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Ndmsg")
            .field("ndm_family", &self.ndm_family())
            .field("ndm_ifindex", &self.ndm_ifindex())
            .field("ndm_state", &self.ndm_state())
            .field("ndm_flags", &self.ndm_flags())
            .field("ndm_type", &self.ndm_type())
            .finish()
    }
}
#[doc = "Original name: \"ndtmsg\""]
#[derive(Clone)]
pub struct PushNdtmsg {
    buf: [u8; 4usize],
}
impl PushNdtmsg {
    #[doc = "Create zero-initialized struct"]
    pub fn new() -> Self {
        Self {
            buf: [0u8; Self::len()],
        }
    }
    #[doc = "Copy from contents from other slice"]
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
    pub fn family(&self) -> u8 {
        parse_u8(&self.buf[0usize..1usize]).unwrap()
    }
    pub fn set_family(&mut self, value: u8) {
        self.buf[0usize..1usize].copy_from_slice(&value.to_ne_bytes())
    }
}
impl std::fmt::Debug for PushNdtmsg {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Ndtmsg")
            .field("family", &self.family())
            .finish()
    }
}
#[doc = "Original name: \"nda-cacheinfo\""]
#[derive(Clone)]
pub struct PushNdaCacheinfo {
    buf: [u8; 16usize],
}
impl PushNdaCacheinfo {
    #[doc = "Create zero-initialized struct"]
    pub fn new() -> Self {
        Self {
            buf: [0u8; Self::len()],
        }
    }
    #[doc = "Copy from contents from other slice"]
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
        16usize
    }
    pub fn confirmed(&self) -> u32 {
        parse_u32(&self.buf[0usize..4usize]).unwrap()
    }
    pub fn set_confirmed(&mut self, value: u32) {
        self.buf[0usize..4usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn used(&self) -> u32 {
        parse_u32(&self.buf[4usize..8usize]).unwrap()
    }
    pub fn set_used(&mut self, value: u32) {
        self.buf[4usize..8usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn updated(&self) -> u32 {
        parse_u32(&self.buf[8usize..12usize]).unwrap()
    }
    pub fn set_updated(&mut self, value: u32) {
        self.buf[8usize..12usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn refcnt(&self) -> u32 {
        parse_u32(&self.buf[12usize..16usize]).unwrap()
    }
    pub fn set_refcnt(&mut self, value: u32) {
        self.buf[12usize..16usize].copy_from_slice(&value.to_ne_bytes())
    }
}
impl std::fmt::Debug for PushNdaCacheinfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("NdaCacheinfo")
            .field("confirmed", &self.confirmed())
            .field("used", &self.used())
            .field("updated", &self.updated())
            .field("refcnt", &self.refcnt())
            .finish()
    }
}
#[doc = "Original name: \"ndt-config\""]
#[derive(Clone)]
pub struct PushNdtConfig {
    buf: [u8; 32usize],
}
impl PushNdtConfig {
    #[doc = "Create zero-initialized struct"]
    pub fn new() -> Self {
        Self {
            buf: [0u8; Self::len()],
        }
    }
    #[doc = "Copy from contents from other slice"]
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
        32usize
    }
    pub fn key_len(&self) -> u16 {
        parse_u16(&self.buf[0usize..2usize]).unwrap()
    }
    pub fn set_key_len(&mut self, value: u16) {
        self.buf[0usize..2usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn entry_size(&self) -> u16 {
        parse_u16(&self.buf[2usize..4usize]).unwrap()
    }
    pub fn set_entry_size(&mut self, value: u16) {
        self.buf[2usize..4usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn entries(&self) -> u32 {
        parse_u32(&self.buf[4usize..8usize]).unwrap()
    }
    pub fn set_entries(&mut self, value: u32) {
        self.buf[4usize..8usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn last_flush(&self) -> u32 {
        parse_u32(&self.buf[8usize..12usize]).unwrap()
    }
    pub fn set_last_flush(&mut self, value: u32) {
        self.buf[8usize..12usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn last_rand(&self) -> u32 {
        parse_u32(&self.buf[12usize..16usize]).unwrap()
    }
    pub fn set_last_rand(&mut self, value: u32) {
        self.buf[12usize..16usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn hash_rnd(&self) -> u32 {
        parse_u32(&self.buf[16usize..20usize]).unwrap()
    }
    pub fn set_hash_rnd(&mut self, value: u32) {
        self.buf[16usize..20usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn hash_mask(&self) -> u32 {
        parse_u32(&self.buf[20usize..24usize]).unwrap()
    }
    pub fn set_hash_mask(&mut self, value: u32) {
        self.buf[20usize..24usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn hash_chain_gc(&self) -> u32 {
        parse_u32(&self.buf[24usize..28usize]).unwrap()
    }
    pub fn set_hash_chain_gc(&mut self, value: u32) {
        self.buf[24usize..28usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn proxy_qlen(&self) -> u32 {
        parse_u32(&self.buf[28usize..32usize]).unwrap()
    }
    pub fn set_proxy_qlen(&mut self, value: u32) {
        self.buf[28usize..32usize].copy_from_slice(&value.to_ne_bytes())
    }
}
impl std::fmt::Debug for PushNdtConfig {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("NdtConfig")
            .field("key_len", &self.key_len())
            .field("entry_size", &self.entry_size())
            .field("entries", &self.entries())
            .field("last_flush", &self.last_flush())
            .field("last_rand", &self.last_rand())
            .field("hash_rnd", &self.hash_rnd())
            .field("hash_mask", &self.hash_mask())
            .field("hash_chain_gc", &self.hash_chain_gc())
            .field("proxy_qlen", &self.proxy_qlen())
            .finish()
    }
}
#[doc = "Original name: \"ndt-stats\""]
#[derive(Clone)]
pub struct PushNdtStats {
    buf: [u8; 88usize],
}
impl PushNdtStats {
    #[doc = "Create zero-initialized struct"]
    pub fn new() -> Self {
        Self {
            buf: [0u8; Self::len()],
        }
    }
    #[doc = "Copy from contents from other slice"]
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
        88usize
    }
    pub fn allocs(&self) -> u64 {
        parse_u64(&self.buf[0usize..8usize]).unwrap()
    }
    pub fn set_allocs(&mut self, value: u64) {
        self.buf[0usize..8usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn destroys(&self) -> u64 {
        parse_u64(&self.buf[8usize..16usize]).unwrap()
    }
    pub fn set_destroys(&mut self, value: u64) {
        self.buf[8usize..16usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn hash_grows(&self) -> u64 {
        parse_u64(&self.buf[16usize..24usize]).unwrap()
    }
    pub fn set_hash_grows(&mut self, value: u64) {
        self.buf[16usize..24usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn res_failed(&self) -> u64 {
        parse_u64(&self.buf[24usize..32usize]).unwrap()
    }
    pub fn set_res_failed(&mut self, value: u64) {
        self.buf[24usize..32usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn lookups(&self) -> u64 {
        parse_u64(&self.buf[32usize..40usize]).unwrap()
    }
    pub fn set_lookups(&mut self, value: u64) {
        self.buf[32usize..40usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn hits(&self) -> u64 {
        parse_u64(&self.buf[40usize..48usize]).unwrap()
    }
    pub fn set_hits(&mut self, value: u64) {
        self.buf[40usize..48usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn rcv_probes_mcast(&self) -> u64 {
        parse_u64(&self.buf[48usize..56usize]).unwrap()
    }
    pub fn set_rcv_probes_mcast(&mut self, value: u64) {
        self.buf[48usize..56usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn rcv_probes_ucast(&self) -> u64 {
        parse_u64(&self.buf[56usize..64usize]).unwrap()
    }
    pub fn set_rcv_probes_ucast(&mut self, value: u64) {
        self.buf[56usize..64usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn periodic_gc_runs(&self) -> u64 {
        parse_u64(&self.buf[64usize..72usize]).unwrap()
    }
    pub fn set_periodic_gc_runs(&mut self, value: u64) {
        self.buf[64usize..72usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn forced_gc_runs(&self) -> u64 {
        parse_u64(&self.buf[72usize..80usize]).unwrap()
    }
    pub fn set_forced_gc_runs(&mut self, value: u64) {
        self.buf[72usize..80usize].copy_from_slice(&value.to_ne_bytes())
    }
    pub fn table_fulls(&self) -> u64 {
        parse_u64(&self.buf[80usize..88usize]).unwrap()
    }
    pub fn set_table_fulls(&mut self, value: u64) {
        self.buf[80usize..88usize].copy_from_slice(&value.to_ne_bytes())
    }
}
impl std::fmt::Debug for PushNdtStats {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("NdtStats")
            .field("allocs", &self.allocs())
            .field("destroys", &self.destroys())
            .field("hash_grows", &self.hash_grows())
            .field("res_failed", &self.res_failed())
            .field("lookups", &self.lookups())
            .field("hits", &self.hits())
            .field("rcv_probes_mcast", &self.rcv_probes_mcast())
            .field("rcv_probes_ucast", &self.rcv_probes_ucast())
            .field("periodic_gc_runs", &self.periodic_gc_runs())
            .field("forced_gc_runs", &self.forced_gc_runs())
            .field("table_fulls", &self.table_fulls())
            .finish()
    }
}
#[doc = "Add new neighbour entry"]
pub struct PushOpNewneighDoRequest<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpNewneighDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpNewneighDoRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushNdmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushNdmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_dst(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 1u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_lladdr(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_probes(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_vlan(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 5u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_port(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 6u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_vni(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 7u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_master(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 9u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_protocol(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 12u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_nh_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 13u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fdb_ext_attrs(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 14u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    #[doc = "Associated type: \"NtfExtFlags\" (enum)"]
    pub fn push_flags_ext(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 15u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpNewneighDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Add new neighbour entry"]
#[doc = "Original name: \"op-newneigh-do-request\""]
#[derive(Clone)]
pub enum OpNewneighDoRequest<'a> {
    Dst(&'a [u8]),
    Lladdr(&'a [u8]),
    Probes(u32),
    Vlan(u16),
    Port(u16),
    Vni(u32),
    Ifindex(u32),
    Master(u32),
    Protocol(u8),
    NhId(u32),
    FdbExtAttrs(&'a [u8]),
    #[doc = "Associated type: \"NtfExtFlags\" (enum)"]
    FlagsExt(u32),
}
impl<'a> Iterable<'a, OpNewneighDoRequest<'a>> {
    pub fn get_dst(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewneighDoRequest::Dst(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpNewneighDoRequest", "Dst"))
    }
    pub fn get_lladdr(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewneighDoRequest::Lladdr(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpNewneighDoRequest", "Lladdr"))
    }
    pub fn get_probes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewneighDoRequest::Probes(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpNewneighDoRequest", "Probes"))
    }
    pub fn get_vlan(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewneighDoRequest::Vlan(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpNewneighDoRequest", "Vlan"))
    }
    pub fn get_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewneighDoRequest::Port(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpNewneighDoRequest", "Port"))
    }
    pub fn get_vni(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewneighDoRequest::Vni(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpNewneighDoRequest", "Vni"))
    }
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewneighDoRequest::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpNewneighDoRequest", "Ifindex"))
    }
    pub fn get_master(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewneighDoRequest::Master(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpNewneighDoRequest", "Master"))
    }
    pub fn get_protocol(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewneighDoRequest::Protocol(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpNewneighDoRequest", "Protocol"))
    }
    pub fn get_nh_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewneighDoRequest::NhId(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpNewneighDoRequest", "NhId"))
    }
    pub fn get_fdb_ext_attrs(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewneighDoRequest::FdbExtAttrs(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpNewneighDoRequest", "FdbExtAttrs"))
    }
    #[doc = "Associated type: \"NtfExtFlags\" (enum)"]
    pub fn get_flags_ext(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpNewneighDoRequest::FlagsExt(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpNewneighDoRequest", "FlagsExt"))
    }
}
impl<'a> OpNewneighDoRequest<'a> {
    pub fn new(buf: &'a [u8]) -> (PushNdmsg, Iterable<'a, OpNewneighDoRequest<'a>>) {
        let mut header = PushNdmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushNdmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushNdmsg::len()..], buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        NeighbourAttrs::attr_from_type(r#type)
    }
}
impl<'a> Iterator for Iterable<'a, OpNewneighDoRequest<'a>> {
    type Item = Result<OpNewneighDoRequest<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpNewneighDoRequest::Dst({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpNewneighDoRequest::Lladdr({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpNewneighDoRequest::Probes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpNewneighDoRequest::Vlan({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpNewneighDoRequest::Port({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => OpNewneighDoRequest::Vni({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpNewneighDoRequest::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => OpNewneighDoRequest::Master({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => OpNewneighDoRequest::Protocol({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => OpNewneighDoRequest::NhId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => OpNewneighDoRequest::FdbExtAttrs({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => OpNewneighDoRequest::FlagsExt({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(test) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpNewneighDoRequest",
            r#type.and_then(|t| OpNewneighDoRequest::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpNewneighDoRequest<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpNewneighDoRequest");
        for attr in self.clone() {
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
                OpNewneighDoRequest::Dst(val) => fmt.field("Dst", &val),
                OpNewneighDoRequest::Lladdr(val) => fmt.field("Lladdr", &val),
                OpNewneighDoRequest::Probes(val) => fmt.field("Probes", &val),
                OpNewneighDoRequest::Vlan(val) => fmt.field("Vlan", &val),
                OpNewneighDoRequest::Port(val) => fmt.field("Port", &val),
                OpNewneighDoRequest::Vni(val) => fmt.field("Vni", &val),
                OpNewneighDoRequest::Ifindex(val) => fmt.field("Ifindex", &val),
                OpNewneighDoRequest::Master(val) => fmt.field("Master", &val),
                OpNewneighDoRequest::Protocol(val) => fmt.field("Protocol", &val),
                OpNewneighDoRequest::NhId(val) => fmt.field("NhId", &val),
                OpNewneighDoRequest::FdbExtAttrs(val) => fmt.field("FdbExtAttrs", &val),
                OpNewneighDoRequest::FlagsExt(val) => fmt.field("FlagsExt", &val),
            };
        }
        fmt.finish()
    }
}
impl<'a> Iterable<'a, OpNewneighDoRequest<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset + PushNdmsg::len() {
            stack.push(("OpNewneighDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpNewneighDoRequest::attr_from_type(t)),
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
                OpNewneighDoRequest::Dst(val) => {
                    if last_off == offset {
                        stack.push(("Dst", last_off));
                        break;
                    }
                }
                OpNewneighDoRequest::Lladdr(val) => {
                    if last_off == offset {
                        stack.push(("Lladdr", last_off));
                        break;
                    }
                }
                OpNewneighDoRequest::Probes(val) => {
                    if last_off == offset {
                        stack.push(("Probes", last_off));
                        break;
                    }
                }
                OpNewneighDoRequest::Vlan(val) => {
                    if last_off == offset {
                        stack.push(("Vlan", last_off));
                        break;
                    }
                }
                OpNewneighDoRequest::Port(val) => {
                    if last_off == offset {
                        stack.push(("Port", last_off));
                        break;
                    }
                }
                OpNewneighDoRequest::Vni(val) => {
                    if last_off == offset {
                        stack.push(("Vni", last_off));
                        break;
                    }
                }
                OpNewneighDoRequest::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OpNewneighDoRequest::Master(val) => {
                    if last_off == offset {
                        stack.push(("Master", last_off));
                        break;
                    }
                }
                OpNewneighDoRequest::Protocol(val) => {
                    if last_off == offset {
                        stack.push(("Protocol", last_off));
                        break;
                    }
                }
                OpNewneighDoRequest::NhId(val) => {
                    if last_off == offset {
                        stack.push(("NhId", last_off));
                        break;
                    }
                }
                OpNewneighDoRequest::FdbExtAttrs(val) => {
                    if last_off == offset {
                        stack.push(("FdbExtAttrs", last_off));
                        break;
                    }
                }
                OpNewneighDoRequest::FlagsExt(val) => {
                    if last_off == offset {
                        stack.push(("FlagsExt", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpNewneighDoRequest", cur));
        }
        (stack, None)
    }
}
#[doc = "Add new neighbour entry"]
pub struct PushOpNewneighDoReply<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpNewneighDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpNewneighDoReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushNdmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushNdmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
}
impl<Prev: Rec> Drop for PushOpNewneighDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Add new neighbour entry"]
#[doc = "Original name: \"op-newneigh-do-reply\""]
#[derive(Clone)]
pub enum OpNewneighDoReply {}
impl<'a> Iterable<'a, OpNewneighDoReply> {}
impl OpNewneighDoReply {
    pub fn new(buf: &'_ [u8]) -> (PushNdmsg, Iterable<'_, OpNewneighDoReply>) {
        let mut header = PushNdmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushNdmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushNdmsg::len()..], buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        NeighbourAttrs::attr_from_type(r#type)
    }
}
impl Iterator for Iterable<'_, OpNewneighDoReply> {
    type Item = Result<OpNewneighDoReply, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                n => {
                    if cfg!(test) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpNewneighDoReply",
            r#type.and_then(|t| OpNewneighDoReply::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl std::fmt::Debug for Iterable<'_, OpNewneighDoReply> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpNewneighDoReply");
        for attr in self.clone() {
            let attr = match attr {
                Ok(a) => a,
                Err(err) => {
                    fmt.finish()?;
                    f.write_str("Err(")?;
                    err.fmt(f)?;
                    return f.write_str(")");
                }
            };
            match attr {};
        }
        fmt.finish()
    }
}
impl Iterable<'_, OpNewneighDoReply> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset + PushNdmsg::len() {
            stack.push(("OpNewneighDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpNewneighDoReply::attr_from_type(t)),
            );
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpNewneighDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpNewneighDoRequest<'r> {
    pub fn new(mut request: Request<'r>, header: &PushNdmsg) -> Self {
        PushOpNewneighDoRequest::write_header(&mut request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpNewneighDoRequest<&mut Vec<u8>> {
        PushOpNewneighDoRequest::new_without_header(self.request.buf_mut())
    }
}
impl NetlinkRequest for RequestOpNewneighDoRequest<'_> {
    type ReplyType<'buf> = (PushNdmsg, Iterable<'buf, OpNewneighDoReply>);
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 28u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpNewneighDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpNewneighDoRequest::new(buf)
            .1
            .lookup_attr(offset, missing_type)
    }
}
#[doc = "Remove an existing neighbour entry"]
pub struct PushOpDelneighDoRequest<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpDelneighDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpDelneighDoRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushNdmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushNdmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_dst(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 1u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpDelneighDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Remove an existing neighbour entry"]
#[doc = "Original name: \"op-delneigh-do-request\""]
#[derive(Clone)]
pub enum OpDelneighDoRequest<'a> {
    Dst(&'a [u8]),
    Ifindex(u32),
}
impl<'a> Iterable<'a, OpDelneighDoRequest<'a>> {
    pub fn get_dst(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelneighDoRequest::Dst(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpDelneighDoRequest", "Dst"))
    }
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpDelneighDoRequest::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpDelneighDoRequest", "Ifindex"))
    }
}
impl<'a> OpDelneighDoRequest<'a> {
    pub fn new(buf: &'a [u8]) -> (PushNdmsg, Iterable<'a, OpDelneighDoRequest<'a>>) {
        let mut header = PushNdmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushNdmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushNdmsg::len()..], buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        NeighbourAttrs::attr_from_type(r#type)
    }
}
impl<'a> Iterator for Iterable<'a, OpDelneighDoRequest<'a>> {
    type Item = Result<OpDelneighDoRequest<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpDelneighDoRequest::Dst({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpDelneighDoRequest::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(test) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpDelneighDoRequest",
            r#type.and_then(|t| OpDelneighDoRequest::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpDelneighDoRequest<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpDelneighDoRequest");
        for attr in self.clone() {
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
                OpDelneighDoRequest::Dst(val) => fmt.field("Dst", &val),
                OpDelneighDoRequest::Ifindex(val) => fmt.field("Ifindex", &val),
            };
        }
        fmt.finish()
    }
}
impl<'a> Iterable<'a, OpDelneighDoRequest<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset + PushNdmsg::len() {
            stack.push(("OpDelneighDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpDelneighDoRequest::attr_from_type(t)),
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
                OpDelneighDoRequest::Dst(val) => {
                    if last_off == offset {
                        stack.push(("Dst", last_off));
                        break;
                    }
                }
                OpDelneighDoRequest::Ifindex(val) => {
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
            stack.push(("OpDelneighDoRequest", cur));
        }
        (stack, None)
    }
}
#[doc = "Remove an existing neighbour entry"]
pub struct PushOpDelneighDoReply<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpDelneighDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpDelneighDoReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushNdmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushNdmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
}
impl<Prev: Rec> Drop for PushOpDelneighDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Remove an existing neighbour entry"]
#[doc = "Original name: \"op-delneigh-do-reply\""]
#[derive(Clone)]
pub enum OpDelneighDoReply {}
impl<'a> Iterable<'a, OpDelneighDoReply> {}
impl OpDelneighDoReply {
    pub fn new(buf: &'_ [u8]) -> (PushNdmsg, Iterable<'_, OpDelneighDoReply>) {
        let mut header = PushNdmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushNdmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushNdmsg::len()..], buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        NeighbourAttrs::attr_from_type(r#type)
    }
}
impl Iterator for Iterable<'_, OpDelneighDoReply> {
    type Item = Result<OpDelneighDoReply, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                n => {
                    if cfg!(test) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpDelneighDoReply",
            r#type.and_then(|t| OpDelneighDoReply::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl std::fmt::Debug for Iterable<'_, OpDelneighDoReply> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpDelneighDoReply");
        for attr in self.clone() {
            let attr = match attr {
                Ok(a) => a,
                Err(err) => {
                    fmt.finish()?;
                    f.write_str("Err(")?;
                    err.fmt(f)?;
                    return f.write_str(")");
                }
            };
            match attr {};
        }
        fmt.finish()
    }
}
impl Iterable<'_, OpDelneighDoReply> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset + PushNdmsg::len() {
            stack.push(("OpDelneighDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpDelneighDoReply::attr_from_type(t)),
            );
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpDelneighDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpDelneighDoRequest<'r> {
    pub fn new(mut request: Request<'r>, header: &PushNdmsg) -> Self {
        PushOpDelneighDoRequest::write_header(&mut request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpDelneighDoRequest<&mut Vec<u8>> {
        PushOpDelneighDoRequest::new_without_header(self.request.buf_mut())
    }
}
impl NetlinkRequest for RequestOpDelneighDoRequest<'_> {
    type ReplyType<'buf> = (PushNdmsg, Iterable<'buf, OpDelneighDoReply>);
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 29u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpDelneighDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpDelneighDoRequest::new(buf)
            .1
            .lookup_attr(offset, missing_type)
    }
}
#[doc = "Get or dump neighbour entries"]
pub struct PushOpGetneighDumpRequest<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetneighDumpRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetneighDumpRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushNdmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushNdmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_master(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 9u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpGetneighDumpRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get or dump neighbour entries"]
#[doc = "Original name: \"op-getneigh-dump-request\""]
#[derive(Clone)]
pub enum OpGetneighDumpRequest {
    Ifindex(u32),
    Master(u32),
}
impl<'a> Iterable<'a, OpGetneighDumpRequest> {
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDumpRequest::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDumpRequest", "Ifindex"))
    }
    pub fn get_master(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDumpRequest::Master(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDumpRequest", "Master"))
    }
}
impl OpGetneighDumpRequest {
    pub fn new(buf: &'_ [u8]) -> (PushNdmsg, Iterable<'_, OpGetneighDumpRequest>) {
        let mut header = PushNdmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushNdmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushNdmsg::len()..], buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        NeighbourAttrs::attr_from_type(r#type)
    }
}
impl Iterator for Iterable<'_, OpGetneighDumpRequest> {
    type Item = Result<OpGetneighDumpRequest, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                8u16 => OpGetneighDumpRequest::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => OpGetneighDumpRequest::Master({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(test) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpGetneighDumpRequest",
            r#type.and_then(|t| OpGetneighDumpRequest::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl std::fmt::Debug for Iterable<'_, OpGetneighDumpRequest> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetneighDumpRequest");
        for attr in self.clone() {
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
                OpGetneighDumpRequest::Ifindex(val) => fmt.field("Ifindex", &val),
                OpGetneighDumpRequest::Master(val) => fmt.field("Master", &val),
            };
        }
        fmt.finish()
    }
}
impl Iterable<'_, OpGetneighDumpRequest> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset + PushNdmsg::len() {
            stack.push(("OpGetneighDumpRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetneighDumpRequest::attr_from_type(t)),
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
                OpGetneighDumpRequest::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OpGetneighDumpRequest::Master(val) => {
                    if last_off == offset {
                        stack.push(("Master", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetneighDumpRequest", cur));
        }
        (stack, None)
    }
}
#[doc = "Get or dump neighbour entries"]
pub struct PushOpGetneighDumpReply<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetneighDumpReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetneighDumpReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushNdmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushNdmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_dst(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 1u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_lladdr(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_probes(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_vlan(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 5u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_port(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 6u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_vni(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 7u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_master(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 9u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_protocol(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 12u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_nh_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 13u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fdb_ext_attrs(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 14u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    #[doc = "Associated type: \"NtfExtFlags\" (enum)"]
    pub fn push_flags_ext(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 15u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpGetneighDumpReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get or dump neighbour entries"]
#[doc = "Original name: \"op-getneigh-dump-reply\""]
#[derive(Clone)]
pub enum OpGetneighDumpReply<'a> {
    Dst(&'a [u8]),
    Lladdr(&'a [u8]),
    Probes(u32),
    Vlan(u16),
    Port(u16),
    Vni(u32),
    Ifindex(u32),
    Master(u32),
    Protocol(u8),
    NhId(u32),
    FdbExtAttrs(&'a [u8]),
    #[doc = "Associated type: \"NtfExtFlags\" (enum)"]
    FlagsExt(u32),
}
impl<'a> Iterable<'a, OpGetneighDumpReply<'a>> {
    pub fn get_dst(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDumpReply::Dst(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDumpReply", "Dst"))
    }
    pub fn get_lladdr(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDumpReply::Lladdr(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDumpReply", "Lladdr"))
    }
    pub fn get_probes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDumpReply::Probes(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDumpReply", "Probes"))
    }
    pub fn get_vlan(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDumpReply::Vlan(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDumpReply", "Vlan"))
    }
    pub fn get_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDumpReply::Port(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDumpReply", "Port"))
    }
    pub fn get_vni(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDumpReply::Vni(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDumpReply", "Vni"))
    }
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDumpReply::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDumpReply", "Ifindex"))
    }
    pub fn get_master(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDumpReply::Master(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDumpReply", "Master"))
    }
    pub fn get_protocol(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDumpReply::Protocol(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDumpReply", "Protocol"))
    }
    pub fn get_nh_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDumpReply::NhId(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDumpReply", "NhId"))
    }
    pub fn get_fdb_ext_attrs(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDumpReply::FdbExtAttrs(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDumpReply", "FdbExtAttrs"))
    }
    #[doc = "Associated type: \"NtfExtFlags\" (enum)"]
    pub fn get_flags_ext(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDumpReply::FlagsExt(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDumpReply", "FlagsExt"))
    }
}
impl<'a> OpGetneighDumpReply<'a> {
    pub fn new(buf: &'a [u8]) -> (PushNdmsg, Iterable<'a, OpGetneighDumpReply<'a>>) {
        let mut header = PushNdmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushNdmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushNdmsg::len()..], buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        NeighbourAttrs::attr_from_type(r#type)
    }
}
impl<'a> Iterator for Iterable<'a, OpGetneighDumpReply<'a>> {
    type Item = Result<OpGetneighDumpReply<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpGetneighDumpReply::Dst({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpGetneighDumpReply::Lladdr({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpGetneighDumpReply::Probes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpGetneighDumpReply::Vlan({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpGetneighDumpReply::Port({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => OpGetneighDumpReply::Vni({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpGetneighDumpReply::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => OpGetneighDumpReply::Master({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => OpGetneighDumpReply::Protocol({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => OpGetneighDumpReply::NhId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => OpGetneighDumpReply::FdbExtAttrs({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => OpGetneighDumpReply::FlagsExt({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(test) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpGetneighDumpReply",
            r#type.and_then(|t| OpGetneighDumpReply::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpGetneighDumpReply<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetneighDumpReply");
        for attr in self.clone() {
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
                OpGetneighDumpReply::Dst(val) => fmt.field("Dst", &val),
                OpGetneighDumpReply::Lladdr(val) => fmt.field("Lladdr", &val),
                OpGetneighDumpReply::Probes(val) => fmt.field("Probes", &val),
                OpGetneighDumpReply::Vlan(val) => fmt.field("Vlan", &val),
                OpGetneighDumpReply::Port(val) => fmt.field("Port", &val),
                OpGetneighDumpReply::Vni(val) => fmt.field("Vni", &val),
                OpGetneighDumpReply::Ifindex(val) => fmt.field("Ifindex", &val),
                OpGetneighDumpReply::Master(val) => fmt.field("Master", &val),
                OpGetneighDumpReply::Protocol(val) => fmt.field("Protocol", &val),
                OpGetneighDumpReply::NhId(val) => fmt.field("NhId", &val),
                OpGetneighDumpReply::FdbExtAttrs(val) => fmt.field("FdbExtAttrs", &val),
                OpGetneighDumpReply::FlagsExt(val) => fmt.field("FlagsExt", &val),
            };
        }
        fmt.finish()
    }
}
impl<'a> Iterable<'a, OpGetneighDumpReply<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset + PushNdmsg::len() {
            stack.push(("OpGetneighDumpReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetneighDumpReply::attr_from_type(t)),
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
                OpGetneighDumpReply::Dst(val) => {
                    if last_off == offset {
                        stack.push(("Dst", last_off));
                        break;
                    }
                }
                OpGetneighDumpReply::Lladdr(val) => {
                    if last_off == offset {
                        stack.push(("Lladdr", last_off));
                        break;
                    }
                }
                OpGetneighDumpReply::Probes(val) => {
                    if last_off == offset {
                        stack.push(("Probes", last_off));
                        break;
                    }
                }
                OpGetneighDumpReply::Vlan(val) => {
                    if last_off == offset {
                        stack.push(("Vlan", last_off));
                        break;
                    }
                }
                OpGetneighDumpReply::Port(val) => {
                    if last_off == offset {
                        stack.push(("Port", last_off));
                        break;
                    }
                }
                OpGetneighDumpReply::Vni(val) => {
                    if last_off == offset {
                        stack.push(("Vni", last_off));
                        break;
                    }
                }
                OpGetneighDumpReply::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OpGetneighDumpReply::Master(val) => {
                    if last_off == offset {
                        stack.push(("Master", last_off));
                        break;
                    }
                }
                OpGetneighDumpReply::Protocol(val) => {
                    if last_off == offset {
                        stack.push(("Protocol", last_off));
                        break;
                    }
                }
                OpGetneighDumpReply::NhId(val) => {
                    if last_off == offset {
                        stack.push(("NhId", last_off));
                        break;
                    }
                }
                OpGetneighDumpReply::FdbExtAttrs(val) => {
                    if last_off == offset {
                        stack.push(("FdbExtAttrs", last_off));
                        break;
                    }
                }
                OpGetneighDumpReply::FlagsExt(val) => {
                    if last_off == offset {
                        stack.push(("FlagsExt", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetneighDumpReply", cur));
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpGetneighDumpRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpGetneighDumpRequest<'r> {
    pub fn new(mut request: Request<'r>, header: &PushNdmsg) -> Self {
        PushOpGetneighDumpRequest::write_header(&mut request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode(&mut self) -> PushOpGetneighDumpRequest<&mut Vec<u8>> {
        PushOpGetneighDumpRequest::new_without_header(self.request.buf_mut())
    }
}
impl NetlinkRequest for RequestOpGetneighDumpRequest<'_> {
    type ReplyType<'buf> = (PushNdmsg, Iterable<'buf, OpGetneighDumpReply<'buf>>);
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 30u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpGetneighDumpReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpGetneighDumpRequest::new(buf)
            .1
            .lookup_attr(offset, missing_type)
    }
}
#[doc = "Get or dump neighbour entries"]
pub struct PushOpGetneighDoRequest<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetneighDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetneighDoRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushNdmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushNdmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_dst(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 1u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
}
impl<Prev: Rec> Drop for PushOpGetneighDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get or dump neighbour entries"]
#[doc = "Original name: \"op-getneigh-do-request\""]
#[derive(Clone)]
pub enum OpGetneighDoRequest<'a> {
    Dst(&'a [u8]),
}
impl<'a> Iterable<'a, OpGetneighDoRequest<'a>> {
    pub fn get_dst(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDoRequest::Dst(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDoRequest", "Dst"))
    }
}
impl<'a> OpGetneighDoRequest<'a> {
    pub fn new(buf: &'a [u8]) -> (PushNdmsg, Iterable<'a, OpGetneighDoRequest<'a>>) {
        let mut header = PushNdmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushNdmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushNdmsg::len()..], buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        NeighbourAttrs::attr_from_type(r#type)
    }
}
impl<'a> Iterator for Iterable<'a, OpGetneighDoRequest<'a>> {
    type Item = Result<OpGetneighDoRequest<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpGetneighDoRequest::Dst({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(test) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpGetneighDoRequest",
            r#type.and_then(|t| OpGetneighDoRequest::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpGetneighDoRequest<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetneighDoRequest");
        for attr in self.clone() {
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
                OpGetneighDoRequest::Dst(val) => fmt.field("Dst", &val),
            };
        }
        fmt.finish()
    }
}
impl<'a> Iterable<'a, OpGetneighDoRequest<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset + PushNdmsg::len() {
            stack.push(("OpGetneighDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetneighDoRequest::attr_from_type(t)),
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
                OpGetneighDoRequest::Dst(val) => {
                    if last_off == offset {
                        stack.push(("Dst", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetneighDoRequest", cur));
        }
        (stack, None)
    }
}
#[doc = "Get or dump neighbour entries"]
pub struct PushOpGetneighDoReply<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetneighDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetneighDoReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushNdmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushNdmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_dst(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 1u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_lladdr(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_probes(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_vlan(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 5u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_port(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 6u16, 2 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_vni(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 7u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_ifindex(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_master(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 9u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_protocol(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 12u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_nh_id(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 13u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_fdb_ext_attrs(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 14u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    #[doc = "Associated type: \"NtfExtFlags\" (enum)"]
    pub fn push_flags_ext(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 15u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpGetneighDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get or dump neighbour entries"]
#[doc = "Original name: \"op-getneigh-do-reply\""]
#[derive(Clone)]
pub enum OpGetneighDoReply<'a> {
    Dst(&'a [u8]),
    Lladdr(&'a [u8]),
    Probes(u32),
    Vlan(u16),
    Port(u16),
    Vni(u32),
    Ifindex(u32),
    Master(u32),
    Protocol(u8),
    NhId(u32),
    FdbExtAttrs(&'a [u8]),
    #[doc = "Associated type: \"NtfExtFlags\" (enum)"]
    FlagsExt(u32),
}
impl<'a> Iterable<'a, OpGetneighDoReply<'a>> {
    pub fn get_dst(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDoReply::Dst(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDoReply", "Dst"))
    }
    pub fn get_lladdr(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDoReply::Lladdr(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDoReply", "Lladdr"))
    }
    pub fn get_probes(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDoReply::Probes(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDoReply", "Probes"))
    }
    pub fn get_vlan(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDoReply::Vlan(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDoReply", "Vlan"))
    }
    pub fn get_port(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDoReply::Port(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDoReply", "Port"))
    }
    pub fn get_vni(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDoReply::Vni(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDoReply", "Vni"))
    }
    pub fn get_ifindex(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDoReply::Ifindex(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDoReply", "Ifindex"))
    }
    pub fn get_master(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDoReply::Master(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDoReply", "Master"))
    }
    pub fn get_protocol(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDoReply::Protocol(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDoReply", "Protocol"))
    }
    pub fn get_nh_id(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDoReply::NhId(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDoReply", "NhId"))
    }
    pub fn get_fdb_ext_attrs(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDoReply::FdbExtAttrs(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDoReply", "FdbExtAttrs"))
    }
    #[doc = "Associated type: \"NtfExtFlags\" (enum)"]
    pub fn get_flags_ext(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneighDoReply::FlagsExt(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneighDoReply", "FlagsExt"))
    }
}
impl<'a> OpGetneighDoReply<'a> {
    pub fn new(buf: &'a [u8]) -> (PushNdmsg, Iterable<'a, OpGetneighDoReply<'a>>) {
        let mut header = PushNdmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushNdmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushNdmsg::len()..], buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        NeighbourAttrs::attr_from_type(r#type)
    }
}
impl<'a> Iterator for Iterable<'a, OpGetneighDoReply<'a>> {
    type Item = Result<OpGetneighDoReply<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpGetneighDoReply::Dst({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpGetneighDoReply::Lladdr({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpGetneighDoReply::Probes({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpGetneighDoReply::Vlan({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpGetneighDoReply::Port({
                    let res = parse_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => OpGetneighDoReply::Vni({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpGetneighDoReply::Ifindex({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => OpGetneighDoReply::Master({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => OpGetneighDoReply::Protocol({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => OpGetneighDoReply::NhId({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => OpGetneighDoReply::FdbExtAttrs({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                15u16 => OpGetneighDoReply::FlagsExt({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(test) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpGetneighDoReply",
            r#type.and_then(|t| OpGetneighDoReply::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpGetneighDoReply<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetneighDoReply");
        for attr in self.clone() {
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
                OpGetneighDoReply::Dst(val) => fmt.field("Dst", &val),
                OpGetneighDoReply::Lladdr(val) => fmt.field("Lladdr", &val),
                OpGetneighDoReply::Probes(val) => fmt.field("Probes", &val),
                OpGetneighDoReply::Vlan(val) => fmt.field("Vlan", &val),
                OpGetneighDoReply::Port(val) => fmt.field("Port", &val),
                OpGetneighDoReply::Vni(val) => fmt.field("Vni", &val),
                OpGetneighDoReply::Ifindex(val) => fmt.field("Ifindex", &val),
                OpGetneighDoReply::Master(val) => fmt.field("Master", &val),
                OpGetneighDoReply::Protocol(val) => fmt.field("Protocol", &val),
                OpGetneighDoReply::NhId(val) => fmt.field("NhId", &val),
                OpGetneighDoReply::FdbExtAttrs(val) => fmt.field("FdbExtAttrs", &val),
                OpGetneighDoReply::FlagsExt(val) => fmt.field("FlagsExt", &val),
            };
        }
        fmt.finish()
    }
}
impl<'a> Iterable<'a, OpGetneighDoReply<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset + PushNdmsg::len() {
            stack.push(("OpGetneighDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetneighDoReply::attr_from_type(t)),
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
                OpGetneighDoReply::Dst(val) => {
                    if last_off == offset {
                        stack.push(("Dst", last_off));
                        break;
                    }
                }
                OpGetneighDoReply::Lladdr(val) => {
                    if last_off == offset {
                        stack.push(("Lladdr", last_off));
                        break;
                    }
                }
                OpGetneighDoReply::Probes(val) => {
                    if last_off == offset {
                        stack.push(("Probes", last_off));
                        break;
                    }
                }
                OpGetneighDoReply::Vlan(val) => {
                    if last_off == offset {
                        stack.push(("Vlan", last_off));
                        break;
                    }
                }
                OpGetneighDoReply::Port(val) => {
                    if last_off == offset {
                        stack.push(("Port", last_off));
                        break;
                    }
                }
                OpGetneighDoReply::Vni(val) => {
                    if last_off == offset {
                        stack.push(("Vni", last_off));
                        break;
                    }
                }
                OpGetneighDoReply::Ifindex(val) => {
                    if last_off == offset {
                        stack.push(("Ifindex", last_off));
                        break;
                    }
                }
                OpGetneighDoReply::Master(val) => {
                    if last_off == offset {
                        stack.push(("Master", last_off));
                        break;
                    }
                }
                OpGetneighDoReply::Protocol(val) => {
                    if last_off == offset {
                        stack.push(("Protocol", last_off));
                        break;
                    }
                }
                OpGetneighDoReply::NhId(val) => {
                    if last_off == offset {
                        stack.push(("NhId", last_off));
                        break;
                    }
                }
                OpGetneighDoReply::FdbExtAttrs(val) => {
                    if last_off == offset {
                        stack.push(("FdbExtAttrs", last_off));
                        break;
                    }
                }
                OpGetneighDoReply::FlagsExt(val) => {
                    if last_off == offset {
                        stack.push(("FlagsExt", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetneighDoReply", cur));
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpGetneighDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpGetneighDoRequest<'r> {
    pub fn new(mut request: Request<'r>, header: &PushNdmsg) -> Self {
        PushOpGetneighDoRequest::write_header(&mut request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpGetneighDoRequest<&mut Vec<u8>> {
        PushOpGetneighDoRequest::new_without_header(self.request.buf_mut())
    }
}
impl NetlinkRequest for RequestOpGetneighDoRequest<'_> {
    type ReplyType<'buf> = (PushNdmsg, Iterable<'buf, OpGetneighDoReply<'buf>>);
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 30u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpGetneighDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpGetneighDoRequest::new(buf)
            .1
            .lookup_attr(offset, missing_type)
    }
}
#[doc = "Get or dump neighbour tables"]
pub struct PushOpGetneightblDumpRequest<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetneightblDumpRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetneightblDumpRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushNdtmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushNdtmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
}
impl<Prev: Rec> Drop for PushOpGetneightblDumpRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get or dump neighbour tables"]
#[doc = "Original name: \"op-getneightbl-dump-request\""]
#[derive(Clone)]
pub enum OpGetneightblDumpRequest {}
impl<'a> Iterable<'a, OpGetneightblDumpRequest> {}
impl OpGetneightblDumpRequest {
    pub fn new(buf: &'_ [u8]) -> (PushNdtmsg, Iterable<'_, OpGetneightblDumpRequest>) {
        let mut header = PushNdtmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushNdtmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushNdtmsg::len()..], buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        NdtAttrs::attr_from_type(r#type)
    }
}
impl Iterator for Iterable<'_, OpGetneightblDumpRequest> {
    type Item = Result<OpGetneightblDumpRequest, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                n => {
                    if cfg!(test) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpGetneightblDumpRequest",
            r#type.and_then(|t| OpGetneightblDumpRequest::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl std::fmt::Debug for Iterable<'_, OpGetneightblDumpRequest> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetneightblDumpRequest");
        for attr in self.clone() {
            let attr = match attr {
                Ok(a) => a,
                Err(err) => {
                    fmt.finish()?;
                    f.write_str("Err(")?;
                    err.fmt(f)?;
                    return f.write_str(")");
                }
            };
            match attr {};
        }
        fmt.finish()
    }
}
impl Iterable<'_, OpGetneightblDumpRequest> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset + PushNdtmsg::len() {
            stack.push(("OpGetneightblDumpRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetneightblDumpRequest::attr_from_type(t)),
            );
        }
        (stack, None)
    }
}
#[doc = "Get or dump neighbour tables"]
pub struct PushOpGetneightblDumpReply<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpGetneightblDumpReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpGetneightblDumpReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushNdtmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushNdtmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_thresh1(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_thresh2(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_thresh3(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_config(mut self, value: PushNdtConfig) -> Self {
        push_header(self.as_rec_mut(), 5u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn nested_parms(mut self) -> PushNdtpaAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 6u16);
        PushNdtpaAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_stats(mut self, value: PushNdtStats) -> Self {
        push_header(self.as_rec_mut(), 7u16, value.as_slice().len() as u16);
        self.as_rec_mut().extend(value.as_slice());
        self
    }
    pub fn push_gc_interval(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 8u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpGetneightblDumpReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Get or dump neighbour tables"]
#[doc = "Original name: \"op-getneightbl-dump-reply\""]
#[derive(Clone)]
pub enum OpGetneightblDumpReply<'a> {
    Name(&'a CStr),
    Thresh1(u32),
    Thresh2(u32),
    Thresh3(u32),
    Config(PushNdtConfig),
    Parms(Iterable<'a, NdtpaAttrs<'a>>),
    Stats(PushNdtStats),
    GcInterval(u64),
}
impl<'a> Iterable<'a, OpGetneightblDumpReply<'a>> {
    pub fn get_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneightblDumpReply::Name(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneightblDumpReply", "Name"))
    }
    pub fn get_thresh1(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneightblDumpReply::Thresh1(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneightblDumpReply", "Thresh1"))
    }
    pub fn get_thresh2(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneightblDumpReply::Thresh2(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneightblDumpReply", "Thresh2"))
    }
    pub fn get_thresh3(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneightblDumpReply::Thresh3(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneightblDumpReply", "Thresh3"))
    }
    pub fn get_config(&self) -> Result<PushNdtConfig, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneightblDumpReply::Config(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneightblDumpReply", "Config"))
    }
    pub fn get_parms(&self) -> Result<Iterable<'a, NdtpaAttrs<'a>>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneightblDumpReply::Parms(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneightblDumpReply", "Parms"))
    }
    pub fn get_stats(&self) -> Result<PushNdtStats, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneightblDumpReply::Stats(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneightblDumpReply", "Stats"))
    }
    pub fn get_gc_interval(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpGetneightblDumpReply::GcInterval(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpGetneightblDumpReply", "GcInterval"))
    }
}
impl<'a> OpGetneightblDumpReply<'a> {
    pub fn new(buf: &'a [u8]) -> (PushNdtmsg, Iterable<'a, OpGetneightblDumpReply<'a>>) {
        let mut header = PushNdtmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushNdtmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushNdtmsg::len()..], buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        NdtAttrs::attr_from_type(r#type)
    }
}
impl<'a> Iterator for Iterable<'a, OpGetneightblDumpReply<'a>> {
    type Item = Result<OpGetneightblDumpReply<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpGetneightblDumpReply::Name({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpGetneightblDumpReply::Thresh1({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpGetneightblDumpReply::Thresh2({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpGetneightblDumpReply::Thresh3({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => OpGetneightblDumpReply::Config({
                    let res = PushNdtConfig::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpGetneightblDumpReply::Parms({
                    let res = Some(Iterable::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => OpGetneightblDumpReply::Stats({
                    let res = PushNdtStats::new_from_slice(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpGetneightblDumpReply::GcInterval({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(test) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpGetneightblDumpReply",
            r#type.and_then(|t| OpGetneightblDumpReply::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpGetneightblDumpReply<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpGetneightblDumpReply");
        for attr in self.clone() {
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
                OpGetneightblDumpReply::Name(val) => fmt.field("Name", &val),
                OpGetneightblDumpReply::Thresh1(val) => fmt.field("Thresh1", &val),
                OpGetneightblDumpReply::Thresh2(val) => fmt.field("Thresh2", &val),
                OpGetneightblDumpReply::Thresh3(val) => fmt.field("Thresh3", &val),
                OpGetneightblDumpReply::Config(val) => fmt.field("Config", &val),
                OpGetneightblDumpReply::Parms(val) => fmt.field("Parms", &val),
                OpGetneightblDumpReply::Stats(val) => fmt.field("Stats", &val),
                OpGetneightblDumpReply::GcInterval(val) => fmt.field("GcInterval", &val),
            };
        }
        fmt.finish()
    }
}
impl<'a> Iterable<'a, OpGetneightblDumpReply<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset + PushNdtmsg::len() {
            stack.push(("OpGetneightblDumpReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpGetneightblDumpReply::attr_from_type(t)),
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
                OpGetneightblDumpReply::Name(val) => {
                    if last_off == offset {
                        stack.push(("Name", last_off));
                        break;
                    }
                }
                OpGetneightblDumpReply::Thresh1(val) => {
                    if last_off == offset {
                        stack.push(("Thresh1", last_off));
                        break;
                    }
                }
                OpGetneightblDumpReply::Thresh2(val) => {
                    if last_off == offset {
                        stack.push(("Thresh2", last_off));
                        break;
                    }
                }
                OpGetneightblDumpReply::Thresh3(val) => {
                    if last_off == offset {
                        stack.push(("Thresh3", last_off));
                        break;
                    }
                }
                OpGetneightblDumpReply::Config(val) => {
                    if last_off == offset {
                        stack.push(("Config", last_off));
                        break;
                    }
                }
                OpGetneightblDumpReply::Parms(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpGetneightblDumpReply::Stats(val) => {
                    if last_off == offset {
                        stack.push(("Stats", last_off));
                        break;
                    }
                }
                OpGetneightblDumpReply::GcInterval(val) => {
                    if last_off == offset {
                        stack.push(("GcInterval", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpGetneightblDumpReply", cur));
        }
        (stack, missing)
    }
}
#[derive(Debug)]
pub struct RequestOpGetneightblDumpRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpGetneightblDumpRequest<'r> {
    pub fn new(mut request: Request<'r>, header: &PushNdtmsg) -> Self {
        PushOpGetneightblDumpRequest::write_header(&mut request.buf_mut(), header);
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode(&mut self) -> PushOpGetneightblDumpRequest<&mut Vec<u8>> {
        PushOpGetneightblDumpRequest::new_without_header(self.request.buf_mut())
    }
}
impl NetlinkRequest for RequestOpGetneightblDumpRequest<'_> {
    type ReplyType<'buf> = (PushNdtmsg, Iterable<'buf, OpGetneightblDumpReply<'buf>>);
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 66u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpGetneightblDumpReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpGetneightblDumpRequest::new(buf)
            .1
            .lookup_attr(offset, missing_type)
    }
}
#[doc = "Set neighbour tables"]
pub struct PushOpSetneightblDoRequest<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpSetneightblDoRequest<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpSetneightblDoRequest<Prev> {
    pub fn new(mut prev: Prev, header: &PushNdtmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushNdtmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_thresh1(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_thresh2(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_thresh3(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 4u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn nested_parms(mut self) -> PushNdtpaAttrs<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 6u16);
        PushNdtpaAttrs {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
    pub fn push_gc_interval(mut self, value: u64) -> Self {
        push_header(self.as_rec_mut(), 8u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushOpSetneightblDoRequest<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Set neighbour tables"]
#[doc = "Original name: \"op-setneightbl-do-request\""]
#[derive(Clone)]
pub enum OpSetneightblDoRequest<'a> {
    Name(&'a CStr),
    Thresh1(u32),
    Thresh2(u32),
    Thresh3(u32),
    Parms(Iterable<'a, NdtpaAttrs<'a>>),
    GcInterval(u64),
}
impl<'a> Iterable<'a, OpSetneightblDoRequest<'a>> {
    pub fn get_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpSetneightblDoRequest::Name(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpSetneightblDoRequest", "Name"))
    }
    pub fn get_thresh1(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpSetneightblDoRequest::Thresh1(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpSetneightblDoRequest", "Thresh1"))
    }
    pub fn get_thresh2(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpSetneightblDoRequest::Thresh2(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpSetneightblDoRequest", "Thresh2"))
    }
    pub fn get_thresh3(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpSetneightblDoRequest::Thresh3(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpSetneightblDoRequest", "Thresh3"))
    }
    pub fn get_parms(&self) -> Result<Iterable<'a, NdtpaAttrs<'a>>, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpSetneightblDoRequest::Parms(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpSetneightblDoRequest", "Parms"))
    }
    pub fn get_gc_interval(&self) -> Result<u64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let OpSetneightblDoRequest::GcInterval(val) = attr? {
                return Ok(val);
            }
        }
        Err(self.error_missing("OpSetneightblDoRequest", "GcInterval"))
    }
}
impl<'a> OpSetneightblDoRequest<'a> {
    pub fn new(buf: &'a [u8]) -> (PushNdtmsg, Iterable<'a, OpSetneightblDoRequest<'a>>) {
        let mut header = PushNdtmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushNdtmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushNdtmsg::len()..], buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        NdtAttrs::attr_from_type(r#type)
    }
}
impl<'a> Iterator for Iterable<'a, OpSetneightblDoRequest<'a>> {
    type Item = Result<OpSetneightblDoRequest<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => OpSetneightblDoRequest::Name({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => OpSetneightblDoRequest::Thresh1({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => OpSetneightblDoRequest::Thresh2({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => OpSetneightblDoRequest::Thresh3({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => OpSetneightblDoRequest::Parms({
                    let res = Some(Iterable::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => OpSetneightblDoRequest::GcInterval({
                    let res = parse_u64(next);
                    let Some(val) = res else { break };
                    val
                }),
                n => {
                    if cfg!(test) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpSetneightblDoRequest",
            r#type.and_then(|t| OpSetneightblDoRequest::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl<'a> std::fmt::Debug for Iterable<'a, OpSetneightblDoRequest<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpSetneightblDoRequest");
        for attr in self.clone() {
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
                OpSetneightblDoRequest::Name(val) => fmt.field("Name", &val),
                OpSetneightblDoRequest::Thresh1(val) => fmt.field("Thresh1", &val),
                OpSetneightblDoRequest::Thresh2(val) => fmt.field("Thresh2", &val),
                OpSetneightblDoRequest::Thresh3(val) => fmt.field("Thresh3", &val),
                OpSetneightblDoRequest::Parms(val) => fmt.field("Parms", &val),
                OpSetneightblDoRequest::GcInterval(val) => fmt.field("GcInterval", &val),
            };
        }
        fmt.finish()
    }
}
impl<'a> Iterable<'a, OpSetneightblDoRequest<'a>> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset + PushNdtmsg::len() {
            stack.push(("OpSetneightblDoRequest", offset));
            return (
                stack,
                missing_type.and_then(|t| OpSetneightblDoRequest::attr_from_type(t)),
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
                OpSetneightblDoRequest::Name(val) => {
                    if last_off == offset {
                        stack.push(("Name", last_off));
                        break;
                    }
                }
                OpSetneightblDoRequest::Thresh1(val) => {
                    if last_off == offset {
                        stack.push(("Thresh1", last_off));
                        break;
                    }
                }
                OpSetneightblDoRequest::Thresh2(val) => {
                    if last_off == offset {
                        stack.push(("Thresh2", last_off));
                        break;
                    }
                }
                OpSetneightblDoRequest::Thresh3(val) => {
                    if last_off == offset {
                        stack.push(("Thresh3", last_off));
                        break;
                    }
                }
                OpSetneightblDoRequest::Parms(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                OpSetneightblDoRequest::GcInterval(val) => {
                    if last_off == offset {
                        stack.push(("GcInterval", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("OpSetneightblDoRequest", cur));
        }
        (stack, missing)
    }
}
#[doc = "Set neighbour tables"]
pub struct PushOpSetneightblDoReply<Prev: Rec> {
    prev: Option<Prev>,
    header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushOpSetneightblDoReply<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
}
impl<Prev: Rec> PushOpSetneightblDoReply<Prev> {
    pub fn new(mut prev: Prev, header: &PushNdtmsg) -> Self {
        Self::write_header(&mut prev, header);
        Self::new_without_header(prev)
    }
    fn new_without_header(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    fn write_header(prev: &mut Prev, header: &PushNdtmsg) {
        prev.as_rec_mut().extend(header.as_slice());
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
}
impl<Prev: Rec> Drop for PushOpSetneightblDoReply<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "Set neighbour tables"]
#[doc = "Original name: \"op-setneightbl-do-reply\""]
#[derive(Clone)]
pub enum OpSetneightblDoReply {}
impl<'a> Iterable<'a, OpSetneightblDoReply> {}
impl OpSetneightblDoReply {
    pub fn new(buf: &'_ [u8]) -> (PushNdtmsg, Iterable<'_, OpSetneightblDoReply>) {
        let mut header = PushNdtmsg::new();
        header
            .as_mut_slice()
            .clone_from_slice(&buf[..PushNdtmsg::len()]);
        (
            header,
            Iterable::with_loc(&buf[PushNdtmsg::len()..], buf.as_ptr() as usize),
        )
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        NdtAttrs::attr_from_type(r#type)
    }
}
impl Iterator for Iterable<'_, OpSetneightblDoReply> {
    type Item = Result<OpSetneightblDoReply, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() == self.pos {
            return None;
        }
        let pos = self.pos;
        let mut r#type = None;
        while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
            r#type = Some(header.r#type);
            let res = match header.r#type {
                n => {
                    if cfg!(test) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
            return Some(Ok(res));
        }
        Some(Err(self.error_context(
            "OpSetneightblDoReply",
            r#type.and_then(|t| OpSetneightblDoReply::attr_from_type(t)),
            self.buf.as_ptr().wrapping_add(pos),
        )))
    }
}
impl std::fmt::Debug for Iterable<'_, OpSetneightblDoReply> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("OpSetneightblDoReply");
        for attr in self.clone() {
            let attr = match attr {
                Ok(a) => a,
                Err(err) => {
                    fmt.finish()?;
                    f.write_str("Err(")?;
                    err.fmt(f)?;
                    return f.write_str(")");
                }
            };
            match attr {};
        }
        fmt.finish()
    }
}
impl Iterable<'_, OpSetneightblDoReply> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = self.calc_offset(self.buf.as_ptr() as usize);
        if cur == offset + PushNdtmsg::len() {
            stack.push(("OpSetneightblDoReply", offset));
            return (
                stack,
                missing_type.and_then(|t| OpSetneightblDoReply::attr_from_type(t)),
            );
        }
        (stack, None)
    }
}
#[derive(Debug)]
pub struct RequestOpSetneightblDoRequest<'r> {
    request: Request<'r>,
}
impl<'r> RequestOpSetneightblDoRequest<'r> {
    pub fn new(mut request: Request<'r>, header: &PushNdtmsg) -> Self {
        PushOpSetneightblDoRequest::write_header(&mut request.buf_mut(), header);
        Self { request: request }
    }
    pub fn encode(&mut self) -> PushOpSetneightblDoRequest<&mut Vec<u8>> {
        PushOpSetneightblDoRequest::new_without_header(self.request.buf_mut())
    }
}
impl NetlinkRequest for RequestOpSetneightblDoRequest<'_> {
    type ReplyType<'buf> = (PushNdtmsg, Iterable<'buf, OpSetneightblDoReply>);
    fn protocol(&self) -> Protocol {
        Protocol::Raw {
            protonum: 0u16,
            request_type: 67u16,
        }
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        OpSetneightblDoReply::new(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        OpSetneightblDoRequest::new(buf)
            .1
            .lookup_attr(offset, missing_type)
    }
}
#[derive(Debug)]
enum RequestBuf<'buf> {
    Ref(&'buf mut Vec<u8>),
    Own(Vec<u8>),
}
#[derive(Debug)]
pub struct Request<'buf> {
    buf: RequestBuf<'buf>,
    flags: u16,
}
impl Request<'static> {
    pub fn new() -> Self {
        Self {
            flags: 0,
            buf: RequestBuf::Own(Vec::new()),
        }
    }
    pub fn from_buf(buf: Vec<u8>) -> Self {
        Self {
            flags: 0,
            buf: RequestBuf::Own(buf),
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
        Self {
            flags: 0,
            buf: RequestBuf::Ref(buf),
        }
    }
    fn buf(&self) -> &Vec<u8> {
        match &self.buf {
            RequestBuf::Ref(buf) => buf,
            RequestBuf::Own(buf) => buf,
        }
    }
    fn buf_mut(&mut self) -> &mut Vec<u8> {
        match &mut self.buf {
            RequestBuf::Ref(buf) => buf,
            RequestBuf::Own(buf) => buf,
        }
    }
    #[doc = "Set [`libc::NLM_F_CREATE`] flag"]
    pub fn set_create(mut self) -> Self {
        self.flags |= consts::NLM_F_CREATE as u16;
        self
    }
    #[doc = "Set [`libc::NLM_F_EXCL`] flag"]
    pub fn set_excl(mut self) -> Self {
        self.flags |= consts::NLM_F_EXCL as u16;
        self
    }
    #[doc = "Set [`libc::NLM_F_REPLACE`] flag"]
    pub fn set_replace(mut self) -> Self {
        self.flags |= consts::NLM_F_REPLACE as u16;
        self
    }
    #[doc = "Set [`libc::NLM_F_CREATE`] and [`libc::NLM_F_REPLACE`] flag"]
    pub fn set_change(self) -> Self {
        self.set_create().set_replace()
    }
    #[doc = "Set [`libc::NLM_F_APPEND`] flag"]
    pub fn set_append(mut self) -> Self {
        self.flags |= consts::NLM_F_APPEND as u16;
        self
    }
    #[doc = "Set [`libc::NLM_F_DUMP`] flag"]
    fn set_dump(mut self) -> Self {
        self.flags |= consts::NLM_F_DUMP as u16;
        self
    }
    pub fn op_newneigh_do_request(self, header: &PushNdmsg) -> RequestOpNewneighDoRequest<'buf> {
        RequestOpNewneighDoRequest::new(self, header)
    }
    pub fn op_delneigh_do_request(self, header: &PushNdmsg) -> RequestOpDelneighDoRequest<'buf> {
        RequestOpDelneighDoRequest::new(self, header)
    }
    pub fn op_getneigh_dump_request(
        self,
        header: &PushNdmsg,
    ) -> RequestOpGetneighDumpRequest<'buf> {
        RequestOpGetneighDumpRequest::new(self, header)
    }
    pub fn op_getneigh_do_request(self, header: &PushNdmsg) -> RequestOpGetneighDoRequest<'buf> {
        RequestOpGetneighDoRequest::new(self, header)
    }
    pub fn op_getneightbl_dump_request(
        self,
        header: &PushNdtmsg,
    ) -> RequestOpGetneightblDumpRequest<'buf> {
        RequestOpGetneightblDumpRequest::new(self, header)
    }
    pub fn op_setneightbl_do_request(
        self,
        header: &PushNdtmsg,
    ) -> RequestOpSetneightblDoRequest<'buf> {
        RequestOpSetneightblDoRequest::new(self, header)
    }
}
